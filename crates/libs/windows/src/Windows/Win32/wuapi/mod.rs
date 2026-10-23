pub type AddServiceFlag = i32;
pub type AutoDownloadMode = i32;
pub type AutoSelectionMode = i32;
pub const AutomaticUpdates: windows_core::GUID = windows_core::GUID::from_u128(0xbfe18e9c_6d87_4450_b37c_e02f0b373803);
pub type AutomaticUpdatesNotificationLevel = i32;
pub type AutomaticUpdatesPermissionType = i32;
pub type AutomaticUpdatesScheduledInstallationDay = i32;
pub type AutomaticUpdatesUserType = i32;
pub const CLSID_AutomaticUpdates: windows_core::GUID = windows_core::GUID::from_u128(0xbfe18e9c_6d87_4450_b37c_e02f0b373803);
pub const CLSID_InstallationAgent: windows_core::GUID = windows_core::GUID::from_u128(0x317e92fc_1679_46fd_a0b5_f08914dd8623);
pub const CLSID_StringCollection: windows_core::GUID = windows_core::GUID::from_u128(0x72c97d74_7c3b_40ae_b77d_abdb22eba6fb);
pub const CLSID_SystemInformation: windows_core::GUID = windows_core::GUID::from_u128(0xc01b9ba0_bea7_41ba_b604_d0a36f469133);
pub const CLSID_UpdateCollection: windows_core::GUID = windows_core::GUID::from_u128(0x13639463_00db_4646_803d_528026140d88);
pub const CLSID_UpdateDownloader: windows_core::GUID = windows_core::GUID::from_u128(0x5baf654a_5a07_4264_a255_9ff54c7151e7);
pub const CLSID_UpdateInstaller: windows_core::GUID = windows_core::GUID::from_u128(0xd2e0fe7f_d23e_48e1_93c0_6fa8cc346474);
pub const CLSID_UpdateSearcher: windows_core::GUID = windows_core::GUID::from_u128(0xb699e5e8_67ff_4177_88b0_3684a3388bfb);
pub const CLSID_UpdateServiceManager: windows_core::GUID = windows_core::GUID::from_u128(0xf8d253d9_89a4_4daa_87b6_1168369f0b21);
pub const CLSID_UpdateSession: windows_core::GUID = windows_core::GUID::from_u128(0x4cb43d7f_7eee_4906_8698_60da1c38f2fe);
pub const CLSID_WebProxy: windows_core::GUID = windows_core::GUID::from_u128(0x650503cf_9108_4ddc_a2ce_6c2341e1c582);
pub const CLSID_WindowsUpdateAgentInfo: windows_core::GUID = windows_core::GUID::from_u128(0xc2e88c2f_6f5b_4aaa_894b_55c847ad3a2d);
pub type DeploymentAction = i32;
pub type DownloadPhase = i32;
pub type DownloadPriority = i32;
pub type DownloadType = i32;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IAutomaticUpdates, IAutomaticUpdates_Vtbl, 0x673425bf_c082_4c7c_bdfd_569464b8e0ce);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IAutomaticUpdates {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IAutomaticUpdates, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IAutomaticUpdates {
    pub unsafe fn DetectNow(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DetectNow)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ShowSettingsDialog(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowSettingsDialog)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_Settings(&self) -> windows_core::Result<IAutomaticUpdatesSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Settings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ServiceEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServiceEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnableService(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableService)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdates_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub DetectNow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowSettingsDialog: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_ServiceEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ServiceEnabled: usize,
    pub EnableService: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAutomaticUpdates_Impl: super::IDispatch_Impl {
    fn DetectNow(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn ShowSettingsDialog(&self) -> windows_core::Result<()>;
    fn get_Settings(&self) -> windows_core::Result<IAutomaticUpdatesSettings>;
    fn get_ServiceEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn EnableService(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAutomaticUpdates_Vtbl {
    pub const fn new<Identity: IAutomaticUpdates_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DetectNow<Identity: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdates_Impl::DetectNow(this).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdates_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdates_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn ShowSettingsDialog<Identity: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdates_Impl::ShowSettingsDialog(this).into()
            }
        }
        unsafe extern "system" fn get_Settings<Identity: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdates_Impl::get_Settings(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ServiceEnabled<Identity: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdates_Impl::get_ServiceEnabled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableService<Identity: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdates_Impl::EnableService(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DetectNow: DetectNow::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            ShowSettingsDialog: ShowSettingsDialog::<Identity, OFFSET>,
            get_Settings: get_Settings::<Identity, OFFSET>,
            get_ServiceEnabled: get_ServiceEnabled::<Identity, OFFSET>,
            EnableService: EnableService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdates as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAutomaticUpdates {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IAutomaticUpdates2, IAutomaticUpdates2_Vtbl, 0x4a2f5c31_cfd9_410e_b7fb_29a653973a0f);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IAutomaticUpdates2 {
    type Target = IAutomaticUpdates;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IAutomaticUpdates2, windows_core::IUnknown, super::IDispatch, IAutomaticUpdates);
#[cfg(feature = "oaidl")]
impl IAutomaticUpdates2 {
    pub unsafe fn get_Results(&self) -> windows_core::Result<IAutomaticUpdatesResults> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Results)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdates2_Vtbl {
    pub base__: IAutomaticUpdates_Vtbl,
    pub get_Results: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAutomaticUpdates2_Impl: IAutomaticUpdates_Impl {
    fn get_Results(&self) -> windows_core::Result<IAutomaticUpdatesResults>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAutomaticUpdates2_Vtbl {
    pub const fn new<Identity: IAutomaticUpdates2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Results<Identity: IAutomaticUpdates2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdates2_Impl::get_Results(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IAutomaticUpdates_Vtbl::new::<Identity, OFFSET>(), get_Results: get_Results::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdates2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IAutomaticUpdates as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAutomaticUpdates2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IAutomaticUpdatesResults, IAutomaticUpdatesResults_Vtbl, 0xe7a4d634_7942_4dd9_a111_82228ba33901);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IAutomaticUpdatesResults {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IAutomaticUpdatesResults, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IAutomaticUpdatesResults {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_LastSearchSuccessDate(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastSearchSuccessDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_LastInstallationSuccessDate(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastInstallationSuccessDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdatesResults_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_LastSearchSuccessDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_LastSearchSuccessDate: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_LastInstallationSuccessDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_LastInstallationSuccessDate: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAutomaticUpdatesResults_Impl: super::IDispatch_Impl {
    fn get_LastSearchSuccessDate(&self) -> windows_core::Result<super::VARIANT>;
    fn get_LastInstallationSuccessDate(&self) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAutomaticUpdatesResults_Vtbl {
    pub const fn new<Identity: IAutomaticUpdatesResults_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_LastSearchSuccessDate<Identity: IAutomaticUpdatesResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesResults_Impl::get_LastSearchSuccessDate(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastInstallationSuccessDate<Identity: IAutomaticUpdatesResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesResults_Impl::get_LastInstallationSuccessDate(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_LastSearchSuccessDate: get_LastSearchSuccessDate::<Identity, OFFSET>,
            get_LastInstallationSuccessDate: get_LastInstallationSuccessDate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesResults as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAutomaticUpdatesResults {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IAutomaticUpdatesSettings, IAutomaticUpdatesSettings_Vtbl, 0x2ee48f22_af3c_405f_8970_f71be12ee9a2);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IAutomaticUpdatesSettings {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IAutomaticUpdatesSettings, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IAutomaticUpdatesSettings {
    pub unsafe fn get_NotificationLevel(&self) -> windows_core::Result<AutomaticUpdatesNotificationLevel> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NotificationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_NotificationLevel(&self, value: AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NotificationLevel)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Required(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Required)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ScheduledInstallationDay(&self) -> windows_core::Result<AutomaticUpdatesScheduledInstallationDay> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ScheduledInstallationDay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ScheduledInstallationDay(&self, value: AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ScheduledInstallationDay)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_ScheduledInstallationTime(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ScheduledInstallationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_ScheduledInstallationTime(&self, value: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ScheduledInstallationTime)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn Refresh(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Save(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdatesSettings_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_NotificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT,
    pub put_NotificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ReadOnly: usize,
    #[cfg(feature = "wtypes")]
    pub get_Required: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Required: usize,
    pub get_ScheduledInstallationDay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT,
    pub put_ScheduledInstallationDay: unsafe extern "system" fn(*mut core::ffi::c_void, AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_ScheduledInstallationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ScheduledInstallationTime: usize,
    #[cfg(feature = "shared")]
    pub put_ScheduledInstallationTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_ScheduledInstallationTime: usize,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAutomaticUpdatesSettings_Impl: super::IDispatch_Impl {
    fn get_NotificationLevel(&self) -> windows_core::Result<AutomaticUpdatesNotificationLevel>;
    fn put_NotificationLevel(&self, value: AutomaticUpdatesNotificationLevel) -> windows_core::Result<()>;
    fn get_ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Required(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_ScheduledInstallationDay(&self) -> windows_core::Result<AutomaticUpdatesScheduledInstallationDay>;
    fn put_ScheduledInstallationDay(&self, value: AutomaticUpdatesScheduledInstallationDay) -> windows_core::Result<()>;
    fn get_ScheduledInstallationTime(&self) -> windows_core::Result<super::LONG>;
    fn put_ScheduledInstallationTime(&self, value: super::LONG) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAutomaticUpdatesSettings_Vtbl {
    pub const fn new<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_NotificationLevel<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings_Impl::get_NotificationLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NotificationLevel<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings_Impl::put_NotificationLevel(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_ReadOnly<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings_Impl::get_ReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Required<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings_Impl::get_Required(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ScheduledInstallationDay<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings_Impl::get_ScheduledInstallationDay(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ScheduledInstallationDay<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings_Impl::put_ScheduledInstallationDay(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_ScheduledInstallationTime<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings_Impl::get_ScheduledInstallationTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ScheduledInstallationTime<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings_Impl::put_ScheduledInstallationTime(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings_Impl::Save(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_NotificationLevel: get_NotificationLevel::<Identity, OFFSET>,
            put_NotificationLevel: put_NotificationLevel::<Identity, OFFSET>,
            get_ReadOnly: get_ReadOnly::<Identity, OFFSET>,
            get_Required: get_Required::<Identity, OFFSET>,
            get_ScheduledInstallationDay: get_ScheduledInstallationDay::<Identity, OFFSET>,
            put_ScheduledInstallationDay: put_ScheduledInstallationDay::<Identity, OFFSET>,
            get_ScheduledInstallationTime: get_ScheduledInstallationTime::<Identity, OFFSET>,
            put_ScheduledInstallationTime: put_ScheduledInstallationTime::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAutomaticUpdatesSettings {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IAutomaticUpdatesSettings2, IAutomaticUpdatesSettings2_Vtbl, 0x6abc136a_c3ca_4384_8171_cb2b1e59b8dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IAutomaticUpdatesSettings2 {
    type Target = IAutomaticUpdatesSettings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IAutomaticUpdatesSettings2, windows_core::IUnknown, super::IDispatch, IAutomaticUpdatesSettings);
#[cfg(feature = "oaidl")]
impl IAutomaticUpdatesSettings2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IncludeRecommendedUpdates(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IncludeRecommendedUpdates)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_IncludeRecommendedUpdates(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IncludeRecommendedUpdates)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CheckPermission(&self, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckPermission)(windows_core::Interface::as_raw(self), usertype, permissiontype, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdatesSettings2_Vtbl {
    pub base__: IAutomaticUpdatesSettings_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_IncludeRecommendedUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IncludeRecommendedUpdates: usize,
    #[cfg(feature = "wtypes")]
    pub put_IncludeRecommendedUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_IncludeRecommendedUpdates: usize,
    #[cfg(feature = "wtypes")]
    pub CheckPermission: unsafe extern "system" fn(*mut core::ffi::c_void, AutomaticUpdatesUserType, AutomaticUpdatesPermissionType, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CheckPermission: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAutomaticUpdatesSettings2_Impl: IAutomaticUpdatesSettings_Impl {
    fn get_IncludeRecommendedUpdates(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_IncludeRecommendedUpdates(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CheckPermission(&self, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAutomaticUpdatesSettings2_Vtbl {
    pub const fn new<Identity: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_IncludeRecommendedUpdates<Identity: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings2_Impl::get_IncludeRecommendedUpdates(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IncludeRecommendedUpdates<Identity: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings2_Impl::put_IncludeRecommendedUpdates(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn CheckPermission<Identity: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType, userhaspermission: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings2_Impl::CheckPermission(this, core::mem::transmute_copy(&usertype), core::mem::transmute_copy(&permissiontype)) {
                    Ok(ok__) => {
                        userhaspermission.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IAutomaticUpdatesSettings_Vtbl::new::<Identity, OFFSET>(),
            get_IncludeRecommendedUpdates: get_IncludeRecommendedUpdates::<Identity, OFFSET>,
            put_IncludeRecommendedUpdates: put_IncludeRecommendedUpdates::<Identity, OFFSET>,
            CheckPermission: CheckPermission::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IAutomaticUpdatesSettings as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAutomaticUpdatesSettings2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IAutomaticUpdatesSettings3, IAutomaticUpdatesSettings3_Vtbl, 0xb587f5c3_f57e_485f_bbf5_0d181c5cd0dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IAutomaticUpdatesSettings3 {
    type Target = IAutomaticUpdatesSettings2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IAutomaticUpdatesSettings3, windows_core::IUnknown, super::IDispatch, IAutomaticUpdatesSettings, IAutomaticUpdatesSettings2);
#[cfg(feature = "oaidl")]
impl IAutomaticUpdatesSettings3 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_NonAdministratorsElevated(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NonAdministratorsElevated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_NonAdministratorsElevated(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NonAdministratorsElevated)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_FeaturedUpdatesEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FeaturedUpdatesEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_FeaturedUpdatesEnabled(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FeaturedUpdatesEnabled)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdatesSettings3_Vtbl {
    pub base__: IAutomaticUpdatesSettings2_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_NonAdministratorsElevated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_NonAdministratorsElevated: usize,
    #[cfg(feature = "wtypes")]
    pub put_NonAdministratorsElevated: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_NonAdministratorsElevated: usize,
    #[cfg(feature = "wtypes")]
    pub get_FeaturedUpdatesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_FeaturedUpdatesEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_FeaturedUpdatesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_FeaturedUpdatesEnabled: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAutomaticUpdatesSettings3_Impl: IAutomaticUpdatesSettings2_Impl {
    fn get_NonAdministratorsElevated(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_NonAdministratorsElevated(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_FeaturedUpdatesEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_FeaturedUpdatesEnabled(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAutomaticUpdatesSettings3_Vtbl {
    pub const fn new<Identity: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_NonAdministratorsElevated<Identity: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings3_Impl::get_NonAdministratorsElevated(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NonAdministratorsElevated<Identity: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings3_Impl::put_NonAdministratorsElevated(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_FeaturedUpdatesEnabled<Identity: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings3_Impl::get_FeaturedUpdatesEnabled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FeaturedUpdatesEnabled<Identity: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings3_Impl::put_FeaturedUpdatesEnabled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IAutomaticUpdatesSettings2_Vtbl::new::<Identity, OFFSET>(),
            get_NonAdministratorsElevated: get_NonAdministratorsElevated::<Identity, OFFSET>,
            put_NonAdministratorsElevated: put_NonAdministratorsElevated::<Identity, OFFSET>,
            get_FeaturedUpdatesEnabled: get_FeaturedUpdatesEnabled::<Identity, OFFSET>,
            put_FeaturedUpdatesEnabled: put_FeaturedUpdatesEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IAutomaticUpdatesSettings as windows_core::Interface>::IID || iid == &<IAutomaticUpdatesSettings2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAutomaticUpdatesSettings3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICategory, ICategory_Vtbl, 0x81ddc1b8_9d35_47a6_b471_5b80f519223b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICategory {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICategory, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ICategory {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CategoryID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CategoryID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Children(&self) -> windows_core::Result<ICategoryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Children)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Image(&self) -> windows_core::Result<IImageInformation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Image)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Order(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Order)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Parent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Type(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Updates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICategory_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CategoryID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Order: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Order: usize,
    pub get_Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICategory_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CategoryID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Children(&self) -> windows_core::Result<ICategoryCollection>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Image(&self) -> windows_core::Result<IImageInformation>;
    fn get_Order(&self) -> windows_core::Result<super::LONG>;
    fn get_Parent(&self) -> windows_core::Result<ICategory>;
    fn get_Type(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Updates(&self) -> windows_core::Result<IUpdateCollection>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICategory_Vtbl {
    pub const fn new<Identity: ICategory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::get_Name(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CategoryID<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::get_CategoryID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Children<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::get_Children(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Description<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Image<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::get_Image(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Order<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::get_Order(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Parent<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::get_Parent(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Type<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::get_Type(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Updates<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::get_Updates(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            get_CategoryID: get_CategoryID::<Identity, OFFSET>,
            get_Children: get_Children::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            get_Image: get_Image::<Identity, OFFSET>,
            get_Order: get_Order::<Identity, OFFSET>,
            get_Parent: get_Parent::<Identity, OFFSET>,
            get_Type: get_Type::<Identity, OFFSET>,
            get_Updates: get_Updates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICategory as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICategory {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICategoryCollection, ICategoryCollection_Vtbl, 0x3a56bfb8_576c_43f7_9335_fe4838fd7e37);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICategoryCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICategoryCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ICategoryCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<ICategory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICategoryCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICategoryCollection_Impl: super::IDispatch_Impl {
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<ICategory>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICategoryCollection_Vtbl {
    pub const fn new<Identity: ICategoryCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategoryCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategoryCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategoryCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICategoryCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICategoryCollection {}
windows_core::imp::define_interface!(IDownloadCompletedCallback, IDownloadCompletedCallback_Vtbl, 0x77254866_9f5b_4c8e_b9e2_c77a8530d64b);
windows_core::imp::interface_hierarchy!(IDownloadCompletedCallback, windows_core::IUnknown);
impl IDownloadCompletedCallback {
    #[cfg(feature = "oaidl")]
    pub unsafe fn Invoke<P0, P1>(&self, downloadjob: P0, callbackargs: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDownloadJob>,
        P1: windows_core::Param<IDownloadCompletedCallbackArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), downloadjob.param().abi(), callbackargs.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadCompletedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    Invoke: usize,
}
#[cfg(feature = "oaidl")]
pub trait IDownloadCompletedCallback_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, downloadjob: windows_core::Ref<IDownloadJob>, callbackargs: windows_core::Ref<IDownloadCompletedCallbackArgs>) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl IDownloadCompletedCallback_Vtbl {
    pub const fn new<Identity: IDownloadCompletedCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IDownloadCompletedCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadjob: *mut core::ffi::c_void, callbackargs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDownloadCompletedCallback_Impl::Invoke(this, core::mem::transmute_copy(&downloadjob), core::mem::transmute_copy(&callbackargs)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadCompletedCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IDownloadCompletedCallback {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDownloadCompletedCallbackArgs, IDownloadCompletedCallbackArgs_Vtbl, 0xfa565b23_498c_47a0_979d_e7d5b1813360);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDownloadCompletedCallbackArgs {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDownloadCompletedCallbackArgs, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadCompletedCallbackArgs_Vtbl {
    pub base__: super::IDispatch_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDownloadCompletedCallbackArgs_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDownloadCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: IDownloadCompletedCallbackArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadCompletedCallbackArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDownloadCompletedCallbackArgs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDownloadJob, IDownloadJob_Vtbl, 0xc574de85_7358_43f6_aae8_8697e62d8ba7);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDownloadJob {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDownloadJob, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDownloadJob {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_AsyncState(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AsyncState)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsCompleted(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsCompleted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Updates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CleanUp(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CleanUp)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetProgress(&self) -> windows_core::Result<IDownloadProgress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RequestAbort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestAbort)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadJob_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_AsyncState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_AsyncState: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsCompleted: usize,
    pub get_Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CleanUp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDownloadJob_Impl: super::IDispatch_Impl {
    fn get_AsyncState(&self) -> windows_core::Result<super::VARIANT>;
    fn get_IsCompleted(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn CleanUp(&self) -> windows_core::Result<()>;
    fn GetProgress(&self) -> windows_core::Result<IDownloadProgress>;
    fn RequestAbort(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDownloadJob_Vtbl {
    pub const fn new<Identity: IDownloadJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AsyncState<Identity: IDownloadJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadJob_Impl::get_AsyncState(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsCompleted<Identity: IDownloadJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadJob_Impl::get_IsCompleted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Updates<Identity: IDownloadJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadJob_Impl::get_Updates(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CleanUp<Identity: IDownloadJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDownloadJob_Impl::CleanUp(this).into()
            }
        }
        unsafe extern "system" fn GetProgress<Identity: IDownloadJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadJob_Impl::GetProgress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestAbort<Identity: IDownloadJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDownloadJob_Impl::RequestAbort(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_AsyncState: get_AsyncState::<Identity, OFFSET>,
            get_IsCompleted: get_IsCompleted::<Identity, OFFSET>,
            get_Updates: get_Updates::<Identity, OFFSET>,
            CleanUp: CleanUp::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
            RequestAbort: RequestAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadJob as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDownloadJob {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDownloadProgress, IDownloadProgress_Vtbl, 0xd31a5bac_f719_4178_9dbb_5e2cb47fd18a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDownloadProgress {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDownloadProgress, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDownloadProgress {
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn get_CurrentUpdateBytesDownloaded(&self) -> windows_core::Result<super::DECIMAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentUpdateBytesDownloaded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn get_CurrentUpdateBytesToDownload(&self) -> windows_core::Result<super::DECIMAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentUpdateBytesToDownload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_CurrentUpdateIndex(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentUpdateIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_PercentComplete(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PercentComplete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn get_TotalBytesDownloaded(&self) -> windows_core::Result<super::DECIMAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TotalBytesDownloaded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn get_TotalBytesToDownload(&self) -> windows_core::Result<super::DECIMAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TotalBytesToDownload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetUpdateResult(&self, updateindex: super::LONG) -> windows_core::Result<IUpdateDownloadResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdateResult)(windows_core::Interface::as_raw(self), updateindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentUpdateDownloadPhase(&self) -> windows_core::Result<DownloadPhase> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentUpdateDownloadPhase)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_CurrentUpdatePercentComplete(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentUpdatePercentComplete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgress_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub get_CurrentUpdateBytesDownloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DECIMAL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    get_CurrentUpdateBytesDownloaded: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub get_CurrentUpdateBytesToDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DECIMAL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    get_CurrentUpdateBytesToDownload: usize,
    #[cfg(feature = "shared")]
    pub get_CurrentUpdateIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CurrentUpdateIndex: usize,
    #[cfg(feature = "shared")]
    pub get_PercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_PercentComplete: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub get_TotalBytesDownloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DECIMAL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    get_TotalBytesDownloaded: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub get_TotalBytesToDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DECIMAL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    get_TotalBytesToDownload: usize,
    #[cfg(feature = "shared")]
    pub GetUpdateResult: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetUpdateResult: usize,
    pub get_CurrentUpdateDownloadPhase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DownloadPhase) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_CurrentUpdatePercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CurrentUpdatePercentComplete: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDownloadProgress_Impl: super::IDispatch_Impl {
    fn get_CurrentUpdateBytesDownloaded(&self) -> windows_core::Result<super::DECIMAL>;
    fn get_CurrentUpdateBytesToDownload(&self) -> windows_core::Result<super::DECIMAL>;
    fn get_CurrentUpdateIndex(&self) -> windows_core::Result<super::LONG>;
    fn get_PercentComplete(&self) -> windows_core::Result<super::LONG>;
    fn get_TotalBytesDownloaded(&self) -> windows_core::Result<super::DECIMAL>;
    fn get_TotalBytesToDownload(&self) -> windows_core::Result<super::DECIMAL>;
    fn GetUpdateResult(&self, updateindex: super::LONG) -> windows_core::Result<IUpdateDownloadResult>;
    fn get_CurrentUpdateDownloadPhase(&self) -> windows_core::Result<DownloadPhase>;
    fn get_CurrentUpdatePercentComplete(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDownloadProgress_Vtbl {
    pub const fn new<Identity: IDownloadProgress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentUpdateBytesDownloaded<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DECIMAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::get_CurrentUpdateBytesDownloaded(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentUpdateBytesToDownload<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DECIMAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::get_CurrentUpdateBytesToDownload(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentUpdateIndex<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::get_CurrentUpdateIndex(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PercentComplete<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::get_PercentComplete(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TotalBytesDownloaded<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DECIMAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::get_TotalBytesDownloaded(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TotalBytesToDownload<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DECIMAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::get_TotalBytesToDownload(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updateindex: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::GetUpdateResult(this, core::mem::transmute_copy(&updateindex)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentUpdateDownloadPhase<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut DownloadPhase) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::get_CurrentUpdateDownloadPhase(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentUpdatePercentComplete<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::get_CurrentUpdatePercentComplete(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentUpdateBytesDownloaded: get_CurrentUpdateBytesDownloaded::<Identity, OFFSET>,
            get_CurrentUpdateBytesToDownload: get_CurrentUpdateBytesToDownload::<Identity, OFFSET>,
            get_CurrentUpdateIndex: get_CurrentUpdateIndex::<Identity, OFFSET>,
            get_PercentComplete: get_PercentComplete::<Identity, OFFSET>,
            get_TotalBytesDownloaded: get_TotalBytesDownloaded::<Identity, OFFSET>,
            get_TotalBytesToDownload: get_TotalBytesToDownload::<Identity, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, OFFSET>,
            get_CurrentUpdateDownloadPhase: get_CurrentUpdateDownloadPhase::<Identity, OFFSET>,
            get_CurrentUpdatePercentComplete: get_CurrentUpdatePercentComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadProgress as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDownloadProgress {}
windows_core::imp::define_interface!(IDownloadProgressChangedCallback, IDownloadProgressChangedCallback_Vtbl, 0x8c3f1cdd_6173_4591_aebd_a56a53ca77c1);
windows_core::imp::interface_hierarchy!(IDownloadProgressChangedCallback, windows_core::IUnknown);
impl IDownloadProgressChangedCallback {
    #[cfg(feature = "oaidl")]
    pub unsafe fn Invoke<P0, P1>(&self, downloadjob: P0, callbackargs: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDownloadJob>,
        P1: windows_core::Param<IDownloadProgressChangedCallbackArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), downloadjob.param().abi(), callbackargs.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgressChangedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    Invoke: usize,
}
#[cfg(feature = "oaidl")]
pub trait IDownloadProgressChangedCallback_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, downloadjob: windows_core::Ref<IDownloadJob>, callbackargs: windows_core::Ref<IDownloadProgressChangedCallbackArgs>) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl IDownloadProgressChangedCallback_Vtbl {
    pub const fn new<Identity: IDownloadProgressChangedCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IDownloadProgressChangedCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadjob: *mut core::ffi::c_void, callbackargs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDownloadProgressChangedCallback_Impl::Invoke(this, core::mem::transmute_copy(&downloadjob), core::mem::transmute_copy(&callbackargs)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadProgressChangedCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IDownloadProgressChangedCallback {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDownloadProgressChangedCallbackArgs, IDownloadProgressChangedCallbackArgs_Vtbl, 0x324ff2c6_4981_4b04_9412_57481745ab24);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDownloadProgressChangedCallbackArgs {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDownloadProgressChangedCallbackArgs, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDownloadProgressChangedCallbackArgs {
    pub unsafe fn get_Progress(&self) -> windows_core::Result<IDownloadProgress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Progress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgressChangedCallbackArgs_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDownloadProgressChangedCallbackArgs_Impl: super::IDispatch_Impl {
    fn get_Progress(&self) -> windows_core::Result<IDownloadProgress>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDownloadProgressChangedCallbackArgs_Vtbl {
    pub const fn new<Identity: IDownloadProgressChangedCallbackArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Progress<Identity: IDownloadProgressChangedCallbackArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgressChangedCallbackArgs_Impl::get_Progress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), get_Progress: get_Progress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadProgressChangedCallbackArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDownloadProgressChangedCallbackArgs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDownloadResult, IDownloadResult_Vtbl, 0xdaa4fdd0_4727_4dbe_a1e7_745dca317144);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDownloadResult {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDownloadResult, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDownloadResult {
    #[cfg(feature = "shared")]
    pub unsafe fn get_HResult(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetUpdateResult(&self, updateindex: super::LONG) -> windows_core::Result<IUpdateDownloadResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdateResult)(windows_core::Interface::as_raw(self), updateindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadResult_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_HResult: usize,
    pub get_ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetUpdateResult: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetUpdateResult: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDownloadResult_Impl: super::IDispatch_Impl {
    fn get_HResult(&self) -> windows_core::Result<super::LONG>;
    fn get_ResultCode(&self) -> windows_core::Result<OperationResultCode>;
    fn GetUpdateResult(&self, updateindex: super::LONG) -> windows_core::Result<IUpdateDownloadResult>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDownloadResult_Vtbl {
    pub const fn new<Identity: IDownloadResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_HResult<Identity: IDownloadResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadResult_Impl::get_HResult(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ResultCode<Identity: IDownloadResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadResult_Impl::get_ResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: IDownloadResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updateindex: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadResult_Impl::GetUpdateResult(this, core::mem::transmute_copy(&updateindex)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_HResult: get_HResult::<Identity, OFFSET>,
            get_ResultCode: get_ResultCode::<Identity, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadResult as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDownloadResult {}
pub const IID_IAutomaticUpdates: windows_core::GUID = windows_core::GUID::from_u128(0x673425bf_c082_4c7c_bdfd_569464b8e0ce);
pub const IID_IAutomaticUpdates2: windows_core::GUID = windows_core::GUID::from_u128(0x4a2f5c31_cfd9_410e_b7fb_29a653973a0f);
pub const IID_IAutomaticUpdatesResults: windows_core::GUID = windows_core::GUID::from_u128(0xe7a4d634_7942_4dd9_a111_82228ba33901);
pub const IID_IAutomaticUpdatesSettings: windows_core::GUID = windows_core::GUID::from_u128(0x2ee48f22_af3c_405f_8970_f71be12ee9a2);
pub const IID_IAutomaticUpdatesSettings2: windows_core::GUID = windows_core::GUID::from_u128(0x6abc136a_c3ca_4384_8171_cb2b1e59b8dc);
pub const IID_IAutomaticUpdatesSettings3: windows_core::GUID = windows_core::GUID::from_u128(0xb587f5c3_f57e_485f_bbf5_0d181c5cd0dc);
pub const IID_ICategory: windows_core::GUID = windows_core::GUID::from_u128(0x81ddc1b8_9d35_47a6_b471_5b80f519223b);
pub const IID_ICategoryCollection: windows_core::GUID = windows_core::GUID::from_u128(0x3a56bfb8_576c_43f7_9335_fe4838fd7e37);
pub const IID_IDownloadCompletedCallback: windows_core::GUID = windows_core::GUID::from_u128(0x77254866_9f5b_4c8e_b9e2_c77a8530d64b);
pub const IID_IDownloadCompletedCallbackArgs: windows_core::GUID = windows_core::GUID::from_u128(0xfa565b23_498c_47a0_979d_e7d5b1813360);
pub const IID_IDownloadJob: windows_core::GUID = windows_core::GUID::from_u128(0xc574de85_7358_43f6_aae8_8697e62d8ba7);
pub const IID_IDownloadProgress: windows_core::GUID = windows_core::GUID::from_u128(0xd31a5bac_f719_4178_9dbb_5e2cb47fd18a);
pub const IID_IDownloadProgressChangedCallback: windows_core::GUID = windows_core::GUID::from_u128(0x8c3f1cdd_6173_4591_aebd_a56a53ca77c1);
pub const IID_IDownloadProgressChangedCallbackArgs: windows_core::GUID = windows_core::GUID::from_u128(0x324ff2c6_4981_4b04_9412_57481745ab24);
pub const IID_IDownloadResult: windows_core::GUID = windows_core::GUID::from_u128(0xdaa4fdd0_4727_4dbe_a1e7_745dca317144);
pub const IID_IImageInformation: windows_core::GUID = windows_core::GUID::from_u128(0x7c907864_346c_4aeb_8f3f_57da289f969f);
pub const IID_IInstallationAgent: windows_core::GUID = windows_core::GUID::from_u128(0x925cbc18_a2ea_4648_bf1c_ec8badcfe20a);
pub const IID_IInstallationBehavior: windows_core::GUID = windows_core::GUID::from_u128(0xd9a59339_e245_4dbd_9686_4d5763e39624);
pub const IID_IInstallationCompletedCallback: windows_core::GUID = windows_core::GUID::from_u128(0x45f4f6f3_d602_4f98_9a8a_3efa152ad2d3);
pub const IID_IInstallationCompletedCallbackArgs: windows_core::GUID = windows_core::GUID::from_u128(0x250e2106_8efb_4705_9653_ef13c581b6a1);
pub const IID_IInstallationJob: windows_core::GUID = windows_core::GUID::from_u128(0x5c209f0b_bad5_432a_9556_4699bed2638a);
pub const IID_IInstallationProgress: windows_core::GUID = windows_core::GUID::from_u128(0x345c8244_43a3_4e32_a368_65f073b76f36);
pub const IID_IInstallationProgressChangedCallback: windows_core::GUID = windows_core::GUID::from_u128(0xe01402d5_f8da_43ba_a012_38894bd048f1);
pub const IID_IInstallationProgressChangedCallbackArgs: windows_core::GUID = windows_core::GUID::from_u128(0xe4f14e1e_689d_4218_a0b9_bc189c484a01);
pub const IID_IInstallationResult: windows_core::GUID = windows_core::GUID::from_u128(0xa43c56d6_7451_48d4_af96_b6cd2d0d9b7a);
pub const IID_IInvalidProductLicenseException: windows_core::GUID = windows_core::GUID::from_u128(0xa37d00f5_7bb0_4953_b414_f9e98326f2e8);
pub const IID_ISearchCompletedCallback: windows_core::GUID = windows_core::GUID::from_u128(0x88aee058_d4b0_4725_a2f1_814a67ae964c);
pub const IID_ISearchCompletedCallbackArgs: windows_core::GUID = windows_core::GUID::from_u128(0xa700a634_2850_4c47_938a_9e4b6e5af9a6);
pub const IID_ISearchJob: windows_core::GUID = windows_core::GUID::from_u128(0x7366ea16_7a1a_4ea2_b042_973d3e9cd99b);
pub const IID_ISearchResult: windows_core::GUID = windows_core::GUID::from_u128(0xd40cff62_e08c_4498_941a_01e25f0fd33c);
pub const IID_IStringCollection: windows_core::GUID = windows_core::GUID::from_u128(0xeff90582_2ddc_480f_a06d_60f3fbc362c3);
pub const IID_ISystemInformation: windows_core::GUID = windows_core::GUID::from_u128(0xade87bf7_7b56_4275_8fab_b9b0e591844b);
pub const IID_IUpdate: windows_core::GUID = windows_core::GUID::from_u128(0x6a92b07a_d821_4682_b423_5c805022cc4d);
pub const IID_IUpdate2: windows_core::GUID = windows_core::GUID::from_u128(0x144fe9b0_d23d_4a8b_8634_fb4457533b7a);
pub const IID_IUpdate3: windows_core::GUID = windows_core::GUID::from_u128(0x112eda6b_95b3_476f_9d90_aee82c6b8181);
pub const IID_IUpdate4: windows_core::GUID = windows_core::GUID::from_u128(0x27e94b0d_5139_49a2_9a61_93522dc54652);
pub const IID_IUpdate5: windows_core::GUID = windows_core::GUID::from_u128(0xc1c2f21a_d2f4_4902_b5c6_8a081c19a890);
pub const IID_IUpdateCollection: windows_core::GUID = windows_core::GUID::from_u128(0x07f7438c_7709_4ca5_b518_91279288134e);
pub const IID_IUpdateDownloadContent: windows_core::GUID = windows_core::GUID::from_u128(0x54a2cb2d_9a0c_48b6_8a50_9abb69ee2d02);
pub const IID_IUpdateDownloadContent2: windows_core::GUID = windows_core::GUID::from_u128(0xc97ad11b_f257_420b_9d9f_377f733f6f68);
pub const IID_IUpdateDownloadContentCollection: windows_core::GUID = windows_core::GUID::from_u128(0xbc5513c8_b3b8_4bf7_a4d4_361c0d8c88ba);
pub const IID_IUpdateDownloadResult: windows_core::GUID = windows_core::GUID::from_u128(0xbf99af76_b575_42ad_8aa4_33cbb5477af1);
pub const IID_IUpdateDownloader: windows_core::GUID = windows_core::GUID::from_u128(0x68f1c6f9_7ecc_4666_a464_247fe12496c3);
pub const IID_IUpdateDownloaderEx: windows_core::GUID = windows_core::GUID::from_u128(0x94726306_f12a_482a_a774_fb4f870d98c0);
pub const IID_IUpdateEx: windows_core::GUID = windows_core::GUID::from_u128(0x769355a3_c5a0_497c_a606_560a36d2121c);
pub const IID_IUpdateException: windows_core::GUID = windows_core::GUID::from_u128(0xa376dd5e_09d4_427f_af7c_fed5b6e1c1d6);
pub const IID_IUpdateExceptionCollection: windows_core::GUID = windows_core::GUID::from_u128(0x503626a3_8e14_4729_9355_0fe664bd2321);
pub const IID_IUpdateHistoryEntry: windows_core::GUID = windows_core::GUID::from_u128(0xbe56a644_af0e_4e0e_a311_c1d8e695cbff);
pub const IID_IUpdateHistoryEntry2: windows_core::GUID = windows_core::GUID::from_u128(0xc2bfb780_4539_4132_ab8c_0a8772013ab6);
pub const IID_IUpdateHistoryEntryCollection: windows_core::GUID = windows_core::GUID::from_u128(0xa7f04f3c_a290_435b_aadf_a116c3357a5c);
pub const IID_IUpdateIdentity: windows_core::GUID = windows_core::GUID::from_u128(0x46297823_9940_4c09_aed9_cd3ea6d05968);
pub const IID_IUpdateInstallationResult: windows_core::GUID = windows_core::GUID::from_u128(0xd940f0f8_3cbb_4fd0_993f_471e7f2328ad);
pub const IID_IUpdateInstaller: windows_core::GUID = windows_core::GUID::from_u128(0x7b929c68_ccdc_4226_96b1_8724600b54c2);
pub const IID_IUpdateInstaller2: windows_core::GUID = windows_core::GUID::from_u128(0x3442d4fe_224d_4cee_98cf_30e0c4d229e6);
pub const IID_IUpdateInstaller3: windows_core::GUID = windows_core::GUID::from_u128(0x16d11c35_099a_48d0_8338_5fae64047f8e);
pub const IID_IUpdateInstaller4: windows_core::GUID = windows_core::GUID::from_u128(0xef8208ea_2304_492d_9109_23813b0958e1);
pub const IID_IUpdateLockdown: windows_core::GUID = windows_core::GUID::from_u128(0xa976c28d_75a1_42aa_94ae_8af8b872089a);
pub const IID_IUpdateSearcher: windows_core::GUID = windows_core::GUID::from_u128(0x8f45abf1_f9ae_4b95_a933_f0f66e5056ea);
pub const IID_IUpdateSearcher2: windows_core::GUID = windows_core::GUID::from_u128(0x4cbdcb2d_1589_4beb_bd1c_3e582ff0add0);
pub const IID_IUpdateSearcher3: windows_core::GUID = windows_core::GUID::from_u128(0x04c6895d_eaf2_4034_97f3_311de9be413a);
pub const IID_IUpdateService: windows_core::GUID = windows_core::GUID::from_u128(0x76b3b17e_aed6_4da5_85f0_83587f81abe3);
pub const IID_IUpdateService2: windows_core::GUID = windows_core::GUID::from_u128(0x1518b460_6518_4172_940f_c75883b24ceb);
pub const IID_IUpdateServiceCollection: windows_core::GUID = windows_core::GUID::from_u128(0x9b0353aa_0e52_44ff_b8b0_1f7fa0437f88);
pub const IID_IUpdateServiceManager: windows_core::GUID = windows_core::GUID::from_u128(0x23857e3c_02ba_44a3_9423_b1c900805f37);
pub const IID_IUpdateServiceManager2: windows_core::GUID = windows_core::GUID::from_u128(0x0bb8531d_7e8d_424f_986c_a0b8f60a3e7b);
pub const IID_IUpdateServiceRegistration: windows_core::GUID = windows_core::GUID::from_u128(0xdde02280_12b3_4e0b_937b_6747f6acb286);
pub const IID_IUpdateSession: windows_core::GUID = windows_core::GUID::from_u128(0x816858a4_260d_4260_933a_2585f1abc76b);
pub const IID_IUpdateSession2: windows_core::GUID = windows_core::GUID::from_u128(0x91caf7b0_eb23_49ed_9937_c52d817f46f7);
pub const IID_IUpdateSession3: windows_core::GUID = windows_core::GUID::from_u128(0x918efd1e_b5d8_4c90_8540_aeb9bdc56f9d);
pub const IID_IWebProxy: windows_core::GUID = windows_core::GUID::from_u128(0x174c81fe_aecd_4dae_b8a0_2c6318dd86a8);
pub const IID_IWindowsDriverUpdate: windows_core::GUID = windows_core::GUID::from_u128(0xb383cd1a_5ce9_4504_9f63_764b1236f191);
pub const IID_IWindowsDriverUpdate2: windows_core::GUID = windows_core::GUID::from_u128(0x615c4269_7a48_43bd_96b7_bf6ca27d6c3e);
pub const IID_IWindowsDriverUpdate3: windows_core::GUID = windows_core::GUID::from_u128(0x49ebd502_4a96_41bd_9e3e_4c5057f4250c);
pub const IID_IWindowsDriverUpdate4: windows_core::GUID = windows_core::GUID::from_u128(0x004c6a2b_0c19_4c69_9f5c_a269b2560db9);
pub const IID_IWindowsDriverUpdate5: windows_core::GUID = windows_core::GUID::from_u128(0x70cf5c82_8642_42bb_9dbc_0cfd263c6c4f);
pub const IID_IWindowsDriverUpdateEntry: windows_core::GUID = windows_core::GUID::from_u128(0xed8bfe40_a60b_42ea_9652_817dfcfa23ec);
pub const IID_IWindowsDriverUpdateEntryCollection: windows_core::GUID = windows_core::GUID::from_u128(0x0d521700_a372_4bef_828b_3d00c10adebd);
pub const IID_IWindowsUpdateAgentInfo: windows_core::GUID = windows_core::GUID::from_u128(0x85713fa1_7796_4fa2_be3b_e2d6124dd373);
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IImageInformation, IImageInformation_Vtbl, 0x7c907864_346c_4aeb_8f3f_57da289f969f);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IImageInformation {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IImageInformation, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IImageInformation {
    pub unsafe fn get_AltText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AltText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Height(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Source(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Source)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Width(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IImageInformation_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_AltText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Height: usize,
    pub get_Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Width: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IImageInformation_Impl: super::IDispatch_Impl {
    fn get_AltText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Height(&self) -> windows_core::Result<super::LONG>;
    fn get_Source(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Width(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IImageInformation_Vtbl {
    pub const fn new<Identity: IImageInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AltText<Identity: IImageInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageInformation_Impl::get_AltText(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Height<Identity: IImageInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageInformation_Impl::get_Height(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Source<Identity: IImageInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageInformation_Impl::get_Source(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Width<Identity: IImageInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageInformation_Impl::get_Width(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_AltText: get_AltText::<Identity, OFFSET>,
            get_Height: get_Height::<Identity, OFFSET>,
            get_Source: get_Source::<Identity, OFFSET>,
            get_Width: get_Width::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageInformation as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IImageInformation {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IInstallationAgent, IInstallationAgent_Vtbl, 0x925cbc18_a2ea_4648_bf1c_ec8badcfe20a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IInstallationAgent {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IInstallationAgent, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IInstallationAgent {
    #[cfg(feature = "shared")]
    pub unsafe fn RecordInstallationResult<P2>(&self, installationresultcookie: &windows_core::BSTR, hresult: super::LONG, extendedreportingdata: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IStringCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).RecordInstallationResult)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(installationresultcookie), hresult, extendedreportingdata.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationAgent_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub RecordInstallationResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RecordInstallationResult: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationAgent_Impl: super::IDispatch_Impl {
    fn RecordInstallationResult(&self, installationresultcookie: &windows_core::BSTR, hresult: super::LONG, extendedreportingdata: windows_core::Ref<IStringCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationAgent_Vtbl {
    pub const fn new<Identity: IInstallationAgent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RecordInstallationResult<Identity: IInstallationAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, installationresultcookie: *mut core::ffi::c_void, hresult: super::LONG, extendedreportingdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInstallationAgent_Impl::RecordInstallationResult(this, core::mem::transmute(&installationresultcookie), core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&extendedreportingdata)).into()
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), RecordInstallationResult: RecordInstallationResult::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationAgent as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IInstallationAgent {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IInstallationBehavior, IInstallationBehavior_Vtbl, 0xd9a59339_e245_4dbd_9686_4d5763e39624);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IInstallationBehavior {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IInstallationBehavior, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IInstallationBehavior {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_CanRequestUserInput(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CanRequestUserInput)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Impact(&self) -> windows_core::Result<InstallationImpact> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Impact)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_RebootBehavior(&self) -> windows_core::Result<InstallationRebootBehavior> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RebootBehavior)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RequiresNetworkConnectivity(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RequiresNetworkConnectivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationBehavior_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_CanRequestUserInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_CanRequestUserInput: usize,
    pub get_Impact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InstallationImpact) -> windows_core::HRESULT,
    pub get_RebootBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InstallationRebootBehavior) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_RequiresNetworkConnectivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RequiresNetworkConnectivity: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationBehavior_Impl: super::IDispatch_Impl {
    fn get_CanRequestUserInput(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Impact(&self) -> windows_core::Result<InstallationImpact>;
    fn get_RebootBehavior(&self) -> windows_core::Result<InstallationRebootBehavior>;
    fn get_RequiresNetworkConnectivity(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationBehavior_Vtbl {
    pub const fn new<Identity: IInstallationBehavior_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CanRequestUserInput<Identity: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationBehavior_Impl::get_CanRequestUserInput(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Impact<Identity: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut InstallationImpact) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationBehavior_Impl::get_Impact(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RebootBehavior<Identity: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut InstallationRebootBehavior) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationBehavior_Impl::get_RebootBehavior(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RequiresNetworkConnectivity<Identity: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationBehavior_Impl::get_RequiresNetworkConnectivity(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_CanRequestUserInput: get_CanRequestUserInput::<Identity, OFFSET>,
            get_Impact: get_Impact::<Identity, OFFSET>,
            get_RebootBehavior: get_RebootBehavior::<Identity, OFFSET>,
            get_RequiresNetworkConnectivity: get_RequiresNetworkConnectivity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationBehavior as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IInstallationBehavior {}
windows_core::imp::define_interface!(IInstallationCompletedCallback, IInstallationCompletedCallback_Vtbl, 0x45f4f6f3_d602_4f98_9a8a_3efa152ad2d3);
windows_core::imp::interface_hierarchy!(IInstallationCompletedCallback, windows_core::IUnknown);
impl IInstallationCompletedCallback {
    #[cfg(feature = "oaidl")]
    pub unsafe fn Invoke<P0, P1>(&self, installationjob: P0, callbackargs: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IInstallationJob>,
        P1: windows_core::Param<IInstallationCompletedCallbackArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), installationjob.param().abi(), callbackargs.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationCompletedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    Invoke: usize,
}
#[cfg(feature = "oaidl")]
pub trait IInstallationCompletedCallback_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, installationjob: windows_core::Ref<IInstallationJob>, callbackargs: windows_core::Ref<IInstallationCompletedCallbackArgs>) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl IInstallationCompletedCallback_Vtbl {
    pub const fn new<Identity: IInstallationCompletedCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IInstallationCompletedCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, installationjob: *mut core::ffi::c_void, callbackargs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInstallationCompletedCallback_Impl::Invoke(this, core::mem::transmute_copy(&installationjob), core::mem::transmute_copy(&callbackargs)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationCompletedCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IInstallationCompletedCallback {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IInstallationCompletedCallbackArgs, IInstallationCompletedCallbackArgs_Vtbl, 0x250e2106_8efb_4705_9653_ef13c581b6a1);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IInstallationCompletedCallbackArgs {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IInstallationCompletedCallbackArgs, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationCompletedCallbackArgs_Vtbl {
    pub base__: super::IDispatch_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationCompletedCallbackArgs_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: IInstallationCompletedCallbackArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationCompletedCallbackArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IInstallationCompletedCallbackArgs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IInstallationJob, IInstallationJob_Vtbl, 0x5c209f0b_bad5_432a_9556_4699bed2638a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IInstallationJob {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IInstallationJob, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IInstallationJob {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_AsyncState(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AsyncState)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsCompleted(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsCompleted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Updates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CleanUp(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CleanUp)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetProgress(&self) -> windows_core::Result<IInstallationProgress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RequestAbort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestAbort)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationJob_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_AsyncState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_AsyncState: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsCompleted: usize,
    pub get_Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CleanUp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationJob_Impl: super::IDispatch_Impl {
    fn get_AsyncState(&self) -> windows_core::Result<super::VARIANT>;
    fn get_IsCompleted(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn CleanUp(&self) -> windows_core::Result<()>;
    fn GetProgress(&self) -> windows_core::Result<IInstallationProgress>;
    fn RequestAbort(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationJob_Vtbl {
    pub const fn new<Identity: IInstallationJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AsyncState<Identity: IInstallationJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationJob_Impl::get_AsyncState(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsCompleted<Identity: IInstallationJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationJob_Impl::get_IsCompleted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Updates<Identity: IInstallationJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationJob_Impl::get_Updates(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CleanUp<Identity: IInstallationJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInstallationJob_Impl::CleanUp(this).into()
            }
        }
        unsafe extern "system" fn GetProgress<Identity: IInstallationJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationJob_Impl::GetProgress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestAbort<Identity: IInstallationJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInstallationJob_Impl::RequestAbort(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_AsyncState: get_AsyncState::<Identity, OFFSET>,
            get_IsCompleted: get_IsCompleted::<Identity, OFFSET>,
            get_Updates: get_Updates::<Identity, OFFSET>,
            CleanUp: CleanUp::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
            RequestAbort: RequestAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationJob as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IInstallationJob {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IInstallationProgress, IInstallationProgress_Vtbl, 0x345c8244_43a3_4e32_a368_65f073b76f36);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IInstallationProgress {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IInstallationProgress, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IInstallationProgress {
    #[cfg(feature = "shared")]
    pub unsafe fn get_CurrentUpdateIndex(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentUpdateIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_CurrentUpdatePercentComplete(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentUpdatePercentComplete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_PercentComplete(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PercentComplete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetUpdateResult(&self, updateindex: super::LONG) -> windows_core::Result<IUpdateInstallationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdateResult)(windows_core::Interface::as_raw(self), updateindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationProgress_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_CurrentUpdateIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CurrentUpdateIndex: usize,
    #[cfg(feature = "shared")]
    pub get_CurrentUpdatePercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CurrentUpdatePercentComplete: usize,
    #[cfg(feature = "shared")]
    pub get_PercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_PercentComplete: usize,
    #[cfg(feature = "shared")]
    pub GetUpdateResult: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetUpdateResult: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationProgress_Impl: super::IDispatch_Impl {
    fn get_CurrentUpdateIndex(&self) -> windows_core::Result<super::LONG>;
    fn get_CurrentUpdatePercentComplete(&self) -> windows_core::Result<super::LONG>;
    fn get_PercentComplete(&self) -> windows_core::Result<super::LONG>;
    fn GetUpdateResult(&self, updateindex: super::LONG) -> windows_core::Result<IUpdateInstallationResult>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationProgress_Vtbl {
    pub const fn new<Identity: IInstallationProgress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentUpdateIndex<Identity: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationProgress_Impl::get_CurrentUpdateIndex(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentUpdatePercentComplete<Identity: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationProgress_Impl::get_CurrentUpdatePercentComplete(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PercentComplete<Identity: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationProgress_Impl::get_PercentComplete(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updateindex: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationProgress_Impl::GetUpdateResult(this, core::mem::transmute_copy(&updateindex)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentUpdateIndex: get_CurrentUpdateIndex::<Identity, OFFSET>,
            get_CurrentUpdatePercentComplete: get_CurrentUpdatePercentComplete::<Identity, OFFSET>,
            get_PercentComplete: get_PercentComplete::<Identity, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationProgress as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IInstallationProgress {}
windows_core::imp::define_interface!(IInstallationProgressChangedCallback, IInstallationProgressChangedCallback_Vtbl, 0xe01402d5_f8da_43ba_a012_38894bd048f1);
windows_core::imp::interface_hierarchy!(IInstallationProgressChangedCallback, windows_core::IUnknown);
impl IInstallationProgressChangedCallback {
    #[cfg(feature = "oaidl")]
    pub unsafe fn Invoke<P0, P1>(&self, installationjob: P0, callbackargs: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IInstallationJob>,
        P1: windows_core::Param<IInstallationProgressChangedCallbackArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), installationjob.param().abi(), callbackargs.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationProgressChangedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    Invoke: usize,
}
#[cfg(feature = "oaidl")]
pub trait IInstallationProgressChangedCallback_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, installationjob: windows_core::Ref<IInstallationJob>, callbackargs: windows_core::Ref<IInstallationProgressChangedCallbackArgs>) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl IInstallationProgressChangedCallback_Vtbl {
    pub const fn new<Identity: IInstallationProgressChangedCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IInstallationProgressChangedCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, installationjob: *mut core::ffi::c_void, callbackargs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInstallationProgressChangedCallback_Impl::Invoke(this, core::mem::transmute_copy(&installationjob), core::mem::transmute_copy(&callbackargs)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationProgressChangedCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IInstallationProgressChangedCallback {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IInstallationProgressChangedCallbackArgs, IInstallationProgressChangedCallbackArgs_Vtbl, 0xe4f14e1e_689d_4218_a0b9_bc189c484a01);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IInstallationProgressChangedCallbackArgs {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IInstallationProgressChangedCallbackArgs, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IInstallationProgressChangedCallbackArgs {
    pub unsafe fn get_Progress(&self) -> windows_core::Result<IInstallationProgress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Progress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationProgressChangedCallbackArgs_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationProgressChangedCallbackArgs_Impl: super::IDispatch_Impl {
    fn get_Progress(&self) -> windows_core::Result<IInstallationProgress>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationProgressChangedCallbackArgs_Vtbl {
    pub const fn new<Identity: IInstallationProgressChangedCallbackArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Progress<Identity: IInstallationProgressChangedCallbackArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationProgressChangedCallbackArgs_Impl::get_Progress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), get_Progress: get_Progress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationProgressChangedCallbackArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IInstallationProgressChangedCallbackArgs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IInstallationResult, IInstallationResult_Vtbl, 0xa43c56d6_7451_48d4_af96_b6cd2d0d9b7a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IInstallationResult {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IInstallationResult, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IInstallationResult {
    #[cfg(feature = "shared")]
    pub unsafe fn get_HResult(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetUpdateResult(&self, updateindex: super::LONG) -> windows_core::Result<IUpdateInstallationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdateResult)(windows_core::Interface::as_raw(self), updateindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationResult_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_HResult: usize,
    #[cfg(feature = "wtypes")]
    pub get_RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RebootRequired: usize,
    pub get_ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetUpdateResult: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetUpdateResult: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationResult_Impl: super::IDispatch_Impl {
    fn get_HResult(&self) -> windows_core::Result<super::LONG>;
    fn get_RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_ResultCode(&self) -> windows_core::Result<OperationResultCode>;
    fn GetUpdateResult(&self, updateindex: super::LONG) -> windows_core::Result<IUpdateInstallationResult>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationResult_Vtbl {
    pub const fn new<Identity: IInstallationResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_HResult<Identity: IInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationResult_Impl::get_HResult(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RebootRequired<Identity: IInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationResult_Impl::get_RebootRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ResultCode<Identity: IInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationResult_Impl::get_ResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: IInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updateindex: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationResult_Impl::GetUpdateResult(this, core::mem::transmute_copy(&updateindex)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_HResult: get_HResult::<Identity, OFFSET>,
            get_RebootRequired: get_RebootRequired::<Identity, OFFSET>,
            get_ResultCode: get_ResultCode::<Identity, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationResult as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IInstallationResult {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IInvalidProductLicenseException, IInvalidProductLicenseException_Vtbl, 0xa37d00f5_7bb0_4953_b414_f9e98326f2e8);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IInvalidProductLicenseException {
    type Target = IUpdateException;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IInvalidProductLicenseException, windows_core::IUnknown, super::IDispatch, IUpdateException);
#[cfg(feature = "oaidl")]
impl IInvalidProductLicenseException {
    pub unsafe fn get_Product(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Product)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInvalidProductLicenseException_Vtbl {
    pub base__: IUpdateException_Vtbl,
    pub get_Product: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInvalidProductLicenseException_Impl: IUpdateException_Impl {
    fn get_Product(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInvalidProductLicenseException_Vtbl {
    pub const fn new<Identity: IInvalidProductLicenseException_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Product<Identity: IInvalidProductLicenseException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInvalidProductLicenseException_Impl::get_Product(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUpdateException_Vtbl::new::<Identity, OFFSET>(), get_Product: get_Product::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInvalidProductLicenseException as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateException as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IInvalidProductLicenseException {}
windows_core::imp::define_interface!(ISearchCompletedCallback, ISearchCompletedCallback_Vtbl, 0x88aee058_d4b0_4725_a2f1_814a67ae964c);
windows_core::imp::interface_hierarchy!(ISearchCompletedCallback, windows_core::IUnknown);
impl ISearchCompletedCallback {
    #[cfg(feature = "oaidl")]
    pub unsafe fn Invoke<P0, P1>(&self, searchjob: P0, callbackargs: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISearchJob>,
        P1: windows_core::Param<ISearchCompletedCallbackArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), searchjob.param().abi(), callbackargs.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchCompletedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    Invoke: usize,
}
#[cfg(feature = "oaidl")]
pub trait ISearchCompletedCallback_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, searchjob: windows_core::Ref<ISearchJob>, callbackargs: windows_core::Ref<ISearchCompletedCallbackArgs>) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl ISearchCompletedCallback_Vtbl {
    pub const fn new<Identity: ISearchCompletedCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: ISearchCompletedCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, searchjob: *mut core::ffi::c_void, callbackargs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchCompletedCallback_Impl::Invoke(this, core::mem::transmute_copy(&searchjob), core::mem::transmute_copy(&callbackargs)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCompletedCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for ISearchCompletedCallback {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISearchCompletedCallbackArgs, ISearchCompletedCallbackArgs_Vtbl, 0xa700a634_2850_4c47_938a_9e4b6e5af9a6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISearchCompletedCallbackArgs {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISearchCompletedCallbackArgs, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchCompletedCallbackArgs_Vtbl {
    pub base__: super::IDispatch_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISearchCompletedCallbackArgs_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISearchCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: ISearchCompletedCallbackArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCompletedCallbackArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISearchCompletedCallbackArgs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISearchJob, ISearchJob_Vtbl, 0x7366ea16_7a1a_4ea2_b042_973d3e9cd99b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISearchJob {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISearchJob, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISearchJob {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_AsyncState(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AsyncState)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsCompleted(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsCompleted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CleanUp(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CleanUp)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RequestAbort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestAbort)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchJob_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_AsyncState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_AsyncState: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsCompleted: usize,
    pub CleanUp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISearchJob_Impl: super::IDispatch_Impl {
    fn get_AsyncState(&self) -> windows_core::Result<super::VARIANT>;
    fn get_IsCompleted(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn CleanUp(&self) -> windows_core::Result<()>;
    fn RequestAbort(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISearchJob_Vtbl {
    pub const fn new<Identity: ISearchJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AsyncState<Identity: ISearchJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchJob_Impl::get_AsyncState(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsCompleted<Identity: ISearchJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchJob_Impl::get_IsCompleted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CleanUp<Identity: ISearchJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchJob_Impl::CleanUp(this).into()
            }
        }
        unsafe extern "system" fn RequestAbort<Identity: ISearchJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISearchJob_Impl::RequestAbort(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_AsyncState: get_AsyncState::<Identity, OFFSET>,
            get_IsCompleted: get_IsCompleted::<Identity, OFFSET>,
            CleanUp: CleanUp::<Identity, OFFSET>,
            RequestAbort: RequestAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchJob as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISearchJob {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISearchResult, ISearchResult_Vtbl, 0xd40cff62_e08c_4498_941a_01e25f0fd33c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISearchResult {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISearchResult, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISearchResult {
    pub unsafe fn get_ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_RootCategories(&self) -> windows_core::Result<ICategoryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RootCategories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Updates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Warnings(&self) -> windows_core::Result<IUpdateExceptionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Warnings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchResult_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
    pub get_RootCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Warnings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISearchResult_Impl: super::IDispatch_Impl {
    fn get_ResultCode(&self) -> windows_core::Result<OperationResultCode>;
    fn get_RootCategories(&self) -> windows_core::Result<ICategoryCollection>;
    fn get_Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn get_Warnings(&self) -> windows_core::Result<IUpdateExceptionCollection>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISearchResult_Vtbl {
    pub const fn new<Identity: ISearchResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ResultCode<Identity: ISearchResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchResult_Impl::get_ResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RootCategories<Identity: ISearchResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchResult_Impl::get_RootCategories(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Updates<Identity: ISearchResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchResult_Impl::get_Updates(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Warnings<Identity: ISearchResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchResult_Impl::get_Warnings(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ResultCode: get_ResultCode::<Identity, OFFSET>,
            get_RootCategories: get_RootCategories::<Identity, OFFSET>,
            get_Updates: get_Updates::<Identity, OFFSET>,
            get_Warnings: get_Warnings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchResult as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISearchResult {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IStringCollection, IStringCollection_Vtbl, 0xeff90582_2ddc_480f_a06d_60f3fbc362c3);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IStringCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IStringCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IStringCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_Item(&self, index: super::LONG, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Item)(windows_core::Interface::as_raw(self), index, core::mem::transmute_copy(value)) }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Add(&self, value: &windows_core::BSTR) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Copy(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Insert(&self, index: super::LONG, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), index, core::mem::transmute_copy(value)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveAt(&self, index: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IStringCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Item: usize,
    #[cfg(feature = "shared")]
    pub put_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    #[cfg(feature = "wtypes")]
    pub get_ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ReadOnly: usize,
    #[cfg(feature = "shared")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Insert: usize,
    #[cfg(feature = "shared")]
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveAt: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IStringCollection_Impl: super::IDispatch_Impl {
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<windows_core::BSTR>;
    fn put_Item(&self, index: super::LONG, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
    fn get_ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Add(&self, value: &windows_core::BSTR) -> windows_core::Result<super::LONG>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn Copy(&self) -> windows_core::Result<IStringCollection>;
    fn Insert(&self, index: super::LONG, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: super::LONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IStringCollection_Vtbl {
    pub const fn new<Identity: IStringCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Item<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStringCollection_Impl::put_Item(this, core::mem::transmute_copy(&index), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ReadOnly<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringCollection_Impl::get_ReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringCollection_Impl::Add(this, core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clear<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStringCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn Copy<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringCollection_Impl::Copy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Insert<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStringCollection_Impl::Insert(this, core::mem::transmute_copy(&index), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStringCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            put_Item: put_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            get_ReadOnly: get_ReadOnly::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStringCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IStringCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISystemInformation, ISystemInformation_Vtbl, 0xade87bf7_7b56_4275_8fab_b9b0e591844b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISystemInformation {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISystemInformation, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISystemInformation {
    pub unsafe fn get_OemHardwareSupportLink(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OemHardwareSupportLink)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISystemInformation_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_OemHardwareSupportLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RebootRequired: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISystemInformation_Impl: super::IDispatch_Impl {
    fn get_OemHardwareSupportLink(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISystemInformation_Vtbl {
    pub const fn new<Identity: ISystemInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_OemHardwareSupportLink<Identity: ISystemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemInformation_Impl::get_OemHardwareSupportLink(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RebootRequired<Identity: ISystemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemInformation_Impl::get_RebootRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_OemHardwareSupportLink: get_OemHardwareSupportLink::<Identity, OFFSET>,
            get_RebootRequired: get_RebootRequired::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemInformation as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISystemInformation {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdate, IUpdate_Vtbl, 0x6a92b07a_d821_4682_b423_5c805022cc4d);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdate {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdate, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdate {
    pub unsafe fn get_Title(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AutoSelectOnWebSites(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutoSelectOnWebSites)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_BundledUpdates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BundledUpdates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_CanRequireSource(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CanRequireSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Categories(&self) -> windows_core::Result<ICategoryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Categories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Deadline(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Deadline)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DeltaCompressedContentAvailable(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DeltaCompressedContentAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DeltaCompressedContentPreferred(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DeltaCompressedContentPreferred)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_EulaAccepted(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EulaAccepted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_EulaText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EulaText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_HandlerID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HandlerID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Identity(&self) -> windows_core::Result<IUpdateIdentity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Identity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Image(&self) -> windows_core::Result<IImageInformation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Image)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_InstallationBehavior(&self) -> windows_core::Result<IInstallationBehavior> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InstallationBehavior)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsBeta(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsBeta)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsDownloaded(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsDownloaded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsHidden(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsHidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_IsHidden(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IsHidden)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsInstalled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsInstalled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsMandatory(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsMandatory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsUninstallable(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsUninstallable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Languages(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Languages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LastDeploymentChangeTime(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastDeploymentChangeTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn get_MaxDownloadSize(&self) -> windows_core::Result<super::DECIMAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MaxDownloadSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn get_MinDownloadSize(&self) -> windows_core::Result<super::DECIMAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MinDownloadSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_MoreInfoUrls(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MoreInfoUrls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_MsrcSeverity(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MsrcSeverity)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_RecommendedCpuSpeed(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RecommendedCpuSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_RecommendedHardDiskSpace(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RecommendedHardDiskSpace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_RecommendedMemory(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RecommendedMemory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ReleaseNotes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReleaseNotes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_SecurityBulletinIDs(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SecurityBulletinIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_SupersededUpdateIDs(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupersededUpdateIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_SupportUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Type(&self) -> windows_core::Result<UpdateType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_UninstallationNotes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UninstallationNotes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_UninstallationBehavior(&self) -> windows_core::Result<IInstallationBehavior> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UninstallationBehavior)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_UninstallationSteps(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UninstallationSteps)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_KBArticleIDs(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KBArticleIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AcceptEula(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcceptEula)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_DeploymentAction(&self) -> windows_core::Result<DeploymentAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DeploymentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CopyFromCache(&self, path: &windows_core::BSTR, toextractcabfiles: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CopyFromCache)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), toextractcabfiles) }
    }
    pub unsafe fn get_DownloadPriority(&self) -> windows_core::Result<DownloadPriority> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DownloadPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_DownloadContents(&self) -> windows_core::Result<IUpdateDownloadContentCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DownloadContents)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_AutoSelectOnWebSites: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AutoSelectOnWebSites: usize,
    pub get_BundledUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_CanRequireSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_CanRequireSource: usize,
    pub get_Categories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Deadline: usize,
    #[cfg(feature = "wtypes")]
    pub get_DeltaCompressedContentAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DeltaCompressedContentAvailable: usize,
    #[cfg(feature = "wtypes")]
    pub get_DeltaCompressedContentPreferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DeltaCompressedContentPreferred: usize,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_EulaAccepted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_EulaAccepted: usize,
    pub get_EulaText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_HandlerID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Identity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_InstallationBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsBeta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsBeta: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsDownloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsDownloaded: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsHidden: usize,
    #[cfg(feature = "wtypes")]
    pub put_IsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_IsHidden: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsInstalled: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsMandatory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsMandatory: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsUninstallable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsUninstallable: usize,
    pub get_Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_LastDeploymentChangeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LastDeploymentChangeTime: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub get_MaxDownloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DECIMAL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    get_MaxDownloadSize: usize,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub get_MinDownloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DECIMAL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    get_MinDownloadSize: usize,
    pub get_MoreInfoUrls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MsrcSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_RecommendedCpuSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_RecommendedCpuSpeed: usize,
    #[cfg(feature = "shared")]
    pub get_RecommendedHardDiskSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_RecommendedHardDiskSpace: usize,
    #[cfg(feature = "shared")]
    pub get_RecommendedMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_RecommendedMemory: usize,
    pub get_ReleaseNotes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SecurityBulletinIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SupersededUpdateIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SupportUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UpdateType) -> windows_core::HRESULT,
    pub get_UninstallationNotes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_UninstallationBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_UninstallationSteps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_KBArticleIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptEula: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DeploymentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeploymentAction) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub CopyFromCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CopyFromCache: usize,
    pub get_DownloadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DownloadPriority) -> windows_core::HRESULT,
    pub get_DownloadContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdate_Impl: super::IDispatch_Impl {
    fn get_Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_AutoSelectOnWebSites(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_BundledUpdates(&self) -> windows_core::Result<IUpdateCollection>;
    fn get_CanRequireSource(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Categories(&self) -> windows_core::Result<ICategoryCollection>;
    fn get_Deadline(&self) -> windows_core::Result<super::VARIANT>;
    fn get_DeltaCompressedContentAvailable(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_DeltaCompressedContentPreferred(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_EulaAccepted(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_EulaText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_HandlerID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Identity(&self) -> windows_core::Result<IUpdateIdentity>;
    fn get_Image(&self) -> windows_core::Result<IImageInformation>;
    fn get_InstallationBehavior(&self) -> windows_core::Result<IInstallationBehavior>;
    fn get_IsBeta(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_IsDownloaded(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_IsHidden(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_IsHidden(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_IsInstalled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_IsMandatory(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_IsUninstallable(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Languages(&self) -> windows_core::Result<IStringCollection>;
    fn get_LastDeploymentChangeTime(&self) -> windows_core::Result<super::DATE>;
    fn get_MaxDownloadSize(&self) -> windows_core::Result<super::DECIMAL>;
    fn get_MinDownloadSize(&self) -> windows_core::Result<super::DECIMAL>;
    fn get_MoreInfoUrls(&self) -> windows_core::Result<IStringCollection>;
    fn get_MsrcSeverity(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_RecommendedCpuSpeed(&self) -> windows_core::Result<super::LONG>;
    fn get_RecommendedHardDiskSpace(&self) -> windows_core::Result<super::LONG>;
    fn get_RecommendedMemory(&self) -> windows_core::Result<super::LONG>;
    fn get_ReleaseNotes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_SecurityBulletinIDs(&self) -> windows_core::Result<IStringCollection>;
    fn get_SupersededUpdateIDs(&self) -> windows_core::Result<IStringCollection>;
    fn get_SupportUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Type(&self) -> windows_core::Result<UpdateType>;
    fn get_UninstallationNotes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_UninstallationBehavior(&self) -> windows_core::Result<IInstallationBehavior>;
    fn get_UninstallationSteps(&self) -> windows_core::Result<IStringCollection>;
    fn get_KBArticleIDs(&self) -> windows_core::Result<IStringCollection>;
    fn AcceptEula(&self) -> windows_core::Result<()>;
    fn get_DeploymentAction(&self) -> windows_core::Result<DeploymentAction>;
    fn CopyFromCache(&self, path: &windows_core::BSTR, toextractcabfiles: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_DownloadPriority(&self) -> windows_core::Result<DownloadPriority>;
    fn get_DownloadContents(&self) -> windows_core::Result<IUpdateDownloadContentCollection>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdate_Vtbl {
    pub const fn new<Identity: IUpdate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Title<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_Title(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AutoSelectOnWebSites<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_AutoSelectOnWebSites(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_BundledUpdates<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_BundledUpdates(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CanRequireSource<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_CanRequireSource(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Categories<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_Categories(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Deadline<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_Deadline(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DeltaCompressedContentAvailable<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_DeltaCompressedContentAvailable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DeltaCompressedContentPreferred<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_DeltaCompressedContentPreferred(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Description<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_EulaAccepted<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_EulaAccepted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_EulaText<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_EulaText(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HandlerID<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_HandlerID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Identity<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_Identity(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Image<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_Image(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_InstallationBehavior<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_InstallationBehavior(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsBeta<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_IsBeta(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsDownloaded<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_IsDownloaded(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsHidden<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_IsHidden(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IsHidden<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdate_Impl::put_IsHidden(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_IsInstalled<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_IsInstalled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsMandatory<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_IsMandatory(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsUninstallable<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_IsUninstallable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Languages<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_Languages(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastDeploymentChangeTime<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_LastDeploymentChangeTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_MaxDownloadSize<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DECIMAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_MaxDownloadSize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_MinDownloadSize<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DECIMAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_MinDownloadSize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_MoreInfoUrls<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_MoreInfoUrls(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_MsrcSeverity<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_MsrcSeverity(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RecommendedCpuSpeed<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_RecommendedCpuSpeed(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RecommendedHardDiskSpace<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_RecommendedHardDiskSpace(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RecommendedMemory<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_RecommendedMemory(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ReleaseNotes<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_ReleaseNotes(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SecurityBulletinIDs<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_SecurityBulletinIDs(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupersededUpdateIDs<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_SupersededUpdateIDs(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportUrl<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_SupportUrl(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Type<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UpdateType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_Type(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UninstallationNotes<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_UninstallationNotes(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UninstallationBehavior<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_UninstallationBehavior(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UninstallationSteps<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_UninstallationSteps(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_KBArticleIDs<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_KBArticleIDs(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AcceptEula<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdate_Impl::AcceptEula(this).into()
            }
        }
        unsafe extern "system" fn get_DeploymentAction<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut DeploymentAction) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_DeploymentAction(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyFromCache<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, toextractcabfiles: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdate_Impl::CopyFromCache(this, core::mem::transmute(&path), core::mem::transmute_copy(&toextractcabfiles)).into()
            }
        }
        unsafe extern "system" fn get_DownloadPriority<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut DownloadPriority) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_DownloadPriority(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DownloadContents<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::get_DownloadContents(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Title: get_Title::<Identity, OFFSET>,
            get_AutoSelectOnWebSites: get_AutoSelectOnWebSites::<Identity, OFFSET>,
            get_BundledUpdates: get_BundledUpdates::<Identity, OFFSET>,
            get_CanRequireSource: get_CanRequireSource::<Identity, OFFSET>,
            get_Categories: get_Categories::<Identity, OFFSET>,
            get_Deadline: get_Deadline::<Identity, OFFSET>,
            get_DeltaCompressedContentAvailable: get_DeltaCompressedContentAvailable::<Identity, OFFSET>,
            get_DeltaCompressedContentPreferred: get_DeltaCompressedContentPreferred::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            get_EulaAccepted: get_EulaAccepted::<Identity, OFFSET>,
            get_EulaText: get_EulaText::<Identity, OFFSET>,
            get_HandlerID: get_HandlerID::<Identity, OFFSET>,
            get_Identity: get_Identity::<Identity, OFFSET>,
            get_Image: get_Image::<Identity, OFFSET>,
            get_InstallationBehavior: get_InstallationBehavior::<Identity, OFFSET>,
            get_IsBeta: get_IsBeta::<Identity, OFFSET>,
            get_IsDownloaded: get_IsDownloaded::<Identity, OFFSET>,
            get_IsHidden: get_IsHidden::<Identity, OFFSET>,
            put_IsHidden: put_IsHidden::<Identity, OFFSET>,
            get_IsInstalled: get_IsInstalled::<Identity, OFFSET>,
            get_IsMandatory: get_IsMandatory::<Identity, OFFSET>,
            get_IsUninstallable: get_IsUninstallable::<Identity, OFFSET>,
            get_Languages: get_Languages::<Identity, OFFSET>,
            get_LastDeploymentChangeTime: get_LastDeploymentChangeTime::<Identity, OFFSET>,
            get_MaxDownloadSize: get_MaxDownloadSize::<Identity, OFFSET>,
            get_MinDownloadSize: get_MinDownloadSize::<Identity, OFFSET>,
            get_MoreInfoUrls: get_MoreInfoUrls::<Identity, OFFSET>,
            get_MsrcSeverity: get_MsrcSeverity::<Identity, OFFSET>,
            get_RecommendedCpuSpeed: get_RecommendedCpuSpeed::<Identity, OFFSET>,
            get_RecommendedHardDiskSpace: get_RecommendedHardDiskSpace::<Identity, OFFSET>,
            get_RecommendedMemory: get_RecommendedMemory::<Identity, OFFSET>,
            get_ReleaseNotes: get_ReleaseNotes::<Identity, OFFSET>,
            get_SecurityBulletinIDs: get_SecurityBulletinIDs::<Identity, OFFSET>,
            get_SupersededUpdateIDs: get_SupersededUpdateIDs::<Identity, OFFSET>,
            get_SupportUrl: get_SupportUrl::<Identity, OFFSET>,
            get_Type: get_Type::<Identity, OFFSET>,
            get_UninstallationNotes: get_UninstallationNotes::<Identity, OFFSET>,
            get_UninstallationBehavior: get_UninstallationBehavior::<Identity, OFFSET>,
            get_UninstallationSteps: get_UninstallationSteps::<Identity, OFFSET>,
            get_KBArticleIDs: get_KBArticleIDs::<Identity, OFFSET>,
            AcceptEula: AcceptEula::<Identity, OFFSET>,
            get_DeploymentAction: get_DeploymentAction::<Identity, OFFSET>,
            CopyFromCache: CopyFromCache::<Identity, OFFSET>,
            get_DownloadPriority: get_DownloadPriority::<Identity, OFFSET>,
            get_DownloadContents: get_DownloadContents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdate {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdate2, IUpdate2_Vtbl, 0x144fe9b0_d23d_4a8b_8634_fb4457533b7a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdate2 {
    type Target = IUpdate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdate2, windows_core::IUnknown, super::IDispatch, IUpdate);
#[cfg(feature = "oaidl")]
impl IUpdate2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsPresent(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsPresent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CveIDs(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CveIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CopyToCache<P0>(&self, pfiles: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStringCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyToCache)(windows_core::Interface::as_raw(self), pfiles.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate2_Vtbl {
    pub base__: IUpdate_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RebootRequired: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsPresent: usize,
    pub get_CveIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyToCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdate2_Impl: IUpdate_Impl {
    fn get_RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_IsPresent(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_CveIDs(&self) -> windows_core::Result<IStringCollection>;
    fn CopyToCache(&self, pfiles: windows_core::Ref<IStringCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdate2_Vtbl {
    pub const fn new<Identity: IUpdate2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_RebootRequired<Identity: IUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate2_Impl::get_RebootRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsPresent<Identity: IUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate2_Impl::get_IsPresent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CveIDs<Identity: IUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate2_Impl::get_CveIDs(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyToCache<Identity: IUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiles: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdate2_Impl::CopyToCache(this, core::mem::transmute_copy(&pfiles)).into()
            }
        }
        Self {
            base__: IUpdate_Vtbl::new::<Identity, OFFSET>(),
            get_RebootRequired: get_RebootRequired::<Identity, OFFSET>,
            get_IsPresent: get_IsPresent::<Identity, OFFSET>,
            get_CveIDs: get_CveIDs::<Identity, OFFSET>,
            CopyToCache: CopyToCache::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdate2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdate3, IUpdate3_Vtbl, 0x112eda6b_95b3_476f_9d90_aee82c6b8181);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdate3 {
    type Target = IUpdate2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdate3, windows_core::IUnknown, super::IDispatch, IUpdate, IUpdate2);
#[cfg(feature = "oaidl")]
impl IUpdate3 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_BrowseOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BrowseOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate3_Vtbl {
    pub base__: IUpdate2_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_BrowseOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_BrowseOnly: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdate3_Impl: IUpdate2_Impl {
    fn get_BrowseOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdate3_Vtbl {
    pub const fn new<Identity: IUpdate3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_BrowseOnly<Identity: IUpdate3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate3_Impl::get_BrowseOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUpdate2_Vtbl::new::<Identity, OFFSET>(), get_BrowseOnly: get_BrowseOnly::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IUpdate2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdate3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdate4, IUpdate4_Vtbl, 0x27e94b0d_5139_49a2_9a61_93522dc54652);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdate4 {
    type Target = IUpdate3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdate4, windows_core::IUnknown, super::IDispatch, IUpdate, IUpdate2, IUpdate3);
#[cfg(feature = "oaidl")]
impl IUpdate4 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_PerUser(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PerUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate4_Vtbl {
    pub base__: IUpdate3_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_PerUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_PerUser: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdate4_Impl: IUpdate3_Impl {
    fn get_PerUser(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdate4_Vtbl {
    pub const fn new<Identity: IUpdate4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_PerUser<Identity: IUpdate4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate4_Impl::get_PerUser(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUpdate3_Vtbl::new::<Identity, OFFSET>(), get_PerUser: get_PerUser::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate4 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IUpdate2 as windows_core::Interface>::IID || iid == &<IUpdate3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdate4 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdate5, IUpdate5_Vtbl, 0xc1c2f21a_d2f4_4902_b5c6_8a081c19a890);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdate5 {
    type Target = IUpdate4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdate5, windows_core::IUnknown, super::IDispatch, IUpdate, IUpdate2, IUpdate3, IUpdate4);
#[cfg(feature = "oaidl")]
impl IUpdate5 {
    pub unsafe fn get_AutoSelection(&self) -> windows_core::Result<AutoSelectionMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutoSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_AutoDownload(&self) -> windows_core::Result<AutoDownloadMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutoDownload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate5_Vtbl {
    pub base__: IUpdate4_Vtbl,
    pub get_AutoSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoSelectionMode) -> windows_core::HRESULT,
    pub get_AutoDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoDownloadMode) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdate5_Impl: IUpdate4_Impl {
    fn get_AutoSelection(&self) -> windows_core::Result<AutoSelectionMode>;
    fn get_AutoDownload(&self) -> windows_core::Result<AutoDownloadMode>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdate5_Vtbl {
    pub const fn new<Identity: IUpdate5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AutoSelection<Identity: IUpdate5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutoSelectionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate5_Impl::get_AutoSelection(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AutoDownload<Identity: IUpdate5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutoDownloadMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate5_Impl::get_AutoDownload(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUpdate4_Vtbl::new::<Identity, OFFSET>(),
            get_AutoSelection: get_AutoSelection::<Identity, OFFSET>,
            get_AutoDownload: get_AutoDownload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate5 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IUpdate2 as windows_core::Interface>::IID || iid == &<IUpdate3 as windows_core::Interface>::IID || iid == &<IUpdate4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdate5 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateCollection, IUpdateCollection_Vtbl, 0x07f7438c_7709_4ca5_b518_91279288134e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<IUpdate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_Item<P1>(&self, index: super::LONG, value: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUpdate>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Item)(windows_core::Interface::as_raw(self), index, value.param().abi()) }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Add<P0>(&self, value: P0) -> windows_core::Result<super::LONG>
    where
        P0: windows_core::Param<IUpdate>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Copy(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Insert<P1>(&self, index: super::LONG, value: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUpdate>,
    {
        unsafe { (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), index, value.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveAt(&self, index: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Item: usize,
    #[cfg(feature = "shared")]
    pub put_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    #[cfg(feature = "wtypes")]
    pub get_ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ReadOnly: usize,
    #[cfg(feature = "shared")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Insert: usize,
    #[cfg(feature = "shared")]
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveAt: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateCollection_Impl: super::IDispatch_Impl {
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<IUpdate>;
    fn put_Item(&self, index: super::LONG, value: windows_core::Ref<IUpdate>) -> windows_core::Result<()>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
    fn get_ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Add(&self, value: windows_core::Ref<IUpdate>) -> windows_core::Result<super::LONG>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn Copy(&self) -> windows_core::Result<IUpdateCollection>;
    fn Insert(&self, index: super::LONG, value: windows_core::Ref<IUpdate>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: super::LONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateCollection_Vtbl {
    pub const fn new<Identity: IUpdateCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Item<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateCollection_Impl::put_Item(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ReadOnly<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateCollection_Impl::get_ReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateCollection_Impl::Add(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clear<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn Copy<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateCollection_Impl::Copy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Insert<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateCollection_Impl::Insert(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            put_Item: put_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            get_ReadOnly: get_ReadOnly::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateDownloadContent, IUpdateDownloadContent_Vtbl, 0x54a2cb2d_9a0c_48b6_8a50_9abb69ee2d02);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateDownloadContent {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateDownloadContent, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateDownloadContent {
    pub unsafe fn get_DownloadUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DownloadUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloadContent_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_DownloadUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateDownloadContent_Impl: super::IDispatch_Impl {
    fn get_DownloadUrl(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateDownloadContent_Vtbl {
    pub const fn new<Identity: IUpdateDownloadContent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DownloadUrl<Identity: IUpdateDownloadContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadContent_Impl::get_DownloadUrl(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), get_DownloadUrl: get_DownloadUrl::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadContent as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateDownloadContent {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateDownloadContent2, IUpdateDownloadContent2_Vtbl, 0xc97ad11b_f257_420b_9d9f_377f733f6f68);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateDownloadContent2 {
    type Target = IUpdateDownloadContent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateDownloadContent2, windows_core::IUnknown, super::IDispatch, IUpdateDownloadContent);
#[cfg(feature = "oaidl")]
impl IUpdateDownloadContent2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsDeltaCompressedContent(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsDeltaCompressedContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloadContent2_Vtbl {
    pub base__: IUpdateDownloadContent_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_IsDeltaCompressedContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsDeltaCompressedContent: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateDownloadContent2_Impl: IUpdateDownloadContent_Impl {
    fn get_IsDeltaCompressedContent(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateDownloadContent2_Vtbl {
    pub const fn new<Identity: IUpdateDownloadContent2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_IsDeltaCompressedContent<Identity: IUpdateDownloadContent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadContent2_Impl::get_IsDeltaCompressedContent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUpdateDownloadContent_Vtbl::new::<Identity, OFFSET>(), get_IsDeltaCompressedContent: get_IsDeltaCompressedContent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadContent2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateDownloadContent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateDownloadContent2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateDownloadContentCollection, IUpdateDownloadContentCollection_Vtbl, 0xbc5513c8_b3b8_4bf7_a4d4_361c0d8c88ba);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateDownloadContentCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateDownloadContentCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateDownloadContentCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<IUpdateDownloadContent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloadContentCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateDownloadContentCollection_Impl: super::IDispatch_Impl {
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<IUpdateDownloadContent>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateDownloadContentCollection_Vtbl {
    pub const fn new<Identity: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadContentCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadContentCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadContentCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadContentCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateDownloadContentCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateDownloadResult, IUpdateDownloadResult_Vtbl, 0xbf99af76_b575_42ad_8aa4_33cbb5477af1);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateDownloadResult {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateDownloadResult, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateDownloadResult {
    #[cfg(feature = "shared")]
    pub unsafe fn get_HResult(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloadResult_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_HResult: usize,
    pub get_ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateDownloadResult_Impl: super::IDispatch_Impl {
    fn get_HResult(&self) -> windows_core::Result<super::LONG>;
    fn get_ResultCode(&self) -> windows_core::Result<OperationResultCode>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateDownloadResult_Vtbl {
    pub const fn new<Identity: IUpdateDownloadResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_HResult<Identity: IUpdateDownloadResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadResult_Impl::get_HResult(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ResultCode<Identity: IUpdateDownloadResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadResult_Impl::get_ResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_HResult: get_HResult::<Identity, OFFSET>,
            get_ResultCode: get_ResultCode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadResult as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateDownloadResult {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateDownloader, IUpdateDownloader_Vtbl, 0x68f1c6f9_7ecc_4666_a464_247fe12496c3);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateDownloader {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateDownloader, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateDownloader {
    pub unsafe fn get_ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClientApplicationID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsForced(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsForced)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_IsForced(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IsForced)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn get_Priority(&self) -> windows_core::Result<DownloadPriority> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Priority(&self, value: DownloadPriority) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Priority)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn get_Updates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_Updates<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUpdateCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Updates)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn BeginDownload<P0, P1>(&self, onprogresschanged: P0, oncompleted: P1, state: &super::VARIANT) -> windows_core::Result<IDownloadJob>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginDownload)(windows_core::Interface::as_raw(self), onprogresschanged.param().abi(), oncompleted.param().abi(), core::mem::transmute_copy(state), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Download(&self) -> windows_core::Result<IDownloadResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn EndDownload<P0>(&self, value: P0) -> windows_core::Result<IDownloadResult>
    where
        P0: windows_core::Param<IDownloadJob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndDownload)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloader_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsForced: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsForced: usize,
    #[cfg(feature = "wtypes")]
    pub put_IsForced: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_IsForced: usize,
    pub get_Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DownloadPriority) -> windows_core::HRESULT,
    pub put_Priority: unsafe extern "system" fn(*mut core::ffi::c_void, DownloadPriority) -> windows_core::HRESULT,
    pub get_Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub BeginDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    BeginDownload: usize,
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateDownloader_Impl: super::IDispatch_Impl {
    fn get_ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_IsForced(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_IsForced(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Priority(&self) -> windows_core::Result<DownloadPriority>;
    fn put_Priority(&self, value: DownloadPriority) -> windows_core::Result<()>;
    fn get_Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn put_Updates(&self, value: windows_core::Ref<IUpdateCollection>) -> windows_core::Result<()>;
    fn BeginDownload(&self, onprogresschanged: windows_core::Ref<windows_core::IUnknown>, oncompleted: windows_core::Ref<windows_core::IUnknown>, state: &super::VARIANT) -> windows_core::Result<IDownloadJob>;
    fn Download(&self) -> windows_core::Result<IDownloadResult>;
    fn EndDownload(&self, value: windows_core::Ref<IDownloadJob>) -> windows_core::Result<IDownloadResult>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateDownloader_Vtbl {
    pub const fn new<Identity: IUpdateDownloader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ClientApplicationID<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloader_Impl::get_ClientApplicationID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClientApplicationID<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateDownloader_Impl::put_ClientApplicationID(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_IsForced<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloader_Impl::get_IsForced(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IsForced<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateDownloader_Impl::put_IsForced(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_Priority<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut DownloadPriority) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloader_Impl::get_Priority(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Priority<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: DownloadPriority) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateDownloader_Impl::put_Priority(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_Updates<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloader_Impl::get_Updates(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Updates<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateDownloader_Impl::put_Updates(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BeginDownload<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, onprogresschanged: *mut core::ffi::c_void, oncompleted: *mut core::ffi::c_void, state: super::VARIANT, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloader_Impl::BeginDownload(this, core::mem::transmute_copy(&onprogresschanged), core::mem::transmute_copy(&oncompleted), core::mem::transmute(&state)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Download<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloader_Impl::Download(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndDownload<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloader_Impl::EndDownload(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ClientApplicationID: get_ClientApplicationID::<Identity, OFFSET>,
            put_ClientApplicationID: put_ClientApplicationID::<Identity, OFFSET>,
            get_IsForced: get_IsForced::<Identity, OFFSET>,
            put_IsForced: put_IsForced::<Identity, OFFSET>,
            get_Priority: get_Priority::<Identity, OFFSET>,
            put_Priority: put_Priority::<Identity, OFFSET>,
            get_Updates: get_Updates::<Identity, OFFSET>,
            put_Updates: put_Updates::<Identity, OFFSET>,
            BeginDownload: BeginDownload::<Identity, OFFSET>,
            Download: Download::<Identity, OFFSET>,
            EndDownload: EndDownload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloader as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateDownloader {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateDownloaderEx, IUpdateDownloaderEx_Vtbl, 0x94726306_f12a_482a_a774_fb4f870d98c0);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateDownloaderEx {
    type Target = IUpdateDownloader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateDownloaderEx, windows_core::IUnknown, super::IDispatch, IUpdateDownloader);
#[cfg(feature = "oaidl")]
impl IUpdateDownloaderEx {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn BeginDownload2<P1, P2>(&self, downloadtype: DownloadType, onprogresschanged: P1, oncompleted: P2, state: &super::VARIANT) -> windows_core::Result<IDownloadJob>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginDownload2)(windows_core::Interface::as_raw(self), downloadtype, onprogresschanged.param().abi(), oncompleted.param().abi(), core::mem::transmute_copy(state), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Download2(&self, downloadtype: DownloadType) -> windows_core::Result<IDownloadResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Download2)(windows_core::Interface::as_raw(self), downloadtype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloaderEx_Vtbl {
    pub base__: IUpdateDownloader_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub BeginDownload2: unsafe extern "system" fn(*mut core::ffi::c_void, DownloadType, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    BeginDownload2: usize,
    pub Download2: unsafe extern "system" fn(*mut core::ffi::c_void, DownloadType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateDownloaderEx_Impl: IUpdateDownloader_Impl {
    fn BeginDownload2(&self, downloadtype: DownloadType, onprogresschanged: windows_core::Ref<windows_core::IUnknown>, oncompleted: windows_core::Ref<windows_core::IUnknown>, state: &super::VARIANT) -> windows_core::Result<IDownloadJob>;
    fn Download2(&self, downloadtype: DownloadType) -> windows_core::Result<IDownloadResult>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateDownloaderEx_Vtbl {
    pub const fn new<Identity: IUpdateDownloaderEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginDownload2<Identity: IUpdateDownloaderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadtype: DownloadType, onprogresschanged: *mut core::ffi::c_void, oncompleted: *mut core::ffi::c_void, state: super::VARIANT, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloaderEx_Impl::BeginDownload2(this, core::mem::transmute_copy(&downloadtype), core::mem::transmute_copy(&onprogresschanged), core::mem::transmute_copy(&oncompleted), core::mem::transmute(&state)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Download2<Identity: IUpdateDownloaderEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadtype: DownloadType, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloaderEx_Impl::Download2(this, core::mem::transmute_copy(&downloadtype)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUpdateDownloader_Vtbl::new::<Identity, OFFSET>(),
            BeginDownload2: BeginDownload2::<Identity, OFFSET>,
            Download2: Download2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloaderEx as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateDownloader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateDownloaderEx {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateEx, IUpdateEx_Vtbl, 0x769355a3_c5a0_497c_a606_560a36d2121c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateEx {
    type Target = IUpdate5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateEx, windows_core::IUnknown, super::IDispatch, IUpdate, IUpdate2, IUpdate3, IUpdate4, IUpdate5);
#[cfg(feature = "oaidl")]
impl IUpdateEx {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_ExtendedStaticProperty(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExtendedStaticProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn EvaluateExtendedDynamicProperty(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EvaluateExtendedDynamicProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateEx_Vtbl {
    pub base__: IUpdate5_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_ExtendedStaticProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_ExtendedStaticProperty: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub EvaluateExtendedDynamicProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    EvaluateExtendedDynamicProperty: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateEx_Impl: IUpdate5_Impl {
    fn get_ExtendedStaticProperty(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT>;
    fn EvaluateExtendedDynamicProperty(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateEx_Vtbl {
    pub const fn new<Identity: IUpdateEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ExtendedStaticProperty<Identity: IUpdateEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateEx_Impl::get_ExtendedStaticProperty(this, core::mem::transmute(&propertyname)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EvaluateExtendedDynamicProperty<Identity: IUpdateEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateEx_Impl::EvaluateExtendedDynamicProperty(this, core::mem::transmute(&propertyname)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUpdate5_Vtbl::new::<Identity, OFFSET>(),
            get_ExtendedStaticProperty: get_ExtendedStaticProperty::<Identity, OFFSET>,
            EvaluateExtendedDynamicProperty: EvaluateExtendedDynamicProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateEx as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IUpdate2 as windows_core::Interface>::IID || iid == &<IUpdate3 as windows_core::Interface>::IID || iid == &<IUpdate4 as windows_core::Interface>::IID || iid == &<IUpdate5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateEx {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateException, IUpdateException_Vtbl, 0xa376dd5e_09d4_427f_af7c_fed5b6e1c1d6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateException {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateException, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateException {
    pub unsafe fn get_Message(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_HResult(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Context(&self) -> windows_core::Result<UpdateExceptionContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Context)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateException_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_HResult: usize,
    pub get_Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UpdateExceptionContext) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateException_Impl: super::IDispatch_Impl {
    fn get_Message(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_HResult(&self) -> windows_core::Result<super::LONG>;
    fn get_Context(&self) -> windows_core::Result<UpdateExceptionContext>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateException_Vtbl {
    pub const fn new<Identity: IUpdateException_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Message<Identity: IUpdateException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateException_Impl::get_Message(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HResult<Identity: IUpdateException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateException_Impl::get_HResult(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Context<Identity: IUpdateException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UpdateExceptionContext) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateException_Impl::get_Context(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Message: get_Message::<Identity, OFFSET>,
            get_HResult: get_HResult::<Identity, OFFSET>,
            get_Context: get_Context::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateException as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateException {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateExceptionCollection, IUpdateExceptionCollection_Vtbl, 0x503626a3_8e14_4729_9355_0fe664bd2321);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateExceptionCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateExceptionCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateExceptionCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<IUpdateException> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateExceptionCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateExceptionCollection_Impl: super::IDispatch_Impl {
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<IUpdateException>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateExceptionCollection_Vtbl {
    pub const fn new<Identity: IUpdateExceptionCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateExceptionCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateExceptionCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateExceptionCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateExceptionCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateExceptionCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateHistoryEntry, IUpdateHistoryEntry_Vtbl, 0xbe56a644_af0e_4e0e_a311_c1d8e695cbff);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateHistoryEntry {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateHistoryEntry, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateHistoryEntry {
    pub unsafe fn get_Operation(&self) -> windows_core::Result<UpdateOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Operation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_HResult(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Date(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Date)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_UpdateIdentity(&self) -> windows_core::Result<IUpdateIdentity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UpdateIdentity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Title(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_UnmappedResultCode(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UnmappedResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ServerSelection(&self) -> windows_core::Result<ServerSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServerSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServiceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_UninstallationSteps(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UninstallationSteps)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_UninstallationNotes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UninstallationNotes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_SupportUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateHistoryEntry_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UpdateOperation) -> windows_core::HRESULT,
    pub get_ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_HResult: usize,
    #[cfg(feature = "wtypes")]
    pub get_Date: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Date: usize,
    pub get_UpdateIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_UnmappedResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_UnmappedResultCode: usize,
    pub get_ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ServerSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ServerSelection) -> windows_core::HRESULT,
    pub get_ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_UninstallationSteps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_UninstallationNotes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SupportUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateHistoryEntry_Impl: super::IDispatch_Impl {
    fn get_Operation(&self) -> windows_core::Result<UpdateOperation>;
    fn get_ResultCode(&self) -> windows_core::Result<OperationResultCode>;
    fn get_HResult(&self) -> windows_core::Result<super::LONG>;
    fn get_Date(&self) -> windows_core::Result<super::DATE>;
    fn get_UpdateIdentity(&self) -> windows_core::Result<IUpdateIdentity>;
    fn get_Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_UnmappedResultCode(&self) -> windows_core::Result<super::LONG>;
    fn get_ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ServerSelection(&self) -> windows_core::Result<ServerSelection>;
    fn get_ServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_UninstallationSteps(&self) -> windows_core::Result<IStringCollection>;
    fn get_UninstallationNotes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_SupportUrl(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateHistoryEntry_Vtbl {
    pub const fn new<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Operation<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UpdateOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_Operation(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ResultCode<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_ResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HResult<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_HResult(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Date<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_Date(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UpdateIdentity<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_UpdateIdentity(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Title<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_Title(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Description<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UnmappedResultCode<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_UnmappedResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ClientApplicationID<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_ClientApplicationID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ServerSelection<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut ServerSelection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_ServerSelection(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ServiceID<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_ServiceID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UninstallationSteps<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_UninstallationSteps(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UninstallationNotes<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_UninstallationNotes(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportUrl<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::get_SupportUrl(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Operation: get_Operation::<Identity, OFFSET>,
            get_ResultCode: get_ResultCode::<Identity, OFFSET>,
            get_HResult: get_HResult::<Identity, OFFSET>,
            get_Date: get_Date::<Identity, OFFSET>,
            get_UpdateIdentity: get_UpdateIdentity::<Identity, OFFSET>,
            get_Title: get_Title::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            get_UnmappedResultCode: get_UnmappedResultCode::<Identity, OFFSET>,
            get_ClientApplicationID: get_ClientApplicationID::<Identity, OFFSET>,
            get_ServerSelection: get_ServerSelection::<Identity, OFFSET>,
            get_ServiceID: get_ServiceID::<Identity, OFFSET>,
            get_UninstallationSteps: get_UninstallationSteps::<Identity, OFFSET>,
            get_UninstallationNotes: get_UninstallationNotes::<Identity, OFFSET>,
            get_SupportUrl: get_SupportUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateHistoryEntry as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateHistoryEntry {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateHistoryEntry2, IUpdateHistoryEntry2_Vtbl, 0xc2bfb780_4539_4132_ab8c_0a8772013ab6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateHistoryEntry2 {
    type Target = IUpdateHistoryEntry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateHistoryEntry2, windows_core::IUnknown, super::IDispatch, IUpdateHistoryEntry);
#[cfg(feature = "oaidl")]
impl IUpdateHistoryEntry2 {
    pub unsafe fn get_Categories(&self) -> windows_core::Result<ICategoryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Categories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateHistoryEntry2_Vtbl {
    pub base__: IUpdateHistoryEntry_Vtbl,
    pub get_Categories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateHistoryEntry2_Impl: IUpdateHistoryEntry_Impl {
    fn get_Categories(&self) -> windows_core::Result<ICategoryCollection>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateHistoryEntry2_Vtbl {
    pub const fn new<Identity: IUpdateHistoryEntry2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Categories<Identity: IUpdateHistoryEntry2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry2_Impl::get_Categories(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUpdateHistoryEntry_Vtbl::new::<Identity, OFFSET>(), get_Categories: get_Categories::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateHistoryEntry2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateHistoryEntry as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateHistoryEntry2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateHistoryEntryCollection, IUpdateHistoryEntryCollection_Vtbl, 0xa7f04f3c_a290_435b_aadf_a116c3357a5c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateHistoryEntryCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateHistoryEntryCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateHistoryEntryCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<IUpdateHistoryEntry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateHistoryEntryCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateHistoryEntryCollection_Impl: super::IDispatch_Impl {
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<IUpdateHistoryEntry>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateHistoryEntryCollection_Vtbl {
    pub const fn new<Identity: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntryCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntryCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntryCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateHistoryEntryCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateHistoryEntryCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateIdentity, IUpdateIdentity_Vtbl, 0x46297823_9940_4c09_aed9_cd3ea6d05968);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateIdentity {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateIdentity, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateIdentity {
    #[cfg(feature = "shared")]
    pub unsafe fn get_RevisionNumber(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RevisionNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_UpdateID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UpdateID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateIdentity_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_RevisionNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_RevisionNumber: usize,
    pub get_UpdateID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateIdentity_Impl: super::IDispatch_Impl {
    fn get_RevisionNumber(&self) -> windows_core::Result<super::LONG>;
    fn get_UpdateID(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateIdentity_Vtbl {
    pub const fn new<Identity: IUpdateIdentity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_RevisionNumber<Identity: IUpdateIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateIdentity_Impl::get_RevisionNumber(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UpdateID<Identity: IUpdateIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateIdentity_Impl::get_UpdateID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_RevisionNumber: get_RevisionNumber::<Identity, OFFSET>,
            get_UpdateID: get_UpdateID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateIdentity as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateIdentity {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateInstallationResult, IUpdateInstallationResult_Vtbl, 0xd940f0f8_3cbb_4fd0_993f_471e7f2328ad);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateInstallationResult {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateInstallationResult, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateInstallationResult {
    #[cfg(feature = "shared")]
    pub unsafe fn get_HResult(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstallationResult_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_HResult: usize,
    #[cfg(feature = "wtypes")]
    pub get_RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RebootRequired: usize,
    pub get_ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateInstallationResult_Impl: super::IDispatch_Impl {
    fn get_HResult(&self) -> windows_core::Result<super::LONG>;
    fn get_RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_ResultCode(&self) -> windows_core::Result<OperationResultCode>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateInstallationResult_Vtbl {
    pub const fn new<Identity: IUpdateInstallationResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_HResult<Identity: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallationResult_Impl::get_HResult(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RebootRequired<Identity: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallationResult_Impl::get_RebootRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ResultCode<Identity: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallationResult_Impl::get_ResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_HResult: get_HResult::<Identity, OFFSET>,
            get_RebootRequired: get_RebootRequired::<Identity, OFFSET>,
            get_ResultCode: get_ResultCode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstallationResult as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateInstallationResult {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateInstaller, IUpdateInstaller_Vtbl, 0x7b929c68_ccdc_4226_96b1_8724600b54c2);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateInstaller {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateInstaller, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateInstaller {
    pub unsafe fn get_ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClientApplicationID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsForced(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsForced)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_IsForced(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IsForced)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn get_ParentHwnd(&self) -> windows_core::Result<super::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ParentHwnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn put_ParentHwnd(&self, value: Option<super::HWND>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ParentHwnd)(windows_core::Interface::as_raw(self), value.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn put_ParentWindow<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_ParentWindow)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn get_ParentWindow(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ParentWindow)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Updates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_Updates<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUpdateCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Updates)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn BeginInstall<P0, P1>(&self, onprogresschanged: P0, oncompleted: P1, state: &super::VARIANT) -> windows_core::Result<IInstallationJob>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginInstall)(windows_core::Interface::as_raw(self), onprogresschanged.param().abi(), oncompleted.param().abi(), core::mem::transmute_copy(state), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn BeginUninstall<P0, P1>(&self, onprogresschanged: P0, oncompleted: P1, state: &super::VARIANT) -> windows_core::Result<IInstallationJob>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginUninstall)(windows_core::Interface::as_raw(self), onprogresschanged.param().abi(), oncompleted.param().abi(), core::mem::transmute_copy(state), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn EndInstall<P0>(&self, value: P0) -> windows_core::Result<IInstallationResult>
    where
        P0: windows_core::Param<IInstallationJob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndInstall)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn EndUninstall<P0>(&self, value: P0) -> windows_core::Result<IInstallationResult>
    where
        P0: windows_core::Param<IInstallationJob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndUninstall)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Install(&self) -> windows_core::Result<IInstallationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Install)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RunWizard(&self, dialogtitle: &windows_core::BSTR) -> windows_core::Result<IInstallationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RunWizard)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(dialogtitle), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsBusy(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsBusy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Uninstall(&self) -> windows_core::Result<IInstallationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uninstall)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AllowSourcePrompts(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowSourcePrompts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowSourcePrompts(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowSourcePrompts)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RebootRequiredBeforeInstallation(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RebootRequiredBeforeInstallation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsForced: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsForced: usize,
    #[cfg(feature = "wtypes")]
    pub put_IsForced: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_IsForced: usize,
    #[cfg(feature = "windef")]
    pub get_ParentHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    get_ParentHwnd: usize,
    #[cfg(feature = "windef")]
    pub put_ParentHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    put_ParentHwnd: usize,
    pub put_ParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub BeginInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    BeginInstall: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub BeginUninstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    BeginUninstall: usize,
    pub EndInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndUninstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Install: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunWizard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsBusy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsBusy: usize,
    pub Uninstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_AllowSourcePrompts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowSourcePrompts: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowSourcePrompts: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowSourcePrompts: usize,
    #[cfg(feature = "wtypes")]
    pub get_RebootRequiredBeforeInstallation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RebootRequiredBeforeInstallation: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateInstaller_Impl: super::IDispatch_Impl {
    fn get_ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_IsForced(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_IsForced(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ParentHwnd(&self) -> windows_core::Result<super::HWND>;
    fn put_ParentHwnd(&self, value: super::HWND) -> windows_core::Result<()>;
    fn put_ParentWindow(&self, value: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn get_ParentWindow(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn put_Updates(&self, value: windows_core::Ref<IUpdateCollection>) -> windows_core::Result<()>;
    fn BeginInstall(&self, onprogresschanged: windows_core::Ref<windows_core::IUnknown>, oncompleted: windows_core::Ref<windows_core::IUnknown>, state: &super::VARIANT) -> windows_core::Result<IInstallationJob>;
    fn BeginUninstall(&self, onprogresschanged: windows_core::Ref<windows_core::IUnknown>, oncompleted: windows_core::Ref<windows_core::IUnknown>, state: &super::VARIANT) -> windows_core::Result<IInstallationJob>;
    fn EndInstall(&self, value: windows_core::Ref<IInstallationJob>) -> windows_core::Result<IInstallationResult>;
    fn EndUninstall(&self, value: windows_core::Ref<IInstallationJob>) -> windows_core::Result<IInstallationResult>;
    fn Install(&self) -> windows_core::Result<IInstallationResult>;
    fn RunWizard(&self, dialogtitle: &windows_core::BSTR) -> windows_core::Result<IInstallationResult>;
    fn get_IsBusy(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Uninstall(&self) -> windows_core::Result<IInstallationResult>;
    fn get_AllowSourcePrompts(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowSourcePrompts(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_RebootRequiredBeforeInstallation(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateInstaller_Vtbl {
    pub const fn new<Identity: IUpdateInstaller_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ClientApplicationID<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::get_ClientApplicationID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClientApplicationID<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller_Impl::put_ClientApplicationID(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_IsForced<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::get_IsForced(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IsForced<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller_Impl::put_IsForced(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_ParentHwnd<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::get_ParentHwnd(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ParentHwnd<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller_Impl::put_ParentHwnd(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn put_ParentWindow<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller_Impl::put_ParentWindow(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_ParentWindow<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::get_ParentWindow(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Updates<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::get_Updates(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Updates<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller_Impl::put_Updates(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BeginInstall<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, onprogresschanged: *mut core::ffi::c_void, oncompleted: *mut core::ffi::c_void, state: super::VARIANT, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::BeginInstall(this, core::mem::transmute_copy(&onprogresschanged), core::mem::transmute_copy(&oncompleted), core::mem::transmute(&state)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginUninstall<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, onprogresschanged: *mut core::ffi::c_void, oncompleted: *mut core::ffi::c_void, state: super::VARIANT, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::BeginUninstall(this, core::mem::transmute_copy(&onprogresschanged), core::mem::transmute_copy(&oncompleted), core::mem::transmute(&state)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndInstall<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::EndInstall(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndUninstall<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::EndUninstall(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Install<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::Install(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RunWizard<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dialogtitle: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::RunWizard(this, core::mem::transmute(&dialogtitle)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsBusy<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::get_IsBusy(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Uninstall<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::Uninstall(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AllowSourcePrompts<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::get_AllowSourcePrompts(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowSourcePrompts<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller_Impl::put_AllowSourcePrompts(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_RebootRequiredBeforeInstallation<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::get_RebootRequiredBeforeInstallation(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ClientApplicationID: get_ClientApplicationID::<Identity, OFFSET>,
            put_ClientApplicationID: put_ClientApplicationID::<Identity, OFFSET>,
            get_IsForced: get_IsForced::<Identity, OFFSET>,
            put_IsForced: put_IsForced::<Identity, OFFSET>,
            get_ParentHwnd: get_ParentHwnd::<Identity, OFFSET>,
            put_ParentHwnd: put_ParentHwnd::<Identity, OFFSET>,
            put_ParentWindow: put_ParentWindow::<Identity, OFFSET>,
            get_ParentWindow: get_ParentWindow::<Identity, OFFSET>,
            get_Updates: get_Updates::<Identity, OFFSET>,
            put_Updates: put_Updates::<Identity, OFFSET>,
            BeginInstall: BeginInstall::<Identity, OFFSET>,
            BeginUninstall: BeginUninstall::<Identity, OFFSET>,
            EndInstall: EndInstall::<Identity, OFFSET>,
            EndUninstall: EndUninstall::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            RunWizard: RunWizard::<Identity, OFFSET>,
            get_IsBusy: get_IsBusy::<Identity, OFFSET>,
            Uninstall: Uninstall::<Identity, OFFSET>,
            get_AllowSourcePrompts: get_AllowSourcePrompts::<Identity, OFFSET>,
            put_AllowSourcePrompts: put_AllowSourcePrompts::<Identity, OFFSET>,
            get_RebootRequiredBeforeInstallation: get_RebootRequiredBeforeInstallation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateInstaller {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateInstaller2, IUpdateInstaller2_Vtbl, 0x3442d4fe_224d_4cee_98cf_30e0c4d229e6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateInstaller2 {
    type Target = IUpdateInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateInstaller2, windows_core::IUnknown, super::IDispatch, IUpdateInstaller);
#[cfg(feature = "oaidl")]
impl IUpdateInstaller2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ForceQuiet(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ForceQuiet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_ForceQuiet(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ForceQuiet)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller2_Vtbl {
    pub base__: IUpdateInstaller_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_ForceQuiet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ForceQuiet: usize,
    #[cfg(feature = "wtypes")]
    pub put_ForceQuiet: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_ForceQuiet: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateInstaller2_Impl: IUpdateInstaller_Impl {
    fn get_ForceQuiet(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_ForceQuiet(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateInstaller2_Vtbl {
    pub const fn new<Identity: IUpdateInstaller2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ForceQuiet<Identity: IUpdateInstaller2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller2_Impl::get_ForceQuiet(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ForceQuiet<Identity: IUpdateInstaller2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller2_Impl::put_ForceQuiet(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IUpdateInstaller_Vtbl::new::<Identity, OFFSET>(),
            get_ForceQuiet: get_ForceQuiet::<Identity, OFFSET>,
            put_ForceQuiet: put_ForceQuiet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateInstaller as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateInstaller2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateInstaller3, IUpdateInstaller3_Vtbl, 0x16d11c35_099a_48d0_8338_5fae64047f8e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateInstaller3 {
    type Target = IUpdateInstaller2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateInstaller3, windows_core::IUnknown, super::IDispatch, IUpdateInstaller, IUpdateInstaller2);
#[cfg(feature = "oaidl")]
impl IUpdateInstaller3 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AttemptCloseAppsIfNecessary(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AttemptCloseAppsIfNecessary)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AttemptCloseAppsIfNecessary(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AttemptCloseAppsIfNecessary)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller3_Vtbl {
    pub base__: IUpdateInstaller2_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_AttemptCloseAppsIfNecessary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AttemptCloseAppsIfNecessary: usize,
    #[cfg(feature = "wtypes")]
    pub put_AttemptCloseAppsIfNecessary: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AttemptCloseAppsIfNecessary: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateInstaller3_Impl: IUpdateInstaller2_Impl {
    fn get_AttemptCloseAppsIfNecessary(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AttemptCloseAppsIfNecessary(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateInstaller3_Vtbl {
    pub const fn new<Identity: IUpdateInstaller3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AttemptCloseAppsIfNecessary<Identity: IUpdateInstaller3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller3_Impl::get_AttemptCloseAppsIfNecessary(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AttemptCloseAppsIfNecessary<Identity: IUpdateInstaller3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller3_Impl::put_AttemptCloseAppsIfNecessary(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IUpdateInstaller2_Vtbl::new::<Identity, OFFSET>(),
            get_AttemptCloseAppsIfNecessary: get_AttemptCloseAppsIfNecessary::<Identity, OFFSET>,
            put_AttemptCloseAppsIfNecessary: put_AttemptCloseAppsIfNecessary::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateInstaller as windows_core::Interface>::IID || iid == &<IUpdateInstaller2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateInstaller3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateInstaller4, IUpdateInstaller4_Vtbl, 0xef8208ea_2304_492d_9109_23813b0958e1);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateInstaller4 {
    type Target = IUpdateInstaller3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateInstaller4, windows_core::IUnknown, super::IDispatch, IUpdateInstaller, IUpdateInstaller2, IUpdateInstaller3);
#[cfg(feature = "oaidl")]
impl IUpdateInstaller4 {
    #[cfg(feature = "shared")]
    pub unsafe fn Commit(&self, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), dwflags) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller4_Vtbl {
    pub base__: IUpdateInstaller3_Vtbl,
    #[cfg(feature = "shared")]
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Commit: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateInstaller4_Impl: IUpdateInstaller3_Impl {
    fn Commit(&self, dwflags: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateInstaller4_Vtbl {
    pub const fn new<Identity: IUpdateInstaller4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: IUpdateInstaller4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller4_Impl::Commit(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self { base__: IUpdateInstaller3_Vtbl::new::<Identity, OFFSET>(), Commit: Commit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller4 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateInstaller as windows_core::Interface>::IID || iid == &<IUpdateInstaller2 as windows_core::Interface>::IID || iid == &<IUpdateInstaller3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateInstaller4 {}
windows_core::imp::define_interface!(IUpdateLockdown, IUpdateLockdown_Vtbl, 0xa976c28d_75a1_42aa_94ae_8af8b872089a);
windows_core::imp::interface_hierarchy!(IUpdateLockdown, windows_core::IUnknown);
impl IUpdateLockdown {
    #[cfg(feature = "shared")]
    pub unsafe fn LockDown(&self, flags: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockDown)(windows_core::Interface::as_raw(self), flags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateLockdown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub LockDown: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    LockDown: usize,
}
#[cfg(feature = "shared")]
pub trait IUpdateLockdown_Impl: windows_core::IUnknownImpl {
    fn LockDown(&self, flags: super::LONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IUpdateLockdown_Vtbl {
    pub const fn new<Identity: IUpdateLockdown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LockDown<Identity: IUpdateLockdown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateLockdown_Impl::LockDown(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LockDown: LockDown::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateLockdown as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IUpdateLockdown {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateSearcher, IUpdateSearcher_Vtbl, 0x8f45abf1_f9ae_4b95_a933_f0f66e5056ea);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateSearcher {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateSearcher, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateSearcher {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_CanAutomaticallyUpgradeService(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CanAutomaticallyUpgradeService)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_CanAutomaticallyUpgradeService(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CanAutomaticallyUpgradeService)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn get_ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClientApplicationID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IncludePotentiallySupersededUpdates(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IncludePotentiallySupersededUpdates)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_IncludePotentiallySupersededUpdates(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IncludePotentiallySupersededUpdates)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn get_ServerSelection(&self) -> windows_core::Result<ServerSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServerSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ServerSelection(&self, value: ServerSelection) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ServerSelection)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn BeginSearch<P1>(&self, criteria: &windows_core::BSTR, oncompleted: P1, state: &super::VARIANT) -> windows_core::Result<ISearchJob>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginSearch)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(criteria), oncompleted.param().abi(), core::mem::transmute_copy(state), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn EndSearch<P0>(&self, searchjob: P0) -> windows_core::Result<ISearchResult>
    where
        P0: windows_core::Param<ISearchJob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndSearch)(windows_core::Interface::as_raw(self), searchjob.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn EscapeString(&self, unescaped: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EscapeString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(unescaped), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn QueryHistory(&self, startindex: super::LONG, count: super::LONG) -> windows_core::Result<IUpdateHistoryEntryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHistory)(windows_core::Interface::as_raw(self), startindex, count, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Search(&self, criteria: &windows_core::BSTR) -> windows_core::Result<ISearchResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Search)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(criteria), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Online(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Online)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Online(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Online)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTotalHistoryCount(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTotalHistoryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServiceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ServiceID(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ServiceID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSearcher_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_CanAutomaticallyUpgradeService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_CanAutomaticallyUpgradeService: usize,
    #[cfg(feature = "wtypes")]
    pub put_CanAutomaticallyUpgradeService: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_CanAutomaticallyUpgradeService: usize,
    pub get_ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IncludePotentiallySupersededUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IncludePotentiallySupersededUpdates: usize,
    #[cfg(feature = "wtypes")]
    pub put_IncludePotentiallySupersededUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_IncludePotentiallySupersededUpdates: usize,
    pub get_ServerSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ServerSelection) -> windows_core::HRESULT,
    pub put_ServerSelection: unsafe extern "system" fn(*mut core::ffi::c_void, ServerSelection) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub BeginSearch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    BeginSearch: usize,
    pub EndSearch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EscapeString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub QueryHistory: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    QueryHistory: usize,
    pub Search: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Online: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Online: usize,
    #[cfg(feature = "wtypes")]
    pub put_Online: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Online: usize,
    #[cfg(feature = "shared")]
    pub GetTotalHistoryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTotalHistoryCount: usize,
    pub get_ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateSearcher_Impl: super::IDispatch_Impl {
    fn get_CanAutomaticallyUpgradeService(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_CanAutomaticallyUpgradeService(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_IncludePotentiallySupersededUpdates(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_IncludePotentiallySupersededUpdates(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ServerSelection(&self) -> windows_core::Result<ServerSelection>;
    fn put_ServerSelection(&self, value: ServerSelection) -> windows_core::Result<()>;
    fn BeginSearch(&self, criteria: &windows_core::BSTR, oncompleted: windows_core::Ref<windows_core::IUnknown>, state: &super::VARIANT) -> windows_core::Result<ISearchJob>;
    fn EndSearch(&self, searchjob: windows_core::Ref<ISearchJob>) -> windows_core::Result<ISearchResult>;
    fn EscapeString(&self, unescaped: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn QueryHistory(&self, startindex: super::LONG, count: super::LONG) -> windows_core::Result<IUpdateHistoryEntryCollection>;
    fn Search(&self, criteria: &windows_core::BSTR) -> windows_core::Result<ISearchResult>;
    fn get_Online(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Online(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetTotalHistoryCount(&self) -> windows_core::Result<super::LONG>;
    fn get_ServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ServiceID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateSearcher_Vtbl {
    pub const fn new<Identity: IUpdateSearcher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CanAutomaticallyUpgradeService<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::get_CanAutomaticallyUpgradeService(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CanAutomaticallyUpgradeService<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher_Impl::put_CanAutomaticallyUpgradeService(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_ClientApplicationID<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::get_ClientApplicationID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClientApplicationID<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher_Impl::put_ClientApplicationID(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_IncludePotentiallySupersededUpdates<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::get_IncludePotentiallySupersededUpdates(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IncludePotentiallySupersededUpdates<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher_Impl::put_IncludePotentiallySupersededUpdates(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_ServerSelection<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut ServerSelection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::get_ServerSelection(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ServerSelection<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ServerSelection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher_Impl::put_ServerSelection(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BeginSearch<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, criteria: *mut core::ffi::c_void, oncompleted: *mut core::ffi::c_void, state: super::VARIANT, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::BeginSearch(this, core::mem::transmute(&criteria), core::mem::transmute_copy(&oncompleted), core::mem::transmute(&state)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndSearch<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, searchjob: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::EndSearch(this, core::mem::transmute_copy(&searchjob)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EscapeString<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unescaped: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::EscapeString(this, core::mem::transmute(&unescaped)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryHistory<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: super::LONG, count: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::QueryHistory(this, core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Search<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, criteria: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::Search(this, core::mem::transmute(&criteria)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Online<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::get_Online(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Online<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher_Impl::put_Online(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetTotalHistoryCount<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::GetTotalHistoryCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ServiceID<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::get_ServiceID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ServiceID<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher_Impl::put_ServiceID(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_CanAutomaticallyUpgradeService: get_CanAutomaticallyUpgradeService::<Identity, OFFSET>,
            put_CanAutomaticallyUpgradeService: put_CanAutomaticallyUpgradeService::<Identity, OFFSET>,
            get_ClientApplicationID: get_ClientApplicationID::<Identity, OFFSET>,
            put_ClientApplicationID: put_ClientApplicationID::<Identity, OFFSET>,
            get_IncludePotentiallySupersededUpdates: get_IncludePotentiallySupersededUpdates::<Identity, OFFSET>,
            put_IncludePotentiallySupersededUpdates: put_IncludePotentiallySupersededUpdates::<Identity, OFFSET>,
            get_ServerSelection: get_ServerSelection::<Identity, OFFSET>,
            put_ServerSelection: put_ServerSelection::<Identity, OFFSET>,
            BeginSearch: BeginSearch::<Identity, OFFSET>,
            EndSearch: EndSearch::<Identity, OFFSET>,
            EscapeString: EscapeString::<Identity, OFFSET>,
            QueryHistory: QueryHistory::<Identity, OFFSET>,
            Search: Search::<Identity, OFFSET>,
            get_Online: get_Online::<Identity, OFFSET>,
            put_Online: put_Online::<Identity, OFFSET>,
            GetTotalHistoryCount: GetTotalHistoryCount::<Identity, OFFSET>,
            get_ServiceID: get_ServiceID::<Identity, OFFSET>,
            put_ServiceID: put_ServiceID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSearcher as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateSearcher {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateSearcher2, IUpdateSearcher2_Vtbl, 0x4cbdcb2d_1589_4beb_bd1c_3e582ff0add0);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateSearcher2 {
    type Target = IUpdateSearcher;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateSearcher2, windows_core::IUnknown, super::IDispatch, IUpdateSearcher);
#[cfg(feature = "oaidl")]
impl IUpdateSearcher2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IgnoreDownloadPriority(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IgnoreDownloadPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_IgnoreDownloadPriority(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IgnoreDownloadPriority)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSearcher2_Vtbl {
    pub base__: IUpdateSearcher_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_IgnoreDownloadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IgnoreDownloadPriority: usize,
    #[cfg(feature = "wtypes")]
    pub put_IgnoreDownloadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_IgnoreDownloadPriority: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateSearcher2_Impl: IUpdateSearcher_Impl {
    fn get_IgnoreDownloadPriority(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_IgnoreDownloadPriority(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateSearcher2_Vtbl {
    pub const fn new<Identity: IUpdateSearcher2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_IgnoreDownloadPriority<Identity: IUpdateSearcher2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher2_Impl::get_IgnoreDownloadPriority(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IgnoreDownloadPriority<Identity: IUpdateSearcher2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher2_Impl::put_IgnoreDownloadPriority(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IUpdateSearcher_Vtbl::new::<Identity, OFFSET>(),
            get_IgnoreDownloadPriority: get_IgnoreDownloadPriority::<Identity, OFFSET>,
            put_IgnoreDownloadPriority: put_IgnoreDownloadPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSearcher2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateSearcher as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateSearcher2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateSearcher3, IUpdateSearcher3_Vtbl, 0x04c6895d_eaf2_4034_97f3_311de9be413a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateSearcher3 {
    type Target = IUpdateSearcher2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateSearcher3, windows_core::IUnknown, super::IDispatch, IUpdateSearcher, IUpdateSearcher2);
#[cfg(feature = "oaidl")]
impl IUpdateSearcher3 {
    pub unsafe fn get_SearchScope(&self) -> windows_core::Result<SearchScope> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SearchScope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SearchScope(&self, value: SearchScope) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SearchScope)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSearcher3_Vtbl {
    pub base__: IUpdateSearcher2_Vtbl,
    pub get_SearchScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SearchScope) -> windows_core::HRESULT,
    pub put_SearchScope: unsafe extern "system" fn(*mut core::ffi::c_void, SearchScope) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateSearcher3_Impl: IUpdateSearcher2_Impl {
    fn get_SearchScope(&self) -> windows_core::Result<SearchScope>;
    fn put_SearchScope(&self, value: SearchScope) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateSearcher3_Vtbl {
    pub const fn new<Identity: IUpdateSearcher3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_SearchScope<Identity: IUpdateSearcher3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut SearchScope) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher3_Impl::get_SearchScope(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SearchScope<Identity: IUpdateSearcher3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SearchScope) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher3_Impl::put_SearchScope(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IUpdateSearcher2_Vtbl::new::<Identity, OFFSET>(),
            get_SearchScope: get_SearchScope::<Identity, OFFSET>,
            put_SearchScope: put_SearchScope::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSearcher3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateSearcher as windows_core::Interface>::IID || iid == &<IUpdateSearcher2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateSearcher3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateService, IUpdateService_Vtbl, 0x76b3b17e_aed6_4da5_85f0_83587f81abe3);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateService {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateService, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateService {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_ContentValidationCert(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ContentValidationCert)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ExpirationDate(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExpirationDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsManaged(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsManaged)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsRegisteredWithAU(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsRegisteredWithAU)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IssueDate(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IssueDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_OffersWindowsUpdates(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OffersWindowsUpdates)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_RedirectUrls(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RedirectUrls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServiceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsScanPackageService(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsScanPackageService)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_CanRegisterWithAU(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CanRegisterWithAU)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ServiceUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServiceUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_SetupPrefix(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SetupPrefix)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateService_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_ContentValidationCert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_ContentValidationCert: usize,
    #[cfg(feature = "wtypes")]
    pub get_ExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ExpirationDate: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsManaged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsManaged: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsRegisteredWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsRegisteredWithAU: usize,
    #[cfg(feature = "wtypes")]
    pub get_IssueDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IssueDate: usize,
    #[cfg(feature = "wtypes")]
    pub get_OffersWindowsUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_OffersWindowsUpdates: usize,
    pub get_RedirectUrls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsScanPackageService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsScanPackageService: usize,
    #[cfg(feature = "wtypes")]
    pub get_CanRegisterWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_CanRegisterWithAU: usize,
    pub get_ServiceUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SetupPrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateService_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ContentValidationCert(&self) -> windows_core::Result<super::VARIANT>;
    fn get_ExpirationDate(&self) -> windows_core::Result<super::DATE>;
    fn get_IsManaged(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_IsRegisteredWithAU(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_IssueDate(&self) -> windows_core::Result<super::DATE>;
    fn get_OffersWindowsUpdates(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_RedirectUrls(&self) -> windows_core::Result<IStringCollection>;
    fn get_ServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_IsScanPackageService(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_CanRegisterWithAU(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_ServiceUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_SetupPrefix(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateService_Vtbl {
    pub const fn new<Identity: IUpdateService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_Name(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ContentValidationCert<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_ContentValidationCert(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ExpirationDate<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_ExpirationDate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsManaged<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_IsManaged(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsRegisteredWithAU<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_IsRegisteredWithAU(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IssueDate<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_IssueDate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_OffersWindowsUpdates<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_OffersWindowsUpdates(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RedirectUrls<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_RedirectUrls(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ServiceID<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_ServiceID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsScanPackageService<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_IsScanPackageService(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CanRegisterWithAU<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_CanRegisterWithAU(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ServiceUrl<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_ServiceUrl(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SetupPrefix<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::get_SetupPrefix(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            get_ContentValidationCert: get_ContentValidationCert::<Identity, OFFSET>,
            get_ExpirationDate: get_ExpirationDate::<Identity, OFFSET>,
            get_IsManaged: get_IsManaged::<Identity, OFFSET>,
            get_IsRegisteredWithAU: get_IsRegisteredWithAU::<Identity, OFFSET>,
            get_IssueDate: get_IssueDate::<Identity, OFFSET>,
            get_OffersWindowsUpdates: get_OffersWindowsUpdates::<Identity, OFFSET>,
            get_RedirectUrls: get_RedirectUrls::<Identity, OFFSET>,
            get_ServiceID: get_ServiceID::<Identity, OFFSET>,
            get_IsScanPackageService: get_IsScanPackageService::<Identity, OFFSET>,
            get_CanRegisterWithAU: get_CanRegisterWithAU::<Identity, OFFSET>,
            get_ServiceUrl: get_ServiceUrl::<Identity, OFFSET>,
            get_SetupPrefix: get_SetupPrefix::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateService as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateService {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateService2, IUpdateService2_Vtbl, 0x1518b460_6518_4172_940f_c75883b24ceb);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateService2 {
    type Target = IUpdateService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateService2, windows_core::IUnknown, super::IDispatch, IUpdateService);
#[cfg(feature = "oaidl")]
impl IUpdateService2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsDefaultAUService(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsDefaultAUService)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateService2_Vtbl {
    pub base__: IUpdateService_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_IsDefaultAUService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsDefaultAUService: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateService2_Impl: IUpdateService_Impl {
    fn get_IsDefaultAUService(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateService2_Vtbl {
    pub const fn new<Identity: IUpdateService2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_IsDefaultAUService<Identity: IUpdateService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService2_Impl::get_IsDefaultAUService(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUpdateService_Vtbl::new::<Identity, OFFSET>(), get_IsDefaultAUService: get_IsDefaultAUService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateService2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateService as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateService2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateServiceCollection, IUpdateServiceCollection_Vtbl, 0x9b0353aa_0e52_44ff_b8b0_1f7fa0437f88);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateServiceCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateServiceCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateServiceCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<IUpdateService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateServiceCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateServiceCollection_Impl: super::IDispatch_Impl {
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<IUpdateService>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateServiceCollection_Vtbl {
    pub const fn new<Identity: IUpdateServiceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateServiceCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateServiceCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateServiceManager, IUpdateServiceManager_Vtbl, 0x23857e3c_02ba_44a3_9423_b1c900805f37);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateServiceManager {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateServiceManager, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateServiceManager {
    pub unsafe fn get_Services(&self) -> windows_core::Result<IUpdateServiceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Services)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddService(&self, serviceid: &windows_core::BSTR, authorizationcabpath: &windows_core::BSTR) -> windows_core::Result<IUpdateService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddService)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serviceid), core::mem::transmute_copy(authorizationcabpath), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterServiceWithAU(&self, serviceid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterServiceWithAU)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serviceid)) }
    }
    pub unsafe fn RemoveService(&self, serviceid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveService)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serviceid)) }
    }
    pub unsafe fn UnregisterServiceWithAU(&self, serviceid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterServiceWithAU)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serviceid)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddScanPackageService(&self, servicename: &windows_core::BSTR, scanfilelocation: &windows_core::BSTR, flags: super::LONG) -> windows_core::Result<IUpdateService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddScanPackageService)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename), core::mem::transmute_copy(scanfilelocation), flags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetOption(&self, optionname: &windows_core::BSTR, optionvalue: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOption)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(optionname), core::mem::transmute_copy(optionvalue)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateServiceManager_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Services: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterServiceWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterServiceWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub AddScanPackageService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddScanPackageService: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetOption: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateServiceManager_Impl: super::IDispatch_Impl {
    fn get_Services(&self) -> windows_core::Result<IUpdateServiceCollection>;
    fn AddService(&self, serviceid: &windows_core::BSTR, authorizationcabpath: &windows_core::BSTR) -> windows_core::Result<IUpdateService>;
    fn RegisterServiceWithAU(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveService(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UnregisterServiceWithAU(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddScanPackageService(&self, servicename: &windows_core::BSTR, scanfilelocation: &windows_core::BSTR, flags: super::LONG) -> windows_core::Result<IUpdateService>;
    fn SetOption(&self, optionname: &windows_core::BSTR, optionvalue: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateServiceManager_Vtbl {
    pub const fn new<Identity: IUpdateServiceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Services<Identity: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceManager_Impl::get_Services(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddService<Identity: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: *mut core::ffi::c_void, authorizationcabpath: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceManager_Impl::AddService(this, core::mem::transmute(&serviceid), core::mem::transmute(&authorizationcabpath)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterServiceWithAU<Identity: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateServiceManager_Impl::RegisterServiceWithAU(this, core::mem::transmute(&serviceid)).into()
            }
        }
        unsafe extern "system" fn RemoveService<Identity: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateServiceManager_Impl::RemoveService(this, core::mem::transmute(&serviceid)).into()
            }
        }
        unsafe extern "system" fn UnregisterServiceWithAU<Identity: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateServiceManager_Impl::UnregisterServiceWithAU(this, core::mem::transmute(&serviceid)).into()
            }
        }
        unsafe extern "system" fn AddScanPackageService<Identity: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void, scanfilelocation: *mut core::ffi::c_void, flags: super::LONG, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceManager_Impl::AddScanPackageService(this, core::mem::transmute(&servicename), core::mem::transmute(&scanfilelocation), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppservice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOption<Identity: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionname: *mut core::ffi::c_void, optionvalue: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateServiceManager_Impl::SetOption(this, core::mem::transmute(&optionname), core::mem::transmute(&optionvalue)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Services: get_Services::<Identity, OFFSET>,
            AddService: AddService::<Identity, OFFSET>,
            RegisterServiceWithAU: RegisterServiceWithAU::<Identity, OFFSET>,
            RemoveService: RemoveService::<Identity, OFFSET>,
            UnregisterServiceWithAU: UnregisterServiceWithAU::<Identity, OFFSET>,
            AddScanPackageService: AddScanPackageService::<Identity, OFFSET>,
            SetOption: SetOption::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateServiceManager as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateServiceManager {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateServiceManager2, IUpdateServiceManager2_Vtbl, 0x0bb8531d_7e8d_424f_986c_a0b8f60a3e7b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateServiceManager2 {
    type Target = IUpdateServiceManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateServiceManager2, windows_core::IUnknown, super::IDispatch, IUpdateServiceManager);
#[cfg(feature = "oaidl")]
impl IUpdateServiceManager2 {
    pub unsafe fn get_ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClientApplicationID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn QueryServiceRegistration(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<IUpdateServiceRegistration> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryServiceRegistration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serviceid), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddService2(&self, serviceid: &windows_core::BSTR, flags: super::LONG, authorizationcabpath: &windows_core::BSTR) -> windows_core::Result<IUpdateServiceRegistration> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddService2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serviceid), flags, core::mem::transmute_copy(authorizationcabpath), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateServiceManager2_Vtbl {
    pub base__: IUpdateServiceManager_Vtbl,
    pub get_ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryServiceRegistration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub AddService2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddService2: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateServiceManager2_Impl: IUpdateServiceManager_Impl {
    fn get_ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn QueryServiceRegistration(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<IUpdateServiceRegistration>;
    fn AddService2(&self, serviceid: &windows_core::BSTR, flags: super::LONG, authorizationcabpath: &windows_core::BSTR) -> windows_core::Result<IUpdateServiceRegistration>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateServiceManager2_Vtbl {
    pub const fn new<Identity: IUpdateServiceManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ClientApplicationID<Identity: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceManager2_Impl::get_ClientApplicationID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClientApplicationID<Identity: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateServiceManager2_Impl::put_ClientApplicationID(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn QueryServiceRegistration<Identity: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceManager2_Impl::QueryServiceRegistration(this, core::mem::transmute(&serviceid)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddService2<Identity: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: *mut core::ffi::c_void, flags: super::LONG, authorizationcabpath: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceManager2_Impl::AddService2(this, core::mem::transmute(&serviceid), core::mem::transmute_copy(&flags), core::mem::transmute(&authorizationcabpath)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUpdateServiceManager_Vtbl::new::<Identity, OFFSET>(),
            get_ClientApplicationID: get_ClientApplicationID::<Identity, OFFSET>,
            put_ClientApplicationID: put_ClientApplicationID::<Identity, OFFSET>,
            QueryServiceRegistration: QueryServiceRegistration::<Identity, OFFSET>,
            AddService2: AddService2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateServiceManager2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateServiceManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateServiceManager2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateServiceRegistration, IUpdateServiceRegistration_Vtbl, 0xdde02280_12b3_4e0b_937b_6747f6acb286);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateServiceRegistration {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateServiceRegistration, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateServiceRegistration {
    pub unsafe fn get_RegistrationState(&self) -> windows_core::Result<UpdateServiceRegistrationState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RegistrationState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServiceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsPendingRegistrationWithAU(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsPendingRegistrationWithAU)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Service(&self) -> windows_core::Result<IUpdateService2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Service)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateServiceRegistration_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_RegistrationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UpdateServiceRegistrationState) -> windows_core::HRESULT,
    pub get_ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsPendingRegistrationWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsPendingRegistrationWithAU: usize,
    pub get_Service: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateServiceRegistration_Impl: super::IDispatch_Impl {
    fn get_RegistrationState(&self) -> windows_core::Result<UpdateServiceRegistrationState>;
    fn get_ServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_IsPendingRegistrationWithAU(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Service(&self) -> windows_core::Result<IUpdateService2>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateServiceRegistration_Vtbl {
    pub const fn new<Identity: IUpdateServiceRegistration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_RegistrationState<Identity: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UpdateServiceRegistrationState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceRegistration_Impl::get_RegistrationState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ServiceID<Identity: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceRegistration_Impl::get_ServiceID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsPendingRegistrationWithAU<Identity: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceRegistration_Impl::get_IsPendingRegistrationWithAU(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Service<Identity: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceRegistration_Impl::get_Service(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_RegistrationState: get_RegistrationState::<Identity, OFFSET>,
            get_ServiceID: get_ServiceID::<Identity, OFFSET>,
            get_IsPendingRegistrationWithAU: get_IsPendingRegistrationWithAU::<Identity, OFFSET>,
            get_Service: get_Service::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateServiceRegistration as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateServiceRegistration {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateSession, IUpdateSession_Vtbl, 0x816858a4_260d_4260_933a_2585f1abc76b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateSession {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateSession, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IUpdateSession {
    pub unsafe fn get_ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClientApplicationID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_WebProxy(&self) -> windows_core::Result<IWebProxy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WebProxy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_WebProxy<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWebProxy>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_WebProxy)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn CreateUpdateSearcher(&self) -> windows_core::Result<IUpdateSearcher> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUpdateSearcher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateUpdateDownloader(&self) -> windows_core::Result<IUpdateDownloader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUpdateDownloader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateUpdateInstaller(&self) -> windows_core::Result<IUpdateInstaller> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUpdateInstaller)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSession_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ReadOnly: usize,
    pub get_WebProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_WebProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUpdateSearcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUpdateDownloader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUpdateInstaller: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateSession_Impl: super::IDispatch_Impl {
    fn get_ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_WebProxy(&self) -> windows_core::Result<IWebProxy>;
    fn put_WebProxy(&self, value: windows_core::Ref<IWebProxy>) -> windows_core::Result<()>;
    fn CreateUpdateSearcher(&self) -> windows_core::Result<IUpdateSearcher>;
    fn CreateUpdateDownloader(&self) -> windows_core::Result<IUpdateDownloader>;
    fn CreateUpdateInstaller(&self) -> windows_core::Result<IUpdateInstaller>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateSession_Vtbl {
    pub const fn new<Identity: IUpdateSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ClientApplicationID<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession_Impl::get_ClientApplicationID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClientApplicationID<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSession_Impl::put_ClientApplicationID(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_ReadOnly<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession_Impl::get_ReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_WebProxy<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession_Impl::get_WebProxy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_WebProxy<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSession_Impl::put_WebProxy(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn CreateUpdateSearcher<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession_Impl::CreateUpdateSearcher(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateUpdateDownloader<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession_Impl::CreateUpdateDownloader(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateUpdateInstaller<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession_Impl::CreateUpdateInstaller(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ClientApplicationID: get_ClientApplicationID::<Identity, OFFSET>,
            put_ClientApplicationID: put_ClientApplicationID::<Identity, OFFSET>,
            get_ReadOnly: get_ReadOnly::<Identity, OFFSET>,
            get_WebProxy: get_WebProxy::<Identity, OFFSET>,
            put_WebProxy: put_WebProxy::<Identity, OFFSET>,
            CreateUpdateSearcher: CreateUpdateSearcher::<Identity, OFFSET>,
            CreateUpdateDownloader: CreateUpdateDownloader::<Identity, OFFSET>,
            CreateUpdateInstaller: CreateUpdateInstaller::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSession as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateSession {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateSession2, IUpdateSession2_Vtbl, 0x91caf7b0_eb23_49ed_9937_c52d817f46f7);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateSession2 {
    type Target = IUpdateSession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateSession2, windows_core::IUnknown, super::IDispatch, IUpdateSession);
#[cfg(feature = "oaidl")]
impl IUpdateSession2 {
    #[cfg(feature = "shared")]
    pub unsafe fn get_UserLocale(&self) -> windows_core::Result<super::LCID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UserLocale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_UserLocale(&self, lcid: super::LCID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UserLocale)(windows_core::Interface::as_raw(self), lcid) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSession2_Vtbl {
    pub base__: IUpdateSession_Vtbl,
    #[cfg(feature = "shared")]
    pub get_UserLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_UserLocale: usize,
    #[cfg(feature = "shared")]
    pub put_UserLocale: unsafe extern "system" fn(*mut core::ffi::c_void, super::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_UserLocale: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateSession2_Impl: IUpdateSession_Impl {
    fn get_UserLocale(&self) -> windows_core::Result<super::LCID>;
    fn put_UserLocale(&self, lcid: super::LCID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateSession2_Vtbl {
    pub const fn new<Identity: IUpdateSession2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_UserLocale<Identity: IUpdateSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession2_Impl::get_UserLocale(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UserLocale<Identity: IUpdateSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: super::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSession2_Impl::put_UserLocale(this, core::mem::transmute_copy(&lcid)).into()
            }
        }
        Self {
            base__: IUpdateSession_Vtbl::new::<Identity, OFFSET>(),
            get_UserLocale: get_UserLocale::<Identity, OFFSET>,
            put_UserLocale: put_UserLocale::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSession2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateSession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateSession2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IUpdateSession3, IUpdateSession3_Vtbl, 0x918efd1e_b5d8_4c90_8540_aeb9bdc56f9d);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IUpdateSession3 {
    type Target = IUpdateSession2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IUpdateSession3, windows_core::IUnknown, super::IDispatch, IUpdateSession, IUpdateSession2);
#[cfg(feature = "oaidl")]
impl IUpdateSession3 {
    pub unsafe fn CreateUpdateServiceManager(&self) -> windows_core::Result<IUpdateServiceManager2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUpdateServiceManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn QueryHistory(&self, criteria: &windows_core::BSTR, startindex: super::LONG, count: super::LONG) -> windows_core::Result<IUpdateHistoryEntryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHistory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(criteria), startindex, count, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSession3_Vtbl {
    pub base__: IUpdateSession2_Vtbl,
    pub CreateUpdateServiceManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub QueryHistory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    QueryHistory: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateSession3_Impl: IUpdateSession2_Impl {
    fn CreateUpdateServiceManager(&self) -> windows_core::Result<IUpdateServiceManager2>;
    fn QueryHistory(&self, criteria: &windows_core::BSTR, startindex: super::LONG, count: super::LONG) -> windows_core::Result<IUpdateHistoryEntryCollection>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateSession3_Vtbl {
    pub const fn new<Identity: IUpdateSession3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateUpdateServiceManager<Identity: IUpdateSession3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession3_Impl::CreateUpdateServiceManager(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryHistory<Identity: IUpdateSession3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, criteria: *mut core::ffi::c_void, startindex: super::LONG, count: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession3_Impl::QueryHistory(this, core::mem::transmute(&criteria), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUpdateSession2_Vtbl::new::<Identity, OFFSET>(),
            CreateUpdateServiceManager: CreateUpdateServiceManager::<Identity, OFFSET>,
            QueryHistory: QueryHistory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSession3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateSession as windows_core::Interface>::IID || iid == &<IUpdateSession2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateSession3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWebProxy, IWebProxy_Vtbl, 0x174c81fe_aecd_4dae_b8a0_2c6318dd86a8);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWebProxy {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWebProxy, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWebProxy {
    pub unsafe fn get_Address(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Address)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Address(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Address)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn get_BypassList(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BypassList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_BypassList<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStringCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_BypassList)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_BypassProxyOnLocal(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BypassProxyOnLocal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_BypassProxyOnLocal(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BypassProxyOnLocal)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_UserName(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UserName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn SetPassword(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn PromptForCredentials<P0>(&self, parentwindow: P0, title: &windows_core::BSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).PromptForCredentials)(windows_core::Interface::as_raw(self), parentwindow.param().abi(), core::mem::transmute_copy(title)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn PromptForCredentialsFromHwnd(&self, parentwindow: Option<super::HWND>, title: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PromptForCredentialsFromHwnd)(windows_core::Interface::as_raw(self), parentwindow.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute_copy(title)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AutoDetect(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutoDetect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AutoDetect(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AutoDetect)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWebProxy_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_BypassList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_BypassList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_BypassProxyOnLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_BypassProxyOnLocal: usize,
    #[cfg(feature = "wtypes")]
    pub put_BypassProxyOnLocal: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_BypassProxyOnLocal: usize,
    #[cfg(feature = "wtypes")]
    pub get_ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ReadOnly: usize,
    pub get_UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PromptForCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub PromptForCredentialsFromHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    PromptForCredentialsFromHwnd: usize,
    #[cfg(feature = "wtypes")]
    pub get_AutoDetect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AutoDetect: usize,
    #[cfg(feature = "wtypes")]
    pub put_AutoDetect: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AutoDetect: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWebProxy_Impl: super::IDispatch_Impl {
    fn get_Address(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Address(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_BypassList(&self) -> windows_core::Result<IStringCollection>;
    fn put_BypassList(&self, value: windows_core::Ref<IStringCollection>) -> windows_core::Result<()>;
    fn get_BypassProxyOnLocal(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_BypassProxyOnLocal(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_UserName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetPassword(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PromptForCredentials(&self, parentwindow: windows_core::Ref<windows_core::IUnknown>, title: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PromptForCredentialsFromHwnd(&self, parentwindow: super::HWND, title: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_AutoDetect(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AutoDetect(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWebProxy_Vtbl {
    pub const fn new<Identity: IWebProxy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Address<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebProxy_Impl::get_Address(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Address<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::put_Address(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_BypassList<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebProxy_Impl::get_BypassList(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BypassList<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::put_BypassList(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_BypassProxyOnLocal<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebProxy_Impl::get_BypassProxyOnLocal(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BypassProxyOnLocal<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::put_BypassProxyOnLocal(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_ReadOnly<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebProxy_Impl::get_ReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UserName<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebProxy_Impl::get_UserName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UserName<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::put_UserName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn SetPassword<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::SetPassword(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn PromptForCredentials<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentwindow: *mut core::ffi::c_void, title: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::PromptForCredentials(this, core::mem::transmute_copy(&parentwindow), core::mem::transmute(&title)).into()
            }
        }
        unsafe extern "system" fn PromptForCredentialsFromHwnd<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentwindow: super::HWND, title: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::PromptForCredentialsFromHwnd(this, core::mem::transmute_copy(&parentwindow), core::mem::transmute(&title)).into()
            }
        }
        unsafe extern "system" fn get_AutoDetect<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebProxy_Impl::get_AutoDetect(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AutoDetect<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::put_AutoDetect(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Address: get_Address::<Identity, OFFSET>,
            put_Address: put_Address::<Identity, OFFSET>,
            get_BypassList: get_BypassList::<Identity, OFFSET>,
            put_BypassList: put_BypassList::<Identity, OFFSET>,
            get_BypassProxyOnLocal: get_BypassProxyOnLocal::<Identity, OFFSET>,
            put_BypassProxyOnLocal: put_BypassProxyOnLocal::<Identity, OFFSET>,
            get_ReadOnly: get_ReadOnly::<Identity, OFFSET>,
            get_UserName: get_UserName::<Identity, OFFSET>,
            put_UserName: put_UserName::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
            PromptForCredentials: PromptForCredentials::<Identity, OFFSET>,
            PromptForCredentialsFromHwnd: PromptForCredentialsFromHwnd::<Identity, OFFSET>,
            get_AutoDetect: get_AutoDetect::<Identity, OFFSET>,
            put_AutoDetect: put_AutoDetect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebProxy as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWebProxy {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWindowsDriverUpdate, IWindowsDriverUpdate_Vtbl, 0xb383cd1a_5ce9_4504_9f63_764b1236f191);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWindowsDriverUpdate {
    type Target = IUpdate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdate, windows_core::IUnknown, super::IDispatch, IUpdate);
#[cfg(feature = "oaidl")]
impl IWindowsDriverUpdate {
    pub unsafe fn get_DriverClass(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DriverClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DriverHardwareID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DriverHardwareID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DriverManufacturer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DriverManufacturer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DriverModel(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DriverModel)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DriverProvider(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DriverProvider)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DriverVerDate(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DriverVerDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_DeviceProblemNumber(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DeviceProblemNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_DeviceStatus(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DeviceStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate_Vtbl {
    pub base__: IUpdate_Vtbl,
    pub get_DriverClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DriverHardwareID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DriverManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DriverModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DriverProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_DriverVerDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DriverVerDate: usize,
    #[cfg(feature = "shared")]
    pub get_DeviceProblemNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_DeviceProblemNumber: usize,
    #[cfg(feature = "shared")]
    pub get_DeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_DeviceStatus: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdate_Impl: IUpdate_Impl {
    fn get_DriverClass(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DriverHardwareID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DriverManufacturer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DriverModel(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DriverProvider(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DriverVerDate(&self) -> windows_core::Result<super::DATE>;
    fn get_DeviceProblemNumber(&self) -> windows_core::Result<super::LONG>;
    fn get_DeviceStatus(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdate_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DriverClass<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::get_DriverClass(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DriverHardwareID<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::get_DriverHardwareID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DriverManufacturer<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::get_DriverManufacturer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DriverModel<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::get_DriverModel(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DriverProvider<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::get_DriverProvider(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DriverVerDate<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::get_DriverVerDate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DeviceProblemNumber<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::get_DeviceProblemNumber(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DeviceStatus<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::get_DeviceStatus(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUpdate_Vtbl::new::<Identity, OFFSET>(),
            get_DriverClass: get_DriverClass::<Identity, OFFSET>,
            get_DriverHardwareID: get_DriverHardwareID::<Identity, OFFSET>,
            get_DriverManufacturer: get_DriverManufacturer::<Identity, OFFSET>,
            get_DriverModel: get_DriverModel::<Identity, OFFSET>,
            get_DriverProvider: get_DriverProvider::<Identity, OFFSET>,
            get_DriverVerDate: get_DriverVerDate::<Identity, OFFSET>,
            get_DeviceProblemNumber: get_DeviceProblemNumber::<Identity, OFFSET>,
            get_DeviceStatus: get_DeviceStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWindowsDriverUpdate {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWindowsDriverUpdate2, IWindowsDriverUpdate2_Vtbl, 0x615c4269_7a48_43bd_96b7_bf6ca27d6c3e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWindowsDriverUpdate2 {
    type Target = IWindowsDriverUpdate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdate2, windows_core::IUnknown, super::IDispatch, IUpdate, IWindowsDriverUpdate);
#[cfg(feature = "oaidl")]
impl IWindowsDriverUpdate2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsPresent(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsPresent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CveIDs(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CveIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CopyToCache<P0>(&self, pfiles: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStringCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyToCache)(windows_core::Interface::as_raw(self), pfiles.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate2_Vtbl {
    pub base__: IWindowsDriverUpdate_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RebootRequired: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsPresent: usize,
    pub get_CveIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyToCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdate2_Impl: IWindowsDriverUpdate_Impl {
    fn get_RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_IsPresent(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_CveIDs(&self) -> windows_core::Result<IStringCollection>;
    fn CopyToCache(&self, pfiles: windows_core::Ref<IStringCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdate2_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdate2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_RebootRequired<Identity: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate2_Impl::get_RebootRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsPresent<Identity: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate2_Impl::get_IsPresent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CveIDs<Identity: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate2_Impl::get_CveIDs(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyToCache<Identity: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiles: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsDriverUpdate2_Impl::CopyToCache(this, core::mem::transmute_copy(&pfiles)).into()
            }
        }
        Self {
            base__: IWindowsDriverUpdate_Vtbl::new::<Identity, OFFSET>(),
            get_RebootRequired: get_RebootRequired::<Identity, OFFSET>,
            get_IsPresent: get_IsPresent::<Identity, OFFSET>,
            get_CveIDs: get_CveIDs::<Identity, OFFSET>,
            CopyToCache: CopyToCache::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWindowsDriverUpdate2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWindowsDriverUpdate3, IWindowsDriverUpdate3_Vtbl, 0x49ebd502_4a96_41bd_9e3e_4c5057f4250c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWindowsDriverUpdate3 {
    type Target = IWindowsDriverUpdate2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdate3, windows_core::IUnknown, super::IDispatch, IUpdate, IWindowsDriverUpdate, IWindowsDriverUpdate2);
#[cfg(feature = "oaidl")]
impl IWindowsDriverUpdate3 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_BrowseOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BrowseOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate3_Vtbl {
    pub base__: IWindowsDriverUpdate2_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_BrowseOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_BrowseOnly: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdate3_Impl: IWindowsDriverUpdate2_Impl {
    fn get_BrowseOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdate3_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdate3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_BrowseOnly<Identity: IWindowsDriverUpdate3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate3_Impl::get_BrowseOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWindowsDriverUpdate2_Vtbl::new::<Identity, OFFSET>(), get_BrowseOnly: get_BrowseOnly::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWindowsDriverUpdate3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWindowsDriverUpdate4, IWindowsDriverUpdate4_Vtbl, 0x004c6a2b_0c19_4c69_9f5c_a269b2560db9);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWindowsDriverUpdate4 {
    type Target = IWindowsDriverUpdate3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdate4, windows_core::IUnknown, super::IDispatch, IUpdate, IWindowsDriverUpdate, IWindowsDriverUpdate2, IWindowsDriverUpdate3);
#[cfg(feature = "oaidl")]
impl IWindowsDriverUpdate4 {
    pub unsafe fn get_WindowsDriverUpdateEntries(&self) -> windows_core::Result<IWindowsDriverUpdateEntryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WindowsDriverUpdateEntries)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_PerUser(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PerUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate4_Vtbl {
    pub base__: IWindowsDriverUpdate3_Vtbl,
    pub get_WindowsDriverUpdateEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_PerUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_PerUser: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdate4_Impl: IWindowsDriverUpdate3_Impl {
    fn get_WindowsDriverUpdateEntries(&self) -> windows_core::Result<IWindowsDriverUpdateEntryCollection>;
    fn get_PerUser(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdate4_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdate4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_WindowsDriverUpdateEntries<Identity: IWindowsDriverUpdate4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate4_Impl::get_WindowsDriverUpdateEntries(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PerUser<Identity: IWindowsDriverUpdate4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate4_Impl::get_PerUser(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWindowsDriverUpdate3_Vtbl::new::<Identity, OFFSET>(),
            get_WindowsDriverUpdateEntries: get_WindowsDriverUpdateEntries::<Identity, OFFSET>,
            get_PerUser: get_PerUser::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate4 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate2 as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWindowsDriverUpdate4 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWindowsDriverUpdate5, IWindowsDriverUpdate5_Vtbl, 0x70cf5c82_8642_42bb_9dbc_0cfd263c6c4f);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWindowsDriverUpdate5 {
    type Target = IWindowsDriverUpdate4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdate5, windows_core::IUnknown, super::IDispatch, IUpdate, IWindowsDriverUpdate, IWindowsDriverUpdate2, IWindowsDriverUpdate3, IWindowsDriverUpdate4);
#[cfg(feature = "oaidl")]
impl IWindowsDriverUpdate5 {
    pub unsafe fn get_AutoSelection(&self) -> windows_core::Result<AutoSelectionMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutoSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_AutoDownload(&self) -> windows_core::Result<AutoDownloadMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutoDownload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate5_Vtbl {
    pub base__: IWindowsDriverUpdate4_Vtbl,
    pub get_AutoSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoSelectionMode) -> windows_core::HRESULT,
    pub get_AutoDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoDownloadMode) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdate5_Impl: IWindowsDriverUpdate4_Impl {
    fn get_AutoSelection(&self) -> windows_core::Result<AutoSelectionMode>;
    fn get_AutoDownload(&self) -> windows_core::Result<AutoDownloadMode>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdate5_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdate5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AutoSelection<Identity: IWindowsDriverUpdate5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutoSelectionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate5_Impl::get_AutoSelection(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AutoDownload<Identity: IWindowsDriverUpdate5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutoDownloadMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate5_Impl::get_AutoDownload(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWindowsDriverUpdate4_Vtbl::new::<Identity, OFFSET>(),
            get_AutoSelection: get_AutoSelection::<Identity, OFFSET>,
            get_AutoDownload: get_AutoDownload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate5 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate2 as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate3 as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWindowsDriverUpdate5 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWindowsDriverUpdateEntry, IWindowsDriverUpdateEntry_Vtbl, 0xed8bfe40_a60b_42ea_9652_817dfcfa23ec);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWindowsDriverUpdateEntry {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdateEntry, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWindowsDriverUpdateEntry {
    pub unsafe fn get_DriverClass(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DriverClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DriverHardwareID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DriverHardwareID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DriverManufacturer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DriverManufacturer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DriverModel(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DriverModel)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DriverProvider(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DriverProvider)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DriverVerDate(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DriverVerDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_DeviceProblemNumber(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DeviceProblemNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_DeviceStatus(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DeviceStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdateEntry_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_DriverClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DriverHardwareID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DriverManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DriverModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DriverProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_DriverVerDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DriverVerDate: usize,
    #[cfg(feature = "shared")]
    pub get_DeviceProblemNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_DeviceProblemNumber: usize,
    #[cfg(feature = "shared")]
    pub get_DeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_DeviceStatus: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdateEntry_Impl: super::IDispatch_Impl {
    fn get_DriverClass(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DriverHardwareID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DriverManufacturer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DriverModel(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DriverProvider(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DriverVerDate(&self) -> windows_core::Result<super::DATE>;
    fn get_DeviceProblemNumber(&self) -> windows_core::Result<super::LONG>;
    fn get_DeviceStatus(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdateEntry_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DriverClass<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::get_DriverClass(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DriverHardwareID<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::get_DriverHardwareID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DriverManufacturer<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::get_DriverManufacturer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DriverModel<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::get_DriverModel(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DriverProvider<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::get_DriverProvider(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DriverVerDate<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::get_DriverVerDate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DeviceProblemNumber<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::get_DeviceProblemNumber(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DeviceStatus<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::get_DeviceStatus(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_DriverClass: get_DriverClass::<Identity, OFFSET>,
            get_DriverHardwareID: get_DriverHardwareID::<Identity, OFFSET>,
            get_DriverManufacturer: get_DriverManufacturer::<Identity, OFFSET>,
            get_DriverModel: get_DriverModel::<Identity, OFFSET>,
            get_DriverProvider: get_DriverProvider::<Identity, OFFSET>,
            get_DriverVerDate: get_DriverVerDate::<Identity, OFFSET>,
            get_DeviceProblemNumber: get_DeviceProblemNumber::<Identity, OFFSET>,
            get_DeviceStatus: get_DeviceStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdateEntry as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWindowsDriverUpdateEntry {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWindowsDriverUpdateEntryCollection, IWindowsDriverUpdateEntryCollection_Vtbl, 0x0d521700_a372_4bef_828b_3d00c10adebd);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWindowsDriverUpdateEntryCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdateEntryCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWindowsDriverUpdateEntryCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<IWindowsDriverUpdateEntry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdateEntryCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdateEntryCollection_Impl: super::IDispatch_Impl {
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<IWindowsDriverUpdateEntry>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdateEntryCollection_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntryCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntryCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntryCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdateEntryCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWindowsDriverUpdateEntryCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWindowsUpdateAgentInfo, IWindowsUpdateAgentInfo_Vtbl, 0x85713fa1_7796_4fa2_be3b_e2d6124dd373);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWindowsUpdateAgentInfo {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWindowsUpdateAgentInfo, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWindowsUpdateAgentInfo {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetInfo(&self, varinfoidentifier: &super::VARIANT) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varinfoidentifier), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAgentInfo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetInfo: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsUpdateAgentInfo_Impl: super::IDispatch_Impl {
    fn GetInfo(&self, varinfoidentifier: &super::VARIANT) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsUpdateAgentInfo_Vtbl {
    pub const fn new<Identity: IWindowsUpdateAgentInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInfo<Identity: IWindowsUpdateAgentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varinfoidentifier: super::VARIANT, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsUpdateAgentInfo_Impl::GetInfo(this, core::mem::transmute(&varinfoidentifier)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetInfo: GetInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsUpdateAgentInfo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWindowsUpdateAgentInfo {}
pub const InstallationAgent: windows_core::GUID = windows_core::GUID::from_u128(0x317e92fc_1679_46fd_a0b5_f08914dd8623);
pub type InstallationImpact = i32;
pub type InstallationRebootBehavior = i32;
pub const LIBID_WUApiLib: windows_core::GUID = windows_core::GUID::from_u128(0xb596cc9f_56e5_419e_a622_e01bb457431e);
pub type OperationResultCode = i32;
pub type SearchScope = i32;
pub type ServerSelection = i32;
pub const StringCollection: windows_core::GUID = windows_core::GUID::from_u128(0x72c97d74_7c3b_40ae_b77d_abdb22eba6fb);
pub const SystemInformation: windows_core::GUID = windows_core::GUID::from_u128(0xc01b9ba0_bea7_41ba_b604_d0a36f469133);
pub const UPDATE_LOCKDOWN_WEBSITE_ACCESS: i32 = 1;
pub const UpdateCollection: windows_core::GUID = windows_core::GUID::from_u128(0x13639463_00db_4646_803d_528026140d88);
pub const UpdateDownloader: windows_core::GUID = windows_core::GUID::from_u128(0x5baf654a_5a07_4264_a255_9ff54c7151e7);
pub type UpdateExceptionContext = i32;
pub const UpdateInstaller: windows_core::GUID = windows_core::GUID::from_u128(0xd2e0fe7f_d23e_48e1_93c0_6fa8cc346474);
pub type UpdateLockdownOption = i32;
pub type UpdateOperation = i32;
pub const UpdateSearcher: windows_core::GUID = windows_core::GUID::from_u128(0xb699e5e8_67ff_4177_88b0_3684a3388bfb);
pub const UpdateServiceManager: windows_core::GUID = windows_core::GUID::from_u128(0xf8d253d9_89a4_4daa_87b6_1168369f0b21);
pub type UpdateServiceOption = i32;
pub type UpdateServiceRegistrationState = i32;
pub const UpdateSession: windows_core::GUID = windows_core::GUID::from_u128(0x4cb43d7f_7eee_4906_8698_60da1c38f2fe);
pub type UpdateType = i32;
pub const WebProxy: windows_core::GUID = windows_core::GUID::from_u128(0x650503cf_9108_4ddc_a2ce_6c2341e1c582);
pub const WindowsUpdateAgentInfo: windows_core::GUID = windows_core::GUID::from_u128(0xc2e88c2f_6f5b_4aaa_894b_55c847ad3a2d);
pub const adAlwaysAutoDownload: AutoDownloadMode = 2;
pub const adLetWindowsUpdateDecide: AutoDownloadMode = 0;
pub const adNeverAutoDownload: AutoDownloadMode = 1;
pub const asAlwaysAutoSelect: AutoSelectionMode = 3;
pub const asAutoSelectIfDownloaded: AutoSelectionMode = 1;
pub const asLetWindowsUpdateDecide: AutoSelectionMode = 0;
pub const asNeverAutoSelect: AutoSelectionMode = 2;
pub const asfAllowOnlineRegistration: AddServiceFlag = 2;
pub const asfAllowPendingRegistration: AddServiceFlag = 1;
pub const asfRegisterServiceWithAU: AddServiceFlag = 4;
pub const aunlDisabled: AutomaticUpdatesNotificationLevel = 1;
pub const aunlNotConfigured: AutomaticUpdatesNotificationLevel = 0;
pub const aunlNotifyBeforeDownload: AutomaticUpdatesNotificationLevel = 2;
pub const aunlNotifyBeforeInstallation: AutomaticUpdatesNotificationLevel = 3;
pub const aunlScheduledInstallation: AutomaticUpdatesNotificationLevel = 4;
pub const auptDisableAutomaticUpdates: AutomaticUpdatesPermissionType = 2;
pub const auptSetFeaturedUpdatesEnabled: AutomaticUpdatesPermissionType = 4;
pub const auptSetIncludeRecommendedUpdates: AutomaticUpdatesPermissionType = 3;
pub const auptSetNonAdministratorsElevated: AutomaticUpdatesPermissionType = 5;
pub const auptSetNotificationLevel: AutomaticUpdatesPermissionType = 1;
pub const ausidEveryDay: AutomaticUpdatesScheduledInstallationDay = 0;
pub const ausidEveryFriday: AutomaticUpdatesScheduledInstallationDay = 6;
pub const ausidEveryMonday: AutomaticUpdatesScheduledInstallationDay = 2;
pub const ausidEverySaturday: AutomaticUpdatesScheduledInstallationDay = 7;
pub const ausidEverySunday: AutomaticUpdatesScheduledInstallationDay = 1;
pub const ausidEveryThursday: AutomaticUpdatesScheduledInstallationDay = 5;
pub const ausidEveryTuesday: AutomaticUpdatesScheduledInstallationDay = 3;
pub const ausidEveryWednesday: AutomaticUpdatesScheduledInstallationDay = 4;
pub const auutCurrentUser: AutomaticUpdatesUserType = 1;
pub const auutLocalAdministrator: AutomaticUpdatesUserType = 2;
pub const daDetection: DeploymentAction = 3;
pub const daInstallation: DeploymentAction = 1;
pub const daNone: DeploymentAction = 0;
pub const daOptionalInstallation: DeploymentAction = 4;
pub const daUninstallation: DeploymentAction = 2;
pub const downloadTypeFull: DownloadType = 0;
pub const downloadTypeUpdateBootstrapper: DownloadType = 1;
pub const dpExtraHigh: DownloadPriority = 4;
pub const dpHigh: DownloadPriority = 3;
pub const dpLow: DownloadPriority = 1;
pub const dpNormal: DownloadPriority = 2;
pub const dphDownloading: DownloadPhase = 2;
pub const dphInitializing: DownloadPhase = 1;
pub const dphVerifying: DownloadPhase = 3;
pub const iiMinor: InstallationImpact = 1;
pub const iiNormal: InstallationImpact = 0;
pub const iiRequiresExclusiveHandling: InstallationImpact = 2;
pub const irbAlwaysRequiresReboot: InstallationRebootBehavior = 1;
pub const irbCanRequestReboot: InstallationRebootBehavior = 2;
pub const irbNeverReboots: InstallationRebootBehavior = 0;
pub const orcAborted: OperationResultCode = 5;
pub const orcFailed: OperationResultCode = 4;
pub const orcInProgress: OperationResultCode = 1;
pub const orcNotStarted: OperationResultCode = 0;
pub const orcSucceeded: OperationResultCode = 2;
pub const orcSucceededWithErrors: OperationResultCode = 3;
pub const searchScopeAllUsers: SearchScope = 5;
pub const searchScopeCurrentUserOnly: SearchScope = 2;
pub const searchScopeDefault: SearchScope = 0;
pub const searchScopeMachineAndAllUsers: SearchScope = 4;
pub const searchScopeMachineAndCurrentUser: SearchScope = 3;
pub const searchScopeMachineOnly: SearchScope = 1;
pub const ssDefault: ServerSelection = 0;
pub const ssManagedServer: ServerSelection = 1;
pub const ssOthers: ServerSelection = 3;
pub const ssWindowsUpdate: ServerSelection = 2;
pub const uecGeneral: UpdateExceptionContext = 1;
pub const uecSearchIncomplete: UpdateExceptionContext = 4;
pub const uecWindowsDriver: UpdateExceptionContext = 2;
pub const uecWindowsInstaller: UpdateExceptionContext = 3;
pub const uloForWebsiteAccess: UpdateLockdownOption = 1;
pub const uoInstallation: UpdateOperation = 1;
pub const uoUninstallation: UpdateOperation = 2;
pub const usoNonVolatileService: UpdateServiceOption = 1;
pub const usrsNotRegistered: UpdateServiceRegistrationState = 1;
pub const usrsRegistered: UpdateServiceRegistrationState = 3;
pub const usrsRegistrationPending: UpdateServiceRegistrationState = 2;
pub const utDriver: UpdateType = 2;
pub const utSoftware: UpdateType = 1;

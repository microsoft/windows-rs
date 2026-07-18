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
    pub unsafe fn Settings(&self) -> windows_core::Result<IAutomaticUpdatesSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Settings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ServiceEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub ServiceEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ServiceEnabled: usize,
    pub EnableService: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAutomaticUpdates_Impl: super::IDispatch_Impl {
    fn DetectNow(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn ShowSettingsDialog(&self) -> windows_core::Result<()>;
    fn Settings(&self) -> windows_core::Result<IAutomaticUpdatesSettings>;
    fn ServiceEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn EnableService(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn Settings<Identity: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdates_Impl::Settings(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceEnabled<Identity: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdates_Impl::ServiceEnabled(this) {
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
            Settings: Settings::<Identity, OFFSET>,
            ServiceEnabled: ServiceEnabled::<Identity, OFFSET>,
            EnableService: EnableService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdates as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Results(&self) -> windows_core::Result<IAutomaticUpdatesResults> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Results)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdates2_Vtbl {
    pub base__: IAutomaticUpdates_Vtbl,
    pub Results: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAutomaticUpdates2_Impl: IAutomaticUpdates_Impl {
    fn Results(&self) -> windows_core::Result<IAutomaticUpdatesResults>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAutomaticUpdates2_Vtbl {
    pub const fn new<Identity: IAutomaticUpdates2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Results<Identity: IAutomaticUpdates2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdates2_Impl::Results(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IAutomaticUpdates_Vtbl::new::<Identity, OFFSET>(), Results: Results::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdates2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IAutomaticUpdates as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn LastSearchSuccessDate(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastSearchSuccessDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn LastInstallationSuccessDate(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastInstallationSuccessDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdatesResults_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub LastSearchSuccessDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    LastSearchSuccessDate: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub LastInstallationSuccessDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    LastInstallationSuccessDate: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAutomaticUpdatesResults_Impl: super::IDispatch_Impl {
    fn LastSearchSuccessDate(&self) -> windows_core::Result<super::VARIANT>;
    fn LastInstallationSuccessDate(&self) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAutomaticUpdatesResults_Vtbl {
    pub const fn new<Identity: IAutomaticUpdatesResults_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LastSearchSuccessDate<Identity: IAutomaticUpdatesResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesResults_Impl::LastSearchSuccessDate(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastInstallationSuccessDate<Identity: IAutomaticUpdatesResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesResults_Impl::LastInstallationSuccessDate(this) {
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
            LastSearchSuccessDate: LastSearchSuccessDate::<Identity, OFFSET>,
            LastInstallationSuccessDate: LastInstallationSuccessDate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesResults as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn NotificationLevel(&self) -> windows_core::Result<AutomaticUpdatesNotificationLevel> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NotificationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNotificationLevel(&self, value: AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationLevel)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Required(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Required)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ScheduledInstallationDay(&self) -> windows_core::Result<AutomaticUpdatesScheduledInstallationDay> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScheduledInstallationDay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScheduledInstallationDay(&self, value: AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScheduledInstallationDay)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn ScheduledInstallationTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScheduledInstallationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScheduledInstallationTime(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScheduledInstallationTime)(windows_core::Interface::as_raw(self), value) }
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
    pub NotificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT,
    pub SetNotificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ReadOnly: usize,
    #[cfg(feature = "wtypes")]
    pub Required: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Required: usize,
    pub ScheduledInstallationDay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT,
    pub SetScheduledInstallationDay: unsafe extern "system" fn(*mut core::ffi::c_void, AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT,
    pub ScheduledInstallationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetScheduledInstallationTime: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAutomaticUpdatesSettings_Impl: super::IDispatch_Impl {
    fn NotificationLevel(&self) -> windows_core::Result<AutomaticUpdatesNotificationLevel>;
    fn SetNotificationLevel(&self, value: AutomaticUpdatesNotificationLevel) -> windows_core::Result<()>;
    fn ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Required(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn ScheduledInstallationDay(&self) -> windows_core::Result<AutomaticUpdatesScheduledInstallationDay>;
    fn SetScheduledInstallationDay(&self, value: AutomaticUpdatesScheduledInstallationDay) -> windows_core::Result<()>;
    fn ScheduledInstallationTime(&self) -> windows_core::Result<i32>;
    fn SetScheduledInstallationTime(&self, value: i32) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAutomaticUpdatesSettings_Vtbl {
    pub const fn new<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotificationLevel<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings_Impl::NotificationLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotificationLevel<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings_Impl::SetNotificationLevel(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings_Impl::ReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Required<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings_Impl::Required(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ScheduledInstallationDay<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings_Impl::ScheduledInstallationDay(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScheduledInstallationDay<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings_Impl::SetScheduledInstallationDay(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ScheduledInstallationTime<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings_Impl::ScheduledInstallationTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScheduledInstallationTime<Identity: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings_Impl::SetScheduledInstallationTime(this, core::mem::transmute_copy(&value)).into()
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
            NotificationLevel: NotificationLevel::<Identity, OFFSET>,
            SetNotificationLevel: SetNotificationLevel::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            Required: Required::<Identity, OFFSET>,
            ScheduledInstallationDay: ScheduledInstallationDay::<Identity, OFFSET>,
            SetScheduledInstallationDay: SetScheduledInstallationDay::<Identity, OFFSET>,
            ScheduledInstallationTime: ScheduledInstallationTime::<Identity, OFFSET>,
            SetScheduledInstallationTime: SetScheduledInstallationTime::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn IncludeRecommendedUpdates(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncludeRecommendedUpdates)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetIncludeRecommendedUpdates(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIncludeRecommendedUpdates)(windows_core::Interface::as_raw(self), value) }
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
    pub IncludeRecommendedUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IncludeRecommendedUpdates: usize,
    #[cfg(feature = "wtypes")]
    pub SetIncludeRecommendedUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetIncludeRecommendedUpdates: usize,
    #[cfg(feature = "wtypes")]
    pub CheckPermission: unsafe extern "system" fn(*mut core::ffi::c_void, AutomaticUpdatesUserType, AutomaticUpdatesPermissionType, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CheckPermission: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAutomaticUpdatesSettings2_Impl: IAutomaticUpdatesSettings_Impl {
    fn IncludeRecommendedUpdates(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetIncludeRecommendedUpdates(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CheckPermission(&self, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAutomaticUpdatesSettings2_Vtbl {
    pub const fn new<Identity: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IncludeRecommendedUpdates<Identity: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings2_Impl::IncludeRecommendedUpdates(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIncludeRecommendedUpdates<Identity: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings2_Impl::SetIncludeRecommendedUpdates(this, core::mem::transmute_copy(&value)).into()
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
            IncludeRecommendedUpdates: IncludeRecommendedUpdates::<Identity, OFFSET>,
            SetIncludeRecommendedUpdates: SetIncludeRecommendedUpdates::<Identity, OFFSET>,
            CheckPermission: CheckPermission::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IAutomaticUpdatesSettings as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn NonAdministratorsElevated(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NonAdministratorsElevated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetNonAdministratorsElevated(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNonAdministratorsElevated)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn FeaturedUpdatesEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FeaturedUpdatesEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetFeaturedUpdatesEnabled(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFeaturedUpdatesEnabled)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdatesSettings3_Vtbl {
    pub base__: IAutomaticUpdatesSettings2_Vtbl,
    #[cfg(feature = "wtypes")]
    pub NonAdministratorsElevated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    NonAdministratorsElevated: usize,
    #[cfg(feature = "wtypes")]
    pub SetNonAdministratorsElevated: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetNonAdministratorsElevated: usize,
    #[cfg(feature = "wtypes")]
    pub FeaturedUpdatesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    FeaturedUpdatesEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetFeaturedUpdatesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetFeaturedUpdatesEnabled: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAutomaticUpdatesSettings3_Impl: IAutomaticUpdatesSettings2_Impl {
    fn NonAdministratorsElevated(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetNonAdministratorsElevated(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FeaturedUpdatesEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetFeaturedUpdatesEnabled(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAutomaticUpdatesSettings3_Vtbl {
    pub const fn new<Identity: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NonAdministratorsElevated<Identity: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings3_Impl::NonAdministratorsElevated(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNonAdministratorsElevated<Identity: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings3_Impl::SetNonAdministratorsElevated(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn FeaturedUpdatesEnabled<Identity: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutomaticUpdatesSettings3_Impl::FeaturedUpdatesEnabled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFeaturedUpdatesEnabled<Identity: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutomaticUpdatesSettings3_Impl::SetFeaturedUpdatesEnabled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IAutomaticUpdatesSettings2_Vtbl::new::<Identity, OFFSET>(),
            NonAdministratorsElevated: NonAdministratorsElevated::<Identity, OFFSET>,
            SetNonAdministratorsElevated: SetNonAdministratorsElevated::<Identity, OFFSET>,
            FeaturedUpdatesEnabled: FeaturedUpdatesEnabled::<Identity, OFFSET>,
            SetFeaturedUpdatesEnabled: SetFeaturedUpdatesEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IAutomaticUpdatesSettings as windows_core::Interface>::IID || iid == &<IAutomaticUpdatesSettings2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CategoryID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CategoryID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Children(&self) -> windows_core::Result<ICategoryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Children)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Image(&self) -> windows_core::Result<IImageInformation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Image)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Order(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Order)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Parent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Updates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICategory_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CategoryID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Order: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICategory_Impl: super::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CategoryID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Children(&self) -> windows_core::Result<ICategoryCollection>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Image(&self) -> windows_core::Result<IImageInformation>;
    fn Order(&self) -> windows_core::Result<i32>;
    fn Parent(&self) -> windows_core::Result<ICategory>;
    fn Type(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Updates(&self) -> windows_core::Result<IUpdateCollection>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICategory_Vtbl {
    pub const fn new<Identity: ICategory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::Name(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CategoryID<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::CategoryID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Children<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::Children(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Image<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::Image(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Order<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::Order(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Parent<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::Parent(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Type<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::Type(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Updates<Identity: ICategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategory_Impl::Updates(this) {
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
            Name: Name::<Identity, OFFSET>,
            CategoryID: CategoryID::<Identity, OFFSET>,
            Children: Children::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Image: Image::<Identity, OFFSET>,
            Order: Order::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Updates: Updates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICategory as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ICategory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICategoryCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICategoryCollection_Impl: super::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<ICategory>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICategoryCollection_Vtbl {
    pub const fn new<Identity: ICategoryCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategoryCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategoryCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICategoryCollection_Impl::Count(this) {
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
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICategoryCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDownloadCompletedCallbackArgs_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDownloadCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: IDownloadCompletedCallbackArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadCompletedCallbackArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AsyncState(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AsyncState)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsCompleted(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCompleted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Updates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CleanUp(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CleanUp)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetProgress(&self) -> windows_core::Result<IDownloadProgress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AsyncState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AsyncState: usize,
    #[cfg(feature = "wtypes")]
    pub IsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsCompleted: usize,
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CleanUp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDownloadJob_Impl: super::IDispatch_Impl {
    fn AsyncState(&self) -> windows_core::Result<super::VARIANT>;
    fn IsCompleted(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn CleanUp(&self) -> windows_core::Result<()>;
    fn GetProgress(&self) -> windows_core::Result<IDownloadProgress>;
    fn RequestAbort(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDownloadJob_Vtbl {
    pub const fn new<Identity: IDownloadJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AsyncState<Identity: IDownloadJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadJob_Impl::AsyncState(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: IDownloadJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadJob_Impl::IsCompleted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Updates<Identity: IDownloadJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadJob_Impl::Updates(this) {
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
            AsyncState: AsyncState::<Identity, OFFSET>,
            IsCompleted: IsCompleted::<Identity, OFFSET>,
            Updates: Updates::<Identity, OFFSET>,
            CleanUp: CleanUp::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
            RequestAbort: RequestAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadJob as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "wtypes")]
    pub unsafe fn CurrentUpdateBytesDownloaded(&self) -> windows_core::Result<super::DECIMAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentUpdateBytesDownloaded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CurrentUpdateBytesToDownload(&self) -> windows_core::Result<super::DECIMAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentUpdateBytesToDownload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentUpdateIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentUpdateIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PercentComplete(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PercentComplete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn TotalBytesDownloaded(&self) -> windows_core::Result<super::DECIMAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalBytesDownloaded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn TotalBytesToDownload(&self) -> windows_core::Result<super::DECIMAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalBytesToDownload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateDownloadResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdateResult)(windows_core::Interface::as_raw(self), updateindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentUpdateDownloadPhase(&self) -> windows_core::Result<DownloadPhase> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentUpdateDownloadPhase)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentUpdatePercentComplete(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentUpdatePercentComplete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgress_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub CurrentUpdateBytesDownloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DECIMAL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CurrentUpdateBytesDownloaded: usize,
    #[cfg(feature = "wtypes")]
    pub CurrentUpdateBytesToDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DECIMAL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CurrentUpdateBytesToDownload: usize,
    pub CurrentUpdateIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub TotalBytesDownloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DECIMAL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    TotalBytesDownloaded: usize,
    #[cfg(feature = "wtypes")]
    pub TotalBytesToDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DECIMAL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    TotalBytesToDownload: usize,
    pub GetUpdateResult: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentUpdateDownloadPhase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DownloadPhase) -> windows_core::HRESULT,
    pub CurrentUpdatePercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDownloadProgress_Impl: super::IDispatch_Impl {
    fn CurrentUpdateBytesDownloaded(&self) -> windows_core::Result<super::DECIMAL>;
    fn CurrentUpdateBytesToDownload(&self) -> windows_core::Result<super::DECIMAL>;
    fn CurrentUpdateIndex(&self) -> windows_core::Result<i32>;
    fn PercentComplete(&self) -> windows_core::Result<i32>;
    fn TotalBytesDownloaded(&self) -> windows_core::Result<super::DECIMAL>;
    fn TotalBytesToDownload(&self) -> windows_core::Result<super::DECIMAL>;
    fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateDownloadResult>;
    fn CurrentUpdateDownloadPhase(&self) -> windows_core::Result<DownloadPhase>;
    fn CurrentUpdatePercentComplete(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDownloadProgress_Vtbl {
    pub const fn new<Identity: IDownloadProgress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentUpdateBytesDownloaded<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DECIMAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::CurrentUpdateBytesDownloaded(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentUpdateBytesToDownload<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DECIMAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::CurrentUpdateBytesToDownload(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentUpdateIndex<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::CurrentUpdateIndex(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PercentComplete<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::PercentComplete(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalBytesDownloaded<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DECIMAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::TotalBytesDownloaded(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalBytesToDownload<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DECIMAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::TotalBytesToDownload(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updateindex: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CurrentUpdateDownloadPhase<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut DownloadPhase) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::CurrentUpdateDownloadPhase(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Identity: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgress_Impl::CurrentUpdatePercentComplete(this) {
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
            CurrentUpdateBytesDownloaded: CurrentUpdateBytesDownloaded::<Identity, OFFSET>,
            CurrentUpdateBytesToDownload: CurrentUpdateBytesToDownload::<Identity, OFFSET>,
            CurrentUpdateIndex: CurrentUpdateIndex::<Identity, OFFSET>,
            PercentComplete: PercentComplete::<Identity, OFFSET>,
            TotalBytesDownloaded: TotalBytesDownloaded::<Identity, OFFSET>,
            TotalBytesToDownload: TotalBytesToDownload::<Identity, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, OFFSET>,
            CurrentUpdateDownloadPhase: CurrentUpdateDownloadPhase::<Identity, OFFSET>,
            CurrentUpdatePercentComplete: CurrentUpdatePercentComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadProgress as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Progress(&self) -> windows_core::Result<IDownloadProgress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Progress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgressChangedCallbackArgs_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDownloadProgressChangedCallbackArgs_Impl: super::IDispatch_Impl {
    fn Progress(&self) -> windows_core::Result<IDownloadProgress>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDownloadProgressChangedCallbackArgs_Vtbl {
    pub const fn new<Identity: IDownloadProgressChangedCallbackArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Progress<Identity: IDownloadProgressChangedCallbackArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadProgressChangedCallbackArgs_Impl::Progress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), Progress: Progress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadProgressChangedCallbackArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn HResult(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateDownloadResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdateResult)(windows_core::Interface::as_raw(self), updateindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadResult_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
    pub GetUpdateResult: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDownloadResult_Impl: super::IDispatch_Impl {
    fn HResult(&self) -> windows_core::Result<i32>;
    fn ResultCode(&self) -> windows_core::Result<OperationResultCode>;
    fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateDownloadResult>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDownloadResult_Vtbl {
    pub const fn new<Identity: IDownloadResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HResult<Identity: IDownloadResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadResult_Impl::HResult(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResultCode<Identity: IDownloadResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDownloadResult_Impl::ResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: IDownloadResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updateindex: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
            HResult: HResult::<Identity, OFFSET>,
            ResultCode: ResultCode::<Identity, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadResult as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDownloadResult {}
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
    pub unsafe fn AltText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AltText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Height(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Source(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Source)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IImageInformation_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub AltText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IImageInformation_Impl: super::IDispatch_Impl {
    fn AltText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Height(&self) -> windows_core::Result<i32>;
    fn Source(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Width(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IImageInformation_Vtbl {
    pub const fn new<Identity: IImageInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AltText<Identity: IImageInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageInformation_Impl::AltText(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Height<Identity: IImageInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageInformation_Impl::Height(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Source<Identity: IImageInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageInformation_Impl::Source(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Width<Identity: IImageInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageInformation_Impl::Width(this) {
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
            AltText: AltText::<Identity, OFFSET>,
            Height: Height::<Identity, OFFSET>,
            Source: Source::<Identity, OFFSET>,
            Width: Width::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageInformation as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn RecordInstallationResult<P2>(&self, installationresultcookie: &windows_core::BSTR, hresult: i32, extendedreportingdata: P2) -> windows_core::HRESULT
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
    pub RecordInstallationResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationAgent_Impl: super::IDispatch_Impl {
    fn RecordInstallationResult(&self, installationresultcookie: &windows_core::BSTR, hresult: i32, extendedreportingdata: windows_core::Ref<IStringCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationAgent_Vtbl {
    pub const fn new<Identity: IInstallationAgent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RecordInstallationResult<Identity: IInstallationAgent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, installationresultcookie: *mut core::ffi::c_void, hresult: i32, extendedreportingdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn CanRequestUserInput(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanRequestUserInput)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Impact(&self) -> windows_core::Result<InstallationImpact> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Impact)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RebootBehavior(&self) -> windows_core::Result<InstallationRebootBehavior> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RebootBehavior)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RequiresNetworkConnectivity(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequiresNetworkConnectivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationBehavior_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub CanRequestUserInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CanRequestUserInput: usize,
    pub Impact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InstallationImpact) -> windows_core::HRESULT,
    pub RebootBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InstallationRebootBehavior) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub RequiresNetworkConnectivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RequiresNetworkConnectivity: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationBehavior_Impl: super::IDispatch_Impl {
    fn CanRequestUserInput(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Impact(&self) -> windows_core::Result<InstallationImpact>;
    fn RebootBehavior(&self) -> windows_core::Result<InstallationRebootBehavior>;
    fn RequiresNetworkConnectivity(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationBehavior_Vtbl {
    pub const fn new<Identity: IInstallationBehavior_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanRequestUserInput<Identity: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationBehavior_Impl::CanRequestUserInput(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Impact<Identity: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut InstallationImpact) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationBehavior_Impl::Impact(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RebootBehavior<Identity: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut InstallationRebootBehavior) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationBehavior_Impl::RebootBehavior(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequiresNetworkConnectivity<Identity: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationBehavior_Impl::RequiresNetworkConnectivity(this) {
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
            CanRequestUserInput: CanRequestUserInput::<Identity, OFFSET>,
            Impact: Impact::<Identity, OFFSET>,
            RebootBehavior: RebootBehavior::<Identity, OFFSET>,
            RequiresNetworkConnectivity: RequiresNetworkConnectivity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationBehavior as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationCompletedCallbackArgs_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: IInstallationCompletedCallbackArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationCompletedCallbackArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AsyncState(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AsyncState)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsCompleted(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCompleted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Updates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CleanUp(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CleanUp)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetProgress(&self) -> windows_core::Result<IInstallationProgress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AsyncState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AsyncState: usize,
    #[cfg(feature = "wtypes")]
    pub IsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsCompleted: usize,
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CleanUp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationJob_Impl: super::IDispatch_Impl {
    fn AsyncState(&self) -> windows_core::Result<super::VARIANT>;
    fn IsCompleted(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn CleanUp(&self) -> windows_core::Result<()>;
    fn GetProgress(&self) -> windows_core::Result<IInstallationProgress>;
    fn RequestAbort(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationJob_Vtbl {
    pub const fn new<Identity: IInstallationJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AsyncState<Identity: IInstallationJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationJob_Impl::AsyncState(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: IInstallationJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationJob_Impl::IsCompleted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Updates<Identity: IInstallationJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationJob_Impl::Updates(this) {
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
            AsyncState: AsyncState::<Identity, OFFSET>,
            IsCompleted: IsCompleted::<Identity, OFFSET>,
            Updates: Updates::<Identity, OFFSET>,
            CleanUp: CleanUp::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
            RequestAbort: RequestAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationJob as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn CurrentUpdateIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentUpdateIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentUpdatePercentComplete(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentUpdatePercentComplete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PercentComplete(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PercentComplete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateInstallationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdateResult)(windows_core::Interface::as_raw(self), updateindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationProgress_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub CurrentUpdateIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentUpdatePercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetUpdateResult: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationProgress_Impl: super::IDispatch_Impl {
    fn CurrentUpdateIndex(&self) -> windows_core::Result<i32>;
    fn CurrentUpdatePercentComplete(&self) -> windows_core::Result<i32>;
    fn PercentComplete(&self) -> windows_core::Result<i32>;
    fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateInstallationResult>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationProgress_Vtbl {
    pub const fn new<Identity: IInstallationProgress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentUpdateIndex<Identity: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationProgress_Impl::CurrentUpdateIndex(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Identity: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationProgress_Impl::CurrentUpdatePercentComplete(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PercentComplete<Identity: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationProgress_Impl::PercentComplete(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updateindex: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
            CurrentUpdateIndex: CurrentUpdateIndex::<Identity, OFFSET>,
            CurrentUpdatePercentComplete: CurrentUpdatePercentComplete::<Identity, OFFSET>,
            PercentComplete: PercentComplete::<Identity, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationProgress as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Progress(&self) -> windows_core::Result<IInstallationProgress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Progress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationProgressChangedCallbackArgs_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationProgressChangedCallbackArgs_Impl: super::IDispatch_Impl {
    fn Progress(&self) -> windows_core::Result<IInstallationProgress>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationProgressChangedCallbackArgs_Vtbl {
    pub const fn new<Identity: IInstallationProgressChangedCallbackArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Progress<Identity: IInstallationProgressChangedCallbackArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationProgressChangedCallbackArgs_Impl::Progress(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), Progress: Progress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationProgressChangedCallbackArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn HResult(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateInstallationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUpdateResult)(windows_core::Interface::as_raw(self), updateindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationResult_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RebootRequired: usize,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
    pub GetUpdateResult: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInstallationResult_Impl: super::IDispatch_Impl {
    fn HResult(&self) -> windows_core::Result<i32>;
    fn RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn ResultCode(&self) -> windows_core::Result<OperationResultCode>;
    fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateInstallationResult>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInstallationResult_Vtbl {
    pub const fn new<Identity: IInstallationResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HResult<Identity: IInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationResult_Impl::HResult(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RebootRequired<Identity: IInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationResult_Impl::RebootRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResultCode<Identity: IInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInstallationResult_Impl::ResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: IInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updateindex: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
            HResult: HResult::<Identity, OFFSET>,
            RebootRequired: RebootRequired::<Identity, OFFSET>,
            ResultCode: ResultCode::<Identity, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInstallationResult as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Product(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Product)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IInvalidProductLicenseException_Vtbl {
    pub base__: IUpdateException_Vtbl,
    pub Product: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IInvalidProductLicenseException_Impl: IUpdateException_Impl {
    fn Product(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IInvalidProductLicenseException_Vtbl {
    pub const fn new<Identity: IInvalidProductLicenseException_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Product<Identity: IInvalidProductLicenseException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInvalidProductLicenseException_Impl::Product(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUpdateException_Vtbl::new::<Identity, OFFSET>(), Product: Product::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInvalidProductLicenseException as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateException as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISearchCompletedCallbackArgs_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISearchCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: ISearchCompletedCallbackArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCompletedCallbackArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AsyncState(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AsyncState)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsCompleted(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCompleted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AsyncState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AsyncState: usize,
    #[cfg(feature = "wtypes")]
    pub IsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsCompleted: usize,
    pub CleanUp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISearchJob_Impl: super::IDispatch_Impl {
    fn AsyncState(&self) -> windows_core::Result<super::VARIANT>;
    fn IsCompleted(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn CleanUp(&self) -> windows_core::Result<()>;
    fn RequestAbort(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISearchJob_Vtbl {
    pub const fn new<Identity: ISearchJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AsyncState<Identity: ISearchJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchJob_Impl::AsyncState(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: ISearchJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchJob_Impl::IsCompleted(this) {
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
            AsyncState: AsyncState::<Identity, OFFSET>,
            IsCompleted: IsCompleted::<Identity, OFFSET>,
            CleanUp: CleanUp::<Identity, OFFSET>,
            RequestAbort: RequestAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchJob as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RootCategories(&self) -> windows_core::Result<ICategoryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RootCategories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Updates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Warnings(&self) -> windows_core::Result<IUpdateExceptionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Warnings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchResult_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
    pub RootCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Warnings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISearchResult_Impl: super::IDispatch_Impl {
    fn ResultCode(&self) -> windows_core::Result<OperationResultCode>;
    fn RootCategories(&self) -> windows_core::Result<ICategoryCollection>;
    fn Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn Warnings(&self) -> windows_core::Result<IUpdateExceptionCollection>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISearchResult_Vtbl {
    pub const fn new<Identity: ISearchResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResultCode<Identity: ISearchResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchResult_Impl::ResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RootCategories<Identity: ISearchResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchResult_Impl::RootCategories(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Updates<Identity: ISearchResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchResult_Impl::Updates(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Warnings<Identity: ISearchResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISearchResult_Impl::Warnings(this) {
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
            ResultCode: ResultCode::<Identity, OFFSET>,
            RootCategories: RootCategories::<Identity, OFFSET>,
            Updates: Updates::<Identity, OFFSET>,
            Warnings: Warnings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchResult as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetItem(&self, index: i32, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetItem)(windows_core::Interface::as_raw(self), index, core::mem::transmute_copy(value)) }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add(&self, value: &windows_core::BSTR) -> windows_core::Result<i32> {
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
            (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Insert(&self, index: i32, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), index, core::mem::transmute_copy(value)) }
    }
    pub unsafe fn RemoveAt(&self, index: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IStringCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ReadOnly: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IStringCollection_Impl: super::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetItem(&self, index: i32, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Add(&self, value: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn Copy(&self) -> windows_core::Result<IStringCollection>;
    fn Insert(&self, index: i32, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IStringCollection_Vtbl {
    pub const fn new<Identity: IStringCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetItem<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStringCollection_Impl::SetItem(this, core::mem::transmute_copy(&index), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringCollection_Impl::ReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Insert<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStringCollection_Impl::Insert(this, core::mem::transmute_copy(&index), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStringCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Item: Item::<Identity, OFFSET>,
            SetItem: SetItem::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
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
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn OemHardwareSupportLink(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OemHardwareSupportLink)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISystemInformation_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub OemHardwareSupportLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RebootRequired: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISystemInformation_Impl: super::IDispatch_Impl {
    fn OemHardwareSupportLink(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISystemInformation_Vtbl {
    pub const fn new<Identity: ISystemInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OemHardwareSupportLink<Identity: ISystemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemInformation_Impl::OemHardwareSupportLink(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RebootRequired<Identity: ISystemInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemInformation_Impl::RebootRequired(this) {
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
            OemHardwareSupportLink: OemHardwareSupportLink::<Identity, OFFSET>,
            RebootRequired: RebootRequired::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemInformation as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Title(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AutoSelectOnWebSites(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoSelectOnWebSites)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BundledUpdates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BundledUpdates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CanRequireSource(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanRequireSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Categories(&self) -> windows_core::Result<ICategoryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Categories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Deadline(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Deadline)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn DeltaCompressedContentAvailable(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeltaCompressedContentAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn DeltaCompressedContentPreferred(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeltaCompressedContentPreferred)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn EulaAccepted(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EulaAccepted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EulaText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EulaText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HandlerID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HandlerID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Identity(&self) -> windows_core::Result<IUpdateIdentity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Identity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Image(&self) -> windows_core::Result<IImageInformation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Image)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InstallationBehavior(&self) -> windows_core::Result<IInstallationBehavior> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstallationBehavior)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsBeta(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsBeta)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsDownloaded(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDownloaded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsHidden(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetIsHidden(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIsHidden)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsInstalled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInstalled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsMandatory(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMandatory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsUninstallable(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUninstallable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Languages(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Languages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn LastDeploymentChangeTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastDeploymentChangeTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn MaxDownloadSize(&self) -> windows_core::Result<super::DECIMAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxDownloadSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn MinDownloadSize(&self) -> windows_core::Result<super::DECIMAL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinDownloadSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoreInfoUrls(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoreInfoUrls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MsrcSeverity(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MsrcSeverity)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RecommendedCpuSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RecommendedCpuSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RecommendedHardDiskSpace(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RecommendedHardDiskSpace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RecommendedMemory(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RecommendedMemory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReleaseNotes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReleaseNotes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SecurityBulletinIDs(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecurityBulletinIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SupersededUpdateIDs(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupersededUpdateIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SupportUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<UpdateType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UninstallationNotes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UninstallationNotes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UninstallationBehavior(&self) -> windows_core::Result<IInstallationBehavior> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UninstallationBehavior)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn UninstallationSteps(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UninstallationSteps)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn KBArticleIDs(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KBArticleIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AcceptEula(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcceptEula)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DeploymentAction(&self) -> windows_core::Result<DeploymentAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeploymentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CopyFromCache(&self, path: &windows_core::BSTR, toextractcabfiles: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CopyFromCache)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), toextractcabfiles) }
    }
    pub unsafe fn DownloadPriority(&self) -> windows_core::Result<DownloadPriority> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DownloadPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DownloadContents(&self) -> windows_core::Result<IUpdateDownloadContentCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DownloadContents)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub AutoSelectOnWebSites: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AutoSelectOnWebSites: usize,
    pub BundledUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub CanRequireSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CanRequireSource: usize,
    pub Categories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Deadline: usize,
    #[cfg(feature = "wtypes")]
    pub DeltaCompressedContentAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    DeltaCompressedContentAvailable: usize,
    #[cfg(feature = "wtypes")]
    pub DeltaCompressedContentPreferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    DeltaCompressedContentPreferred: usize,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub EulaAccepted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    EulaAccepted: usize,
    pub EulaText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HandlerID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Identity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallationBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsBeta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsBeta: usize,
    #[cfg(feature = "wtypes")]
    pub IsDownloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsDownloaded: usize,
    #[cfg(feature = "wtypes")]
    pub IsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsHidden: usize,
    #[cfg(feature = "wtypes")]
    pub SetIsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetIsHidden: usize,
    #[cfg(feature = "wtypes")]
    pub IsInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsInstalled: usize,
    #[cfg(feature = "wtypes")]
    pub IsMandatory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsMandatory: usize,
    #[cfg(feature = "wtypes")]
    pub IsUninstallable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsUninstallable: usize,
    pub Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastDeploymentChangeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub MaxDownloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DECIMAL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    MaxDownloadSize: usize,
    #[cfg(feature = "wtypes")]
    pub MinDownloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DECIMAL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    MinDownloadSize: usize,
    pub MoreInfoUrls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MsrcSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecommendedCpuSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RecommendedHardDiskSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RecommendedMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ReleaseNotes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SecurityBulletinIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupersededUpdateIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UpdateType) -> windows_core::HRESULT,
    pub UninstallationNotes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UninstallationBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UninstallationSteps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KBArticleIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptEula: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeploymentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeploymentAction) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub CopyFromCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CopyFromCache: usize,
    pub DownloadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DownloadPriority) -> windows_core::HRESULT,
    pub DownloadContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdate_Impl: super::IDispatch_Impl {
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AutoSelectOnWebSites(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn BundledUpdates(&self) -> windows_core::Result<IUpdateCollection>;
    fn CanRequireSource(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Categories(&self) -> windows_core::Result<ICategoryCollection>;
    fn Deadline(&self) -> windows_core::Result<super::VARIANT>;
    fn DeltaCompressedContentAvailable(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn DeltaCompressedContentPreferred(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EulaAccepted(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn EulaText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn HandlerID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Identity(&self) -> windows_core::Result<IUpdateIdentity>;
    fn Image(&self) -> windows_core::Result<IImageInformation>;
    fn InstallationBehavior(&self) -> windows_core::Result<IInstallationBehavior>;
    fn IsBeta(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn IsDownloaded(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn IsHidden(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetIsHidden(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsInstalled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn IsMandatory(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn IsUninstallable(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Languages(&self) -> windows_core::Result<IStringCollection>;
    fn LastDeploymentChangeTime(&self) -> windows_core::Result<f64>;
    fn MaxDownloadSize(&self) -> windows_core::Result<super::DECIMAL>;
    fn MinDownloadSize(&self) -> windows_core::Result<super::DECIMAL>;
    fn MoreInfoUrls(&self) -> windows_core::Result<IStringCollection>;
    fn MsrcSeverity(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RecommendedCpuSpeed(&self) -> windows_core::Result<i32>;
    fn RecommendedHardDiskSpace(&self) -> windows_core::Result<i32>;
    fn RecommendedMemory(&self) -> windows_core::Result<i32>;
    fn ReleaseNotes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SecurityBulletinIDs(&self) -> windows_core::Result<IStringCollection>;
    fn SupersededUpdateIDs(&self) -> windows_core::Result<IStringCollection>;
    fn SupportUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<UpdateType>;
    fn UninstallationNotes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UninstallationBehavior(&self) -> windows_core::Result<IInstallationBehavior>;
    fn UninstallationSteps(&self) -> windows_core::Result<IStringCollection>;
    fn KBArticleIDs(&self) -> windows_core::Result<IStringCollection>;
    fn AcceptEula(&self) -> windows_core::Result<()>;
    fn DeploymentAction(&self) -> windows_core::Result<DeploymentAction>;
    fn CopyFromCache(&self, path: &windows_core::BSTR, toextractcabfiles: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DownloadPriority(&self) -> windows_core::Result<DownloadPriority>;
    fn DownloadContents(&self) -> windows_core::Result<IUpdateDownloadContentCollection>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdate_Vtbl {
    pub const fn new<Identity: IUpdate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Title<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::Title(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AutoSelectOnWebSites<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::AutoSelectOnWebSites(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BundledUpdates<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::BundledUpdates(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanRequireSource<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::CanRequireSource(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Categories<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::Categories(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Deadline<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::Deadline(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeltaCompressedContentAvailable<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::DeltaCompressedContentAvailable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeltaCompressedContentPreferred<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::DeltaCompressedContentPreferred(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EulaAccepted<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::EulaAccepted(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EulaText<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::EulaText(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HandlerID<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::HandlerID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Identity<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::Identity(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Image<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::Image(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstallationBehavior<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::InstallationBehavior(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsBeta<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::IsBeta(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsDownloaded<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::IsDownloaded(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsHidden<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::IsHidden(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsHidden<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdate_Impl::SetIsHidden(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn IsInstalled<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::IsInstalled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsMandatory<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::IsMandatory(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsUninstallable<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::IsUninstallable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Languages<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::Languages(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastDeploymentChangeTime<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::LastDeploymentChangeTime(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MaxDownloadSize<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DECIMAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::MaxDownloadSize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinDownloadSize<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DECIMAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::MinDownloadSize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoreInfoUrls<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::MoreInfoUrls(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MsrcSeverity<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::MsrcSeverity(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RecommendedCpuSpeed<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::RecommendedCpuSpeed(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RecommendedHardDiskSpace<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::RecommendedHardDiskSpace(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RecommendedMemory<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::RecommendedMemory(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseNotes<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::ReleaseNotes(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SecurityBulletinIDs<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::SecurityBulletinIDs(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupersededUpdateIDs<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::SupersededUpdateIDs(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportUrl<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::SupportUrl(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Type<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UpdateType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::Type(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UninstallationNotes<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::UninstallationNotes(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UninstallationBehavior<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::UninstallationBehavior(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UninstallationSteps<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::UninstallationSteps(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KBArticleIDs<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::KBArticleIDs(this) {
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
        unsafe extern "system" fn DeploymentAction<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut DeploymentAction) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::DeploymentAction(this) {
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
        unsafe extern "system" fn DownloadPriority<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut DownloadPriority) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::DownloadPriority(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DownloadContents<Identity: IUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate_Impl::DownloadContents(this) {
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
            Title: Title::<Identity, OFFSET>,
            AutoSelectOnWebSites: AutoSelectOnWebSites::<Identity, OFFSET>,
            BundledUpdates: BundledUpdates::<Identity, OFFSET>,
            CanRequireSource: CanRequireSource::<Identity, OFFSET>,
            Categories: Categories::<Identity, OFFSET>,
            Deadline: Deadline::<Identity, OFFSET>,
            DeltaCompressedContentAvailable: DeltaCompressedContentAvailable::<Identity, OFFSET>,
            DeltaCompressedContentPreferred: DeltaCompressedContentPreferred::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            EulaAccepted: EulaAccepted::<Identity, OFFSET>,
            EulaText: EulaText::<Identity, OFFSET>,
            HandlerID: HandlerID::<Identity, OFFSET>,
            Identity: Identity::<Identity, OFFSET>,
            Image: Image::<Identity, OFFSET>,
            InstallationBehavior: InstallationBehavior::<Identity, OFFSET>,
            IsBeta: IsBeta::<Identity, OFFSET>,
            IsDownloaded: IsDownloaded::<Identity, OFFSET>,
            IsHidden: IsHidden::<Identity, OFFSET>,
            SetIsHidden: SetIsHidden::<Identity, OFFSET>,
            IsInstalled: IsInstalled::<Identity, OFFSET>,
            IsMandatory: IsMandatory::<Identity, OFFSET>,
            IsUninstallable: IsUninstallable::<Identity, OFFSET>,
            Languages: Languages::<Identity, OFFSET>,
            LastDeploymentChangeTime: LastDeploymentChangeTime::<Identity, OFFSET>,
            MaxDownloadSize: MaxDownloadSize::<Identity, OFFSET>,
            MinDownloadSize: MinDownloadSize::<Identity, OFFSET>,
            MoreInfoUrls: MoreInfoUrls::<Identity, OFFSET>,
            MsrcSeverity: MsrcSeverity::<Identity, OFFSET>,
            RecommendedCpuSpeed: RecommendedCpuSpeed::<Identity, OFFSET>,
            RecommendedHardDiskSpace: RecommendedHardDiskSpace::<Identity, OFFSET>,
            RecommendedMemory: RecommendedMemory::<Identity, OFFSET>,
            ReleaseNotes: ReleaseNotes::<Identity, OFFSET>,
            SecurityBulletinIDs: SecurityBulletinIDs::<Identity, OFFSET>,
            SupersededUpdateIDs: SupersededUpdateIDs::<Identity, OFFSET>,
            SupportUrl: SupportUrl::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            UninstallationNotes: UninstallationNotes::<Identity, OFFSET>,
            UninstallationBehavior: UninstallationBehavior::<Identity, OFFSET>,
            UninstallationSteps: UninstallationSteps::<Identity, OFFSET>,
            KBArticleIDs: KBArticleIDs::<Identity, OFFSET>,
            AcceptEula: AcceptEula::<Identity, OFFSET>,
            DeploymentAction: DeploymentAction::<Identity, OFFSET>,
            CopyFromCache: CopyFromCache::<Identity, OFFSET>,
            DownloadPriority: DownloadPriority::<Identity, OFFSET>,
            DownloadContents: DownloadContents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsPresent(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPresent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CveIDs(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CveIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RebootRequired: usize,
    #[cfg(feature = "wtypes")]
    pub IsPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsPresent: usize,
    pub CveIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyToCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdate2_Impl: IUpdate_Impl {
    fn RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn IsPresent(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn CveIDs(&self) -> windows_core::Result<IStringCollection>;
    fn CopyToCache(&self, pfiles: windows_core::Ref<IStringCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdate2_Vtbl {
    pub const fn new<Identity: IUpdate2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RebootRequired<Identity: IUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate2_Impl::RebootRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPresent<Identity: IUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate2_Impl::IsPresent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CveIDs<Identity: IUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate2_Impl::CveIDs(this) {
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
            RebootRequired: RebootRequired::<Identity, OFFSET>,
            IsPresent: IsPresent::<Identity, OFFSET>,
            CveIDs: CveIDs::<Identity, OFFSET>,
            CopyToCache: CopyToCache::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn BrowseOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BrowseOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate3_Vtbl {
    pub base__: IUpdate2_Vtbl,
    #[cfg(feature = "wtypes")]
    pub BrowseOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    BrowseOnly: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdate3_Impl: IUpdate2_Impl {
    fn BrowseOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdate3_Vtbl {
    pub const fn new<Identity: IUpdate3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BrowseOnly<Identity: IUpdate3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate3_Impl::BrowseOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUpdate2_Vtbl::new::<Identity, OFFSET>(), BrowseOnly: BrowseOnly::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IUpdate2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn PerUser(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PerUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate4_Vtbl {
    pub base__: IUpdate3_Vtbl,
    #[cfg(feature = "wtypes")]
    pub PerUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    PerUser: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdate4_Impl: IUpdate3_Impl {
    fn PerUser(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdate4_Vtbl {
    pub const fn new<Identity: IUpdate4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PerUser<Identity: IUpdate4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate4_Impl::PerUser(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUpdate3_Vtbl::new::<Identity, OFFSET>(), PerUser: PerUser::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate4 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IUpdate2 as windows_core::Interface>::IID || iid == &<IUpdate3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn AutoSelection(&self) -> windows_core::Result<AutoSelectionMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AutoDownload(&self) -> windows_core::Result<AutoDownloadMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoDownload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate5_Vtbl {
    pub base__: IUpdate4_Vtbl,
    pub AutoSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoSelectionMode) -> windows_core::HRESULT,
    pub AutoDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoDownloadMode) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdate5_Impl: IUpdate4_Impl {
    fn AutoSelection(&self) -> windows_core::Result<AutoSelectionMode>;
    fn AutoDownload(&self) -> windows_core::Result<AutoDownloadMode>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdate5_Vtbl {
    pub const fn new<Identity: IUpdate5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AutoSelection<Identity: IUpdate5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutoSelectionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate5_Impl::AutoSelection(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AutoDownload<Identity: IUpdate5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutoDownloadMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdate5_Impl::AutoDownload(this) {
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
            AutoSelection: AutoSelection::<Identity, OFFSET>,
            AutoDownload: AutoDownload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdate5 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IUpdate2 as windows_core::Interface>::IID || iid == &<IUpdate3 as windows_core::Interface>::IID || iid == &<IUpdate4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<IUpdate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetItem<P1>(&self, index: i32, value: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUpdate>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetItem)(windows_core::Interface::as_raw(self), index, value.param().abi()) }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0>(&self, value: P0) -> windows_core::Result<i32>
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
            (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Insert<P1>(&self, index: i32, value: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUpdate>,
    {
        unsafe { (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), index, value.param().abi()) }
    }
    pub unsafe fn RemoveAt(&self, index: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ReadOnly: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateCollection_Impl: super::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<IUpdate>;
    fn SetItem(&self, index: i32, value: windows_core::Ref<IUpdate>) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Add(&self, value: windows_core::Ref<IUpdate>) -> windows_core::Result<i32>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn Copy(&self) -> windows_core::Result<IUpdateCollection>;
    fn Insert(&self, index: i32, value: windows_core::Ref<IUpdate>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateCollection_Vtbl {
    pub const fn new<Identity: IUpdateCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetItem<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateCollection_Impl::SetItem(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateCollection_Impl::ReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Insert<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateCollection_Impl::Insert(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<Identity: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Item: Item::<Identity, OFFSET>,
            SetItem: SetItem::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
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
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn DownloadUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DownloadUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloadContent_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub DownloadUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateDownloadContent_Impl: super::IDispatch_Impl {
    fn DownloadUrl(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateDownloadContent_Vtbl {
    pub const fn new<Identity: IUpdateDownloadContent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DownloadUrl<Identity: IUpdateDownloadContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadContent_Impl::DownloadUrl(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), DownloadUrl: DownloadUrl::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadContent as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn IsDeltaCompressedContent(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDeltaCompressedContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloadContent2_Vtbl {
    pub base__: IUpdateDownloadContent_Vtbl,
    #[cfg(feature = "wtypes")]
    pub IsDeltaCompressedContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsDeltaCompressedContent: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateDownloadContent2_Impl: IUpdateDownloadContent_Impl {
    fn IsDeltaCompressedContent(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateDownloadContent2_Vtbl {
    pub const fn new<Identity: IUpdateDownloadContent2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDeltaCompressedContent<Identity: IUpdateDownloadContent2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadContent2_Impl::IsDeltaCompressedContent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUpdateDownloadContent_Vtbl::new::<Identity, OFFSET>(), IsDeltaCompressedContent: IsDeltaCompressedContent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadContent2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateDownloadContent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<IUpdateDownloadContent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloadContentCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateDownloadContentCollection_Impl: super::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<IUpdateDownloadContent>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateDownloadContentCollection_Vtbl {
    pub const fn new<Identity: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadContentCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadContentCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadContentCollection_Impl::Count(this) {
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
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadContentCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn HResult(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloadResult_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateDownloadResult_Impl: super::IDispatch_Impl {
    fn HResult(&self) -> windows_core::Result<i32>;
    fn ResultCode(&self) -> windows_core::Result<OperationResultCode>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateDownloadResult_Vtbl {
    pub const fn new<Identity: IUpdateDownloadResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HResult<Identity: IUpdateDownloadResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadResult_Impl::HResult(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResultCode<Identity: IUpdateDownloadResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloadResult_Impl::ResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), HResult: HResult::<Identity, OFFSET>, ResultCode: ResultCode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadResult as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientApplicationID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsForced(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsForced)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetIsForced(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIsForced)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<DownloadPriority> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPriority(&self, value: DownloadPriority) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn Updates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetUpdates<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUpdateCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUpdates)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn BeginDownload<P0, P1>(&self, onprogresschanged: P0, oncompleted: P1, state: &super::VARIANT) -> windows_core::Result<IDownloadJob>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginDownload)(windows_core::Interface::as_raw(self), onprogresschanged.param().abi(), oncompleted.param().abi(), core::mem::transmute_copy(state), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Download(&self) -> windows_core::Result<IDownloadResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EndDownload<P0>(&self, value: P0) -> windows_core::Result<IDownloadResult>
    where
        P0: windows_core::Param<IDownloadJob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndDownload)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloader_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsForced: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsForced: usize,
    #[cfg(feature = "wtypes")]
    pub SetIsForced: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetIsForced: usize,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DownloadPriority) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, DownloadPriority) -> windows_core::HRESULT,
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub BeginDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    BeginDownload: usize,
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateDownloader_Impl: super::IDispatch_Impl {
    fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsForced(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetIsForced(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<DownloadPriority>;
    fn SetPriority(&self, value: DownloadPriority) -> windows_core::Result<()>;
    fn Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn SetUpdates(&self, value: windows_core::Ref<IUpdateCollection>) -> windows_core::Result<()>;
    fn BeginDownload(&self, onprogresschanged: windows_core::Ref<windows_core::IUnknown>, oncompleted: windows_core::Ref<windows_core::IUnknown>, state: &super::VARIANT) -> windows_core::Result<IDownloadJob>;
    fn Download(&self) -> windows_core::Result<IDownloadResult>;
    fn EndDownload(&self, value: windows_core::Ref<IDownloadJob>) -> windows_core::Result<IDownloadResult>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateDownloader_Vtbl {
    pub const fn new<Identity: IUpdateDownloader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClientApplicationID<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloader_Impl::ClientApplicationID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateDownloader_Impl::SetClientApplicationID(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn IsForced<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloader_Impl::IsForced(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsForced<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateDownloader_Impl::SetIsForced(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Priority<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut DownloadPriority) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloader_Impl::Priority(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: DownloadPriority) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateDownloader_Impl::SetPriority(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Updates<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateDownloader_Impl::Updates(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUpdates<Identity: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateDownloader_Impl::SetUpdates(this, core::mem::transmute_copy(&value)).into()
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
            ClientApplicationID: ClientApplicationID::<Identity, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, OFFSET>,
            IsForced: IsForced::<Identity, OFFSET>,
            SetIsForced: SetIsForced::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Updates: Updates::<Identity, OFFSET>,
            SetUpdates: SetUpdates::<Identity, OFFSET>,
            BeginDownload: BeginDownload::<Identity, OFFSET>,
            Download: Download::<Identity, OFFSET>,
            EndDownload: EndDownload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateDownloader as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn BeginDownload2<P1, P2>(&self, downloadtype: DownloadType, onprogresschanged: P1, oncompleted: P2, state: &super::VARIANT) -> windows_core::Result<IDownloadJob>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginDownload2)(windows_core::Interface::as_raw(self), downloadtype, onprogresschanged.param().abi(), oncompleted.param().abi(), core::mem::transmute_copy(state), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Download2(&self, downloadtype: DownloadType) -> windows_core::Result<IDownloadResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Download2)(windows_core::Interface::as_raw(self), downloadtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloaderEx_Vtbl {
    pub base__: IUpdateDownloader_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub BeginDownload2: unsafe extern "system" fn(*mut core::ffi::c_void, DownloadType, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    BeginDownload2: usize,
    pub Download2: unsafe extern "system" fn(*mut core::ffi::c_void, DownloadType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateDownloaderEx_Impl: IUpdateDownloader_Impl {
    fn BeginDownload2(&self, downloadtype: DownloadType, onprogresschanged: windows_core::Ref<windows_core::IUnknown>, oncompleted: windows_core::Ref<windows_core::IUnknown>, state: &super::VARIANT) -> windows_core::Result<IDownloadJob>;
    fn Download2(&self, downloadtype: DownloadType) -> windows_core::Result<IDownloadResult>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ExtendedStaticProperty(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedStaticProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ExtendedStaticProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ExtendedStaticProperty: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub EvaluateExtendedDynamicProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    EvaluateExtendedDynamicProperty: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateEx_Impl: IUpdate5_Impl {
    fn ExtendedStaticProperty(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT>;
    fn EvaluateExtendedDynamicProperty(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateEx_Vtbl {
    pub const fn new<Identity: IUpdateEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExtendedStaticProperty<Identity: IUpdateEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateEx_Impl::ExtendedStaticProperty(this, core::mem::transmute(&propertyname)) {
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
            ExtendedStaticProperty: ExtendedStaticProperty::<Identity, OFFSET>,
            EvaluateExtendedDynamicProperty: EvaluateExtendedDynamicProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateEx as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IUpdate2 as windows_core::Interface>::IID || iid == &<IUpdate3 as windows_core::Interface>::IID || iid == &<IUpdate4 as windows_core::Interface>::IID || iid == &<IUpdate5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Message(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HResult(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Context(&self) -> windows_core::Result<UpdateExceptionContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Context)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateException_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UpdateExceptionContext) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateException_Impl: super::IDispatch_Impl {
    fn Message(&self) -> windows_core::Result<windows_core::BSTR>;
    fn HResult(&self) -> windows_core::Result<i32>;
    fn Context(&self) -> windows_core::Result<UpdateExceptionContext>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateException_Vtbl {
    pub const fn new<Identity: IUpdateException_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Message<Identity: IUpdateException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateException_Impl::Message(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HResult<Identity: IUpdateException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateException_Impl::HResult(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Context<Identity: IUpdateException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UpdateExceptionContext) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateException_Impl::Context(this) {
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
            Message: Message::<Identity, OFFSET>,
            HResult: HResult::<Identity, OFFSET>,
            Context: Context::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateException as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<IUpdateException> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateExceptionCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateExceptionCollection_Impl: super::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<IUpdateException>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateExceptionCollection_Vtbl {
    pub const fn new<Identity: IUpdateExceptionCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateExceptionCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateExceptionCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateExceptionCollection_Impl::Count(this) {
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
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateExceptionCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Operation(&self) -> windows_core::Result<UpdateOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Operation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HResult(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Date(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Date)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UpdateIdentity(&self) -> windows_core::Result<IUpdateIdentity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateIdentity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Title(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UnmappedResultCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnmappedResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ServerSelection(&self) -> windows_core::Result<ServerSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServerSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UninstallationSteps(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UninstallationSteps)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn UninstallationNotes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UninstallationNotes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SupportUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateHistoryEntry_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UpdateOperation) -> windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
    pub HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub UpdateIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnmappedResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServerSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ServerSelection) -> windows_core::HRESULT,
    pub ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UninstallationSteps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UninstallationNotes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateHistoryEntry_Impl: super::IDispatch_Impl {
    fn Operation(&self) -> windows_core::Result<UpdateOperation>;
    fn ResultCode(&self) -> windows_core::Result<OperationResultCode>;
    fn HResult(&self) -> windows_core::Result<i32>;
    fn Date(&self) -> windows_core::Result<f64>;
    fn UpdateIdentity(&self) -> windows_core::Result<IUpdateIdentity>;
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UnmappedResultCode(&self) -> windows_core::Result<i32>;
    fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServerSelection(&self) -> windows_core::Result<ServerSelection>;
    fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UninstallationSteps(&self) -> windows_core::Result<IStringCollection>;
    fn UninstallationNotes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SupportUrl(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateHistoryEntry_Vtbl {
    pub const fn new<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Operation<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UpdateOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::Operation(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResultCode<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::ResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HResult<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::HResult(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Date<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::Date(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateIdentity<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::UpdateIdentity(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Title<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::Title(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::Description(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnmappedResultCode<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::UnmappedResultCode(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClientApplicationID<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::ClientApplicationID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServerSelection<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut ServerSelection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::ServerSelection(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceID<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::ServiceID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UninstallationSteps<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::UninstallationSteps(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UninstallationNotes<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::UninstallationNotes(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportUrl<Identity: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry_Impl::SupportUrl(this) {
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
            Operation: Operation::<Identity, OFFSET>,
            ResultCode: ResultCode::<Identity, OFFSET>,
            HResult: HResult::<Identity, OFFSET>,
            Date: Date::<Identity, OFFSET>,
            UpdateIdentity: UpdateIdentity::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            UnmappedResultCode: UnmappedResultCode::<Identity, OFFSET>,
            ClientApplicationID: ClientApplicationID::<Identity, OFFSET>,
            ServerSelection: ServerSelection::<Identity, OFFSET>,
            ServiceID: ServiceID::<Identity, OFFSET>,
            UninstallationSteps: UninstallationSteps::<Identity, OFFSET>,
            UninstallationNotes: UninstallationNotes::<Identity, OFFSET>,
            SupportUrl: SupportUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateHistoryEntry as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Categories(&self) -> windows_core::Result<ICategoryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Categories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateHistoryEntry2_Vtbl {
    pub base__: IUpdateHistoryEntry_Vtbl,
    pub Categories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateHistoryEntry2_Impl: IUpdateHistoryEntry_Impl {
    fn Categories(&self) -> windows_core::Result<ICategoryCollection>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateHistoryEntry2_Vtbl {
    pub const fn new<Identity: IUpdateHistoryEntry2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Categories<Identity: IUpdateHistoryEntry2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntry2_Impl::Categories(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUpdateHistoryEntry_Vtbl::new::<Identity, OFFSET>(), Categories: Categories::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateHistoryEntry2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateHistoryEntry as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<IUpdateHistoryEntry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateHistoryEntryCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateHistoryEntryCollection_Impl: super::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<IUpdateHistoryEntry>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateHistoryEntryCollection_Vtbl {
    pub const fn new<Identity: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntryCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntryCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateHistoryEntryCollection_Impl::Count(this) {
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
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateHistoryEntryCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn RevisionNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RevisionNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UpdateID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateIdentity_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub RevisionNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UpdateID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateIdentity_Impl: super::IDispatch_Impl {
    fn RevisionNumber(&self) -> windows_core::Result<i32>;
    fn UpdateID(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateIdentity_Vtbl {
    pub const fn new<Identity: IUpdateIdentity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RevisionNumber<Identity: IUpdateIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateIdentity_Impl::RevisionNumber(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateID<Identity: IUpdateIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateIdentity_Impl::UpdateID(this) {
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
            RevisionNumber: RevisionNumber::<Identity, OFFSET>,
            UpdateID: UpdateID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateIdentity as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn HResult(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstallationResult_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RebootRequired: usize,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateInstallationResult_Impl: super::IDispatch_Impl {
    fn HResult(&self) -> windows_core::Result<i32>;
    fn RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn ResultCode(&self) -> windows_core::Result<OperationResultCode>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateInstallationResult_Vtbl {
    pub const fn new<Identity: IUpdateInstallationResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HResult<Identity: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallationResult_Impl::HResult(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RebootRequired<Identity: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallationResult_Impl::RebootRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResultCode<Identity: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut OperationResultCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallationResult_Impl::ResultCode(this) {
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
            HResult: HResult::<Identity, OFFSET>,
            RebootRequired: RebootRequired::<Identity, OFFSET>,
            ResultCode: ResultCode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstallationResult as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientApplicationID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsForced(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsForced)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetIsForced(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIsForced)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ParentHwnd(&self) -> windows_core::Result<super::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParentHwnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetParentHwnd(&self, value: super::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParentHwnd)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn SetParentWindow<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetParentWindow)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn ParentWindow(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParentWindow)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Updates(&self) -> windows_core::Result<IUpdateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetUpdates<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUpdateCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUpdates)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn BeginInstall<P0, P1>(&self, onprogresschanged: P0, oncompleted: P1, state: &super::VARIANT) -> windows_core::Result<IInstallationJob>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginInstall)(windows_core::Interface::as_raw(self), onprogresschanged.param().abi(), oncompleted.param().abi(), core::mem::transmute_copy(state), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn BeginUninstall<P0, P1>(&self, onprogresschanged: P0, oncompleted: P1, state: &super::VARIANT) -> windows_core::Result<IInstallationJob>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginUninstall)(windows_core::Interface::as_raw(self), onprogresschanged.param().abi(), oncompleted.param().abi(), core::mem::transmute_copy(state), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EndInstall<P0>(&self, value: P0) -> windows_core::Result<IInstallationResult>
    where
        P0: windows_core::Param<IInstallationJob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndInstall)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EndUninstall<P0>(&self, value: P0) -> windows_core::Result<IInstallationResult>
    where
        P0: windows_core::Param<IInstallationJob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndUninstall)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Install(&self) -> windows_core::Result<IInstallationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Install)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RunWizard(&self, dialogtitle: &windows_core::BSTR) -> windows_core::Result<IInstallationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RunWizard)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(dialogtitle), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsBusy(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsBusy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Uninstall(&self) -> windows_core::Result<IInstallationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Uninstall)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AllowSourcePrompts(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowSourcePrompts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAllowSourcePrompts(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllowSourcePrompts)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RebootRequiredBeforeInstallation(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RebootRequiredBeforeInstallation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsForced: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsForced: usize,
    #[cfg(feature = "wtypes")]
    pub SetIsForced: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetIsForced: usize,
    #[cfg(feature = "windef")]
    pub ParentHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ParentHwnd: usize,
    #[cfg(feature = "windef")]
    pub SetParentHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetParentHwnd: usize,
    pub SetParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub BeginInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    BeginInstall: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub BeginUninstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    BeginUninstall: usize,
    pub EndInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndUninstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Install: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunWizard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsBusy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsBusy: usize,
    pub Uninstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub AllowSourcePrompts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AllowSourcePrompts: usize,
    #[cfg(feature = "wtypes")]
    pub SetAllowSourcePrompts: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAllowSourcePrompts: usize,
    #[cfg(feature = "wtypes")]
    pub RebootRequiredBeforeInstallation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RebootRequiredBeforeInstallation: usize,
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateInstaller_Impl: super::IDispatch_Impl {
    fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsForced(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetIsForced(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ParentHwnd(&self) -> windows_core::Result<super::HWND>;
    fn SetParentHwnd(&self, value: super::HWND) -> windows_core::Result<()>;
    fn SetParentWindow(&self, value: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ParentWindow(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Updates(&self) -> windows_core::Result<IUpdateCollection>;
    fn SetUpdates(&self, value: windows_core::Ref<IUpdateCollection>) -> windows_core::Result<()>;
    fn BeginInstall(&self, onprogresschanged: windows_core::Ref<windows_core::IUnknown>, oncompleted: windows_core::Ref<windows_core::IUnknown>, state: &super::VARIANT) -> windows_core::Result<IInstallationJob>;
    fn BeginUninstall(&self, onprogresschanged: windows_core::Ref<windows_core::IUnknown>, oncompleted: windows_core::Ref<windows_core::IUnknown>, state: &super::VARIANT) -> windows_core::Result<IInstallationJob>;
    fn EndInstall(&self, value: windows_core::Ref<IInstallationJob>) -> windows_core::Result<IInstallationResult>;
    fn EndUninstall(&self, value: windows_core::Ref<IInstallationJob>) -> windows_core::Result<IInstallationResult>;
    fn Install(&self) -> windows_core::Result<IInstallationResult>;
    fn RunWizard(&self, dialogtitle: &windows_core::BSTR) -> windows_core::Result<IInstallationResult>;
    fn IsBusy(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Uninstall(&self) -> windows_core::Result<IInstallationResult>;
    fn AllowSourcePrompts(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAllowSourcePrompts(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RebootRequiredBeforeInstallation(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateInstaller_Vtbl {
    pub const fn new<Identity: IUpdateInstaller_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClientApplicationID<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::ClientApplicationID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller_Impl::SetClientApplicationID(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn IsForced<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::IsForced(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsForced<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller_Impl::SetIsForced(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ParentHwnd<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::ParentHwnd(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParentHwnd<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller_Impl::SetParentHwnd(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetParentWindow<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller_Impl::SetParentWindow(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ParentWindow<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::ParentWindow(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Updates<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::Updates(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUpdates<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller_Impl::SetUpdates(this, core::mem::transmute_copy(&value)).into()
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
        unsafe extern "system" fn IsBusy<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::IsBusy(this) {
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
        unsafe extern "system" fn AllowSourcePrompts<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::AllowSourcePrompts(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowSourcePrompts<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller_Impl::SetAllowSourcePrompts(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn RebootRequiredBeforeInstallation<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::RebootRequiredBeforeInstallation(this) {
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
            ClientApplicationID: ClientApplicationID::<Identity, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, OFFSET>,
            IsForced: IsForced::<Identity, OFFSET>,
            SetIsForced: SetIsForced::<Identity, OFFSET>,
            ParentHwnd: ParentHwnd::<Identity, OFFSET>,
            SetParentHwnd: SetParentHwnd::<Identity, OFFSET>,
            SetParentWindow: SetParentWindow::<Identity, OFFSET>,
            ParentWindow: ParentWindow::<Identity, OFFSET>,
            Updates: Updates::<Identity, OFFSET>,
            SetUpdates: SetUpdates::<Identity, OFFSET>,
            BeginInstall: BeginInstall::<Identity, OFFSET>,
            BeginUninstall: BeginUninstall::<Identity, OFFSET>,
            EndInstall: EndInstall::<Identity, OFFSET>,
            EndUninstall: EndUninstall::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            RunWizard: RunWizard::<Identity, OFFSET>,
            IsBusy: IsBusy::<Identity, OFFSET>,
            Uninstall: Uninstall::<Identity, OFFSET>,
            AllowSourcePrompts: AllowSourcePrompts::<Identity, OFFSET>,
            SetAllowSourcePrompts: SetAllowSourcePrompts::<Identity, OFFSET>,
            RebootRequiredBeforeInstallation: RebootRequiredBeforeInstallation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn ForceQuiet(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ForceQuiet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetForceQuiet(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetForceQuiet)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller2_Vtbl {
    pub base__: IUpdateInstaller_Vtbl,
    #[cfg(feature = "wtypes")]
    pub ForceQuiet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ForceQuiet: usize,
    #[cfg(feature = "wtypes")]
    pub SetForceQuiet: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetForceQuiet: usize,
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateInstaller2_Impl: IUpdateInstaller_Impl {
    fn ForceQuiet(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetForceQuiet(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateInstaller2_Vtbl {
    pub const fn new<Identity: IUpdateInstaller2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ForceQuiet<Identity: IUpdateInstaller2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller2_Impl::ForceQuiet(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetForceQuiet<Identity: IUpdateInstaller2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller2_Impl::SetForceQuiet(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IUpdateInstaller_Vtbl::new::<Identity, OFFSET>(),
            ForceQuiet: ForceQuiet::<Identity, OFFSET>,
            SetForceQuiet: SetForceQuiet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateInstaller as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn AttemptCloseAppsIfNecessary(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AttemptCloseAppsIfNecessary)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAttemptCloseAppsIfNecessary(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAttemptCloseAppsIfNecessary)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller3_Vtbl {
    pub base__: IUpdateInstaller2_Vtbl,
    #[cfg(feature = "wtypes")]
    pub AttemptCloseAppsIfNecessary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AttemptCloseAppsIfNecessary: usize,
    #[cfg(feature = "wtypes")]
    pub SetAttemptCloseAppsIfNecessary: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAttemptCloseAppsIfNecessary: usize,
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateInstaller3_Impl: IUpdateInstaller2_Impl {
    fn AttemptCloseAppsIfNecessary(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAttemptCloseAppsIfNecessary(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateInstaller3_Vtbl {
    pub const fn new<Identity: IUpdateInstaller3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AttemptCloseAppsIfNecessary<Identity: IUpdateInstaller3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller3_Impl::AttemptCloseAppsIfNecessary(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttemptCloseAppsIfNecessary<Identity: IUpdateInstaller3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateInstaller3_Impl::SetAttemptCloseAppsIfNecessary(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IUpdateInstaller2_Vtbl::new::<Identity, OFFSET>(),
            AttemptCloseAppsIfNecessary: AttemptCloseAppsIfNecessary::<Identity, OFFSET>,
            SetAttemptCloseAppsIfNecessary: SetAttemptCloseAppsIfNecessary::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateInstaller as windows_core::Interface>::IID || iid == &<IUpdateInstaller2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Commit(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), dwflags) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller4_Vtbl {
    pub base__: IUpdateInstaller3_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateInstaller4_Impl: IUpdateInstaller3_Impl {
    fn Commit(&self, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateInstaller4_Vtbl {
    pub const fn new<Identity: IUpdateInstaller4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: IUpdateInstaller4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUpdateInstaller4 {}
windows_core::imp::define_interface!(IUpdateLockdown, IUpdateLockdown_Vtbl, 0xa976c28d_75a1_42aa_94ae_8af8b872089a);
windows_core::imp::interface_hierarchy!(IUpdateLockdown, windows_core::IUnknown);
impl IUpdateLockdown {
    pub unsafe fn LockDown(&self, flags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockDown)(windows_core::Interface::as_raw(self), flags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateLockdown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LockDown: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IUpdateLockdown_Impl: windows_core::IUnknownImpl {
    fn LockDown(&self, flags: i32) -> windows_core::Result<()>;
}
impl IUpdateLockdown_Vtbl {
    pub const fn new<Identity: IUpdateLockdown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LockDown<Identity: IUpdateLockdown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
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
    pub unsafe fn CanAutomaticallyUpgradeService(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanAutomaticallyUpgradeService)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetCanAutomaticallyUpgradeService(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCanAutomaticallyUpgradeService)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientApplicationID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IncludePotentiallySupersededUpdates(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncludePotentiallySupersededUpdates)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetIncludePotentiallySupersededUpdates(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIncludePotentiallySupersededUpdates)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn ServerSelection(&self) -> windows_core::Result<ServerSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServerSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetServerSelection(&self, value: ServerSelection) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetServerSelection)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn BeginSearch<P1>(&self, criteria: &windows_core::BSTR, oncompleted: P1, state: &super::VARIANT) -> windows_core::Result<ISearchJob>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginSearch)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(criteria), oncompleted.param().abi(), core::mem::transmute_copy(state), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EndSearch<P0>(&self, searchjob: P0) -> windows_core::Result<ISearchResult>
    where
        P0: windows_core::Param<ISearchJob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndSearch)(windows_core::Interface::as_raw(self), searchjob.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EscapeString(&self, unescaped: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EscapeString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(unescaped), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn QueryHistory(&self, startindex: i32, count: i32) -> windows_core::Result<IUpdateHistoryEntryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHistory)(windows_core::Interface::as_raw(self), startindex, count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Search(&self, criteria: &windows_core::BSTR) -> windows_core::Result<ISearchResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Search)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(criteria), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Online(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Online)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetOnline(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOnline)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetTotalHistoryCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTotalHistoryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetServiceID(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetServiceID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSearcher_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub CanAutomaticallyUpgradeService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CanAutomaticallyUpgradeService: usize,
    #[cfg(feature = "wtypes")]
    pub SetCanAutomaticallyUpgradeService: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetCanAutomaticallyUpgradeService: usize,
    pub ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IncludePotentiallySupersededUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IncludePotentiallySupersededUpdates: usize,
    #[cfg(feature = "wtypes")]
    pub SetIncludePotentiallySupersededUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetIncludePotentiallySupersededUpdates: usize,
    pub ServerSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ServerSelection) -> windows_core::HRESULT,
    pub SetServerSelection: unsafe extern "system" fn(*mut core::ffi::c_void, ServerSelection) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub BeginSearch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    BeginSearch: usize,
    pub EndSearch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EscapeString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryHistory: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Search: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Online: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Online: usize,
    #[cfg(feature = "wtypes")]
    pub SetOnline: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetOnline: usize,
    pub GetTotalHistoryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateSearcher_Impl: super::IDispatch_Impl {
    fn CanAutomaticallyUpgradeService(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetCanAutomaticallyUpgradeService(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IncludePotentiallySupersededUpdates(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetIncludePotentiallySupersededUpdates(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ServerSelection(&self) -> windows_core::Result<ServerSelection>;
    fn SetServerSelection(&self, value: ServerSelection) -> windows_core::Result<()>;
    fn BeginSearch(&self, criteria: &windows_core::BSTR, oncompleted: windows_core::Ref<windows_core::IUnknown>, state: &super::VARIANT) -> windows_core::Result<ISearchJob>;
    fn EndSearch(&self, searchjob: windows_core::Ref<ISearchJob>) -> windows_core::Result<ISearchResult>;
    fn EscapeString(&self, unescaped: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn QueryHistory(&self, startindex: i32, count: i32) -> windows_core::Result<IUpdateHistoryEntryCollection>;
    fn Search(&self, criteria: &windows_core::BSTR) -> windows_core::Result<ISearchResult>;
    fn Online(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetOnline(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetTotalHistoryCount(&self) -> windows_core::Result<i32>;
    fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateSearcher_Vtbl {
    pub const fn new<Identity: IUpdateSearcher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanAutomaticallyUpgradeService<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::CanAutomaticallyUpgradeService(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCanAutomaticallyUpgradeService<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher_Impl::SetCanAutomaticallyUpgradeService(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ClientApplicationID<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::ClientApplicationID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher_Impl::SetClientApplicationID(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn IncludePotentiallySupersededUpdates<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::IncludePotentiallySupersededUpdates(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIncludePotentiallySupersededUpdates<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher_Impl::SetIncludePotentiallySupersededUpdates(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ServerSelection<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut ServerSelection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::ServerSelection(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServerSelection<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ServerSelection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher_Impl::SetServerSelection(this, core::mem::transmute_copy(&value)).into()
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
        unsafe extern "system" fn QueryHistory<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: i32, count: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Online<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::Online(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOnline<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher_Impl::SetOnline(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetTotalHistoryCount<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ServiceID<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher_Impl::ServiceID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServiceID<Identity: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher_Impl::SetServiceID(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CanAutomaticallyUpgradeService: CanAutomaticallyUpgradeService::<Identity, OFFSET>,
            SetCanAutomaticallyUpgradeService: SetCanAutomaticallyUpgradeService::<Identity, OFFSET>,
            ClientApplicationID: ClientApplicationID::<Identity, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, OFFSET>,
            IncludePotentiallySupersededUpdates: IncludePotentiallySupersededUpdates::<Identity, OFFSET>,
            SetIncludePotentiallySupersededUpdates: SetIncludePotentiallySupersededUpdates::<Identity, OFFSET>,
            ServerSelection: ServerSelection::<Identity, OFFSET>,
            SetServerSelection: SetServerSelection::<Identity, OFFSET>,
            BeginSearch: BeginSearch::<Identity, OFFSET>,
            EndSearch: EndSearch::<Identity, OFFSET>,
            EscapeString: EscapeString::<Identity, OFFSET>,
            QueryHistory: QueryHistory::<Identity, OFFSET>,
            Search: Search::<Identity, OFFSET>,
            Online: Online::<Identity, OFFSET>,
            SetOnline: SetOnline::<Identity, OFFSET>,
            GetTotalHistoryCount: GetTotalHistoryCount::<Identity, OFFSET>,
            ServiceID: ServiceID::<Identity, OFFSET>,
            SetServiceID: SetServiceID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSearcher as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn IgnoreDownloadPriority(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IgnoreDownloadPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetIgnoreDownloadPriority(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIgnoreDownloadPriority)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSearcher2_Vtbl {
    pub base__: IUpdateSearcher_Vtbl,
    #[cfg(feature = "wtypes")]
    pub IgnoreDownloadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IgnoreDownloadPriority: usize,
    #[cfg(feature = "wtypes")]
    pub SetIgnoreDownloadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetIgnoreDownloadPriority: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateSearcher2_Impl: IUpdateSearcher_Impl {
    fn IgnoreDownloadPriority(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetIgnoreDownloadPriority(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateSearcher2_Vtbl {
    pub const fn new<Identity: IUpdateSearcher2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IgnoreDownloadPriority<Identity: IUpdateSearcher2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher2_Impl::IgnoreDownloadPriority(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIgnoreDownloadPriority<Identity: IUpdateSearcher2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher2_Impl::SetIgnoreDownloadPriority(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IUpdateSearcher_Vtbl::new::<Identity, OFFSET>(),
            IgnoreDownloadPriority: IgnoreDownloadPriority::<Identity, OFFSET>,
            SetIgnoreDownloadPriority: SetIgnoreDownloadPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSearcher2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateSearcher as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn SearchScope(&self) -> windows_core::Result<SearchScope> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchScope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSearchScope(&self, value: SearchScope) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSearchScope)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSearcher3_Vtbl {
    pub base__: IUpdateSearcher2_Vtbl,
    pub SearchScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SearchScope) -> windows_core::HRESULT,
    pub SetSearchScope: unsafe extern "system" fn(*mut core::ffi::c_void, SearchScope) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateSearcher3_Impl: IUpdateSearcher2_Impl {
    fn SearchScope(&self) -> windows_core::Result<SearchScope>;
    fn SetSearchScope(&self, value: SearchScope) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateSearcher3_Vtbl {
    pub const fn new<Identity: IUpdateSearcher3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SearchScope<Identity: IUpdateSearcher3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut SearchScope) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSearcher3_Impl::SearchScope(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSearchScope<Identity: IUpdateSearcher3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SearchScope) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSearcher3_Impl::SetSearchScope(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: IUpdateSearcher2_Vtbl::new::<Identity, OFFSET>(),
            SearchScope: SearchScope::<Identity, OFFSET>,
            SetSearchScope: SetSearchScope::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSearcher3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateSearcher as windows_core::Interface>::IID || iid == &<IUpdateSearcher2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ContentValidationCert(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentValidationCert)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ExpirationDate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpirationDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsManaged(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsManaged)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsRegisteredWithAU(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRegisteredWithAU)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IssueDate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IssueDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn OffersWindowsUpdates(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OffersWindowsUpdates)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RedirectUrls(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RedirectUrls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsScanPackageService(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsScanPackageService)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CanRegisterWithAU(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanRegisterWithAU)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ServiceUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetupPrefix(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetupPrefix)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateService_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ContentValidationCert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ContentValidationCert: usize,
    pub ExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsManaged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsManaged: usize,
    #[cfg(feature = "wtypes")]
    pub IsRegisteredWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsRegisteredWithAU: usize,
    pub IssueDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub OffersWindowsUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    OffersWindowsUpdates: usize,
    pub RedirectUrls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsScanPackageService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsScanPackageService: usize,
    #[cfg(feature = "wtypes")]
    pub CanRegisterWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CanRegisterWithAU: usize,
    pub ServiceUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetupPrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateService_Impl: super::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ContentValidationCert(&self) -> windows_core::Result<super::VARIANT>;
    fn ExpirationDate(&self) -> windows_core::Result<f64>;
    fn IsManaged(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn IsRegisteredWithAU(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn IssueDate(&self) -> windows_core::Result<f64>;
    fn OffersWindowsUpdates(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn RedirectUrls(&self) -> windows_core::Result<IStringCollection>;
    fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsScanPackageService(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn CanRegisterWithAU(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn ServiceUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetupPrefix(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateService_Vtbl {
    pub const fn new<Identity: IUpdateService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::Name(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContentValidationCert<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::ContentValidationCert(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpirationDate<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::ExpirationDate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsManaged<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::IsManaged(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsRegisteredWithAU<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::IsRegisteredWithAU(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IssueDate<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::IssueDate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OffersWindowsUpdates<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::OffersWindowsUpdates(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RedirectUrls<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::RedirectUrls(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceID<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::ServiceID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsScanPackageService<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::IsScanPackageService(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanRegisterWithAU<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::CanRegisterWithAU(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceUrl<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::ServiceUrl(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetupPrefix<Identity: IUpdateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService_Impl::SetupPrefix(this) {
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
            Name: Name::<Identity, OFFSET>,
            ContentValidationCert: ContentValidationCert::<Identity, OFFSET>,
            ExpirationDate: ExpirationDate::<Identity, OFFSET>,
            IsManaged: IsManaged::<Identity, OFFSET>,
            IsRegisteredWithAU: IsRegisteredWithAU::<Identity, OFFSET>,
            IssueDate: IssueDate::<Identity, OFFSET>,
            OffersWindowsUpdates: OffersWindowsUpdates::<Identity, OFFSET>,
            RedirectUrls: RedirectUrls::<Identity, OFFSET>,
            ServiceID: ServiceID::<Identity, OFFSET>,
            IsScanPackageService: IsScanPackageService::<Identity, OFFSET>,
            CanRegisterWithAU: CanRegisterWithAU::<Identity, OFFSET>,
            ServiceUrl: ServiceUrl::<Identity, OFFSET>,
            SetupPrefix: SetupPrefix::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateService as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn IsDefaultAUService(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDefaultAUService)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateService2_Vtbl {
    pub base__: IUpdateService_Vtbl,
    #[cfg(feature = "wtypes")]
    pub IsDefaultAUService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsDefaultAUService: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateService2_Impl: IUpdateService_Impl {
    fn IsDefaultAUService(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateService2_Vtbl {
    pub const fn new<Identity: IUpdateService2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDefaultAUService<Identity: IUpdateService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateService2_Impl::IsDefaultAUService(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUpdateService_Vtbl::new::<Identity, OFFSET>(), IsDefaultAUService: IsDefaultAUService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateService2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateService as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<IUpdateService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateServiceCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateServiceCollection_Impl: super::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<IUpdateService>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateServiceCollection_Vtbl {
    pub const fn new<Identity: IUpdateServiceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceCollection_Impl::Count(this) {
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
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateServiceCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Services(&self) -> windows_core::Result<IUpdateServiceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Services)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddService(&self, serviceid: &windows_core::BSTR, authorizationcabpath: &windows_core::BSTR) -> windows_core::Result<IUpdateService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddService)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serviceid), core::mem::transmute_copy(authorizationcabpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub unsafe fn AddScanPackageService(&self, servicename: &windows_core::BSTR, scanfilelocation: &windows_core::BSTR, flags: i32) -> windows_core::Result<IUpdateService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddScanPackageService)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename), core::mem::transmute_copy(scanfilelocation), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetOption(&self, optionname: &windows_core::BSTR, optionvalue: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOption)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(optionname), core::mem::transmute_copy(optionvalue)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateServiceManager_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Services: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterServiceWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterServiceWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddScanPackageService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetOption: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateServiceManager_Impl: super::IDispatch_Impl {
    fn Services(&self) -> windows_core::Result<IUpdateServiceCollection>;
    fn AddService(&self, serviceid: &windows_core::BSTR, authorizationcabpath: &windows_core::BSTR) -> windows_core::Result<IUpdateService>;
    fn RegisterServiceWithAU(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveService(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UnregisterServiceWithAU(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddScanPackageService(&self, servicename: &windows_core::BSTR, scanfilelocation: &windows_core::BSTR, flags: i32) -> windows_core::Result<IUpdateService>;
    fn SetOption(&self, optionname: &windows_core::BSTR, optionvalue: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateServiceManager_Vtbl {
    pub const fn new<Identity: IUpdateServiceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Services<Identity: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceManager_Impl::Services(this) {
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
        unsafe extern "system" fn AddScanPackageService<Identity: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void, scanfilelocation: *mut core::ffi::c_void, flags: i32, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
            Services: Services::<Identity, OFFSET>,
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
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientApplicationID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn QueryServiceRegistration(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<IUpdateServiceRegistration> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryServiceRegistration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddService2(&self, serviceid: &windows_core::BSTR, flags: i32, authorizationcabpath: &windows_core::BSTR) -> windows_core::Result<IUpdateServiceRegistration> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddService2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serviceid), flags, core::mem::transmute_copy(authorizationcabpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateServiceManager2_Vtbl {
    pub base__: IUpdateServiceManager_Vtbl,
    pub ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryServiceRegistration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddService2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateServiceManager2_Impl: IUpdateServiceManager_Impl {
    fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn QueryServiceRegistration(&self, serviceid: &windows_core::BSTR) -> windows_core::Result<IUpdateServiceRegistration>;
    fn AddService2(&self, serviceid: &windows_core::BSTR, flags: i32, authorizationcabpath: &windows_core::BSTR) -> windows_core::Result<IUpdateServiceRegistration>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateServiceManager2_Vtbl {
    pub const fn new<Identity: IUpdateServiceManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClientApplicationID<Identity: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceManager2_Impl::ClientApplicationID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateServiceManager2_Impl::SetClientApplicationID(this, core::mem::transmute(&value)).into()
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
        unsafe extern "system" fn AddService2<Identity: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: *mut core::ffi::c_void, flags: i32, authorizationcabpath: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
            ClientApplicationID: ClientApplicationID::<Identity, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, OFFSET>,
            QueryServiceRegistration: QueryServiceRegistration::<Identity, OFFSET>,
            AddService2: AddService2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateServiceManager2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateServiceManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn RegistrationState(&self) -> windows_core::Result<UpdateServiceRegistrationState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegistrationState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsPendingRegistrationWithAU(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPendingRegistrationWithAU)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Service(&self) -> windows_core::Result<IUpdateService2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Service)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateServiceRegistration_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub RegistrationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UpdateServiceRegistrationState) -> windows_core::HRESULT,
    pub ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsPendingRegistrationWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsPendingRegistrationWithAU: usize,
    pub Service: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateServiceRegistration_Impl: super::IDispatch_Impl {
    fn RegistrationState(&self) -> windows_core::Result<UpdateServiceRegistrationState>;
    fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsPendingRegistrationWithAU(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Service(&self) -> windows_core::Result<IUpdateService2>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateServiceRegistration_Vtbl {
    pub const fn new<Identity: IUpdateServiceRegistration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegistrationState<Identity: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UpdateServiceRegistrationState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceRegistration_Impl::RegistrationState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceID<Identity: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceRegistration_Impl::ServiceID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPendingRegistrationWithAU<Identity: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceRegistration_Impl::IsPendingRegistrationWithAU(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Service<Identity: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateServiceRegistration_Impl::Service(this) {
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
            RegistrationState: RegistrationState::<Identity, OFFSET>,
            ServiceID: ServiceID::<Identity, OFFSET>,
            IsPendingRegistrationWithAU: IsPendingRegistrationWithAU::<Identity, OFFSET>,
            Service: Service::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateServiceRegistration as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientApplicationID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WebProxy(&self) -> windows_core::Result<IWebProxy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WebProxy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetWebProxy<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWebProxy>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWebProxy)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn CreateUpdateSearcher(&self) -> windows_core::Result<IUpdateSearcher> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUpdateSearcher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateUpdateDownloader(&self) -> windows_core::Result<IUpdateDownloader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUpdateDownloader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateUpdateInstaller(&self) -> windows_core::Result<IUpdateInstaller> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUpdateInstaller)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSession_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ReadOnly: usize,
    pub WebProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWebProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUpdateSearcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUpdateDownloader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUpdateInstaller: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateSession_Impl: super::IDispatch_Impl {
    fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn WebProxy(&self) -> windows_core::Result<IWebProxy>;
    fn SetWebProxy(&self, value: windows_core::Ref<IWebProxy>) -> windows_core::Result<()>;
    fn CreateUpdateSearcher(&self) -> windows_core::Result<IUpdateSearcher>;
    fn CreateUpdateDownloader(&self) -> windows_core::Result<IUpdateDownloader>;
    fn CreateUpdateInstaller(&self) -> windows_core::Result<IUpdateInstaller>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateSession_Vtbl {
    pub const fn new<Identity: IUpdateSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClientApplicationID<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession_Impl::ClientApplicationID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSession_Impl::SetClientApplicationID(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession_Impl::ReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WebProxy<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession_Impl::WebProxy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWebProxy<Identity: IUpdateSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSession_Impl::SetWebProxy(this, core::mem::transmute_copy(&value)).into()
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
            ClientApplicationID: ClientApplicationID::<Identity, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            WebProxy: WebProxy::<Identity, OFFSET>,
            SetWebProxy: SetWebProxy::<Identity, OFFSET>,
            CreateUpdateSearcher: CreateUpdateSearcher::<Identity, OFFSET>,
            CreateUpdateDownloader: CreateUpdateDownloader::<Identity, OFFSET>,
            CreateUpdateInstaller: CreateUpdateInstaller::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSession as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "winnt")]
    pub unsafe fn UserLocale(&self) -> windows_core::Result<super::LCID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserLocale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SetUserLocale(&self, lcid: super::LCID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUserLocale)(windows_core::Interface::as_raw(self), lcid) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSession2_Vtbl {
    pub base__: IUpdateSession_Vtbl,
    #[cfg(feature = "winnt")]
    pub UserLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    UserLocale: usize,
    #[cfg(feature = "winnt")]
    pub SetUserLocale: unsafe extern "system" fn(*mut core::ffi::c_void, super::LCID) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetUserLocale: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateSession2_Impl: IUpdateSession_Impl {
    fn UserLocale(&self) -> windows_core::Result<super::LCID>;
    fn SetUserLocale(&self, lcid: super::LCID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUpdateSession2_Vtbl {
    pub const fn new<Identity: IUpdateSession2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UserLocale<Identity: IUpdateSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateSession2_Impl::UserLocale(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUserLocale<Identity: IUpdateSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: super::LCID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUpdateSession2_Impl::SetUserLocale(this, core::mem::transmute_copy(&lcid)).into()
            }
        }
        Self {
            base__: IUpdateSession_Vtbl::new::<Identity, OFFSET>(),
            UserLocale: UserLocale::<Identity, OFFSET>,
            SetUserLocale: SetUserLocale::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateSession2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdateSession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
            (windows_core::Interface::vtable(self).CreateUpdateServiceManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QueryHistory(&self, criteria: &windows_core::BSTR, startindex: i32, count: i32) -> windows_core::Result<IUpdateHistoryEntryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHistory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(criteria), startindex, count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSession3_Vtbl {
    pub base__: IUpdateSession2_Vtbl,
    pub CreateUpdateServiceManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryHistory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUpdateSession3_Impl: IUpdateSession2_Impl {
    fn CreateUpdateServiceManager(&self) -> windows_core::Result<IUpdateServiceManager2>;
    fn QueryHistory(&self, criteria: &windows_core::BSTR, startindex: i32, count: i32) -> windows_core::Result<IUpdateHistoryEntryCollection>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn QueryHistory<Identity: IUpdateSession3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, criteria: *mut core::ffi::c_void, startindex: i32, count: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Address(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetAddress(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAddress)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn BypassList(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BypassList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBypassList<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStringCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBypassList)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn BypassProxyOnLocal(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BypassProxyOnLocal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetBypassProxyOnLocal(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBypassProxyOnLocal)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetUserName(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUserName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
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
    pub unsafe fn PromptForCredentialsFromHwnd(&self, parentwindow: super::HWND, title: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PromptForCredentialsFromHwnd)(windows_core::Interface::as_raw(self), parentwindow, core::mem::transmute_copy(title)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AutoDetect(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoDetect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAutoDetect(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoDetect)(windows_core::Interface::as_raw(self), value) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWebProxy_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BypassList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBypassList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub BypassProxyOnLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    BypassProxyOnLocal: usize,
    #[cfg(feature = "wtypes")]
    pub SetBypassProxyOnLocal: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetBypassProxyOnLocal: usize,
    #[cfg(feature = "wtypes")]
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ReadOnly: usize,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PromptForCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub PromptForCredentialsFromHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    PromptForCredentialsFromHwnd: usize,
    #[cfg(feature = "wtypes")]
    pub AutoDetect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AutoDetect: usize,
    #[cfg(feature = "wtypes")]
    pub SetAutoDetect: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAutoDetect: usize,
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWebProxy_Impl: super::IDispatch_Impl {
    fn Address(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAddress(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn BypassList(&self) -> windows_core::Result<IStringCollection>;
    fn SetBypassList(&self, value: windows_core::Ref<IStringCollection>) -> windows_core::Result<()>;
    fn BypassProxyOnLocal(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetBypassProxyOnLocal(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ReadOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetUserName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetPassword(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PromptForCredentials(&self, parentwindow: windows_core::Ref<windows_core::IUnknown>, title: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PromptForCredentialsFromHwnd(&self, parentwindow: super::HWND, title: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AutoDetect(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAutoDetect(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWebProxy_Vtbl {
    pub const fn new<Identity: IWebProxy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Address<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebProxy_Impl::Address(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAddress<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::SetAddress(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn BypassList<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebProxy_Impl::BypassList(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBypassList<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::SetBypassList(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BypassProxyOnLocal<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebProxy_Impl::BypassProxyOnLocal(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBypassProxyOnLocal<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::SetBypassProxyOnLocal(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebProxy_Impl::ReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserName<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebProxy_Impl::UserName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUserName<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::SetUserName(this, core::mem::transmute(&value)).into()
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
        unsafe extern "system" fn AutoDetect<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebProxy_Impl::AutoDetect(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutoDetect<Identity: IWebProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebProxy_Impl::SetAutoDetect(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Address: Address::<Identity, OFFSET>,
            SetAddress: SetAddress::<Identity, OFFSET>,
            BypassList: BypassList::<Identity, OFFSET>,
            SetBypassList: SetBypassList::<Identity, OFFSET>,
            BypassProxyOnLocal: BypassProxyOnLocal::<Identity, OFFSET>,
            SetBypassProxyOnLocal: SetBypassProxyOnLocal::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            UserName: UserName::<Identity, OFFSET>,
            SetUserName: SetUserName::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
            PromptForCredentials: PromptForCredentials::<Identity, OFFSET>,
            PromptForCredentialsFromHwnd: PromptForCredentialsFromHwnd::<Identity, OFFSET>,
            AutoDetect: AutoDetect::<Identity, OFFSET>,
            SetAutoDetect: SetAutoDetect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebProxy as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn DriverClass(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DriverHardwareID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverHardwareID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DriverManufacturer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverManufacturer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DriverModel(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverModel)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DriverProvider(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverProvider)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DriverVerDate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverVerDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceProblemNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceProblemNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceStatus(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate_Vtbl {
    pub base__: IUpdate_Vtbl,
    pub DriverClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DriverHardwareID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DriverManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DriverModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DriverProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DriverVerDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub DeviceProblemNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdate_Impl: IUpdate_Impl {
    fn DriverClass(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverHardwareID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverManufacturer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverModel(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverProvider(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverVerDate(&self) -> windows_core::Result<f64>;
    fn DeviceProblemNumber(&self) -> windows_core::Result<i32>;
    fn DeviceStatus(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdate_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DriverClass<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::DriverClass(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DriverHardwareID<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::DriverHardwareID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DriverManufacturer<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::DriverManufacturer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DriverModel<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::DriverModel(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DriverProvider<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::DriverProvider(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DriverVerDate<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::DriverVerDate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceProblemNumber<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::DeviceProblemNumber(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceStatus<Identity: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate_Impl::DeviceStatus(this) {
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
            DriverClass: DriverClass::<Identity, OFFSET>,
            DriverHardwareID: DriverHardwareID::<Identity, OFFSET>,
            DriverManufacturer: DriverManufacturer::<Identity, OFFSET>,
            DriverModel: DriverModel::<Identity, OFFSET>,
            DriverProvider: DriverProvider::<Identity, OFFSET>,
            DriverVerDate: DriverVerDate::<Identity, OFFSET>,
            DeviceProblemNumber: DeviceProblemNumber::<Identity, OFFSET>,
            DeviceStatus: DeviceStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsPresent(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPresent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CveIDs(&self) -> windows_core::Result<IStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CveIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RebootRequired: usize,
    #[cfg(feature = "wtypes")]
    pub IsPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsPresent: usize,
    pub CveIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyToCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdate2_Impl: IWindowsDriverUpdate_Impl {
    fn RebootRequired(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn IsPresent(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn CveIDs(&self) -> windows_core::Result<IStringCollection>;
    fn CopyToCache(&self, pfiles: windows_core::Ref<IStringCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdate2_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdate2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RebootRequired<Identity: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate2_Impl::RebootRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPresent<Identity: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate2_Impl::IsPresent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CveIDs<Identity: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate2_Impl::CveIDs(this) {
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
            RebootRequired: RebootRequired::<Identity, OFFSET>,
            IsPresent: IsPresent::<Identity, OFFSET>,
            CveIDs: CveIDs::<Identity, OFFSET>,
            CopyToCache: CopyToCache::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn BrowseOnly(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BrowseOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate3_Vtbl {
    pub base__: IWindowsDriverUpdate2_Vtbl,
    #[cfg(feature = "wtypes")]
    pub BrowseOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    BrowseOnly: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdate3_Impl: IWindowsDriverUpdate2_Impl {
    fn BrowseOnly(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdate3_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdate3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BrowseOnly<Identity: IWindowsDriverUpdate3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate3_Impl::BrowseOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWindowsDriverUpdate2_Vtbl::new::<Identity, OFFSET>(), BrowseOnly: BrowseOnly::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn WindowsDriverUpdateEntries(&self) -> windows_core::Result<IWindowsDriverUpdateEntryCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowsDriverUpdateEntries)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn PerUser(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PerUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate4_Vtbl {
    pub base__: IWindowsDriverUpdate3_Vtbl,
    pub WindowsDriverUpdateEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub PerUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    PerUser: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdate4_Impl: IWindowsDriverUpdate3_Impl {
    fn WindowsDriverUpdateEntries(&self) -> windows_core::Result<IWindowsDriverUpdateEntryCollection>;
    fn PerUser(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdate4_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdate4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WindowsDriverUpdateEntries<Identity: IWindowsDriverUpdate4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate4_Impl::WindowsDriverUpdateEntries(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PerUser<Identity: IWindowsDriverUpdate4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate4_Impl::PerUser(this) {
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
            WindowsDriverUpdateEntries: WindowsDriverUpdateEntries::<Identity, OFFSET>,
            PerUser: PerUser::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate4 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate2 as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn AutoSelection(&self) -> windows_core::Result<AutoSelectionMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AutoDownload(&self) -> windows_core::Result<AutoDownloadMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoDownload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate5_Vtbl {
    pub base__: IWindowsDriverUpdate4_Vtbl,
    pub AutoSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoSelectionMode) -> windows_core::HRESULT,
    pub AutoDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoDownloadMode) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdate5_Impl: IWindowsDriverUpdate4_Impl {
    fn AutoSelection(&self) -> windows_core::Result<AutoSelectionMode>;
    fn AutoDownload(&self) -> windows_core::Result<AutoDownloadMode>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdate5_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdate5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AutoSelection<Identity: IWindowsDriverUpdate5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutoSelectionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate5_Impl::AutoSelection(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AutoDownload<Identity: IWindowsDriverUpdate5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut AutoDownloadMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdate5_Impl::AutoDownload(this) {
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
            AutoSelection: AutoSelection::<Identity, OFFSET>,
            AutoDownload: AutoDownload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate5 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate2 as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate3 as windows_core::Interface>::IID || iid == &<IWindowsDriverUpdate4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn DriverClass(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DriverHardwareID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverHardwareID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DriverManufacturer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverManufacturer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DriverModel(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverModel)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DriverProvider(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverProvider)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DriverVerDate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DriverVerDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceProblemNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceProblemNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceStatus(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdateEntry_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub DriverClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DriverHardwareID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DriverManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DriverModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DriverProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DriverVerDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub DeviceProblemNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdateEntry_Impl: super::IDispatch_Impl {
    fn DriverClass(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverHardwareID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverManufacturer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverModel(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverProvider(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DriverVerDate(&self) -> windows_core::Result<f64>;
    fn DeviceProblemNumber(&self) -> windows_core::Result<i32>;
    fn DeviceStatus(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdateEntry_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DriverClass<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::DriverClass(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DriverHardwareID<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::DriverHardwareID(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DriverManufacturer<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::DriverManufacturer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DriverModel<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::DriverModel(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DriverProvider<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::DriverProvider(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DriverVerDate<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::DriverVerDate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceProblemNumber<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::DeviceProblemNumber(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceStatus<Identity: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntry_Impl::DeviceStatus(this) {
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
            DriverClass: DriverClass::<Identity, OFFSET>,
            DriverHardwareID: DriverHardwareID::<Identity, OFFSET>,
            DriverManufacturer: DriverManufacturer::<Identity, OFFSET>,
            DriverModel: DriverModel::<Identity, OFFSET>,
            DriverProvider: DriverProvider::<Identity, OFFSET>,
            DriverVerDate: DriverVerDate::<Identity, OFFSET>,
            DeviceProblemNumber: DeviceProblemNumber::<Identity, OFFSET>,
            DeviceStatus: DeviceStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdateEntry as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<IWindowsDriverUpdateEntry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdateEntryCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsDriverUpdateEntryCollection_Impl: super::IDispatch_Impl {
    fn Item(&self, index: i32) -> windows_core::Result<IWindowsDriverUpdateEntry>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWindowsDriverUpdateEntryCollection_Vtbl {
    pub const fn new<Identity: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntryCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntryCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsDriverUpdateEntryCollection_Impl::Count(this) {
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
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdateEntryCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetInfo: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWindowsUpdateAgentInfo_Impl: super::IDispatch_Impl {
    fn GetInfo(&self, varinfoidentifier: &super::VARIANT) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
pub const UPDATE_LOCKDOWN_WEBSITE_ACCESS: u32 = 1;
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

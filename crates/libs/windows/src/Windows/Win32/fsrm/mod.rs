pub const FSRM_DISPID_ACTION: u32 = 19922944;
pub const FSRM_DISPID_ACTION_COMMAND: u32 = 20185088;
pub const FSRM_DISPID_ACTION_EMAIL: u32 = 19988480;
pub const FSRM_DISPID_ACTION_EMAIL2: u32 = 20250624;
pub const FSRM_DISPID_ACTION_EVENTLOG: u32 = 20119552;
pub const FSRM_DISPID_ACTION_REPORT: u32 = 20054016;
pub const FSRM_DISPID_ADR: u32 = 25165824;
pub const FSRM_DISPID_COLLECTION: u32 = 18874368;
pub const FSRM_DISPID_COLLECTION_COMMITTABLE: u32 = 18944000;
pub const FSRM_DISPID_COLLECTION_MUTABLE: u32 = 18939904;
pub const FSRM_DISPID_DERIVEDOBJECTSRESULT: u32 = 24117248;
pub const FSRM_DISPID_EXPORTIMPORT: u32 = 23068672;
pub const FSRM_DISPID_FEATURE_CLASSIFICATION: u32 = 83886080;
pub const FSRM_DISPID_FEATURE_FILESCREEN: u32 = 50331648;
pub const FSRM_DISPID_FEATURE_GENERAL: u32 = 16777216;
pub const FSRM_DISPID_FEATURE_MASK: u32 = 251658240;
pub const FSRM_DISPID_FEATURE_PIPELINE: u32 = 100663296;
pub const FSRM_DISPID_FEATURE_QUOTA: u32 = 33554432;
pub const FSRM_DISPID_FEATURE_REPORTS: u32 = 67108864;
pub const FSRM_DISPID_INTERFACE_A_MASK: u32 = 15728640;
pub const FSRM_DISPID_INTERFACE_B_MASK: u32 = 983040;
pub const FSRM_DISPID_INTERFACE_C_MASK: u32 = 61440;
pub const FSRM_DISPID_INTERFACE_D_MASK: u32 = 3840;
pub const FSRM_DISPID_INTERFACE_MASK: u32 = 16776960;
pub const FSRM_DISPID_IS_PROPERTY: u32 = 128;
pub const FSRM_DISPID_METHOD_MASK: u32 = 255;
pub const FSRM_DISPID_METHOD_NUM_MASK: u32 = 127;
pub const FSRM_DISPID_OBJECT: u32 = 17825792;
pub const FSRM_DISPID_PATHMAPPER: u32 = 22020096;
pub const FSRM_DISPID_SETTING: u32 = 20971520;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmAccessDeniedRemediationClient, IFsrmAccessDeniedRemediationClient_Vtbl, 0x40002314_590b_45a5_8e1b_8c05da527e52);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmAccessDeniedRemediationClient {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmAccessDeniedRemediationClient, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmAccessDeniedRemediationClient {
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn Show(&self, parentwnd: usize, accesspath: &windows_core::BSTR, errortype: super::fsrmenums::AdrClientErrorType, flags: i32, windowtitle: &windows_core::BSTR, windowmessage: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), parentwnd, core::mem::transmute_copy(accesspath), errortype, flags, core::mem::transmute_copy(windowtitle), core::mem::transmute_copy(windowmessage), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAccessDeniedRemediationClient_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut core::ffi::c_void, super::fsrmenums::AdrClientErrorType, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    Show: usize,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmAccessDeniedRemediationClient_Impl: super::oaidl::IDispatch_Impl {
    fn Show(&self, parentwnd: usize, accesspath: &windows_core::BSTR, errortype: super::fsrmenums::AdrClientErrorType, flags: i32, windowtitle: &windows_core::BSTR, windowmessage: &windows_core::BSTR) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmAccessDeniedRemediationClient_Vtbl {
    pub const fn new<Identity: IFsrmAccessDeniedRemediationClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Show<Identity: IFsrmAccessDeniedRemediationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentwnd: usize, accesspath: *mut core::ffi::c_void, errortype: super::fsrmenums::AdrClientErrorType, flags: i32, windowtitle: *mut core::ffi::c_void, windowmessage: *mut core::ffi::c_void, result: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAccessDeniedRemediationClient_Impl::Show(this, core::mem::transmute_copy(&parentwnd), core::mem::transmute(&accesspath), core::mem::transmute_copy(&errortype), core::mem::transmute_copy(&flags), core::mem::transmute(&windowtitle), core::mem::transmute(&windowmessage)) {
                    Ok(ok__) => {
                        result.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), Show: Show::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmAccessDeniedRemediationClient as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmAccessDeniedRemediationClient {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmAction, IFsrmAction_Vtbl, 0x6cd6408a_ae60_463b_9ef1_e117534d69dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmAction {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmAction, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmAction {
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn Id(&self) -> windows_core::Result<super::fsrmenums::FSRM_OBJECT_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn ActionType(&self) -> windows_core::Result<super::fsrmenums::FsrmActionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RunLimitInterval(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RunLimitInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRunLimitInterval)(windows_core::Interface::as_raw(self), minutes) }
    }
    pub unsafe fn Delete(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAction_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::fsrmenums::FSRM_OBJECT_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    Id: usize,
    #[cfg(feature = "fsrmenums")]
    pub ActionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::fsrmenums::FsrmActionType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    ActionType: usize,
    pub RunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmAction_Impl: super::oaidl::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<super::fsrmenums::FSRM_OBJECT_ID>;
    fn ActionType(&self) -> windows_core::Result<super::fsrmenums::FsrmActionType>;
    fn RunLimitInterval(&self) -> windows_core::Result<i32>;
    fn SetRunLimitInterval(&self, minutes: i32) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmAction_Vtbl {
    pub const fn new<Identity: IFsrmAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut super::fsrmenums::FSRM_OBJECT_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAction_Impl::Id(this) {
                    Ok(ok__) => {
                        id.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActionType<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actiontype: *mut super::fsrmenums::FsrmActionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAction_Impl::ActionType(this) {
                    Ok(ok__) => {
                        actiontype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RunLimitInterval<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAction_Impl::RunLimitInterval(this) {
                    Ok(ok__) => {
                        minutes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRunLimitInterval<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmAction_Impl::SetRunLimitInterval(this, core::mem::transmute_copy(&minutes)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmAction_Impl::Delete(this).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            ActionType: ActionType::<Identity, OFFSET>,
            RunLimitInterval: RunLimitInterval::<Identity, OFFSET>,
            SetRunLimitInterval: SetRunLimitInterval::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmAction as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmAction {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmActionCommand, IFsrmActionCommand_Vtbl, 0x12937789_e247_4917_9c20_f3ee9c7ee783);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmActionCommand {
    type Target = IFsrmAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmActionCommand, windows_core::IUnknown, super::oaidl::IDispatch, IFsrmAction);
#[cfg(feature = "oaidl")]
impl IFsrmActionCommand {
    pub unsafe fn ExecutablePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecutablePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetExecutablePath(&self, executablepath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExecutablePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(executablepath)) }
    }
    pub unsafe fn Arguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Arguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetArguments(&self, arguments: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(arguments)) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn Account(&self) -> windows_core::Result<super::fsrmenums::FsrmAccountType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Account)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn SetAccount(&self, account: super::fsrmenums::FsrmAccountType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAccount)(windows_core::Interface::as_raw(self), account) }
    }
    pub unsafe fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WorkingDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetWorkingDirectory(&self, workingdirectory: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(workingdirectory)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn MonitorCommand(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MonitorCommand)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetMonitorCommand(&self, monitorcommand: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMonitorCommand)(windows_core::Interface::as_raw(self), monitorcommand) }
    }
    pub unsafe fn KillTimeOut(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KillTimeOut)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKillTimeOut(&self, minutes: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetKillTimeOut)(windows_core::Interface::as_raw(self), minutes) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn LogResult(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetLogResult(&self, logresults: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLogResult)(windows_core::Interface::as_raw(self), logresults) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionCommand_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub ExecutablePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExecutablePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub Account: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::fsrmenums::FsrmAccountType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    Account: usize,
    #[cfg(feature = "fsrmenums")]
    pub SetAccount: unsafe extern "system" fn(*mut core::ffi::c_void, super::fsrmenums::FsrmAccountType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    SetAccount: usize,
    pub WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub MonitorCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    MonitorCommand: usize,
    #[cfg(feature = "wtypes")]
    pub SetMonitorCommand: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetMonitorCommand: usize,
    pub KillTimeOut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetKillTimeOut: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub LogResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    LogResult: usize,
    #[cfg(feature = "wtypes")]
    pub SetLogResult: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetLogResult: usize,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmActionCommand_Impl: IFsrmAction_Impl {
    fn ExecutablePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetExecutablePath(&self, executablepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Arguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetArguments(&self, arguments: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Account(&self) -> windows_core::Result<super::fsrmenums::FsrmAccountType>;
    fn SetAccount(&self, account: super::fsrmenums::FsrmAccountType) -> windows_core::Result<()>;
    fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetWorkingDirectory(&self, workingdirectory: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MonitorCommand(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetMonitorCommand(&self, monitorcommand: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn KillTimeOut(&self) -> windows_core::Result<i32>;
    fn SetKillTimeOut(&self, minutes: i32) -> windows_core::Result<()>;
    fn LogResult(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetLogResult(&self, logresults: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmActionCommand_Vtbl {
    pub const fn new<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExecutablePath<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executablepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::ExecutablePath(this) {
                    Ok(ok__) => {
                        executablepath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExecutablePath<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executablepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetExecutablePath(this, core::mem::transmute(&executablepath)).into()
            }
        }
        unsafe extern "system" fn Arguments<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, arguments: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::Arguments(this) {
                    Ok(ok__) => {
                        arguments.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetArguments<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, arguments: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetArguments(this, core::mem::transmute(&arguments)).into()
            }
        }
        unsafe extern "system" fn Account<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, account: *mut super::fsrmenums::FsrmAccountType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::Account(this) {
                    Ok(ok__) => {
                        account.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccount<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, account: super::fsrmenums::FsrmAccountType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetAccount(this, core::mem::transmute_copy(&account)).into()
            }
        }
        unsafe extern "system" fn WorkingDirectory<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, workingdirectory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::WorkingDirectory(this) {
                    Ok(ok__) => {
                        workingdirectory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, workingdirectory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetWorkingDirectory(this, core::mem::transmute(&workingdirectory)).into()
            }
        }
        unsafe extern "system" fn MonitorCommand<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitorcommand: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::MonitorCommand(this) {
                    Ok(ok__) => {
                        monitorcommand.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMonitorCommand<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitorcommand: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetMonitorCommand(this, core::mem::transmute_copy(&monitorcommand)).into()
            }
        }
        unsafe extern "system" fn KillTimeOut<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::KillTimeOut(this) {
                    Ok(ok__) => {
                        minutes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKillTimeOut<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetKillTimeOut(this, core::mem::transmute_copy(&minutes)).into()
            }
        }
        unsafe extern "system" fn LogResult<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logresults: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::LogResult(this) {
                    Ok(ok__) => {
                        logresults.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogResult<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logresults: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::SetLogResult(this, core::mem::transmute_copy(&logresults)).into()
            }
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, OFFSET>(),
            ExecutablePath: ExecutablePath::<Identity, OFFSET>,
            SetExecutablePath: SetExecutablePath::<Identity, OFFSET>,
            Arguments: Arguments::<Identity, OFFSET>,
            SetArguments: SetArguments::<Identity, OFFSET>,
            Account: Account::<Identity, OFFSET>,
            SetAccount: SetAccount::<Identity, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, OFFSET>,
            MonitorCommand: MonitorCommand::<Identity, OFFSET>,
            SetMonitorCommand: SetMonitorCommand::<Identity, OFFSET>,
            KillTimeOut: KillTimeOut::<Identity, OFFSET>,
            SetKillTimeOut: SetKillTimeOut::<Identity, OFFSET>,
            LogResult: LogResult::<Identity, OFFSET>,
            SetLogResult: SetLogResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionCommand as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmActionCommand {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmActionEmail, IFsrmActionEmail_Vtbl, 0xd646567d_26ae_4caa_9f84_4e0aad207fca);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmActionEmail {
    type Target = IFsrmAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmActionEmail, windows_core::IUnknown, super::oaidl::IDispatch, IFsrmAction);
#[cfg(feature = "oaidl")]
impl IFsrmActionEmail {
    pub unsafe fn MailFrom(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailFrom)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailFrom(&self, mailfrom: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMailFrom)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailfrom)) }
    }
    pub unsafe fn MailReplyTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailReplyTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailReplyTo(&self, mailreplyto: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMailReplyTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailreplyto)) }
    }
    pub unsafe fn MailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMailTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)) }
    }
    pub unsafe fn MailCc(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailCc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailCc(&self, mailcc: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMailCc)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailcc)) }
    }
    pub unsafe fn MailBcc(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailBcc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailBcc(&self, mailbcc: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMailBcc)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailbcc)) }
    }
    pub unsafe fn MailSubject(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailSubject)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailSubject(&self, mailsubject: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMailSubject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailsubject)) }
    }
    pub unsafe fn MessageText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MessageText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMessageText(&self, messagetext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMessageText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(messagetext)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub MailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MailReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MailCc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailCc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MailBcc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailBcc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MailSubject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailSubject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmActionEmail_Impl: IFsrmAction_Impl {
    fn MailFrom(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailFrom(&self, mailfrom: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MailReplyTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailReplyTo(&self, mailreplyto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MailTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MailCc(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailCc(&self, mailcc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MailBcc(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailBcc(&self, mailbcc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MailSubject(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailSubject(&self, mailsubject: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MessageText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMessageText(&self, messagetext: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmActionEmail_Vtbl {
    pub const fn new<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MailFrom<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailfrom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MailFrom(this) {
                    Ok(ok__) => {
                        mailfrom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailFrom<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailfrom: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMailFrom(this, core::mem::transmute(&mailfrom)).into()
            }
        }
        unsafe extern "system" fn MailReplyTo<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailreplyto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MailReplyTo(this) {
                    Ok(ok__) => {
                        mailreplyto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailReplyTo<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailreplyto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMailReplyTo(this, core::mem::transmute(&mailreplyto)).into()
            }
        }
        unsafe extern "system" fn MailTo<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MailTo(this) {
                    Ok(ok__) => {
                        mailto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailTo<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMailTo(this, core::mem::transmute(&mailto)).into()
            }
        }
        unsafe extern "system" fn MailCc<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailcc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MailCc(this) {
                    Ok(ok__) => {
                        mailcc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailCc<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailcc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMailCc(this, core::mem::transmute(&mailcc)).into()
            }
        }
        unsafe extern "system" fn MailBcc<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailbcc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MailBcc(this) {
                    Ok(ok__) => {
                        mailbcc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailBcc<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailbcc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMailBcc(this, core::mem::transmute(&mailbcc)).into()
            }
        }
        unsafe extern "system" fn MailSubject<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailsubject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MailSubject(this) {
                    Ok(ok__) => {
                        mailsubject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailSubject<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailsubject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMailSubject(this, core::mem::transmute(&mailsubject)).into()
            }
        }
        unsafe extern "system" fn MessageText<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::MessageText(this) {
                    Ok(ok__) => {
                        messagetext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMessageText<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::SetMessageText(this, core::mem::transmute(&messagetext)).into()
            }
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, OFFSET>(),
            MailFrom: MailFrom::<Identity, OFFSET>,
            SetMailFrom: SetMailFrom::<Identity, OFFSET>,
            MailReplyTo: MailReplyTo::<Identity, OFFSET>,
            SetMailReplyTo: SetMailReplyTo::<Identity, OFFSET>,
            MailTo: MailTo::<Identity, OFFSET>,
            SetMailTo: SetMailTo::<Identity, OFFSET>,
            MailCc: MailCc::<Identity, OFFSET>,
            SetMailCc: SetMailCc::<Identity, OFFSET>,
            MailBcc: MailBcc::<Identity, OFFSET>,
            SetMailBcc: SetMailBcc::<Identity, OFFSET>,
            MailSubject: MailSubject::<Identity, OFFSET>,
            SetMailSubject: SetMailSubject::<Identity, OFFSET>,
            MessageText: MessageText::<Identity, OFFSET>,
            SetMessageText: SetMessageText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionEmail as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmActionEmail {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmActionEmail2, IFsrmActionEmail2_Vtbl, 0x8276702f_2532_4839_89bf_4872609a2ea4);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmActionEmail2 {
    type Target = IFsrmActionEmail;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmActionEmail2, windows_core::IUnknown, super::oaidl::IDispatch, IFsrmAction, IFsrmActionEmail);
#[cfg(feature = "oaidl")]
impl IFsrmActionEmail2 {
    pub unsafe fn AttachmentFileListSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AttachmentFileListSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAttachmentFileListSize(&self, attachmentfilelistsize: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAttachmentFileListSize)(windows_core::Interface::as_raw(self), attachmentfilelistsize) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail2_Vtbl {
    pub base__: IFsrmActionEmail_Vtbl,
    pub AttachmentFileListSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAttachmentFileListSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmActionEmail2_Impl: IFsrmActionEmail_Impl {
    fn AttachmentFileListSize(&self) -> windows_core::Result<i32>;
    fn SetAttachmentFileListSize(&self, attachmentfilelistsize: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmActionEmail2_Vtbl {
    pub const fn new<Identity: IFsrmActionEmail2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AttachmentFileListSize<Identity: IFsrmActionEmail2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachmentfilelistsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail2_Impl::AttachmentFileListSize(this) {
                    Ok(ok__) => {
                        attachmentfilelistsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttachmentFileListSize<Identity: IFsrmActionEmail2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachmentfilelistsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail2_Impl::SetAttachmentFileListSize(this, core::mem::transmute_copy(&attachmentfilelistsize)).into()
            }
        }
        Self {
            base__: IFsrmActionEmail_Vtbl::new::<Identity, OFFSET>(),
            AttachmentFileListSize: AttachmentFileListSize::<Identity, OFFSET>,
            SetAttachmentFileListSize: SetAttachmentFileListSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionEmail2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID || iid == &<IFsrmActionEmail as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmActionEmail2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmActionEventLog, IFsrmActionEventLog_Vtbl, 0x4c8f96c3_5d94_4f37_a4f4_f56ab463546f);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmActionEventLog {
    type Target = IFsrmAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmActionEventLog, windows_core::IUnknown, super::oaidl::IDispatch, IFsrmAction);
#[cfg(feature = "oaidl")]
impl IFsrmActionEventLog {
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn EventType(&self) -> windows_core::Result<super::fsrmenums::FsrmEventType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn SetEventType(&self, eventtype: super::fsrmenums::FsrmEventType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEventType)(windows_core::Interface::as_raw(self), eventtype) }
    }
    pub unsafe fn MessageText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MessageText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMessageText(&self, messagetext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMessageText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(messagetext)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEventLog_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::fsrmenums::FsrmEventType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    EventType: usize,
    #[cfg(feature = "fsrmenums")]
    pub SetEventType: unsafe extern "system" fn(*mut core::ffi::c_void, super::fsrmenums::FsrmEventType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    SetEventType: usize,
    pub MessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmActionEventLog_Impl: IFsrmAction_Impl {
    fn EventType(&self) -> windows_core::Result<super::fsrmenums::FsrmEventType>;
    fn SetEventType(&self, eventtype: super::fsrmenums::FsrmEventType) -> windows_core::Result<()>;
    fn MessageText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMessageText(&self, messagetext: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmActionEventLog_Vtbl {
    pub const fn new<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventType<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventtype: *mut super::fsrmenums::FsrmEventType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEventLog_Impl::EventType(this) {
                    Ok(ok__) => {
                        eventtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventType<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventtype: super::fsrmenums::FsrmEventType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEventLog_Impl::SetEventType(this, core::mem::transmute_copy(&eventtype)).into()
            }
        }
        unsafe extern "system" fn MessageText<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEventLog_Impl::MessageText(this) {
                    Ok(ok__) => {
                        messagetext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMessageText<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEventLog_Impl::SetMessageText(this, core::mem::transmute(&messagetext)).into()
            }
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, OFFSET>(),
            EventType: EventType::<Identity, OFFSET>,
            SetEventType: SetEventType::<Identity, OFFSET>,
            MessageText: MessageText::<Identity, OFFSET>,
            SetMessageText: SetMessageText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionEventLog as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmActionEventLog {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmActionReport, IFsrmActionReport_Vtbl, 0x2dbe63c4_b340_48a0_a5b0_158e07fc567e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmActionReport {
    type Target = IFsrmAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmActionReport, windows_core::IUnknown, super::oaidl::IDispatch, IFsrmAction);
#[cfg(feature = "oaidl")]
impl IFsrmActionReport {
    pub unsafe fn ReportTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReportTypes(&self, reporttypes: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReportTypes)(windows_core::Interface::as_raw(self), reporttypes) }
    }
    pub unsafe fn MailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMailTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionReport_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub ReportTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetReportTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmActionReport_Impl: IFsrmAction_Impl {
    fn ReportTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetReportTypes(&self, reporttypes: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn MailTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmActionReport_Vtbl {
    pub const fn new<Identity: IFsrmActionReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportTypes<Identity: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttypes: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionReport_Impl::ReportTypes(this) {
                    Ok(ok__) => {
                        reporttypes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReportTypes<Identity: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttypes: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionReport_Impl::SetReportTypes(this, core::mem::transmute_copy(&reporttypes)).into()
            }
        }
        unsafe extern "system" fn MailTo<Identity: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionReport_Impl::MailTo(this) {
                    Ok(ok__) => {
                        mailto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailTo<Identity: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionReport_Impl::SetMailTo(this, core::mem::transmute(&mailto)).into()
            }
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, OFFSET>(),
            ReportTypes: ReportTypes::<Identity, OFFSET>,
            SetReportTypes: SetReportTypes::<Identity, OFFSET>,
            MailTo: MailTo::<Identity, OFFSET>,
            SetMailTo: SetMailTo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionReport as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmActionReport {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmCollection, IFsrmCollection_Vtbl, 0xf76fbf3b_8ddd_4b42_b05a_cb1c3ff1fee8);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn State(&self) -> windows_core::Result<super::fsrmenums::FsrmCollectionState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitForCompletion)(windows_core::Interface::as_raw(self), waitseconds, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "fsrmenums", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetById(&self, id: super::fsrmenums::FSRM_OBJECT_ID) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetById)(windows_core::Interface::as_raw(self), core::mem::transmute(id), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::fsrmenums::FsrmCollectionState) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    State: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub WaitForCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    WaitForCompletion: usize,
    #[cfg(all(feature = "fsrmenums", feature = "wtypes", feature = "wtypesbase"))]
    pub GetById: unsafe extern "system" fn(*mut core::ffi::c_void, super::fsrmenums::FSRM_OBJECT_ID, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrmenums", feature = "wtypes", feature = "wtypesbase")))]
    GetById: usize,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmCollection_Impl: super::oaidl::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn State(&self) -> windows_core::Result<super::fsrmenums::FsrmCollectionState>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn GetById(&self, id: &super::fsrmenums::FSRM_OBJECT_ID) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmCollection_Vtbl {
    pub const fn new<Identity: IFsrmCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        unknown.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, item: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut super::fsrmenums::FsrmCollectionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::State(this) {
                    Ok(ok__) => {
                        state.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmCollection_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn WaitForCompletion<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, waitseconds: i32, completed: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::WaitForCompletion(this, core::mem::transmute_copy(&waitseconds)) {
                    Ok(ok__) => {
                        completed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetById<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::fsrmenums::FSRM_OBJECT_ID, entry: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::GetById(this, core::mem::transmute(&id)) {
                    Ok(ok__) => {
                        entry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, OFFSET>,
            GetById: GetById::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmCommittableCollection, IFsrmCommittableCollection_Vtbl, 0x96deb3b5_8b91_4a2a_9d93_80a35d8aa847);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmCommittableCollection {
    type Target = IFsrmMutableCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmCommittableCollection, windows_core::IUnknown, super::oaidl::IDispatch, IFsrmCollection, IFsrmMutableCollection);
#[cfg(feature = "oaidl")]
impl IFsrmCommittableCollection {
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn Commit(&self, options: super::fsrmenums::FsrmCommitOptions) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCommittableCollection_Vtbl {
    pub base__: IFsrmMutableCollection_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, super::fsrmenums::FsrmCommitOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    Commit: usize,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmCommittableCollection_Impl: IFsrmMutableCollection_Impl {
    fn Commit(&self, options: super::fsrmenums::FsrmCommitOptions) -> windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmCommittableCollection_Vtbl {
    pub const fn new<Identity: IFsrmCommittableCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: IFsrmCommittableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: super::fsrmenums::FsrmCommitOptions, results: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCommittableCollection_Impl::Commit(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        results.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IFsrmMutableCollection_Vtbl::new::<Identity, OFFSET>(), Commit: Commit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmCommittableCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmCollection as windows_core::Interface>::IID || iid == &<IFsrmMutableCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmCommittableCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmDerivedObjectsResult, IFsrmDerivedObjectsResult_Vtbl, 0x39322a2d_38ee_4d0d_8095_421a80849a82);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmDerivedObjectsResult {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmDerivedObjectsResult, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmDerivedObjectsResult {
    pub unsafe fn DerivedObjects(&self) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DerivedObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Results(&self) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Results)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmDerivedObjectsResult_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub DerivedObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Results: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmDerivedObjectsResult_Impl: super::oaidl::IDispatch_Impl {
    fn DerivedObjects(&self) -> windows_core::Result<IFsrmCollection>;
    fn Results(&self) -> windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmDerivedObjectsResult_Vtbl {
    pub const fn new<Identity: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DerivedObjects<Identity: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, derivedobjects: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmDerivedObjectsResult_Impl::DerivedObjects(this) {
                    Ok(ok__) => {
                        derivedobjects.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Results<Identity: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, results: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmDerivedObjectsResult_Impl::Results(this) {
                    Ok(ok__) => {
                        results.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DerivedObjects: DerivedObjects::<Identity, OFFSET>,
            Results: Results::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmDerivedObjectsResult as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmDerivedObjectsResult {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmExportImport, IFsrmExportImport_Vtbl, 0xefcb0ab1_16c4_4a79_812c_725614c3306b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmExportImport {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmExportImport, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmExportImport {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ExportFileGroups(&self, filepath: &windows_core::BSTR, filegroupnamessafearray: *const super::oaidl::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExportFileGroups)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute(filegroupnamessafearray), core::mem::transmute_copy(remotehost)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ImportFileGroups(&self, filepath: &windows_core::BSTR, filegroupnamessafearray: *const super::oaidl::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportFileGroups)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute(filegroupnamessafearray), core::mem::transmute_copy(remotehost), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ExportFileScreenTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::oaidl::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExportFileScreenTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute(templatenamessafearray), core::mem::transmute_copy(remotehost)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ImportFileScreenTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::oaidl::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportFileScreenTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute(templatenamessafearray), core::mem::transmute_copy(remotehost), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ExportQuotaTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::oaidl::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExportQuotaTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute(templatenamessafearray), core::mem::transmute_copy(remotehost)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ImportQuotaTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::oaidl::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportQuotaTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute(templatenamessafearray), core::mem::transmute_copy(remotehost), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmExportImport_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ExportFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ExportFileGroups: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ImportFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ImportFileGroups: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ExportFileScreenTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ExportFileScreenTemplates: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ImportFileScreenTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ImportFileScreenTemplates: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ExportQuotaTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ExportQuotaTemplates: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ImportQuotaTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ImportQuotaTemplates: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmExportImport_Impl: super::oaidl::IDispatch_Impl {
    fn ExportFileGroups(&self, filepath: &windows_core::BSTR, filegroupnamessafearray: *const super::oaidl::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportFileGroups(&self, filepath: &windows_core::BSTR, filegroupnamessafearray: *const super::oaidl::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection>;
    fn ExportFileScreenTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::oaidl::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportFileScreenTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::oaidl::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection>;
    fn ExportQuotaTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::oaidl::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportQuotaTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::oaidl::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmExportImport_Vtbl {
    pub const fn new<Identity: IFsrmExportImport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExportFileGroups<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, filegroupnamessafearray: *const super::oaidl::VARIANT, remotehost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmExportImport_Impl::ExportFileGroups(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&filegroupnamessafearray), core::mem::transmute(&remotehost)).into()
            }
        }
        unsafe extern "system" fn ImportFileGroups<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, filegroupnamessafearray: *const super::oaidl::VARIANT, remotehost: *mut core::ffi::c_void, filegroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmExportImport_Impl::ImportFileGroups(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&filegroupnamessafearray), core::mem::transmute(&remotehost)) {
                    Ok(ok__) => {
                        filegroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExportFileScreenTemplates<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, templatenamessafearray: *const super::oaidl::VARIANT, remotehost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmExportImport_Impl::ExportFileScreenTemplates(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&templatenamessafearray), core::mem::transmute(&remotehost)).into()
            }
        }
        unsafe extern "system" fn ImportFileScreenTemplates<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, templatenamessafearray: *const super::oaidl::VARIANT, remotehost: *mut core::ffi::c_void, templates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmExportImport_Impl::ImportFileScreenTemplates(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&templatenamessafearray), core::mem::transmute(&remotehost)) {
                    Ok(ok__) => {
                        templates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExportQuotaTemplates<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, templatenamessafearray: *const super::oaidl::VARIANT, remotehost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmExportImport_Impl::ExportQuotaTemplates(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&templatenamessafearray), core::mem::transmute(&remotehost)).into()
            }
        }
        unsafe extern "system" fn ImportQuotaTemplates<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, templatenamessafearray: *const super::oaidl::VARIANT, remotehost: *mut core::ffi::c_void, templates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmExportImport_Impl::ImportQuotaTemplates(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&templatenamessafearray), core::mem::transmute(&remotehost)) {
                    Ok(ok__) => {
                        templates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ExportFileGroups: ExportFileGroups::<Identity, OFFSET>,
            ImportFileGroups: ImportFileGroups::<Identity, OFFSET>,
            ExportFileScreenTemplates: ExportFileScreenTemplates::<Identity, OFFSET>,
            ImportFileScreenTemplates: ImportFileScreenTemplates::<Identity, OFFSET>,
            ExportQuotaTemplates: ExportQuotaTemplates::<Identity, OFFSET>,
            ImportQuotaTemplates: ImportQuotaTemplates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmExportImport as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmExportImport {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmMutableCollection, IFsrmMutableCollection_Vtbl, 0x1bb617b8_3886_49dc_af82_a6c90fa35dda);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmMutableCollection {
    type Target = IFsrmCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmMutableCollection, windows_core::IUnknown, super::oaidl::IDispatch, IFsrmCollection);
#[cfg(feature = "oaidl")]
impl IFsrmMutableCollection {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Add(&self, item: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(item)) }
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn RemoveById(&self, id: super::fsrmenums::FSRM_OBJECT_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveById)(windows_core::Interface::as_raw(self), core::mem::transmute(id)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmMutableCollection_Vtbl {
    pub base__: IFsrmCollection_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub RemoveById: unsafe extern "system" fn(*mut core::ffi::c_void, super::fsrmenums::FSRM_OBJECT_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    RemoveById: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmMutableCollection_Impl: IFsrmCollection_Impl {
    fn Add(&self, item: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn RemoveById(&self, id: &super::fsrmenums::FSRM_OBJECT_ID) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IFsrmMutableCollection>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmMutableCollection_Vtbl {
    pub const fn new<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmMutableCollection_Impl::Add(this, core::mem::transmute(&item)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmMutableCollection_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn RemoveById<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::fsrmenums::FSRM_OBJECT_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmMutableCollection_Impl::RemoveById(this, core::mem::transmute(&id)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmMutableCollection_Impl::Clone(this) {
                    Ok(ok__) => {
                        collection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmCollection_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveById: RemoveById::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmMutableCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmMutableCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmObject, IFsrmObject_Vtbl, 0x22bcef93_4a3f_4183_89f9_2f8b8a628aee);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmObject {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmObject, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmObject {
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn Id(&self) -> windows_core::Result<super::fsrmenums::FSRM_OBJECT_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)) }
    }
    pub unsafe fn Delete(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Commit(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmObject_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::fsrmenums::FSRM_OBJECT_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    Id: usize,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmObject_Impl: super::oaidl::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<super::fsrmenums::FSRM_OBJECT_ID>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmObject_Vtbl {
    pub const fn new<Identity: IFsrmObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut super::fsrmenums::FSRM_OBJECT_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmObject_Impl::Id(this) {
                    Ok(ok__) => {
                        id.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmObject_Impl::Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmObject_Impl::SetDescription(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmObject_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmObject_Impl::Commit(this).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmObject {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmPathMapper, IFsrmPathMapper_Vtbl, 0x6f4dbfff_6920_4821_a6c3_b7e94c1fd60c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmPathMapper {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmPathMapper, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmPathMapper {
    pub unsafe fn GetSharePathsForLocalPath(&self, localpath: &windows_core::BSTR) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSharePathsForLocalPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(localpath), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPathMapper_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub GetSharePathsForLocalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPathMapper_Impl: super::oaidl::IDispatch_Impl {
    fn GetSharePathsForLocalPath(&self, localpath: &windows_core::BSTR) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPathMapper_Vtbl {
    pub const fn new<Identity: IFsrmPathMapper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSharePathsForLocalPath<Identity: IFsrmPathMapper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpath: *mut core::ffi::c_void, sharepaths: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPathMapper_Impl::GetSharePathsForLocalPath(this, core::mem::transmute(&localpath)) {
                    Ok(ok__) => {
                        sharepaths.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetSharePathsForLocalPath: GetSharePathsForLocalPath::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPathMapper as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmPathMapper {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmSetting, IFsrmSetting_Vtbl, 0xf411d4fd_14be_4260_8c40_03b7c95e608a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmSetting {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmSetting, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmSetting {
    pub unsafe fn SmtpServer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SmtpServer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSmtpServer(&self, smtpserver: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSmtpServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(smtpserver)) }
    }
    pub unsafe fn MailFrom(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MailFrom)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetMailFrom(&self, mailfrom: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMailFrom)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailfrom)) }
    }
    pub unsafe fn AdminEmail(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdminEmail)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetAdminEmail(&self, adminemail: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAdminEmail)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(adminemail)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn DisableCommandLine(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisableCommandLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetDisableCommandLine(&self, disablecommandline: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisableCommandLine)(windows_core::Interface::as_raw(self), disablecommandline) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn EnableScreeningAudit(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnableScreeningAudit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetEnableScreeningAudit(&self, enablescreeningaudit: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnableScreeningAudit)(windows_core::Interface::as_raw(self), enablescreeningaudit) }
    }
    pub unsafe fn EmailTest(&self, mailto: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EmailTest)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn SetActionRunLimitInterval(&self, actiontype: super::fsrmenums::FsrmActionType, delaytimeminutes: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetActionRunLimitInterval)(windows_core::Interface::as_raw(self), actiontype, delaytimeminutes) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn GetActionRunLimitInterval(&self, actiontype: super::fsrmenums::FsrmActionType) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActionRunLimitInterval)(windows_core::Interface::as_raw(self), actiontype, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmSetting_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub SmtpServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSmtpServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AdminEmail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAdminEmail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub DisableCommandLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    DisableCommandLine: usize,
    #[cfg(feature = "wtypes")]
    pub SetDisableCommandLine: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetDisableCommandLine: usize,
    #[cfg(feature = "wtypes")]
    pub EnableScreeningAudit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    EnableScreeningAudit: usize,
    #[cfg(feature = "wtypes")]
    pub SetEnableScreeningAudit: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetEnableScreeningAudit: usize,
    pub EmailTest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub SetActionRunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::fsrmenums::FsrmActionType, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    SetActionRunLimitInterval: usize,
    #[cfg(feature = "fsrmenums")]
    pub GetActionRunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::fsrmenums::FsrmActionType, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    GetActionRunLimitInterval: usize,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmSetting_Impl: super::oaidl::IDispatch_Impl {
    fn SmtpServer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSmtpServer(&self, smtpserver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MailFrom(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMailFrom(&self, mailfrom: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AdminEmail(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAdminEmail(&self, adminemail: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisableCommandLine(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetDisableCommandLine(&self, disablecommandline: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EnableScreeningAudit(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetEnableScreeningAudit(&self, enablescreeningaudit: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EmailTest(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetActionRunLimitInterval(&self, actiontype: super::fsrmenums::FsrmActionType, delaytimeminutes: i32) -> windows_core::Result<()>;
    fn GetActionRunLimitInterval(&self, actiontype: super::fsrmenums::FsrmActionType) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmSetting_Vtbl {
    pub const fn new<Identity: IFsrmSetting_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SmtpServer<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smtpserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::SmtpServer(this) {
                    Ok(ok__) => {
                        smtpserver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSmtpServer<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smtpserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetSmtpServer(this, core::mem::transmute(&smtpserver)).into()
            }
        }
        unsafe extern "system" fn MailFrom<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailfrom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::MailFrom(this) {
                    Ok(ok__) => {
                        mailfrom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMailFrom<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailfrom: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetMailFrom(this, core::mem::transmute(&mailfrom)).into()
            }
        }
        unsafe extern "system" fn AdminEmail<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adminemail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::AdminEmail(this) {
                    Ok(ok__) => {
                        adminemail.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAdminEmail<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adminemail: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetAdminEmail(this, core::mem::transmute(&adminemail)).into()
            }
        }
        unsafe extern "system" fn DisableCommandLine<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disablecommandline: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::DisableCommandLine(this) {
                    Ok(ok__) => {
                        disablecommandline.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisableCommandLine<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disablecommandline: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetDisableCommandLine(this, core::mem::transmute_copy(&disablecommandline)).into()
            }
        }
        unsafe extern "system" fn EnableScreeningAudit<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enablescreeningaudit: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::EnableScreeningAudit(this) {
                    Ok(ok__) => {
                        enablescreeningaudit.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableScreeningAudit<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enablescreeningaudit: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetEnableScreeningAudit(this, core::mem::transmute_copy(&enablescreeningaudit)).into()
            }
        }
        unsafe extern "system" fn EmailTest<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::EmailTest(this, core::mem::transmute(&mailto)).into()
            }
        }
        unsafe extern "system" fn SetActionRunLimitInterval<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actiontype: super::fsrmenums::FsrmActionType, delaytimeminutes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetActionRunLimitInterval(this, core::mem::transmute_copy(&actiontype), core::mem::transmute_copy(&delaytimeminutes)).into()
            }
        }
        unsafe extern "system" fn GetActionRunLimitInterval<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actiontype: super::fsrmenums::FsrmActionType, delaytimeminutes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::GetActionRunLimitInterval(this, core::mem::transmute_copy(&actiontype)) {
                    Ok(ok__) => {
                        delaytimeminutes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SmtpServer: SmtpServer::<Identity, OFFSET>,
            SetSmtpServer: SetSmtpServer::<Identity, OFFSET>,
            MailFrom: MailFrom::<Identity, OFFSET>,
            SetMailFrom: SetMailFrom::<Identity, OFFSET>,
            AdminEmail: AdminEmail::<Identity, OFFSET>,
            SetAdminEmail: SetAdminEmail::<Identity, OFFSET>,
            DisableCommandLine: DisableCommandLine::<Identity, OFFSET>,
            SetDisableCommandLine: SetDisableCommandLine::<Identity, OFFSET>,
            EnableScreeningAudit: EnableScreeningAudit::<Identity, OFFSET>,
            SetEnableScreeningAudit: SetEnableScreeningAudit::<Identity, OFFSET>,
            EmailTest: EmailTest::<Identity, OFFSET>,
            SetActionRunLimitInterval: SetActionRunLimitInterval::<Identity, OFFSET>,
            GetActionRunLimitInterval: GetActionRunLimitInterval::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmSetting as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmSetting {}

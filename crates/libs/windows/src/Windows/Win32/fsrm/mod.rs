pub const FSRM_DISPID_ACTION: i32 = 19922944;
pub const FSRM_DISPID_ACTION_COMMAND: i32 = 20185088;
pub const FSRM_DISPID_ACTION_EMAIL: i32 = 19988480;
pub const FSRM_DISPID_ACTION_EMAIL2: i32 = 20250624;
pub const FSRM_DISPID_ACTION_EVENTLOG: i32 = 20119552;
pub const FSRM_DISPID_ACTION_REPORT: i32 = 20054016;
pub const FSRM_DISPID_ADR: i32 = 25165824;
pub const FSRM_DISPID_COLLECTION: i32 = 18874368;
pub const FSRM_DISPID_COLLECTION_COMMITTABLE: i32 = 18944000;
pub const FSRM_DISPID_COLLECTION_MUTABLE: i32 = 18939904;
pub const FSRM_DISPID_DERIVEDOBJECTSRESULT: i32 = 24117248;
pub const FSRM_DISPID_EXPORTIMPORT: i32 = 23068672;
pub const FSRM_DISPID_FEATURE_CLASSIFICATION: i32 = 83886080;
pub const FSRM_DISPID_FEATURE_FILESCREEN: i32 = 50331648;
pub const FSRM_DISPID_FEATURE_GENERAL: i32 = 16777216;
pub const FSRM_DISPID_FEATURE_MASK: i32 = 251658240;
pub const FSRM_DISPID_FEATURE_PIPELINE: i32 = 100663296;
pub const FSRM_DISPID_FEATURE_QUOTA: i32 = 33554432;
pub const FSRM_DISPID_FEATURE_REPORTS: i32 = 67108864;
pub const FSRM_DISPID_INTERFACE_A_MASK: i32 = 15728640;
pub const FSRM_DISPID_INTERFACE_B_MASK: i32 = 983040;
pub const FSRM_DISPID_INTERFACE_C_MASK: i32 = 61440;
pub const FSRM_DISPID_INTERFACE_D_MASK: i32 = 3840;
pub const FSRM_DISPID_INTERFACE_MASK: i32 = 16776960;
pub const FSRM_DISPID_IS_PROPERTY: i32 = 128;
pub const FSRM_DISPID_METHOD_MASK: i32 = 255;
pub const FSRM_DISPID_METHOD_NUM_MASK: i32 = 127;
pub const FSRM_DISPID_OBJECT: i32 = 17825792;
pub const FSRM_DISPID_PATHMAPPER: i32 = 22020096;
pub const FSRM_DISPID_SETTING: i32 = 20971520;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmAccessDeniedRemediationClient, IFsrmAccessDeniedRemediationClient_Vtbl, 0x40002314_590b_45a5_8e1b_8c05da527e52);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmAccessDeniedRemediationClient {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmAccessDeniedRemediationClient, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmAccessDeniedRemediationClient {
    #[cfg(all(feature = "fsrmenums", feature = "shared"))]
    pub unsafe fn Show(&self, parentwnd: super::ULONG_PTR, accesspath: &windows_core::BSTR, errortype: super::AdrClientErrorType, flags: i32, windowtitle: &windows_core::BSTR, windowmessage: &windows_core::BSTR) -> windows_core::Result<i32> {
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
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "fsrmenums", feature = "shared"))]
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG_PTR, *mut core::ffi::c_void, super::AdrClientErrorType, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrmenums", feature = "shared")))]
    Show: usize,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmAccessDeniedRemediationClient_Impl: super::IDispatch_Impl {
    fn Show(&self, parentwnd: super::ULONG_PTR, accesspath: &windows_core::BSTR, errortype: super::AdrClientErrorType, flags: i32, windowtitle: &windows_core::BSTR, windowmessage: &windows_core::BSTR) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmAccessDeniedRemediationClient_Vtbl {
    pub const fn new<Identity: IFsrmAccessDeniedRemediationClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Show<Identity: IFsrmAccessDeniedRemediationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentwnd: super::ULONG_PTR, accesspath: *mut core::ffi::c_void, errortype: super::AdrClientErrorType, flags: i32, windowtitle: *mut core::ffi::c_void, windowmessage: *mut core::ffi::c_void, result: *mut i32) -> windows_core::HRESULT {
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
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), Show: Show::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmAccessDeniedRemediationClient as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmAccessDeniedRemediationClient {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmAction, IFsrmAction_Vtbl, 0x6cd6408a_ae60_463b_9ef1_e117534d69dc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmAction {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmAction, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmAction {
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_Id(&self) -> windows_core::Result<super::FSRM_OBJECT_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_ActionType(&self) -> windows_core::Result<super::FsrmActionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ActionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_RunLimitInterval(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RunLimitInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_RunLimitInterval(&self, minutes: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RunLimitInterval)(windows_core::Interface::as_raw(self), minutes) }
    }
    pub unsafe fn Delete(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAction_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub get_Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FSRM_OBJECT_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_Id: usize,
    #[cfg(feature = "fsrmenums")]
    pub get_ActionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmActionType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_ActionType: usize,
    pub get_RunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_RunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmAction_Impl: super::IDispatch_Impl {
    fn get_Id(&self) -> windows_core::Result<super::FSRM_OBJECT_ID>;
    fn get_ActionType(&self) -> windows_core::Result<super::FsrmActionType>;
    fn get_RunLimitInterval(&self) -> windows_core::Result<i32>;
    fn put_RunLimitInterval(&self, minutes: i32) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmAction_Vtbl {
    pub const fn new<Identity: IFsrmAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Id<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut super::FSRM_OBJECT_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAction_Impl::get_Id(this) {
                    Ok(ok__) => {
                        id.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ActionType<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actiontype: *mut super::FsrmActionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAction_Impl::get_ActionType(this) {
                    Ok(ok__) => {
                        actiontype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RunLimitInterval<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmAction_Impl::get_RunLimitInterval(this) {
                    Ok(ok__) => {
                        minutes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RunLimitInterval<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmAction_Impl::put_RunLimitInterval(this, core::mem::transmute_copy(&minutes)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IFsrmAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmAction_Impl::Delete(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Id: get_Id::<Identity, OFFSET>,
            get_ActionType: get_ActionType::<Identity, OFFSET>,
            get_RunLimitInterval: get_RunLimitInterval::<Identity, OFFSET>,
            put_RunLimitInterval: put_RunLimitInterval::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmAction as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IFsrmActionCommand, windows_core::IUnknown, super::IDispatch, IFsrmAction);
#[cfg(feature = "oaidl")]
impl IFsrmActionCommand {
    pub unsafe fn get_ExecutablePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExecutablePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ExecutablePath(&self, executablepath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ExecutablePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(executablepath)) }
    }
    pub unsafe fn get_Arguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Arguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Arguments(&self, arguments: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Arguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(arguments)) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_Account(&self) -> windows_core::Result<super::FsrmAccountType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Account)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn put_Account(&self, account: super::FsrmAccountType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Account)(windows_core::Interface::as_raw(self), account) }
    }
    pub unsafe fn get_WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WorkingDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_WorkingDirectory(&self, workingdirectory: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_WorkingDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(workingdirectory)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_MonitorCommand(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MonitorCommand)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_MonitorCommand(&self, monitorcommand: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MonitorCommand)(windows_core::Interface::as_raw(self), monitorcommand) }
    }
    pub unsafe fn get_KillTimeOut(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KillTimeOut)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_KillTimeOut(&self, minutes: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_KillTimeOut)(windows_core::Interface::as_raw(self), minutes) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LogResult(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LogResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_LogResult(&self, logresults: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LogResult)(windows_core::Interface::as_raw(self), logresults) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionCommand_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub get_ExecutablePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ExecutablePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub get_Account: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmAccountType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_Account: usize,
    #[cfg(feature = "fsrmenums")]
    pub put_Account: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmAccountType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    put_Account: usize,
    pub get_WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_MonitorCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_MonitorCommand: usize,
    #[cfg(feature = "wtypes")]
    pub put_MonitorCommand: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_MonitorCommand: usize,
    pub get_KillTimeOut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_KillTimeOut: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_LogResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LogResult: usize,
    #[cfg(feature = "wtypes")]
    pub put_LogResult: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_LogResult: usize,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmActionCommand_Impl: IFsrmAction_Impl {
    fn get_ExecutablePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ExecutablePath(&self, executablepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Arguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Arguments(&self, arguments: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Account(&self) -> windows_core::Result<super::FsrmAccountType>;
    fn put_Account(&self, account: super::FsrmAccountType) -> windows_core::Result<()>;
    fn get_WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_WorkingDirectory(&self, workingdirectory: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MonitorCommand(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_MonitorCommand(&self, monitorcommand: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_KillTimeOut(&self) -> windows_core::Result<i32>;
    fn put_KillTimeOut(&self, minutes: i32) -> windows_core::Result<()>;
    fn get_LogResult(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_LogResult(&self, logresults: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmActionCommand_Vtbl {
    pub const fn new<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ExecutablePath<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executablepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::get_ExecutablePath(this) {
                    Ok(ok__) => {
                        executablepath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExecutablePath<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executablepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::put_ExecutablePath(this, core::mem::transmute(&executablepath)).into()
            }
        }
        unsafe extern "system" fn get_Arguments<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, arguments: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::get_Arguments(this) {
                    Ok(ok__) => {
                        arguments.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Arguments<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, arguments: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::put_Arguments(this, core::mem::transmute(&arguments)).into()
            }
        }
        unsafe extern "system" fn get_Account<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, account: *mut super::FsrmAccountType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::get_Account(this) {
                    Ok(ok__) => {
                        account.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Account<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, account: super::FsrmAccountType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::put_Account(this, core::mem::transmute_copy(&account)).into()
            }
        }
        unsafe extern "system" fn get_WorkingDirectory<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, workingdirectory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::get_WorkingDirectory(this) {
                    Ok(ok__) => {
                        workingdirectory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_WorkingDirectory<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, workingdirectory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::put_WorkingDirectory(this, core::mem::transmute(&workingdirectory)).into()
            }
        }
        unsafe extern "system" fn get_MonitorCommand<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitorcommand: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::get_MonitorCommand(this) {
                    Ok(ok__) => {
                        monitorcommand.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MonitorCommand<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitorcommand: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::put_MonitorCommand(this, core::mem::transmute_copy(&monitorcommand)).into()
            }
        }
        unsafe extern "system" fn get_KillTimeOut<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::get_KillTimeOut(this) {
                    Ok(ok__) => {
                        minutes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_KillTimeOut<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::put_KillTimeOut(this, core::mem::transmute_copy(&minutes)).into()
            }
        }
        unsafe extern "system" fn get_LogResult<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logresults: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionCommand_Impl::get_LogResult(this) {
                    Ok(ok__) => {
                        logresults.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LogResult<Identity: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logresults: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionCommand_Impl::put_LogResult(this, core::mem::transmute_copy(&logresults)).into()
            }
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, OFFSET>(),
            get_ExecutablePath: get_ExecutablePath::<Identity, OFFSET>,
            put_ExecutablePath: put_ExecutablePath::<Identity, OFFSET>,
            get_Arguments: get_Arguments::<Identity, OFFSET>,
            put_Arguments: put_Arguments::<Identity, OFFSET>,
            get_Account: get_Account::<Identity, OFFSET>,
            put_Account: put_Account::<Identity, OFFSET>,
            get_WorkingDirectory: get_WorkingDirectory::<Identity, OFFSET>,
            put_WorkingDirectory: put_WorkingDirectory::<Identity, OFFSET>,
            get_MonitorCommand: get_MonitorCommand::<Identity, OFFSET>,
            put_MonitorCommand: put_MonitorCommand::<Identity, OFFSET>,
            get_KillTimeOut: get_KillTimeOut::<Identity, OFFSET>,
            put_KillTimeOut: put_KillTimeOut::<Identity, OFFSET>,
            get_LogResult: get_LogResult::<Identity, OFFSET>,
            put_LogResult: put_LogResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionCommand as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IFsrmActionEmail, windows_core::IUnknown, super::IDispatch, IFsrmAction);
#[cfg(feature = "oaidl")]
impl IFsrmActionEmail {
    pub unsafe fn get_MailFrom(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MailFrom)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_MailFrom(&self, mailfrom: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MailFrom)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailfrom)) }
    }
    pub unsafe fn get_MailReplyTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MailReplyTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_MailReplyTo(&self, mailreplyto: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MailReplyTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailreplyto)) }
    }
    pub unsafe fn get_MailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MailTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_MailTo(&self, mailto: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MailTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)) }
    }
    pub unsafe fn get_MailCc(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MailCc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_MailCc(&self, mailcc: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MailCc)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailcc)) }
    }
    pub unsafe fn get_MailBcc(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MailBcc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_MailBcc(&self, mailbcc: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MailBcc)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailbcc)) }
    }
    pub unsafe fn get_MailSubject(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MailSubject)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_MailSubject(&self, mailsubject: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MailSubject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailsubject)) }
    }
    pub unsafe fn get_MessageText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MessageText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_MessageText(&self, messagetext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MessageText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(messagetext)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub get_MailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MailReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MailReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MailCc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MailCc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MailBcc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MailBcc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MailSubject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MailSubject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmActionEmail_Impl: IFsrmAction_Impl {
    fn get_MailFrom(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_MailFrom(&self, mailfrom: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MailReplyTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_MailReplyTo(&self, mailreplyto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MailTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_MailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MailCc(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_MailCc(&self, mailcc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MailBcc(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_MailBcc(&self, mailbcc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MailSubject(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_MailSubject(&self, mailsubject: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MessageText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_MessageText(&self, messagetext: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmActionEmail_Vtbl {
    pub const fn new<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_MailFrom<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailfrom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::get_MailFrom(this) {
                    Ok(ok__) => {
                        mailfrom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MailFrom<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailfrom: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::put_MailFrom(this, core::mem::transmute(&mailfrom)).into()
            }
        }
        unsafe extern "system" fn get_MailReplyTo<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailreplyto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::get_MailReplyTo(this) {
                    Ok(ok__) => {
                        mailreplyto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MailReplyTo<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailreplyto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::put_MailReplyTo(this, core::mem::transmute(&mailreplyto)).into()
            }
        }
        unsafe extern "system" fn get_MailTo<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::get_MailTo(this) {
                    Ok(ok__) => {
                        mailto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MailTo<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::put_MailTo(this, core::mem::transmute(&mailto)).into()
            }
        }
        unsafe extern "system" fn get_MailCc<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailcc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::get_MailCc(this) {
                    Ok(ok__) => {
                        mailcc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MailCc<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailcc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::put_MailCc(this, core::mem::transmute(&mailcc)).into()
            }
        }
        unsafe extern "system" fn get_MailBcc<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailbcc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::get_MailBcc(this) {
                    Ok(ok__) => {
                        mailbcc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MailBcc<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailbcc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::put_MailBcc(this, core::mem::transmute(&mailbcc)).into()
            }
        }
        unsafe extern "system" fn get_MailSubject<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailsubject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::get_MailSubject(this) {
                    Ok(ok__) => {
                        mailsubject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MailSubject<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailsubject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::put_MailSubject(this, core::mem::transmute(&mailsubject)).into()
            }
        }
        unsafe extern "system" fn get_MessageText<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail_Impl::get_MessageText(this) {
                    Ok(ok__) => {
                        messagetext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MessageText<Identity: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail_Impl::put_MessageText(this, core::mem::transmute(&messagetext)).into()
            }
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, OFFSET>(),
            get_MailFrom: get_MailFrom::<Identity, OFFSET>,
            put_MailFrom: put_MailFrom::<Identity, OFFSET>,
            get_MailReplyTo: get_MailReplyTo::<Identity, OFFSET>,
            put_MailReplyTo: put_MailReplyTo::<Identity, OFFSET>,
            get_MailTo: get_MailTo::<Identity, OFFSET>,
            put_MailTo: put_MailTo::<Identity, OFFSET>,
            get_MailCc: get_MailCc::<Identity, OFFSET>,
            put_MailCc: put_MailCc::<Identity, OFFSET>,
            get_MailBcc: get_MailBcc::<Identity, OFFSET>,
            put_MailBcc: put_MailBcc::<Identity, OFFSET>,
            get_MailSubject: get_MailSubject::<Identity, OFFSET>,
            put_MailSubject: put_MailSubject::<Identity, OFFSET>,
            get_MessageText: get_MessageText::<Identity, OFFSET>,
            put_MessageText: put_MessageText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionEmail as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IFsrmActionEmail2, windows_core::IUnknown, super::IDispatch, IFsrmAction, IFsrmActionEmail);
#[cfg(feature = "oaidl")]
impl IFsrmActionEmail2 {
    pub unsafe fn get_AttachmentFileListSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AttachmentFileListSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AttachmentFileListSize(&self, attachmentfilelistsize: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AttachmentFileListSize)(windows_core::Interface::as_raw(self), attachmentfilelistsize) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail2_Vtbl {
    pub base__: IFsrmActionEmail_Vtbl,
    pub get_AttachmentFileListSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_AttachmentFileListSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmActionEmail2_Impl: IFsrmActionEmail_Impl {
    fn get_AttachmentFileListSize(&self) -> windows_core::Result<i32>;
    fn put_AttachmentFileListSize(&self, attachmentfilelistsize: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmActionEmail2_Vtbl {
    pub const fn new<Identity: IFsrmActionEmail2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AttachmentFileListSize<Identity: IFsrmActionEmail2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachmentfilelistsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEmail2_Impl::get_AttachmentFileListSize(this) {
                    Ok(ok__) => {
                        attachmentfilelistsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AttachmentFileListSize<Identity: IFsrmActionEmail2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attachmentfilelistsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEmail2_Impl::put_AttachmentFileListSize(this, core::mem::transmute_copy(&attachmentfilelistsize)).into()
            }
        }
        Self {
            base__: IFsrmActionEmail_Vtbl::new::<Identity, OFFSET>(),
            get_AttachmentFileListSize: get_AttachmentFileListSize::<Identity, OFFSET>,
            put_AttachmentFileListSize: put_AttachmentFileListSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionEmail2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID || iid == &<IFsrmActionEmail as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IFsrmActionEventLog, windows_core::IUnknown, super::IDispatch, IFsrmAction);
#[cfg(feature = "oaidl")]
impl IFsrmActionEventLog {
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_EventType(&self) -> windows_core::Result<super::FsrmEventType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn put_EventType(&self, eventtype: super::FsrmEventType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EventType)(windows_core::Interface::as_raw(self), eventtype) }
    }
    pub unsafe fn get_MessageText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MessageText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_MessageText(&self, messagetext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MessageText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(messagetext)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEventLog_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub get_EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmEventType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_EventType: usize,
    #[cfg(feature = "fsrmenums")]
    pub put_EventType: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmEventType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    put_EventType: usize,
    pub get_MessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmActionEventLog_Impl: IFsrmAction_Impl {
    fn get_EventType(&self) -> windows_core::Result<super::FsrmEventType>;
    fn put_EventType(&self, eventtype: super::FsrmEventType) -> windows_core::Result<()>;
    fn get_MessageText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_MessageText(&self, messagetext: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmActionEventLog_Vtbl {
    pub const fn new<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_EventType<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventtype: *mut super::FsrmEventType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEventLog_Impl::get_EventType(this) {
                    Ok(ok__) => {
                        eventtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EventType<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventtype: super::FsrmEventType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEventLog_Impl::put_EventType(this, core::mem::transmute_copy(&eventtype)).into()
            }
        }
        unsafe extern "system" fn get_MessageText<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionEventLog_Impl::get_MessageText(this) {
                    Ok(ok__) => {
                        messagetext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MessageText<Identity: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionEventLog_Impl::put_MessageText(this, core::mem::transmute(&messagetext)).into()
            }
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, OFFSET>(),
            get_EventType: get_EventType::<Identity, OFFSET>,
            put_EventType: put_EventType::<Identity, OFFSET>,
            get_MessageText: get_MessageText::<Identity, OFFSET>,
            put_MessageText: put_MessageText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionEventLog as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IFsrmActionReport, windows_core::IUnknown, super::IDispatch, IFsrmAction);
#[cfg(feature = "oaidl")]
impl IFsrmActionReport {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_ReportTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReportTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_ReportTypes(&self, reporttypes: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ReportTypes)(windows_core::Interface::as_raw(self), reporttypes) }
    }
    pub unsafe fn get_MailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MailTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_MailTo(&self, mailto: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MailTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionReport_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_ReportTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_ReportTypes: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_ReportTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_ReportTypes: usize,
    pub get_MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmActionReport_Impl: IFsrmAction_Impl {
    fn get_ReportTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_ReportTypes(&self, reporttypes: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_MailTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_MailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmActionReport_Vtbl {
    pub const fn new<Identity: IFsrmActionReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ReportTypes<Identity: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttypes: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionReport_Impl::get_ReportTypes(this) {
                    Ok(ok__) => {
                        reporttypes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ReportTypes<Identity: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttypes: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionReport_Impl::put_ReportTypes(this, core::mem::transmute_copy(&reporttypes)).into()
            }
        }
        unsafe extern "system" fn get_MailTo<Identity: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmActionReport_Impl::get_MailTo(this) {
                    Ok(ok__) => {
                        mailto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MailTo<Identity: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmActionReport_Impl::put_MailTo(this, core::mem::transmute(&mailto)).into()
            }
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, OFFSET>(),
            get_ReportTypes: get_ReportTypes::<Identity, OFFSET>,
            put_ReportTypes: put_ReportTypes::<Identity, OFFSET>,
            get_MailTo: get_MailTo::<Identity, OFFSET>,
            put_MailTo: put_MailTo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmActionReport as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmActionReport {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmCollection, IFsrmCollection_Vtbl, 0xf76fbf3b_8ddd_4b42_b05a_cb1c3ff1fee8);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmCollection {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_State(&self) -> windows_core::Result<super::FsrmCollectionState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitForCompletion)(windows_core::Interface::as_raw(self), waitseconds, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "fsrmenums", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetById(&self, id: super::FSRM_OBJECT_ID) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetById)(windows_core::Interface::as_raw(self), id, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Item: usize,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub get_State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmCollectionState) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_State: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub WaitForCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    WaitForCompletion: usize,
    #[cfg(all(feature = "fsrmenums", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetById: unsafe extern "system" fn(*mut core::ffi::c_void, super::FSRM_OBJECT_ID, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrmenums", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetById: usize,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmCollection_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, index: i32) -> windows_core::Result<super::VARIANT>;
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn get_State(&self) -> windows_core::Result<super::FsrmCollectionState>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::VARIANT_BOOL>;
    fn GetById(&self, id: &super::FSRM_OBJECT_ID) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmCollection_Vtbl {
    pub const fn new<Identity: IFsrmCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        unknown.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, item: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_State<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut super::FsrmCollectionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmCollection_Impl::get_State(this) {
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
        unsafe extern "system" fn WaitForCompletion<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, waitseconds: i32, completed: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetById<Identity: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::FSRM_OBJECT_ID, entry: *mut super::VARIANT) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            get_State: get_State::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, OFFSET>,
            GetById: GetById::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IFsrmCommittableCollection, windows_core::IUnknown, super::IDispatch, IFsrmCollection, IFsrmMutableCollection);
#[cfg(feature = "oaidl")]
impl IFsrmCommittableCollection {
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn Commit(&self, options: super::FsrmCommitOptions) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCommittableCollection_Vtbl {
    pub base__: IFsrmMutableCollection_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmCommitOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    Commit: usize,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmCommittableCollection_Impl: IFsrmMutableCollection_Impl {
    fn Commit(&self, options: super::FsrmCommitOptions) -> windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmCommittableCollection_Vtbl {
    pub const fn new<Identity: IFsrmCommittableCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: IFsrmCommittableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: super::FsrmCommitOptions, results: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        iid == &<IFsrmCommittableCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmCollection as windows_core::Interface>::IID || iid == &<IFsrmMutableCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmCommittableCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmDerivedObjectsResult, IFsrmDerivedObjectsResult_Vtbl, 0x39322a2d_38ee_4d0d_8095_421a80849a82);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmDerivedObjectsResult {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmDerivedObjectsResult, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmDerivedObjectsResult {
    pub unsafe fn get_DerivedObjects(&self) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DerivedObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Results(&self) -> windows_core::Result<IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Results)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmDerivedObjectsResult_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_DerivedObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Results: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmDerivedObjectsResult_Impl: super::IDispatch_Impl {
    fn get_DerivedObjects(&self) -> windows_core::Result<IFsrmCollection>;
    fn get_Results(&self) -> windows_core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmDerivedObjectsResult_Vtbl {
    pub const fn new<Identity: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DerivedObjects<Identity: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, derivedobjects: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmDerivedObjectsResult_Impl::get_DerivedObjects(this) {
                    Ok(ok__) => {
                        derivedobjects.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Results<Identity: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, results: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmDerivedObjectsResult_Impl::get_Results(this) {
                    Ok(ok__) => {
                        results.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_DerivedObjects: get_DerivedObjects::<Identity, OFFSET>,
            get_Results: get_Results::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmDerivedObjectsResult as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmDerivedObjectsResult {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmExportImport, IFsrmExportImport_Vtbl, 0xefcb0ab1_16c4_4a79_812c_725614c3306b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmExportImport {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmExportImport, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmExportImport {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ExportFileGroups(&self, filepath: &windows_core::BSTR, filegroupnamessafearray: *const super::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExportFileGroups)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), filegroupnamessafearray, core::mem::transmute_copy(remotehost)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ImportFileGroups(&self, filepath: &windows_core::BSTR, filegroupnamessafearray: *const super::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportFileGroups)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), filegroupnamessafearray, core::mem::transmute_copy(remotehost), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ExportFileScreenTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExportFileScreenTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), templatenamessafearray, core::mem::transmute_copy(remotehost)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ImportFileScreenTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportFileScreenTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), templatenamessafearray, core::mem::transmute_copy(remotehost), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ExportQuotaTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExportQuotaTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), templatenamessafearray, core::mem::transmute_copy(remotehost)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ImportQuotaTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportQuotaTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), templatenamessafearray, core::mem::transmute_copy(remotehost), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmExportImport_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ExportFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ExportFileGroups: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ImportFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::VARIANT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ImportFileGroups: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ExportFileScreenTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ExportFileScreenTemplates: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ImportFileScreenTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::VARIANT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ImportFileScreenTemplates: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ExportQuotaTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ExportQuotaTemplates: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ImportQuotaTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::VARIANT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ImportQuotaTemplates: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmExportImport_Impl: super::IDispatch_Impl {
    fn ExportFileGroups(&self, filepath: &windows_core::BSTR, filegroupnamessafearray: *const super::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportFileGroups(&self, filepath: &windows_core::BSTR, filegroupnamessafearray: *const super::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection>;
    fn ExportFileScreenTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportFileScreenTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection>;
    fn ExportQuotaTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportQuotaTemplates(&self, filepath: &windows_core::BSTR, templatenamessafearray: *const super::VARIANT, remotehost: &windows_core::BSTR) -> windows_core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmExportImport_Vtbl {
    pub const fn new<Identity: IFsrmExportImport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExportFileGroups<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, filegroupnamessafearray: *const super::VARIANT, remotehost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmExportImport_Impl::ExportFileGroups(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&filegroupnamessafearray), core::mem::transmute(&remotehost)).into()
            }
        }
        unsafe extern "system" fn ImportFileGroups<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, filegroupnamessafearray: *const super::VARIANT, remotehost: *mut core::ffi::c_void, filegroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ExportFileScreenTemplates<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, templatenamessafearray: *const super::VARIANT, remotehost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmExportImport_Impl::ExportFileScreenTemplates(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&templatenamessafearray), core::mem::transmute(&remotehost)).into()
            }
        }
        unsafe extern "system" fn ImportFileScreenTemplates<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, templatenamessafearray: *const super::VARIANT, remotehost: *mut core::ffi::c_void, templates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ExportQuotaTemplates<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, templatenamessafearray: *const super::VARIANT, remotehost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmExportImport_Impl::ExportQuotaTemplates(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&templatenamessafearray), core::mem::transmute(&remotehost)).into()
            }
        }
        unsafe extern "system" fn ImportQuotaTemplates<Identity: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, templatenamessafearray: *const super::VARIANT, remotehost: *mut core::ffi::c_void, templates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ExportFileGroups: ExportFileGroups::<Identity, OFFSET>,
            ImportFileGroups: ImportFileGroups::<Identity, OFFSET>,
            ExportFileScreenTemplates: ExportFileScreenTemplates::<Identity, OFFSET>,
            ImportFileScreenTemplates: ImportFileScreenTemplates::<Identity, OFFSET>,
            ExportQuotaTemplates: ExportQuotaTemplates::<Identity, OFFSET>,
            ImportQuotaTemplates: ImportQuotaTemplates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmExportImport as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IFsrmMutableCollection, windows_core::IUnknown, super::IDispatch, IFsrmCollection);
#[cfg(feature = "oaidl")]
impl IFsrmMutableCollection {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Add(&self, item: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(item)) }
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn RemoveById(&self, id: super::FSRM_OBJECT_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveById)(windows_core::Interface::as_raw(self), id) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmMutableCollection_Vtbl {
    pub base__: IFsrmCollection_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub RemoveById: unsafe extern "system" fn(*mut core::ffi::c_void, super::FSRM_OBJECT_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    RemoveById: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmMutableCollection_Impl: IFsrmCollection_Impl {
    fn Add(&self, item: &super::VARIANT) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn RemoveById(&self, id: &super::FSRM_OBJECT_ID) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IFsrmMutableCollection>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmMutableCollection_Vtbl {
    pub const fn new<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn RemoveById<Identity: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::FSRM_OBJECT_ID) -> windows_core::HRESULT {
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
        iid == &<IFsrmMutableCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmMutableCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmObject, IFsrmObject_Vtbl, 0x22bcef93_4a3f_4183_89f9_2f8b8a628aee);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmObject {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmObject, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmObject {
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_Id(&self) -> windows_core::Result<super::FSRM_OBJECT_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, description: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)) }
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
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub get_Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FSRM_OBJECT_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_Id: usize,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmObject_Impl: super::IDispatch_Impl {
    fn get_Id(&self) -> windows_core::Result<super::FSRM_OBJECT_ID>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmObject_Vtbl {
    pub const fn new<Identity: IFsrmObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Id<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut super::FSRM_OBJECT_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmObject_Impl::get_Id(this) {
                    Ok(ok__) => {
                        id.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Description<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmObject_Impl::get_Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IFsrmObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmObject_Impl::put_Description(this, core::mem::transmute(&description)).into()
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Id: get_Id::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmObject as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmObject {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmPathMapper, IFsrmPathMapper_Vtbl, 0x6f4dbfff_6920_4821_a6c3_b7e94c1fd60c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmPathMapper {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmPathMapper, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmPathMapper {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn GetSharePathsForLocalPath(&self, localpath: &windows_core::BSTR) -> windows_core::Result<*mut super::SAFEARRAY> {
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
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub GetSharePathsForLocalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    GetSharePathsForLocalPath: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPathMapper_Impl: super::IDispatch_Impl {
    fn GetSharePathsForLocalPath(&self, localpath: &windows_core::BSTR) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPathMapper_Vtbl {
    pub const fn new<Identity: IFsrmPathMapper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSharePathsForLocalPath<Identity: IFsrmPathMapper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpath: *mut core::ffi::c_void, sharepaths: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetSharePathsForLocalPath: GetSharePathsForLocalPath::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPathMapper as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmPathMapper {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmSetting, IFsrmSetting_Vtbl, 0xf411d4fd_14be_4260_8c40_03b7c95e608a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmSetting {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmSetting, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmSetting {
    pub unsafe fn get_SmtpServer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SmtpServer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_SmtpServer(&self, smtpserver: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SmtpServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(smtpserver)) }
    }
    pub unsafe fn get_MailFrom(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MailFrom)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_MailFrom(&self, mailfrom: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MailFrom)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailfrom)) }
    }
    pub unsafe fn get_AdminEmail(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AdminEmail)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_AdminEmail(&self, adminemail: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AdminEmail)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(adminemail)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DisableCommandLine(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisableCommandLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_DisableCommandLine(&self, disablecommandline: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisableCommandLine)(windows_core::Interface::as_raw(self), disablecommandline) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_EnableScreeningAudit(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EnableScreeningAudit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_EnableScreeningAudit(&self, enablescreeningaudit: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EnableScreeningAudit)(windows_core::Interface::as_raw(self), enablescreeningaudit) }
    }
    pub unsafe fn EmailTest(&self, mailto: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EmailTest)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn SetActionRunLimitInterval(&self, actiontype: super::FsrmActionType, delaytimeminutes: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetActionRunLimitInterval)(windows_core::Interface::as_raw(self), actiontype, delaytimeminutes) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn GetActionRunLimitInterval(&self, actiontype: super::FsrmActionType) -> windows_core::Result<i32> {
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
    pub base__: super::IDispatch_Vtbl,
    pub get_SmtpServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SmtpServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_AdminEmail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_AdminEmail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_DisableCommandLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DisableCommandLine: usize,
    #[cfg(feature = "wtypes")]
    pub put_DisableCommandLine: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_DisableCommandLine: usize,
    #[cfg(feature = "wtypes")]
    pub get_EnableScreeningAudit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_EnableScreeningAudit: usize,
    #[cfg(feature = "wtypes")]
    pub put_EnableScreeningAudit: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_EnableScreeningAudit: usize,
    pub EmailTest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub SetActionRunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmActionType, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    SetActionRunLimitInterval: usize,
    #[cfg(feature = "fsrmenums")]
    pub GetActionRunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmActionType, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    GetActionRunLimitInterval: usize,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmSetting_Impl: super::IDispatch_Impl {
    fn get_SmtpServer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_SmtpServer(&self, smtpserver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MailFrom(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_MailFrom(&self, mailfrom: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_AdminEmail(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_AdminEmail(&self, adminemail: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_DisableCommandLine(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_DisableCommandLine(&self, disablecommandline: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_EnableScreeningAudit(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_EnableScreeningAudit(&self, enablescreeningaudit: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EmailTest(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetActionRunLimitInterval(&self, actiontype: super::FsrmActionType, delaytimeminutes: i32) -> windows_core::Result<()>;
    fn GetActionRunLimitInterval(&self, actiontype: super::FsrmActionType) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmSetting_Vtbl {
    pub const fn new<Identity: IFsrmSetting_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_SmtpServer<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smtpserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::get_SmtpServer(this) {
                    Ok(ok__) => {
                        smtpserver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SmtpServer<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, smtpserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::put_SmtpServer(this, core::mem::transmute(&smtpserver)).into()
            }
        }
        unsafe extern "system" fn get_MailFrom<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailfrom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::get_MailFrom(this) {
                    Ok(ok__) => {
                        mailfrom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MailFrom<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailfrom: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::put_MailFrom(this, core::mem::transmute(&mailfrom)).into()
            }
        }
        unsafe extern "system" fn get_AdminEmail<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adminemail: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::get_AdminEmail(this) {
                    Ok(ok__) => {
                        adminemail.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AdminEmail<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adminemail: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::put_AdminEmail(this, core::mem::transmute(&adminemail)).into()
            }
        }
        unsafe extern "system" fn get_DisableCommandLine<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disablecommandline: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::get_DisableCommandLine(this) {
                    Ok(ok__) => {
                        disablecommandline.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisableCommandLine<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disablecommandline: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::put_DisableCommandLine(this, core::mem::transmute_copy(&disablecommandline)).into()
            }
        }
        unsafe extern "system" fn get_EnableScreeningAudit<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enablescreeningaudit: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmSetting_Impl::get_EnableScreeningAudit(this) {
                    Ok(ok__) => {
                        enablescreeningaudit.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EnableScreeningAudit<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enablescreeningaudit: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::put_EnableScreeningAudit(this, core::mem::transmute_copy(&enablescreeningaudit)).into()
            }
        }
        unsafe extern "system" fn EmailTest<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::EmailTest(this, core::mem::transmute(&mailto)).into()
            }
        }
        unsafe extern "system" fn SetActionRunLimitInterval<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actiontype: super::FsrmActionType, delaytimeminutes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmSetting_Impl::SetActionRunLimitInterval(this, core::mem::transmute_copy(&actiontype), core::mem::transmute_copy(&delaytimeminutes)).into()
            }
        }
        unsafe extern "system" fn GetActionRunLimitInterval<Identity: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actiontype: super::FsrmActionType, delaytimeminutes: *mut i32) -> windows_core::HRESULT {
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
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_SmtpServer: get_SmtpServer::<Identity, OFFSET>,
            put_SmtpServer: put_SmtpServer::<Identity, OFFSET>,
            get_MailFrom: get_MailFrom::<Identity, OFFSET>,
            put_MailFrom: put_MailFrom::<Identity, OFFSET>,
            get_AdminEmail: get_AdminEmail::<Identity, OFFSET>,
            put_AdminEmail: put_AdminEmail::<Identity, OFFSET>,
            get_DisableCommandLine: get_DisableCommandLine::<Identity, OFFSET>,
            put_DisableCommandLine: put_DisableCommandLine::<Identity, OFFSET>,
            get_EnableScreeningAudit: get_EnableScreeningAudit::<Identity, OFFSET>,
            put_EnableScreeningAudit: put_EnableScreeningAudit::<Identity, OFFSET>,
            EmailTest: EmailTest::<Identity, OFFSET>,
            SetActionRunLimitInterval: SetActionRunLimitInterval::<Identity, OFFSET>,
            GetActionRunLimitInterval: GetActionRunLimitInterval::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmSetting as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmSetting {}

#[cfg(all(feature = "windef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn BrowseForGPO(lpbrowseinfo: LPGPOBROWSEINFO) -> windows_core::HRESULT {
    windows_core::link!("gpedit.dll" "system" fn BrowseForGPO(lpbrowseinfo : LPGPOBROWSEINFO) -> windows_core::HRESULT);
    unsafe { BrowseForGPO(lpbrowseinfo) }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn CreateGPOLink(lpgpo: super::LPOLESTR, lpcontainer: super::LPOLESTR, fhighpriority: bool) -> windows_core::HRESULT {
    windows_core::link!("gpedit.dll" "system" fn CreateGPOLink(lpgpo : super::LPOLESTR, lpcontainer : super::LPOLESTR, fhighpriority : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { CreateGPOLink(lpgpo, lpcontainer, fhighpriority.into()) }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn DeleteAllGPOLinks(lpcontainer: super::LPOLESTR) -> windows_core::HRESULT {
    windows_core::link!("gpedit.dll" "system" fn DeleteAllGPOLinks(lpcontainer : super::LPOLESTR) -> windows_core::HRESULT);
    unsafe { DeleteAllGPOLinks(lpcontainer) }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn DeleteGPOLink(lpgpo: super::LPOLESTR, lpcontainer: super::LPOLESTR) -> windows_core::HRESULT {
    windows_core::link!("gpedit.dll" "system" fn DeleteGPOLink(lpgpo : super::LPOLESTR, lpcontainer : super::LPOLESTR) -> windows_core::HRESULT);
    unsafe { DeleteGPOLink(lpgpo, lpcontainer) }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn ExportRSoPData(lpnamespace: super::LPOLESTR, lpfilename: super::LPOLESTR) -> windows_core::HRESULT {
    windows_core::link!("gpedit.dll" "system" fn ExportRSoPData(lpnamespace : super::LPOLESTR, lpfilename : super::LPOLESTR) -> windows_core::HRESULT);
    unsafe { ExportRSoPData(lpnamespace, lpfilename) }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn ImportRSoPData(lpnamespace: super::LPOLESTR, lpfilename: super::LPOLESTR) -> windows_core::HRESULT {
    windows_core::link!("gpedit.dll" "system" fn ImportRSoPData(lpnamespace : super::LPOLESTR, lpfilename : super::LPOLESTR) -> windows_core::HRESULT);
    unsafe { ImportRSoPData(lpnamespace, lpfilename) }
}
pub const CLSID_GPESnapIn: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b734_a0e1_11d1_a7d3_0000f87571e3);
pub const CLSID_GroupPolicyObject: windows_core::GUID = windows_core::GUID::from_u128(0xea502722_a23d_11d1_a7d3_0000f87571e3);
pub const CLSID_RSOPSnapIn: windows_core::GUID = windows_core::GUID::from_u128(0x6dc3804b_7212_458d_adb0_9a07e2ae1fa2);
pub const GPHintDomain: GROUP_POLICY_HINT_TYPE = 3;
pub const GPHintMachine: GROUP_POLICY_HINT_TYPE = 1;
pub const GPHintOrganizationalUnit: GROUP_POLICY_HINT_TYPE = 4;
pub const GPHintSite: GROUP_POLICY_HINT_TYPE = 2;
pub const GPHintUnknown: GROUP_POLICY_HINT_TYPE = 0;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPOBROWSEINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::HWND,
    pub lpTitle: super::LPOLESTR,
    pub lpInitialOU: super::LPOLESTR,
    pub lpDSPath: super::LPOLESTR,
    pub dwDSPathSize: u32,
    pub lpName: super::LPOLESTR,
    pub dwNameSize: u32,
    pub gpoType: GROUP_POLICY_OBJECT_TYPE,
    pub gpoHint: GROUP_POLICY_HINT_TYPE,
}
pub const GPOTypeDS: GROUP_POLICY_OBJECT_TYPE = 2;
pub const GPOTypeLocal: GROUP_POLICY_OBJECT_TYPE = 0;
pub const GPOTypeLocalGroup: GROUP_POLICY_OBJECT_TYPE = 4;
pub const GPOTypeLocalUser: GROUP_POLICY_OBJECT_TYPE = 3;
pub const GPOTypeRemote: GROUP_POLICY_OBJECT_TYPE = 1;
pub const GPO_BROWSE_DISABLENEW: i32 = 1;
pub const GPO_BROWSE_INITTOALL: i32 = 16;
pub const GPO_BROWSE_NOCOMPUTERS: i32 = 2;
pub const GPO_BROWSE_NODSGPOS: i32 = 4;
pub const GPO_BROWSE_NOUSERGPOS: i32 = 32;
pub const GPO_BROWSE_OPENBUTTON: i32 = 8;
pub const GPO_BROWSE_SENDAPPLYONEDIT: i32 = 64;
pub const GPO_OPEN_LOAD_REGISTRY: i32 = 1;
pub const GPO_OPEN_READ_ONLY: i32 = 2;
pub const GPO_OPTION_DISABLE_MACHINE: i32 = 2;
pub const GPO_OPTION_DISABLE_USER: i32 = 1;
pub const GPO_SECTION_MACHINE: i32 = 2;
pub const GPO_SECTION_ROOT: i32 = 0;
pub const GPO_SECTION_USER: i32 = 1;
pub type GROUP_POLICY_HINT_TYPE = i32;
pub type GROUP_POLICY_OBJECT_TYPE = i32;
windows_core::imp::define_interface!(IGPEInformation, IGPEInformation_Vtbl, 0x8fc0b735_a0e1_11d1_a7d3_0000f87571e3);
windows_core::imp::interface_hierarchy!(IGPEInformation, windows_core::IUnknown);
impl IGPEInformation {
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetName(&self, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), pszname as _, cchmaxlength) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetDisplayName(&self, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), pszname as _, cchmaxlength) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetRegistryKey(&self, dwsection: u32) -> windows_core::Result<super::HKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegistryKey)(windows_core::Interface::as_raw(self), dwsection, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: super::LPOLESTR, cchmaxpath: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDSPath)(windows_core::Interface::as_raw(self), dwsection, pszpath as _, cchmaxpath) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: super::LPOLESTR, cchmaxpath: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileSysPath)(windows_core::Interface::as_raw(self), dwsection, pszpath as _, cchmaxpath) }
    }
    pub unsafe fn GetOptions(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<GROUP_POLICY_OBJECT_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHint(&self) -> windows_core::Result<GROUP_POLICY_HINT_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PolicyChanged(&self, bmachine: bool, badd: bool, pguidextension: *mut windows_core::GUID, pguidsnapin: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PolicyChanged)(windows_core::Interface::as_raw(self), bmachine.into(), badd.into(), pguidextension as _, pguidsnapin as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPEInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypesbase")]
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetName: usize,
    #[cfg(feature = "wtypesbase")]
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetDisplayName: usize,
    #[cfg(feature = "minwindef")]
    pub GetRegistryKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetRegistryKey: usize,
    #[cfg(feature = "wtypesbase")]
    pub GetDSPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::LPOLESTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetDSPath: usize,
    #[cfg(feature = "wtypesbase")]
    pub GetFileSysPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::LPOLESTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetFileSysPath: usize,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::HRESULT,
    pub GetHint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GROUP_POLICY_HINT_TYPE) -> windows_core::HRESULT,
    pub PolicyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
pub trait IGPEInformation_Impl: windows_core::IUnknownImpl {
    fn GetName(&self, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetDisplayName(&self, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetRegistryKey(&self, dwsection: u32) -> windows_core::Result<super::HKEY>;
    fn GetDSPath(&self, dwsection: u32, pszpath: super::LPOLESTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetFileSysPath(&self, dwsection: u32, pszpath: super::LPOLESTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetOptions(&self) -> windows_core::Result<u32>;
    fn GetType(&self) -> windows_core::Result<GROUP_POLICY_OBJECT_TYPE>;
    fn GetHint(&self) -> windows_core::Result<GROUP_POLICY_HINT_TYPE>;
    fn PolicyChanged(&self, bmachine: windows_core::BOOL, badd: windows_core::BOOL, pguidextension: *mut windows_core::GUID, pguidsnapin: *mut windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
impl IGPEInformation_Vtbl {
    pub const fn new<Identity: IGPEInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetDisplayName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetRegistryKey<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, hkey: *mut super::HKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPEInformation_Impl::GetRegistryKey(this, core::mem::transmute_copy(&dwsection)) {
                    Ok(ok__) => {
                        hkey.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDSPath<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: super::LPOLESTR, cchmaxpath: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetDSPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
            }
        }
        unsafe extern "system" fn GetFileSysPath<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: super::LPOLESTR, cchmaxpath: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetFileSysPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
            }
        }
        unsafe extern "system" fn GetOptions<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPEInformation_Impl::GetOptions(this) {
                    Ok(ok__) => {
                        dwoptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPEInformation_Impl::GetType(this) {
                    Ok(ok__) => {
                        gpotype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHint<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gphint: *mut GROUP_POLICY_HINT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGPEInformation_Impl::GetHint(this) {
                    Ok(ok__) => {
                        gphint.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PolicyChanged<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmachine: windows_core::BOOL, badd: windows_core::BOOL, pguidextension: *mut windows_core::GUID, pguidsnapin: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::PolicyChanged(this, core::mem::transmute_copy(&bmachine), core::mem::transmute_copy(&badd), core::mem::transmute_copy(&pguidextension), core::mem::transmute_copy(&pguidsnapin)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            GetRegistryKey: GetRegistryKey::<Identity, OFFSET>,
            GetDSPath: GetDSPath::<Identity, OFFSET>,
            GetFileSysPath: GetFileSysPath::<Identity, OFFSET>,
            GetOptions: GetOptions::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetHint: GetHint::<Identity, OFFSET>,
            PolicyChanged: PolicyChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPEInformation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IGPEInformation {}
windows_core::imp::define_interface!(IGroupPolicyObject, IGroupPolicyObject_Vtbl, 0xea502723_a23d_11d1_a7d3_0000f87571e3);
windows_core::imp::interface_hierarchy!(IGroupPolicyObject, windows_core::IUnknown);
impl IGroupPolicyObject {
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn New(&self, pszdomainname: super::LPOLESTR, pszdisplayname: Option<super::LPOLESTR>, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).New)(windows_core::Interface::as_raw(self), pszdomainname, pszdisplayname.unwrap_or(core::mem::zeroed()) as _, dwflags) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn OpenDSGPO(&self, pszpath: super::LPOLESTR, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OpenDSGPO)(windows_core::Interface::as_raw(self), pszpath, dwflags) }
    }
    pub unsafe fn OpenLocalMachineGPO(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OpenLocalMachineGPO)(windows_core::Interface::as_raw(self), dwflags) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn OpenRemoteMachineGPO(&self, pszcomputername: super::LPOLESTR, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OpenRemoteMachineGPO)(windows_core::Interface::as_raw(self), pszcomputername, dwflags) }
    }
    pub unsafe fn Save(&self, bmachine: bool, badd: bool, pguidextension: *mut windows_core::GUID, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), bmachine.into(), badd.into(), pguidextension as _, pguid as _) }
    }
    pub unsafe fn Delete(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetName(&self, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), pszname as _, cchmaxlength) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetDisplayName(&self, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), pszname as _, cchmaxlength) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn SetDisplayName(&self, pszname: super::LPOLESTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), pszname) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetPath(&self, pszpath: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), pszpath as _, cchmaxlength) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: super::LPOLESTR, cchmaxpath: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDSPath)(windows_core::Interface::as_raw(self), dwsection, pszpath as _, cchmaxpath) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: super::LPOLESTR, cchmaxpath: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileSysPath)(windows_core::Interface::as_raw(self), dwsection, pszpath as _, cchmaxpath) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetRegistryKey(&self, dwsection: u32) -> windows_core::Result<super::HKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegistryKey)(windows_core::Interface::as_raw(self), dwsection, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOptions(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOptions(&self, dwoptions: u32, dwmask: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOptions)(windows_core::Interface::as_raw(self), dwoptions, dwmask) }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<GROUP_POLICY_OBJECT_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetMachineName(&self, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMachineName)(windows_core::Interface::as_raw(self), pszname as _, cchmaxlength) }
    }
    #[cfg(feature = "prsht")]
    pub unsafe fn GetPropertySheetPages(&self, hpages: *mut *mut super::HPROPSHEETPAGE, upagecount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertySheetPages)(windows_core::Interface::as_raw(self), hpages as _, upagecount as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGroupPolicyObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypesbase")]
    pub New: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, super::LPOLESTR, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    New: usize,
    #[cfg(feature = "wtypesbase")]
    pub OpenDSGPO: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    OpenDSGPO: usize,
    pub OpenLocalMachineGPO: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypesbase")]
    pub OpenRemoteMachineGPO: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    OpenRemoteMachineGPO: usize,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypesbase")]
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetName: usize,
    #[cfg(feature = "wtypesbase")]
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetDisplayName: usize,
    #[cfg(feature = "wtypesbase")]
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    SetDisplayName: usize,
    #[cfg(feature = "wtypesbase")]
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetPath: usize,
    #[cfg(feature = "wtypesbase")]
    pub GetDSPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::LPOLESTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetDSPath: usize,
    #[cfg(feature = "wtypesbase")]
    pub GetFileSysPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::LPOLESTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetFileSysPath: usize,
    #[cfg(feature = "minwindef")]
    pub GetRegistryKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetRegistryKey: usize,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "wtypesbase")]
    pub GetMachineName: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetMachineName: usize,
    #[cfg(feature = "prsht")]
    pub GetPropertySheetPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::HPROPSHEETPAGE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "prsht"))]
    GetPropertySheetPages: usize,
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "wtypesbase"))]
pub trait IGroupPolicyObject_Impl: windows_core::IUnknownImpl {
    fn New(&self, pszdomainname: super::LPOLESTR, pszdisplayname: super::LPOLESTR, dwflags: u32) -> windows_core::Result<()>;
    fn OpenDSGPO(&self, pszpath: super::LPOLESTR, dwflags: u32) -> windows_core::Result<()>;
    fn OpenLocalMachineGPO(&self, dwflags: u32) -> windows_core::Result<()>;
    fn OpenRemoteMachineGPO(&self, pszcomputername: super::LPOLESTR, dwflags: u32) -> windows_core::Result<()>;
    fn Save(&self, bmachine: windows_core::BOOL, badd: windows_core::BOOL, pguidextension: *mut windows_core::GUID, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn GetName(&self, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetDisplayName(&self, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn SetDisplayName(&self, pszname: super::LPOLESTR) -> windows_core::Result<()>;
    fn GetPath(&self, pszpath: super::LPOLESTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetDSPath(&self, dwsection: u32, pszpath: super::LPOLESTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetFileSysPath(&self, dwsection: u32, pszpath: super::LPOLESTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetRegistryKey(&self, dwsection: u32) -> windows_core::Result<super::HKEY>;
    fn GetOptions(&self) -> windows_core::Result<u32>;
    fn SetOptions(&self, dwoptions: u32, dwmask: u32) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<GROUP_POLICY_OBJECT_TYPE>;
    fn GetMachineName(&self, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetPropertySheetPages(&self, hpages: *mut *mut super::HPROPSHEETPAGE, upagecount: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "wtypesbase"))]
impl IGroupPolicyObject_Vtbl {
    pub const fn new<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn New<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdomainname: super::LPOLESTR, pszdisplayname: super::LPOLESTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::New(this, core::mem::transmute_copy(&pszdomainname), core::mem::transmute_copy(&pszdisplayname), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OpenDSGPO<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: super::LPOLESTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::OpenDSGPO(this, core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OpenLocalMachineGPO<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::OpenLocalMachineGPO(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OpenRemoteMachineGPO<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcomputername: super::LPOLESTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::OpenRemoteMachineGPO(this, core::mem::transmute_copy(&pszcomputername), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmachine: windows_core::BOOL, badd: windows_core::BOOL, pguidextension: *mut windows_core::GUID, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::Save(this, core::mem::transmute_copy(&bmachine), core::mem::transmute_copy(&badd), core::mem::transmute_copy(&pguidextension), core::mem::transmute_copy(&pguid)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetDisplayName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::SetDisplayName(this, core::mem::transmute_copy(&pszname)).into()
            }
        }
        unsafe extern "system" fn GetPath<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetPath(this, core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetDSPath<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: super::LPOLESTR, cchmaxpath: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetDSPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
            }
        }
        unsafe extern "system" fn GetFileSysPath<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: super::LPOLESTR, cchmaxpath: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetFileSysPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
            }
        }
        unsafe extern "system" fn GetRegistryKey<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, hkey: *mut super::HKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGroupPolicyObject_Impl::GetRegistryKey(this, core::mem::transmute_copy(&dwsection)) {
                    Ok(ok__) => {
                        hkey.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOptions<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGroupPolicyObject_Impl::GetOptions(this) {
                    Ok(ok__) => {
                        dwoptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOptions<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: u32, dwmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::SetOptions(this, core::mem::transmute_copy(&dwoptions), core::mem::transmute_copy(&dwmask)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGroupPolicyObject_Impl::GetType(this) {
                    Ok(ok__) => {
                        gpotype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMachineName<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetMachineName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetPropertySheetPages<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpages: *mut *mut super::HPROPSHEETPAGE, upagecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetPropertySheetPages(this, core::mem::transmute_copy(&hpages), core::mem::transmute_copy(&upagecount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            New: New::<Identity, OFFSET>,
            OpenDSGPO: OpenDSGPO::<Identity, OFFSET>,
            OpenLocalMachineGPO: OpenLocalMachineGPO::<Identity, OFFSET>,
            OpenRemoteMachineGPO: OpenRemoteMachineGPO::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            GetDSPath: GetDSPath::<Identity, OFFSET>,
            GetFileSysPath: GetFileSysPath::<Identity, OFFSET>,
            GetRegistryKey: GetRegistryKey::<Identity, OFFSET>,
            GetOptions: GetOptions::<Identity, OFFSET>,
            SetOptions: SetOptions::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetMachineName: GetMachineName::<Identity, OFFSET>,
            GetPropertySheetPages: GetPropertySheetPages::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGroupPolicyObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IGroupPolicyObject {}
windows_core::imp::define_interface!(IRSOPInformation, IRSOPInformation_Vtbl, 0x9a5a81b5_d9c7_49ef_9d11_ddf50968c48d);
windows_core::imp::interface_hierarchy!(IRSOPInformation, windows_core::IUnknown);
impl IRSOPInformation {
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetNamespace(&self, dwsection: u32, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNamespace)(windows_core::Interface::as_raw(self), dwsection, pszname as _, cchmaxlength) }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetEventLogEntryText(&self, pszeventsource: super::LPOLESTR, pszeventlogname: super::LPOLESTR, pszeventtime: super::LPOLESTR, dweventid: u32) -> windows_core::Result<super::LPOLESTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventLogEntryText)(windows_core::Interface::as_raw(self), pszeventsource, pszeventlogname, pszeventtime, dweventid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRSOPInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypesbase")]
    pub GetNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::LPOLESTR, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetNamespace: usize,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypesbase")]
    pub GetEventLogEntryText: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPOLESTR, super::LPOLESTR, super::LPOLESTR, u32, *mut super::LPOLESTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetEventLogEntryText: usize,
}
#[cfg(feature = "wtypesbase")]
pub trait IRSOPInformation_Impl: windows_core::IUnknownImpl {
    fn GetNamespace(&self, dwsection: u32, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
    fn GetEventLogEntryText(&self, pszeventsource: super::LPOLESTR, pszeventlogname: super::LPOLESTR, pszeventtime: super::LPOLESTR, dweventid: u32) -> windows_core::Result<super::LPOLESTR>;
}
#[cfg(feature = "wtypesbase")]
impl IRSOPInformation_Vtbl {
    pub const fn new<Identity: IRSOPInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNamespace<Identity: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszname: super::LPOLESTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRSOPInformation_Impl::GetNamespace(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRSOPInformation_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventLogEntryText<Identity: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszeventsource: super::LPOLESTR, pszeventlogname: super::LPOLESTR, pszeventtime: super::LPOLESTR, dweventid: u32, ppsztext: *mut super::LPOLESTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRSOPInformation_Impl::GetEventLogEntryText(this, core::mem::transmute_copy(&pszeventsource), core::mem::transmute_copy(&pszeventlogname), core::mem::transmute_copy(&pszeventtime), core::mem::transmute_copy(&dweventid)) {
                    Ok(ok__) => {
                        ppsztext.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNamespace: GetNamespace::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetEventLogEntryText: GetEventLogEntryText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRSOPInformation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypesbase")]
impl windows_core::RuntimeName for IRSOPInformation {}
#[cfg(all(feature = "windef", feature = "wtypesbase"))]
pub type LPGPOBROWSEINFO = *mut GPOBROWSEINFO;
pub const NODEID_Machine: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b737_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_MachineSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b73a_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_RSOPMachine: windows_core::GUID = windows_core::GUID::from_u128(0xbd4c1a2e_0b7a_4a62_a6b0_c0577539c97e);
pub const NODEID_RSOPMachineSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0x6a76273e_eb8e_45db_94c5_25663a5f2c1a);
pub const NODEID_RSOPUser: windows_core::GUID = windows_core::GUID::from_u128(0xab87364f_0cec_4cd8_9bf8_898f34628fb8);
pub const NODEID_RSOPUserSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0xe52c5ce3_fd27_4402_84de_d9a5f2858910);
pub const NODEID_User: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b738_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_UserSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b73c_a0e1_11d1_a7d3_0000f87571e3);
pub type PGROUP_POLICY_HINT_TYPE = *mut GROUP_POLICY_HINT_TYPE;
pub type PGROUP_POLICY_OBJECT_TYPE = *mut GROUP_POLICY_OBJECT_TYPE;
pub const RSOP_INFO_FLAG_DIAGNOSTIC_MODE: i32 = 1;

#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn BrowseForGPO(lpbrowseinfo: *mut GPOBROWSEINFO) -> windows_core::HRESULT {
    windows_core::link!("gpedit.dll" "system" fn BrowseForGPO(lpbrowseinfo : *mut GPOBROWSEINFO) -> windows_core::HRESULT);
    unsafe { BrowseForGPO(lpbrowseinfo as _) }
}
#[inline]
pub unsafe fn CreateGPOLink<P0, P1>(lpgpo: P0, lpcontainer: P1, fhighpriority: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gpedit.dll" "system" fn CreateGPOLink(lpgpo : windows_core::PCWSTR, lpcontainer : windows_core::PCWSTR, fhighpriority : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { CreateGPOLink(lpgpo.param().abi(), lpcontainer.param().abi(), fhighpriority.into()) }
}
#[inline]
pub unsafe fn DeleteAllGPOLinks<P0>(lpcontainer: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gpedit.dll" "system" fn DeleteAllGPOLinks(lpcontainer : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DeleteAllGPOLinks(lpcontainer.param().abi()) }
}
#[inline]
pub unsafe fn DeleteGPOLink<P0, P1>(lpgpo: P0, lpcontainer: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gpedit.dll" "system" fn DeleteGPOLink(lpgpo : windows_core::PCWSTR, lpcontainer : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DeleteGPOLink(lpgpo.param().abi(), lpcontainer.param().abi()) }
}
#[inline]
pub unsafe fn ExportRSoPData<P0, P1>(lpnamespace: P0, lpfilename: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gpedit.dll" "system" fn ExportRSoPData(lpnamespace : windows_core::PCWSTR, lpfilename : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { ExportRSoPData(lpnamespace.param().abi(), lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn ImportRSoPData<P0, P1>(lpnamespace: P0, lpfilename: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gpedit.dll" "system" fn ImportRSoPData(lpnamespace : windows_core::PCWSTR, lpfilename : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { ImportRSoPData(lpnamespace.param().abi(), lpfilename.param().abi()) }
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
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GPOBROWSEINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::windef::HWND,
    pub lpTitle: windows_core::PWSTR,
    pub lpInitialOU: windows_core::PWSTR,
    pub lpDSPath: windows_core::PWSTR,
    pub dwDSPathSize: u32,
    pub lpName: windows_core::PWSTR,
    pub dwNameSize: u32,
    pub gpoType: GROUP_POLICY_OBJECT_TYPE,
    pub gpoHint: GROUP_POLICY_HINT_TYPE,
}
pub const GPOTypeDS: GROUP_POLICY_OBJECT_TYPE = 2;
pub const GPOTypeLocal: GROUP_POLICY_OBJECT_TYPE = 0;
pub const GPOTypeLocalGroup: GROUP_POLICY_OBJECT_TYPE = 4;
pub const GPOTypeLocalUser: GROUP_POLICY_OBJECT_TYPE = 3;
pub const GPOTypeRemote: GROUP_POLICY_OBJECT_TYPE = 1;
pub const GPO_BROWSE_DISABLENEW: u32 = 1;
pub const GPO_BROWSE_INITTOALL: u32 = 16;
pub const GPO_BROWSE_NOCOMPUTERS: u32 = 2;
pub const GPO_BROWSE_NODSGPOS: u32 = 4;
pub const GPO_BROWSE_NOUSERGPOS: u32 = 32;
pub const GPO_BROWSE_OPENBUTTON: u32 = 8;
pub const GPO_BROWSE_SENDAPPLYONEDIT: u32 = 64;
pub const GPO_OPEN_LOAD_REGISTRY: u32 = 1;
pub const GPO_OPEN_READ_ONLY: u32 = 2;
pub const GPO_OPTION_DISABLE_MACHINE: u32 = 2;
pub const GPO_OPTION_DISABLE_USER: u32 = 1;
pub const GPO_SECTION_MACHINE: u32 = 2;
pub const GPO_SECTION_ROOT: u32 = 0;
pub const GPO_SECTION_USER: u32 = 1;
pub type GROUP_POLICY_HINT_TYPE = i32;
pub type GROUP_POLICY_OBJECT_TYPE = i32;
windows_core::imp::define_interface!(IGPEInformation, IGPEInformation_Vtbl, 0x8fc0b735_a0e1_11d1_a7d3_0000f87571e3);
windows_core::imp::interface_hierarchy!(IGPEInformation, windows_core::IUnknown);
impl IGPEInformation {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetName(&self, pszname: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()) }
    }
    pub unsafe fn GetDisplayName(&self, pszname: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()) }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn GetRegistryKey(&self, dwsection: u32) -> windows_core::Result<super::minwindef::HKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegistryKey)(windows_core::Interface::as_raw(self), dwsection, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDSPath)(windows_core::Interface::as_raw(self), dwsection, core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()) }
    }
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileSysPath)(windows_core::Interface::as_raw(self), dwsection, core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()) }
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
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub GetRegistryKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::minwindef::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    GetRegistryKey: usize,
    pub GetDSPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetFileSysPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::HRESULT,
    pub GetHint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GROUP_POLICY_HINT_TYPE) -> windows_core::HRESULT,
    pub PolicyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_minwindef")]
pub trait IGPEInformation_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetDisplayName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetRegistryKey(&self, dwsection: u32) -> windows_core::Result<super::minwindef::HKEY>;
    fn GetDSPath(&self, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetFileSysPath(&self, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetOptions(&self) -> windows_core::Result<u32>;
    fn GetType(&self) -> windows_core::Result<GROUP_POLICY_OBJECT_TYPE>;
    fn GetHint(&self) -> windows_core::Result<GROUP_POLICY_HINT_TYPE>;
    fn PolicyChanged(&self, bmachine: windows_core::BOOL, badd: windows_core::BOOL, pguidextension: *mut windows_core::GUID, pguidsnapin: *mut windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_minwindef")]
impl IGPEInformation_Vtbl {
    pub const fn new<Identity: IGPEInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetName<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetDisplayName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetRegistryKey<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, hkey: *mut super::minwindef::HKEY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetDSPath<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGPEInformation_Impl::GetDSPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
            }
        }
        unsafe extern "system" fn GetFileSysPath<Identity: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::HRESULT {
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
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
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
#[cfg(feature = "Win32_minwindef")]
impl windows_core::RuntimeName for IGPEInformation {}
windows_core::imp::define_interface!(IGroupPolicyObject, IGroupPolicyObject_Vtbl, 0xea502723_a23d_11d1_a7d3_0000f87571e3);
windows_core::imp::interface_hierarchy!(IGroupPolicyObject, windows_core::IUnknown);
impl IGroupPolicyObject {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn New<P0, P1>(&self, pszdomainname: P0, pszdisplayname: P1, dwflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).New)(windows_core::Interface::as_raw(self), pszdomainname.param().abi(), pszdisplayname.param().abi(), dwflags) }
    }
    pub unsafe fn OpenDSGPO<P0>(&self, pszpath: P0, dwflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenDSGPO)(windows_core::Interface::as_raw(self), pszpath.param().abi(), dwflags) }
    }
    pub unsafe fn OpenLocalMachineGPO(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OpenLocalMachineGPO)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn OpenRemoteMachineGPO<P0>(&self, pszcomputername: P0, dwflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenRemoteMachineGPO)(windows_core::Interface::as_raw(self), pszcomputername.param().abi(), dwflags) }
    }
    pub unsafe fn Save(&self, bmachine: bool, badd: bool, pguidextension: *mut windows_core::GUID, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), bmachine.into(), badd.into(), pguidextension as _, pguid as _) }
    }
    pub unsafe fn Delete(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetName(&self, pszname: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()) }
    }
    pub unsafe fn GetDisplayName(&self, pszname: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()) }
    }
    pub unsafe fn SetDisplayName<P0>(&self, pszname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), pszname.param().abi()) }
    }
    pub unsafe fn GetPath(&self, pszpath: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()) }
    }
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDSPath)(windows_core::Interface::as_raw(self), dwsection, core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()) }
    }
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileSysPath)(windows_core::Interface::as_raw(self), dwsection, core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()) }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn GetRegistryKey(&self, dwsection: u32) -> windows_core::Result<super::minwindef::HKEY> {
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
    pub unsafe fn GetMachineName(&self, pszname: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMachineName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()) }
    }
    #[cfg(feature = "Win32_prsht")]
    pub unsafe fn GetPropertySheetPages(&self, hpages: *mut *mut super::prsht::HPROPSHEETPAGE, upagecount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertySheetPages)(windows_core::Interface::as_raw(self), hpages as _, upagecount as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGroupPolicyObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub New: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub OpenDSGPO: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub OpenLocalMachineGPO: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OpenRemoteMachineGPO: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetDSPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetFileSysPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub GetRegistryKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::minwindef::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    GetRegistryKey: usize,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::HRESULT,
    pub GetMachineName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_prsht")]
    pub GetPropertySheetPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::prsht::HPROPSHEETPAGE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_prsht"))]
    GetPropertySheetPages: usize,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht"))]
pub trait IGroupPolicyObject_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn New(&self, pszdomainname: &windows_core::PCWSTR, pszdisplayname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn OpenDSGPO(&self, pszpath: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn OpenLocalMachineGPO(&self, dwflags: u32) -> windows_core::Result<()>;
    fn OpenRemoteMachineGPO(&self, pszcomputername: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn Save(&self, bmachine: windows_core::BOOL, badd: windows_core::BOOL, pguidextension: *mut windows_core::GUID, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn GetName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetDisplayName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn SetDisplayName(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPath(&self, pszpath: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetDSPath(&self, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetFileSysPath(&self, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetRegistryKey(&self, dwsection: u32) -> windows_core::Result<super::minwindef::HKEY>;
    fn GetOptions(&self) -> windows_core::Result<u32>;
    fn SetOptions(&self, dwoptions: u32, dwmask: u32) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<GROUP_POLICY_OBJECT_TYPE>;
    fn GetMachineName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetPropertySheetPages(&self, hpages: *mut *mut super::prsht::HPROPSHEETPAGE, upagecount: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht"))]
impl IGroupPolicyObject_Vtbl {
    pub const fn new<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::Release(this)
            }
        }
        unsafe extern "system" fn New<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdomainname: windows_core::PCWSTR, pszdisplayname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::New(this, core::mem::transmute(&pszdomainname), core::mem::transmute(&pszdisplayname), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OpenDSGPO<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::OpenDSGPO(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OpenLocalMachineGPO<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::OpenLocalMachineGPO(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn OpenRemoteMachineGPO<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcomputername: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::OpenRemoteMachineGPO(this, core::mem::transmute(&pszcomputername), core::mem::transmute_copy(&dwflags)).into()
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
        unsafe extern "system" fn GetName<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetDisplayName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::SetDisplayName(this, core::mem::transmute(&pszname)).into()
            }
        }
        unsafe extern "system" fn GetPath<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetPath(this, core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetDSPath<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetDSPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
            }
        }
        unsafe extern "system" fn GetFileSysPath<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetFileSysPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
            }
        }
        unsafe extern "system" fn GetRegistryKey<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, hkey: *mut super::minwindef::HKEY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetMachineName<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetMachineName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
            }
        }
        unsafe extern "system" fn GetPropertySheetPages<Identity: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpages: *mut *mut super::prsht::HPROPSHEETPAGE, upagecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGroupPolicyObject_Impl::GetPropertySheetPages(this, core::mem::transmute_copy(&hpages), core::mem::transmute_copy(&upagecount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
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
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht"))]
impl windows_core::RuntimeName for IGroupPolicyObject {}
windows_core::imp::define_interface!(IRSOPInformation, IRSOPInformation_Vtbl, 0x9a5a81b5_d9c7_49ef_9d11_ddf50968c48d);
windows_core::imp::interface_hierarchy!(IRSOPInformation, windows_core::IUnknown);
impl IRSOPInformation {
    pub unsafe fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInterface)(windows_core::Interface::as_raw(self), riid, ppvobj as _) }
    }
    pub unsafe fn AddRef(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRef)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Release(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).Release)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetNamespace(&self, dwsection: u32, pszname: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNamespace)(windows_core::Interface::as_raw(self), dwsection, core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()) }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEventLogEntryText<P0, P1, P2>(&self, pszeventsource: P0, pszeventlogname: P1, pszeventtime: P2, dweventid: u32) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventLogEntryText)(windows_core::Interface::as_raw(self), pszeventsource.param().abi(), pszeventlogname.param().abi(), pszeventtime.param().abi(), dweventid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRSOPInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetEventLogEntryText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IRSOPInformation_Impl: windows_core::IUnknownImpl {
    fn QueryInterface(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddRef(&self) -> u32;
    fn Release(&self) -> u32;
    fn GetNamespace(&self, dwsection: u32, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
    fn GetEventLogEntryText(&self, pszeventsource: &windows_core::PCWSTR, pszeventlogname: &windows_core::PCWSTR, pszeventtime: &windows_core::PCWSTR, dweventid: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl IRSOPInformation_Vtbl {
    pub const fn new<Identity: IRSOPInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<Identity: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRSOPInformation_Impl::QueryInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn AddRef<Identity: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRSOPInformation_Impl::AddRef(this)
            }
        }
        unsafe extern "system" fn Release<Identity: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRSOPInformation_Impl::Release(this)
            }
        }
        unsafe extern "system" fn GetNamespace<Identity: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetEventLogEntryText<Identity: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszeventsource: windows_core::PCWSTR, pszeventlogname: windows_core::PCWSTR, pszeventtime: windows_core::PCWSTR, dweventid: u32, ppsztext: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRSOPInformation_Impl::GetEventLogEntryText(this, core::mem::transmute(&pszeventsource), core::mem::transmute(&pszeventlogname), core::mem::transmute(&pszeventtime), core::mem::transmute_copy(&dweventid)) {
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
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
            GetNamespace: GetNamespace::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetEventLogEntryText: GetEventLogEntryText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRSOPInformation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRSOPInformation {}
#[cfg(feature = "Win32_windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGPOBROWSEINFO(pub *mut GPOBROWSEINFO);
#[cfg(feature = "Win32_windef")]
impl LPGPOBROWSEINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_windef")]
impl Default for LPGPOBROWSEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NODEID_Machine: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b737_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_MachineSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b73a_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_RSOPMachine: windows_core::GUID = windows_core::GUID::from_u128(0xbd4c1a2e_0b7a_4a62_a6b0_c0577539c97e);
pub const NODEID_RSOPMachineSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0x6a76273e_eb8e_45db_94c5_25663a5f2c1a);
pub const NODEID_RSOPUser: windows_core::GUID = windows_core::GUID::from_u128(0xab87364f_0cec_4cd8_9bf8_898f34628fb8);
pub const NODEID_RSOPUserSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0xe52c5ce3_fd27_4402_84de_d9a5f2858910);
pub const NODEID_User: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b738_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_UserSWSettings: windows_core::GUID = windows_core::GUID::from_u128(0x8fc0b73c_a0e1_11d1_a7d3_0000f87571e3);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGROUP_POLICY_HINT_TYPE(pub *mut GROUP_POLICY_HINT_TYPE);
impl PGROUP_POLICY_HINT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PGROUP_POLICY_HINT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGROUP_POLICY_OBJECT_TYPE(pub *mut GROUP_POLICY_OBJECT_TYPE);
impl PGROUP_POLICY_OBJECT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PGROUP_POLICY_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RSOP_INFO_FLAG_DIAGNOSTIC_MODE: u32 = 1;

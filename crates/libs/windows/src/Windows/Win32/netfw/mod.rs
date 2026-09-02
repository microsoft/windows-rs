#[inline]
pub unsafe fn NetworkIsolationDiagnoseConnectFailureAndGetInfo<P0>(wszservername: P0, netisoerror: *mut NETISO_ERROR_TYPE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername : windows_core::PCWSTR, netisoerror : *mut NETISO_ERROR_TYPE) -> u32);
    unsafe { NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername.param().abi(), netisoerror as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut PINET_FIREWALL_APP_CONTAINER) -> u32 {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationEnumAppContainers(flags : u32, pdwnumpublicappcs : *mut u32, pppublicappcs : *mut PINET_FIREWALL_APP_CONTAINER) -> u32);
    unsafe { NetworkIsolationEnumAppContainers(flags, pdwnumpublicappcs as _, pppublicappcs as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32 {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationFreeAppContainers(ppublicappcs : *const INET_FIREWALL_APP_CONTAINER) -> u32);
    unsafe { NetworkIsolationFreeAppContainers(ppublicappcs) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut super::PSID_AND_ATTRIBUTES) -> u32 {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs : *mut u32, appcontainersids : *mut super::PSID_AND_ATTRIBUTES) -> u32);
    unsafe { NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs as _, appcontainersids as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: PAC_CHANGES_CALLBACK_FN, context: Option<*const core::ffi::c_void>, registrationobject: *mut super::HANDLE) -> u32 {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationRegisterForAppContainerChanges(flags : u32, callback : PAC_CHANGES_CALLBACK_FN, context : *const core::ffi::c_void, registrationobject : *mut super::HANDLE) -> u32);
    unsafe { NetworkIsolationRegisterForAppContainerChanges(flags, callback, context.unwrap_or(core::mem::zeroed()) as _, registrationobject as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NetworkIsolationSetAppContainerConfig(appcontainersids: &[super::SID_AND_ATTRIBUTES]) -> u32 {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs : u32, appcontainersids : *const super::SID_AND_ATTRIBUTES) -> u32);
    unsafe { NetworkIsolationSetAppContainerConfig(appcontainersids.len().try_into().unwrap(), appcontainersids.as_ptr()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NetworkIsolationSetupAppContainerBinaries<P1, P2, P3>(applicationcontainersid: super::PSID, packagefullname: P1, packagefolder: P2, displayname: P3, bbinariesfullycomputed: bool, binaries: &[windows_core::PCWSTR]) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid : super::PSID, packagefullname : windows_core::PCWSTR, packagefolder : windows_core::PCWSTR, displayname : windows_core::PCWSTR, bbinariesfullycomputed : windows_core::BOOL, binaries : *const windows_core::PCWSTR, binariescount : u32) -> windows_core::HRESULT);
    unsafe { NetworkIsolationSetupAppContainerBinaries(applicationcontainersid, packagefullname.param().abi(), packagefolder.param().abi(), displayname.param().abi(), bbinariesfullycomputed.into(), binaries.as_ptr(), binaries.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject: super::HANDLE) -> u32 {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject : super::HANDLE) -> u32);
    unsafe { NetworkIsolationUnregisterForAppContainerChanges(registrationobject) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: windows_core::GUID,
    pub keyword: windows_core::PCWSTR,
    pub flags: u32,
    pub addresses: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut Self,
    pub schemaVersion: u16,
    pub originType: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
pub type FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = u32;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 3;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 1;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 2;
pub type FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = u32;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = 1;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 0;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 1;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 2;
pub type FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: u32,
    pub binaries: *mut windows_core::PWSTR,
}
pub const INET_FIREWALL_AC_BINARY: INET_FIREWALL_AC_CREATION_TYPE = 2;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: u32,
    pub capabilities: *mut super::SID_AND_ATTRIBUTES,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct INET_FIREWALL_AC_CHANGE {
    pub changeType: INET_FIREWALL_AC_CHANGE_TYPE,
    pub createType: INET_FIREWALL_AC_CREATION_TYPE,
    pub appContainerSid: *mut super::SID,
    pub userSid: *mut super::SID,
    pub displayName: windows_core::PWSTR,
    pub Anonymous: INET_FIREWALL_AC_CHANGE_0,
}
#[cfg(feature = "winnt")]
impl Default for INET_FIREWALL_AC_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union INET_FIREWALL_AC_CHANGE_0 {
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
}
#[cfg(feature = "winnt")]
impl Default for INET_FIREWALL_AC_CHANGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INET_FIREWALL_AC_CHANGE_CREATE: INET_FIREWALL_AC_CHANGE_TYPE = 1;
pub const INET_FIREWALL_AC_CHANGE_DELETE: INET_FIREWALL_AC_CHANGE_TYPE = 2;
pub const INET_FIREWALL_AC_CHANGE_INVALID: INET_FIREWALL_AC_CHANGE_TYPE = 0;
pub const INET_FIREWALL_AC_CHANGE_MAX: INET_FIREWALL_AC_CHANGE_TYPE = 3;
pub type INET_FIREWALL_AC_CHANGE_TYPE = i32;
pub type INET_FIREWALL_AC_CREATION_TYPE = i32;
pub const INET_FIREWALL_AC_MAX: INET_FIREWALL_AC_CREATION_TYPE = 4;
pub const INET_FIREWALL_AC_NONE: INET_FIREWALL_AC_CREATION_TYPE = 0;
pub const INET_FIREWALL_AC_PACKAGE_ID_ONLY: INET_FIREWALL_AC_CREATION_TYPE = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INET_FIREWALL_APP_CONTAINER {
    pub appContainerSid: *mut super::SID,
    pub userSid: *mut super::SID,
    pub appContainerName: windows_core::PWSTR,
    pub displayName: windows_core::PWSTR,
    pub description: windows_core::PWSTR,
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
    pub workingDirectory: windows_core::PWSTR,
    pub packageFullName: windows_core::PWSTR,
}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwAuthorizedApplication, INetFwAuthorizedApplication_Vtbl, 0xb5e64ffa_c2c5_444e_a301_fb5e00018050);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwAuthorizedApplication {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwAuthorizedApplication, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwAuthorizedApplication {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn ProcessImageFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProcessImageFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetProcessImageFileName(&self, imagefilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProcessImageFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename)) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetIpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIpVersion)(windows_core::Interface::as_raw(self), ipversion) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetScope(&self, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), scope) }
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplication_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessImageFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProcessImageFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    IpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub SetIpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetIpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Scope: usize,
    #[cfg(feature = "icftypes")]
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetScope: usize,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetEnabled: usize,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwAuthorizedApplication_Impl: super::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ProcessImageFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProcessImageFileName(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE>;
    fn SetScope(&self, scope: super::NET_FW_SCOPE) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwAuthorizedApplication_Vtbl {
    pub const fn new<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn ProcessImageFileName<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::ProcessImageFileName(this) {
                    Ok(ok__) => {
                        imagefilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProcessImageFileName<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::SetProcessImageFileName(this, core::mem::transmute(&imagefilename)).into()
            }
        }
        unsafe extern "system" fn IpVersion<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::IpVersion(this) {
                    Ok(ok__) => {
                        ipversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
            }
        }
        unsafe extern "system" fn Scope<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::Scope(this) {
                    Ok(ok__) => {
                        scope.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScope<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn RemoteAddresses<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            ProcessImageFileName: ProcessImageFileName::<Identity, OFFSET>,
            SetProcessImageFileName: SetProcessImageFileName::<Identity, OFFSET>,
            IpVersion: IpVersion::<Identity, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplication as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwAuthorizedApplication {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwAuthorizedApplications, INetFwAuthorizedApplications_Vtbl, 0x644efd52_ccf9_486c_97a2_39f352570b30);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwAuthorizedApplications {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwAuthorizedApplications, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwAuthorizedApplications {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0>(&self, app: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<INetFwAuthorizedApplication>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), app.param().abi()) }
    }
    pub unsafe fn Remove(&self, imagefilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename)) }
    }
    pub unsafe fn Item(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<INetFwAuthorizedApplication> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplications_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwAuthorizedApplications_Impl: super::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, app: windows_core::Ref<INetFwAuthorizedApplication>) -> windows_core::Result<()>;
    fn Remove(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Item(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<INetFwAuthorizedApplication>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwAuthorizedApplications_Vtbl {
    pub const fn new<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplications_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, app: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplications_Impl::Add(this, core::mem::transmute_copy(&app)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplications_Impl::Remove(this, core::mem::transmute(&imagefilename)).into()
            }
        }
        unsafe extern "system" fn Item<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void, app: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplications_Impl::Item(this, core::mem::transmute(&imagefilename)) {
                    Ok(ok__) => {
                        app.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplications_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplications as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwAuthorizedApplications {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwIcmpSettings, INetFwIcmpSettings_Vtbl, 0xa6207b2e_7cdd_426a_951e_5e1cbc5afead);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwIcmpSettings {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwIcmpSettings, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwIcmpSettings {
    #[cfg(feature = "wtypes")]
    pub unsafe fn AllowOutboundDestinationUnreachable(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowOutboundDestinationUnreachable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAllowOutboundDestinationUnreachable(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllowOutboundDestinationUnreachable)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AllowRedirect(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowRedirect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAllowRedirect(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllowRedirect)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AllowInboundEchoRequest(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowInboundEchoRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAllowInboundEchoRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllowInboundEchoRequest)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AllowOutboundTimeExceeded(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowOutboundTimeExceeded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAllowOutboundTimeExceeded(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllowOutboundTimeExceeded)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AllowOutboundParameterProblem(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowOutboundParameterProblem)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAllowOutboundParameterProblem(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllowOutboundParameterProblem)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AllowOutboundSourceQuench(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowOutboundSourceQuench)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAllowOutboundSourceQuench(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllowOutboundSourceQuench)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AllowInboundRouterRequest(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowInboundRouterRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAllowInboundRouterRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllowInboundRouterRequest)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AllowInboundTimestampRequest(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowInboundTimestampRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAllowInboundTimestampRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllowInboundTimestampRequest)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AllowInboundMaskRequest(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowInboundMaskRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAllowInboundMaskRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllowInboundMaskRequest)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AllowOutboundPacketTooBig(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowOutboundPacketTooBig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAllowOutboundPacketTooBig(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllowOutboundPacketTooBig)(windows_core::Interface::as_raw(self), allow) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwIcmpSettings_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub AllowOutboundDestinationUnreachable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AllowOutboundDestinationUnreachable: usize,
    #[cfg(feature = "wtypes")]
    pub SetAllowOutboundDestinationUnreachable: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAllowOutboundDestinationUnreachable: usize,
    #[cfg(feature = "wtypes")]
    pub AllowRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AllowRedirect: usize,
    #[cfg(feature = "wtypes")]
    pub SetAllowRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAllowRedirect: usize,
    #[cfg(feature = "wtypes")]
    pub AllowInboundEchoRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AllowInboundEchoRequest: usize,
    #[cfg(feature = "wtypes")]
    pub SetAllowInboundEchoRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAllowInboundEchoRequest: usize,
    #[cfg(feature = "wtypes")]
    pub AllowOutboundTimeExceeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AllowOutboundTimeExceeded: usize,
    #[cfg(feature = "wtypes")]
    pub SetAllowOutboundTimeExceeded: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAllowOutboundTimeExceeded: usize,
    #[cfg(feature = "wtypes")]
    pub AllowOutboundParameterProblem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AllowOutboundParameterProblem: usize,
    #[cfg(feature = "wtypes")]
    pub SetAllowOutboundParameterProblem: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAllowOutboundParameterProblem: usize,
    #[cfg(feature = "wtypes")]
    pub AllowOutboundSourceQuench: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AllowOutboundSourceQuench: usize,
    #[cfg(feature = "wtypes")]
    pub SetAllowOutboundSourceQuench: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAllowOutboundSourceQuench: usize,
    #[cfg(feature = "wtypes")]
    pub AllowInboundRouterRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AllowInboundRouterRequest: usize,
    #[cfg(feature = "wtypes")]
    pub SetAllowInboundRouterRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAllowInboundRouterRequest: usize,
    #[cfg(feature = "wtypes")]
    pub AllowInboundTimestampRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AllowInboundTimestampRequest: usize,
    #[cfg(feature = "wtypes")]
    pub SetAllowInboundTimestampRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAllowInboundTimestampRequest: usize,
    #[cfg(feature = "wtypes")]
    pub AllowInboundMaskRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AllowInboundMaskRequest: usize,
    #[cfg(feature = "wtypes")]
    pub SetAllowInboundMaskRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAllowInboundMaskRequest: usize,
    #[cfg(feature = "wtypes")]
    pub AllowOutboundPacketTooBig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AllowOutboundPacketTooBig: usize,
    #[cfg(feature = "wtypes")]
    pub SetAllowOutboundPacketTooBig: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAllowOutboundPacketTooBig: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwIcmpSettings_Impl: super::IDispatch_Impl {
    fn AllowOutboundDestinationUnreachable(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAllowOutboundDestinationUnreachable(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowRedirect(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAllowRedirect(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowInboundEchoRequest(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAllowInboundEchoRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowOutboundTimeExceeded(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAllowOutboundTimeExceeded(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowOutboundParameterProblem(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAllowOutboundParameterProblem(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowOutboundSourceQuench(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAllowOutboundSourceQuench(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowInboundRouterRequest(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAllowInboundRouterRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowInboundTimestampRequest(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAllowInboundTimestampRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowInboundMaskRequest(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAllowInboundMaskRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowOutboundPacketTooBig(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAllowOutboundPacketTooBig(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwIcmpSettings_Vtbl {
    pub const fn new<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllowOutboundDestinationUnreachable<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowOutboundDestinationUnreachable(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowOutboundDestinationUnreachable<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowOutboundDestinationUnreachable(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowRedirect<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowRedirect(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowRedirect<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowRedirect(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowInboundEchoRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowInboundEchoRequest(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowInboundEchoRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowInboundEchoRequest(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowOutboundTimeExceeded<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowOutboundTimeExceeded(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowOutboundTimeExceeded<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowOutboundTimeExceeded(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowOutboundParameterProblem<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowOutboundParameterProblem(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowOutboundParameterProblem<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowOutboundParameterProblem(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowOutboundSourceQuench<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowOutboundSourceQuench(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowOutboundSourceQuench<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowOutboundSourceQuench(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowInboundRouterRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowInboundRouterRequest(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowInboundRouterRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowInboundRouterRequest(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowInboundTimestampRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowInboundTimestampRequest(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowInboundTimestampRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowInboundTimestampRequest(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowInboundMaskRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowInboundMaskRequest(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowInboundMaskRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowInboundMaskRequest(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn AllowOutboundPacketTooBig<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::AllowOutboundPacketTooBig(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowOutboundPacketTooBig<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::SetAllowOutboundPacketTooBig(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AllowOutboundDestinationUnreachable: AllowOutboundDestinationUnreachable::<Identity, OFFSET>,
            SetAllowOutboundDestinationUnreachable: SetAllowOutboundDestinationUnreachable::<Identity, OFFSET>,
            AllowRedirect: AllowRedirect::<Identity, OFFSET>,
            SetAllowRedirect: SetAllowRedirect::<Identity, OFFSET>,
            AllowInboundEchoRequest: AllowInboundEchoRequest::<Identity, OFFSET>,
            SetAllowInboundEchoRequest: SetAllowInboundEchoRequest::<Identity, OFFSET>,
            AllowOutboundTimeExceeded: AllowOutboundTimeExceeded::<Identity, OFFSET>,
            SetAllowOutboundTimeExceeded: SetAllowOutboundTimeExceeded::<Identity, OFFSET>,
            AllowOutboundParameterProblem: AllowOutboundParameterProblem::<Identity, OFFSET>,
            SetAllowOutboundParameterProblem: SetAllowOutboundParameterProblem::<Identity, OFFSET>,
            AllowOutboundSourceQuench: AllowOutboundSourceQuench::<Identity, OFFSET>,
            SetAllowOutboundSourceQuench: SetAllowOutboundSourceQuench::<Identity, OFFSET>,
            AllowInboundRouterRequest: AllowInboundRouterRequest::<Identity, OFFSET>,
            SetAllowInboundRouterRequest: SetAllowInboundRouterRequest::<Identity, OFFSET>,
            AllowInboundTimestampRequest: AllowInboundTimestampRequest::<Identity, OFFSET>,
            SetAllowInboundTimestampRequest: SetAllowInboundTimestampRequest::<Identity, OFFSET>,
            AllowInboundMaskRequest: AllowInboundMaskRequest::<Identity, OFFSET>,
            SetAllowInboundMaskRequest: SetAllowInboundMaskRequest::<Identity, OFFSET>,
            AllowOutboundPacketTooBig: AllowOutboundPacketTooBig::<Identity, OFFSET>,
            SetAllowOutboundPacketTooBig: SetAllowOutboundPacketTooBig::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwIcmpSettings as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwIcmpSettings {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwMgr, INetFwMgr_Vtbl, 0xf7898af5_cac4_4632_a2ec_da06e5111af2);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwMgr {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwMgr, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwMgr {
    pub unsafe fn LocalPolicy(&self) -> windows_core::Result<INetFwPolicy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalPolicy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn CurrentProfileType(&self) -> windows_core::Result<super::NET_FW_PROFILE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentProfileType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RestoreDefaults(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreDefaults)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn IsPortAllowed(&self, imagefilename: &windows_core::BSTR, ipversion: super::NET_FW_IP_VERSION, portnumber: i32, localaddress: &windows_core::BSTR, ipprotocol: super::NET_FW_IP_PROTOCOL, allowed: *mut super::VARIANT, restricted: *mut super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsPortAllowed)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename), ipversion, portnumber, core::mem::transmute_copy(localaddress), ipprotocol, allowed, restricted) }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn IsIcmpTypeAllowed(&self, ipversion: super::NET_FW_IP_VERSION, localaddress: &windows_core::BSTR, r#type: u8, allowed: *mut super::VARIANT, restricted: *mut super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsIcmpTypeAllowed)(windows_core::Interface::as_raw(self), ipversion, core::mem::transmute_copy(localaddress), r#type, allowed, restricted) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwMgr_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub LocalPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub CurrentProfileType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_PROFILE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    CurrentProfileType: usize,
    pub RestoreDefaults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "icftypes", feature = "wtypes", feature = "wtypesbase"))]
    pub IsPortAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::NET_FW_IP_VERSION, i32, *mut core::ffi::c_void, super::NET_FW_IP_PROTOCOL, *mut super::VARIANT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes", feature = "wtypesbase")))]
    IsPortAllowed: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes", feature = "wtypesbase"))]
    pub IsIcmpTypeAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_IP_VERSION, *mut core::ffi::c_void, u8, *mut super::VARIANT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes", feature = "wtypesbase")))]
    IsIcmpTypeAllowed: usize,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwMgr_Impl: super::IDispatch_Impl {
    fn LocalPolicy(&self) -> windows_core::Result<INetFwPolicy>;
    fn CurrentProfileType(&self) -> windows_core::Result<super::NET_FW_PROFILE_TYPE>;
    fn RestoreDefaults(&self) -> windows_core::Result<()>;
    fn IsPortAllowed(&self, imagefilename: &windows_core::BSTR, ipversion: super::NET_FW_IP_VERSION, portnumber: i32, localaddress: &windows_core::BSTR, ipprotocol: super::NET_FW_IP_PROTOCOL, allowed: *mut super::VARIANT, restricted: *mut super::VARIANT) -> windows_core::Result<()>;
    fn IsIcmpTypeAllowed(&self, ipversion: super::NET_FW_IP_VERSION, localaddress: &windows_core::BSTR, r#type: u8, allowed: *mut super::VARIANT, restricted: *mut super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwMgr_Vtbl {
    pub const fn new<Identity: INetFwMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LocalPolicy<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpolicy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwMgr_Impl::LocalPolicy(this) {
                    Ok(ok__) => {
                        localpolicy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentProfileType<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: *mut super::NET_FW_PROFILE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwMgr_Impl::CurrentProfileType(this) {
                    Ok(ok__) => {
                        profiletype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestoreDefaults<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwMgr_Impl::RestoreDefaults(this).into()
            }
        }
        unsafe extern "system" fn IsPortAllowed<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void, ipversion: super::NET_FW_IP_VERSION, portnumber: i32, localaddress: *mut core::ffi::c_void, ipprotocol: super::NET_FW_IP_PROTOCOL, allowed: *mut super::VARIANT, restricted: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwMgr_Impl::IsPortAllowed(this, core::mem::transmute(&imagefilename), core::mem::transmute_copy(&ipversion), core::mem::transmute_copy(&portnumber), core::mem::transmute(&localaddress), core::mem::transmute_copy(&ipprotocol), core::mem::transmute_copy(&allowed), core::mem::transmute_copy(&restricted)).into()
            }
        }
        unsafe extern "system" fn IsIcmpTypeAllowed<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: super::NET_FW_IP_VERSION, localaddress: *mut core::ffi::c_void, r#type: u8, allowed: *mut super::VARIANT, restricted: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwMgr_Impl::IsIcmpTypeAllowed(this, core::mem::transmute_copy(&ipversion), core::mem::transmute(&localaddress), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&allowed), core::mem::transmute_copy(&restricted)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LocalPolicy: LocalPolicy::<Identity, OFFSET>,
            CurrentProfileType: CurrentProfileType::<Identity, OFFSET>,
            RestoreDefaults: RestoreDefaults::<Identity, OFFSET>,
            IsPortAllowed: IsPortAllowed::<Identity, OFFSET>,
            IsIcmpTypeAllowed: IsIcmpTypeAllowed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwMgr as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwMgr {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwOpenPort, INetFwOpenPort_Vtbl, 0xe0483ba0_47ff_4d9c_a6d6_7741d0b195f7);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwOpenPort {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwOpenPort, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwOpenPort {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetIpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIpVersion)(windows_core::Interface::as_raw(self), ipversion) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn Protocol(&self) -> windows_core::Result<super::NET_FW_IP_PROTOCOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetProtocol(&self, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProtocol)(windows_core::Interface::as_raw(self), ipprotocol) }
    }
    pub unsafe fn Port(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Port)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPort(&self, portnumber: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPort)(windows_core::Interface::as_raw(self), portnumber) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetScope(&self, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), scope) }
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn BuiltIn(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuiltIn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPort_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    IpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub SetIpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetIpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Protocol: usize,
    #[cfg(feature = "icftypes")]
    pub SetProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetProtocol: usize,
    pub Port: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Scope: usize,
    #[cfg(feature = "icftypes")]
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetScope: usize,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub BuiltIn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    BuiltIn: usize,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwOpenPort_Impl: super::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn Protocol(&self) -> windows_core::Result<super::NET_FW_IP_PROTOCOL>;
    fn SetProtocol(&self, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::Result<()>;
    fn Port(&self) -> windows_core::Result<i32>;
    fn SetPort(&self, portnumber: i32) -> windows_core::Result<()>;
    fn Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE>;
    fn SetScope(&self, scope: super::NET_FW_SCOPE) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BuiltIn(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwOpenPort_Vtbl {
    pub const fn new<Identity: INetFwOpenPort_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn IpVersion<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::IpVersion(this) {
                    Ok(ok__) => {
                        ipversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
            }
        }
        unsafe extern "system" fn Protocol<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipprotocol: *mut super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::Protocol(this) {
                    Ok(ok__) => {
                        ipprotocol.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProtocol<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetProtocol(this, core::mem::transmute_copy(&ipprotocol)).into()
            }
        }
        unsafe extern "system" fn Port<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::Port(this) {
                    Ok(ok__) => {
                        portnumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPort<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetPort(this, core::mem::transmute_copy(&portnumber)).into()
            }
        }
        unsafe extern "system" fn Scope<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::Scope(this) {
                    Ok(ok__) => {
                        scope.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScope<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn RemoteAddresses<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn BuiltIn<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, builtin: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::BuiltIn(this) {
                    Ok(ok__) => {
                        builtin.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            IpVersion: IpVersion::<Identity, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, OFFSET>,
            Protocol: Protocol::<Identity, OFFSET>,
            SetProtocol: SetProtocol::<Identity, OFFSET>,
            Port: Port::<Identity, OFFSET>,
            SetPort: SetPort::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            BuiltIn: BuiltIn::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwOpenPort as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwOpenPort {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwOpenPorts, INetFwOpenPorts_Vtbl, 0xc0e9d7fa_e07e_430a_b19a_090ce82d92e2);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwOpenPorts {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwOpenPorts, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwOpenPorts {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0>(&self, port: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<INetFwOpenPort>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), port.param().abi()) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn Remove(&self, portnumber: i32, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), portnumber, ipprotocol) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn Item(&self, portnumber: i32, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::Result<INetFwOpenPort> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), portnumber, ipprotocol, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPorts_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Remove: usize,
    #[cfg(feature = "icftypes")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::NET_FW_IP_PROTOCOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwOpenPorts_Impl: super::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, port: windows_core::Ref<INetFwOpenPort>) -> windows_core::Result<()>;
    fn Remove(&self, portnumber: i32, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::Result<()>;
    fn Item(&self, portnumber: i32, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::Result<INetFwOpenPort>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwOpenPorts_Vtbl {
    pub const fn new<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPorts_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, port: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPorts_Impl::Add(this, core::mem::transmute_copy(&port)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: i32, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPorts_Impl::Remove(this, core::mem::transmute_copy(&portnumber), core::mem::transmute_copy(&ipprotocol)).into()
            }
        }
        unsafe extern "system" fn Item<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: i32, ipprotocol: super::NET_FW_IP_PROTOCOL, openport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPorts_Impl::Item(this, core::mem::transmute_copy(&portnumber), core::mem::transmute_copy(&ipprotocol)) {
                    Ok(ok__) => {
                        openport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPorts_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwOpenPorts as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwOpenPorts {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwPolicy, INetFwPolicy_Vtbl, 0xd46d2478_9ac9_4008_9dc7_5563ce5536cc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwPolicy {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwPolicy, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwPolicy {
    pub unsafe fn CurrentProfile(&self) -> windows_core::Result<INetFwProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentProfile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn GetProfileByType(&self, profiletype: super::NET_FW_PROFILE_TYPE) -> windows_core::Result<INetFwProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProfileByType)(windows_core::Interface::as_raw(self), profiletype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub CurrentProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub GetProfileByType: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    GetProfileByType: usize,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwPolicy_Impl: super::IDispatch_Impl {
    fn CurrentProfile(&self) -> windows_core::Result<INetFwProfile>;
    fn GetProfileByType(&self, profiletype: super::NET_FW_PROFILE_TYPE) -> windows_core::Result<INetFwProfile>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwPolicy_Vtbl {
    pub const fn new<Identity: INetFwPolicy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentProfile<Identity: INetFwPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy_Impl::CurrentProfile(this) {
                    Ok(ok__) => {
                        profile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProfileByType<Identity: INetFwPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE, profile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy_Impl::GetProfileByType(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        profile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CurrentProfile: CurrentProfile::<Identity, OFFSET>,
            GetProfileByType: GetProfileByType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwPolicy as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwPolicy {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwPolicy2, INetFwPolicy2_Vtbl, 0x98325047_c671_4174_8d81_defcd3f03186);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwPolicy2 {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwPolicy2, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwPolicy2 {
    pub unsafe fn CurrentProfileTypes(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentProfileTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn FirewallEnabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FirewallEnabled)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn SetFirewallEnabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFirewallEnabled)(windows_core::Interface::as_raw(self), profiletype, enabled) }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ExcludedInterfaces(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExcludedInterfaces)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetExcludedInterfaces(&self, profiletype: super::NET_FW_PROFILE_TYPE2, interfaces: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExcludedInterfaces)(windows_core::Interface::as_raw(self), profiletype, core::mem::transmute_copy(interfaces)) }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn BlockAllInboundTraffic(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BlockAllInboundTraffic)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn SetBlockAllInboundTraffic(&self, profiletype: super::NET_FW_PROFILE_TYPE2, block: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBlockAllInboundTraffic)(windows_core::Interface::as_raw(self), profiletype, block) }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn NotificationsDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NotificationsDisabled)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn SetNotificationsDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationsDisabled)(windows_core::Interface::as_raw(self), profiletype, disabled) }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), profiletype, disabled) }
    }
    pub unsafe fn Rules(&self) -> windows_core::Result<INetFwRules> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Rules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ServiceRestriction(&self) -> windows_core::Result<INetFwServiceRestriction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceRestriction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn EnableRuleGroup(&self, profiletypesbitmask: i32, group: &windows_core::BSTR, enable: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableRuleGroup)(windows_core::Interface::as_raw(self), profiletypesbitmask, core::mem::transmute_copy(group), enable) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsRuleGroupEnabled(&self, profiletypesbitmask: i32, group: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRuleGroupEnabled)(windows_core::Interface::as_raw(self), profiletypesbitmask, core::mem::transmute_copy(group), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RestoreLocalFirewallDefaults(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreLocalFirewallDefaults)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn DefaultInboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::NET_FW_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultInboundAction)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetDefaultInboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2, action: super::NET_FW_ACTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultInboundAction)(windows_core::Interface::as_raw(self), profiletype, action) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn DefaultOutboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::NET_FW_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultOutboundAction)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetDefaultOutboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2, action: super::NET_FW_ACTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultOutboundAction)(windows_core::Interface::as_raw(self), profiletype, action) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsRuleGroupCurrentlyEnabled(&self, group: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRuleGroupCurrentlyEnabled)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(group), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn LocalPolicyModifyState(&self) -> windows_core::Result<super::NET_FW_MODIFY_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalPolicyModifyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy2_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub CurrentProfileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub FirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    FirewallEnabled: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub SetFirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    SetFirewallEnabled: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes", feature = "wtypesbase"))]
    pub ExcludedInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes", feature = "wtypesbase")))]
    ExcludedInterfaces: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes", feature = "wtypesbase"))]
    pub SetExcludedInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes", feature = "wtypesbase")))]
    SetExcludedInterfaces: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub BlockAllInboundTraffic: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    BlockAllInboundTraffic: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub SetBlockAllInboundTraffic: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    SetBlockAllInboundTraffic: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub NotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    NotificationsDisabled: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub SetNotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    SetNotificationsDisabled: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    UnicastResponsesToMulticastBroadcastDisabled: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub SetUnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    SetUnicastResponsesToMulticastBroadcastDisabled: usize,
    pub Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceRestriction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub EnableRuleGroup: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    EnableRuleGroup: usize,
    #[cfg(feature = "wtypes")]
    pub IsRuleGroupEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsRuleGroupEnabled: usize,
    pub RestoreLocalFirewallDefaults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub DefaultInboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::NET_FW_ACTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    DefaultInboundAction: usize,
    #[cfg(feature = "icftypes")]
    pub SetDefaultInboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::NET_FW_ACTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetDefaultInboundAction: usize,
    #[cfg(feature = "icftypes")]
    pub DefaultOutboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::NET_FW_ACTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    DefaultOutboundAction: usize,
    #[cfg(feature = "icftypes")]
    pub SetDefaultOutboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::NET_FW_ACTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetDefaultOutboundAction: usize,
    #[cfg(feature = "wtypes")]
    pub IsRuleGroupCurrentlyEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsRuleGroupCurrentlyEnabled: usize,
    #[cfg(feature = "icftypes")]
    pub LocalPolicyModifyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_MODIFY_STATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    LocalPolicyModifyState: usize,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwPolicy2_Impl: super::IDispatch_Impl {
    fn CurrentProfileTypes(&self) -> windows_core::Result<i32>;
    fn FirewallEnabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetFirewallEnabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ExcludedInterfaces(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT>;
    fn SetExcludedInterfaces(&self, profiletype: super::NET_FW_PROFILE_TYPE2, interfaces: &super::VARIANT) -> windows_core::Result<()>;
    fn BlockAllInboundTraffic(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetBlockAllInboundTraffic(&self, profiletype: super::NET_FW_PROFILE_TYPE2, block: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn NotificationsDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetNotificationsDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Rules(&self) -> windows_core::Result<INetFwRules>;
    fn ServiceRestriction(&self) -> windows_core::Result<INetFwServiceRestriction>;
    fn EnableRuleGroup(&self, profiletypesbitmask: i32, group: &windows_core::BSTR, enable: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsRuleGroupEnabled(&self, profiletypesbitmask: i32, group: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn RestoreLocalFirewallDefaults(&self) -> windows_core::Result<()>;
    fn DefaultInboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::NET_FW_ACTION>;
    fn SetDefaultInboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2, action: super::NET_FW_ACTION) -> windows_core::Result<()>;
    fn DefaultOutboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::NET_FW_ACTION>;
    fn SetDefaultOutboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2, action: super::NET_FW_ACTION) -> windows_core::Result<()>;
    fn IsRuleGroupCurrentlyEnabled(&self, group: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn LocalPolicyModifyState(&self) -> windows_core::Result<super::NET_FW_MODIFY_STATE>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwPolicy2_Vtbl {
    pub const fn new<Identity: INetFwPolicy2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentProfileTypes<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::CurrentProfileTypes(this) {
                    Ok(ok__) => {
                        profiletypesbitmask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FirewallEnabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::FirewallEnabled(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFirewallEnabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::SetFirewallEnabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn ExcludedInterfaces<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, interfaces: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::ExcludedInterfaces(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        interfaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExcludedInterfaces<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, interfaces: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::SetExcludedInterfaces(this, core::mem::transmute_copy(&profiletype), core::mem::transmute(&interfaces)).into()
            }
        }
        unsafe extern "system" fn BlockAllInboundTraffic<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, block: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::BlockAllInboundTraffic(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        block.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBlockAllInboundTraffic<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, block: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::SetBlockAllInboundTraffic(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&block)).into()
            }
        }
        unsafe extern "system" fn NotificationsDisabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::NotificationsDisabled(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        disabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotificationsDisabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::SetNotificationsDisabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&disabled)).into()
            }
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::UnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        disabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::SetUnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&disabled)).into()
            }
        }
        unsafe extern "system" fn Rules<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::Rules(this) {
                    Ok(ok__) => {
                        rules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceRestriction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicerestriction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::ServiceRestriction(this) {
                    Ok(ok__) => {
                        servicerestriction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableRuleGroup<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32, group: *mut core::ffi::c_void, enable: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::EnableRuleGroup(this, core::mem::transmute_copy(&profiletypesbitmask), core::mem::transmute(&group), core::mem::transmute_copy(&enable)).into()
            }
        }
        unsafe extern "system" fn IsRuleGroupEnabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32, group: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::IsRuleGroupEnabled(this, core::mem::transmute_copy(&profiletypesbitmask), core::mem::transmute(&group)) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestoreLocalFirewallDefaults<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::RestoreLocalFirewallDefaults(this).into()
            }
        }
        unsafe extern "system" fn DefaultInboundAction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, action: *mut super::NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::DefaultInboundAction(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        action.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultInboundAction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, action: super::NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::SetDefaultInboundAction(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn DefaultOutboundAction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, action: *mut super::NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::DefaultOutboundAction(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        action.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultOutboundAction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, action: super::NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::SetDefaultOutboundAction(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn IsRuleGroupCurrentlyEnabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::IsRuleGroupCurrentlyEnabled(this, core::mem::transmute(&group)) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocalPolicyModifyState<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modifystate: *mut super::NET_FW_MODIFY_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::LocalPolicyModifyState(this) {
                    Ok(ok__) => {
                        modifystate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CurrentProfileTypes: CurrentProfileTypes::<Identity, OFFSET>,
            FirewallEnabled: FirewallEnabled::<Identity, OFFSET>,
            SetFirewallEnabled: SetFirewallEnabled::<Identity, OFFSET>,
            ExcludedInterfaces: ExcludedInterfaces::<Identity, OFFSET>,
            SetExcludedInterfaces: SetExcludedInterfaces::<Identity, OFFSET>,
            BlockAllInboundTraffic: BlockAllInboundTraffic::<Identity, OFFSET>,
            SetBlockAllInboundTraffic: SetBlockAllInboundTraffic::<Identity, OFFSET>,
            NotificationsDisabled: NotificationsDisabled::<Identity, OFFSET>,
            SetNotificationsDisabled: SetNotificationsDisabled::<Identity, OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled: UnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled: SetUnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            Rules: Rules::<Identity, OFFSET>,
            ServiceRestriction: ServiceRestriction::<Identity, OFFSET>,
            EnableRuleGroup: EnableRuleGroup::<Identity, OFFSET>,
            IsRuleGroupEnabled: IsRuleGroupEnabled::<Identity, OFFSET>,
            RestoreLocalFirewallDefaults: RestoreLocalFirewallDefaults::<Identity, OFFSET>,
            DefaultInboundAction: DefaultInboundAction::<Identity, OFFSET>,
            SetDefaultInboundAction: SetDefaultInboundAction::<Identity, OFFSET>,
            DefaultOutboundAction: DefaultOutboundAction::<Identity, OFFSET>,
            SetDefaultOutboundAction: SetDefaultOutboundAction::<Identity, OFFSET>,
            IsRuleGroupCurrentlyEnabled: IsRuleGroupCurrentlyEnabled::<Identity, OFFSET>,
            LocalPolicyModifyState: LocalPolicyModifyState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwPolicy2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwPolicy2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwProduct, INetFwProduct_Vtbl, 0x71881699_18f4_458b_b892_3ffce5e07f75);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwProduct {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwProduct, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwProduct {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn RuleCategories(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RuleCategories)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetRuleCategories(&self, rulecategories: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRuleCategories)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(rulecategories)) }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(displayname)) }
    }
    pub unsafe fn PathToSignedProductExe(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PathToSignedProductExe)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProduct_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub RuleCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    RuleCategories: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetRuleCategories: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetRuleCategories: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PathToSignedProductExe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwProduct_Impl: super::IDispatch_Impl {
    fn RuleCategories(&self) -> windows_core::Result<super::VARIANT>;
    fn SetRuleCategories(&self, rulecategories: &super::VARIANT) -> windows_core::Result<()>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathToSignedProductExe(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwProduct_Vtbl {
    pub const fn new<Identity: INetFwProduct_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RuleCategories<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulecategories: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProduct_Impl::RuleCategories(this) {
                    Ok(ok__) => {
                        rulecategories.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRuleCategories<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulecategories: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProduct_Impl::SetRuleCategories(this, core::mem::transmute(&rulecategories)).into()
            }
        }
        unsafe extern "system" fn DisplayName<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProduct_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        displayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProduct_Impl::SetDisplayName(this, core::mem::transmute(&displayname)).into()
            }
        }
        unsafe extern "system" fn PathToSignedProductExe<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProduct_Impl::PathToSignedProductExe(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RuleCategories: RuleCategories::<Identity, OFFSET>,
            SetRuleCategories: SetRuleCategories::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            PathToSignedProductExe: PathToSignedProductExe::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwProduct as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwProduct {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwProducts, INetFwProducts_Vtbl, 0x39eb36e0_2097_40bd_8af2_63a13b525362);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwProducts {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwProducts, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwProducts {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Register<P0>(&self, product: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<INetFwProduct>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), product.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<INetFwProduct> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProducts_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwProducts_Impl: super::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Register(&self, product: windows_core::Ref<INetFwProduct>) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<INetFwProduct>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwProducts_Vtbl {
    pub const fn new<Identity: INetFwProducts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProducts_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Register<Identity: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, product: *mut core::ffi::c_void, registration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProducts_Impl::Register(this, core::mem::transmute_copy(&product)) {
                    Ok(ok__) => {
                        registration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, product: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProducts_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        product.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProducts_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Register: Register::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwProducts as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwProducts {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwProfile, INetFwProfile_Vtbl, 0x174a0dda_e9f9_449d_993b_21ab667ca456);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwProfile {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwProfile, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwProfile {
    #[cfg(feature = "icftypes")]
    pub unsafe fn Type(&self) -> windows_core::Result<super::NET_FW_PROFILE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn FirewallEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FirewallEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetFirewallEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFirewallEnabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ExceptionsNotAllowed(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExceptionsNotAllowed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetExceptionsNotAllowed(&self, notallowed: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExceptionsNotAllowed)(windows_core::Interface::as_raw(self), notallowed) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn NotificationsDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NotificationsDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetNotificationsDisabled(&self, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationsDisabled)(windows_core::Interface::as_raw(self), disabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn UnicastResponsesToMulticastBroadcastDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), disabled) }
    }
    pub unsafe fn RemoteAdminSettings(&self) -> windows_core::Result<INetFwRemoteAdminSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteAdminSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn IcmpSettings(&self) -> windows_core::Result<INetFwIcmpSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IcmpSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GloballyOpenPorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Services(&self) -> windows_core::Result<INetFwServices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Services)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AuthorizedApplications(&self) -> windows_core::Result<INetFwAuthorizedApplications> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AuthorizedApplications)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProfile_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "icftypes")]
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_PROFILE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Type: usize,
    #[cfg(feature = "wtypes")]
    pub FirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    FirewallEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetFirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetFirewallEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub ExceptionsNotAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ExceptionsNotAllowed: usize,
    #[cfg(feature = "wtypes")]
    pub SetExceptionsNotAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetExceptionsNotAllowed: usize,
    #[cfg(feature = "wtypes")]
    pub NotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    NotificationsDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetNotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetNotificationsDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    UnicastResponsesToMulticastBroadcastDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetUnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetUnicastResponsesToMulticastBroadcastDisabled: usize,
    pub RemoteAdminSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IcmpSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GloballyOpenPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Services: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AuthorizedApplications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwProfile_Impl: super::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<super::NET_FW_PROFILE_TYPE>;
    fn FirewallEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetFirewallEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ExceptionsNotAllowed(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetExceptionsNotAllowed(&self, notallowed: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn NotificationsDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetNotificationsDisabled(&self, disabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UnicastResponsesToMulticastBroadcastDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, disabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RemoteAdminSettings(&self) -> windows_core::Result<INetFwRemoteAdminSettings>;
    fn IcmpSettings(&self) -> windows_core::Result<INetFwIcmpSettings>;
    fn GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts>;
    fn Services(&self) -> windows_core::Result<INetFwServices>;
    fn AuthorizedApplications(&self) -> windows_core::Result<INetFwAuthorizedApplications>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwProfile_Vtbl {
    pub const fn new<Identity: INetFwProfile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut super::NET_FW_PROFILE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::Type(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FirewallEnabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::FirewallEnabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFirewallEnabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProfile_Impl::SetFirewallEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn ExceptionsNotAllowed<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notallowed: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::ExceptionsNotAllowed(this) {
                    Ok(ok__) => {
                        notallowed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExceptionsNotAllowed<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notallowed: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProfile_Impl::SetExceptionsNotAllowed(this, core::mem::transmute_copy(&notallowed)).into()
            }
        }
        unsafe extern "system" fn NotificationsDisabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::NotificationsDisabled(this) {
                    Ok(ok__) => {
                        disabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNotificationsDisabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProfile_Impl::SetNotificationsDisabled(this, core::mem::transmute_copy(&disabled)).into()
            }
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::UnicastResponsesToMulticastBroadcastDisabled(this) {
                    Ok(ok__) => {
                        disabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProfile_Impl::SetUnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&disabled)).into()
            }
        }
        unsafe extern "system" fn RemoteAdminSettings<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteadminsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::RemoteAdminSettings(this) {
                    Ok(ok__) => {
                        remoteadminsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IcmpSettings<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmpsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::IcmpSettings(this) {
                    Ok(ok__) => {
                        icmpsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GloballyOpenPorts<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, openports: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::GloballyOpenPorts(this) {
                    Ok(ok__) => {
                        openports.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Services<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, services: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::Services(this) {
                    Ok(ok__) => {
                        services.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AuthorizedApplications<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, apps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::AuthorizedApplications(this) {
                    Ok(ok__) => {
                        apps.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            FirewallEnabled: FirewallEnabled::<Identity, OFFSET>,
            SetFirewallEnabled: SetFirewallEnabled::<Identity, OFFSET>,
            ExceptionsNotAllowed: ExceptionsNotAllowed::<Identity, OFFSET>,
            SetExceptionsNotAllowed: SetExceptionsNotAllowed::<Identity, OFFSET>,
            NotificationsDisabled: NotificationsDisabled::<Identity, OFFSET>,
            SetNotificationsDisabled: SetNotificationsDisabled::<Identity, OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled: UnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled: SetUnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            RemoteAdminSettings: RemoteAdminSettings::<Identity, OFFSET>,
            IcmpSettings: IcmpSettings::<Identity, OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Identity, OFFSET>,
            Services: Services::<Identity, OFFSET>,
            AuthorizedApplications: AuthorizedApplications::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwProfile as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwProfile {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwRemoteAdminSettings, INetFwRemoteAdminSettings_Vtbl, 0xd4becddf_6f73_4a83_b832_9c66874cd20e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwRemoteAdminSettings {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwRemoteAdminSettings, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwRemoteAdminSettings {
    #[cfg(feature = "icftypes")]
    pub unsafe fn IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetIpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIpVersion)(windows_core::Interface::as_raw(self), ipversion) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetScope(&self, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), scope) }
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRemoteAdminSettings_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "icftypes")]
    pub IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    IpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub SetIpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetIpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Scope: usize,
    #[cfg(feature = "icftypes")]
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetScope: usize,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetEnabled: usize,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwRemoteAdminSettings_Impl: super::IDispatch_Impl {
    fn IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE>;
    fn SetScope(&self, scope: super::NET_FW_SCOPE) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwRemoteAdminSettings_Vtbl {
    pub const fn new<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IpVersion<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRemoteAdminSettings_Impl::IpVersion(this) {
                    Ok(ok__) => {
                        ipversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRemoteAdminSettings_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
            }
        }
        unsafe extern "system" fn Scope<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRemoteAdminSettings_Impl::Scope(this) {
                    Ok(ok__) => {
                        scope.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScope<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRemoteAdminSettings_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn RemoteAddresses<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRemoteAdminSettings_Impl::RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRemoteAdminSettings_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRemoteAdminSettings_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRemoteAdminSettings_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IpVersion: IpVersion::<Identity, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRemoteAdminSettings as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwRemoteAdminSettings {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwRule, INetFwRule_Vtbl, 0xaf230d27_baba_4e42_aced_f524f22cfce2);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwRule {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwRule, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwRule {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, desc: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(desc)) }
    }
    pub unsafe fn ApplicationName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApplicationName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetApplicationName(&self, imagefilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetApplicationName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename)) }
    }
    pub unsafe fn ServiceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetServiceName(&self, servicename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetServiceName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename)) }
    }
    pub unsafe fn Protocol(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProtocol(&self, protocol: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProtocol)(windows_core::Interface::as_raw(self), protocol) }
    }
    pub unsafe fn LocalPorts(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalPorts)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalPorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocalPorts)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(portnumbers)) }
    }
    pub unsafe fn RemotePorts(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemotePorts)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemotePorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRemotePorts)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(portnumbers)) }
    }
    pub unsafe fn LocalAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalAddresses(&self, localaddrs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocalAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(localaddrs)) }
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)) }
    }
    pub unsafe fn IcmpTypesAndCodes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IcmpTypesAndCodes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetIcmpTypesAndCodes(&self, icmptypesandcodes: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIcmpTypesAndCodes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(icmptypesandcodes)) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn Direction(&self) -> windows_core::Result<super::NET_FW_RULE_DIRECTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetDirection(&self, dir: super::NET_FW_RULE_DIRECTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDirection)(windows_core::Interface::as_raw(self), dir) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Interfaces(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Interfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetInterfaces(&self, interfaces: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInterfaces)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(interfaces)) }
    }
    pub unsafe fn InterfaceTypes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InterfaceTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetInterfaceTypes(&self, interfacetypes: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInterfaceTypes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(interfacetypes)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    pub unsafe fn Grouping(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Grouping)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetGrouping(&self, context: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGrouping)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(context)) }
    }
    pub unsafe fn Profiles(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Profiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProfiles)(windows_core::Interface::as_raw(self), profiletypesbitmask) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn EdgeTraversal(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EdgeTraversal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetEdgeTraversal(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEdgeTraversal)(windows_core::Interface::as_raw(self), enabled) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn Action(&self) -> windows_core::Result<super::NET_FW_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Action)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetAction(&self, action: super::NET_FW_ACTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAction)(windows_core::Interface::as_raw(self), action) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplicationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetApplicationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LocalPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotePorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemotePorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IcmpTypesAndCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIcmpTypesAndCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_RULE_DIRECTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Direction: usize,
    #[cfg(feature = "icftypes")]
    pub SetDirection: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_RULE_DIRECTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetDirection: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Interfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Interfaces: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetInterfaces: usize,
    pub InterfaceTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInterfaceTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetEnabled: usize,
    pub Grouping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGrouping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Profiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub EdgeTraversal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    EdgeTraversal: usize,
    #[cfg(feature = "wtypes")]
    pub SetEdgeTraversal: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetEdgeTraversal: usize,
    #[cfg(feature = "icftypes")]
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_ACTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Action: usize,
    #[cfg(feature = "icftypes")]
    pub SetAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_ACTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetAction: usize,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwRule_Impl: super::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, desc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ApplicationName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetApplicationName(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServiceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceName(&self, servicename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Protocol(&self) -> windows_core::Result<i32>;
    fn SetProtocol(&self, protocol: i32) -> windows_core::Result<()>;
    fn LocalPorts(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalPorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemotePorts(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemotePorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalAddresses(&self, localaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IcmpTypesAndCodes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetIcmpTypesAndCodes(&self, icmptypesandcodes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Direction(&self) -> windows_core::Result<super::NET_FW_RULE_DIRECTION>;
    fn SetDirection(&self, dir: super::NET_FW_RULE_DIRECTION) -> windows_core::Result<()>;
    fn Interfaces(&self) -> windows_core::Result<super::VARIANT>;
    fn SetInterfaces(&self, interfaces: &super::VARIANT) -> windows_core::Result<()>;
    fn InterfaceTypes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInterfaceTypes(&self, interfacetypes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Grouping(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetGrouping(&self, context: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Profiles(&self) -> windows_core::Result<i32>;
    fn SetProfiles(&self, profiletypesbitmask: i32) -> windows_core::Result<()>;
    fn EdgeTraversal(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetEdgeTraversal(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Action(&self) -> windows_core::Result<super::NET_FW_ACTION>;
    fn SetAction(&self, action: super::NET_FW_ACTION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwRule_Vtbl {
    pub const fn new<Identity: INetFwRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Description(this) {
                    Ok(ok__) => {
                        desc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetDescription(this, core::mem::transmute(&desc)).into()
            }
        }
        unsafe extern "system" fn ApplicationName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::ApplicationName(this) {
                    Ok(ok__) => {
                        imagefilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetApplicationName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetApplicationName(this, core::mem::transmute(&imagefilename)).into()
            }
        }
        unsafe extern "system" fn ServiceName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::ServiceName(this) {
                    Ok(ok__) => {
                        servicename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServiceName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetServiceName(this, core::mem::transmute(&servicename)).into()
            }
        }
        unsafe extern "system" fn Protocol<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protocol: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Protocol(this) {
                    Ok(ok__) => {
                        protocol.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProtocol<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protocol: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetProtocol(this, core::mem::transmute_copy(&protocol)).into()
            }
        }
        unsafe extern "system" fn LocalPorts<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::LocalPorts(this) {
                    Ok(ok__) => {
                        portnumbers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalPorts<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetLocalPorts(this, core::mem::transmute(&portnumbers)).into()
            }
        }
        unsafe extern "system" fn RemotePorts<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::RemotePorts(this) {
                    Ok(ok__) => {
                        portnumbers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemotePorts<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetRemotePorts(this, core::mem::transmute(&portnumbers)).into()
            }
        }
        unsafe extern "system" fn LocalAddresses<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::LocalAddresses(this) {
                    Ok(ok__) => {
                        localaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalAddresses<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetLocalAddresses(this, core::mem::transmute(&localaddrs)).into()
            }
        }
        unsafe extern "system" fn RemoteAddresses<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn IcmpTypesAndCodes<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmptypesandcodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::IcmpTypesAndCodes(this) {
                    Ok(ok__) => {
                        icmptypesandcodes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIcmpTypesAndCodes<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmptypesandcodes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetIcmpTypesAndCodes(this, core::mem::transmute(&icmptypesandcodes)).into()
            }
        }
        unsafe extern "system" fn Direction<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dir: *mut super::NET_FW_RULE_DIRECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Direction(this) {
                    Ok(ok__) => {
                        dir.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDirection<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dir: super::NET_FW_RULE_DIRECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetDirection(this, core::mem::transmute_copy(&dir)).into()
            }
        }
        unsafe extern "system" fn Interfaces<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaces: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Interfaces(this) {
                    Ok(ok__) => {
                        interfaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInterfaces<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaces: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetInterfaces(this, core::mem::transmute(&interfaces)).into()
            }
        }
        unsafe extern "system" fn InterfaceTypes<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacetypes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::InterfaceTypes(this) {
                    Ok(ok__) => {
                        interfacetypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInterfaceTypes<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacetypes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetInterfaceTypes(this, core::mem::transmute(&interfacetypes)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn Grouping<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Grouping(this) {
                    Ok(ok__) => {
                        context.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGrouping<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetGrouping(this, core::mem::transmute(&context)).into()
            }
        }
        unsafe extern "system" fn Profiles<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Profiles(this) {
                    Ok(ok__) => {
                        profiletypesbitmask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProfiles<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetProfiles(this, core::mem::transmute_copy(&profiletypesbitmask)).into()
            }
        }
        unsafe extern "system" fn EdgeTraversal<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::EdgeTraversal(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEdgeTraversal<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetEdgeTraversal(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn Action<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: *mut super::NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::Action(this) {
                    Ok(ok__) => {
                        action.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAction<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: super::NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::SetAction(this, core::mem::transmute_copy(&action)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            ApplicationName: ApplicationName::<Identity, OFFSET>,
            SetApplicationName: SetApplicationName::<Identity, OFFSET>,
            ServiceName: ServiceName::<Identity, OFFSET>,
            SetServiceName: SetServiceName::<Identity, OFFSET>,
            Protocol: Protocol::<Identity, OFFSET>,
            SetProtocol: SetProtocol::<Identity, OFFSET>,
            LocalPorts: LocalPorts::<Identity, OFFSET>,
            SetLocalPorts: SetLocalPorts::<Identity, OFFSET>,
            RemotePorts: RemotePorts::<Identity, OFFSET>,
            SetRemotePorts: SetRemotePorts::<Identity, OFFSET>,
            LocalAddresses: LocalAddresses::<Identity, OFFSET>,
            SetLocalAddresses: SetLocalAddresses::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            IcmpTypesAndCodes: IcmpTypesAndCodes::<Identity, OFFSET>,
            SetIcmpTypesAndCodes: SetIcmpTypesAndCodes::<Identity, OFFSET>,
            Direction: Direction::<Identity, OFFSET>,
            SetDirection: SetDirection::<Identity, OFFSET>,
            Interfaces: Interfaces::<Identity, OFFSET>,
            SetInterfaces: SetInterfaces::<Identity, OFFSET>,
            InterfaceTypes: InterfaceTypes::<Identity, OFFSET>,
            SetInterfaceTypes: SetInterfaceTypes::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Grouping: Grouping::<Identity, OFFSET>,
            SetGrouping: SetGrouping::<Identity, OFFSET>,
            Profiles: Profiles::<Identity, OFFSET>,
            SetProfiles: SetProfiles::<Identity, OFFSET>,
            EdgeTraversal: EdgeTraversal::<Identity, OFFSET>,
            SetEdgeTraversal: SetEdgeTraversal::<Identity, OFFSET>,
            Action: Action::<Identity, OFFSET>,
            SetAction: SetAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRule as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwRule {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwRule2, INetFwRule2_Vtbl, 0x9c27c8da_189b_4dde_89f7_8b39a316782c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwRule2 {
    type Target = INetFwRule;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwRule2, windows_core::IUnknown, super::IDispatch, INetFwRule);
#[cfg(feature = "oaidl")]
impl INetFwRule2 {
    pub unsafe fn EdgeTraversalOptions(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EdgeTraversalOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEdgeTraversalOptions)(windows_core::Interface::as_raw(self), loptions) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule2_Vtbl {
    pub base__: INetFwRule_Vtbl,
    pub EdgeTraversalOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEdgeTraversalOptions: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwRule2_Impl: INetFwRule_Impl {
    fn EdgeTraversalOptions(&self) -> windows_core::Result<i32>;
    fn SetEdgeTraversalOptions(&self, loptions: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwRule2_Vtbl {
    pub const fn new<Identity: INetFwRule2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EdgeTraversalOptions<Identity: INetFwRule2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule2_Impl::EdgeTraversalOptions(this) {
                    Ok(ok__) => {
                        loptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEdgeTraversalOptions<Identity: INetFwRule2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule2_Impl::SetEdgeTraversalOptions(this, core::mem::transmute_copy(&loptions)).into()
            }
        }
        Self {
            base__: INetFwRule_Vtbl::new::<Identity, OFFSET>(),
            EdgeTraversalOptions: EdgeTraversalOptions::<Identity, OFFSET>,
            SetEdgeTraversalOptions: SetEdgeTraversalOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRule2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<INetFwRule as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwRule2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwRule3, INetFwRule3_Vtbl, 0xb21563ff_d696_4222_ab46_4e89b73ab34a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwRule3 {
    type Target = INetFwRule2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwRule3, windows_core::IUnknown, super::IDispatch, INetFwRule, INetFwRule2);
#[cfg(feature = "oaidl")]
impl INetFwRule3 {
    pub unsafe fn LocalAppPackageId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalAppPackageId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalAppPackageId(&self, wszpackageid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocalAppPackageId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszpackageid)) }
    }
    pub unsafe fn LocalUserOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalUserOwner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalUserOwner(&self, wszuserowner: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocalUserOwner)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszuserowner)) }
    }
    pub unsafe fn LocalUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalUserAuthorizedList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLocalUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLocalUserAuthorizedList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszuserauthlist)) }
    }
    pub unsafe fn RemoteUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteUserAuthorizedList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteUserAuthorizedList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszuserauthlist)) }
    }
    pub unsafe fn RemoteMachineAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteMachineAuthorizedList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteMachineAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteMachineAuthorizedList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszuserauthlist)) }
    }
    pub unsafe fn SecureFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SecureFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSecureFlags(&self, loptions: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecureFlags)(windows_core::Interface::as_raw(self), loptions) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule3_Vtbl {
    pub base__: INetFwRule2_Vtbl,
    pub LocalAppPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalAppPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalUserOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalUserOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteMachineAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteMachineAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SecureFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSecureFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwRule3_Impl: INetFwRule2_Impl {
    fn LocalAppPackageId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalAppPackageId(&self, wszpackageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalUserOwner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalUserOwner(&self, wszuserowner: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoteUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoteMachineAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteMachineAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SecureFlags(&self) -> windows_core::Result<i32>;
    fn SetSecureFlags(&self, loptions: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwRule3_Vtbl {
    pub const fn new<Identity: INetFwRule3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LocalAppPackageId<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpackageid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::LocalAppPackageId(this) {
                    Ok(ok__) => {
                        wszpackageid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalAppPackageId<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpackageid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::SetLocalAppPackageId(this, core::mem::transmute(&wszpackageid)).into()
            }
        }
        unsafe extern "system" fn LocalUserOwner<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserowner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::LocalUserOwner(this) {
                    Ok(ok__) => {
                        wszuserowner.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalUserOwner<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserowner: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::SetLocalUserOwner(this, core::mem::transmute(&wszuserowner)).into()
            }
        }
        unsafe extern "system" fn LocalUserAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::LocalUserAuthorizedList(this) {
                    Ok(ok__) => {
                        wszuserauthlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalUserAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::SetLocalUserAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
            }
        }
        unsafe extern "system" fn RemoteUserAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::RemoteUserAuthorizedList(this) {
                    Ok(ok__) => {
                        wszuserauthlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteUserAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::SetRemoteUserAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
            }
        }
        unsafe extern "system" fn RemoteMachineAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::RemoteMachineAuthorizedList(this) {
                    Ok(ok__) => {
                        wszuserauthlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteMachineAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::SetRemoteMachineAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
            }
        }
        unsafe extern "system" fn SecureFlags<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::SecureFlags(this) {
                    Ok(ok__) => {
                        loptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecureFlags<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::SetSecureFlags(this, core::mem::transmute_copy(&loptions)).into()
            }
        }
        Self {
            base__: INetFwRule2_Vtbl::new::<Identity, OFFSET>(),
            LocalAppPackageId: LocalAppPackageId::<Identity, OFFSET>,
            SetLocalAppPackageId: SetLocalAppPackageId::<Identity, OFFSET>,
            LocalUserOwner: LocalUserOwner::<Identity, OFFSET>,
            SetLocalUserOwner: SetLocalUserOwner::<Identity, OFFSET>,
            LocalUserAuthorizedList: LocalUserAuthorizedList::<Identity, OFFSET>,
            SetLocalUserAuthorizedList: SetLocalUserAuthorizedList::<Identity, OFFSET>,
            RemoteUserAuthorizedList: RemoteUserAuthorizedList::<Identity, OFFSET>,
            SetRemoteUserAuthorizedList: SetRemoteUserAuthorizedList::<Identity, OFFSET>,
            RemoteMachineAuthorizedList: RemoteMachineAuthorizedList::<Identity, OFFSET>,
            SetRemoteMachineAuthorizedList: SetRemoteMachineAuthorizedList::<Identity, OFFSET>,
            SecureFlags: SecureFlags::<Identity, OFFSET>,
            SetSecureFlags: SetSecureFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRule3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<INetFwRule as windows_core::Interface>::IID || iid == &<INetFwRule2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwRule3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwRules, INetFwRules_Vtbl, 0x9c4c6277_5027_441e_afae_ca1f542da009);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwRules {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwRules, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwRules {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0>(&self, rule: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<INetFwRule>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), rule.param().abi()) }
    }
    pub unsafe fn Remove(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn Item(&self, name: &windows_core::BSTR) -> windows_core::Result<INetFwRule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRules_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwRules_Impl: super::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, rule: windows_core::Ref<INetFwRule>) -> windows_core::Result<()>;
    fn Remove(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Item(&self, name: &windows_core::BSTR) -> windows_core::Result<INetFwRule>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwRules_Vtbl {
    pub const fn new<Identity: INetFwRules_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRules_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rule: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRules_Impl::Add(this, core::mem::transmute_copy(&rule)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRules_Impl::Remove(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Item<Identity: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRules_Impl::Item(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        rule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRules_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRules as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwRules {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwService, INetFwService_Vtbl, 0x79fd57c8_908e_4a36_9888_d5b3f0a444cf);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwService {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwService, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwService {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn Type(&self) -> windows_core::Result<super::NET_FW_SERVICE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Customized(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Customized)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetIpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIpVersion)(windows_core::Interface::as_raw(self), ipversion) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn SetScope(&self, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), scope) }
    }
    pub unsafe fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    pub unsafe fn GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GloballyOpenPorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwService_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_SERVICE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Type: usize,
    #[cfg(feature = "wtypes")]
    pub Customized: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Customized: usize,
    #[cfg(feature = "icftypes")]
    pub IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    IpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub SetIpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetIpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Scope: usize,
    #[cfg(feature = "icftypes")]
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    SetScope: usize,
    pub RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetEnabled: usize,
    pub GloballyOpenPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwService_Impl: super::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<super::NET_FW_SERVICE_TYPE>;
    fn Customized(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE>;
    fn SetScope(&self, scope: super::NET_FW_SCOPE) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwService_Vtbl {
    pub const fn new<Identity: INetFwService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Type<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut super::NET_FW_SERVICE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::Type(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Customized<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customized: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::Customized(this) {
                    Ok(ok__) => {
                        customized.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IpVersion<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::IpVersion(this) {
                    Ok(ok__) => {
                        ipversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwService_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
            }
        }
        unsafe extern "system" fn Scope<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::Scope(this) {
                    Ok(ok__) => {
                        scope.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScope<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwService_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn RemoteAddresses<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwService_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwService_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn GloballyOpenPorts<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, openports: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::GloballyOpenPorts(this) {
                    Ok(ok__) => {
                        openports.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Customized: Customized::<Identity, OFFSET>,
            IpVersion: IpVersion::<Identity, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwService as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwService {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwServiceRestriction, INetFwServiceRestriction_Vtbl, 0x8267bbe3_f890_491c_b7b6_2db1ef0e5d2b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwServiceRestriction {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwServiceRestriction, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwServiceRestriction {
    #[cfg(feature = "wtypes")]
    pub unsafe fn RestrictService(&self, servicename: &windows_core::BSTR, appname: &windows_core::BSTR, restrictservice: super::VARIANT_BOOL, servicesidrestricted: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestrictService)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename), core::mem::transmute_copy(appname), restrictservice, servicesidrestricted) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ServiceRestricted(&self, servicename: &windows_core::BSTR, appname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceRestricted)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename), core::mem::transmute_copy(appname), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Rules(&self) -> windows_core::Result<INetFwRules> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Rules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServiceRestriction_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub RestrictService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RestrictService: usize,
    #[cfg(feature = "wtypes")]
    pub ServiceRestricted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ServiceRestricted: usize,
    pub Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwServiceRestriction_Impl: super::IDispatch_Impl {
    fn RestrictService(&self, servicename: &windows_core::BSTR, appname: &windows_core::BSTR, restrictservice: super::VARIANT_BOOL, servicesidrestricted: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ServiceRestricted(&self, servicename: &windows_core::BSTR, appname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn Rules(&self) -> windows_core::Result<INetFwRules>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwServiceRestriction_Vtbl {
    pub const fn new<Identity: INetFwServiceRestriction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RestrictService<Identity: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void, appname: *mut core::ffi::c_void, restrictservice: super::VARIANT_BOOL, servicesidrestricted: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwServiceRestriction_Impl::RestrictService(this, core::mem::transmute(&servicename), core::mem::transmute(&appname), core::mem::transmute_copy(&restrictservice), core::mem::transmute_copy(&servicesidrestricted)).into()
            }
        }
        unsafe extern "system" fn ServiceRestricted<Identity: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void, appname: *mut core::ffi::c_void, servicerestricted: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServiceRestriction_Impl::ServiceRestricted(this, core::mem::transmute(&servicename), core::mem::transmute(&appname)) {
                    Ok(ok__) => {
                        servicerestricted.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Rules<Identity: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServiceRestriction_Impl::Rules(this) {
                    Ok(ok__) => {
                        rules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RestrictService: RestrictService::<Identity, OFFSET>,
            ServiceRestricted: ServiceRestricted::<Identity, OFFSET>,
            Rules: Rules::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwServiceRestriction as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwServiceRestriction {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetFwServices, INetFwServices_Vtbl, 0x79649bb4_903e_421b_94c9_79848e79f6ee);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetFwServices {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetFwServices, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetFwServices {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn Item(&self, svctype: super::NET_FW_SERVICE_TYPE) -> windows_core::Result<INetFwService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), svctype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServices_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_SERVICE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwServices_Impl: super::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, svctype: super::NET_FW_SERVICE_TYPE) -> windows_core::Result<INetFwService>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwServices_Vtbl {
    pub const fn new<Identity: INetFwServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: INetFwServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServices_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: INetFwServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, svctype: super::NET_FW_SERVICE_TYPE, service: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServices_Impl::Item(this, core::mem::transmute_copy(&svctype)) {
                    Ok(ok__) => {
                        service.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: INetFwServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServices_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwServices as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "icftypes", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetFwServices {}
pub type NETISO_ERROR_TYPE = i32;
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT: NETISO_ERROR_TYPE = 2;
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER: NETISO_ERROR_TYPE = 3;
pub const NETISO_ERROR_TYPE_MAX: NETISO_ERROR_TYPE = 4;
pub const NETISO_ERROR_TYPE_NONE: NETISO_ERROR_TYPE = 0;
pub const NETISO_ERROR_TYPE_PRIVATE_NETWORK: NETISO_ERROR_TYPE = 1;
pub type NETISO_FLAG = i32;
pub const NETISO_FLAG_FORCE_COMPUTE_BINARIES: NETISO_FLAG = 1;
pub const NETISO_FLAG_MAX: NETISO_FLAG = 2;
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: i32 = 2;
pub const NETISO_GEID_FOR_WDAG: i32 = 1;
pub const NetFwAuthorizedApplication: windows_core::GUID = windows_core::GUID::from_u128(0xec9846b3_2762_4a6b_a214_6acb603462d2);
pub const NetFwMgr: windows_core::GUID = windows_core::GUID::from_u128(0x304ce942_6e39_40d8_943a_b913c40c9cd4);
pub const NetFwOpenPort: windows_core::GUID = windows_core::GUID::from_u128(0x0ca545c6_37ad_4a6c_bf92_9f7610067ef5);
pub const NetFwPolicy2: windows_core::GUID = windows_core::GUID::from_u128(0xe2b3c97f_6ae1_41ac_817a_f6f92166d7dd);
pub const NetFwProduct: windows_core::GUID = windows_core::GUID::from_u128(0x9d745ed8_c514_4d1d_bf42_751fed2d5ac7);
pub const NetFwProducts: windows_core::GUID = windows_core::GUID::from_u128(0xcc19079b_8272_4d73_bb70_cdb533527b61);
pub const NetFwRule: windows_core::GUID = windows_core::GUID::from_u128(0x2c5bc43e_3369_4c33_ab0c_be9469677af4);
#[cfg(feature = "winnt")]
pub type PAC_CHANGES_CALLBACK_FN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE)>;
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddress: *const FW_DYNAMIC_KEYWORD_ADDRESS0) -> u32>;
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_core::GUID) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_core::GUID, dynamickeywordaddressdata: *mut PFW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = Option<unsafe extern "system" fn(flags: u32, dynamickeywordaddressdata: *mut PFW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = Option<unsafe extern "system" fn(dynamickeywordaddressdata: *const FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_core::GUID, updatedaddresses: windows_core::PCWSTR, append: windows_core::BOOL) -> u32>;
pub type PFW_DYNAMIC_KEYWORD_ADDRESS0 = *mut FW_DYNAMIC_KEYWORD_ADDRESS0;
pub type PFW_DYNAMIC_KEYWORD_ADDRESS_DATA0 = *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0;
pub type PINET_FIREWALL_AC_BINARIES = *mut INET_FIREWALL_AC_BINARIES;
#[cfg(feature = "winnt")]
pub type PINET_FIREWALL_AC_CAPABILITIES = *mut INET_FIREWALL_AC_CAPABILITIES;
#[cfg(feature = "winnt")]
pub type PINET_FIREWALL_AC_CHANGE = *mut INET_FIREWALL_AC_CHANGE;
#[cfg(feature = "winnt")]
pub type PINET_FIREWALL_APP_CONTAINER = *mut INET_FIREWALL_APP_CONTAINER;
pub type PNETISO_EDP_ID_CALLBACK_FN = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, wszenterpriseid: windows_core::PCWSTR, dwerr: u32)>;

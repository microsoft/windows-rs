#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetworkIsolationDiagnoseConnectFailureAndGetInfo<P0>(wszservername: P0, netisoerror: *mut NETISO_ERROR_TYPE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername : windows_core::PCWSTR, netisoerror : *mut NETISO_ERROR_TYPE) -> super::DWORD);
    unsafe { NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername.param().abi(), netisoerror as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NetworkIsolationEnumAppContainers(flags: super::DWORD, pdwnumpublicappcs: *mut super::DWORD, pppublicappcs: *mut PINET_FIREWALL_APP_CONTAINER) -> super::DWORD {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationEnumAppContainers(flags : super::DWORD, pdwnumpublicappcs : *mut super::DWORD, pppublicappcs : *mut PINET_FIREWALL_APP_CONTAINER) -> super::DWORD);
    unsafe { NetworkIsolationEnumAppContainers(flags, pdwnumpublicappcs as _, pppublicappcs as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NetworkIsolationFreeAppContainers(ppublicappcs: PINET_FIREWALL_APP_CONTAINER) -> super::DWORD {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationFreeAppContainers(ppublicappcs : PINET_FIREWALL_APP_CONTAINER) -> super::DWORD);
    unsafe { NetworkIsolationFreeAppContainers(ppublicappcs) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut super::DWORD, appcontainersids: *mut super::PSID_AND_ATTRIBUTES) -> super::DWORD {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs : *mut super::DWORD, appcontainersids : *mut super::PSID_AND_ATTRIBUTES) -> super::DWORD);
    unsafe { NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs as _, appcontainersids as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NetworkIsolationRegisterForAppContainerChanges(flags: super::DWORD, callback: PAC_CHANGES_CALLBACK_FN, context: Option<super::PVOID>, registrationobject: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationRegisterForAppContainerChanges(flags : super::DWORD, callback : PAC_CHANGES_CALLBACK_FN, context : super::PVOID, registrationobject : *mut super::HANDLE) -> super::DWORD);
    unsafe { NetworkIsolationRegisterForAppContainerChanges(flags, callback, context.unwrap_or(core::mem::zeroed()) as _, registrationobject as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NetworkIsolationSetAppContainerConfig(appcontainersids: &[super::SID_AND_ATTRIBUTES]) -> super::DWORD {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs : super::DWORD, appcontainersids : super::PSID_AND_ATTRIBUTES) -> super::DWORD);
    unsafe { NetworkIsolationSetAppContainerConfig(appcontainersids.len().try_into().unwrap(), core::mem::transmute(appcontainersids.as_ptr())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NetworkIsolationSetupAppContainerBinaries<P1, P2, P3>(applicationcontainersid: super::PSID, packagefullname: P1, packagefolder: P2, displayname: P3, bbinariesfullycomputed: bool, binaries: &[windows_core::PCWSTR]) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid : super::PSID, packagefullname : windows_core::PCWSTR, packagefolder : windows_core::PCWSTR, displayname : windows_core::PCWSTR, bbinariesfullycomputed : windows_core::BOOL, binaries : *const windows_core::PCWSTR, binariescount : super::DWORD) -> windows_core::HRESULT);
    unsafe { NetworkIsolationSetupAppContainerBinaries(applicationcontainersid, packagefullname.param().abi(), packagefolder.param().abi(), displayname.param().abi(), bbinariesfullycomputed.into(), binaries.as_ptr(), binaries.len().try_into().unwrap()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject: super::HANDLE) -> super::DWORD {
    windows_core::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject : super::HANDLE) -> super::DWORD);
    unsafe { NetworkIsolationUnregisterForAppContainerChanges(registrationobject) }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: windows_core::GUID,
    pub keyword: windows_core::PCWSTR,
    pub flags: super::DWORD,
    pub addresses: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut Self,
    pub schemaVersion: super::WORD,
    pub originType: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
pub type FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = i32;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 3;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 1;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 2;
pub type FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = i32;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = 1;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 0;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 1;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 2;
pub type FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: super::DWORD,
    pub binaries: *mut windows_core::PWSTR,
}
pub const INET_FIREWALL_AC_BINARY: INET_FIREWALL_AC_CREATION_TYPE = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: super::DWORD,
    pub capabilities: *mut super::SID_AND_ATTRIBUTES,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct INET_FIREWALL_AC_CHANGE {
    pub changeType: INET_FIREWALL_AC_CHANGE_TYPE,
    pub createType: INET_FIREWALL_AC_CREATION_TYPE,
    pub appContainerSid: *mut super::SID,
    pub userSid: *mut super::SID,
    pub displayName: windows_core::PWSTR,
    pub Anonymous: INET_FIREWALL_AC_CHANGE_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for INET_FIREWALL_AC_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union INET_FIREWALL_AC_CHANGE_0 {
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
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
#[cfg(all(feature = "shared", feature = "winnt"))]
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
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn get_ProcessImageFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProcessImageFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ProcessImageFileName(&self, imagefilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ProcessImageFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename)) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_IpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IpVersion)(windows_core::Interface::as_raw(self), ipversion) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_Scope(&self, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Scope)(windows_core::Interface::as_raw(self), scope) }
    }
    pub unsafe fn get_RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Enabled)(windows_core::Interface::as_raw(self), enabled) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplication_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ProcessImageFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ProcessImageFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub get_IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_IpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub put_IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_IpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub get_Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_Scope: usize,
    #[cfg(feature = "icftypes")]
    pub put_Scope: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_Scope: usize,
    pub get_RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Enabled: usize,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwAuthorizedApplication_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ProcessImageFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ProcessImageFileName(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION>;
    fn put_IpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn get_Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE>;
    fn put_Scope(&self, scope: super::NET_FW_SCOPE) -> windows_core::Result<()>;
    fn get_RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwAuthorizedApplication_Vtbl {
    pub const fn new<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::get_Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Name<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::put_Name(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_ProcessImageFileName<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::get_ProcessImageFileName(this) {
                    Ok(ok__) => {
                        imagefilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ProcessImageFileName<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::put_ProcessImageFileName(this, core::mem::transmute(&imagefilename)).into()
            }
        }
        unsafe extern "system" fn get_IpVersion<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::get_IpVersion(this) {
                    Ok(ok__) => {
                        ipversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IpVersion<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::put_IpVersion(this, core::mem::transmute_copy(&ipversion)).into()
            }
        }
        unsafe extern "system" fn get_Scope<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::get_Scope(this) {
                    Ok(ok__) => {
                        scope.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Scope<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::put_Scope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn get_RemoteAddresses<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::get_RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RemoteAddresses<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::put_RemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn get_Enabled<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplication_Impl::get_Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Enabled<Identity: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwAuthorizedApplication_Impl::put_Enabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            put_Name: put_Name::<Identity, OFFSET>,
            get_ProcessImageFileName: get_ProcessImageFileName::<Identity, OFFSET>,
            put_ProcessImageFileName: put_ProcessImageFileName::<Identity, OFFSET>,
            get_IpVersion: get_IpVersion::<Identity, OFFSET>,
            put_IpVersion: put_IpVersion::<Identity, OFFSET>,
            get_Scope: get_Scope::<Identity, OFFSET>,
            put_Scope: put_Scope::<Identity, OFFSET>,
            get_RemoteAddresses: get_RemoteAddresses::<Identity, OFFSET>,
            put_RemoteAddresses: put_RemoteAddresses::<Identity, OFFSET>,
            get_Enabled: get_Enabled::<Identity, OFFSET>,
            put_Enabled: put_Enabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplication as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplications_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwAuthorizedApplications_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, app: windows_core::Ref<INetFwAuthorizedApplication>) -> windows_core::Result<()>;
    fn Remove(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Item(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<INetFwAuthorizedApplication>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwAuthorizedApplications_Vtbl {
    pub const fn new<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplications_Impl::get_Count(this) {
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
        unsafe extern "system" fn get__NewEnum<Identity: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwAuthorizedApplications_Impl::get__NewEnum(this) {
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
            get_Count: get_Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplications as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_AllowOutboundDestinationUnreachable(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowOutboundDestinationUnreachable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowOutboundDestinationUnreachable(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowOutboundDestinationUnreachable)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AllowRedirect(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowRedirect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowRedirect(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowRedirect)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AllowInboundEchoRequest(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowInboundEchoRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowInboundEchoRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowInboundEchoRequest)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AllowOutboundTimeExceeded(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowOutboundTimeExceeded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowOutboundTimeExceeded(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowOutboundTimeExceeded)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AllowOutboundParameterProblem(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowOutboundParameterProblem)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowOutboundParameterProblem(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowOutboundParameterProblem)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AllowOutboundSourceQuench(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowOutboundSourceQuench)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowOutboundSourceQuench(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowOutboundSourceQuench)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AllowInboundRouterRequest(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowInboundRouterRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowInboundRouterRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowInboundRouterRequest)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AllowInboundTimestampRequest(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowInboundTimestampRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowInboundTimestampRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowInboundTimestampRequest)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AllowInboundMaskRequest(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowInboundMaskRequest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowInboundMaskRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowInboundMaskRequest)(windows_core::Interface::as_raw(self), allow) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AllowOutboundPacketTooBig(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowOutboundPacketTooBig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowOutboundPacketTooBig(&self, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowOutboundPacketTooBig)(windows_core::Interface::as_raw(self), allow) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwIcmpSettings_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_AllowOutboundDestinationUnreachable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowOutboundDestinationUnreachable: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowOutboundDestinationUnreachable: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowOutboundDestinationUnreachable: usize,
    #[cfg(feature = "wtypes")]
    pub get_AllowRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowRedirect: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowRedirect: usize,
    #[cfg(feature = "wtypes")]
    pub get_AllowInboundEchoRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowInboundEchoRequest: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowInboundEchoRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowInboundEchoRequest: usize,
    #[cfg(feature = "wtypes")]
    pub get_AllowOutboundTimeExceeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowOutboundTimeExceeded: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowOutboundTimeExceeded: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowOutboundTimeExceeded: usize,
    #[cfg(feature = "wtypes")]
    pub get_AllowOutboundParameterProblem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowOutboundParameterProblem: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowOutboundParameterProblem: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowOutboundParameterProblem: usize,
    #[cfg(feature = "wtypes")]
    pub get_AllowOutboundSourceQuench: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowOutboundSourceQuench: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowOutboundSourceQuench: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowOutboundSourceQuench: usize,
    #[cfg(feature = "wtypes")]
    pub get_AllowInboundRouterRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowInboundRouterRequest: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowInboundRouterRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowInboundRouterRequest: usize,
    #[cfg(feature = "wtypes")]
    pub get_AllowInboundTimestampRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowInboundTimestampRequest: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowInboundTimestampRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowInboundTimestampRequest: usize,
    #[cfg(feature = "wtypes")]
    pub get_AllowInboundMaskRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowInboundMaskRequest: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowInboundMaskRequest: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowInboundMaskRequest: usize,
    #[cfg(feature = "wtypes")]
    pub get_AllowOutboundPacketTooBig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowOutboundPacketTooBig: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowOutboundPacketTooBig: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowOutboundPacketTooBig: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwIcmpSettings_Impl: super::IDispatch_Impl {
    fn get_AllowOutboundDestinationUnreachable(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowOutboundDestinationUnreachable(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AllowRedirect(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowRedirect(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AllowInboundEchoRequest(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowInboundEchoRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AllowOutboundTimeExceeded(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowOutboundTimeExceeded(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AllowOutboundParameterProblem(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowOutboundParameterProblem(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AllowOutboundSourceQuench(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowOutboundSourceQuench(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AllowInboundRouterRequest(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowInboundRouterRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AllowInboundTimestampRequest(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowInboundTimestampRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AllowInboundMaskRequest(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowInboundMaskRequest(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AllowOutboundPacketTooBig(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowOutboundPacketTooBig(&self, allow: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwIcmpSettings_Vtbl {
    pub const fn new<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AllowOutboundDestinationUnreachable<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::get_AllowOutboundDestinationUnreachable(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowOutboundDestinationUnreachable<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::put_AllowOutboundDestinationUnreachable(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn get_AllowRedirect<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::get_AllowRedirect(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowRedirect<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::put_AllowRedirect(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn get_AllowInboundEchoRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::get_AllowInboundEchoRequest(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowInboundEchoRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::put_AllowInboundEchoRequest(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn get_AllowOutboundTimeExceeded<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::get_AllowOutboundTimeExceeded(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowOutboundTimeExceeded<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::put_AllowOutboundTimeExceeded(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn get_AllowOutboundParameterProblem<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::get_AllowOutboundParameterProblem(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowOutboundParameterProblem<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::put_AllowOutboundParameterProblem(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn get_AllowOutboundSourceQuench<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::get_AllowOutboundSourceQuench(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowOutboundSourceQuench<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::put_AllowOutboundSourceQuench(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn get_AllowInboundRouterRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::get_AllowInboundRouterRequest(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowInboundRouterRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::put_AllowInboundRouterRequest(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn get_AllowInboundTimestampRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::get_AllowInboundTimestampRequest(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowInboundTimestampRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::put_AllowInboundTimestampRequest(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn get_AllowInboundMaskRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::get_AllowInboundMaskRequest(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowInboundMaskRequest<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::put_AllowInboundMaskRequest(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        unsafe extern "system" fn get_AllowOutboundPacketTooBig<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwIcmpSettings_Impl::get_AllowOutboundPacketTooBig(this) {
                    Ok(ok__) => {
                        allow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowOutboundPacketTooBig<Identity: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwIcmpSettings_Impl::put_AllowOutboundPacketTooBig(this, core::mem::transmute_copy(&allow)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_AllowOutboundDestinationUnreachable: get_AllowOutboundDestinationUnreachable::<Identity, OFFSET>,
            put_AllowOutboundDestinationUnreachable: put_AllowOutboundDestinationUnreachable::<Identity, OFFSET>,
            get_AllowRedirect: get_AllowRedirect::<Identity, OFFSET>,
            put_AllowRedirect: put_AllowRedirect::<Identity, OFFSET>,
            get_AllowInboundEchoRequest: get_AllowInboundEchoRequest::<Identity, OFFSET>,
            put_AllowInboundEchoRequest: put_AllowInboundEchoRequest::<Identity, OFFSET>,
            get_AllowOutboundTimeExceeded: get_AllowOutboundTimeExceeded::<Identity, OFFSET>,
            put_AllowOutboundTimeExceeded: put_AllowOutboundTimeExceeded::<Identity, OFFSET>,
            get_AllowOutboundParameterProblem: get_AllowOutboundParameterProblem::<Identity, OFFSET>,
            put_AllowOutboundParameterProblem: put_AllowOutboundParameterProblem::<Identity, OFFSET>,
            get_AllowOutboundSourceQuench: get_AllowOutboundSourceQuench::<Identity, OFFSET>,
            put_AllowOutboundSourceQuench: put_AllowOutboundSourceQuench::<Identity, OFFSET>,
            get_AllowInboundRouterRequest: get_AllowInboundRouterRequest::<Identity, OFFSET>,
            put_AllowInboundRouterRequest: put_AllowInboundRouterRequest::<Identity, OFFSET>,
            get_AllowInboundTimestampRequest: get_AllowInboundTimestampRequest::<Identity, OFFSET>,
            put_AllowInboundTimestampRequest: put_AllowInboundTimestampRequest::<Identity, OFFSET>,
            get_AllowInboundMaskRequest: get_AllowInboundMaskRequest::<Identity, OFFSET>,
            put_AllowInboundMaskRequest: put_AllowInboundMaskRequest::<Identity, OFFSET>,
            get_AllowOutboundPacketTooBig: get_AllowOutboundPacketTooBig::<Identity, OFFSET>,
            put_AllowOutboundPacketTooBig: put_AllowOutboundPacketTooBig::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwIcmpSettings as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_LocalPolicy(&self) -> windows_core::Result<INetFwPolicy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LocalPolicy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_CurrentProfileType(&self) -> windows_core::Result<super::NET_FW_PROFILE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentProfileType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RestoreDefaults(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreDefaults)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "icftypes", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn IsPortAllowed(&self, imagefilename: &windows_core::BSTR, ipversion: super::NET_FW_IP_VERSION, portnumber: super::LONG, localaddress: &windows_core::BSTR, ipprotocol: super::NET_FW_IP_PROTOCOL, allowed: *mut super::VARIANT, restricted: *mut super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsPortAllowed)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename), ipversion, portnumber, core::mem::transmute_copy(localaddress), ipprotocol, allowed, restricted) }
    }
    #[cfg(all(feature = "icftypes", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn IsIcmpTypeAllowed(&self, ipversion: super::NET_FW_IP_VERSION, localaddress: &windows_core::BSTR, r#type: super::BYTE, allowed: *mut super::VARIANT, restricted: *mut super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsIcmpTypeAllowed)(windows_core::Interface::as_raw(self), ipversion, core::mem::transmute_copy(localaddress), r#type, allowed, restricted) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwMgr_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_LocalPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub get_CurrentProfileType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_PROFILE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_CurrentProfileType: usize,
    pub RestoreDefaults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "icftypes", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub IsPortAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::NET_FW_IP_VERSION, super::LONG, *mut core::ffi::c_void, super::NET_FW_IP_PROTOCOL, *mut super::VARIANT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    IsPortAllowed: usize,
    #[cfg(all(feature = "icftypes", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub IsIcmpTypeAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_IP_VERSION, *mut core::ffi::c_void, super::BYTE, *mut super::VARIANT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    IsIcmpTypeAllowed: usize,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwMgr_Impl: super::IDispatch_Impl {
    fn get_LocalPolicy(&self) -> windows_core::Result<INetFwPolicy>;
    fn get_CurrentProfileType(&self) -> windows_core::Result<super::NET_FW_PROFILE_TYPE>;
    fn RestoreDefaults(&self) -> windows_core::Result<()>;
    fn IsPortAllowed(&self, imagefilename: &windows_core::BSTR, ipversion: super::NET_FW_IP_VERSION, portnumber: super::LONG, localaddress: &windows_core::BSTR, ipprotocol: super::NET_FW_IP_PROTOCOL, allowed: *mut super::VARIANT, restricted: *mut super::VARIANT) -> windows_core::Result<()>;
    fn IsIcmpTypeAllowed(&self, ipversion: super::NET_FW_IP_VERSION, localaddress: &windows_core::BSTR, r#type: super::BYTE, allowed: *mut super::VARIANT, restricted: *mut super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwMgr_Vtbl {
    pub const fn new<Identity: INetFwMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_LocalPolicy<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpolicy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwMgr_Impl::get_LocalPolicy(this) {
                    Ok(ok__) => {
                        localpolicy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentProfileType<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: *mut super::NET_FW_PROFILE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwMgr_Impl::get_CurrentProfileType(this) {
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
        unsafe extern "system" fn IsPortAllowed<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void, ipversion: super::NET_FW_IP_VERSION, portnumber: super::LONG, localaddress: *mut core::ffi::c_void, ipprotocol: super::NET_FW_IP_PROTOCOL, allowed: *mut super::VARIANT, restricted: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwMgr_Impl::IsPortAllowed(this, core::mem::transmute(&imagefilename), core::mem::transmute_copy(&ipversion), core::mem::transmute_copy(&portnumber), core::mem::transmute(&localaddress), core::mem::transmute_copy(&ipprotocol), core::mem::transmute_copy(&allowed), core::mem::transmute_copy(&restricted)).into()
            }
        }
        unsafe extern "system" fn IsIcmpTypeAllowed<Identity: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: super::NET_FW_IP_VERSION, localaddress: *mut core::ffi::c_void, r#type: super::BYTE, allowed: *mut super::VARIANT, restricted: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwMgr_Impl::IsIcmpTypeAllowed(this, core::mem::transmute_copy(&ipversion), core::mem::transmute(&localaddress), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&allowed), core::mem::transmute_copy(&restricted)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_LocalPolicy: get_LocalPolicy::<Identity, OFFSET>,
            get_CurrentProfileType: get_CurrentProfileType::<Identity, OFFSET>,
            RestoreDefaults: RestoreDefaults::<Identity, OFFSET>,
            IsPortAllowed: IsPortAllowed::<Identity, OFFSET>,
            IsIcmpTypeAllowed: IsIcmpTypeAllowed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwMgr as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_IpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IpVersion)(windows_core::Interface::as_raw(self), ipversion) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_Protocol(&self) -> windows_core::Result<super::NET_FW_IP_PROTOCOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Protocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_Protocol(&self, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Protocol)(windows_core::Interface::as_raw(self), ipprotocol) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Port(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Port)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_Port(&self, portnumber: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Port)(windows_core::Interface::as_raw(self), portnumber) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_Scope(&self, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Scope)(windows_core::Interface::as_raw(self), scope) }
    }
    pub unsafe fn get_RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Enabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_BuiltIn(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BuiltIn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPort_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub get_IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_IpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub put_IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_IpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub get_Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_Protocol: usize,
    #[cfg(feature = "icftypes")]
    pub put_Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_Protocol: usize,
    #[cfg(feature = "shared")]
    pub get_Port: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Port: usize,
    #[cfg(feature = "shared")]
    pub put_Port: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_Port: usize,
    #[cfg(feature = "icftypes")]
    pub get_Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_Scope: usize,
    #[cfg(feature = "icftypes")]
    pub put_Scope: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_Scope: usize,
    pub get_RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_BuiltIn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_BuiltIn: usize,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwOpenPort_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION>;
    fn put_IpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn get_Protocol(&self) -> windows_core::Result<super::NET_FW_IP_PROTOCOL>;
    fn put_Protocol(&self, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::Result<()>;
    fn get_Port(&self) -> windows_core::Result<super::LONG>;
    fn put_Port(&self, portnumber: super::LONG) -> windows_core::Result<()>;
    fn get_Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE>;
    fn put_Scope(&self, scope: super::NET_FW_SCOPE) -> windows_core::Result<()>;
    fn get_RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_BuiltIn(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwOpenPort_Vtbl {
    pub const fn new<Identity: INetFwOpenPort_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::get_Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Name<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::put_Name(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_IpVersion<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::get_IpVersion(this) {
                    Ok(ok__) => {
                        ipversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IpVersion<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::put_IpVersion(this, core::mem::transmute_copy(&ipversion)).into()
            }
        }
        unsafe extern "system" fn get_Protocol<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipprotocol: *mut super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::get_Protocol(this) {
                    Ok(ok__) => {
                        ipprotocol.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Protocol<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::put_Protocol(this, core::mem::transmute_copy(&ipprotocol)).into()
            }
        }
        unsafe extern "system" fn get_Port<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::get_Port(this) {
                    Ok(ok__) => {
                        portnumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Port<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::put_Port(this, core::mem::transmute_copy(&portnumber)).into()
            }
        }
        unsafe extern "system" fn get_Scope<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::get_Scope(this) {
                    Ok(ok__) => {
                        scope.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Scope<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::put_Scope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn get_RemoteAddresses<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::get_RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RemoteAddresses<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::put_RemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn get_Enabled<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::get_Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Enabled<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPort_Impl::put_Enabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn get_BuiltIn<Identity: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, builtin: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPort_Impl::get_BuiltIn(this) {
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
            get_Name: get_Name::<Identity, OFFSET>,
            put_Name: put_Name::<Identity, OFFSET>,
            get_IpVersion: get_IpVersion::<Identity, OFFSET>,
            put_IpVersion: put_IpVersion::<Identity, OFFSET>,
            get_Protocol: get_Protocol::<Identity, OFFSET>,
            put_Protocol: put_Protocol::<Identity, OFFSET>,
            get_Port: get_Port::<Identity, OFFSET>,
            put_Port: put_Port::<Identity, OFFSET>,
            get_Scope: get_Scope::<Identity, OFFSET>,
            put_Scope: put_Scope::<Identity, OFFSET>,
            get_RemoteAddresses: get_RemoteAddresses::<Identity, OFFSET>,
            put_RemoteAddresses: put_RemoteAddresses::<Identity, OFFSET>,
            get_Enabled: get_Enabled::<Identity, OFFSET>,
            put_Enabled: put_Enabled::<Identity, OFFSET>,
            get_BuiltIn: get_BuiltIn::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwOpenPort as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0>(&self, port: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<INetFwOpenPort>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), port.param().abi()) }
    }
    #[cfg(all(feature = "icftypes", feature = "shared"))]
    pub unsafe fn Remove(&self, portnumber: super::LONG, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), portnumber, ipprotocol) }
    }
    #[cfg(all(feature = "icftypes", feature = "shared"))]
    pub unsafe fn Item(&self, portnumber: super::LONG, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::Result<INetFwOpenPort> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), portnumber, ipprotocol, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPorts_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "icftypes", feature = "shared"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "shared")))]
    Remove: usize,
    #[cfg(all(feature = "icftypes", feature = "shared"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::NET_FW_IP_PROTOCOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "shared")))]
    Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwOpenPorts_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, port: windows_core::Ref<INetFwOpenPort>) -> windows_core::Result<()>;
    fn Remove(&self, portnumber: super::LONG, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::Result<()>;
    fn Item(&self, portnumber: super::LONG, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::Result<INetFwOpenPort>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwOpenPorts_Vtbl {
    pub const fn new<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPorts_Impl::get_Count(this) {
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
        unsafe extern "system" fn Remove<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: super::LONG, ipprotocol: super::NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwOpenPorts_Impl::Remove(this, core::mem::transmute_copy(&portnumber), core::mem::transmute_copy(&ipprotocol)).into()
            }
        }
        unsafe extern "system" fn Item<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: super::LONG, ipprotocol: super::NET_FW_IP_PROTOCOL, openport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get__NewEnum<Identity: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwOpenPorts_Impl::get__NewEnum(this) {
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
            get_Count: get_Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwOpenPorts as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_CurrentProfile(&self) -> windows_core::Result<INetFwProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentProfile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub get_CurrentProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub GetProfileByType: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    GetProfileByType: usize,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwPolicy_Impl: super::IDispatch_Impl {
    fn get_CurrentProfile(&self) -> windows_core::Result<INetFwProfile>;
    fn GetProfileByType(&self, profiletype: super::NET_FW_PROFILE_TYPE) -> windows_core::Result<INetFwProfile>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwPolicy_Vtbl {
    pub const fn new<Identity: INetFwPolicy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentProfile<Identity: INetFwPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy_Impl::get_CurrentProfile(this) {
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
            get_CurrentProfile: get_CurrentProfile::<Identity, OFFSET>,
            GetProfileByType: GetProfileByType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwPolicy as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_CurrentProfileTypes(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentProfileTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn get_FirewallEnabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FirewallEnabled)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn put_FirewallEnabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FirewallEnabled)(windows_core::Interface::as_raw(self), profiletype, enabled) }
    }
    #[cfg(all(feature = "icftypes", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_ExcludedInterfaces(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExcludedInterfaces)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "icftypes", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_ExcludedInterfaces(&self, profiletype: super::NET_FW_PROFILE_TYPE2, interfaces: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ExcludedInterfaces)(windows_core::Interface::as_raw(self), profiletype, core::mem::transmute_copy(interfaces)) }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn get_BlockAllInboundTraffic(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BlockAllInboundTraffic)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn put_BlockAllInboundTraffic(&self, profiletype: super::NET_FW_PROFILE_TYPE2, block: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BlockAllInboundTraffic)(windows_core::Interface::as_raw(self), profiletype, block) }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn get_NotificationsDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NotificationsDisabled)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn put_NotificationsDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NotificationsDisabled)(windows_core::Interface::as_raw(self), profiletype, disabled) }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn get_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub unsafe fn put_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), profiletype, disabled) }
    }
    pub unsafe fn get_Rules(&self) -> windows_core::Result<INetFwRules> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Rules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ServiceRestriction(&self) -> windows_core::Result<INetFwServiceRestriction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServiceRestriction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub unsafe fn get_DefaultInboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::NET_FW_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DefaultInboundAction)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_DefaultInboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2, action: super::NET_FW_ACTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DefaultInboundAction)(windows_core::Interface::as_raw(self), profiletype, action) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_DefaultOutboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::NET_FW_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DefaultOutboundAction)(windows_core::Interface::as_raw(self), profiletype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_DefaultOutboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2, action: super::NET_FW_ACTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DefaultOutboundAction)(windows_core::Interface::as_raw(self), profiletype, action) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsRuleGroupCurrentlyEnabled(&self, group: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsRuleGroupCurrentlyEnabled)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(group), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_LocalPolicyModifyState(&self) -> windows_core::Result<super::NET_FW_MODIFY_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LocalPolicyModifyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy2_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_CurrentProfileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub get_FirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    get_FirewallEnabled: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub put_FirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    put_FirewallEnabled: usize,
    #[cfg(all(feature = "icftypes", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_ExcludedInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_ExcludedInterfaces: usize,
    #[cfg(all(feature = "icftypes", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_ExcludedInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_ExcludedInterfaces: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub get_BlockAllInboundTraffic: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    get_BlockAllInboundTraffic: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub put_BlockAllInboundTraffic: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    put_BlockAllInboundTraffic: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub get_NotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    get_NotificationsDisabled: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub put_NotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    put_NotificationsDisabled: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub get_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    get_UnicastResponsesToMulticastBroadcastDisabled: usize,
    #[cfg(all(feature = "icftypes", feature = "wtypes"))]
    pub put_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "icftypes", feature = "wtypes")))]
    put_UnicastResponsesToMulticastBroadcastDisabled: usize,
    pub get_Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ServiceRestriction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
    pub get_DefaultInboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::NET_FW_ACTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_DefaultInboundAction: usize,
    #[cfg(feature = "icftypes")]
    pub put_DefaultInboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::NET_FW_ACTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_DefaultInboundAction: usize,
    #[cfg(feature = "icftypes")]
    pub get_DefaultOutboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, *mut super::NET_FW_ACTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_DefaultOutboundAction: usize,
    #[cfg(feature = "icftypes")]
    pub put_DefaultOutboundAction: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_PROFILE_TYPE2, super::NET_FW_ACTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_DefaultOutboundAction: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsRuleGroupCurrentlyEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsRuleGroupCurrentlyEnabled: usize,
    #[cfg(feature = "icftypes")]
    pub get_LocalPolicyModifyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_MODIFY_STATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_LocalPolicyModifyState: usize,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwPolicy2_Impl: super::IDispatch_Impl {
    fn get_CurrentProfileTypes(&self) -> windows_core::Result<i32>;
    fn get_FirewallEnabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_FirewallEnabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ExcludedInterfaces(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT>;
    fn put_ExcludedInterfaces(&self, profiletype: super::NET_FW_PROFILE_TYPE2, interfaces: &super::VARIANT) -> windows_core::Result<()>;
    fn get_BlockAllInboundTraffic(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_BlockAllInboundTraffic(&self, profiletype: super::NET_FW_PROFILE_TYPE2, block: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_NotificationsDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_NotificationsDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Rules(&self) -> windows_core::Result<INetFwRules>;
    fn get_ServiceRestriction(&self) -> windows_core::Result<INetFwServiceRestriction>;
    fn EnableRuleGroup(&self, profiletypesbitmask: i32, group: &windows_core::BSTR, enable: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsRuleGroupEnabled(&self, profiletypesbitmask: i32, group: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn RestoreLocalFirewallDefaults(&self) -> windows_core::Result<()>;
    fn get_DefaultInboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::NET_FW_ACTION>;
    fn put_DefaultInboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2, action: super::NET_FW_ACTION) -> windows_core::Result<()>;
    fn get_DefaultOutboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::NET_FW_ACTION>;
    fn put_DefaultOutboundAction(&self, profiletype: super::NET_FW_PROFILE_TYPE2, action: super::NET_FW_ACTION) -> windows_core::Result<()>;
    fn get_IsRuleGroupCurrentlyEnabled(&self, group: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_LocalPolicyModifyState(&self) -> windows_core::Result<super::NET_FW_MODIFY_STATE>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwPolicy2_Vtbl {
    pub const fn new<Identity: INetFwPolicy2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentProfileTypes<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_CurrentProfileTypes(this) {
                    Ok(ok__) => {
                        profiletypesbitmask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FirewallEnabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_FirewallEnabled(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FirewallEnabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_FirewallEnabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn get_ExcludedInterfaces<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, interfaces: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_ExcludedInterfaces(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        interfaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExcludedInterfaces<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, interfaces: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_ExcludedInterfaces(this, core::mem::transmute_copy(&profiletype), core::mem::transmute(&interfaces)).into()
            }
        }
        unsafe extern "system" fn get_BlockAllInboundTraffic<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, block: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_BlockAllInboundTraffic(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        block.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BlockAllInboundTraffic<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, block: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_BlockAllInboundTraffic(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&block)).into()
            }
        }
        unsafe extern "system" fn get_NotificationsDisabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_NotificationsDisabled(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        disabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NotificationsDisabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_NotificationsDisabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&disabled)).into()
            }
        }
        unsafe extern "system" fn get_UnicastResponsesToMulticastBroadcastDisabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_UnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        disabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UnicastResponsesToMulticastBroadcastDisabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_UnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&disabled)).into()
            }
        }
        unsafe extern "system" fn get_Rules<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_Rules(this) {
                    Ok(ok__) => {
                        rules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ServiceRestriction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicerestriction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_ServiceRestriction(this) {
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
        unsafe extern "system" fn get_DefaultInboundAction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, action: *mut super::NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_DefaultInboundAction(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        action.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DefaultInboundAction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, action: super::NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_DefaultInboundAction(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn get_DefaultOutboundAction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, action: *mut super::NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_DefaultOutboundAction(this, core::mem::transmute_copy(&profiletype)) {
                    Ok(ok__) => {
                        action.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DefaultOutboundAction<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: super::NET_FW_PROFILE_TYPE2, action: super::NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwPolicy2_Impl::put_DefaultOutboundAction(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn get_IsRuleGroupCurrentlyEnabled<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_IsRuleGroupCurrentlyEnabled(this, core::mem::transmute(&group)) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LocalPolicyModifyState<Identity: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modifystate: *mut super::NET_FW_MODIFY_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwPolicy2_Impl::get_LocalPolicyModifyState(this) {
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
            get_CurrentProfileTypes: get_CurrentProfileTypes::<Identity, OFFSET>,
            get_FirewallEnabled: get_FirewallEnabled::<Identity, OFFSET>,
            put_FirewallEnabled: put_FirewallEnabled::<Identity, OFFSET>,
            get_ExcludedInterfaces: get_ExcludedInterfaces::<Identity, OFFSET>,
            put_ExcludedInterfaces: put_ExcludedInterfaces::<Identity, OFFSET>,
            get_BlockAllInboundTraffic: get_BlockAllInboundTraffic::<Identity, OFFSET>,
            put_BlockAllInboundTraffic: put_BlockAllInboundTraffic::<Identity, OFFSET>,
            get_NotificationsDisabled: get_NotificationsDisabled::<Identity, OFFSET>,
            put_NotificationsDisabled: put_NotificationsDisabled::<Identity, OFFSET>,
            get_UnicastResponsesToMulticastBroadcastDisabled: get_UnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            put_UnicastResponsesToMulticastBroadcastDisabled: put_UnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            get_Rules: get_Rules::<Identity, OFFSET>,
            get_ServiceRestriction: get_ServiceRestriction::<Identity, OFFSET>,
            EnableRuleGroup: EnableRuleGroup::<Identity, OFFSET>,
            IsRuleGroupEnabled: IsRuleGroupEnabled::<Identity, OFFSET>,
            RestoreLocalFirewallDefaults: RestoreLocalFirewallDefaults::<Identity, OFFSET>,
            get_DefaultInboundAction: get_DefaultInboundAction::<Identity, OFFSET>,
            put_DefaultInboundAction: put_DefaultInboundAction::<Identity, OFFSET>,
            get_DefaultOutboundAction: get_DefaultOutboundAction::<Identity, OFFSET>,
            put_DefaultOutboundAction: put_DefaultOutboundAction::<Identity, OFFSET>,
            get_IsRuleGroupCurrentlyEnabled: get_IsRuleGroupCurrentlyEnabled::<Identity, OFFSET>,
            get_LocalPolicyModifyState: get_LocalPolicyModifyState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwPolicy2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_RuleCategories(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RuleCategories)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_RuleCategories(&self, rulecategories: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RuleCategories)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(rulecategories)) }
    }
    pub unsafe fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(displayname)) }
    }
    pub unsafe fn get_PathToSignedProductExe(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PathToSignedProductExe)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProduct_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_RuleCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_RuleCategories: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_RuleCategories: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_RuleCategories: usize,
    pub get_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PathToSignedProductExe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwProduct_Impl: super::IDispatch_Impl {
    fn get_RuleCategories(&self) -> windows_core::Result<super::VARIANT>;
    fn put_RuleCategories(&self, rulecategories: &super::VARIANT) -> windows_core::Result<()>;
    fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PathToSignedProductExe(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwProduct_Vtbl {
    pub const fn new<Identity: INetFwProduct_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_RuleCategories<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulecategories: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProduct_Impl::get_RuleCategories(this) {
                    Ok(ok__) => {
                        rulecategories.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RuleCategories<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulecategories: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProduct_Impl::put_RuleCategories(this, core::mem::transmute(&rulecategories)).into()
            }
        }
        unsafe extern "system" fn get_DisplayName<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProduct_Impl::get_DisplayName(this) {
                    Ok(ok__) => {
                        displayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisplayName<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProduct_Impl::put_DisplayName(this, core::mem::transmute(&displayname)).into()
            }
        }
        unsafe extern "system" fn get_PathToSignedProductExe<Identity: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProduct_Impl::get_PathToSignedProductExe(this) {
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
            get_RuleCategories: get_RuleCategories::<Identity, OFFSET>,
            put_RuleCategories: put_RuleCategories::<Identity, OFFSET>,
            get_DisplayName: get_DisplayName::<Identity, OFFSET>,
            put_DisplayName: put_DisplayName::<Identity, OFFSET>,
            get_PathToSignedProductExe: get_PathToSignedProductExe::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwProduct as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProducts_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwProducts_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn Register(&self, product: windows_core::Ref<INetFwProduct>) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<INetFwProduct>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwProducts_Vtbl {
    pub const fn new<Identity: INetFwProducts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProducts_Impl::get_Count(this) {
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
        unsafe extern "system" fn get__NewEnum<Identity: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProducts_Impl::get__NewEnum(this) {
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
            get_Count: get_Count::<Identity, OFFSET>,
            Register: Register::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwProducts as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Type(&self) -> windows_core::Result<super::NET_FW_PROFILE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_FirewallEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FirewallEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_FirewallEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FirewallEnabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ExceptionsNotAllowed(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExceptionsNotAllowed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_ExceptionsNotAllowed(&self, notallowed: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ExceptionsNotAllowed)(windows_core::Interface::as_raw(self), notallowed) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_NotificationsDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NotificationsDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_NotificationsDisabled(&self, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NotificationsDisabled)(windows_core::Interface::as_raw(self), disabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_UnicastResponsesToMulticastBroadcastDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_UnicastResponsesToMulticastBroadcastDisabled(&self, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UnicastResponsesToMulticastBroadcastDisabled)(windows_core::Interface::as_raw(self), disabled) }
    }
    pub unsafe fn get_RemoteAdminSettings(&self) -> windows_core::Result<INetFwRemoteAdminSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemoteAdminSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_IcmpSettings(&self) -> windows_core::Result<INetFwIcmpSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IcmpSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_GloballyOpenPorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Services(&self) -> windows_core::Result<INetFwServices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Services)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_AuthorizedApplications(&self) -> windows_core::Result<INetFwAuthorizedApplications> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AuthorizedApplications)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProfile_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "icftypes")]
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_PROFILE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_Type: usize,
    #[cfg(feature = "wtypes")]
    pub get_FirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_FirewallEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_FirewallEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_FirewallEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_ExceptionsNotAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ExceptionsNotAllowed: usize,
    #[cfg(feature = "wtypes")]
    pub put_ExceptionsNotAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_ExceptionsNotAllowed: usize,
    #[cfg(feature = "wtypes")]
    pub get_NotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_NotificationsDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_NotificationsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_NotificationsDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_UnicastResponsesToMulticastBroadcastDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_UnicastResponsesToMulticastBroadcastDisabled: usize,
    pub get_RemoteAdminSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_IcmpSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_GloballyOpenPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Services: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_AuthorizedApplications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwProfile_Impl: super::IDispatch_Impl {
    fn get_Type(&self) -> windows_core::Result<super::NET_FW_PROFILE_TYPE>;
    fn get_FirewallEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_FirewallEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ExceptionsNotAllowed(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_ExceptionsNotAllowed(&self, notallowed: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_NotificationsDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_NotificationsDisabled(&self, disabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_UnicastResponsesToMulticastBroadcastDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_UnicastResponsesToMulticastBroadcastDisabled(&self, disabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_RemoteAdminSettings(&self) -> windows_core::Result<INetFwRemoteAdminSettings>;
    fn get_IcmpSettings(&self) -> windows_core::Result<INetFwIcmpSettings>;
    fn get_GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts>;
    fn get_Services(&self) -> windows_core::Result<INetFwServices>;
    fn get_AuthorizedApplications(&self) -> windows_core::Result<INetFwAuthorizedApplications>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwProfile_Vtbl {
    pub const fn new<Identity: INetFwProfile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Type<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut super::NET_FW_PROFILE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::get_Type(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FirewallEnabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::get_FirewallEnabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FirewallEnabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProfile_Impl::put_FirewallEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn get_ExceptionsNotAllowed<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notallowed: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::get_ExceptionsNotAllowed(this) {
                    Ok(ok__) => {
                        notallowed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExceptionsNotAllowed<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notallowed: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProfile_Impl::put_ExceptionsNotAllowed(this, core::mem::transmute_copy(&notallowed)).into()
            }
        }
        unsafe extern "system" fn get_NotificationsDisabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::get_NotificationsDisabled(this) {
                    Ok(ok__) => {
                        disabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NotificationsDisabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProfile_Impl::put_NotificationsDisabled(this, core::mem::transmute_copy(&disabled)).into()
            }
        }
        unsafe extern "system" fn get_UnicastResponsesToMulticastBroadcastDisabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::get_UnicastResponsesToMulticastBroadcastDisabled(this) {
                    Ok(ok__) => {
                        disabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UnicastResponsesToMulticastBroadcastDisabled<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwProfile_Impl::put_UnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&disabled)).into()
            }
        }
        unsafe extern "system" fn get_RemoteAdminSettings<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteadminsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::get_RemoteAdminSettings(this) {
                    Ok(ok__) => {
                        remoteadminsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IcmpSettings<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmpsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::get_IcmpSettings(this) {
                    Ok(ok__) => {
                        icmpsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_GloballyOpenPorts<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, openports: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::get_GloballyOpenPorts(this) {
                    Ok(ok__) => {
                        openports.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Services<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, services: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::get_Services(this) {
                    Ok(ok__) => {
                        services.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AuthorizedApplications<Identity: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, apps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwProfile_Impl::get_AuthorizedApplications(this) {
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
            get_Type: get_Type::<Identity, OFFSET>,
            get_FirewallEnabled: get_FirewallEnabled::<Identity, OFFSET>,
            put_FirewallEnabled: put_FirewallEnabled::<Identity, OFFSET>,
            get_ExceptionsNotAllowed: get_ExceptionsNotAllowed::<Identity, OFFSET>,
            put_ExceptionsNotAllowed: put_ExceptionsNotAllowed::<Identity, OFFSET>,
            get_NotificationsDisabled: get_NotificationsDisabled::<Identity, OFFSET>,
            put_NotificationsDisabled: put_NotificationsDisabled::<Identity, OFFSET>,
            get_UnicastResponsesToMulticastBroadcastDisabled: get_UnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            put_UnicastResponsesToMulticastBroadcastDisabled: put_UnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            get_RemoteAdminSettings: get_RemoteAdminSettings::<Identity, OFFSET>,
            get_IcmpSettings: get_IcmpSettings::<Identity, OFFSET>,
            get_GloballyOpenPorts: get_GloballyOpenPorts::<Identity, OFFSET>,
            get_Services: get_Services::<Identity, OFFSET>,
            get_AuthorizedApplications: get_AuthorizedApplications::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwProfile as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_IpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IpVersion)(windows_core::Interface::as_raw(self), ipversion) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_Scope(&self, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Scope)(windows_core::Interface::as_raw(self), scope) }
    }
    pub unsafe fn get_RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Enabled)(windows_core::Interface::as_raw(self), enabled) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRemoteAdminSettings_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "icftypes")]
    pub get_IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_IpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub put_IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_IpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub get_Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_Scope: usize,
    #[cfg(feature = "icftypes")]
    pub put_Scope: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_Scope: usize,
    pub get_RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Enabled: usize,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwRemoteAdminSettings_Impl: super::IDispatch_Impl {
    fn get_IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION>;
    fn put_IpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn get_Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE>;
    fn put_Scope(&self, scope: super::NET_FW_SCOPE) -> windows_core::Result<()>;
    fn get_RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwRemoteAdminSettings_Vtbl {
    pub const fn new<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_IpVersion<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRemoteAdminSettings_Impl::get_IpVersion(this) {
                    Ok(ok__) => {
                        ipversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IpVersion<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRemoteAdminSettings_Impl::put_IpVersion(this, core::mem::transmute_copy(&ipversion)).into()
            }
        }
        unsafe extern "system" fn get_Scope<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRemoteAdminSettings_Impl::get_Scope(this) {
                    Ok(ok__) => {
                        scope.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Scope<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRemoteAdminSettings_Impl::put_Scope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn get_RemoteAddresses<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRemoteAdminSettings_Impl::get_RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RemoteAddresses<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRemoteAdminSettings_Impl::put_RemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn get_Enabled<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRemoteAdminSettings_Impl::get_Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Enabled<Identity: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRemoteAdminSettings_Impl::put_Enabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_IpVersion: get_IpVersion::<Identity, OFFSET>,
            put_IpVersion: put_IpVersion::<Identity, OFFSET>,
            get_Scope: get_Scope::<Identity, OFFSET>,
            put_Scope: put_Scope::<Identity, OFFSET>,
            get_RemoteAddresses: get_RemoteAddresses::<Identity, OFFSET>,
            put_RemoteAddresses: put_RemoteAddresses::<Identity, OFFSET>,
            get_Enabled: get_Enabled::<Identity, OFFSET>,
            put_Enabled: put_Enabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRemoteAdminSettings as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, desc: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(desc)) }
    }
    pub unsafe fn get_ApplicationName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ApplicationName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ApplicationName(&self, imagefilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ApplicationName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(imagefilename)) }
    }
    pub unsafe fn get_ServiceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServiceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ServiceName(&self, servicename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ServiceName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Protocol(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Protocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_Protocol(&self, protocol: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Protocol)(windows_core::Interface::as_raw(self), protocol) }
    }
    pub unsafe fn get_LocalPorts(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LocalPorts)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LocalPorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LocalPorts)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(portnumbers)) }
    }
    pub unsafe fn get_RemotePorts(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemotePorts)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RemotePorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RemotePorts)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(portnumbers)) }
    }
    pub unsafe fn get_LocalAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LocalAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LocalAddresses(&self, localaddrs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LocalAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(localaddrs)) }
    }
    pub unsafe fn get_RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)) }
    }
    pub unsafe fn get_IcmpTypesAndCodes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IcmpTypesAndCodes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_IcmpTypesAndCodes(&self, icmptypesandcodes: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IcmpTypesAndCodes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(icmptypesandcodes)) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_Direction(&self) -> windows_core::Result<super::NET_FW_RULE_DIRECTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_Direction(&self, dir: super::NET_FW_RULE_DIRECTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Direction)(windows_core::Interface::as_raw(self), dir) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Interfaces(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Interfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Interfaces(&self, interfaces: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Interfaces)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(interfaces)) }
    }
    pub unsafe fn get_InterfaceTypes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InterfaceTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_InterfaceTypes(&self, interfacetypes: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InterfaceTypes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(interfacetypes)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Enabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    pub unsafe fn get_Grouping(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Grouping)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Grouping(&self, context: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Grouping)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(context)) }
    }
    pub unsafe fn get_Profiles(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Profiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Profiles(&self, profiletypesbitmask: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Profiles)(windows_core::Interface::as_raw(self), profiletypesbitmask) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_EdgeTraversal(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EdgeTraversal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_EdgeTraversal(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EdgeTraversal)(windows_core::Interface::as_raw(self), enabled) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_Action(&self) -> windows_core::Result<super::NET_FW_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Action)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_Action(&self, action: super::NET_FW_ACTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Action)(windows_core::Interface::as_raw(self), action) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ApplicationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ApplicationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Protocol: usize,
    #[cfg(feature = "shared")]
    pub put_Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_Protocol: usize,
    pub get_LocalPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LocalPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RemotePorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RemotePorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LocalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LocalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_IcmpTypesAndCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_IcmpTypesAndCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub get_Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_RULE_DIRECTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_Direction: usize,
    #[cfg(feature = "icftypes")]
    pub put_Direction: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_RULE_DIRECTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_Direction: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Interfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Interfaces: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Interfaces: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Interfaces: usize,
    pub get_InterfaceTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_InterfaceTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Enabled: usize,
    pub get_Grouping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Grouping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Profiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Profiles: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_EdgeTraversal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_EdgeTraversal: usize,
    #[cfg(feature = "wtypes")]
    pub put_EdgeTraversal: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_EdgeTraversal: usize,
    #[cfg(feature = "icftypes")]
    pub get_Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_ACTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_Action: usize,
    #[cfg(feature = "icftypes")]
    pub put_Action: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_ACTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_Action: usize,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwRule_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, desc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ApplicationName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ApplicationName(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ServiceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ServiceName(&self, servicename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Protocol(&self) -> windows_core::Result<super::LONG>;
    fn put_Protocol(&self, protocol: super::LONG) -> windows_core::Result<()>;
    fn get_LocalPorts(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LocalPorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_RemotePorts(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RemotePorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_LocalAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LocalAddresses(&self, localaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_IcmpTypesAndCodes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_IcmpTypesAndCodes(&self, icmptypesandcodes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Direction(&self) -> windows_core::Result<super::NET_FW_RULE_DIRECTION>;
    fn put_Direction(&self, dir: super::NET_FW_RULE_DIRECTION) -> windows_core::Result<()>;
    fn get_Interfaces(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Interfaces(&self, interfaces: &super::VARIANT) -> windows_core::Result<()>;
    fn get_InterfaceTypes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_InterfaceTypes(&self, interfacetypes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Grouping(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Grouping(&self, context: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Profiles(&self) -> windows_core::Result<i32>;
    fn put_Profiles(&self, profiletypesbitmask: i32) -> windows_core::Result<()>;
    fn get_EdgeTraversal(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_EdgeTraversal(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Action(&self) -> windows_core::Result<super::NET_FW_ACTION>;
    fn put_Action(&self, action: super::NET_FW_ACTION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwRule_Vtbl {
    pub const fn new<Identity: INetFwRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Name<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_Name(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_Description<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_Description(this) {
                    Ok(ok__) => {
                        desc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_Description(this, core::mem::transmute(&desc)).into()
            }
        }
        unsafe extern "system" fn get_ApplicationName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_ApplicationName(this) {
                    Ok(ok__) => {
                        imagefilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ApplicationName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_ApplicationName(this, core::mem::transmute(&imagefilename)).into()
            }
        }
        unsafe extern "system" fn get_ServiceName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_ServiceName(this) {
                    Ok(ok__) => {
                        servicename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ServiceName<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_ServiceName(this, core::mem::transmute(&servicename)).into()
            }
        }
        unsafe extern "system" fn get_Protocol<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protocol: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_Protocol(this) {
                    Ok(ok__) => {
                        protocol.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Protocol<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protocol: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_Protocol(this, core::mem::transmute_copy(&protocol)).into()
            }
        }
        unsafe extern "system" fn get_LocalPorts<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_LocalPorts(this) {
                    Ok(ok__) => {
                        portnumbers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LocalPorts<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_LocalPorts(this, core::mem::transmute(&portnumbers)).into()
            }
        }
        unsafe extern "system" fn get_RemotePorts<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_RemotePorts(this) {
                    Ok(ok__) => {
                        portnumbers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RemotePorts<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_RemotePorts(this, core::mem::transmute(&portnumbers)).into()
            }
        }
        unsafe extern "system" fn get_LocalAddresses<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_LocalAddresses(this) {
                    Ok(ok__) => {
                        localaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LocalAddresses<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_LocalAddresses(this, core::mem::transmute(&localaddrs)).into()
            }
        }
        unsafe extern "system" fn get_RemoteAddresses<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RemoteAddresses<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_RemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn get_IcmpTypesAndCodes<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmptypesandcodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_IcmpTypesAndCodes(this) {
                    Ok(ok__) => {
                        icmptypesandcodes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IcmpTypesAndCodes<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmptypesandcodes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_IcmpTypesAndCodes(this, core::mem::transmute(&icmptypesandcodes)).into()
            }
        }
        unsafe extern "system" fn get_Direction<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dir: *mut super::NET_FW_RULE_DIRECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_Direction(this) {
                    Ok(ok__) => {
                        dir.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Direction<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dir: super::NET_FW_RULE_DIRECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_Direction(this, core::mem::transmute_copy(&dir)).into()
            }
        }
        unsafe extern "system" fn get_Interfaces<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaces: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_Interfaces(this) {
                    Ok(ok__) => {
                        interfaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Interfaces<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaces: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_Interfaces(this, core::mem::transmute(&interfaces)).into()
            }
        }
        unsafe extern "system" fn get_InterfaceTypes<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacetypes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_InterfaceTypes(this) {
                    Ok(ok__) => {
                        interfacetypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_InterfaceTypes<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacetypes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_InterfaceTypes(this, core::mem::transmute(&interfacetypes)).into()
            }
        }
        unsafe extern "system" fn get_Enabled<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Enabled<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_Enabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn get_Grouping<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_Grouping(this) {
                    Ok(ok__) => {
                        context.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Grouping<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_Grouping(this, core::mem::transmute(&context)).into()
            }
        }
        unsafe extern "system" fn get_Profiles<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_Profiles(this) {
                    Ok(ok__) => {
                        profiletypesbitmask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Profiles<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_Profiles(this, core::mem::transmute_copy(&profiletypesbitmask)).into()
            }
        }
        unsafe extern "system" fn get_EdgeTraversal<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_EdgeTraversal(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EdgeTraversal<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_EdgeTraversal(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn get_Action<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: *mut super::NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule_Impl::get_Action(this) {
                    Ok(ok__) => {
                        action.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Action<Identity: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: super::NET_FW_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule_Impl::put_Action(this, core::mem::transmute_copy(&action)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            put_Name: put_Name::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_ApplicationName: get_ApplicationName::<Identity, OFFSET>,
            put_ApplicationName: put_ApplicationName::<Identity, OFFSET>,
            get_ServiceName: get_ServiceName::<Identity, OFFSET>,
            put_ServiceName: put_ServiceName::<Identity, OFFSET>,
            get_Protocol: get_Protocol::<Identity, OFFSET>,
            put_Protocol: put_Protocol::<Identity, OFFSET>,
            get_LocalPorts: get_LocalPorts::<Identity, OFFSET>,
            put_LocalPorts: put_LocalPorts::<Identity, OFFSET>,
            get_RemotePorts: get_RemotePorts::<Identity, OFFSET>,
            put_RemotePorts: put_RemotePorts::<Identity, OFFSET>,
            get_LocalAddresses: get_LocalAddresses::<Identity, OFFSET>,
            put_LocalAddresses: put_LocalAddresses::<Identity, OFFSET>,
            get_RemoteAddresses: get_RemoteAddresses::<Identity, OFFSET>,
            put_RemoteAddresses: put_RemoteAddresses::<Identity, OFFSET>,
            get_IcmpTypesAndCodes: get_IcmpTypesAndCodes::<Identity, OFFSET>,
            put_IcmpTypesAndCodes: put_IcmpTypesAndCodes::<Identity, OFFSET>,
            get_Direction: get_Direction::<Identity, OFFSET>,
            put_Direction: put_Direction::<Identity, OFFSET>,
            get_Interfaces: get_Interfaces::<Identity, OFFSET>,
            put_Interfaces: put_Interfaces::<Identity, OFFSET>,
            get_InterfaceTypes: get_InterfaceTypes::<Identity, OFFSET>,
            put_InterfaceTypes: put_InterfaceTypes::<Identity, OFFSET>,
            get_Enabled: get_Enabled::<Identity, OFFSET>,
            put_Enabled: put_Enabled::<Identity, OFFSET>,
            get_Grouping: get_Grouping::<Identity, OFFSET>,
            put_Grouping: put_Grouping::<Identity, OFFSET>,
            get_Profiles: get_Profiles::<Identity, OFFSET>,
            put_Profiles: put_Profiles::<Identity, OFFSET>,
            get_EdgeTraversal: get_EdgeTraversal::<Identity, OFFSET>,
            put_EdgeTraversal: put_EdgeTraversal::<Identity, OFFSET>,
            get_Action: get_Action::<Identity, OFFSET>,
            put_Action: put_Action::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRule as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_EdgeTraversalOptions(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EdgeTraversalOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_EdgeTraversalOptions(&self, loptions: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EdgeTraversalOptions)(windows_core::Interface::as_raw(self), loptions) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule2_Vtbl {
    pub base__: INetFwRule_Vtbl,
    pub get_EdgeTraversalOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_EdgeTraversalOptions: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwRule2_Impl: INetFwRule_Impl {
    fn get_EdgeTraversalOptions(&self) -> windows_core::Result<i32>;
    fn put_EdgeTraversalOptions(&self, loptions: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwRule2_Vtbl {
    pub const fn new<Identity: INetFwRule2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_EdgeTraversalOptions<Identity: INetFwRule2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule2_Impl::get_EdgeTraversalOptions(this) {
                    Ok(ok__) => {
                        loptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EdgeTraversalOptions<Identity: INetFwRule2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule2_Impl::put_EdgeTraversalOptions(this, core::mem::transmute_copy(&loptions)).into()
            }
        }
        Self {
            base__: INetFwRule_Vtbl::new::<Identity, OFFSET>(),
            get_EdgeTraversalOptions: get_EdgeTraversalOptions::<Identity, OFFSET>,
            put_EdgeTraversalOptions: put_EdgeTraversalOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRule2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<INetFwRule as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_LocalAppPackageId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LocalAppPackageId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LocalAppPackageId(&self, wszpackageid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LocalAppPackageId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszpackageid)) }
    }
    pub unsafe fn get_LocalUserOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LocalUserOwner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LocalUserOwner(&self, wszuserowner: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LocalUserOwner)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszuserowner)) }
    }
    pub unsafe fn get_LocalUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LocalUserAuthorizedList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_LocalUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LocalUserAuthorizedList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszuserauthlist)) }
    }
    pub unsafe fn get_RemoteUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemoteUserAuthorizedList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RemoteUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RemoteUserAuthorizedList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszuserauthlist)) }
    }
    pub unsafe fn get_RemoteMachineAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemoteMachineAuthorizedList)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RemoteMachineAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RemoteMachineAuthorizedList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(wszuserauthlist)) }
    }
    pub unsafe fn get_SecureFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SecureFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_SecureFlags(&self, loptions: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SecureFlags)(windows_core::Interface::as_raw(self), loptions) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule3_Vtbl {
    pub base__: INetFwRule2_Vtbl,
    pub get_LocalAppPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LocalAppPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LocalUserOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LocalUserOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LocalUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_LocalUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RemoteUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RemoteUserAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RemoteMachineAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RemoteMachineAuthorizedList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SecureFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_SecureFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwRule3_Impl: INetFwRule2_Impl {
    fn get_LocalAppPackageId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LocalAppPackageId(&self, wszpackageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_LocalUserOwner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LocalUserOwner(&self, wszuserowner: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_LocalUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_LocalUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_RemoteUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RemoteUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_RemoteMachineAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RemoteMachineAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SecureFlags(&self) -> windows_core::Result<i32>;
    fn put_SecureFlags(&self, loptions: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwRule3_Vtbl {
    pub const fn new<Identity: INetFwRule3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_LocalAppPackageId<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpackageid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::get_LocalAppPackageId(this) {
                    Ok(ok__) => {
                        wszpackageid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LocalAppPackageId<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpackageid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::put_LocalAppPackageId(this, core::mem::transmute(&wszpackageid)).into()
            }
        }
        unsafe extern "system" fn get_LocalUserOwner<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserowner: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::get_LocalUserOwner(this) {
                    Ok(ok__) => {
                        wszuserowner.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LocalUserOwner<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserowner: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::put_LocalUserOwner(this, core::mem::transmute(&wszuserowner)).into()
            }
        }
        unsafe extern "system" fn get_LocalUserAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::get_LocalUserAuthorizedList(this) {
                    Ok(ok__) => {
                        wszuserauthlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LocalUserAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::put_LocalUserAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
            }
        }
        unsafe extern "system" fn get_RemoteUserAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::get_RemoteUserAuthorizedList(this) {
                    Ok(ok__) => {
                        wszuserauthlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RemoteUserAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::put_RemoteUserAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
            }
        }
        unsafe extern "system" fn get_RemoteMachineAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::get_RemoteMachineAuthorizedList(this) {
                    Ok(ok__) => {
                        wszuserauthlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RemoteMachineAuthorizedList<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::put_RemoteMachineAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
            }
        }
        unsafe extern "system" fn get_SecureFlags<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRule3_Impl::get_SecureFlags(this) {
                    Ok(ok__) => {
                        loptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SecureFlags<Identity: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwRule3_Impl::put_SecureFlags(this, core::mem::transmute_copy(&loptions)).into()
            }
        }
        Self {
            base__: INetFwRule2_Vtbl::new::<Identity, OFFSET>(),
            get_LocalAppPackageId: get_LocalAppPackageId::<Identity, OFFSET>,
            put_LocalAppPackageId: put_LocalAppPackageId::<Identity, OFFSET>,
            get_LocalUserOwner: get_LocalUserOwner::<Identity, OFFSET>,
            put_LocalUserOwner: put_LocalUserOwner::<Identity, OFFSET>,
            get_LocalUserAuthorizedList: get_LocalUserAuthorizedList::<Identity, OFFSET>,
            put_LocalUserAuthorizedList: put_LocalUserAuthorizedList::<Identity, OFFSET>,
            get_RemoteUserAuthorizedList: get_RemoteUserAuthorizedList::<Identity, OFFSET>,
            put_RemoteUserAuthorizedList: put_RemoteUserAuthorizedList::<Identity, OFFSET>,
            get_RemoteMachineAuthorizedList: get_RemoteMachineAuthorizedList::<Identity, OFFSET>,
            put_RemoteMachineAuthorizedList: put_RemoteMachineAuthorizedList::<Identity, OFFSET>,
            get_SecureFlags: get_SecureFlags::<Identity, OFFSET>,
            put_SecureFlags: put_SecureFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRule3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<INetFwRule as windows_core::Interface>::IID || iid == &<INetFwRule2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRules_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwRules_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, rule: windows_core::Ref<INetFwRule>) -> windows_core::Result<()>;
    fn Remove(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Item(&self, name: &windows_core::BSTR) -> windows_core::Result<INetFwRule>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwRules_Vtbl {
    pub const fn new<Identity: INetFwRules_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRules_Impl::get_Count(this) {
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
        unsafe extern "system" fn get__NewEnum<Identity: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwRules_Impl::get__NewEnum(this) {
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
            get_Count: get_Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRules as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_Type(&self) -> windows_core::Result<super::NET_FW_SERVICE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Customized(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Customized)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IpVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_IpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IpVersion)(windows_core::Interface::as_raw(self), ipversion) }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn get_Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn put_Scope(&self, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Scope)(windows_core::Interface::as_raw(self), scope) }
    }
    pub unsafe fn get_RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemoteAddresses)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RemoteAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(remoteaddrs)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Enabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    pub unsafe fn get_GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_GloballyOpenPorts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwService_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_SERVICE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_Type: usize,
    #[cfg(feature = "wtypes")]
    pub get_Customized: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Customized: usize,
    #[cfg(feature = "icftypes")]
    pub get_IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_IpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub put_IpVersion: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_IP_VERSION) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_IpVersion: usize,
    #[cfg(feature = "icftypes")]
    pub get_Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    get_Scope: usize,
    #[cfg(feature = "icftypes")]
    pub put_Scope: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_SCOPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    put_Scope: usize,
    pub get_RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RemoteAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Enabled: usize,
    pub get_GloballyOpenPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwService_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Type(&self) -> windows_core::Result<super::NET_FW_SERVICE_TYPE>;
    fn get_Customized(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_IpVersion(&self) -> windows_core::Result<super::NET_FW_IP_VERSION>;
    fn put_IpVersion(&self, ipversion: super::NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn get_Scope(&self) -> windows_core::Result<super::NET_FW_SCOPE>;
    fn put_Scope(&self, scope: super::NET_FW_SCOPE) -> windows_core::Result<()>;
    fn get_RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwService_Vtbl {
    pub const fn new<Identity: INetFwService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::get_Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Type<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut super::NET_FW_SERVICE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::get_Type(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Customized<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customized: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::get_Customized(this) {
                    Ok(ok__) => {
                        customized.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IpVersion<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::get_IpVersion(this) {
                    Ok(ok__) => {
                        ipversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IpVersion<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: super::NET_FW_IP_VERSION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwService_Impl::put_IpVersion(this, core::mem::transmute_copy(&ipversion)).into()
            }
        }
        unsafe extern "system" fn get_Scope<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::get_Scope(this) {
                    Ok(ok__) => {
                        scope.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Scope<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: super::NET_FW_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwService_Impl::put_Scope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn get_RemoteAddresses<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::get_RemoteAddresses(this) {
                    Ok(ok__) => {
                        remoteaddrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RemoteAddresses<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwService_Impl::put_RemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
            }
        }
        unsafe extern "system" fn get_Enabled<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::get_Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Enabled<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetFwService_Impl::put_Enabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn get_GloballyOpenPorts<Identity: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, openports: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwService_Impl::get_GloballyOpenPorts(this) {
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
            get_Name: get_Name::<Identity, OFFSET>,
            get_Type: get_Type::<Identity, OFFSET>,
            get_Customized: get_Customized::<Identity, OFFSET>,
            get_IpVersion: get_IpVersion::<Identity, OFFSET>,
            put_IpVersion: put_IpVersion::<Identity, OFFSET>,
            get_Scope: get_Scope::<Identity, OFFSET>,
            put_Scope: put_Scope::<Identity, OFFSET>,
            get_RemoteAddresses: get_RemoteAddresses::<Identity, OFFSET>,
            put_RemoteAddresses: put_RemoteAddresses::<Identity, OFFSET>,
            get_Enabled: get_Enabled::<Identity, OFFSET>,
            put_Enabled: put_Enabled::<Identity, OFFSET>,
            get_GloballyOpenPorts: get_GloballyOpenPorts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwService as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Rules(&self) -> windows_core::Result<INetFwRules> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Rules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub get_Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwServiceRestriction_Impl: super::IDispatch_Impl {
    fn RestrictService(&self, servicename: &windows_core::BSTR, appname: &windows_core::BSTR, restrictservice: super::VARIANT_BOOL, servicesidrestricted: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ServiceRestricted(&self, servicename: &windows_core::BSTR, appname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Rules(&self) -> windows_core::Result<INetFwRules>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn get_Rules<Identity: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServiceRestriction_Impl::get_Rules(this) {
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
            get_Rules: get_Rules::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwServiceRestriction as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "icftypes")]
    pub unsafe fn Item(&self, svctype: super::NET_FW_SERVICE_TYPE) -> windows_core::Result<INetFwService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), svctype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServices_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "icftypes")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::NET_FW_SERVICE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "icftypes"))]
    Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetFwServices_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, svctype: super::NET_FW_SERVICE_TYPE) -> windows_core::Result<INetFwService>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetFwServices_Vtbl {
    pub const fn new<Identity: INetFwServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: INetFwServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServices_Impl::get_Count(this) {
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
        unsafe extern "system" fn get__NewEnum<Identity: INetFwServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetFwServices_Impl::get__NewEnum(this) {
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
            get_Count: get_Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwServices as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "icftypes", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PAC_CHANGES_CALLBACK_FN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE)>;
#[cfg(feature = "shared")]
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddress: PFW_DYNAMIC_KEYWORD_ADDRESS0) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_core::GUID) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_core::GUID, dynamickeywordaddressdata: *mut PFW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = Option<unsafe extern "system" fn(flags: super::DWORD, dynamickeywordaddressdata: *mut PFW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = Option<unsafe extern "system" fn(dynamickeywordaddressdata: PFW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_core::GUID, updatedaddresses: windows_core::PCWSTR, append: windows_core::BOOL) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PFW_DYNAMIC_KEYWORD_ADDRESS0 = *mut FW_DYNAMIC_KEYWORD_ADDRESS0;
#[cfg(feature = "shared")]
pub type PFW_DYNAMIC_KEYWORD_ADDRESS_DATA0 = *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0;
#[cfg(feature = "shared")]
pub type PINET_FIREWALL_AC_BINARIES = *mut INET_FIREWALL_AC_BINARIES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PINET_FIREWALL_AC_CAPABILITIES = *mut INET_FIREWALL_AC_CAPABILITIES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PINET_FIREWALL_AC_CHANGE = *mut INET_FIREWALL_AC_CHANGE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PINET_FIREWALL_APP_CONTAINER = *mut INET_FIREWALL_APP_CONTAINER;
#[cfg(feature = "shared")]
pub type PNETISO_EDP_ID_CALLBACK_FN = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, wszenterpriseid: windows_core::PCWSTR, dwerr: super::DWORD)>;

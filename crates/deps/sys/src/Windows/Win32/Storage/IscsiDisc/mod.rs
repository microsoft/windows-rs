#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddISNSServerA(address: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddISNSServerW(address: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiConnectionA(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, reserved: *mut ::core::ffi::c_void, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALA, securityflags: u64, loginoptions: *mut ISCSI_LOGIN_OPTIONS, keysize: u32, key: super::super::Foundation::PSTR, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiConnectionW(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, reserved: *mut ::core::ffi::c_void, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALW, securityflags: u64, loginoptions: *mut ISCSI_LOGIN_OPTIONS, keysize: u32, key: super::super::Foundation::PSTR, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiSendTargetPortalA(initiatorinstance: super::super::Foundation::PSTR, initiatorportnumber: u32, loginoptions: *mut ISCSI_LOGIN_OPTIONS, securityflags: u64, portal: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiSendTargetPortalW(initiatorinstance: super::super::Foundation::PWSTR, initiatorportnumber: u32, loginoptions: *mut ISCSI_LOGIN_OPTIONS, securityflags: u64, portal: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiStaticTargetA(targetname: super::super::Foundation::PSTR, targetalias: super::super::Foundation::PSTR, targetflags: u32, persist: super::super::Foundation::BOOLEAN, mappings: *mut ISCSI_TARGET_MAPPINGA, loginoptions: *mut ISCSI_LOGIN_OPTIONS, portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPA) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiStaticTargetW(targetname: super::super::Foundation::PWSTR, targetalias: super::super::Foundation::PWSTR, targetflags: u32, persist: super::super::Foundation::BOOLEAN, mappings: *mut ISCSI_TARGET_MAPPINGW, loginoptions: *mut ISCSI_LOGIN_OPTIONS, portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPW) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPersistentIScsiDeviceA(devicepath: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPersistentIScsiDeviceW(devicepath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddRadiusServerA(address: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddRadiusServerW(address: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn ClearPersistentIScsiDevices() -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`, `Win32_System_Ioctl`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ioctl"))]
    pub fn GetDevicesForIScsiSessionA(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, devicecount: *mut u32, devices: *mut ISCSI_DEVICE_ON_SESSIONA) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_System_Ioctl`*"]
    #[cfg(feature = "Win32_System_Ioctl")]
    pub fn GetDevicesForIScsiSessionW(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, devicecount: *mut u32, devices: *mut ISCSI_DEVICE_ON_SESSIONW) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiIKEInfoA(initiatorname: super::super::Foundation::PSTR, initiatorportnumber: u32, reserved: *mut u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiIKEInfoW(initiatorname: super::super::Foundation::PWSTR, initiatorportnumber: u32, reserved: *mut u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiInitiatorNodeNameA(initiatornodename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiInitiatorNodeNameW(initiatornodename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiSessionListA(buffersize: *mut u32, sessioncount: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFOA) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiSessionListEx(buffersize: *mut u32, sessioncountptr: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFO_EX) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiSessionListW(buffersize: *mut u32, sessioncount: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFOW) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiTargetInformationA(targetname: super::super::Foundation::PSTR, discoverymechanism: super::super::Foundation::PSTR, infoclass: TARGET_INFORMATION_CLASS, buffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiTargetInformationW(targetname: super::super::Foundation::PWSTR, discoverymechanism: super::super::Foundation::PWSTR, infoclass: TARGET_INFORMATION_CLASS, buffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn GetIScsiVersionInformation(versioninfo: *mut ISCSI_VERSION_INFO) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoginIScsiTargetA(
        targetname: super::super::Foundation::PSTR,
        isinformationalsession: super::super::Foundation::BOOLEAN,
        initiatorinstance: super::super::Foundation::PSTR,
        initiatorportnumber: u32,
        targetportal: *mut ISCSI_TARGET_PORTALA,
        securityflags: u64,
        mappings: *mut ISCSI_TARGET_MAPPINGA,
        loginoptions: *mut ISCSI_LOGIN_OPTIONS,
        keysize: u32,
        key: super::super::Foundation::PSTR,
        ispersistent: super::super::Foundation::BOOLEAN,
        uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
        uniqueconnectionid: *mut ISCSI_UNIQUE_SESSION_ID,
    ) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoginIScsiTargetW(
        targetname: super::super::Foundation::PWSTR,
        isinformationalsession: super::super::Foundation::BOOLEAN,
        initiatorinstance: super::super::Foundation::PWSTR,
        initiatorportnumber: u32,
        targetportal: *mut ISCSI_TARGET_PORTALW,
        securityflags: u64,
        mappings: *mut ISCSI_TARGET_MAPPINGW,
        loginoptions: *mut ISCSI_LOGIN_OPTIONS,
        keysize: u32,
        key: super::super::Foundation::PSTR,
        ispersistent: super::super::Foundation::BOOLEAN,
        uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
        uniqueconnectionid: *mut ISCSI_UNIQUE_SESSION_ID,
    ) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn LogoutIScsiTarget(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshISNSServerA(address: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshISNSServerW(address: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshIScsiSendTargetPortalA(initiatorinstance: super::super::Foundation::PSTR, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshIScsiSendTargetPortalW(initiatorinstance: super::super::Foundation::PWSTR, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveISNSServerA(address: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveISNSServerW(address: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn RemoveIScsiConnection(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiPersistentTargetA(initiatorinstance: super::super::Foundation::PSTR, initiatorportnumber: u32, targetname: super::super::Foundation::PSTR, portal: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiPersistentTargetW(initiatorinstance: super::super::Foundation::PWSTR, initiatorportnumber: u32, targetname: super::super::Foundation::PWSTR, portal: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiSendTargetPortalA(initiatorinstance: super::super::Foundation::PSTR, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiSendTargetPortalW(initiatorinstance: super::super::Foundation::PWSTR, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiStaticTargetA(targetname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiStaticTargetW(targetname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePersistentIScsiDeviceA(devicepath: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePersistentIScsiDeviceW(devicepath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveRadiusServerA(address: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveRadiusServerW(address: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportActiveIScsiTargetMappingsA(buffersize: *mut u32, mappingcount: *mut u32, mappings: *mut ISCSI_TARGET_MAPPINGA) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn ReportActiveIScsiTargetMappingsW(buffersize: *mut u32, mappingcount: *mut u32, mappings: *mut ISCSI_TARGET_MAPPINGW) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportISNSServerListA(buffersizeinchar: *mut u32, buffer: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportISNSServerListW(buffersizeinchar: *mut u32, buffer: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiInitiatorListA(buffersize: *mut u32, buffer: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiInitiatorListW(buffersize: *mut u32, buffer: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiPersistentLoginsA(count: *mut u32, persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOA, buffersizeinbytes: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiPersistentLoginsW(count: *mut u32, persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOW, buffersizeinbytes: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiSendTargetPortalsA(portalcount: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFOA) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiSendTargetPortalsExA(portalcount: *mut u32, portalinfosize: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXA) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn ReportIScsiSendTargetPortalsExW(portalcount: *mut u32, portalinfosize: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXW) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn ReportIScsiSendTargetPortalsW(portalcount: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFOW) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetPortalsA(initiatorname: super::super::Foundation::PSTR, targetname: super::super::Foundation::PSTR, targetportaltag: *mut u16, elementcount: *mut u32, portals: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetPortalsW(initiatorname: super::super::Foundation::PWSTR, targetname: super::super::Foundation::PWSTR, targetportaltag: *mut u16, elementcount: *mut u32, portals: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetsA(forceupdate: super::super::Foundation::BOOLEAN, buffersize: *mut u32, buffer: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetsW(forceupdate: super::super::Foundation::BOOLEAN, buffersize: *mut u32, buffer: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportPersistentIScsiDevicesA(buffersizeinchar: *mut u32, buffer: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportPersistentIScsiDevicesW(buffersizeinchar: *mut u32, buffer: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportRadiusServerListA(buffersizeinchar: *mut u32, buffer: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportRadiusServerListW(buffersizeinchar: *mut u32, buffer: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SendScsiInquiry(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, lun: u64, evpdcmddt: u8, pagecode: u8, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SendScsiReadCapacity(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, lun: u64, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SendScsiReportLuns(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiGroupPresharedKey(keylength: u32, key: *mut u8, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiIKEInfoA(initiatorname: super::super::Foundation::PSTR, initiatorportnumber: u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiIKEInfoW(initiatorname: super::super::Foundation::PWSTR, initiatorportnumber: u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SetIScsiInitiatorCHAPSharedSecret(sharedsecretlength: u32, sharedsecret: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiInitiatorNodeNameA(initiatornodename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiInitiatorNodeNameW(initiatornodename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SetIScsiInitiatorRADIUSSharedSecret(sharedsecretlength: u32, sharedsecret: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiTunnelModeOuterAddressA(initiatorname: super::super::Foundation::PSTR, initiatorportnumber: u32, destinationaddress: super::super::Foundation::PSTR, outermodeaddress: super::super::Foundation::PSTR, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiTunnelModeOuterAddressW(initiatorname: super::super::Foundation::PWSTR, initiatorportnumber: u32, destinationaddress: super::super::Foundation::PWSTR, outermodeaddress: super::super::Foundation::PWSTR, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SetupPersistentIScsiDevices() -> u32;
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SetupPersistentIScsiVolumes() -> u32;
}

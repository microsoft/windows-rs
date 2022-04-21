#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn AddISNSServerA(address: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn AddISNSServerW(address: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiConnectionA(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, reserved: *mut ::core::ffi::c_void, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALA, securityflags: u64, loginoptions: *mut ISCSI_LOGIN_OPTIONS, keysize: u32, key: ::windows_sys::core::PCSTR, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn AddIScsiConnectionW(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, reserved: *mut ::core::ffi::c_void, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALW, securityflags: u64, loginoptions: *mut ISCSI_LOGIN_OPTIONS, keysize: u32, key: ::windows_sys::core::PCSTR, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiSendTargetPortalA(initiatorinstance: ::windows_sys::core::PCSTR, initiatorportnumber: u32, loginoptions: *mut ISCSI_LOGIN_OPTIONS, securityflags: u64, portal: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn AddIScsiSendTargetPortalW(initiatorinstance: ::windows_sys::core::PCWSTR, initiatorportnumber: u32, loginoptions: *mut ISCSI_LOGIN_OPTIONS, securityflags: u64, portal: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiStaticTargetA(targetname: ::windows_sys::core::PCSTR, targetalias: ::windows_sys::core::PCSTR, targetflags: u32, persist: super::super::Foundation::BOOLEAN, mappings: *mut ISCSI_TARGET_MAPPINGA, loginoptions: *mut ISCSI_LOGIN_OPTIONS, portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPA) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiStaticTargetW(targetname: ::windows_sys::core::PCWSTR, targetalias: ::windows_sys::core::PCWSTR, targetflags: u32, persist: super::super::Foundation::BOOLEAN, mappings: *mut ISCSI_TARGET_MAPPINGW, loginoptions: *mut ISCSI_LOGIN_OPTIONS, portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPW) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn AddPersistentIScsiDeviceA(devicepath: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn AddPersistentIScsiDeviceW(devicepath: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn AddRadiusServerA(address: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn AddRadiusServerW(address: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ClearPersistentIScsiDevices() -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ioctl\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ioctl"))]
    pub fn GetDevicesForIScsiSessionA(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, devicecount: *mut u32, devices: *mut ISCSI_DEVICE_ON_SESSIONA) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_System_Ioctl\"`*"]
    #[cfg(feature = "Win32_System_Ioctl")]
    pub fn GetDevicesForIScsiSessionW(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, devicecount: *mut u32, devices: *mut ISCSI_DEVICE_ON_SESSIONW) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn GetIScsiIKEInfoA(initiatorname: ::windows_sys::core::PCSTR, initiatorportnumber: u32, reserved: *mut u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn GetIScsiIKEInfoW(initiatorname: ::windows_sys::core::PCWSTR, initiatorportnumber: u32, reserved: *mut u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn GetIScsiInitiatorNodeNameA(initiatornodename: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn GetIScsiInitiatorNodeNameW(initiatornodename: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn GetIScsiSessionListA(buffersize: *mut u32, sessioncount: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFOA) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiSessionListEx(buffersize: *mut u32, sessioncountptr: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFO_EX) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn GetIScsiSessionListW(buffersize: *mut u32, sessioncount: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFOW) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn GetIScsiTargetInformationA(targetname: ::windows_sys::core::PCSTR, discoverymechanism: ::windows_sys::core::PCSTR, infoclass: TARGET_INFORMATION_CLASS, buffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn GetIScsiTargetInformationW(targetname: ::windows_sys::core::PCWSTR, discoverymechanism: ::windows_sys::core::PCWSTR, infoclass: TARGET_INFORMATION_CLASS, buffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn GetIScsiVersionInformation(versioninfo: *mut ISCSI_VERSION_INFO) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoginIScsiTargetA(targetname: ::windows_sys::core::PCSTR, isinformationalsession: super::super::Foundation::BOOLEAN, initiatorinstance: ::windows_sys::core::PCSTR, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALA, securityflags: u64, mappings: *mut ISCSI_TARGET_MAPPINGA, loginoptions: *mut ISCSI_LOGIN_OPTIONS, keysize: u32, key: ::windows_sys::core::PCSTR, ispersistent: super::super::Foundation::BOOLEAN, uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, uniqueconnectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoginIScsiTargetW(targetname: ::windows_sys::core::PCWSTR, isinformationalsession: super::super::Foundation::BOOLEAN, initiatorinstance: ::windows_sys::core::PCWSTR, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALW, securityflags: u64, mappings: *mut ISCSI_TARGET_MAPPINGW, loginoptions: *mut ISCSI_LOGIN_OPTIONS, keysize: u32, key: ::windows_sys::core::PCSTR, ispersistent: super::super::Foundation::BOOLEAN, uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, uniqueconnectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn LogoutIScsiTarget(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RefreshISNSServerA(address: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RefreshISNSServerW(address: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshIScsiSendTargetPortalA(initiatorinstance: ::windows_sys::core::PCSTR, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RefreshIScsiSendTargetPortalW(initiatorinstance: ::windows_sys::core::PCWSTR, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RemoveISNSServerA(address: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RemoveISNSServerW(address: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RemoveIScsiConnection(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiPersistentTargetA(initiatorinstance: ::windows_sys::core::PCSTR, initiatorportnumber: u32, targetname: ::windows_sys::core::PCSTR, portal: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RemoveIScsiPersistentTargetW(initiatorinstance: ::windows_sys::core::PCWSTR, initiatorportnumber: u32, targetname: ::windows_sys::core::PCWSTR, portal: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiSendTargetPortalA(initiatorinstance: ::windows_sys::core::PCSTR, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RemoveIScsiSendTargetPortalW(initiatorinstance: ::windows_sys::core::PCWSTR, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RemoveIScsiStaticTargetA(targetname: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RemoveIScsiStaticTargetW(targetname: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RemovePersistentIScsiDeviceA(devicepath: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RemovePersistentIScsiDeviceW(devicepath: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RemoveRadiusServerA(address: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn RemoveRadiusServerW(address: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportActiveIScsiTargetMappingsA(buffersize: *mut u32, mappingcount: *mut u32, mappings: *mut ISCSI_TARGET_MAPPINGA) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ReportActiveIScsiTargetMappingsW(buffersize: *mut u32, mappingcount: *mut u32, mappings: *mut ISCSI_TARGET_MAPPINGW) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ReportISNSServerListA(buffersizeinchar: *mut u32, buffer: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ReportISNSServerListW(buffersizeinchar: *mut u32, buffer: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ReportIScsiInitiatorListA(buffersize: *mut u32, buffer: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ReportIScsiInitiatorListW(buffersize: *mut u32, buffer: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiPersistentLoginsA(count: *mut u32, persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOA, buffersizeinbytes: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiPersistentLoginsW(count: *mut u32, persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOW, buffersizeinbytes: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiSendTargetPortalsA(portalcount: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFOA) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiSendTargetPortalsExA(portalcount: *mut u32, portalinfosize: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXA) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ReportIScsiSendTargetPortalsExW(portalcount: *mut u32, portalinfosize: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXW) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ReportIScsiSendTargetPortalsW(portalcount: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFOW) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetPortalsA(initiatorname: ::windows_sys::core::PCSTR, targetname: ::windows_sys::core::PCSTR, targetportaltag: *mut u16, elementcount: *mut u32, portals: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ReportIScsiTargetPortalsW(initiatorname: ::windows_sys::core::PCWSTR, targetname: ::windows_sys::core::PCWSTR, targetportaltag: *mut u16, elementcount: *mut u32, portals: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetsA(forceupdate: super::super::Foundation::BOOLEAN, buffersize: *mut u32, buffer: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetsW(forceupdate: super::super::Foundation::BOOLEAN, buffersize: *mut u32, buffer: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ReportPersistentIScsiDevicesA(buffersizeinchar: *mut u32, buffer: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ReportPersistentIScsiDevicesW(buffersizeinchar: *mut u32, buffer: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ReportRadiusServerListA(buffersizeinchar: *mut u32, buffer: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn ReportRadiusServerListW(buffersizeinchar: *mut u32, buffer: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn SendScsiInquiry(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, lun: u64, evpdcmddt: u8, pagecode: u8, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn SendScsiReadCapacity(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, lun: u64, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn SendScsiReportLuns(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiGroupPresharedKey(keylength: u32, key: *mut u8, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiIKEInfoA(initiatorname: ::windows_sys::core::PCSTR, initiatorportnumber: u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiIKEInfoW(initiatorname: ::windows_sys::core::PCWSTR, initiatorportnumber: u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn SetIScsiInitiatorCHAPSharedSecret(sharedsecretlength: u32, sharedsecret: *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn SetIScsiInitiatorNodeNameA(initiatornodename: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn SetIScsiInitiatorNodeNameW(initiatornodename: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn SetIScsiInitiatorRADIUSSharedSecret(sharedsecretlength: u32, sharedsecret: *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiTunnelModeOuterAddressA(initiatorname: ::windows_sys::core::PCSTR, initiatorportnumber: u32, destinationaddress: ::windows_sys::core::PCSTR, outermodeaddress: ::windows_sys::core::PCSTR, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiTunnelModeOuterAddressW(initiatorname: ::windows_sys::core::PCWSTR, initiatorportnumber: u32, destinationaddress: ::windows_sys::core::PCWSTR, outermodeaddress: ::windows_sys::core::PCWSTR, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn SetupPersistentIScsiDevices() -> u32;
    #[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
    pub fn SetupPersistentIScsiVolumes() -> u32;
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ATA_FLAGS_48BIT_COMMAND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ATA_FLAGS_DATA_IN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ATA_FLAGS_DATA_OUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ATA_FLAGS_DRDY_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ATA_FLAGS_NO_MULTIPLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ATA_FLAGS_USE_DMA: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ATA_PASS_THROUGH_DIRECT {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBuffer: *mut ::core::ffi::c_void,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
impl ::core::marker::Copy for ATA_PASS_THROUGH_DIRECT {}
impl ::core::clone::Clone for ATA_PASS_THROUGH_DIRECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct ATA_PASS_THROUGH_DIRECT32 {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBuffer: *mut ::core::ffi::c_void,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for ATA_PASS_THROUGH_DIRECT32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for ATA_PASS_THROUGH_DIRECT32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ATA_PASS_THROUGH_EX {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBufferOffset: usize,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
impl ::core::marker::Copy for ATA_PASS_THROUGH_EX {}
impl ::core::clone::Clone for ATA_PASS_THROUGH_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct ATA_PASS_THROUGH_EX32 {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBufferOffset: u32,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for ATA_PASS_THROUGH_EX32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for ATA_PASS_THROUGH_EX32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DD_SCSI_DEVICE_NAME: &str = "\\Device\\ScsiPort";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct DSM_NOTIFICATION_REQUEST_BLOCK {
    pub Size: u32,
    pub Version: u32,
    pub NotifyFlags: u32,
    pub DataSetProfile: u32,
    pub Reserved: [u32; 3],
    pub DataSetRangesCount: u32,
    pub DataSetRanges: [MP_DEVICE_DATA_SET_RANGE; 1],
}
impl ::core::marker::Copy for DSM_NOTIFICATION_REQUEST_BLOCK {}
impl ::core::clone::Clone for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct DUMP_DRIVER {
    pub DumpDriverList: *mut ::core::ffi::c_void,
    pub DriverName: [u16; 15],
    pub BaseName: [u16; 15],
}
impl ::core::marker::Copy for DUMP_DRIVER {}
impl ::core::clone::Clone for DUMP_DRIVER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct DUMP_DRIVER_EX {
    pub DumpDriverList: *mut ::core::ffi::c_void,
    pub DriverName: [u16; 15],
    pub BaseName: [u16; 15],
    pub DriverFullPath: NTSCSI_UNICODE_STRING,
}
impl ::core::marker::Copy for DUMP_DRIVER_EX {}
impl ::core::clone::Clone for DUMP_DRIVER_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_DRIVER_NAME_LENGTH: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_EX_FLAG_DRIVER_FULL_PATH_SUPPORT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_EX_FLAG_RESUME_SUPPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_EX_FLAG_SUPPORT_64BITMEMORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_EX_FLAG_SUPPORT_DD_TELEMETRY: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DUMP_POINTERS {
    pub AdapterObject: *mut _ADAPTER_OBJECT,
    pub MappedRegisterBase: *mut ::core::ffi::c_void,
    pub DumpData: *mut ::core::ffi::c_void,
    pub CommonBufferVa: *mut ::core::ffi::c_void,
    pub CommonBufferPa: i64,
    pub CommonBufferSize: u32,
    pub AllocateCommonBuffers: super::super::Foundation::BOOLEAN,
    pub UseDiskDump: super::super::Foundation::BOOLEAN,
    pub Spare1: [u8; 2],
    pub DeviceObject: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUMP_POINTERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUMP_POINTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DUMP_POINTERS_EX {
    pub Header: DUMP_POINTERS_VERSION,
    pub DumpData: *mut ::core::ffi::c_void,
    pub CommonBufferVa: *mut ::core::ffi::c_void,
    pub CommonBufferSize: u32,
    pub AllocateCommonBuffers: super::super::Foundation::BOOLEAN,
    pub DeviceObject: *mut ::core::ffi::c_void,
    pub DriverList: *mut ::core::ffi::c_void,
    pub dwPortFlags: u32,
    pub MaxDeviceDumpSectionSize: u32,
    pub MaxDeviceDumpLevel: u32,
    pub MaxTransferSize: u32,
    pub AdapterObject: *mut ::core::ffi::c_void,
    pub MappedRegisterBase: *mut ::core::ffi::c_void,
    pub DeviceReady: *mut super::super::Foundation::BOOLEAN,
    pub DumpDevicePowerOn: PDUMP_DEVICE_POWERON_ROUTINE,
    pub DumpDevicePowerOnContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUMP_POINTERS_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUMP_POINTERS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct DUMP_POINTERS_VERSION {
    pub Version: u32,
    pub Size: u32,
}
impl ::core::marker::Copy for DUMP_POINTERS_VERSION {}
impl ::core::clone::Clone for DUMP_POINTERS_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_POINTERS_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_POINTERS_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_POINTERS_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_POINTERS_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FILE_DEVICE_SCSI: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_FUNCTION_ACTIVATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_FUNCTION_DOWNLOAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_FUNCTION_GET_INFO: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct FIRMWARE_REQUEST_BLOCK {
    pub Version: u32,
    pub Size: u32,
    pub Function: u32,
    pub Flags: u32,
    pub DataBufferOffset: u32,
    pub DataBufferLength: u32,
}
impl ::core::marker::Copy for FIRMWARE_REQUEST_BLOCK {}
impl ::core::clone::Clone for FIRMWARE_REQUEST_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_REQUEST_FLAG_CONTROLLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_COMMAND_ABORT: u32 = 133u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_CONTROLLER_ERROR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_DEVICE_ERROR: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_END_OF_MEDIA: u32 = 134u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_ID_NOT_FOUND: u32 = 131u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_ILLEGAL_LENGTH: u32 = 135u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_ILLEGAL_REQUEST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_INPUT_BUFFER_TOO_BIG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_INTERFACE_CRC_ERROR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_INVALID_IMAGE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_INVALID_PARAMETER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_INVALID_SLOT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_MEDIA_CHANGE: u32 = 130u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_MEDIA_CHANGE_REQUEST: u32 = 132u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_POWER_CYCLE_REQUIRED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_UNCORRECTABLE_DATA_ERROR: u32 = 129u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct HYBRID_DEMOTE_BY_SIZE {
    pub Version: u32,
    pub Size: u32,
    pub SourcePriority: u8,
    pub TargetPriority: u8,
    pub Reserved0: u16,
    pub Reserved1: u32,
    pub LbaCount: u64,
}
impl ::core::marker::Copy for HYBRID_DEMOTE_BY_SIZE {}
impl ::core::clone::Clone for HYBRID_DEMOTE_BY_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct HYBRID_DIRTY_THRESHOLDS {
    pub Version: u32,
    pub Size: u32,
    pub DirtyLowThreshold: u32,
    pub DirtyHighThreshold: u32,
}
impl ::core::marker::Copy for HYBRID_DIRTY_THRESHOLDS {}
impl ::core::clone::Clone for HYBRID_DIRTY_THRESHOLDS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_FUNCTION_DEMOTE_BY_SIZE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_FUNCTION_DISABLE_CACHING_MEDIUM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_FUNCTION_ENABLE_CACHING_MEDIUM: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_FUNCTION_GET_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_FUNCTION_SET_DIRTY_THRESHOLD: u32 = 18u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HYBRID_INFORMATION {
    pub Version: u32,
    pub Size: u32,
    pub HybridSupported: super::super::Foundation::BOOLEAN,
    pub Status: NVCACHE_STATUS,
    pub CacheTypeEffective: NVCACHE_TYPE,
    pub CacheTypeDefault: NVCACHE_TYPE,
    pub FractionBase: u32,
    pub CacheSize: u64,
    pub Attributes: HYBRID_INFORMATION_0,
    pub Priorities: HYBRID_INFORMATION_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HYBRID_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HYBRID_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HYBRID_INFORMATION_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HYBRID_INFORMATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HYBRID_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HYBRID_INFORMATION_1 {
    pub PriorityLevelCount: u8,
    pub MaxPriorityBehavior: super::super::Foundation::BOOLEAN,
    pub OptimalWriteGranularity: u8,
    pub Reserved: u8,
    pub DirtyThresholdLow: u32,
    pub DirtyThresholdHigh: u32,
    pub SupportedCommands: HYBRID_INFORMATION_1_0,
    pub Priority: [NVCACHE_PRIORITY_LEVEL_DESCRIPTOR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HYBRID_INFORMATION_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HYBRID_INFORMATION_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HYBRID_INFORMATION_1_0 {
    pub _bitfield: u32,
    pub MaxEvictCommands: u32,
    pub MaxLbaRangeCountForEvict: u32,
    pub MaxLbaRangeCountForChangeLba: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HYBRID_INFORMATION_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HYBRID_INFORMATION_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct HYBRID_REQUEST_BLOCK {
    pub Version: u32,
    pub Size: u32,
    pub Function: u32,
    pub Flags: u32,
    pub DataBufferOffset: u32,
    pub DataBufferLength: u32,
}
impl ::core::marker::Copy for HYBRID_REQUEST_BLOCK {}
impl ::core::clone::Clone for HYBRID_REQUEST_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_REQUEST_INFO_STRUCTURE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_STATUS_ENABLE_REFCOUNT_HOLD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_STATUS_ILLEGAL_REQUEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_STATUS_INVALID_PARAMETER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_STATUS_SUCCESS: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct IDE_IO_CONTROL {
    pub HeaderLength: u32,
    pub Signature: [u8; 8],
    pub Timeout: u32,
    pub ControlCode: u32,
    pub ReturnStatus: u32,
    pub DataLength: u32,
}
impl ::core::marker::Copy for IDE_IO_CONTROL {}
impl ::core::clone::Clone for IDE_IO_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ID_FQDN: &str = "2";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ID_IPV4_ADDR: &str = "1";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ID_IPV6_ADDR: &str = "5";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ID_USER_FQDN: &str = "3";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct IKE_AUTHENTICATION_INFORMATION {
    pub AuthMethod: IKE_AUTHENTICATION_METHOD,
    pub Anonymous: IKE_AUTHENTICATION_INFORMATION_0,
}
impl ::core::marker::Copy for IKE_AUTHENTICATION_INFORMATION {}
impl ::core::clone::Clone for IKE_AUTHENTICATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub union IKE_AUTHENTICATION_INFORMATION_0 {
    pub PsKey: IKE_AUTHENTICATION_PRESHARED_KEY,
}
impl ::core::marker::Copy for IKE_AUTHENTICATION_INFORMATION_0 {}
impl ::core::clone::Clone for IKE_AUTHENTICATION_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub type IKE_AUTHENTICATION_METHOD = i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IKE_AUTHENTICATION_PRESHARED_KEY_METHOD: IKE_AUTHENTICATION_METHOD = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct IKE_AUTHENTICATION_PRESHARED_KEY {
    pub SecurityFlags: u64,
    pub IdType: u8,
    pub IdLengthInBytes: u32,
    pub Id: *mut u8,
    pub KeyLengthInBytes: u32,
    pub Key: *mut u8,
}
impl ::core::marker::Copy for IKE_AUTHENTICATION_PRESHARED_KEY {}
impl ::core::clone::Clone for IKE_AUTHENTICATION_PRESHARED_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_ATA_MINIPORT: u32 = 315444u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_ATA_PASS_THROUGH: u32 = 315436u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_ATA_PASS_THROUGH_DIRECT: u32 = 315440u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_IDE_PASS_THROUGH: u32 = 315432u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_PROCESS_SERVICE_IRP: u32 = 315448u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_DSM_GENERAL: &str = "MPDSMGEN";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_DSM_NOTIFICATION: &str = "MPDSM   ";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_ENDURANCE_INFO: &str = "ENDURINF";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_FIRMWARE: &str = "FIRMWARE";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_HYBRDISK: &str = "HYBRDISK";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_PHYSICAL_TOPOLOGY: &str = "TOPOLOGY";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_PROTOCOL: &str = "PROTOCOL";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_TEMPERATURE: &str = "TEMPERAT";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_SCSIDISK: &str = "SCSIDISK";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_SET_PROTOCOL: &str = "SETPROTO";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_SET_TEMPERATURE_THRESHOLD: &str = "SETTEMPT";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MPIO_PASS_THROUGH_PATH: u32 = 315452u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT: u32 = 315456u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT_EX: u32 = 315472u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MPIO_PASS_THROUGH_PATH_EX: u32 = 315468u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_BASE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_FREE_DUMP_POINTERS: u32 = 266276u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_GET_ADDRESS: u32 = 266264u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_GET_CAPABILITIES: u32 = 266256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_GET_DUMP_POINTERS: u32 = 266272u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_GET_INQUIRY_DATA: u32 = 266252u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_MINIPORT: u32 = 315400u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_PASS_THROUGH: u32 = 315396u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT: u32 = 315412u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT_EX: u32 = 315464u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_PASS_THROUGH_EX: u32 = 315460u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_RESCAN_BUS: u32 = 266268u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IO_SCSI_CAPABILITIES {
    pub Length: u32,
    pub MaximumTransferLength: u32,
    pub MaximumPhysicalPages: u32,
    pub SupportedAsynchronousEvents: u32,
    pub AlignmentMask: u32,
    pub TaggedQueuing: super::super::Foundation::BOOLEAN,
    pub AdapterScansDown: super::super::Foundation::BOOLEAN,
    pub AdapterUsesPio: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IO_SCSI_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IO_SCSI_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub type ISCSI_AUTH_TYPES = i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_NO_AUTH_TYPE: ISCSI_AUTH_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_CHAP_AUTH_TYPE: ISCSI_AUTH_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_MUTUAL_CHAP_AUTH_TYPE: ISCSI_AUTH_TYPES = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_CONNECTION_INFOA {
    pub ConnectionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorAddress: ::windows_sys::core::PSTR,
    pub TargetAddress: ::windows_sys::core::PSTR,
    pub InitiatorSocket: u16,
    pub TargetSocket: u16,
    pub CID: [u8; 2],
}
impl ::core::marker::Copy for ISCSI_CONNECTION_INFOA {}
impl ::core::clone::Clone for ISCSI_CONNECTION_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_CONNECTION_INFOW {
    pub ConnectionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorAddress: ::windows_sys::core::PWSTR,
    pub TargetAddress: ::windows_sys::core::PWSTR,
    pub InitiatorSocket: u16,
    pub TargetSocket: u16,
    pub CID: [u8; 2],
}
impl ::core::marker::Copy for ISCSI_CONNECTION_INFOW {}
impl ::core::clone::Clone for ISCSI_CONNECTION_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_CONNECTION_INFO_EX {
    pub ConnectionId: ISCSI_UNIQUE_SESSION_ID,
    pub State: u8,
    pub Protocol: u8,
    pub HeaderDigest: u8,
    pub DataDigest: u8,
    pub MaxRecvDataSegmentLength: u32,
    pub AuthType: ISCSI_AUTH_TYPES,
    pub EstimatedThroughput: u64,
    pub MaxDatagramSize: u32,
}
impl ::core::marker::Copy for ISCSI_CONNECTION_INFO_EX {}
impl ::core::clone::Clone for ISCSI_CONNECTION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ioctl\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ioctl"))]
pub struct ISCSI_DEVICE_ON_SESSIONA {
    pub InitiatorName: [super::super::Foundation::CHAR; 256],
    pub TargetName: [super::super::Foundation::CHAR; 224],
    pub ScsiAddress: SCSI_ADDRESS,
    pub DeviceInterfaceType: ::windows_sys::core::GUID,
    pub DeviceInterfaceName: [super::super::Foundation::CHAR; 260],
    pub LegacyName: [super::super::Foundation::CHAR; 260],
    pub StorageDeviceNumber: super::super::System::Ioctl::STORAGE_DEVICE_NUMBER,
    pub DeviceInstance: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ioctl"))]
impl ::core::marker::Copy for ISCSI_DEVICE_ON_SESSIONA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ioctl"))]
impl ::core::clone::Clone for ISCSI_DEVICE_ON_SESSIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_System_Ioctl\"`*"]
#[cfg(feature = "Win32_System_Ioctl")]
pub struct ISCSI_DEVICE_ON_SESSIONW {
    pub InitiatorName: [u16; 256],
    pub TargetName: [u16; 224],
    pub ScsiAddress: SCSI_ADDRESS,
    pub DeviceInterfaceType: ::windows_sys::core::GUID,
    pub DeviceInterfaceName: [u16; 260],
    pub LegacyName: [u16; 260],
    pub StorageDeviceNumber: super::super::System::Ioctl::STORAGE_DEVICE_NUMBER,
    pub DeviceInstance: u32,
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::marker::Copy for ISCSI_DEVICE_ON_SESSIONW {}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::clone::Clone for ISCSI_DEVICE_ON_SESSIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub type ISCSI_DIGEST_TYPES = i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_DIGEST_TYPE_NONE: ISCSI_DIGEST_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_DIGEST_TYPE_CRC32C: ISCSI_DIGEST_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_FLAG_ALLOW_PORTAL_HOPPING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_FLAG_MULTIPATH_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_FLAG_REQUIRE_IPSEC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_FLAG_RESERVED1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_FLAG_USE_RADIUS_RESPONSE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_FLAG_USE_RADIUS_VERIFICATION: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_LOGIN_OPTIONS {
    pub Version: u32,
    pub InformationSpecified: u32,
    pub LoginFlags: u32,
    pub AuthType: ISCSI_AUTH_TYPES,
    pub HeaderDigest: ISCSI_DIGEST_TYPES,
    pub DataDigest: ISCSI_DIGEST_TYPES,
    pub MaximumConnections: u32,
    pub DefaultTime2Wait: u32,
    pub DefaultTime2Retain: u32,
    pub UsernameLength: u32,
    pub PasswordLength: u32,
    pub Username: *mut u8,
    pub Password: *mut u8,
}
impl ::core::marker::Copy for ISCSI_LOGIN_OPTIONS {}
impl ::core::clone::Clone for ISCSI_LOGIN_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_AUTH_TYPE: &str = "0x00000080";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_DATA_DIGEST: &str = "0x00000002";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_RETAIN: &str = "0x00000010";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_WAIT: &str = "0x00000008";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_HEADER_DIGEST: &str = "0x00000001";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_MAXIMUM_CONNECTIONS: &str = "0x00000004";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_PASSWORD: &str = "0x00000040";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_USERNAME: &str = "0x00000020";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_AGGRESSIVE_MODE_ENABLED: &str = "0x00000008";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_IKE_IPSEC_ENABLED: &str = "0x00000002";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_MAIN_MODE_ENABLED: &str = "0x00000004";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_PFS_ENABLED: &str = "0x00000010";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_TRANSPORT_MODE_PREFERRED: &str = "0x00000020";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_TUNNEL_MODE_PREFERRED: &str = "0x00000040";
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_VALID: &str = "0x00000001";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_SESSION_INFOA {
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorName: ::windows_sys::core::PSTR,
    pub TargetNodeName: ::windows_sys::core::PSTR,
    pub TargetName: ::windows_sys::core::PSTR,
    pub ISID: [u8; 6],
    pub TSID: [u8; 2],
    pub ConnectionCount: u32,
    pub Connections: *mut ISCSI_CONNECTION_INFOA,
}
impl ::core::marker::Copy for ISCSI_SESSION_INFOA {}
impl ::core::clone::Clone for ISCSI_SESSION_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_SESSION_INFOW {
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorName: ::windows_sys::core::PWSTR,
    pub TargetNodeName: ::windows_sys::core::PWSTR,
    pub TargetName: ::windows_sys::core::PWSTR,
    pub ISID: [u8; 6],
    pub TSID: [u8; 2],
    pub ConnectionCount: u32,
    pub Connections: *mut ISCSI_CONNECTION_INFOW,
}
impl ::core::marker::Copy for ISCSI_SESSION_INFOW {}
impl ::core::clone::Clone for ISCSI_SESSION_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ISCSI_SESSION_INFO_EX {
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitialR2t: super::super::Foundation::BOOLEAN,
    pub ImmediateData: super::super::Foundation::BOOLEAN,
    pub Type: u8,
    pub DataSequenceInOrder: super::super::Foundation::BOOLEAN,
    pub DataPduInOrder: super::super::Foundation::BOOLEAN,
    pub ErrorRecoveryLevel: u8,
    pub MaxOutstandingR2t: u32,
    pub FirstBurstLength: u32,
    pub MaxBurstLength: u32,
    pub MaximumConnections: u32,
    pub ConnectionCount: u32,
    pub Connections: *mut ISCSI_CONNECTION_INFO_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ISCSI_SESSION_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ISCSI_SESSION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_TARGET_FLAG_HIDE_STATIC_TARGET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_TARGET_FLAG_MERGE_TARGET_INFORMATION: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ISCSI_TARGET_MAPPINGA {
    pub InitiatorName: [super::super::Foundation::CHAR; 256],
    pub TargetName: [super::super::Foundation::CHAR; 224],
    pub OSDeviceName: [super::super::Foundation::CHAR; 260],
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub OSBusNumber: u32,
    pub OSTargetNumber: u32,
    pub LUNCount: u32,
    pub LUNList: *mut SCSI_LUN_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ISCSI_TARGET_MAPPINGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ISCSI_TARGET_MAPPINGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_MAPPINGW {
    pub InitiatorName: [u16; 256],
    pub TargetName: [u16; 224],
    pub OSDeviceName: [u16; 260],
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub OSBusNumber: u32,
    pub OSTargetNumber: u32,
    pub LUNCount: u32,
    pub LUNList: *mut SCSI_LUN_LIST,
}
impl ::core::marker::Copy for ISCSI_TARGET_MAPPINGW {}
impl ::core::clone::Clone for ISCSI_TARGET_MAPPINGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ISCSI_TARGET_PORTALA {
    pub SymbolicName: [super::super::Foundation::CHAR; 256],
    pub Address: [super::super::Foundation::CHAR; 256],
    pub Socket: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ISCSI_TARGET_PORTALA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ISCSI_TARGET_PORTALA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_PORTALW {
    pub SymbolicName: [u16; 256],
    pub Address: [u16; 256],
    pub Socket: u16,
}
impl ::core::marker::Copy for ISCSI_TARGET_PORTALW {}
impl ::core::clone::Clone for ISCSI_TARGET_PORTALW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ISCSI_TARGET_PORTAL_GROUPA {
    pub Count: u32,
    pub Portals: [ISCSI_TARGET_PORTALA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ISCSI_TARGET_PORTAL_GROUPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ISCSI_TARGET_PORTAL_GROUPA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_PORTAL_GROUPW {
    pub Count: u32,
    pub Portals: [ISCSI_TARGET_PORTALW; 1],
}
impl ::core::marker::Copy for ISCSI_TARGET_PORTAL_GROUPW {}
impl ::core::clone::Clone for ISCSI_TARGET_PORTAL_GROUPW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ISCSI_TARGET_PORTAL_INFOA {
    pub InitiatorName: [super::super::Foundation::CHAR; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [super::super::Foundation::CHAR; 256],
    pub Address: [super::super::Foundation::CHAR; 256],
    pub Socket: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ISCSI_TARGET_PORTAL_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ISCSI_TARGET_PORTAL_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_PORTAL_INFOW {
    pub InitiatorName: [u16; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [u16; 256],
    pub Address: [u16; 256],
    pub Socket: u16,
}
impl ::core::marker::Copy for ISCSI_TARGET_PORTAL_INFOW {}
impl ::core::clone::Clone for ISCSI_TARGET_PORTAL_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ISCSI_TARGET_PORTAL_INFO_EXA {
    pub InitiatorName: [super::super::Foundation::CHAR; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [super::super::Foundation::CHAR; 256],
    pub Address: [super::super::Foundation::CHAR; 256],
    pub Socket: u16,
    pub SecurityFlags: u64,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ISCSI_TARGET_PORTAL_INFO_EXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ISCSI_TARGET_PORTAL_INFO_EXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_PORTAL_INFO_EXW {
    pub InitiatorName: [u16; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [u16; 256],
    pub Address: [u16; 256],
    pub Socket: u16,
    pub SecurityFlags: u64,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
impl ::core::marker::Copy for ISCSI_TARGET_PORTAL_INFO_EXW {}
impl ::core::clone::Clone for ISCSI_TARGET_PORTAL_INFO_EXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_UNIQUE_SESSION_ID {
    pub AdapterUnique: u64,
    pub AdapterSpecific: u64,
}
impl ::core::marker::Copy for ISCSI_UNIQUE_SESSION_ID {}
impl ::core::clone::Clone for ISCSI_UNIQUE_SESSION_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_VERSION_INFO {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub BuildNumber: u32,
}
impl ::core::marker::Copy for ISCSI_VERSION_INFO {}
impl ::core::clone::Clone for ISCSI_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_ALIAS_LEN: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_DISCOVERY_DOMAIN_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_HBANAME_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_NAME_LEN: u32 = 223u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_PORTAL_ADDRESS_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_PORTAL_ALIAS_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_PORTAL_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_TEXT_ADDRESS_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_RADIUS_ADDRESS_LEN: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_NOTIFICATION_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_NOTIFICATION_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_NOTIFY_FLAG_BEGIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_NOTIFY_FLAG_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_PROFILE_CRASHDUMP_FILE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_PROFILE_HIBERNATION_FILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_PROFILE_PAGE_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_PROFILE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MPIO_IOCTL_FLAG_INVOLVE_DSM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MPIO_IOCTL_FLAG_USE_PATHID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MPIO_IOCTL_FLAG_USE_SCSIADDRESS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct MPIO_PASS_THROUGH_PATH {
    pub PassThrough: SCSI_PASS_THROUGH,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH {}
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MPIO_PASS_THROUGH_PATH32 {
    pub PassThrough: SCSI_PASS_THROUGH32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MPIO_PASS_THROUGH_PATH32_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH32_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT {
    pub PassThrough: SCSI_PASS_THROUGH_DIRECT,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH_DIRECT {}
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH_DIRECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32 {
    pub PassThrough: SCSI_PASS_THROUGH_DIRECT32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH_DIRECT32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH_DIRECT_EX {}
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct MPIO_PASS_THROUGH_PATH_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH_EX {}
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct MP_DEVICE_DATA_SET_RANGE {
    pub StartingOffset: i64,
    pub LengthInBytes: u64,
}
impl ::core::marker::Copy for MP_DEVICE_DATA_SET_RANGE {}
impl ::core::clone::Clone for MP_DEVICE_DATA_SET_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub type MP_STORAGE_DIAGNOSTIC_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MpStorageDiagnosticLevelDefault: MP_STORAGE_DIAGNOSTIC_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MpStorageDiagnosticLevelMax: MP_STORAGE_DIAGNOSTIC_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub type MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MpStorageDiagnosticTargetTypeUndefined: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MpStorageDiagnosticTargetTypeMiniport: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MpStorageDiagnosticTargetTypeHbaFirmware: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MpStorageDiagnosticTargetTypeMax: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_ADD_LBAS_PINNED_SET: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_FLUSH_NVCACHE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVCACHE_INFO: u32 = 236u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_RETURN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_SET: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVSEPARATED_FLUSH: u32 = 193u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVSEPARATED_INFO: u32 = 192u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVSEPARATED_WB_DISABLE: u32 = 194u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVSEPARATED_WB_REVERT_DEFAULT: u32 = 195u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_PASS_HINT_PAYLOAD: u32 = 224u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_QUERY_ASCENDER_STATUS: u32 = 208u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_QUERY_CACHE_MISS: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_QUERY_HYBRID_DISK_STATUS: u32 = 209u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_QUERY_PINNED_SET: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_REMOVE_LBAS_PINNED_SET: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_SPINDLE_STATUS: u32 = 229u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_ILLEGAL_REQUEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_INPUT_DATA_OVERRUN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_INPUT_DATA_UNDERRUN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_INVALID_PARAMETER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_OUTPUT_DATA_OVERRUN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_OUTPUT_DATA_UNDERRUN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_SUCCESS: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NTSCSI_UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for NTSCSI_UNICODE_STRING {}
impl ::core::clone::Clone for NTSCSI_UNICODE_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NVCACHE_HINT_PAYLOAD {
    pub Command: u8,
    pub Feature7_0: u8,
    pub Feature15_8: u8,
    pub Count15_8: u8,
    pub LBA7_0: u8,
    pub LBA15_8: u8,
    pub LBA23_16: u8,
    pub LBA31_24: u8,
    pub LBA39_32: u8,
    pub LBA47_40: u8,
    pub Auxiliary7_0: u8,
    pub Auxiliary23_16: u8,
    pub Reserved: [u8; 4],
}
impl ::core::marker::Copy for NVCACHE_HINT_PAYLOAD {}
impl ::core::clone::Clone for NVCACHE_HINT_PAYLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    pub PriorityLevel: u8,
    pub Reserved0: [u8; 3],
    pub ConsumedNVMSizeFraction: u32,
    pub ConsumedMappingResourcesFraction: u32,
    pub ConsumedNVMSizeForDirtyDataFraction: u32,
    pub ConsumedMappingResourcesForDirtyDataFraction: u32,
    pub Reserved1: u32,
}
impl ::core::marker::Copy for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {}
impl ::core::clone::Clone for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NVCACHE_REQUEST_BLOCK {
    pub NRBSize: u32,
    pub Function: u16,
    pub NRBFlags: u32,
    pub NRBStatus: u32,
    pub Count: u32,
    pub LBA: u64,
    pub DataBufSize: u32,
    pub NVCacheStatus: u32,
    pub NVCacheSubStatus: u32,
}
impl ::core::marker::Copy for NVCACHE_REQUEST_BLOCK {}
impl ::core::clone::Clone for NVCACHE_REQUEST_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub type NVCACHE_STATUS = i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheStatusUnknown: NVCACHE_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheStatusDisabling: NVCACHE_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheStatusDisabled: NVCACHE_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheStatusEnabled: NVCACHE_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub type NVCACHE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheTypeUnknown: NVCACHE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheTypeNone: NVCACHE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheTypeWriteBack: NVCACHE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheTypeWriteThrough: NVCACHE_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NV_FEATURE_PARAMETER {
    pub NVPowerModeEnabled: u16,
    pub NVParameterReserv1: u16,
    pub NVCmdEnabled: u16,
    pub NVParameterReserv2: u16,
    pub NVPowerModeVer: u16,
    pub NVCmdVer: u16,
    pub NVSize: u32,
    pub NVReadSpeed: u16,
    pub NVWrtSpeed: u16,
    pub DeviceSpinUpTime: u32,
}
impl ::core::marker::Copy for NV_FEATURE_PARAMETER {}
impl ::core::clone::Clone for NV_FEATURE_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NV_SEP_CACHE_PARAMETER {
    pub Version: u32,
    pub Size: u32,
    pub Flags: NV_SEP_CACHE_PARAMETER_0,
    pub WriteCacheType: u8,
    pub WriteCacheTypeEffective: u8,
    pub ParameterReserve1: [u8; 3],
}
impl ::core::marker::Copy for NV_SEP_CACHE_PARAMETER {}
impl ::core::clone::Clone for NV_SEP_CACHE_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub union NV_SEP_CACHE_PARAMETER_0 {
    pub CacheFlags: NV_SEP_CACHE_PARAMETER_0_0,
    pub CacheFlagsSet: u8,
}
impl ::core::marker::Copy for NV_SEP_CACHE_PARAMETER_0 {}
impl ::core::clone::Clone for NV_SEP_CACHE_PARAMETER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NV_SEP_CACHE_PARAMETER_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NV_SEP_CACHE_PARAMETER_0_0 {}
impl ::core::clone::Clone for NV_SEP_CACHE_PARAMETER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NV_SEP_CACHE_PARAMETER_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NV_SEP_CACHE_PARAMETER_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub type NV_SEP_WRITE_CACHE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NVSEPWriteCacheTypeUnknown: NV_SEP_WRITE_CACHE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NVSEPWriteCacheTypeNone: NV_SEP_WRITE_CACHE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NVSEPWriteCacheTypeWriteBack: NV_SEP_WRITE_CACHE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NVSEPWriteCacheTypeWriteThrough: NV_SEP_WRITE_CACHE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub type PDUMP_DEVICE_POWERON_ROUTINE = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> i32>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERSISTENT_ISCSI_LOGIN_INFOA {
    pub TargetName: [super::super::Foundation::CHAR; 224],
    pub IsInformationalSession: super::super::Foundation::BOOLEAN,
    pub InitiatorInstance: [super::super::Foundation::CHAR; 256],
    pub InitiatorPortNumber: u32,
    pub TargetPortal: ISCSI_TARGET_PORTALA,
    pub SecurityFlags: u64,
    pub Mappings: *mut ISCSI_TARGET_MAPPINGA,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERSISTENT_ISCSI_LOGIN_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERSISTENT_ISCSI_LOGIN_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERSISTENT_ISCSI_LOGIN_INFOW {
    pub TargetName: [u16; 224],
    pub IsInformationalSession: super::super::Foundation::BOOLEAN,
    pub InitiatorInstance: [u16; 256],
    pub InitiatorPortNumber: u32,
    pub TargetPortal: ISCSI_TARGET_PORTALW,
    pub SecurityFlags: u64,
    pub Mappings: *mut ISCSI_TARGET_MAPPINGW,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERSISTENT_ISCSI_LOGIN_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERSISTENT_ISCSI_LOGIN_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_ADAPTER_BUS_INFO {
    pub NumberOfBuses: u8,
    pub BusData: [SCSI_BUS_DATA; 1],
}
impl ::core::marker::Copy for SCSI_ADAPTER_BUS_INFO {}
impl ::core::clone::Clone for SCSI_ADAPTER_BUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_ADDRESS {
    pub Length: u32,
    pub PortNumber: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
impl ::core::marker::Copy for SCSI_ADDRESS {}
impl ::core::clone::Clone for SCSI_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_BUS_DATA {
    pub NumberOfLogicalUnits: u8,
    pub InitiatorBusId: u8,
    pub InquiryDataOffset: u32,
}
impl ::core::marker::Copy for SCSI_BUS_DATA {}
impl ::core::clone::Clone for SCSI_BUS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCSI_INQUIRY_DATA {
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub DeviceClaimed: super::super::Foundation::BOOLEAN,
    pub InquiryDataLength: u32,
    pub NextInquiryDataOffset: u32,
    pub InquiryData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCSI_INQUIRY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCSI_INQUIRY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const SCSI_IOCTL_DATA_BIDIRECTIONAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const SCSI_IOCTL_DATA_IN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const SCSI_IOCTL_DATA_OUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const SCSI_IOCTL_DATA_UNSPECIFIED: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_LUN_LIST {
    pub OSLUN: u32,
    pub TargetLUN: u64,
}
impl ::core::marker::Copy for SCSI_LUN_LIST {}
impl ::core::clone::Clone for SCSI_LUN_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_PASS_THROUGH {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBufferOffset: usize,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
impl ::core::marker::Copy for SCSI_PASS_THROUGH {}
impl ::core::clone::Clone for SCSI_PASS_THROUGH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SCSI_PASS_THROUGH32 {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBufferOffset: u32,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SCSI_PASS_THROUGH32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SCSI_PASS_THROUGH32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SCSI_PASS_THROUGH32_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBufferOffset: u32,
    pub DataInBufferOffset: u32,
    pub Cdb: [u8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SCSI_PASS_THROUGH32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SCSI_PASS_THROUGH32_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_PASS_THROUGH_DIRECT {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBuffer: *mut ::core::ffi::c_void,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
impl ::core::marker::Copy for SCSI_PASS_THROUGH_DIRECT {}
impl ::core::clone::Clone for SCSI_PASS_THROUGH_DIRECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SCSI_PASS_THROUGH_DIRECT32 {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBuffer: *mut ::core::ffi::c_void,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SCSI_PASS_THROUGH_DIRECT32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SCSI_PASS_THROUGH_DIRECT32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SCSI_PASS_THROUGH_DIRECT32_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBuffer: *mut ::core::ffi::c_void,
    pub DataInBuffer: *mut ::core::ffi::c_void,
    pub Cdb: [u8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SCSI_PASS_THROUGH_DIRECT32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_PASS_THROUGH_DIRECT_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBuffer: *mut ::core::ffi::c_void,
    pub DataInBuffer: *mut ::core::ffi::c_void,
    pub Cdb: [u8; 1],
}
impl ::core::marker::Copy for SCSI_PASS_THROUGH_DIRECT_EX {}
impl ::core::clone::Clone for SCSI_PASS_THROUGH_DIRECT_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_PASS_THROUGH_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBufferOffset: usize,
    pub DataInBufferOffset: usize,
    pub Cdb: [u8; 1],
}
impl ::core::marker::Copy for SCSI_PASS_THROUGH_EX {}
impl ::core::clone::Clone for SCSI_PASS_THROUGH_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SRB_IO_CONTROL {
    pub HeaderLength: u32,
    pub Signature: [u8; 8],
    pub Timeout: u32,
    pub ControlCode: u32,
    pub ReturnCode: u32,
    pub Length: u32,
}
impl ::core::marker::Copy for SRB_IO_CONTROL {}
impl ::core::clone::Clone for SRB_IO_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_DIAGNOSTIC_MP_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub TargetType: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE,
    pub Level: MP_STORAGE_DIAGNOSTIC_LEVEL,
    pub ProviderId: ::windows_sys::core::GUID,
    pub BufferSize: u32,
    pub Reserved: u32,
    pub DataBuffer: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_DIAGNOSTIC_MP_REQUEST {}
impl ::core::clone::Clone for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_BUFFER_TOO_SMALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_PARAMETER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_SIGNATURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_TARGET_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_MORE_DATA: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_UNSUPPORTED_VERSION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub EnduranceInfo: STORAGE_ENDURANCE_INFO,
}
impl ::core::marker::Copy for STORAGE_ENDURANCE_DATA_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_ENDURANCE_INFO {
    pub ValidFields: u32,
    pub GroupId: u32,
    pub Flags: STORAGE_ENDURANCE_INFO_0,
    pub LifePercentage: u32,
    pub BytesReadCount: [u8; 16],
    pub ByteWriteCount: [u8; 16],
}
impl ::core::marker::Copy for STORAGE_ENDURANCE_INFO {}
impl ::core::clone::Clone for STORAGE_ENDURANCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_ENDURANCE_INFO_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for STORAGE_ENDURANCE_INFO_0 {}
impl ::core::clone::Clone for STORAGE_ENDURANCE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_FIRMWARE_ACTIVATE {
    pub Version: u32,
    pub Size: u32,
    pub SlotToActivate: u8,
    pub Reserved0: [u8; 3],
}
impl ::core::marker::Copy for STORAGE_FIRMWARE_ACTIVATE {}
impl ::core::clone::Clone for STORAGE_FIRMWARE_ACTIVATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_ACTIVATE_STRUCTURE_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_FIRMWARE_DOWNLOAD {
    pub Version: u32,
    pub Size: u32,
    pub Offset: u64,
    pub BufferSize: u64,
    pub ImageBuffer: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_FIRMWARE_DOWNLOAD {}
impl ::core::clone::Clone for STORAGE_FIRMWARE_DOWNLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION_V2: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_FIRMWARE_DOWNLOAD_V2 {
    pub Version: u32,
    pub Size: u32,
    pub Offset: u64,
    pub BufferSize: u64,
    pub Slot: u8,
    pub Reserved: [u8; 3],
    pub ImageSize: u32,
    pub ImageBuffer: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_FIRMWARE_DOWNLOAD_V2 {}
impl ::core::clone::Clone for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FIRMWARE_INFO {
    pub Version: u32,
    pub Size: u32,
    pub UpgradeSupport: super::super::Foundation::BOOLEAN,
    pub SlotCount: u8,
    pub ActiveSlot: u8,
    pub PendingActivateSlot: u8,
    pub Reserved: u32,
    pub Slot: [STORAGE_FIRMWARE_SLOT_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_FIRMWARE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_FIRMWARE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_INFO_INVALID_SLOT: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION_V2: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FIRMWARE_INFO_V2 {
    pub Version: u32,
    pub Size: u32,
    pub UpgradeSupport: super::super::Foundation::BOOLEAN,
    pub SlotCount: u8,
    pub ActiveSlot: u8,
    pub PendingActivateSlot: u8,
    pub FirmwareShared: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 3],
    pub ImagePayloadAlignment: u32,
    pub ImagePayloadMaxSize: u32,
    pub Slot: [STORAGE_FIRMWARE_SLOT_INFO_V2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_FIRMWARE_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_FIRMWARE_INFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FIRMWARE_SLOT_INFO {
    pub SlotNumber: u8,
    pub ReadOnly: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 6],
    pub Revision: STORAGE_FIRMWARE_SLOT_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_FIRMWARE_SLOT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_FIRMWARE_SLOT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union STORAGE_FIRMWARE_SLOT_INFO_0 {
    pub Info: [u8; 8],
    pub AsUlonglong: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_FIRMWARE_SLOT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_FIRMWARE_SLOT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FIRMWARE_SLOT_INFO_V2 {
    pub SlotNumber: u8,
    pub ReadOnly: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 6],
    pub Revision: [u8; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_FIRMWARE_SLOT_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_SLOT_INFO_V2_REVISION_LENGTH: u32 = 16u32;
pub const ScsiRawInterfaceGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408590601, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub type TARGETPROTOCOLTYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_TCP_PROTOCOL_TYPE: TARGETPROTOCOLTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub type TARGET_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ProtocolType: TARGET_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const TargetAlias: TARGET_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DiscoveryMechanisms: TARGET_INFORMATION_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const PortalGroups: TARGET_INFORMATION_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const PersistentTargetMappings: TARGET_INFORMATION_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const InitiatorName: TARGET_INFORMATION_CLASS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const TargetFlags: TARGET_INFORMATION_CLASS = 6i32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const LoginOptions: TARGET_INFORMATION_CLASS = 7i32;
pub const WmiScsiAddressGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408590607, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
#[repr(C)]
pub struct _ADAPTER_OBJECT(pub u8);

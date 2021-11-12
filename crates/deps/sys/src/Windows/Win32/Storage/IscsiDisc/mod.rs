#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddISNSServerA(address: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddISNSServerW(address: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiConnectionA(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, reserved: *mut ::core::ffi::c_void, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALA, securityflags: u64, loginoptions: *mut ISCSI_LOGIN_OPTIONS, keysize: u32, key: super::super::Foundation::PSTR, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiConnectionW(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, reserved: *mut ::core::ffi::c_void, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALW, securityflags: u64, loginoptions: *mut ISCSI_LOGIN_OPTIONS, keysize: u32, key: super::super::Foundation::PSTR, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiSendTargetPortalA(initiatorinstance: super::super::Foundation::PSTR, initiatorportnumber: u32, loginoptions: *mut ISCSI_LOGIN_OPTIONS, securityflags: u64, portal: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiSendTargetPortalW(initiatorinstance: super::super::Foundation::PWSTR, initiatorportnumber: u32, loginoptions: *mut ISCSI_LOGIN_OPTIONS, securityflags: u64, portal: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiStaticTargetA(targetname: super::super::Foundation::PSTR, targetalias: super::super::Foundation::PSTR, targetflags: u32, persist: super::super::Foundation::BOOLEAN, mappings: *mut ISCSI_TARGET_MAPPINGA, loginoptions: *mut ISCSI_LOGIN_OPTIONS, portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiStaticTargetW(targetname: super::super::Foundation::PWSTR, targetalias: super::super::Foundation::PWSTR, targetflags: u32, persist: super::super::Foundation::BOOLEAN, mappings: *mut ISCSI_TARGET_MAPPINGW, loginoptions: *mut ISCSI_LOGIN_OPTIONS, portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPersistentIScsiDeviceA(devicepath: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPersistentIScsiDeviceW(devicepath: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddRadiusServerA(address: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddRadiusServerW(address: super::super::Foundation::PWSTR) -> u32;
    pub fn ClearPersistentIScsiDevices() -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ioctl"))]
    pub fn GetDevicesForIScsiSessionA(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, devicecount: *mut u32, devices: *mut ISCSI_DEVICE_ON_SESSIONA) -> u32;
    #[cfg(feature = "Win32_System_Ioctl")]
    pub fn GetDevicesForIScsiSessionW(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, devicecount: *mut u32, devices: *mut ISCSI_DEVICE_ON_SESSIONW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiIKEInfoA(initiatorname: super::super::Foundation::PSTR, initiatorportnumber: u32, reserved: *mut u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiIKEInfoW(initiatorname: super::super::Foundation::PWSTR, initiatorportnumber: u32, reserved: *mut u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiInitiatorNodeNameA(initiatornodename: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiInitiatorNodeNameW(initiatornodename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiSessionListA(buffersize: *mut u32, sessioncount: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFOA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiSessionListEx(buffersize: *mut u32, sessioncountptr: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFO_EX) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiSessionListW(buffersize: *mut u32, sessioncount: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFOW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiTargetInformationA(targetname: super::super::Foundation::PSTR, discoverymechanism: super::super::Foundation::PSTR, infoclass: TARGET_INFORMATION_CLASS, buffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiTargetInformationW(targetname: super::super::Foundation::PWSTR, discoverymechanism: super::super::Foundation::PWSTR, infoclass: TARGET_INFORMATION_CLASS, buffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    pub fn GetIScsiVersionInformation(versioninfo: *mut ISCSI_VERSION_INFO) -> u32;
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
    pub fn LogoutIScsiTarget(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshISNSServerA(address: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshISNSServerW(address: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshIScsiSendTargetPortalA(initiatorinstance: super::super::Foundation::PSTR, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshIScsiSendTargetPortalW(initiatorinstance: super::super::Foundation::PWSTR, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveISNSServerA(address: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveISNSServerW(address: super::super::Foundation::PWSTR) -> u32;
    pub fn RemoveIScsiConnection(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiPersistentTargetA(initiatorinstance: super::super::Foundation::PSTR, initiatorportnumber: u32, targetname: super::super::Foundation::PSTR, portal: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiPersistentTargetW(initiatorinstance: super::super::Foundation::PWSTR, initiatorportnumber: u32, targetname: super::super::Foundation::PWSTR, portal: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiSendTargetPortalA(initiatorinstance: super::super::Foundation::PSTR, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiSendTargetPortalW(initiatorinstance: super::super::Foundation::PWSTR, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiStaticTargetA(targetname: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiStaticTargetW(targetname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePersistentIScsiDeviceA(devicepath: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePersistentIScsiDeviceW(devicepath: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveRadiusServerA(address: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveRadiusServerW(address: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportActiveIScsiTargetMappingsA(buffersize: *mut u32, mappingcount: *mut u32, mappings: *mut ISCSI_TARGET_MAPPINGA) -> u32;
    pub fn ReportActiveIScsiTargetMappingsW(buffersize: *mut u32, mappingcount: *mut u32, mappings: *mut ISCSI_TARGET_MAPPINGW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportISNSServerListA(buffersizeinchar: *mut u32, buffer: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportISNSServerListW(buffersizeinchar: *mut u32, buffer: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiInitiatorListA(buffersize: *mut u32, buffer: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiInitiatorListW(buffersize: *mut u32, buffer: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiPersistentLoginsA(count: *mut u32, persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOA, buffersizeinbytes: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiPersistentLoginsW(count: *mut u32, persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOW, buffersizeinbytes: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiSendTargetPortalsA(portalcount: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFOA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiSendTargetPortalsExA(portalcount: *mut u32, portalinfosize: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXA) -> u32;
    pub fn ReportIScsiSendTargetPortalsExW(portalcount: *mut u32, portalinfosize: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXW) -> u32;
    pub fn ReportIScsiSendTargetPortalsW(portalcount: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFOW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetPortalsA(initiatorname: super::super::Foundation::PSTR, targetname: super::super::Foundation::PSTR, targetportaltag: *mut u16, elementcount: *mut u32, portals: *mut ISCSI_TARGET_PORTALA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetPortalsW(initiatorname: super::super::Foundation::PWSTR, targetname: super::super::Foundation::PWSTR, targetportaltag: *mut u16, elementcount: *mut u32, portals: *mut ISCSI_TARGET_PORTALW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetsA(forceupdate: super::super::Foundation::BOOLEAN, buffersize: *mut u32, buffer: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetsW(forceupdate: super::super::Foundation::BOOLEAN, buffersize: *mut u32, buffer: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportPersistentIScsiDevicesA(buffersizeinchar: *mut u32, buffer: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportPersistentIScsiDevicesW(buffersizeinchar: *mut u32, buffer: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportRadiusServerListA(buffersizeinchar: *mut u32, buffer: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportRadiusServerListW(buffersizeinchar: *mut u32, buffer: super::super::Foundation::PWSTR) -> u32;
    pub fn SendScsiInquiry(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, lun: u64, evpdcmddt: u8, pagecode: u8, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32;
    pub fn SendScsiReadCapacity(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, lun: u64, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32;
    pub fn SendScsiReportLuns(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiGroupPresharedKey(keylength: u32, key: *mut u8, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiIKEInfoA(initiatorname: super::super::Foundation::PSTR, initiatorportnumber: u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiIKEInfoW(initiatorname: super::super::Foundation::PWSTR, initiatorportnumber: u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION, persist: super::super::Foundation::BOOLEAN) -> u32;
    pub fn SetIScsiInitiatorCHAPSharedSecret(sharedsecretlength: u32, sharedsecret: *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiInitiatorNodeNameA(initiatornodename: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiInitiatorNodeNameW(initiatornodename: super::super::Foundation::PWSTR) -> u32;
    pub fn SetIScsiInitiatorRADIUSSharedSecret(sharedsecretlength: u32, sharedsecret: *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiTunnelModeOuterAddressA(initiatorname: super::super::Foundation::PSTR, initiatorportnumber: u32, destinationaddress: super::super::Foundation::PSTR, outermodeaddress: super::super::Foundation::PSTR, persist: super::super::Foundation::BOOLEAN) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiTunnelModeOuterAddressW(initiatorname: super::super::Foundation::PWSTR, initiatorportnumber: u32, destinationaddress: super::super::Foundation::PWSTR, outermodeaddress: super::super::Foundation::PWSTR, persist: super::super::Foundation::BOOLEAN) -> u32;
    pub fn SetupPersistentIScsiDevices() -> u32;
    pub fn SetupPersistentIScsiVolumes() -> u32;
}
pub const ATA_FLAGS_48BIT_COMMAND: u32 = 8u32;
pub const ATA_FLAGS_DATA_IN: u32 = 2u32;
pub const ATA_FLAGS_DATA_OUT: u32 = 4u32;
pub const ATA_FLAGS_DRDY_REQUIRED: u32 = 1u32;
pub const ATA_FLAGS_NO_MULTIPLE: u32 = 32u32;
pub const ATA_FLAGS_USE_DMA: u32 = 16u32;
#[repr(C)]
pub struct ATA_PASS_THROUGH_DIRECT(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct ATA_PASS_THROUGH_DIRECT32(i32);
#[repr(C)]
pub struct ATA_PASS_THROUGH_EX(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct ATA_PASS_THROUGH_EX32(i32);
#[repr(C)]
pub struct DSM_NOTIFICATION_REQUEST_BLOCK(i32);
pub type DUMP_DEVICE_POWERON_ROUTINE = unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> i32;
#[repr(C)]
pub struct DUMP_DRIVER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DUMP_DRIVER_EX(i32);
pub const DUMP_DRIVER_NAME_LENGTH: u32 = 15u32;
pub const DUMP_EX_FLAG_DRIVER_FULL_PATH_SUPPORT: u32 = 8u32;
pub const DUMP_EX_FLAG_RESUME_SUPPORT: u32 = 4u32;
pub const DUMP_EX_FLAG_SUPPORT_64BITMEMORY: u32 = 1u32;
pub const DUMP_EX_FLAG_SUPPORT_DD_TELEMETRY: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DUMP_POINTERS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DUMP_POINTERS_EX(i32);
#[repr(C)]
pub struct DUMP_POINTERS_VERSION(i32);
pub const DUMP_POINTERS_VERSION_1: u32 = 1u32;
pub const DUMP_POINTERS_VERSION_2: u32 = 2u32;
pub const DUMP_POINTERS_VERSION_3: u32 = 3u32;
pub const DUMP_POINTERS_VERSION_4: u32 = 4u32;
pub const FILE_DEVICE_SCSI: u32 = 27u32;
pub const FIRMWARE_FUNCTION_ACTIVATE: u32 = 3u32;
pub const FIRMWARE_FUNCTION_DOWNLOAD: u32 = 2u32;
pub const FIRMWARE_FUNCTION_GET_INFO: u32 = 1u32;
#[repr(C)]
pub struct FIRMWARE_REQUEST_BLOCK(i32);
pub const FIRMWARE_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1u32;
pub const FIRMWARE_REQUEST_FLAG_CONTROLLER: u32 = 1u32;
pub const FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT: u32 = 4u32;
pub const FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: u32 = 2u32;
pub const FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: u32 = 2147483648u32;
pub const FIRMWARE_STATUS_COMMAND_ABORT: u32 = 133u32;
pub const FIRMWARE_STATUS_CONTROLLER_ERROR: u32 = 16u32;
pub const FIRMWARE_STATUS_DEVICE_ERROR: u32 = 64u32;
pub const FIRMWARE_STATUS_END_OF_MEDIA: u32 = 134u32;
pub const FIRMWARE_STATUS_ERROR: u32 = 1u32;
pub const FIRMWARE_STATUS_ID_NOT_FOUND: u32 = 131u32;
pub const FIRMWARE_STATUS_ILLEGAL_LENGTH: u32 = 135u32;
pub const FIRMWARE_STATUS_ILLEGAL_REQUEST: u32 = 2u32;
pub const FIRMWARE_STATUS_INPUT_BUFFER_TOO_BIG: u32 = 4u32;
pub const FIRMWARE_STATUS_INTERFACE_CRC_ERROR: u32 = 128u32;
pub const FIRMWARE_STATUS_INVALID_IMAGE: u32 = 7u32;
pub const FIRMWARE_STATUS_INVALID_PARAMETER: u32 = 3u32;
pub const FIRMWARE_STATUS_INVALID_SLOT: u32 = 6u32;
pub const FIRMWARE_STATUS_MEDIA_CHANGE: u32 = 130u32;
pub const FIRMWARE_STATUS_MEDIA_CHANGE_REQUEST: u32 = 132u32;
pub const FIRMWARE_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 5u32;
pub const FIRMWARE_STATUS_POWER_CYCLE_REQUIRED: u32 = 32u32;
pub const FIRMWARE_STATUS_SUCCESS: u32 = 0u32;
pub const FIRMWARE_STATUS_UNCORRECTABLE_DATA_ERROR: u32 = 129u32;
#[repr(C)]
pub struct HYBRID_DEMOTE_BY_SIZE(i32);
#[repr(C)]
pub struct HYBRID_DIRTY_THRESHOLDS(i32);
pub const HYBRID_FUNCTION_DEMOTE_BY_SIZE: u32 = 19u32;
pub const HYBRID_FUNCTION_DISABLE_CACHING_MEDIUM: u32 = 16u32;
pub const HYBRID_FUNCTION_ENABLE_CACHING_MEDIUM: u32 = 17u32;
pub const HYBRID_FUNCTION_GET_INFO: u32 = 1u32;
pub const HYBRID_FUNCTION_SET_DIRTY_THRESHOLD: u32 = 18u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HYBRID_INFORMATION(i32);
#[repr(C)]
pub struct HYBRID_REQUEST_BLOCK(i32);
pub const HYBRID_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1u32;
pub const HYBRID_REQUEST_INFO_STRUCTURE_VERSION: u32 = 1u32;
pub const HYBRID_STATUS_ENABLE_REFCOUNT_HOLD: u32 = 16u32;
pub const HYBRID_STATUS_ILLEGAL_REQUEST: u32 = 1u32;
pub const HYBRID_STATUS_INVALID_PARAMETER: u32 = 2u32;
pub const HYBRID_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 3u32;
pub const HYBRID_STATUS_SUCCESS: u32 = 0u32;
#[repr(C)]
pub struct IDE_IO_CONTROL(i32);
pub const ID_FQDN: u32 = 2u32;
pub const ID_IPV4_ADDR: u32 = 1u32;
pub const ID_IPV6_ADDR: u32 = 5u32;
pub const ID_USER_FQDN: u32 = 3u32;
#[repr(C)]
pub struct IKE_AUTHENTICATION_INFORMATION(i32);
#[repr(transparent)]
pub struct IKE_AUTHENTICATION_METHOD(pub i32);
pub const IKE_AUTHENTICATION_PRESHARED_KEY_METHOD: IKE_AUTHENTICATION_METHOD = IKE_AUTHENTICATION_METHOD(1i32);
#[repr(C)]
pub struct IKE_AUTHENTICATION_PRESHARED_KEY(i32);
pub const IOCTL_ATA_MINIPORT: u32 = 315444u32;
pub const IOCTL_ATA_PASS_THROUGH: u32 = 315436u32;
pub const IOCTL_ATA_PASS_THROUGH_DIRECT: u32 = 315440u32;
pub const IOCTL_IDE_PASS_THROUGH: u32 = 315432u32;
pub const IOCTL_MINIPORT_PROCESS_SERVICE_IRP: u32 = 315448u32;
pub const IOCTL_MPIO_PASS_THROUGH_PATH: u32 = 315452u32;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT: u32 = 315456u32;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT_EX: u32 = 315472u32;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_EX: u32 = 315468u32;
pub const IOCTL_SCSI_BASE: u32 = 4u32;
pub const IOCTL_SCSI_FREE_DUMP_POINTERS: u32 = 266276u32;
pub const IOCTL_SCSI_GET_ADDRESS: u32 = 266264u32;
pub const IOCTL_SCSI_GET_CAPABILITIES: u32 = 266256u32;
pub const IOCTL_SCSI_GET_DUMP_POINTERS: u32 = 266272u32;
pub const IOCTL_SCSI_GET_INQUIRY_DATA: u32 = 266252u32;
pub const IOCTL_SCSI_MINIPORT: u32 = 315400u32;
pub const IOCTL_SCSI_PASS_THROUGH: u32 = 315396u32;
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT: u32 = 315412u32;
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT_EX: u32 = 315464u32;
pub const IOCTL_SCSI_PASS_THROUGH_EX: u32 = 315460u32;
pub const IOCTL_SCSI_RESCAN_BUS: u32 = 266268u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IO_SCSI_CAPABILITIES(i32);
#[repr(transparent)]
pub struct ISCSI_AUTH_TYPES(pub i32);
pub const ISCSI_NO_AUTH_TYPE: ISCSI_AUTH_TYPES = ISCSI_AUTH_TYPES(0i32);
pub const ISCSI_CHAP_AUTH_TYPE: ISCSI_AUTH_TYPES = ISCSI_AUTH_TYPES(1i32);
pub const ISCSI_MUTUAL_CHAP_AUTH_TYPE: ISCSI_AUTH_TYPES = ISCSI_AUTH_TYPES(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ISCSI_CONNECTION_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ISCSI_CONNECTION_INFOW(i32);
#[repr(C)]
pub struct ISCSI_CONNECTION_INFO_EX(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ioctl"))]
#[repr(C)]
pub struct ISCSI_DEVICE_ON_SESSIONA(i32);
#[cfg(feature = "Win32_System_Ioctl")]
#[repr(C)]
pub struct ISCSI_DEVICE_ON_SESSIONW(i32);
#[repr(transparent)]
pub struct ISCSI_DIGEST_TYPES(pub i32);
pub const ISCSI_DIGEST_TYPE_NONE: ISCSI_DIGEST_TYPES = ISCSI_DIGEST_TYPES(0i32);
pub const ISCSI_DIGEST_TYPE_CRC32C: ISCSI_DIGEST_TYPES = ISCSI_DIGEST_TYPES(1i32);
pub const ISCSI_LOGIN_FLAG_ALLOW_PORTAL_HOPPING: u32 = 8u32;
pub const ISCSI_LOGIN_FLAG_MULTIPATH_ENABLED: u32 = 2u32;
pub const ISCSI_LOGIN_FLAG_REQUIRE_IPSEC: u32 = 1u32;
pub const ISCSI_LOGIN_FLAG_RESERVED1: u32 = 4u32;
pub const ISCSI_LOGIN_FLAG_USE_RADIUS_RESPONSE: u32 = 16u32;
pub const ISCSI_LOGIN_FLAG_USE_RADIUS_VERIFICATION: u32 = 32u32;
#[repr(C)]
pub struct ISCSI_LOGIN_OPTIONS(i32);
pub const ISCSI_LOGIN_OPTIONS_AUTH_TYPE: u32 = 128u32;
pub const ISCSI_LOGIN_OPTIONS_DATA_DIGEST: u32 = 2u32;
pub const ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_RETAIN: u32 = 16u32;
pub const ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_WAIT: u32 = 8u32;
pub const ISCSI_LOGIN_OPTIONS_HEADER_DIGEST: u32 = 1u32;
pub const ISCSI_LOGIN_OPTIONS_MAXIMUM_CONNECTIONS: u32 = 4u32;
pub const ISCSI_LOGIN_OPTIONS_PASSWORD: u32 = 64u32;
pub const ISCSI_LOGIN_OPTIONS_USERNAME: u32 = 32u32;
pub const ISCSI_LOGIN_OPTIONS_VERSION: u32 = 0u32;
pub const ISCSI_SECURITY_FLAG_AGGRESSIVE_MODE_ENABLED: u32 = 8u32;
pub const ISCSI_SECURITY_FLAG_IKE_IPSEC_ENABLED: u32 = 2u32;
pub const ISCSI_SECURITY_FLAG_MAIN_MODE_ENABLED: u32 = 4u32;
pub const ISCSI_SECURITY_FLAG_PFS_ENABLED: u32 = 16u32;
pub const ISCSI_SECURITY_FLAG_TRANSPORT_MODE_PREFERRED: u32 = 32u32;
pub const ISCSI_SECURITY_FLAG_TUNNEL_MODE_PREFERRED: u32 = 64u32;
pub const ISCSI_SECURITY_FLAG_VALID: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ISCSI_SESSION_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ISCSI_SESSION_INFOW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ISCSI_SESSION_INFO_EX(i32);
pub const ISCSI_TARGET_FLAG_HIDE_STATIC_TARGET: u32 = 2u32;
pub const ISCSI_TARGET_FLAG_MERGE_TARGET_INFORMATION: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ISCSI_TARGET_MAPPINGA(i32);
#[repr(C)]
pub struct ISCSI_TARGET_MAPPINGW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ISCSI_TARGET_PORTALA(i32);
#[repr(C)]
pub struct ISCSI_TARGET_PORTALW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ISCSI_TARGET_PORTAL_GROUPA(i32);
#[repr(C)]
pub struct ISCSI_TARGET_PORTAL_GROUPW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ISCSI_TARGET_PORTAL_INFOA(i32);
#[repr(C)]
pub struct ISCSI_TARGET_PORTAL_INFOW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ISCSI_TARGET_PORTAL_INFO_EXA(i32);
#[repr(C)]
pub struct ISCSI_TARGET_PORTAL_INFO_EXW(i32);
#[repr(C)]
pub struct ISCSI_UNIQUE_SESSION_ID(i32);
#[repr(C)]
pub struct ISCSI_VERSION_INFO(i32);
pub const MAX_ISCSI_ALIAS_LEN: u32 = 255u32;
pub const MAX_ISCSI_DISCOVERY_DOMAIN_LEN: u32 = 256u32;
pub const MAX_ISCSI_HBANAME_LEN: u32 = 256u32;
pub const MAX_ISCSI_NAME_LEN: u32 = 223u32;
pub const MAX_ISCSI_PORTAL_ADDRESS_LEN: u32 = 256u32;
pub const MAX_ISCSI_PORTAL_ALIAS_LEN: u32 = 256u32;
pub const MAX_ISCSI_PORTAL_NAME_LEN: u32 = 256u32;
pub const MAX_ISCSI_TEXT_ADDRESS_LEN: u32 = 256u32;
pub const MAX_RADIUS_ADDRESS_LEN: u32 = 41u32;
pub const MINIPORT_DSM_NOTIFICATION_VERSION: u32 = 1u32;
pub const MINIPORT_DSM_NOTIFICATION_VERSION_1: u32 = 1u32;
pub const MINIPORT_DSM_NOTIFY_FLAG_BEGIN: u32 = 1u32;
pub const MINIPORT_DSM_NOTIFY_FLAG_END: u32 = 2u32;
pub const MINIPORT_DSM_PROFILE_CRASHDUMP_FILE: u32 = 3u32;
pub const MINIPORT_DSM_PROFILE_HIBERNATION_FILE: u32 = 2u32;
pub const MINIPORT_DSM_PROFILE_PAGE_FILE: u32 = 1u32;
pub const MINIPORT_DSM_PROFILE_UNKNOWN: u32 = 0u32;
pub const MPIO_IOCTL_FLAG_INVOLVE_DSM: u32 = 4u32;
pub const MPIO_IOCTL_FLAG_USE_PATHID: u32 = 1u32;
pub const MPIO_IOCTL_FLAG_USE_SCSIADDRESS: u32 = 2u32;
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH32(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH32_EX(i32);
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32_EX(i32);
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT_EX(i32);
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH_EX(i32);
#[repr(C)]
pub struct MP_DEVICE_DATA_SET_RANGE(i32);
#[repr(transparent)]
pub struct MP_STORAGE_DIAGNOSTIC_LEVEL(pub i32);
pub const MpStorageDiagnosticLevelDefault: MP_STORAGE_DIAGNOSTIC_LEVEL = MP_STORAGE_DIAGNOSTIC_LEVEL(0i32);
pub const MpStorageDiagnosticLevelMax: MP_STORAGE_DIAGNOSTIC_LEVEL = MP_STORAGE_DIAGNOSTIC_LEVEL(1i32);
#[repr(transparent)]
pub struct MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(pub i32);
pub const MpStorageDiagnosticTargetTypeUndefined: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(0i32);
pub const MpStorageDiagnosticTargetTypeMiniport: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(2i32);
pub const MpStorageDiagnosticTargetTypeHbaFirmware: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(3i32);
pub const MpStorageDiagnosticTargetTypeMax: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(4i32);
pub const NRB_FUNCTION_ADD_LBAS_PINNED_SET: u32 = 16u32;
pub const NRB_FUNCTION_FLUSH_NVCACHE: u32 = 20u32;
pub const NRB_FUNCTION_NVCACHE_INFO: u32 = 236u32;
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_RETURN: u32 = 1u32;
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_SET: u32 = 0u32;
pub const NRB_FUNCTION_NVSEPARATED_FLUSH: u32 = 193u32;
pub const NRB_FUNCTION_NVSEPARATED_INFO: u32 = 192u32;
pub const NRB_FUNCTION_NVSEPARATED_WB_DISABLE: u32 = 194u32;
pub const NRB_FUNCTION_NVSEPARATED_WB_REVERT_DEFAULT: u32 = 195u32;
pub const NRB_FUNCTION_PASS_HINT_PAYLOAD: u32 = 224u32;
pub const NRB_FUNCTION_QUERY_ASCENDER_STATUS: u32 = 208u32;
pub const NRB_FUNCTION_QUERY_CACHE_MISS: u32 = 19u32;
pub const NRB_FUNCTION_QUERY_HYBRID_DISK_STATUS: u32 = 209u32;
pub const NRB_FUNCTION_QUERY_PINNED_SET: u32 = 18u32;
pub const NRB_FUNCTION_REMOVE_LBAS_PINNED_SET: u32 = 17u32;
pub const NRB_FUNCTION_SPINDLE_STATUS: u32 = 229u32;
pub const NRB_ILLEGAL_REQUEST: u32 = 1u32;
pub const NRB_INPUT_DATA_OVERRUN: u32 = 3u32;
pub const NRB_INPUT_DATA_UNDERRUN: u32 = 4u32;
pub const NRB_INVALID_PARAMETER: u32 = 2u32;
pub const NRB_OUTPUT_DATA_OVERRUN: u32 = 5u32;
pub const NRB_OUTPUT_DATA_UNDERRUN: u32 = 6u32;
pub const NRB_SUCCESS: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTSCSI_UNICODE_STRING(i32);
#[repr(C)]
pub struct NVCACHE_HINT_PAYLOAD(i32);
#[repr(C)]
pub struct NVCACHE_PRIORITY_LEVEL_DESCRIPTOR(i32);
#[repr(C)]
pub struct NVCACHE_REQUEST_BLOCK(i32);
#[repr(transparent)]
pub struct NVCACHE_STATUS(pub i32);
pub const NvCacheStatusUnknown: NVCACHE_STATUS = NVCACHE_STATUS(0i32);
pub const NvCacheStatusDisabling: NVCACHE_STATUS = NVCACHE_STATUS(1i32);
pub const NvCacheStatusDisabled: NVCACHE_STATUS = NVCACHE_STATUS(2i32);
pub const NvCacheStatusEnabled: NVCACHE_STATUS = NVCACHE_STATUS(3i32);
#[repr(transparent)]
pub struct NVCACHE_TYPE(pub i32);
pub const NvCacheTypeUnknown: NVCACHE_TYPE = NVCACHE_TYPE(0i32);
pub const NvCacheTypeNone: NVCACHE_TYPE = NVCACHE_TYPE(1i32);
pub const NvCacheTypeWriteBack: NVCACHE_TYPE = NVCACHE_TYPE(2i32);
pub const NvCacheTypeWriteThrough: NVCACHE_TYPE = NVCACHE_TYPE(3i32);
#[repr(C)]
pub struct NV_FEATURE_PARAMETER(i32);
#[repr(C)]
pub struct NV_SEP_CACHE_PARAMETER(i32);
pub const NV_SEP_CACHE_PARAMETER_VERSION: u32 = 1u32;
pub const NV_SEP_CACHE_PARAMETER_VERSION_1: u32 = 1u32;
#[repr(transparent)]
pub struct NV_SEP_WRITE_CACHE_TYPE(pub i32);
pub const NVSEPWriteCacheTypeUnknown: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(0i32);
pub const NVSEPWriteCacheTypeNone: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(1i32);
pub const NVSEPWriteCacheTypeWriteBack: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(2i32);
pub const NVSEPWriteCacheTypeWriteThrough: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(3i32);
pub type PDUMP_DEVICE_POWERON_ROUTINE = unsafe extern "system" fn() -> i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PERSISTENT_ISCSI_LOGIN_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PERSISTENT_ISCSI_LOGIN_INFOW(i32);
#[repr(C)]
pub struct SCSI_ADAPTER_BUS_INFO(i32);
#[repr(C)]
pub struct SCSI_ADDRESS(i32);
#[repr(C)]
pub struct SCSI_BUS_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SCSI_INQUIRY_DATA(i32);
pub const SCSI_IOCTL_DATA_BIDIRECTIONAL: u32 = 3u32;
pub const SCSI_IOCTL_DATA_IN: u32 = 1u32;
pub const SCSI_IOCTL_DATA_OUT: u32 = 0u32;
pub const SCSI_IOCTL_DATA_UNSPECIFIED: u32 = 2u32;
#[repr(C)]
pub struct SCSI_LUN_LIST(i32);
#[repr(C)]
pub struct SCSI_PASS_THROUGH(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct SCSI_PASS_THROUGH32(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct SCSI_PASS_THROUGH32_EX(i32);
#[repr(C)]
pub struct SCSI_PASS_THROUGH_DIRECT(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct SCSI_PASS_THROUGH_DIRECT32(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct SCSI_PASS_THROUGH_DIRECT32_EX(i32);
#[repr(C)]
pub struct SCSI_PASS_THROUGH_DIRECT_EX(i32);
#[repr(C)]
pub struct SCSI_PASS_THROUGH_EX(i32);
#[repr(C)]
pub struct SRB_IO_CONTROL(i32);
#[repr(C)]
pub struct STORAGE_DIAGNOSTIC_MP_REQUEST(i32);
pub const STORAGE_DIAGNOSTIC_STATUS_BUFFER_TOO_SMALL: u32 = 1u32;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_PARAMETER: u32 = 3u32;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_SIGNATURE: u32 = 4u32;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_TARGET_TYPE: u32 = 5u32;
pub const STORAGE_DIAGNOSTIC_STATUS_MORE_DATA: u32 = 6u32;
pub const STORAGE_DIAGNOSTIC_STATUS_SUCCESS: u32 = 0u32;
pub const STORAGE_DIAGNOSTIC_STATUS_UNSUPPORTED_VERSION: u32 = 2u32;
#[repr(C)]
pub struct STORAGE_ENDURANCE_DATA_DESCRIPTOR(i32);
#[repr(C)]
pub struct STORAGE_ENDURANCE_INFO(i32);
#[repr(C)]
pub struct STORAGE_FIRMWARE_ACTIVATE(i32);
pub const STORAGE_FIRMWARE_ACTIVATE_STRUCTURE_VERSION: u32 = 1u32;
#[repr(C)]
pub struct STORAGE_FIRMWARE_DOWNLOAD(i32);
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION: u32 = 1u32;
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION_V2: u32 = 2u32;
#[repr(C)]
pub struct STORAGE_FIRMWARE_DOWNLOAD_V2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STORAGE_FIRMWARE_INFO(i32);
pub const STORAGE_FIRMWARE_INFO_INVALID_SLOT: u32 = 255u32;
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION: u32 = 1u32;
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION_V2: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STORAGE_FIRMWARE_INFO_V2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STORAGE_FIRMWARE_SLOT_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STORAGE_FIRMWARE_SLOT_INFO_V2(i32);
pub const STORAGE_FIRMWARE_SLOT_INFO_V2_REVISION_LENGTH: u32 = 16u32;
pub const ScsiRawInterfaceGuid: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1408590601, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
#[repr(transparent)]
pub struct TARGETPROTOCOLTYPE(pub i32);
pub const ISCSI_TCP_PROTOCOL_TYPE: TARGETPROTOCOLTYPE = TARGETPROTOCOLTYPE(0i32);
#[repr(transparent)]
pub struct TARGET_INFORMATION_CLASS(pub i32);
pub const ProtocolType: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(0i32);
pub const TargetAlias: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(1i32);
pub const DiscoveryMechanisms: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(2i32);
pub const PortalGroups: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(3i32);
pub const PersistentTargetMappings: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(4i32);
pub const InitiatorName: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(5i32);
pub const TargetFlags: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(6i32);
pub const LoginOptions: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(7i32);
pub const WmiScsiAddressGuid: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1408590607, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
#[repr(C)]
pub struct _ADAPTER_OBJECT(i32);

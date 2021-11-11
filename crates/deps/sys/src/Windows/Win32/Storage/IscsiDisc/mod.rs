#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddISNSServerA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddISNSServerW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiConnectionA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiConnectionW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiSendTargetPortalA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiSendTargetPortalW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiStaticTargetA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIScsiStaticTargetW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPersistentIScsiDeviceA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPersistentIScsiDeviceW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddRadiusServerA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddRadiusServerW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn ClearPersistentIScsiDevices();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`, `Win32_System_Ioctl`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ioctl"))]
    pub fn GetDevicesForIScsiSessionA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_System_Ioctl`*"]
    #[cfg(feature = "Win32_System_Ioctl")]
    pub fn GetDevicesForIScsiSessionW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiIKEInfoA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiIKEInfoW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiInitiatorNodeNameA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiInitiatorNodeNameW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiSessionListA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiSessionListEx();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiSessionListW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiTargetInformationA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIScsiTargetInformationW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn GetIScsiVersionInformation();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoginIScsiTargetA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoginIScsiTargetW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn LogoutIScsiTarget();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshISNSServerA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshISNSServerW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshIScsiSendTargetPortalA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshIScsiSendTargetPortalW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveISNSServerA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveISNSServerW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn RemoveIScsiConnection();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiPersistentTargetA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiPersistentTargetW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiSendTargetPortalA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiSendTargetPortalW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiStaticTargetA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveIScsiStaticTargetW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePersistentIScsiDeviceA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePersistentIScsiDeviceW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveRadiusServerA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveRadiusServerW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportActiveIScsiTargetMappingsA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn ReportActiveIScsiTargetMappingsW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportISNSServerListA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportISNSServerListW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiInitiatorListA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiInitiatorListW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiPersistentLoginsA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiPersistentLoginsW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiSendTargetPortalsA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiSendTargetPortalsExA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn ReportIScsiSendTargetPortalsExW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn ReportIScsiSendTargetPortalsW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetPortalsA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetPortalsW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetsA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportIScsiTargetsW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportPersistentIScsiDevicesA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportPersistentIScsiDevicesW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportRadiusServerListA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportRadiusServerListW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SendScsiInquiry();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SendScsiReadCapacity();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SendScsiReportLuns();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiGroupPresharedKey();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiIKEInfoA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiIKEInfoW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SetIScsiInitiatorCHAPSharedSecret();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiInitiatorNodeNameA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiInitiatorNodeNameW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SetIScsiInitiatorRADIUSSharedSecret();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiTunnelModeOuterAddressA();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIScsiTunnelModeOuterAddressW();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SetupPersistentIScsiDevices();
    #[doc = "*Required features: `Win32_Storage_IscsiDisc`*"]
    pub fn SetupPersistentIScsiVolumes();
}

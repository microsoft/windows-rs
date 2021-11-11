#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ChangeServiceConfig2A();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ChangeServiceConfig2W();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ChangeServiceConfigA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ChangeServiceConfigW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CloseServiceHandle();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ControlService();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ControlServiceExA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ControlServiceExW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateServiceA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateServiceW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DeleteService();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EnumDependentServicesA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EnumDependentServicesW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EnumServicesStatusA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EnumServicesStatusExA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EnumServicesStatusExW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EnumServicesStatusW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetServiceDirectory();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn GetServiceDisplayNameA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn GetServiceDisplayNameW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn GetServiceKeyNameA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn GetServiceKeyNameW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetServiceRegistryStateKey();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn GetSharedServiceDirectory();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Security`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
    pub fn GetSharedServiceRegistryStateKey();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Security`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn LockServiceDatabase();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyBootConfigStatus();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NotifyServiceStatusChangeA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NotifyServiceStatusChangeW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenSCManagerA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenSCManagerW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenServiceA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenServiceW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceConfig2A();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceConfig2W();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceConfigA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceConfigW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryServiceDynamicInformation();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceLockStatusA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceLockStatusW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceObjectSecurity();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceStatus();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceStatusEx();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterServiceCtrlHandlerA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterServiceCtrlHandlerExA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterServiceCtrlHandlerExW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterServiceCtrlHandlerW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetServiceBits();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SetServiceObjectSecurity();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetServiceStatus();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn StartServiceA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartServiceCtrlDispatcherA();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartServiceCtrlDispatcherW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn StartServiceW();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockServiceDatabase();
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WaitServiceState();
}

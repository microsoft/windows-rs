#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpCancelMsg();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpCleanup();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpCleanupEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpClose();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpContextToStr();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpCountVbl();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpCreatePdu();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpCreateSession();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpCreateVbl();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpDecodeMsg();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpDeleteVb();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpDuplicatePdu();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpDuplicateVbl();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpEncodeMsg();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpEntityToStr();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpFreeContext();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpFreeDescriptor();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpFreeEntity();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpFreePdu();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpFreeVbl();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpGetLastError();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpGetPduData();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpGetRetransmitMode();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpGetRetry();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpGetTimeout();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpGetTranslateMode();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpGetVb();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpGetVendorInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpListen();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpListenEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrClose();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrCtl();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrGetTrap();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrGetTrapEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrOidToStr();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrOpen();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrRequest();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrStrToOid();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrTrapListen();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpOidCompare();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpOidCopy();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpOidToStr();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpOpen();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpRecvMsg();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpRegister();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpSendMsg();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpSetPduData();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpSetPort();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpSetRetransmitMode();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpSetRetry();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpSetTimeout();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpSetTranslateMode();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpSetVb();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpStartup();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpStartupEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpStrToContext();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpStrToEntity();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpStrToOid();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpSvcGetUptime();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpSvcSetLogLevel();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpSvcSetLogType();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilAsnAnyCpy();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilAsnAnyFree();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilDbgPrint();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilIdsToA();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpUtilMemAlloc();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpUtilMemFree();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpUtilMemReAlloc();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOctetsCmp();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOctetsCpy();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOctetsFree();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOctetsNCmp();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpUtilOidAppend();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpUtilOidCmp();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpUtilOidCpy();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpUtilOidFree();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpUtilOidNCmp();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOidToA();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilPrintAsnAny();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`*"]
    pub fn SnmpUtilPrintOid();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilVarBindCpy();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilVarBindFree();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilVarBindListCpy();
    #[doc = "*Required features: `Win32_NetworkManagement_Snmp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilVarBindListFree();
}

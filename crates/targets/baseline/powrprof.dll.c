void __stdcall CallNtPowerInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CanUserWritePwrScheme() {}
void __stdcall DeletePwrScheme(int p0) {}
void __stdcall DevicePowerClose() {}
void __stdcall DevicePowerEnumDevices(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall DevicePowerOpen(int p0) {}
void __stdcall DevicePowerSetDeviceState(int p0, int p1, int p2) {}
void __stdcall EnumPwrSchemes(int p0, int p1) {}
void __stdcall GetActivePwrScheme(int p0) {}
void __stdcall GetCurrentPowerPolicies(int p0, int p1) {}
void __stdcall GetPwrCapabilities(int p0) {}
void __stdcall GetPwrDiskSpindownRange(int p0, int p1) {}
void __stdcall IsAdminOverrideActive(int p0) {}
void __stdcall IsPwrHibernateAllowed() {}
void __stdcall IsPwrShutdownAllowed() {}
void __stdcall IsPwrSuspendAllowed() {}
void __stdcall PowerCanRestoreIndividualDefaultPowerScheme(int p0) {}
void __stdcall PowerCreatePossibleSetting(int p0, int p1, int p2, int p3) {}
void __stdcall PowerCreateSetting(int p0, int p1, int p2) {}
void __stdcall PowerDeleteScheme(int p0, int p1) {}
void __stdcall PowerDeterminePlatformRole() {}
void __stdcall PowerDeterminePlatformRoleEx(int p0) {}
void __stdcall PowerDuplicateScheme(int p0, int p1, int p2) {}
void __stdcall PowerEnumerate(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall PowerGetActiveScheme(int p0, int p1) {}
void __stdcall PowerImportPowerScheme(int p0, int p1, int p2) {}
void __stdcall PowerIsSettingRangeDefined(int p0, int p1) {}
void __stdcall PowerOpenSystemPowerKey(int p0, int p1, int p2) {}
void __stdcall PowerOpenUserPowerKey(int p0, int p1, int p2) {}
void __stdcall PowerReadACDefaultIndex(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PowerReadACValue(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall PowerReadACValueIndex(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PowerReadDCDefaultIndex(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PowerReadDCValue(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall PowerReadDCValueIndex(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PowerReadDescription(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall PowerReadFriendlyName(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall PowerReadIconResourceSpecifier(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall PowerReadPossibleDescription(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall PowerReadPossibleFriendlyName(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall PowerReadPossibleValue(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall PowerReadSettingAttributes(int p0, int p1) {}
void __stdcall PowerReadValueIncrement(int p0, int p1, int p2, int p3) {}
void __stdcall PowerReadValueMax(int p0, int p1, int p2, int p3) {}
void __stdcall PowerReadValueMin(int p0, int p1, int p2, int p3) {}
void __stdcall PowerReadValueUnitsSpecifier(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PowerRegisterForEffectivePowerModeNotifications(int p0, int p1, int p2, int p3) {}
void __stdcall PowerRegisterSuspendResumeNotification(int p0, int p1, int p2) {}
void __stdcall PowerRemovePowerSetting(int p0, int p1) {}
void __stdcall PowerReplaceDefaultPowerSchemes() {}
void __stdcall PowerReportThermalEvent(int p0) {}
void __stdcall PowerRestoreDefaultPowerSchemes() {}
void __stdcall PowerRestoreIndividualDefaultPowerScheme(int p0) {}
void __stdcall PowerSetActiveScheme(int p0, int p1) {}
void __stdcall PowerSettingAccessCheck(int p0, int p1) {}
void __stdcall PowerSettingAccessCheckEx(int p0, int p1, int p2) {}
void __stdcall PowerSettingRegisterNotification(int p0, int p1, int p2, int p3) {}
void __stdcall PowerSettingUnregisterNotification(int p0) {}
void __stdcall PowerUnregisterFromEffectivePowerModeNotifications(int p0) {}
void __stdcall PowerUnregisterSuspendResumeNotification(int p0) {}
void __stdcall PowerWriteACDefaultIndex(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PowerWriteACValueIndex(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PowerWriteDCDefaultIndex(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PowerWriteDCValueIndex(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PowerWriteDescription(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall PowerWriteFriendlyName(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall PowerWriteIconResourceSpecifier(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall PowerWritePossibleDescription(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall PowerWritePossibleFriendlyName(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall PowerWritePossibleValue(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall PowerWriteSettingAttributes(int p0, int p1, int p2) {}
void __stdcall PowerWriteValueIncrement(int p0, int p1, int p2, int p3) {}
void __stdcall PowerWriteValueMax(int p0, int p1, int p2, int p3) {}
void __stdcall PowerWriteValueMin(int p0, int p1, int p2, int p3) {}
void __stdcall PowerWriteValueUnitsSpecifier(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ReadGlobalPwrPolicy(int p0) {}
void __stdcall ReadProcessorPwrScheme(int p0, int p1) {}
void __stdcall ReadPwrScheme(int p0, int p1) {}
void __stdcall SetActivePwrScheme(int p0, int p1, int p2) {}
void __stdcall SetSuspendState(int p0, int p1, int p2) {}
void __stdcall ValidatePowerPolicies(int p0, int p1) {}
void __stdcall WriteGlobalPwrPolicy(int p0) {}
void __stdcall WriteProcessorPwrScheme(int p0, int p1) {}
void __stdcall WritePwrScheme(int p0, int p1, int p2, int p3) {}

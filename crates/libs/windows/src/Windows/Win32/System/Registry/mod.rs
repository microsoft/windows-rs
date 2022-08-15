pub const AGP_FLAG_NO_1X_RATE: i32 = 1i32;
pub const AGP_FLAG_NO_2X_RATE: i32 = 2i32;
pub const AGP_FLAG_NO_4X_RATE: i32 = 4i32;
pub const AGP_FLAG_NO_8X_RATE: i32 = 8i32;
pub const AGP_FLAG_NO_FW_ENABLE: i32 = 512i32;
pub const AGP_FLAG_NO_SBA_ENABLE: i32 = 256i32;
pub const AGP_FLAG_REVERSE_INITIALIZATION: i32 = 128i32;
pub const AGP_FLAG_SPECIAL_RESERVE: i32 = 1015808i32;
pub const AGP_FLAG_SPECIAL_TARGET: i32 = 1048575i32;
pub const APMMENUSUSPEND_DISABLED: u32 = 0u32;
pub const APMMENUSUSPEND_ENABLED: u32 = 1u32;
pub const APMMENUSUSPEND_NOCHANGE: u32 = 128u32;
pub const APMMENUSUSPEND_UNDOCKED: u32 = 2u32;
pub const APMTIMEOUT_DISABLED: u32 = 0u32;
pub const BIF_RAWDEVICENEEDSDRIVER: u32 = 2u32;
pub const BIF_SHOWSIMILARDRIVERS: u32 = 1u32;
pub const CONFIGFLAG_BOOT_DEVICE: u32 = 262144u32;
pub const CONFIGFLAG_CANTSTOPACHILD: u32 = 128u32;
pub const CONFIGFLAG_DISABLED: u32 = 1u32;
pub const CONFIGFLAG_FAILEDINSTALL: u32 = 64u32;
pub const CONFIGFLAG_FINISHINSTALL_ACTION: u32 = 131072u32;
pub const CONFIGFLAG_FINISHINSTALL_UI: u32 = 65536u32;
pub const CONFIGFLAG_FINISH_INSTALL: u32 = 1024u32;
pub const CONFIGFLAG_IGNORE_BOOT_LC: u32 = 8u32;
pub const CONFIGFLAG_MANUAL_INSTALL: u32 = 4u32;
pub const CONFIGFLAG_NEEDS_CLASS_CONFIG: u32 = 524288u32;
pub const CONFIGFLAG_NEEDS_FORCED_CONFIG: u32 = 2048u32;
pub const CONFIGFLAG_NETBOOT_CARD: u32 = 4096u32;
pub const CONFIGFLAG_NET_BOOT: u32 = 16u32;
pub const CONFIGFLAG_NOREMOVEEXIT: u32 = 512u32;
pub const CONFIGFLAG_OKREMOVEROM: u32 = 256u32;
pub const CONFIGFLAG_PARTIAL_LOG_CONF: u32 = 8192u32;
pub const CONFIGFLAG_REINSTALL: u32 = 32u32;
pub const CONFIGFLAG_REMOVED: u32 = 2u32;
pub const CONFIGFLAG_SUPPRESS_SURPRISE: u32 = 16384u32;
pub const CONFIGFLAG_VERIFY_HARDWARE: u32 = 32768u32;
pub const CSCONFIGFLAG_BITS: u32 = 7u32;
pub const CSCONFIGFLAG_DISABLED: u32 = 1u32;
pub const CSCONFIGFLAG_DO_NOT_CREATE: u32 = 2u32;
pub const CSCONFIGFLAG_DO_NOT_START: u32 = 4u32;
pub const DMSTATEFLAG_APPLYTOALL: u32 = 1u32;
pub const DOSOPTF_ALWAYSUSE: i32 = 4i32;
pub const DOSOPTF_DEFAULT: i32 = 1i32;
pub const DOSOPTF_INDOSSTART: i32 = 64i32;
pub const DOSOPTF_MULTIPLE: i32 = 128i32;
pub const DOSOPTF_NEEDSETUP: i32 = 32i32;
pub const DOSOPTF_PROVIDESUMB: i32 = 16i32;
pub const DOSOPTF_SUPPORTED: i32 = 2i32;
pub const DOSOPTF_USESPMODE: i32 = 8i32;
pub const DOSOPTGF_DEFCLEAN: i32 = 1i32;
pub const DRIVERSIGN_BLOCKING: u32 = 2u32;
pub const DRIVERSIGN_NONE: u32 = 0u32;
pub const DRIVERSIGN_WARNING: u32 = 1u32;
#[repr(C)]
pub struct DSKTLSYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
    pub wResult: u16,
}
impl ::core::marker::Copy for DSKTLSYSTEMTIME {}
impl ::core::clone::Clone for DSKTLSYSTEMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSKTLSYSTEMTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSKTLSYSTEMTIME").field("wYear", &self.wYear).field("wMonth", &self.wMonth).field("wDayOfWeek", &self.wDayOfWeek).field("wDay", &self.wDay).field("wHour", &self.wHour).field("wMinute", &self.wMinute).field("wSecond", &self.wSecond).field("wMilliseconds", &self.wMilliseconds).field("wResult", &self.wResult).finish()
    }
}
unsafe impl ::windows::core::Abi for DSKTLSYSTEMTIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSKTLSYSTEMTIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSKTLSYSTEMTIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSKTLSYSTEMTIME {}
impl ::core::default::Default for DSKTLSYSTEMTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DTRESULTFIX: u32 = 1u32;
pub const DTRESULTOK: u32 = 0u32;
pub const DTRESULTPART: u32 = 3u32;
pub const DTRESULTPROB: u32 = 2u32;
pub const EISAFLAG_NO_IO_MERGE: u32 = 1u32;
pub const EISAFLAG_SLOT_IO_FIRST: u32 = 2u32;
pub const EISA_NO_MAX_FUNCTION: u32 = 255u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRegistryValueWithFallbackW<'a, P0, P1, P2, P3, P4>(hkeyprimary: P0, pwszprimarysubkey: P1, hkeyfallback: P2, pwszfallbacksubkey: P3, pwszvalue: P4, dwflags: u32, pdwtype: ::core::option::Option<&mut u32>, pvdata: *mut ::core::ffi::c_void, cbdatain: u32, pcbdataout: ::core::option::Option<&mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<HKEY>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
    P4: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetRegistryValueWithFallbackW(hkeyprimary: HKEY, pwszprimarysubkey: ::windows::core::PCWSTR, hkeyfallback: HKEY, pwszfallbacksubkey: ::windows::core::PCWSTR, pwszvalue: ::windows::core::PCWSTR, dwflags: u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, cbdatain: u32, pcbdataout: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    }
    GetRegistryValueWithFallbackW(hkeyprimary.into(), pwszprimarysubkey.into(), hkeyfallback.into(), pwszfallbacksubkey.into(), pwszvalue.into(), dwflags, ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvdata), cbdatain, ::core::mem::transmute(pcbdataout))
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HKEY(pub isize);
impl HKEY {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HKEY {}
impl ::core::fmt::Debug for HKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HKEY").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HKEY>> for HKEY {
    fn from(optional: ::core::option::Option<HKEY>) -> HKEY {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HKEY {
    type Abi = Self;
}
pub const HKEY_CLASSES_ROOT: HKEY = HKEY(-2147483648i32 as _);
pub const HKEY_CURRENT_CONFIG: HKEY = HKEY(-2147483643i32 as _);
pub const HKEY_CURRENT_USER: HKEY = HKEY(-2147483647i32 as _);
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: HKEY = HKEY(-2147483641i32 as _);
pub const HKEY_DYN_DATA: HKEY = HKEY(-2147483642i32 as _);
pub const HKEY_LOCAL_MACHINE: HKEY = HKEY(-2147483646i32 as _);
pub const HKEY_PERFORMANCE_DATA: HKEY = HKEY(-2147483644i32 as _);
pub const HKEY_PERFORMANCE_NLSTEXT: HKEY = HKEY(-2147483552i32 as _);
pub const HKEY_PERFORMANCE_TEXT: HKEY = HKEY(-2147483568i32 as _);
pub const HKEY_USERS: HKEY = HKEY(-2147483645i32 as _);
pub const IT_COMPACT: u32 = 0u32;
pub const IT_CUSTOM: u32 = 3u32;
pub const IT_PORTABLE: u32 = 2u32;
pub const IT_TYPICAL: u32 = 1u32;
pub const LASTGOOD_OPERATION: u32 = 255u32;
pub const LASTGOOD_OPERATION_DELETE: u32 = 1u32;
pub const LASTGOOD_OPERATION_NOPOSTPROC: u32 = 0u32;
pub const MF_FLAGS_CREATE_BUT_NO_SHOW_DISABLED: u32 = 8u32;
pub const MF_FLAGS_EVEN_IF_NO_RESOURCE: u32 = 1u32;
pub const MF_FLAGS_FILL_IN_UNKNOWN_RESOURCE: u32 = 4u32;
pub const MF_FLAGS_NO_CREATE_IF_NO_RESOURCE: u32 = 2u32;
pub const NUM_EISA_RANGES: u32 = 4u32;
pub const NUM_RESOURCE_MAP: u32 = 256u32;
pub const PCIC_DEFAULT_IRQMASK: u32 = 20152u32;
pub const PCIC_DEFAULT_NUMSOCKETS: u32 = 0u32;
pub const PCI_OPTIONS_USE_BIOS: i32 = 1i32;
pub const PCI_OPTIONS_USE_IRQ_STEERING: i32 = 2i32;
pub const PCMCIA_DEF_MEMBEGIN: u32 = 786432u32;
pub const PCMCIA_DEF_MEMEND: u32 = 16777215u32;
pub const PCMCIA_DEF_MEMLEN: u32 = 4096u32;
pub const PCMCIA_DEF_MIN_REGION: u32 = 65536u32;
pub const PCMCIA_OPT_AUTOMEM: i32 = 4i32;
pub const PCMCIA_OPT_HAVE_SOCKET: i32 = 1i32;
pub const PCMCIA_OPT_NO_APMREMOVE: i32 = 32i32;
pub const PCMCIA_OPT_NO_AUDIO: i32 = 16i32;
pub const PCMCIA_OPT_NO_SOUND: i32 = 8i32;
pub const PIR_OPTION_DEFAULT: u32 = 15u32;
pub const PIR_OPTION_ENABLED: u32 = 1u32;
pub const PIR_OPTION_MSSPEC: u32 = 4u32;
pub const PIR_OPTION_REALMODE: u32 = 8u32;
pub const PIR_OPTION_REGISTRY: u32 = 2u32;
pub const PIR_STATUS_DISABLED: u32 = 2u32;
pub const PIR_STATUS_ENABLED: u32 = 1u32;
pub const PIR_STATUS_ERROR: u32 = 0u32;
pub const PIR_STATUS_MAX: u32 = 3u32;
pub const PIR_STATUS_MINIPORT_COMPATIBLE: u32 = 1u32;
pub const PIR_STATUS_MINIPORT_ERROR: u32 = 4u32;
pub const PIR_STATUS_MINIPORT_INVALID: u32 = 7u32;
pub const PIR_STATUS_MINIPORT_MAX: u32 = 8u32;
pub const PIR_STATUS_MINIPORT_NOKEY: u32 = 5u32;
pub const PIR_STATUS_MINIPORT_NONE: u32 = 3u32;
pub const PIR_STATUS_MINIPORT_NORMAL: u32 = 0u32;
pub const PIR_STATUS_MINIPORT_OVERRIDE: u32 = 2u32;
pub const PIR_STATUS_MINIPORT_SUCCESS: u32 = 6u32;
pub const PIR_STATUS_TABLE_BAD: u32 = 5u32;
pub const PIR_STATUS_TABLE_ERROR: u32 = 4u32;
pub const PIR_STATUS_TABLE_MAX: u32 = 7u32;
pub const PIR_STATUS_TABLE_MSSPEC: u32 = 1u32;
pub const PIR_STATUS_TABLE_NONE: u32 = 3u32;
pub const PIR_STATUS_TABLE_REALMODE: u32 = 2u32;
pub const PIR_STATUS_TABLE_REGISTRY: u32 = 0u32;
pub const PIR_STATUS_TABLE_SUCCESS: u32 = 6u32;
pub type PQUERYHANDLER = ::core::option::Option<unsafe extern "system" fn(keycontext: *mut ::core::ffi::c_void, val_list: *mut val_context, num_vals: u32, outputbuffer: *mut ::core::ffi::c_void, total_outlen: *mut u32, input_blen: u32) -> u32>;
pub const PROVIDER_KEEPS_VALUE_LENGTH: u32 = 1u32;
pub const REGDF_CONFLICTDMA: u32 = 524288u32;
pub const REGDF_CONFLICTIO: u32 = 65536u32;
pub const REGDF_CONFLICTIRQ: u32 = 262144u32;
pub const REGDF_CONFLICTMEM: u32 = 131072u32;
pub const REGDF_GENFORCEDCONFIG: u32 = 32u32;
pub const REGDF_MAPIRQ2TO9: u32 = 1048576u32;
pub const REGDF_NEEDFULLCONFIG: u32 = 16u32;
pub const REGDF_NODETCONFIG: u32 = 32768u32;
pub const REGDF_NOTDETDMA: u32 = 8u32;
pub const REGDF_NOTDETIO: u32 = 1u32;
pub const REGDF_NOTDETIRQ: u32 = 4u32;
pub const REGDF_NOTDETMEM: u32 = 2u32;
pub const REGDF_NOTVERIFIED: u32 = 2147483648u32;
pub const REGSTR_DATA_NETOS_IPX: &str = "IPX";
pub const REGSTR_DATA_NETOS_NDIS: &str = "NDIS";
pub const REGSTR_DATA_NETOS_ODI: &str = "ODI";
pub const REGSTR_DEFAULT_INSTANCE: &str = "0000";
pub const REGSTR_KEY_ACPIENUM: &str = "ACPI";
pub const REGSTR_KEY_APM: &str = "*PNP0C05";
pub const REGSTR_KEY_BIOSENUM: &str = "BIOS";
pub const REGSTR_KEY_CLASS: &str = "Class";
pub const REGSTR_KEY_CONFIG: &str = "Config";
pub const REGSTR_KEY_CONTROL: &str = "Control";
pub const REGSTR_KEY_CRASHES: &str = "Crashes";
pub const REGSTR_KEY_CURRENT: &str = "Current";
pub const REGSTR_KEY_CURRENT_ENV: &str = "\\Windows 4.0";
pub const REGSTR_KEY_DANGERS: &str = "Dangers";
pub const REGSTR_KEY_DEFAULT: &str = "Default";
pub const REGSTR_KEY_DETMODVARS: &str = "DetModVars";
pub const REGSTR_KEY_DEVICEPARAMETERS: &str = "Device Parameters";
pub const REGSTR_KEY_DEVICE_PROPERTIES: &str = "Properties";
pub const REGSTR_KEY_DISPLAY_CLASS: &str = "Display";
pub const REGSTR_KEY_DOSOPTCDROM: &str = "CD-ROM";
pub const REGSTR_KEY_DOSOPTMOUSE: &str = "MOUSE";
pub const REGSTR_KEY_DRIVERPARAMETERS: &str = "Driver Parameters";
pub const REGSTR_KEY_DRIVERS: &str = "\\Drivers";
pub const REGSTR_KEY_EBDAUTOEXECBATKEYBOARD: &str = "EBDAutoexecBatKeyboard";
pub const REGSTR_KEY_EBDAUTOEXECBATLOCAL: &str = "EBDAutoexecBatLocale";
pub const REGSTR_KEY_EBDCONFIGSYSKEYBOARD: &str = "EBDConfigSysKeyboard";
pub const REGSTR_KEY_EBDCONFIGSYSLOCAL: &str = "EBDConfigSysLocale";
pub const REGSTR_KEY_EBDFILESKEYBOARD: &str = "EBDFilesKeyboard";
pub const REGSTR_KEY_EBDFILESLOCAL: &str = "EBDFilesLocale";
pub const REGSTR_KEY_EISAENUM: &str = "EISA";
pub const REGSTR_KEY_ENUM: &str = "Enum";
pub const REGSTR_KEY_EXPLORER: &str = "Explorer";
pub const REGSTR_KEY_FILTERS: &str = "Filters";
pub const REGSTR_KEY_INIUPDATE: &str = "IniUpdate";
pub const REGSTR_KEY_ISAENUM: &str = "ISAPnP";
pub const REGSTR_KEY_JOYCURR: &str = "CurrentJoystickSettings";
pub const REGSTR_KEY_JOYSETTINGS: &str = "JoystickSettings";
pub const REGSTR_KEY_KEYBOARD_CLASS: &str = "Keyboard";
pub const REGSTR_KEY_KNOWNDOCKINGSTATES: &str = "Hardware Profiles";
pub const REGSTR_KEY_LOGCONFIG: &str = "LogConfig";
pub const REGSTR_KEY_LOGON: &str = "\\Logon";
pub const REGSTR_KEY_LOWER_FILTER_LEVEL_DEFAULT: &str = "*Lower";
pub const REGSTR_KEY_MEDIA_CLASS: &str = "MEDIA";
pub const REGSTR_KEY_MODEM_CLASS: &str = "Modem";
pub const REGSTR_KEY_MODES: &str = "Modes";
pub const REGSTR_KEY_MONITOR_CLASS: &str = "Monitor";
pub const REGSTR_KEY_MOUSE_CLASS: &str = "Mouse";
pub const REGSTR_KEY_NDISINFO: &str = "NDISInfo";
pub const REGSTR_KEY_NETWORK: &str = "Network";
pub const REGSTR_KEY_NETWORKPROVIDER: &str = "\\NetworkProvider";
pub const REGSTR_KEY_NETWORK_PERSISTENT: &str = "\\Persistent";
pub const REGSTR_KEY_NETWORK_RECENT: &str = "\\Recent";
pub const REGSTR_KEY_OVERRIDE: &str = "Override";
pub const REGSTR_KEY_PCIENUM: &str = "PCI";
pub const REGSTR_KEY_PCMCIA: &str = "PCMCIA\\";
pub const REGSTR_KEY_PCMCIAENUM: &str = "PCMCIA";
pub const REGSTR_KEY_PCMCIA_CLASS: &str = "PCMCIA";
pub const REGSTR_KEY_PCMTD: &str = "MTD-";
pub const REGSTR_KEY_PCUNKNOWN: &str = "UNKNOWN_MANUFACTURER";
pub const REGSTR_KEY_POL_COMPUTERS: &str = "Computers";
pub const REGSTR_KEY_POL_DEFAULT: &str = ".default";
pub const REGSTR_KEY_POL_USERGROUPDATA: &str = "GroupData\\UserGroups\\Priority";
pub const REGSTR_KEY_POL_USERGROUPS: &str = "UserGroups";
pub const REGSTR_KEY_POL_USERS: &str = "Users";
pub const REGSTR_KEY_PORTS_CLASS: &str = "ports";
pub const REGSTR_KEY_PRINTERS: &str = "Printers";
pub const REGSTR_KEY_PRINT_PROC: &str = "\\Print Processors";
pub const REGSTR_KEY_ROOTENUM: &str = "Root";
pub const REGSTR_KEY_RUNHISTORY: &str = "RunHistory";
pub const REGSTR_KEY_SCSI_CLASS: &str = "SCSIAdapter";
pub const REGSTR_KEY_SETUP: &str = "\\Setup";
pub const REGSTR_KEY_SHARES: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Network\\LanMan";
pub const REGSTR_KEY_SYSTEM: &str = "System";
pub const REGSTR_KEY_SYSTEMBOARD: &str = "*PNP0C01";
pub const REGSTR_KEY_UPPER_FILTER_LEVEL_DEFAULT: &str = "*Upper";
pub const REGSTR_KEY_USER: &str = "User";
pub const REGSTR_KEY_VPOWERDENUM: &str = "VPOWERD";
pub const REGSTR_KEY_WINOLDAPP: &str = "WinOldApp";
pub const REGSTR_MACHTYPE_ATT_PC: &str = "AT&T PC";
pub const REGSTR_MACHTYPE_HP_VECTRA: &str = "HP Vectra";
pub const REGSTR_MACHTYPE_IBMPC: &str = "IBM PC";
pub const REGSTR_MACHTYPE_IBMPCAT: &str = "IBM PC/AT";
pub const REGSTR_MACHTYPE_IBMPCCONV: &str = "IBM PC Convertible";
pub const REGSTR_MACHTYPE_IBMPCJR: &str = "IBM PCjr";
pub const REGSTR_MACHTYPE_IBMPCXT: &str = "IBM PC/XT";
pub const REGSTR_MACHTYPE_IBMPCXT_286: &str = "IBM PC/XT 286";
pub const REGSTR_MACHTYPE_IBMPS1: &str = "IBM PS/1";
pub const REGSTR_MACHTYPE_IBMPS2_25: &str = "IBM PS/2-25";
pub const REGSTR_MACHTYPE_IBMPS2_30: &str = "IBM PS/2-30";
pub const REGSTR_MACHTYPE_IBMPS2_30_286: &str = "IBM PS/2-30 286";
pub const REGSTR_MACHTYPE_IBMPS2_50: &str = "IBM PS/2-50";
pub const REGSTR_MACHTYPE_IBMPS2_50Z: &str = "IBM PS/2-50Z";
pub const REGSTR_MACHTYPE_IBMPS2_55SX: &str = "IBM PS/2-55SX";
pub const REGSTR_MACHTYPE_IBMPS2_60: &str = "IBM PS/2-60";
pub const REGSTR_MACHTYPE_IBMPS2_65SX: &str = "IBM PS/2-65SX";
pub const REGSTR_MACHTYPE_IBMPS2_70: &str = "IBM PS/2-70";
pub const REGSTR_MACHTYPE_IBMPS2_70_80: &str = "IBM PS/2-70/80";
pub const REGSTR_MACHTYPE_IBMPS2_80: &str = "IBM PS/2-80";
pub const REGSTR_MACHTYPE_IBMPS2_90: &str = "IBM PS/2-90";
pub const REGSTR_MACHTYPE_IBMPS2_P70: &str = "IBM PS/2-P70";
pub const REGSTR_MACHTYPE_PHOENIX_PCAT: &str = "Phoenix PC/AT Compatible";
pub const REGSTR_MACHTYPE_UNKNOWN: &str = "Unknown";
pub const REGSTR_MACHTYPE_ZENITH_PC: &str = "Zenith PC";
pub const REGSTR_MAX_VALUE_LENGTH: u32 = 256u32;
pub const REGSTR_PATH_ADDRARB: &str = "System\\CurrentControlSet\\Services\\Arbitrators\\AddrArb";
pub const REGSTR_PATH_AEDEBUG: &str = "Software\\Microsoft\\Windows NT\\CurrentVersion\\AeDebug";
pub const REGSTR_PATH_APPEARANCE: &str = "Control Panel\\Appearance";
pub const REGSTR_PATH_APPPATCH: &str = "System\\CurrentControlSet\\Control\\SessionManager\\AppPatches";
pub const REGSTR_PATH_APPPATHS: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\App Paths";
pub const REGSTR_PATH_BIOSINFO: &str = "System\\CurrentControlSet\\Control\\BiosInfo";
pub const REGSTR_PATH_BUSINFORMATION: &str = "System\\CurrentControlSet\\Control\\PnP\\BusInformation";
pub const REGSTR_PATH_CDFS: &str = "System\\CurrentControlSet\\Control\\FileSystem\\CDFS";
pub const REGSTR_PATH_CHECKBADAPPS: &str = "System\\CurrentControlSet\\Control\\SessionManager\\CheckBadApps";
pub const REGSTR_PATH_CHECKBADAPPS400: &str = "System\\CurrentControlSet\\Control\\SessionManager\\CheckBadApps400";
pub const REGSTR_PATH_CHECKDISK: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive";
pub const REGSTR_PATH_CHECKDISKSET: &str = "Settings";
pub const REGSTR_PATH_CHECKDISKUDRVS: &str = "NoUnknownDDErrDrvs";
pub const REGSTR_PATH_CHECKVERDLLS: &str = "System\\CurrentControlSet\\Control\\SessionManager\\CheckVerDLLs";
pub const REGSTR_PATH_CHILD_PREFIX: &str = "Child";
pub const REGSTR_PATH_CHKLASTCHECK: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive\\LastCheck";
pub const REGSTR_PATH_CHKLASTSURFAN: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive\\LastSurfaceAnalysis";
pub const REGSTR_PATH_CLASS: &str = "System\\CurrentControlSet\\Services\\Class";
pub const REGSTR_PATH_CLASS_NT: &str = "System\\CurrentControlSet\\Control\\Class";
pub const REGSTR_PATH_CODEPAGE: &str = "System\\CurrentControlSet\\Control\\Nls\\Codepage";
pub const REGSTR_PATH_CODEVICEINSTALLERS: &str = "System\\CurrentControlSet\\Control\\CoDeviceInstallers";
pub const REGSTR_PATH_COLORS: &str = "Control Panel\\Colors";
pub const REGSTR_PATH_COMPUTRNAME: &str = "System\\CurrentControlSet\\Control\\ComputerName\\ComputerName";
pub const REGSTR_PATH_CONTROLPANEL: &str = "Control Panel";
pub const REGSTR_PATH_CONTROLSFOLDER: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Controls Folder";
pub const REGSTR_PATH_CRITICALDEVICEDATABASE: &str = "System\\CurrentControlSet\\Control\\CriticalDeviceDatabase";
pub const REGSTR_PATH_CURRENTCONTROLSET: &str = "System\\CurrentControlSet";
pub const REGSTR_PATH_CURRENT_CONTROL_SET: &str = "System\\CurrentControlSet\\Control";
pub const REGSTR_PATH_CURSORS: &str = "Control Panel\\Cursors";
pub const REGSTR_PATH_CVNETWORK: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Network";
pub const REGSTR_PATH_DESKTOP: &str = "Control Panel\\Desktop";
pub const REGSTR_PATH_DETECT: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Detect";
pub const REGSTR_PATH_DEVICEINSTALLER: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Device Installer";
pub const REGSTR_PATH_DEVICE_CLASSES: &str = "System\\CurrentControlSet\\Control\\DeviceClasses";
pub const REGSTR_PATH_DIFX: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\DIFX";
pub const REGSTR_PATH_DISPLAYSETTINGS: &str = "Display\\Settings";
pub const REGSTR_PATH_DMAARB: &str = "System\\CurrentControlSet\\Services\\Arbitrators\\DMAArb";
pub const REGSTR_PATH_DRIVERSIGN: &str = "Software\\Microsoft\\Driver Signing";
pub const REGSTR_PATH_DRIVERSIGN_POLICY: &str = "Software\\Policies\\Microsoft\\Windows NT\\Driver Signing";
pub const REGSTR_PATH_ENUM: &str = "Enum";
pub const REGSTR_PATH_ENVIRONMENTS: &str = "System\\CurrentControlSet\\Control\\Print\\Environments";
pub const REGSTR_PATH_EVENTLABELS: &str = "AppEvents\\EventLabels";
pub const REGSTR_PATH_EXPLORER: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer";
pub const REGSTR_PATH_FAULT: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Fault";
pub const REGSTR_PATH_FILESYSTEM: &str = "System\\CurrentControlSet\\Control\\FileSystem";
pub const REGSTR_PATH_FILESYSTEM_NOVOLTRACK: &str = "System\\CurrentControlSet\\Control\\FileSystem\\NoVolTrack";
pub const REGSTR_PATH_FLOATINGPOINTPROCESSOR: &str = "HARDWARE\\DESCRIPTION\\System\\FloatingPointProcessor";
pub const REGSTR_PATH_FLOATINGPOINTPROCESSOR0: &str = "HARDWARE\\DESCRIPTION\\System\\FloatingPointProcessor\\0";
pub const REGSTR_PATH_FONTS: &str = "Display\\Fonts";
pub const REGSTR_PATH_GRPCONV: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\GrpConv";
pub const REGSTR_PATH_HACKINIFILE: &str = "System\\CurrentControlSet\\Control\\SessionManager\\HackIniFiles";
pub const REGSTR_PATH_HWPROFILES: &str = "System\\CurrentControlSet\\Hardware Profiles";
pub const REGSTR_PATH_HWPROFILESCURRENT: &str = "System\\CurrentControlSet\\Hardware Profiles\\Current";
pub const REGSTR_PATH_ICONS: &str = "Control Panel\\Icons";
pub const REGSTR_PATH_IDCONFIGDB: &str = "System\\CurrentControlSet\\Control\\IDConfigDB";
pub const REGSTR_PATH_INSTALLEDFILES: &str = "System\\CurrentControlSet\\Control\\InstalledFiles";
pub const REGSTR_PATH_IOARB: &str = "System\\CurrentControlSet\\Services\\Arbitrators\\IOArb";
pub const REGSTR_PATH_IOS: &str = "System\\CurrentControlSet\\Services\\VxD\\IOS";
pub const REGSTR_PATH_IRQARB: &str = "System\\CurrentControlSet\\Services\\Arbitrators\\IRQArb";
pub const REGSTR_PATH_KEYBOARD: &str = "Control Panel\\Keyboard";
pub const REGSTR_PATH_KNOWN16DLLS: &str = "System\\CurrentControlSet\\Control\\SessionManager\\Known16DLLs";
pub const REGSTR_PATH_KNOWNDLLS: &str = "System\\CurrentControlSet\\Control\\SessionManager\\KnownDLLs";
pub const REGSTR_PATH_KNOWNVXDS: &str = "System\\CurrentControlSet\\Control\\SessionManager\\KnownVxDs";
pub const REGSTR_PATH_LASTBACKUP: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastBackup";
pub const REGSTR_PATH_LASTCHECK: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastCheck";
pub const REGSTR_PATH_LASTGOOD: &str = "System\\LastKnownGoodRecovery\\LastGood";
pub const REGSTR_PATH_LASTGOODTMP: &str = "System\\LastKnownGoodRecovery\\LastGood.Tmp";
pub const REGSTR_PATH_LASTOPTIMIZE: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastOptimize";
pub const REGSTR_PATH_LOOKSCHEMES: &str = "Control Panel\\Appearance\\Schemes";
pub const REGSTR_PATH_METRICS: &str = "Control Panel\\Desktop\\WindowMetrics";
pub const REGSTR_PATH_MONITORS: &str = "System\\CurrentControlSet\\Control\\Print\\Monitors";
pub const REGSTR_PATH_MOUSE: &str = "Control Panel\\Mouse";
pub const REGSTR_PATH_MSDOSOPTS: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\MS-DOSOptions";
pub const REGSTR_PATH_MULTIMEDIA_AUDIO: &str = "Software\\Microsoft\\Multimedia\\Audio";
pub const REGSTR_PATH_MULTI_FUNCTION: &str = "MF";
pub const REGSTR_PATH_NCPSERVER: &str = "System\\CurrentControlSet\\Services\\NcpServer\\Parameters";
pub const REGSTR_PATH_NETEQUIV: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Network\\Equivalent";
pub const REGSTR_PATH_NETWORK_USERSETTINGS: &str = "Network";
pub const REGSTR_PATH_NEWDOSBOX: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\MS-DOSSpecialConfig";
pub const REGSTR_PATH_NONDRIVERSIGN: &str = "Software\\Microsoft\\Non-Driver Signing";
pub const REGSTR_PATH_NONDRIVERSIGN_POLICY: &str = "Software\\Policies\\Microsoft\\Windows NT\\Non-Driver Signing";
pub const REGSTR_PATH_NOSUGGMSDOS: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\NoMSDOSWarn";
pub const REGSTR_PATH_NT_CURRENTVERSION: &str = "Software\\Microsoft\\Windows NT\\CurrentVersion";
pub const REGSTR_PATH_NWREDIR: &str = "System\\CurrentControlSet\\Services\\VxD\\NWREDIR";
pub const REGSTR_PATH_PCIIR: &str = "System\\CurrentControlSet\\Control\\Pnp\\PciIrqRouting";
pub const REGSTR_PATH_PER_HW_ID_STORAGE: &str = "Software\\Microsoft\\Windows NT\\CurrentVersion\\PerHwIdStorage";
pub const REGSTR_PATH_PIFCONVERT: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\PIFConvert";
pub const REGSTR_PATH_POLICIES: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Policies";
pub const REGSTR_PATH_PRINT: &str = "System\\CurrentControlSet\\Control\\Print";
pub const REGSTR_PATH_PRINTERS: &str = "System\\CurrentControlSet\\Control\\Print\\Printers";
pub const REGSTR_PATH_PROPERTYSYSTEM: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\PropertySystem";
pub const REGSTR_PATH_PROVIDERS: &str = "System\\CurrentControlSet\\Control\\Print\\Providers";
pub const REGSTR_PATH_PWDPROVIDER: &str = "System\\CurrentControlSet\\Control\\PwdProvider";
pub const REGSTR_PATH_REALMODENET: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Network\\Real Mode Net";
pub const REGSTR_PATH_REINSTALL: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Reinstall";
pub const REGSTR_PATH_RELIABILITY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Reliability";
pub const REGSTR_PATH_RELIABILITY_POLICY: &str = "Software\\Policies\\Microsoft\\Windows NT\\Reliability";
pub const REGSTR_PATH_RELIABILITY_POLICY_REPORTSNAPSHOT: &str = "ReportSnapshot";
pub const REGSTR_PATH_RELIABILITY_POLICY_SHUTDOWNREASONUI: &str = "ShutdownReasonUI";
pub const REGSTR_PATH_RELIABILITY_POLICY_SNAPSHOT: &str = "Snapshot";
pub const REGSTR_PATH_ROOT: &str = "Enum\\Root";
pub const REGSTR_PATH_RUN: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
pub const REGSTR_PATH_RUNONCE: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce";
pub const REGSTR_PATH_RUNONCEEX: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\RunOnceEx";
pub const REGSTR_PATH_RUNSERVICES: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\RunServices";
pub const REGSTR_PATH_RUNSERVICESONCE: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\RunServicesOnce";
pub const REGSTR_PATH_SCHEMES: &str = "AppEvents\\Schemes";
pub const REGSTR_PATH_SCREENSAVE: &str = "Control Panel\\Desktop";
pub const REGSTR_PATH_SERVICES: &str = "System\\CurrentControlSet\\Services";
pub const REGSTR_PATH_SETUP: &str = "Software\\Microsoft\\Windows\\CurrentVersion";
pub const REGSTR_PATH_SHUTDOWN: &str = "System\\CurrentControlSet\\Control\\Shutdown";
pub const REGSTR_PATH_SOUND: &str = "Control Panel\\Sound";
pub const REGSTR_PATH_SYSTEMENUM: &str = "System\\CurrentControlSet\\Enum";
pub const REGSTR_PATH_SYSTRAY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\SysTray";
pub const REGSTR_PATH_TIMEZONE: &str = "System\\CurrentControlSet\\Control\\TimeZoneInformation";
pub const REGSTR_PATH_UNINSTALL: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
pub const REGSTR_PATH_UPDATE: &str = "System\\CurrentControlSet\\Control\\Update";
pub const REGSTR_PATH_VCOMM: &str = "System\\CurrentControlSet\\Services\\VxD\\VCOMM";
pub const REGSTR_PATH_VMM: &str = "System\\CurrentControlSet\\Services\\VxD\\VMM";
pub const REGSTR_PATH_VMM32FILES: &str = "System\\CurrentControlSet\\Control\\VMM32Files";
pub const REGSTR_PATH_VNETSUP: &str = "System\\CurrentControlSet\\Services\\VxD\\VNETSUP";
pub const REGSTR_PATH_VOLUMECACHE: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VolumeCaches";
pub const REGSTR_PATH_VPOWERD: &str = "System\\CurrentControlSet\\Services\\VxD\\VPOWERD";
pub const REGSTR_PATH_VXD: &str = "System\\CurrentControlSet\\Services\\VxD";
pub const REGSTR_PATH_WARNVERDLLS: &str = "System\\CurrentControlSet\\Control\\SessionManager\\WarnVerDLLs";
pub const REGSTR_PATH_WINBOOT: &str = "System\\CurrentControlSet\\Control\\WinBoot";
pub const REGSTR_PATH_WINDOWSAPPLETS: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Applets";
pub const REGSTR_PATH_WINLOGON: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Winlogon";
pub const REGSTR_PATH_WMI_SECURITY: &str = "System\\CurrentControlSet\\Control\\Wmi\\Security";
pub const REGSTR_PCI_DUAL_IDE: &str = "PCIDualIDE";
pub const REGSTR_PCI_OPTIONS: &str = "Options";
pub const REGSTR_VALUE_DEFAULTLOC: &str = "UseDefaultNetLocation";
pub const REGSTR_VALUE_ENABLE: &str = "Enable";
pub const REGSTR_VALUE_LOWPOWERACTIVE: &str = "ScreenSaveLowPowerActive";
pub const REGSTR_VALUE_LOWPOWERTIMEOUT: &str = "ScreenSaveLowPowerTimeout";
pub const REGSTR_VALUE_NETPATH: &str = "NetworkPath";
pub const REGSTR_VALUE_POWEROFFACTIVE: &str = "ScreenSavePowerOffActive";
pub const REGSTR_VALUE_POWEROFFTIMEOUT: &str = "ScreenSavePowerOffTimeout";
pub const REGSTR_VALUE_SCRPASSWORD: &str = "ScreenSave_Data";
pub const REGSTR_VALUE_USESCRPASSWORD: &str = "ScreenSaveUsePassword";
pub const REGSTR_VALUE_VERBOSE: &str = "Verbose";
pub const REGSTR_VAL_ACDRIVESPINDOWN: &str = "ACDriveSpinDown";
pub const REGSTR_VAL_ACSPINDOWNPREVIOUS: &str = "ACSpinDownPrevious";
pub const REGSTR_VAL_ACTIVESERVICE: &str = "ActiveService";
pub const REGSTR_VAL_ADDRESS: &str = "Address";
pub const REGSTR_VAL_AEDEBUG_AUTO: &str = "Auto";
pub const REGSTR_VAL_AEDEBUG_DEBUGGER: &str = "Debugger";
pub const REGSTR_VAL_ALPHANUMPWDS: &str = "AlphanumPwds";
pub const REGSTR_VAL_APISUPPORT: &str = "APISupport";
pub const REGSTR_VAL_APMACTIMEOUT: &str = "APMACTimeout";
pub const REGSTR_VAL_APMBATTIMEOUT: &str = "APMBatTimeout";
pub const REGSTR_VAL_APMBIOSVER: &str = "APMBiosVer";
pub const REGSTR_VAL_APMFLAGS: &str = "APMFlags";
pub const REGSTR_VAL_APMMENUSUSPEND: &str = "APMMenuSuspend";
pub const REGSTR_VAL_APMSHUTDOWNPOWER: &str = "APMShutDownPower";
pub const REGSTR_VAL_APPINSTPATH: &str = "AppInstallPath";
pub const REGSTR_VAL_ASKFORCONFIG: &str = "AskForConfig";
pub const REGSTR_VAL_ASKFORCONFIGFUNC: &str = "AskForConfigFunc";
pub const REGSTR_VAL_ASYNCFILECOMMIT: &str = "AsyncFileCommit";
pub const REGSTR_VAL_AUDIO_BITMAP: &str = "bitmap";
pub const REGSTR_VAL_AUDIO_ICON: &str = "icon";
pub const REGSTR_VAL_AUTHENT_AGENT: &str = "AuthenticatingAgent";
pub const REGSTR_VAL_AUTOEXEC: &str = "Autoexec.Bat";
pub const REGSTR_VAL_AUTOINSNOTE: &str = "AutoInsertNotification";
pub const REGSTR_VAL_AUTOLOGON: &str = "AutoLogon";
pub const REGSTR_VAL_AUTOMOUNT: &str = "AutoMountDrives";
pub const REGSTR_VAL_AUTOSTART: &str = "AutoStart";
pub const REGSTR_VAL_BASICPROPERTIES: &str = "BasicProperties";
pub const REGSTR_VAL_BASICPROPERTIES_32: &str = "BasicProperties32";
pub const REGSTR_VAL_BATDRIVESPINDOWN: &str = "BatDriveSpinDown";
pub const REGSTR_VAL_BATSPINDOWNPREVIOUS: &str = "BatSpinDownPrevious";
pub const REGSTR_VAL_BEHAVIOR_ON_FAILED_VERIFY: &str = "BehaviorOnFailedVerify";
pub const REGSTR_VAL_BIOSDATE: &str = "BIOSDate";
pub const REGSTR_VAL_BIOSNAME: &str = "BIOSName";
pub const REGSTR_VAL_BIOSVERSION: &str = "BIOSVersion";
pub const REGSTR_VAL_BITSPERPIXEL: &str = "BitsPerPixel";
pub const REGSTR_VAL_BOOTCONFIG: &str = "BootConfig";
pub const REGSTR_VAL_BOOTCOUNT: &str = "BootCount";
pub const REGSTR_VAL_BOOTDIR: &str = "BootDir";
pub const REGSTR_VAL_BPP: &str = "BPP";
pub const REGSTR_VAL_BT: &str = "6005BT";
pub const REGSTR_VAL_BUFFAGETIMEOUT: &str = "BufferAgeTimeout";
pub const REGSTR_VAL_BUFFIDLETIMEOUT: &str = "BufferIdleTimeout";
pub const REGSTR_VAL_BUSTYPE: &str = "BusType";
pub const REGSTR_VAL_CAPABILITIES: &str = "Capabilities";
pub const REGSTR_VAL_CARDSPECIFIC: &str = "CardSpecific";
pub const REGSTR_VAL_CDCACHESIZE: &str = "CacheSize";
pub const REGSTR_VAL_CDCOMPATNAMES: &str = "MSCDEXCompatNames";
pub const REGSTR_VAL_CDEXTERRORS: &str = "ExtendedErrors";
pub const REGSTR_VAL_CDNOREADAHEAD: &str = "NoReadAhead";
pub const REGSTR_VAL_CDPREFETCH: &str = "Prefetch";
pub const REGSTR_VAL_CDPREFETCHTAIL: &str = "PrefetchTail";
pub const REGSTR_VAL_CDRAWCACHE: &str = "RawCache";
pub const REGSTR_VAL_CDROM: &str = "GenCD";
pub const REGSTR_VAL_CDROMCLASSNAME: &str = "CDROM";
pub const REGSTR_VAL_CDSHOWVERSIONS: &str = "ShowVersions";
pub const REGSTR_VAL_CDSVDSENSE: &str = "SVDSense";
pub const REGSTR_VAL_CHECKSUM: &str = "CurrentChecksum";
pub const REGSTR_VAL_CLASS: &str = "Class";
pub const REGSTR_VAL_CLASSDESC: &str = "ClassDesc";
pub const REGSTR_VAL_CLASSGUID: &str = "ClassGUID";
pub const REGSTR_VAL_CMDRIVFLAGS: &str = "CMDrivFlags";
pub const REGSTR_VAL_CMENUMFLAGS: &str = "CMEnumFlags";
pub const REGSTR_VAL_COINSTALLERS_32: &str = "CoInstallers32";
pub const REGSTR_VAL_COMINFO: &str = "ComInfo";
pub const REGSTR_VAL_COMMENT: &str = "Comment";
pub const REGSTR_VAL_COMPATIBLEIDS: &str = "CompatibleIDs";
pub const REGSTR_VAL_COMPRESSIONMETHOD: &str = "CompressionAlgorithm";
pub const REGSTR_VAL_COMPRESSIONTHRESHOLD: &str = "CompressionThreshold";
pub const REGSTR_VAL_COMPUTERNAME: &str = "ComputerName";
pub const REGSTR_VAL_COMPUTRNAME: &str = "ComputerName";
pub const REGSTR_VAL_COMVERIFYBASE: &str = "COMVerifyBase";
pub const REGSTR_VAL_CONFIG: &str = "ConfigPath";
pub const REGSTR_VAL_CONFIGFLAGS: &str = "ConfigFlags";
pub const REGSTR_VAL_CONFIGMG: &str = "CONFIGMG";
pub const REGSTR_VAL_CONFIGSYS: &str = "Config.Sys";
pub const REGSTR_VAL_CONNECTION_TYPE: &str = "ConnectionType";
pub const REGSTR_VAL_CONTAINERID: &str = "ContainerID";
pub const REGSTR_VAL_CONTIGFILEALLOC: &str = "ContigFileAllocSize";
pub const REGSTR_VAL_CONVMEM: &str = "ConvMem";
pub const REGSTR_VAL_CPU: &str = "CPU";
pub const REGSTR_VAL_CRASHFUNCS: &str = "CrashFuncs";
pub const REGSTR_VAL_CSCONFIGFLAGS: &str = "CSConfigFlags";
pub const REGSTR_VAL_CURCONFIG: &str = "CurrentConfig";
pub const REGSTR_VAL_CURDRVLET: &str = "CurrentDriveLetterAssignment";
pub const REGSTR_VAL_CURRENTCONFIG: &str = "CurrentConfig";
pub const REGSTR_VAL_CURRENT_BUILD: &str = "CurrentBuildNumber";
pub const REGSTR_VAL_CURRENT_CSDVERSION: &str = "CSDVersion";
pub const REGSTR_VAL_CURRENT_TYPE: &str = "CurrentType";
pub const REGSTR_VAL_CURRENT_USER: &str = "Current User";
pub const REGSTR_VAL_CURRENT_VERSION: &str = "CurrentVersion";
pub const REGSTR_VAL_CUSTOMCOLORS: &str = "CustomColors";
pub const REGSTR_VAL_CUSTOM_PROPERTY_CACHE_DATE: &str = "CustomPropertyCacheDate";
pub const REGSTR_VAL_CUSTOM_PROPERTY_HW_ID_KEY: &str = "CustomPropertyHwIdKey";
pub const REGSTR_VAL_DEFAULT: &str = "Default";
pub const REGSTR_VAL_DETCONFIG: &str = "DetConfig";
pub const REGSTR_VAL_DETECT: &str = "Detect";
pub const REGSTR_VAL_DETECTFUNC: &str = "DetectFunc";
pub const REGSTR_VAL_DETFLAGS: &str = "DetFlags";
pub const REGSTR_VAL_DETFUNC: &str = "DetFunc";
pub const REGSTR_VAL_DEVDESC: &str = "DeviceDesc";
pub const REGSTR_VAL_DEVICEDRIVER: &str = "DeviceDriver";
pub const REGSTR_VAL_DEVICEPATH: &str = "DevicePath";
pub const REGSTR_VAL_DEVICE_CHARACTERISTICS: &str = "DeviceCharacteristics";
pub const REGSTR_VAL_DEVICE_EXCLUSIVE: &str = "Exclusive";
pub const REGSTR_VAL_DEVICE_INSTANCE: &str = "DeviceInstance";
pub const REGSTR_VAL_DEVICE_SECURITY_DESCRIPTOR: &str = "Security";
pub const REGSTR_VAL_DEVICE_TYPE: &str = "DeviceType";
pub const REGSTR_VAL_DEVLOADER: &str = "DevLoader";
pub const REGSTR_VAL_DEVTYPE: &str = "DeviceType";
pub const REGSTR_VAL_DIRECTHOST: &str = "DirectHost";
pub const REGSTR_VAL_DIRTYSHUTDOWN: &str = "DirtyShutdown";
pub const REGSTR_VAL_DIRTYSHUTDOWNTIME: &str = "DirtyShutdownTime";
pub const REGSTR_VAL_DISABLECOUNT: &str = "DisableCount";
pub const REGSTR_VAL_DISABLEPWDCACHING: &str = "DisablePwdCaching";
pub const REGSTR_VAL_DISABLEREGTOOLS: &str = "DisableRegistryTools";
pub const REGSTR_VAL_DISCONNECT: &str = "Disconnect";
pub const REGSTR_VAL_DISK: &str = "GenDisk";
pub const REGSTR_VAL_DISKCLASSNAME: &str = "DiskDrive";
pub const REGSTR_VAL_DISPCPL_NOAPPEARANCEPAGE: &str = "NoDispAppearancePage";
pub const REGSTR_VAL_DISPCPL_NOBACKGROUNDPAGE: &str = "NoDispBackgroundPage";
pub const REGSTR_VAL_DISPCPL_NODISPCPL: &str = "NoDispCPL";
pub const REGSTR_VAL_DISPCPL_NOSCRSAVPAGE: &str = "NoDispScrSavPage";
pub const REGSTR_VAL_DISPCPL_NOSETTINGSPAGE: &str = "NoDispSettingsPage";
pub const REGSTR_VAL_DISPLAY: &str = "display";
pub const REGSTR_VAL_DISPLAYFLAGS: &str = "DisplayFlags";
pub const REGSTR_VAL_DOCKED: &str = "CurrentDockedState";
pub const REGSTR_VAL_DOCKSTATE: &str = "DockState";
pub const REGSTR_VAL_DOES_POLLING: &str = "PollingSupportNeeded";
pub const REGSTR_VAL_DONTLOADIFCONFLICT: &str = "DontLoadIfConflict";
pub const REGSTR_VAL_DONTUSEMEM: &str = "DontAllocLastMem";
pub const REGSTR_VAL_DOSCP: &str = "OEMCP";
pub const REGSTR_VAL_DOSOPTFLAGS: &str = "Flags";
pub const REGSTR_VAL_DOSOPTGLOBALFLAGS: &str = "GlobalFlags";
pub const REGSTR_VAL_DOSOPTTIP: &str = "TipText";
pub const REGSTR_VAL_DOSPAGER: &str = "DOSPager";
pub const REGSTR_VAL_DOS_SPOOL_MASK: &str = "DOSSpoolMask";
pub const REGSTR_VAL_DOUBLEBUFFER: &str = "DoubleBuffer";
pub const REGSTR_VAL_DPI: &str = "dpi";
pub const REGSTR_VAL_DPILOGICALX: &str = "DPILogicalX";
pub const REGSTR_VAL_DPILOGICALY: &str = "DPILogicalY";
pub const REGSTR_VAL_DPIPHYSICALX: &str = "DPIPhysicalX";
pub const REGSTR_VAL_DPIPHYSICALY: &str = "DPIPhysicalY";
pub const REGSTR_VAL_DPMS: &str = "DPMS";
pub const REGSTR_VAL_DRIVER: &str = "Driver";
pub const REGSTR_VAL_DRIVERCACHEPATH: &str = "DriverCachePath";
pub const REGSTR_VAL_DRIVERDATE: &str = "DriverDate";
pub const REGSTR_VAL_DRIVERDATEDATA: &str = "DriverDateData";
pub const REGSTR_VAL_DRIVERVERSION: &str = "DriverVersion";
pub const REGSTR_VAL_DRIVESPINDOWN: &str = "DriveSpinDown";
pub const REGSTR_VAL_DRIVEWRITEBEHIND: &str = "DriveWriteBehind";
pub const REGSTR_VAL_DRIVE_SPINDOWN: &str = "NoDispSpinDown";
pub const REGSTR_VAL_DRV: &str = "drv";
pub const REGSTR_VAL_DRVDESC: &str = "DriverDesc";
pub const REGSTR_VAL_DYNAMIC: &str = "Dynamic";
pub const REGSTR_VAL_EISA_FLAGS: &str = "EISAFlags";
pub const REGSTR_VAL_EISA_FUNCTIONS: &str = "EISAFunctions";
pub const REGSTR_VAL_EISA_FUNCTIONS_MASK: &str = "EISAFunctionsMask";
pub const REGSTR_VAL_EISA_RANGES: &str = "EISARanges";
pub const REGSTR_VAL_EISA_SIMULATE_INT15: &str = "EISASimulateInt15";
pub const REGSTR_VAL_EJECT_PRIORITY: &str = "EjectPriority";
pub const REGSTR_VAL_ENABLEINTS: &str = "EnableInts";
pub const REGSTR_VAL_ENUMERATOR: &str = "Enumerator";
pub const REGSTR_VAL_ENUMPROPPAGES: &str = "EnumPropPages";
pub const REGSTR_VAL_ENUMPROPPAGES_32: &str = "EnumPropPages32";
pub const REGSTR_VAL_ESDI: &str = "ESDI\\";
pub const REGSTR_VAL_EXISTS: &str = "Exists";
pub const REGSTR_VAL_EXTMEM: &str = "ExtMem";
pub const REGSTR_VAL_FAULT_LOGFILE: &str = "LogFile";
pub const REGSTR_VAL_FIFODEPTH: &str = "FIFODepth";
pub const REGSTR_VAL_FILESHARING: &str = "FileSharing";
pub const REGSTR_VAL_FIRSTINSTALLDATETIME: &str = "FirstInstallDateTime";
pub const REGSTR_VAL_FIRSTNETDRIVE: &str = "FirstNetworkDrive";
pub const REGSTR_VAL_FLOP: &str = "FLOP\\";
pub const REGSTR_VAL_FLOPPY: &str = "FLOPPY";
pub const REGSTR_VAL_FONTSIZE: &str = "FontSize";
pub const REGSTR_VAL_FORCECL: &str = "ForceChangeLine";
pub const REGSTR_VAL_FORCEDCONFIG: &str = "ForcedConfig";
pub const REGSTR_VAL_FORCEFIFO: &str = "ForceFIFO";
pub const REGSTR_VAL_FORCELOAD: &str = "ForceLoadPD";
pub const REGSTR_VAL_FORCEPMIO: &str = "ForcePMIO";
pub const REGSTR_VAL_FORCEREBOOT: &str = "ForceReboot";
pub const REGSTR_VAL_FORCERMIO: &str = "ForceRMIO";
pub const REGSTR_VAL_FREESPACERATIO: &str = "FreeSpaceRatio";
pub const REGSTR_VAL_FRIENDLYNAME: &str = "FriendlyName";
pub const REGSTR_VAL_FSFILTERCLASS: &str = "FSFilterClass";
pub const REGSTR_VAL_FULLTRACE: &str = "FullTrace";
pub const REGSTR_VAL_FUNCDESC: &str = "FunctionDesc";
pub const REGSTR_VAL_GAPTIME: &str = "GapTime";
pub const REGSTR_VAL_GRB: &str = "grb";
pub const REGSTR_VAL_HARDWAREID: &str = "HardwareID";
pub const REGSTR_VAL_HIDESHAREPWDS: &str = "HideSharePwds";
pub const REGSTR_VAL_HRES: &str = "HRes";
pub const REGSTR_VAL_HWDETECT: &str = "HardwareDetect";
pub const REGSTR_VAL_HWMECHANISM: &str = "HWMechanism";
pub const REGSTR_VAL_HWREV: &str = "HWRevision";
pub const REGSTR_VAL_ID: &str = "CurrentID";
pub const REGSTR_VAL_IDE_FORCE_SERIALIZE: &str = "ForceSerialization";
pub const REGSTR_VAL_IDE_NO_SERIALIZE: &str = "IDENoSerialize";
pub const REGSTR_VAL_INFNAME: &str = "InfName";
pub const REGSTR_VAL_INFPATH: &str = "InfPath";
pub const REGSTR_VAL_INFSECTION: &str = "InfSection";
pub const REGSTR_VAL_INFSECTIONEXT: &str = "InfSectionExt";
pub const REGSTR_VAL_INHIBITRESULTS: &str = "InhibitResults";
pub const REGSTR_VAL_INSICON: &str = "Icon";
pub const REGSTR_VAL_INSTALLER: &str = "Installer";
pub const REGSTR_VAL_INSTALLER_32: &str = "Installer32";
pub const REGSTR_VAL_INSTALLTYPE: &str = "InstallType";
pub const REGSTR_VAL_INT13: &str = "Int13";
pub const REGSTR_VAL_ISAPNP: &str = "ISAPNP";
pub const REGSTR_VAL_ISAPNP_RDP_OVERRIDE: &str = "RDPOverRide";
pub const REGSTR_VAL_JOYCALLOUT: &str = "JoystickCallout";
pub const REGSTR_VAL_JOYNCONFIG: &str = "Joystick%dConfiguration";
pub const REGSTR_VAL_JOYNOEMCALLOUT: &str = "Joystick%dOEMCallout";
pub const REGSTR_VAL_JOYNOEMNAME: &str = "Joystick%dOEMName";
pub const REGSTR_VAL_JOYOEMCAL1: &str = "OEMCal1";
pub const REGSTR_VAL_JOYOEMCAL10: &str = "OEMCal10";
pub const REGSTR_VAL_JOYOEMCAL11: &str = "OEMCal11";
pub const REGSTR_VAL_JOYOEMCAL12: &str = "OEMCal12";
pub const REGSTR_VAL_JOYOEMCAL2: &str = "OEMCal2";
pub const REGSTR_VAL_JOYOEMCAL3: &str = "OEMCal3";
pub const REGSTR_VAL_JOYOEMCAL4: &str = "OEMCal4";
pub const REGSTR_VAL_JOYOEMCAL5: &str = "OEMCal5";
pub const REGSTR_VAL_JOYOEMCAL6: &str = "OEMCal6";
pub const REGSTR_VAL_JOYOEMCAL7: &str = "OEMCal7";
pub const REGSTR_VAL_JOYOEMCAL8: &str = "OEMCal8";
pub const REGSTR_VAL_JOYOEMCAL9: &str = "OEMCal9";
pub const REGSTR_VAL_JOYOEMCALCAP: &str = "OEMCalCap";
pub const REGSTR_VAL_JOYOEMCALLOUT: &str = "OEMCallout";
pub const REGSTR_VAL_JOYOEMCALWINCAP: &str = "OEMCalWinCap";
pub const REGSTR_VAL_JOYOEMDATA: &str = "OEMData";
pub const REGSTR_VAL_JOYOEMNAME: &str = "OEMName";
pub const REGSTR_VAL_JOYOEMPOVLABEL: &str = "OEMPOVLabel";
pub const REGSTR_VAL_JOYOEMRLABEL: &str = "OEMRLabel";
pub const REGSTR_VAL_JOYOEMTESTBUTTONCAP: &str = "OEMTestButtonCap";
pub const REGSTR_VAL_JOYOEMTESTBUTTONDESC: &str = "OEMTestButtonDesc";
pub const REGSTR_VAL_JOYOEMTESTMOVECAP: &str = "OEMTestMoveCap";
pub const REGSTR_VAL_JOYOEMTESTMOVEDESC: &str = "OEMTestMoveDesc";
pub const REGSTR_VAL_JOYOEMTESTWINCAP: &str = "OEMTestWinCap";
pub const REGSTR_VAL_JOYOEMULABEL: &str = "OEMULabel";
pub const REGSTR_VAL_JOYOEMVLABEL: &str = "OEMVLabel";
pub const REGSTR_VAL_JOYOEMXYLABEL: &str = "OEMXYLabel";
pub const REGSTR_VAL_JOYOEMZLABEL: &str = "OEMZLabel";
pub const REGSTR_VAL_JOYUSERVALUES: &str = "JoystickUserValues";
pub const REGSTR_VAL_LASTALIVEBT: &str = "LastAliveBT";
pub const REGSTR_VAL_LASTALIVEINTERVAL: &str = "TimeStampInterval";
pub const REGSTR_VAL_LASTALIVEPMPOLICY: &str = "LastAlivePMPolicy";
pub const REGSTR_VAL_LASTALIVESTAMP: &str = "LastAliveStamp";
pub const REGSTR_VAL_LASTALIVESTAMPFORCED: &str = "LastAliveStampForced";
pub const REGSTR_VAL_LASTALIVESTAMPINTERVAL: &str = "LastAliveStampInterval";
pub const REGSTR_VAL_LASTALIVESTAMPPOLICYINTERVAL: &str = "LastAliveStampPolicyInterval";
pub const REGSTR_VAL_LASTALIVEUPTIME: &str = "LastAliveUptime";
pub const REGSTR_VAL_LASTBOOTPMDRVS: &str = "LastBootPMDrvs";
pub const REGSTR_VAL_LASTCOMPUTERNAME: &str = "LastComputerName";
pub const REGSTR_VAL_LASTPCIBUSNUM: &str = "LastPCIBusNum";
pub const REGSTR_VAL_LAST_UPDATE_TIME: &str = "LastUpdateTime";
pub const REGSTR_VAL_LEGALNOTICECAPTION: &str = "LegalNoticeCaption";
pub const REGSTR_VAL_LEGALNOTICETEXT: &str = "LegalNoticeText";
pub const REGSTR_VAL_LICENSINGINFO: &str = "LicensingInfo";
pub const REGSTR_VAL_LINKED: &str = "Linked";
pub const REGSTR_VAL_LOADHI: &str = "LoadHi";
pub const REGSTR_VAL_LOADRMDRIVERS: &str = "LoadRMDrivers";
pub const REGSTR_VAL_LOCATION_INFORMATION: &str = "LocationInformation";
pub const REGSTR_VAL_LOCATION_INFORMATION_OVERRIDE: &str = "LocationInformationOverride";
pub const REGSTR_VAL_LOWERFILTERS: &str = "LowerFilters";
pub const REGSTR_VAL_LOWER_FILTER_DEFAULT_LEVEL: &str = "LowerFilterDefaultLevel";
pub const REGSTR_VAL_LOWER_FILTER_LEVELS: &str = "LowerFilterLevels";
pub const REGSTR_VAL_MACHINETYPE: &str = "MachineType";
pub const REGSTR_VAL_MANUFACTURER: &str = "Manufacturer";
pub const REGSTR_VAL_MAP: &str = "Map";
pub const REGSTR_VAL_MATCHINGDEVID: &str = "MatchingDeviceId";
pub const REGSTR_VAL_MAXCONNECTIONS: &str = "MaxConnections";
pub const REGSTR_VAL_MAXLIP: &str = "MaxLIP";
pub const REGSTR_VAL_MAXRES: &str = "MaxResolution";
pub const REGSTR_VAL_MAXRETRY: &str = "MaxRetry";
pub const REGSTR_VAL_MAX_HCID_LEN: u32 = 1024u32;
pub const REGSTR_VAL_MEDIA: &str = "MediaPath";
pub const REGSTR_VAL_MFG: &str = "Mfg";
pub const REGSTR_VAL_MF_FLAGS: &str = "MFFlags";
pub const REGSTR_VAL_MINIPORT_STAT: &str = "MiniportStatus";
pub const REGSTR_VAL_MINPWDLEN: &str = "MinPwdLen";
pub const REGSTR_VAL_MINRETRY: &str = "MinRetry";
pub const REGSTR_VAL_MODE: &str = "Mode";
pub const REGSTR_VAL_MODEL: &str = "Model";
pub const REGSTR_VAL_MSDOSMODE: &str = "MSDOSMode";
pub const REGSTR_VAL_MSDOSMODEDISCARD: &str = "Discard";
pub const REGSTR_VAL_MUSTBEVALIDATED: &str = "MustBeValidated";
pub const REGSTR_VAL_NAMECACHECOUNT: &str = "NameCache";
pub const REGSTR_VAL_NAMENUMERICTAIL: &str = "NameNumericTail";
pub const REGSTR_VAL_NCP_BROWSEMASTER: &str = "BrowseMaster";
pub const REGSTR_VAL_NCP_USEPEERBROWSING: &str = "Use_PeerBrowsing";
pub const REGSTR_VAL_NCP_USESAP: &str = "Use_Sap";
pub const REGSTR_VAL_NDP: &str = "NDP";
pub const REGSTR_VAL_NETCARD: &str = "Netcard";
pub const REGSTR_VAL_NETCLEAN: &str = "NetClean";
pub const REGSTR_VAL_NETOSTYPE: &str = "NetOSType";
pub const REGSTR_VAL_NETSETUP_DISABLE: &str = "NoNetSetup";
pub const REGSTR_VAL_NETSETUP_NOCONFIGPAGE: &str = "NoNetSetupConfigPage";
pub const REGSTR_VAL_NETSETUP_NOIDPAGE: &str = "NoNetSetupIDPage";
pub const REGSTR_VAL_NETSETUP_NOSECURITYPAGE: &str = "NoNetSetupSecurityPage";
pub const REGSTR_VAL_NOCMOSORFDPT: &str = "NoCMOSorFDPT";
pub const REGSTR_VAL_NODISPLAYCLASS: &str = "NoDisplayClass";
pub const REGSTR_VAL_NOENTIRENETWORK: &str = "NoEntireNetwork";
pub const REGSTR_VAL_NOFILESHARING: &str = "NoFileSharing";
pub const REGSTR_VAL_NOFILESHARINGCTRL: &str = "NoFileSharingControl";
pub const REGSTR_VAL_NOIDE: &str = "NoIDE";
pub const REGSTR_VAL_NOINSTALLCLASS: &str = "NoInstallClass";
pub const REGSTR_VAL_NONSTANDARD_ATAPI: &str = "NonStandardATAPI";
pub const REGSTR_VAL_NOPRINTSHARING: &str = "NoPrintSharing";
pub const REGSTR_VAL_NOPRINTSHARINGCTRL: &str = "NoPrintSharingControl";
pub const REGSTR_VAL_NOUSECLASS: &str = "NoUseClass";
pub const REGSTR_VAL_NOWORKGROUPCONTENTS: &str = "NoWorkgroupContents";
pub const REGSTR_VAL_OLDMSDOSVER: &str = "OldMSDOSVer";
pub const REGSTR_VAL_OLDWINDIR: &str = "OldWinDir";
pub const REGSTR_VAL_OPTIMIZESFN: &str = "OptimizeSFN";
pub const REGSTR_VAL_OPTIONS: &str = "Options";
pub const REGSTR_VAL_OPTORDER: &str = "Order";
pub const REGSTR_VAL_P1284MDL: &str = "Model";
pub const REGSTR_VAL_P1284MFG: &str = "Manufacturer";
pub const REGSTR_VAL_PATHCACHECOUNT: &str = "PathCache";
pub const REGSTR_VAL_PCCARD_POWER: &str = "EnablePowerManagement";
pub const REGSTR_VAL_PCI: &str = "PCI";
pub const REGSTR_VAL_PCIBIOSVER: &str = "PCIBIOSVer";
pub const REGSTR_VAL_PCICIRQMAP: &str = "PCICIRQMap";
pub const REGSTR_VAL_PCICOPTIONS: &str = "PCICOptions";
pub const REGSTR_VAL_PCMCIA_ALLOC: &str = "AllocMemWin";
pub const REGSTR_VAL_PCMCIA_ATAD: &str = "ATADelay";
pub const REGSTR_VAL_PCMCIA_MEM: &str = "Memory";
pub const REGSTR_VAL_PCMCIA_OPT: &str = "Options";
pub const REGSTR_VAL_PCMCIA_SIZ: &str = "MinRegionSize";
pub const REGSTR_VAL_PCMTDRIVER: &str = "MTD";
pub const REGSTR_VAL_PCSSDRIVER: &str = "Driver";
pub const REGSTR_VAL_PHYSICALDEVICEOBJECT: &str = "PhysicalDeviceObject";
pub const REGSTR_VAL_PMODE_INT13: &str = "PModeInt13";
pub const REGSTR_VAL_PNPBIOSVER: &str = "PnPBIOSVer";
pub const REGSTR_VAL_PNPSTRUCOFFSET: &str = "PnPStrucOffset";
pub const REGSTR_VAL_POLICY: &str = "Policy";
pub const REGSTR_VAL_POLLING: &str = "Polling";
pub const REGSTR_VAL_PORTNAME: &str = "PortName";
pub const REGSTR_VAL_PORTSUBCLASS: &str = "PortSubClass";
pub const REGSTR_VAL_PREFREDIR: &str = "PreferredRedir";
pub const REGSTR_VAL_PRESERVECASE: &str = "PreserveCase";
pub const REGSTR_VAL_PRESERVELONGNAMES: &str = "PreserveLongNames";
pub const REGSTR_VAL_PRINTERS_HIDETABS: &str = "NoPrinterTabs";
pub const REGSTR_VAL_PRINTERS_MASK: &str = "PrintersMask";
pub const REGSTR_VAL_PRINTERS_NOADD: &str = "NoAddPrinter";
pub const REGSTR_VAL_PRINTERS_NODELETE: &str = "NoDeletePrinter";
pub const REGSTR_VAL_PRINTSHARING: &str = "PrintSharing";
pub const REGSTR_VAL_PRIORITY: &str = "Priority";
pub const REGSTR_VAL_PRIVATE: &str = "Private";
pub const REGSTR_VAL_PRIVATEFUNC: &str = "PrivateFunc";
pub const REGSTR_VAL_PRIVATEPROBLEM: &str = "PrivateProblem";
pub const REGSTR_VAL_PRODUCTID: &str = "ProductId";
pub const REGSTR_VAL_PRODUCTTYPE: &str = "ProductType";
pub const REGSTR_VAL_PROFILEFLAGS: &str = "ProfileFlags";
pub const REGSTR_VAL_PROPERTIES: &str = "Properties";
pub const REGSTR_VAL_PROTINIPATH: &str = "ProtIniPath";
pub const REGSTR_VAL_PROVIDER_NAME: &str = "ProviderName";
pub const REGSTR_VAL_PWDEXPIRATION: &str = "PwdExpiration";
pub const REGSTR_VAL_PWDPROVIDER_CHANGEORDER: &str = "ChangeOrder";
pub const REGSTR_VAL_PWDPROVIDER_CHANGEPWD: &str = "ChangePassword";
pub const REGSTR_VAL_PWDPROVIDER_CHANGEPWDHWND: &str = "ChangePasswordHwnd";
pub const REGSTR_VAL_PWDPROVIDER_DESC: &str = "Description";
pub const REGSTR_VAL_PWDPROVIDER_GETPWDSTATUS: &str = "GetPasswordStatus";
pub const REGSTR_VAL_PWDPROVIDER_ISNP: &str = "NetworkProvider";
pub const REGSTR_VAL_PWDPROVIDER_PATH: &str = "ProviderPath";
pub const REGSTR_VAL_RDINTTHRESHOLD: &str = "RDIntThreshold";
pub const REGSTR_VAL_READAHEADTHRESHOLD: &str = "ReadAheadThreshold";
pub const REGSTR_VAL_READCACHING: &str = "ReadCaching";
pub const REGSTR_VAL_REALNETSTART: &str = "RealNetStart";
pub const REGSTR_VAL_REASONCODE: &str = "ReasonCode";
pub const REGSTR_VAL_REFRESHRATE: &str = "RefreshRate";
pub const REGSTR_VAL_REGITEMDELETEMESSAGE: &str = "Removal Message";
pub const REGSTR_VAL_REGORGANIZATION: &str = "RegisteredOrganization";
pub const REGSTR_VAL_REGOWNER: &str = "RegisteredOwner";
pub const REGSTR_VAL_REINSTALL_DEVICEINSTANCEIDS: &str = "DeviceInstanceIds";
pub const REGSTR_VAL_REINSTALL_DISPLAYNAME: &str = "DisplayName";
pub const REGSTR_VAL_REINSTALL_STRING: &str = "ReinstallString";
pub const REGSTR_VAL_REMOTE_PATH: &str = "RemotePath";
pub const REGSTR_VAL_REMOVABLE: &str = "Removable";
pub const REGSTR_VAL_REMOVAL_POLICY: &str = "RemovalPolicy";
pub const REGSTR_VAL_REMOVEROMOKAY: &str = "RemoveRomOkay";
pub const REGSTR_VAL_REMOVEROMOKAYFUNC: &str = "RemoveRomOkayFunc";
pub const REGSTR_VAL_RESERVED_DEVNODE: &str = "HTREE\\RESERVED\\0";
pub const REGSTR_VAL_RESOLUTION: &str = "Resolution";
pub const REGSTR_VAL_RESOURCES: &str = "Resources";
pub const REGSTR_VAL_RESOURCE_MAP: &str = "ResourceMap";
pub const REGSTR_VAL_RESOURCE_PICKER_EXCEPTIONS: &str = "ResourcePickerExceptions";
pub const REGSTR_VAL_RESOURCE_PICKER_TAGS: &str = "ResourcePickerTags";
pub const REGSTR_VAL_RESTRICTRUN: &str = "RestrictRun";
pub const REGSTR_VAL_RESUMERESET: &str = "ResumeReset";
pub const REGSTR_VAL_REVISION: &str = "Revision";
pub const REGSTR_VAL_REVLEVEL: &str = "RevisionLevel";
pub const REGSTR_VAL_ROOT_DEVNODE: &str = "HTREE\\ROOT\\0";
pub const REGSTR_VAL_RUNLOGINSCRIPT: &str = "ProcessLoginScript";
pub const REGSTR_VAL_SCANNER: &str = "SCANNER";
pub const REGSTR_VAL_SCAN_ONLY_FIRST: &str = "ScanOnlyFirstDrive";
pub const REGSTR_VAL_SCSI: &str = "SCSI\\";
pub const REGSTR_VAL_SCSILUN: &str = "SCSILUN";
pub const REGSTR_VAL_SCSITID: &str = "SCSITargetID";
pub const REGSTR_VAL_SEARCHMODE: &str = "SearchMode";
pub const REGSTR_VAL_SEARCHOPTIONS: &str = "SearchOptions";
pub const REGSTR_VAL_SECCPL_NOADMINPAGE: &str = "NoAdminPage";
pub const REGSTR_VAL_SECCPL_NOPROFILEPAGE: &str = "NoProfilePage";
pub const REGSTR_VAL_SECCPL_NOPWDPAGE: &str = "NoPwdPage";
pub const REGSTR_VAL_SECCPL_NOSECCPL: &str = "NoSecCPL";
pub const REGSTR_VAL_SERVICE: &str = "Service";
pub const REGSTR_VAL_SETUPFLAGS: &str = "SetupFlags";
pub const REGSTR_VAL_SETUPMACHINETYPE: &str = "SetupMachineType";
pub const REGSTR_VAL_SETUPN: &str = "SetupN";
pub const REGSTR_VAL_SETUPNPATH: &str = "SetupNPath";
pub const REGSTR_VAL_SETUPPROGRAMRAN: &str = "SetupProgramRan";
pub const REGSTR_VAL_SHARES_FLAGS: &str = "Flags";
pub const REGSTR_VAL_SHARES_PATH: &str = "Path";
pub const REGSTR_VAL_SHARES_REMARK: &str = "Remark";
pub const REGSTR_VAL_SHARES_RO_PASS: &str = "Parm2";
pub const REGSTR_VAL_SHARES_RW_PASS: &str = "Parm1";
pub const REGSTR_VAL_SHARES_TYPE: &str = "Type";
pub const REGSTR_VAL_SHARE_IRQ: &str = "ForceIRQSharing";
pub const REGSTR_VAL_SHELLVERSION: &str = "ShellVersion";
pub const REGSTR_VAL_SHOWDOTS: &str = "ShowDots";
pub const REGSTR_VAL_SHOWREASONUI: &str = "ShutdownReasonUI";
pub const REGSTR_VAL_SHUTDOWNREASON: &str = "ShutdownReason";
pub const REGSTR_VAL_SHUTDOWNREASON_CODE: &str = "ShutdownReasonCode";
pub const REGSTR_VAL_SHUTDOWNREASON_COMMENT: &str = "ShutdownReasonComment";
pub const REGSTR_VAL_SHUTDOWNREASON_PROCESS: &str = "ShutdownReasonProcess";
pub const REGSTR_VAL_SHUTDOWNREASON_USERNAME: &str = "ShutdownReasonUserName";
pub const REGSTR_VAL_SHUTDOWN_FLAGS: &str = "ShutdownFlags";
pub const REGSTR_VAL_SHUTDOWN_IGNORE_PREDEFINED: &str = "ShutdownIgnorePredefinedReasons";
pub const REGSTR_VAL_SHUTDOWN_STATE_SNAPSHOT: &str = "ShutdownStateSnapshot";
pub const REGSTR_VAL_SILENTINSTALL: &str = "SilentInstall";
pub const REGSTR_VAL_SLSUPPORT: &str = "SLSupport";
pub const REGSTR_VAL_SOFTCOMPATMODE: &str = "SoftCompatMode";
pub const REGSTR_VAL_SRCPATH: &str = "SourcePath";
pub const REGSTR_VAL_SRVNAMECACHE: &str = "ServerNameCache";
pub const REGSTR_VAL_SRVNAMECACHECOUNT: &str = "ServerNameCacheMax";
pub const REGSTR_VAL_SRVNAMECACHENETPROV: &str = "ServerNameCacheNumNets";
pub const REGSTR_VAL_START_ON_BOOT: &str = "StartOnBoot";
pub const REGSTR_VAL_STAT: &str = "Status";
pub const REGSTR_VAL_STATICDRIVE: &str = "StaticDrive";
pub const REGSTR_VAL_STATICVXD: &str = "StaticVxD";
pub const REGSTR_VAL_STDDOSOPTION: &str = "StdOption";
pub const REGSTR_VAL_SUBMODEL: &str = "Submodel";
pub const REGSTR_VAL_SUPPORTBURST: &str = "SupportBurst";
pub const REGSTR_VAL_SUPPORTLFN: &str = "SupportLFN";
pub const REGSTR_VAL_SUPPORTTUNNELLING: &str = "SupportTunnelling";
pub const REGSTR_VAL_SYMBOLIC_LINK: &str = "SymbolicLink";
pub const REGSTR_VAL_SYNCDATAXFER: &str = "SyncDataXfer";
pub const REGSTR_VAL_SYSDM: &str = "SysDM";
pub const REGSTR_VAL_SYSDMFUNC: &str = "SysDMFunc";
pub const REGSTR_VAL_SYSTEMCPL_NOCONFIGPAGE: &str = "NoConfigPage";
pub const REGSTR_VAL_SYSTEMCPL_NODEVMGRPAGE: &str = "NoDevMgrPage";
pub const REGSTR_VAL_SYSTEMCPL_NOFILESYSPAGE: &str = "NoFileSysPage";
pub const REGSTR_VAL_SYSTEMCPL_NOVIRTMEMPAGE: &str = "NoVirtMemPage";
pub const REGSTR_VAL_SYSTEMROOT: &str = "SystemRoot";
pub const REGSTR_VAL_SYSTRAYBATFLAGS: &str = "PowerFlags";
pub const REGSTR_VAL_SYSTRAYPCCARDFLAGS: &str = "PCMCIAFlags";
pub const REGSTR_VAL_SYSTRAYSVCS: &str = "Services";
pub const REGSTR_VAL_TABLE_STAT: &str = "TableStatus";
pub const REGSTR_VAL_TAPE: &str = "TAPE";
pub const REGSTR_VAL_TRANSITION: &str = "Transition";
pub const REGSTR_VAL_TRANSPORT: &str = "Transport";
pub const REGSTR_VAL_TZACTBIAS: &str = "ActiveTimeBias";
pub const REGSTR_VAL_TZBIAS: &str = "Bias";
pub const REGSTR_VAL_TZDLTBIAS: &str = "DaylightBias";
pub const REGSTR_VAL_TZDLTFLAG: &str = "DaylightFlag";
pub const REGSTR_VAL_TZDLTNAME: &str = "DaylightName";
pub const REGSTR_VAL_TZDLTSTART: &str = "DaylightStart";
pub const REGSTR_VAL_TZNOAUTOTIME: &str = "DisableAutoDaylightTimeSet";
pub const REGSTR_VAL_TZNOCHANGEEND: &str = "NoChangeEnd";
pub const REGSTR_VAL_TZNOCHANGESTART: &str = "NoChangeStart";
pub const REGSTR_VAL_TZSTDBIAS: &str = "StandardBias";
pub const REGSTR_VAL_TZSTDNAME: &str = "StandardName";
pub const REGSTR_VAL_TZSTDSTART: &str = "StandardStart";
pub const REGSTR_VAL_UI_NUMBER: &str = "UINumber";
pub const REGSTR_VAL_UI_NUMBER_DESC_FORMAT: &str = "UINumberDescFormat";
pub const REGSTR_VAL_UNDOCK_WITHOUT_LOGON: &str = "UndockWithoutLogon";
pub const REGSTR_VAL_UNINSTALLER_COMMANDLINE: &str = "UninstallString";
pub const REGSTR_VAL_UNINSTALLER_DISPLAYNAME: &str = "DisplayName";
pub const REGSTR_VAL_UPGRADE: &str = "Upgrade";
pub const REGSTR_VAL_UPPERFILTERS: &str = "UpperFilters";
pub const REGSTR_VAL_UPPER_FILTER_DEFAULT_LEVEL: &str = "UpperFilterDefaultLevel";
pub const REGSTR_VAL_UPPER_FILTER_LEVELS: &str = "UpperFilterLevels";
pub const REGSTR_VAL_USERSETTINGS: &str = "AdapterSettings";
pub const REGSTR_VAL_USER_NAME: &str = "UserName";
pub const REGSTR_VAL_USRDRVLET: &str = "UserDriveLetterAssignment";
pub const REGSTR_VAL_VDD: &str = "vdd";
pub const REGSTR_VAL_VER: &str = "Ver";
pub const REGSTR_VAL_VERIFYKEY: &str = "VerifyKey";
pub const REGSTR_VAL_VIRTUALHDIRQ: &str = "VirtualHDIRQ";
pub const REGSTR_VAL_VOLIDLETIMEOUT: &str = "VolumeIdleTimeout";
pub const REGSTR_VAL_VPOWERDFLAGS: &str = "Flags";
pub const REGSTR_VAL_VRES: &str = "VRes";
pub const REGSTR_VAL_VXDGROUPS: &str = "VXDGroups";
pub const REGSTR_VAL_WAITFORUNDOCK: &str = "WaitForUndock";
pub const REGSTR_VAL_WAITFORUNDOCKFUNC: &str = "WaitForUndockFunc";
pub const REGSTR_VAL_WIN31FILESYSTEM: &str = "Win31FileSystem";
pub const REGSTR_VAL_WIN31PROVIDER: &str = "Win31Provider";
pub const REGSTR_VAL_WINBOOTDIR: &str = "WinbootDir";
pub const REGSTR_VAL_WINCP: &str = "ACP";
pub const REGSTR_VAL_WINDIR: &str = "WinDir";
pub const REGSTR_VAL_WINOLDAPP_DISABLED: &str = "Disabled";
pub const REGSTR_VAL_WINOLDAPP_NOREALMODE: &str = "NoRealMode";
pub const REGSTR_VAL_WORKGROUP: &str = "Workgroup";
pub const REGSTR_VAL_WRAPPER: &str = "Wrapper";
pub const REGSTR_VAL_WRINTTHRESHOLD: &str = "WRIntThreshold";
pub const REGSTR_VAL_WRKGRP_FORCEMAPPING: &str = "WrkgrpForceMapping";
pub const REGSTR_VAL_WRKGRP_REQUIRED: &str = "WrkgrpRequired";
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_CREATE_KEY_DISPOSITION(pub u32);
pub const REG_CREATED_NEW_KEY: REG_CREATE_KEY_DISPOSITION = REG_CREATE_KEY_DISPOSITION(1u32);
pub const REG_OPENED_EXISTING_KEY: REG_CREATE_KEY_DISPOSITION = REG_CREATE_KEY_DISPOSITION(2u32);
impl ::core::marker::Copy for REG_CREATE_KEY_DISPOSITION {}
impl ::core::clone::Clone for REG_CREATE_KEY_DISPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_CREATE_KEY_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REG_CREATE_KEY_DISPOSITION {
    type Abi = Self;
}
impl ::core::fmt::Debug for REG_CREATE_KEY_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_CREATE_KEY_DISPOSITION").field(&self.0).finish()
    }
}
pub const REG_KEY_INSTDEV: &str = "Installed";
pub const REG_MUI_STRING_TRUNCATE: u32 = 1u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_NOTIFY_FILTER(pub u32);
pub const REG_NOTIFY_CHANGE_NAME: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(1u32);
pub const REG_NOTIFY_CHANGE_ATTRIBUTES: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(2u32);
pub const REG_NOTIFY_CHANGE_LAST_SET: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(4u32);
pub const REG_NOTIFY_CHANGE_SECURITY: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(8u32);
pub const REG_NOTIFY_THREAD_AGNOSTIC: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(268435456u32);
impl ::core::marker::Copy for REG_NOTIFY_FILTER {}
impl ::core::clone::Clone for REG_NOTIFY_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_NOTIFY_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REG_NOTIFY_FILTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for REG_NOTIFY_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_NOTIFY_FILTER").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REG_NOTIFY_FILTER {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REG_NOTIFY_FILTER {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REG_NOTIFY_FILTER {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REG_NOTIFY_FILTER {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REG_NOTIFY_FILTER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_OPEN_CREATE_OPTIONS(pub u32);
pub const REG_OPTION_RESERVED: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(0u32);
pub const REG_OPTION_NON_VOLATILE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(0u32);
pub const REG_OPTION_VOLATILE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(1u32);
pub const REG_OPTION_CREATE_LINK: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(2u32);
pub const REG_OPTION_BACKUP_RESTORE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(4u32);
pub const REG_OPTION_OPEN_LINK: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(8u32);
pub const REG_OPTION_DONT_VIRTUALIZE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(16u32);
impl ::core::marker::Copy for REG_OPEN_CREATE_OPTIONS {}
impl ::core::clone::Clone for REG_OPEN_CREATE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_OPEN_CREATE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REG_OPEN_CREATE_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REG_OPEN_CREATE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_OPEN_CREATE_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REG_OPEN_CREATE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REG_OPEN_CREATE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const REG_PROCESS_APPKEY: u32 = 1u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_RESTORE_KEY_FLAGS(pub i32);
pub const REG_FORCE_RESTORE: REG_RESTORE_KEY_FLAGS = REG_RESTORE_KEY_FLAGS(8i32);
pub const REG_WHOLE_HIVE_VOLATILE: REG_RESTORE_KEY_FLAGS = REG_RESTORE_KEY_FLAGS(1i32);
impl ::core::marker::Copy for REG_RESTORE_KEY_FLAGS {}
impl ::core::clone::Clone for REG_RESTORE_KEY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_RESTORE_KEY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REG_RESTORE_KEY_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REG_RESTORE_KEY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_RESTORE_KEY_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_SAM_FLAGS(pub u32);
pub const KEY_QUERY_VALUE: REG_SAM_FLAGS = REG_SAM_FLAGS(1u32);
pub const KEY_SET_VALUE: REG_SAM_FLAGS = REG_SAM_FLAGS(2u32);
pub const KEY_CREATE_SUB_KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(4u32);
pub const KEY_ENUMERATE_SUB_KEYS: REG_SAM_FLAGS = REG_SAM_FLAGS(8u32);
pub const KEY_NOTIFY: REG_SAM_FLAGS = REG_SAM_FLAGS(16u32);
pub const KEY_CREATE_LINK: REG_SAM_FLAGS = REG_SAM_FLAGS(32u32);
pub const KEY_WOW64_32KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(512u32);
pub const KEY_WOW64_64KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(256u32);
pub const KEY_WOW64_RES: REG_SAM_FLAGS = REG_SAM_FLAGS(768u32);
pub const KEY_READ: REG_SAM_FLAGS = REG_SAM_FLAGS(131097u32);
pub const KEY_WRITE: REG_SAM_FLAGS = REG_SAM_FLAGS(131078u32);
pub const KEY_EXECUTE: REG_SAM_FLAGS = REG_SAM_FLAGS(131097u32);
pub const KEY_ALL_ACCESS: REG_SAM_FLAGS = REG_SAM_FLAGS(983103u32);
impl ::core::marker::Copy for REG_SAM_FLAGS {}
impl ::core::clone::Clone for REG_SAM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_SAM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REG_SAM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REG_SAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_SAM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REG_SAM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REG_SAM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REG_SAM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REG_SAM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REG_SAM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_SAVE_FORMAT(pub u32);
pub const REG_STANDARD_FORMAT: REG_SAVE_FORMAT = REG_SAVE_FORMAT(1u32);
pub const REG_LATEST_FORMAT: REG_SAVE_FORMAT = REG_SAVE_FORMAT(2u32);
pub const REG_NO_COMPRESSION: REG_SAVE_FORMAT = REG_SAVE_FORMAT(4u32);
impl ::core::marker::Copy for REG_SAVE_FORMAT {}
impl ::core::clone::Clone for REG_SAVE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_SAVE_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REG_SAVE_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for REG_SAVE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_SAVE_FORMAT").field(&self.0).finish()
    }
}
pub const REG_SECURE_CONNECTION: u32 = 1u32;
pub const REG_USE_CURRENT_SECURITY_CONTEXT: u32 = 2u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REG_VALUE_TYPE(pub u32);
pub const REG_NONE: REG_VALUE_TYPE = REG_VALUE_TYPE(0u32);
pub const REG_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(1u32);
pub const REG_EXPAND_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(2u32);
pub const REG_BINARY: REG_VALUE_TYPE = REG_VALUE_TYPE(3u32);
pub const REG_DWORD: REG_VALUE_TYPE = REG_VALUE_TYPE(4u32);
pub const REG_DWORD_LITTLE_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(4u32);
pub const REG_DWORD_BIG_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(5u32);
pub const REG_LINK: REG_VALUE_TYPE = REG_VALUE_TYPE(6u32);
pub const REG_MULTI_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(7u32);
pub const REG_RESOURCE_LIST: REG_VALUE_TYPE = REG_VALUE_TYPE(8u32);
pub const REG_FULL_RESOURCE_DESCRIPTOR: REG_VALUE_TYPE = REG_VALUE_TYPE(9u32);
pub const REG_RESOURCE_REQUIREMENTS_LIST: REG_VALUE_TYPE = REG_VALUE_TYPE(10u32);
pub const REG_QWORD: REG_VALUE_TYPE = REG_VALUE_TYPE(11u32);
pub const REG_QWORD_LITTLE_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(11u32);
impl ::core::marker::Copy for REG_VALUE_TYPE {}
impl ::core::clone::Clone for REG_VALUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REG_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REG_VALUE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for REG_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REG_VALUE_TYPE").field(&self.0).finish()
    }
}
pub const RRF_NOEXPAND: u32 = 268435456u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RRF_RT(pub u32);
pub const RRF_RT_ANY: RRF_RT = RRF_RT(65535u32);
pub const RRF_RT_DWORD: RRF_RT = RRF_RT(24u32);
pub const RRF_RT_QWORD: RRF_RT = RRF_RT(72u32);
pub const RRF_RT_REG_BINARY: RRF_RT = RRF_RT(8u32);
pub const RRF_RT_REG_DWORD: RRF_RT = RRF_RT(16u32);
pub const RRF_RT_REG_EXPAND_SZ: RRF_RT = RRF_RT(4u32);
pub const RRF_RT_REG_MULTI_SZ: RRF_RT = RRF_RT(32u32);
pub const RRF_RT_REG_NONE: RRF_RT = RRF_RT(1u32);
pub const RRF_RT_REG_QWORD: RRF_RT = RRF_RT(64u32);
pub const RRF_RT_REG_SZ: RRF_RT = RRF_RT(2u32);
impl ::core::marker::Copy for RRF_RT {}
impl ::core::clone::Clone for RRF_RT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RRF_RT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RRF_RT {
    type Abi = Self;
}
impl ::core::fmt::Debug for RRF_RT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RRF_RT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RRF_RT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RRF_RT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RRF_RT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RRF_RT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RRF_RT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RRF_SUBKEY_WOW6432KEY: u32 = 131072u32;
pub const RRF_SUBKEY_WOW6464KEY: u32 = 65536u32;
pub const RRF_WOW64_MASK: u32 = 196608u32;
pub const RRF_ZEROONFAILURE: u32 = 536870912u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCloseKey<'a, P0>(hkey: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegCloseKey(hkey: HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegCloseKey(hkey.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegConnectRegistryA<'a, P0, P1>(lpmachinename: P0, hkey: P1, phkresult: &mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegConnectRegistryA(lpmachinename: ::windows::core::PCSTR, hkey: HKEY, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegConnectRegistryA(lpmachinename.into(), hkey.into(), ::core::mem::transmute(phkresult))
}
#[inline]
pub unsafe fn RegConnectRegistryExA<'a, P0, P1>(lpmachinename: P0, hkey: P1, flags: u32, phkresult: &mut HKEY) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegConnectRegistryExA(lpmachinename: ::windows::core::PCSTR, hkey: HKEY, flags: u32, phkresult: *mut HKEY) -> i32;
    }
    RegConnectRegistryExA(lpmachinename.into(), hkey.into(), flags, ::core::mem::transmute(phkresult))
}
#[inline]
pub unsafe fn RegConnectRegistryExW<'a, P0, P1>(lpmachinename: P0, hkey: P1, flags: u32, phkresult: &mut HKEY) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegConnectRegistryExW(lpmachinename: ::windows::core::PCWSTR, hkey: HKEY, flags: u32, phkresult: *mut HKEY) -> i32;
    }
    RegConnectRegistryExW(lpmachinename.into(), hkey.into(), flags, ::core::mem::transmute(phkresult))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegConnectRegistryW<'a, P0, P1>(lpmachinename: P0, hkey: P1, phkresult: &mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegConnectRegistryW(lpmachinename: ::windows::core::PCWSTR, hkey: HKEY, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegConnectRegistryW(lpmachinename.into(), hkey.into(), ::core::mem::transmute(phkresult))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCopyTreeA<'a, P0, P1, P2>(hkeysrc: P0, lpsubkey: P1, hkeydest: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegCopyTreeA(hkeysrc: HKEY, lpsubkey: ::windows::core::PCSTR, hkeydest: HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegCopyTreeA(hkeysrc.into(), lpsubkey.into(), hkeydest.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCopyTreeW<'a, P0, P1, P2>(hkeysrc: P0, lpsubkey: P1, hkeydest: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegCopyTreeW(hkeysrc: HKEY, lpsubkey: ::windows::core::PCWSTR, hkeydest: HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegCopyTreeW(hkeysrc.into(), lpsubkey.into(), hkeydest.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCreateKeyA<'a, P0, P1>(hkey: P0, lpsubkey: P1, phkresult: &mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegCreateKeyA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegCreateKeyA(hkey.into(), lpsubkey.into(), ::core::mem::transmute(phkresult))
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyExA<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: ::core::option::Option<&super::super::Security::SECURITY_ATTRIBUTES>, phkresult: &mut HKEY, lpdwdisposition: ::core::option::Option<&mut REG_CREATE_KEY_DISPOSITION>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegCreateKeyExA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, reserved: u32, lpclass: ::windows::core::PCSTR, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut HKEY, lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION) -> super::super::Foundation::WIN32_ERROR;
    }
    RegCreateKeyExA(hkey.into(), lpsubkey.into(), reserved, lpclass.into(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes), ::core::mem::transmute(phkresult), ::core::mem::transmute(lpdwdisposition))
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyExW<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: ::core::option::Option<&super::super::Security::SECURITY_ATTRIBUTES>, phkresult: &mut HKEY, lpdwdisposition: ::core::option::Option<&mut REG_CREATE_KEY_DISPOSITION>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegCreateKeyExW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, reserved: u32, lpclass: ::windows::core::PCWSTR, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut HKEY, lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION) -> super::super::Foundation::WIN32_ERROR;
    }
    RegCreateKeyExW(hkey.into(), lpsubkey.into(), reserved, lpclass.into(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes), ::core::mem::transmute(phkresult), ::core::mem::transmute(lpdwdisposition))
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyTransactedA<'a, P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: ::core::option::Option<&super::super::Security::SECURITY_ATTRIBUTES>, phkresult: &mut HKEY, lpdwdisposition: ::core::option::Option<&mut REG_CREATE_KEY_DISPOSITION>, htransaction: P3, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegCreateKeyTransactedA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, reserved: u32, lpclass: ::windows::core::PCSTR, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut HKEY, lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION, htransaction: super::super::Foundation::HANDLE, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR;
    }
    RegCreateKeyTransactedA(hkey.into(), lpsubkey.into(), reserved, lpclass.into(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes), ::core::mem::transmute(phkresult), ::core::mem::transmute(lpdwdisposition), htransaction.into(), ::core::mem::transmute(pextendedparemeter))
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyTransactedW<'a, P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: ::core::option::Option<&super::super::Security::SECURITY_ATTRIBUTES>, phkresult: &mut HKEY, lpdwdisposition: ::core::option::Option<&mut REG_CREATE_KEY_DISPOSITION>, htransaction: P3, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegCreateKeyTransactedW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, reserved: u32, lpclass: ::windows::core::PCWSTR, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut HKEY, lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION, htransaction: super::super::Foundation::HANDLE, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR;
    }
    RegCreateKeyTransactedW(hkey.into(), lpsubkey.into(), reserved, lpclass.into(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes), ::core::mem::transmute(phkresult), ::core::mem::transmute(lpdwdisposition), htransaction.into(), ::core::mem::transmute(pextendedparemeter))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCreateKeyW<'a, P0, P1>(hkey: P0, lpsubkey: P1, phkresult: &mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegCreateKeyW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegCreateKeyW(hkey.into(), lpsubkey.into(), ::core::mem::transmute(phkresult))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyA<'a, P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDeleteKeyA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDeleteKeyA(hkey.into(), lpsubkey.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyExA<'a, P0, P1>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDeleteKeyExA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, samdesired: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDeleteKeyExA(hkey.into(), lpsubkey.into(), samdesired, reserved)
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyExW<'a, P0, P1>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDeleteKeyExW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, samdesired: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDeleteKeyExW(hkey.into(), lpsubkey.into(), samdesired, reserved)
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyTransactedA<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32, htransaction: P2, pextendedparameter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDeleteKeyTransactedA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, samdesired: u32, reserved: u32, htransaction: super::super::Foundation::HANDLE, pextendedparameter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDeleteKeyTransactedA(hkey.into(), lpsubkey.into(), samdesired, reserved, htransaction.into(), ::core::mem::transmute(pextendedparameter))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyTransactedW<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32, htransaction: P2, pextendedparameter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDeleteKeyTransactedW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, samdesired: u32, reserved: u32, htransaction: super::super::Foundation::HANDLE, pextendedparameter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDeleteKeyTransactedW(hkey.into(), lpsubkey.into(), samdesired, reserved, htransaction.into(), ::core::mem::transmute(pextendedparameter))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyValueA<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDeleteKeyValueA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, lpvaluename: ::windows::core::PCSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDeleteKeyValueA(hkey.into(), lpsubkey.into(), lpvaluename.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyValueW<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDeleteKeyValueW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, lpvaluename: ::windows::core::PCWSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDeleteKeyValueW(hkey.into(), lpsubkey.into(), lpvaluename.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyW<'a, P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDeleteKeyW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDeleteKeyW(hkey.into(), lpsubkey.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteTreeA<'a, P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDeleteTreeA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDeleteTreeA(hkey.into(), lpsubkey.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteTreeW<'a, P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDeleteTreeW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDeleteTreeW(hkey.into(), lpsubkey.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteValueA<'a, P0, P1>(hkey: P0, lpvaluename: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDeleteValueA(hkey: HKEY, lpvaluename: ::windows::core::PCSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDeleteValueA(hkey.into(), lpvaluename.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteValueW<'a, P0, P1>(hkey: P0, lpvaluename: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDeleteValueW(hkey: HKEY, lpvaluename: ::windows::core::PCWSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDeleteValueW(hkey.into(), lpvaluename.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDisablePredefinedCache() -> super::super::Foundation::WIN32_ERROR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDisablePredefinedCache() -> super::super::Foundation::WIN32_ERROR;
    }
    RegDisablePredefinedCache()
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDisablePredefinedCacheEx() -> super::super::Foundation::WIN32_ERROR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDisablePredefinedCacheEx() -> super::super::Foundation::WIN32_ERROR;
    }
    RegDisablePredefinedCacheEx()
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDisableReflectionKey<'a, P0>(hbase: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegDisableReflectionKey(hbase: HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegDisableReflectionKey(hbase.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnableReflectionKey<'a, P0>(hbase: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegEnableReflectionKey(hbase: HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegEnableReflectionKey(hbase.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyA<'a, P0>(hkey: P0, dwindex: u32, lpname: ::core::option::Option<&mut [u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegEnumKeyA(hkey: HKEY, dwindex: u32, lpname: ::windows::core::PSTR, cchname: u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegEnumKeyA(hkey.into(), dwindex, ::core::mem::transmute(lpname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpname.as_deref().map_or(0, |slice| slice.len() as _))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyExA<'a, P0>(hkey: P0, dwindex: u32, lpname: ::windows::core::PSTR, lpcchname: &mut u32, lpreserved: &mut u32, lpclass: ::windows::core::PSTR, lpcchclass: ::core::option::Option<&mut u32>, lpftlastwritetime: ::core::option::Option<&mut super::super::Foundation::FILETIME>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegEnumKeyExA(hkey: HKEY, dwindex: u32, lpname: ::windows::core::PSTR, lpcchname: *mut u32, lpreserved: *mut u32, lpclass: ::windows::core::PSTR, lpcchclass: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::WIN32_ERROR;
    }
    RegEnumKeyExA(hkey.into(), dwindex, ::core::mem::transmute(lpname), ::core::mem::transmute(lpcchname), ::core::mem::transmute(lpreserved), ::core::mem::transmute(lpclass), ::core::mem::transmute(lpcchclass), ::core::mem::transmute(lpftlastwritetime))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyExW<'a, P0>(hkey: P0, dwindex: u32, lpname: ::windows::core::PWSTR, lpcchname: &mut u32, lpreserved: &mut u32, lpclass: ::windows::core::PWSTR, lpcchclass: ::core::option::Option<&mut u32>, lpftlastwritetime: ::core::option::Option<&mut super::super::Foundation::FILETIME>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegEnumKeyExW(hkey: HKEY, dwindex: u32, lpname: ::windows::core::PWSTR, lpcchname: *mut u32, lpreserved: *mut u32, lpclass: ::windows::core::PWSTR, lpcchclass: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::WIN32_ERROR;
    }
    RegEnumKeyExW(hkey.into(), dwindex, ::core::mem::transmute(lpname), ::core::mem::transmute(lpcchname), ::core::mem::transmute(lpreserved), ::core::mem::transmute(lpclass), ::core::mem::transmute(lpcchclass), ::core::mem::transmute(lpftlastwritetime))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyW<'a, P0>(hkey: P0, dwindex: u32, lpname: ::core::option::Option<&mut [u16]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegEnumKeyW(hkey: HKEY, dwindex: u32, lpname: ::windows::core::PWSTR, cchname: u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegEnumKeyW(hkey.into(), dwindex, ::core::mem::transmute(lpname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpname.as_deref().map_or(0, |slice| slice.len() as _))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumValueA<'a, P0>(hkey: P0, dwindex: u32, lpvaluename: ::windows::core::PSTR, lpcchvaluename: &mut u32, lpreserved: &mut u32, lptype: ::core::option::Option<&mut u32>, lpdata: *mut u8, lpcbdata: ::core::option::Option<&mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegEnumValueA(hkey: HKEY, dwindex: u32, lpvaluename: ::windows::core::PSTR, lpcchvaluename: *mut u32, lpreserved: *mut u32, lptype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegEnumValueA(hkey.into(), dwindex, ::core::mem::transmute(lpvaluename), ::core::mem::transmute(lpcchvaluename), ::core::mem::transmute(lpreserved), ::core::mem::transmute(lptype), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpcbdata))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumValueW<'a, P0>(hkey: P0, dwindex: u32, lpvaluename: ::windows::core::PWSTR, lpcchvaluename: &mut u32, lpreserved: &mut u32, lptype: ::core::option::Option<&mut u32>, lpdata: *mut u8, lpcbdata: ::core::option::Option<&mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegEnumValueW(hkey: HKEY, dwindex: u32, lpvaluename: ::windows::core::PWSTR, lpcchvaluename: *mut u32, lpreserved: *mut u32, lptype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegEnumValueW(hkey.into(), dwindex, ::core::mem::transmute(lpvaluename), ::core::mem::transmute(lpcchvaluename), ::core::mem::transmute(lpreserved), ::core::mem::transmute(lptype), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpcbdata))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegFlushKey<'a, P0>(hkey: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegFlushKey(hkey: HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegFlushKey(hkey.into())
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegGetKeySecurity<'a, P0>(hkey: P0, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: &mut u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegGetKeySecurity(hkey: HKEY, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegGetKeySecurity(hkey.into(), securityinformation, ::core::mem::transmute(psecuritydescriptor), ::core::mem::transmute(lpcbsecuritydescriptor))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegGetValueA<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvalue: P2, dwflags: RRF_RT, pdwtype: ::core::option::Option<&mut u32>, pvdata: *mut ::core::ffi::c_void, pcbdata: ::core::option::Option<&mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegGetValueA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, lpvalue: ::windows::core::PCSTR, dwflags: RRF_RT, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegGetValueA(hkey.into(), lpsubkey.into(), lpvalue.into(), dwflags, ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvdata), ::core::mem::transmute(pcbdata))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegGetValueW<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvalue: P2, dwflags: RRF_RT, pdwtype: ::core::option::Option<&mut u32>, pvdata: *mut ::core::ffi::c_void, pcbdata: ::core::option::Option<&mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegGetValueW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, lpvalue: ::windows::core::PCWSTR, dwflags: RRF_RT, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegGetValueW(hkey.into(), lpsubkey.into(), lpvalue.into(), dwflags, ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvdata), ::core::mem::transmute(pcbdata))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadAppKeyA<'a, P0>(lpfile: P0, phkresult: &mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegLoadAppKeyA(lpfile: ::windows::core::PCSTR, phkresult: *mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegLoadAppKeyA(lpfile.into(), ::core::mem::transmute(phkresult), samdesired, dwoptions, reserved)
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadAppKeyW<'a, P0>(lpfile: P0, phkresult: &mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegLoadAppKeyW(lpfile: ::windows::core::PCWSTR, phkresult: *mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegLoadAppKeyW(lpfile.into(), ::core::mem::transmute(phkresult), samdesired, dwoptions, reserved)
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadKeyA<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, lpfile: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegLoadKeyA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, lpfile: ::windows::core::PCSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegLoadKeyA(hkey.into(), lpsubkey.into(), lpfile.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadKeyW<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, lpfile: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegLoadKeyW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, lpfile: ::windows::core::PCWSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegLoadKeyW(hkey.into(), lpsubkey.into(), lpfile.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadMUIStringA<'a, P0, P1, P2>(hkey: P0, pszvalue: P1, pszoutbuf: ::core::option::Option<&mut [u8]>, pcbdata: ::core::option::Option<&mut u32>, flags: u32, pszdirectory: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegLoadMUIStringA(hkey: HKEY, pszvalue: ::windows::core::PCSTR, pszoutbuf: ::windows::core::PSTR, cboutbuf: u32, pcbdata: *mut u32, flags: u32, pszdirectory: ::windows::core::PCSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegLoadMUIStringA(hkey.into(), pszvalue.into(), ::core::mem::transmute(pszoutbuf.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pszoutbuf.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pcbdata), flags, pszdirectory.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadMUIStringW<'a, P0, P1, P2>(hkey: P0, pszvalue: P1, pszoutbuf: ::core::option::Option<&mut [u8]>, pcbdata: ::core::option::Option<&mut u32>, flags: u32, pszdirectory: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegLoadMUIStringW(hkey: HKEY, pszvalue: ::windows::core::PCWSTR, pszoutbuf: ::windows::core::PWSTR, cboutbuf: u32, pcbdata: *mut u32, flags: u32, pszdirectory: ::windows::core::PCWSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegLoadMUIStringW(hkey.into(), pszvalue.into(), ::core::mem::transmute(pszoutbuf.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pszoutbuf.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pcbdata), flags, pszdirectory.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegNotifyChangeKeyValue<'a, P0, P1, P2, P3>(hkey: P0, bwatchsubtree: P1, dwnotifyfilter: REG_NOTIFY_FILTER, hevent: P2, fasynchronous: P3) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P3: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegNotifyChangeKeyValue(hkey: HKEY, bwatchsubtree: super::super::Foundation::BOOL, dwnotifyfilter: REG_NOTIFY_FILTER, hevent: super::super::Foundation::HANDLE, fasynchronous: super::super::Foundation::BOOL) -> super::super::Foundation::WIN32_ERROR;
    }
    RegNotifyChangeKeyValue(hkey.into(), bwatchsubtree.into(), dwnotifyfilter, hevent.into(), fasynchronous.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenCurrentUser(samdesired: u32, phkresult: &mut HKEY) -> super::super::Foundation::WIN32_ERROR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegOpenCurrentUser(samdesired: u32, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegOpenCurrentUser(samdesired, ::core::mem::transmute(phkresult))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyA<'a, P0, P1>(hkey: P0, lpsubkey: P1, phkresult: &mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegOpenKeyA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegOpenKeyA(hkey.into(), lpsubkey.into(), ::core::mem::transmute(phkresult))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyExA<'a, P0, P1>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: &mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegOpenKeyExA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegOpenKeyExA(hkey.into(), lpsubkey.into(), uloptions, samdesired, ::core::mem::transmute(phkresult))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyExW<'a, P0, P1>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: &mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegOpenKeyExW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegOpenKeyExW(hkey.into(), lpsubkey.into(), uloptions, samdesired, ::core::mem::transmute(phkresult))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyTransactedA<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: &mut HKEY, htransaction: P2, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegOpenKeyTransactedA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY, htransaction: super::super::Foundation::HANDLE, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR;
    }
    RegOpenKeyTransactedA(hkey.into(), lpsubkey.into(), uloptions, samdesired, ::core::mem::transmute(phkresult), htransaction.into(), ::core::mem::transmute(pextendedparemeter))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyTransactedW<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: &mut HKEY, htransaction: P2, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegOpenKeyTransactedW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY, htransaction: super::super::Foundation::HANDLE, pextendedparemeter: *mut ::core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR;
    }
    RegOpenKeyTransactedW(hkey.into(), lpsubkey.into(), uloptions, samdesired, ::core::mem::transmute(phkresult), htransaction.into(), ::core::mem::transmute(pextendedparemeter))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyW<'a, P0, P1>(hkey: P0, lpsubkey: P1, phkresult: &mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegOpenKeyW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegOpenKeyW(hkey.into(), lpsubkey.into(), ::core::mem::transmute(phkresult))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenUserClassesRoot<'a, P0>(htoken: P0, dwoptions: u32, samdesired: u32, phkresult: &mut HKEY) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegOpenUserClassesRoot(htoken: super::super::Foundation::HANDLE, dwoptions: u32, samdesired: u32, phkresult: *mut HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegOpenUserClassesRoot(htoken.into(), dwoptions, samdesired, ::core::mem::transmute(phkresult))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOverridePredefKey<'a, P0, P1>(hkey: P0, hnewhkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegOverridePredefKey(hkey: HKEY, hnewhkey: HKEY) -> super::super::Foundation::WIN32_ERROR;
    }
    RegOverridePredefKey(hkey.into(), hnewhkey.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryInfoKeyA<'a, P0>(hkey: P0, lpclass: ::windows::core::PSTR, lpcchclass: ::core::option::Option<&mut u32>, lpreserved: &mut u32, lpcsubkeys: ::core::option::Option<&mut u32>, lpcbmaxsubkeylen: ::core::option::Option<&mut u32>, lpcbmaxclasslen: ::core::option::Option<&mut u32>, lpcvalues: ::core::option::Option<&mut u32>, lpcbmaxvaluenamelen: ::core::option::Option<&mut u32>, lpcbmaxvaluelen: ::core::option::Option<&mut u32>, lpcbsecuritydescriptor: ::core::option::Option<&mut u32>, lpftlastwritetime: ::core::option::Option<&mut super::super::Foundation::FILETIME>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegQueryInfoKeyA(hkey: HKEY, lpclass: ::windows::core::PSTR, lpcchclass: *mut u32, lpreserved: *mut u32, lpcsubkeys: *mut u32, lpcbmaxsubkeylen: *mut u32, lpcbmaxclasslen: *mut u32, lpcvalues: *mut u32, lpcbmaxvaluenamelen: *mut u32, lpcbmaxvaluelen: *mut u32, lpcbsecuritydescriptor: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::WIN32_ERROR;
    }
    RegQueryInfoKeyA(hkey.into(), ::core::mem::transmute(lpclass), ::core::mem::transmute(lpcchclass), ::core::mem::transmute(lpreserved), ::core::mem::transmute(lpcsubkeys), ::core::mem::transmute(lpcbmaxsubkeylen), ::core::mem::transmute(lpcbmaxclasslen), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lpcbmaxvaluenamelen), ::core::mem::transmute(lpcbmaxvaluelen), ::core::mem::transmute(lpcbsecuritydescriptor), ::core::mem::transmute(lpftlastwritetime))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryInfoKeyW<'a, P0>(hkey: P0, lpclass: ::windows::core::PWSTR, lpcchclass: ::core::option::Option<&mut u32>, lpreserved: &mut u32, lpcsubkeys: ::core::option::Option<&mut u32>, lpcbmaxsubkeylen: ::core::option::Option<&mut u32>, lpcbmaxclasslen: ::core::option::Option<&mut u32>, lpcvalues: ::core::option::Option<&mut u32>, lpcbmaxvaluenamelen: ::core::option::Option<&mut u32>, lpcbmaxvaluelen: ::core::option::Option<&mut u32>, lpcbsecuritydescriptor: ::core::option::Option<&mut u32>, lpftlastwritetime: ::core::option::Option<&mut super::super::Foundation::FILETIME>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegQueryInfoKeyW(hkey: HKEY, lpclass: ::windows::core::PWSTR, lpcchclass: *mut u32, lpreserved: *mut u32, lpcsubkeys: *mut u32, lpcbmaxsubkeylen: *mut u32, lpcbmaxclasslen: *mut u32, lpcvalues: *mut u32, lpcbmaxvaluenamelen: *mut u32, lpcbmaxvaluelen: *mut u32, lpcbsecuritydescriptor: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::WIN32_ERROR;
    }
    RegQueryInfoKeyW(hkey.into(), ::core::mem::transmute(lpclass), ::core::mem::transmute(lpcchclass), ::core::mem::transmute(lpreserved), ::core::mem::transmute(lpcsubkeys), ::core::mem::transmute(lpcbmaxsubkeylen), ::core::mem::transmute(lpcbmaxclasslen), ::core::mem::transmute(lpcvalues), ::core::mem::transmute(lpcbmaxvaluenamelen), ::core::mem::transmute(lpcbmaxvaluelen), ::core::mem::transmute(lpcbsecuritydescriptor), ::core::mem::transmute(lpftlastwritetime))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryMultipleValuesA<'a, P0>(hkey: P0, val_list: &mut [VALENTA], lpvaluebuf: ::windows::core::PSTR, ldwtotsize: ::core::option::Option<&mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegQueryMultipleValuesA(hkey: HKEY, val_list: *mut VALENTA, num_vals: u32, lpvaluebuf: ::windows::core::PSTR, ldwtotsize: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegQueryMultipleValuesA(hkey.into(), ::core::mem::transmute(val_list.as_ptr()), val_list.len() as _, ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(ldwtotsize))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryMultipleValuesW<'a, P0>(hkey: P0, val_list: &mut [VALENTW], lpvaluebuf: ::windows::core::PWSTR, ldwtotsize: ::core::option::Option<&mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegQueryMultipleValuesW(hkey: HKEY, val_list: *mut VALENTW, num_vals: u32, lpvaluebuf: ::windows::core::PWSTR, ldwtotsize: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegQueryMultipleValuesW(hkey.into(), ::core::mem::transmute(val_list.as_ptr()), val_list.len() as _, ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(ldwtotsize))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryReflectionKey<'a, P0>(hbase: P0, bisreflectiondisabled: &mut super::super::Foundation::BOOL) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegQueryReflectionKey(hbase: HKEY, bisreflectiondisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::WIN32_ERROR;
    }
    RegQueryReflectionKey(hbase.into(), ::core::mem::transmute(bisreflectiondisabled))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueA<'a, P0, P1>(hkey: P0, lpsubkey: P1, lpdata: ::windows::core::PSTR, lpcbdata: ::core::option::Option<&mut i32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegQueryValueA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, lpdata: ::windows::core::PSTR, lpcbdata: *mut i32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegQueryValueA(hkey.into(), lpsubkey.into(), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpcbdata))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueExA<'a, P0, P1>(hkey: P0, lpvaluename: P1, lpreserved: &mut u32, lptype: ::core::option::Option<&mut REG_VALUE_TYPE>, lpdata: *mut u8, lpcbdata: ::core::option::Option<&mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegQueryValueExA(hkey: HKEY, lpvaluename: ::windows::core::PCSTR, lpreserved: *mut u32, lptype: *mut REG_VALUE_TYPE, lpdata: *mut u8, lpcbdata: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegQueryValueExA(hkey.into(), lpvaluename.into(), ::core::mem::transmute(lpreserved), ::core::mem::transmute(lptype), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpcbdata))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueExW<'a, P0, P1>(hkey: P0, lpvaluename: P1, lpreserved: &mut u32, lptype: ::core::option::Option<&mut REG_VALUE_TYPE>, lpdata: *mut u8, lpcbdata: ::core::option::Option<&mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegQueryValueExW(hkey: HKEY, lpvaluename: ::windows::core::PCWSTR, lpreserved: *mut u32, lptype: *mut REG_VALUE_TYPE, lpdata: *mut u8, lpcbdata: *mut u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegQueryValueExW(hkey.into(), lpvaluename.into(), ::core::mem::transmute(lpreserved), ::core::mem::transmute(lptype), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpcbdata))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueW<'a, P0, P1>(hkey: P0, lpsubkey: P1, lpdata: ::windows::core::PWSTR, lpcbdata: ::core::option::Option<&mut i32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegQueryValueW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, lpdata: ::windows::core::PWSTR, lpcbdata: *mut i32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegQueryValueW(hkey.into(), lpsubkey.into(), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpcbdata))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegRenameKey<'a, P0, P1, P2>(hkey: P0, lpsubkeyname: P1, lpnewkeyname: P2) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegRenameKey(hkey: HKEY, lpsubkeyname: ::windows::core::PCWSTR, lpnewkeyname: ::windows::core::PCWSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegRenameKey(hkey.into(), lpsubkeyname.into(), lpnewkeyname.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegReplaceKeyA<'a, P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, lpnewfile: P2, lpoldfile: P3) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegReplaceKeyA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, lpnewfile: ::windows::core::PCSTR, lpoldfile: ::windows::core::PCSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegReplaceKeyA(hkey.into(), lpsubkey.into(), lpnewfile.into(), lpoldfile.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegReplaceKeyW<'a, P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, lpnewfile: P2, lpoldfile: P3) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegReplaceKeyW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, lpnewfile: ::windows::core::PCWSTR, lpoldfile: ::windows::core::PCWSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegReplaceKeyW(hkey.into(), lpsubkey.into(), lpnewfile.into(), lpoldfile.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegRestoreKeyA<'a, P0, P1>(hkey: P0, lpfile: P1, dwflags: REG_RESTORE_KEY_FLAGS) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegRestoreKeyA(hkey: HKEY, lpfile: ::windows::core::PCSTR, dwflags: REG_RESTORE_KEY_FLAGS) -> super::super::Foundation::WIN32_ERROR;
    }
    RegRestoreKeyA(hkey.into(), lpfile.into(), dwflags)
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegRestoreKeyW<'a, P0, P1>(hkey: P0, lpfile: P1, dwflags: REG_RESTORE_KEY_FLAGS) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegRestoreKeyW(hkey: HKEY, lpfile: ::windows::core::PCWSTR, dwflags: REG_RESTORE_KEY_FLAGS) -> super::super::Foundation::WIN32_ERROR;
    }
    RegRestoreKeyW(hkey.into(), lpfile.into(), dwflags)
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyA<'a, P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: ::core::option::Option<&super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegSaveKeyA(hkey: HKEY, lpfile: ::windows::core::PCSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::WIN32_ERROR;
    }
    RegSaveKeyA(hkey.into(), lpfile.into(), ::core::mem::transmute(lpsecurityattributes))
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyExA<'a, P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: ::core::option::Option<&super::super::Security::SECURITY_ATTRIBUTES>, flags: REG_SAVE_FORMAT) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegSaveKeyExA(hkey: HKEY, lpfile: ::windows::core::PCSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flags: REG_SAVE_FORMAT) -> super::super::Foundation::WIN32_ERROR;
    }
    RegSaveKeyExA(hkey.into(), lpfile.into(), ::core::mem::transmute(lpsecurityattributes), flags)
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyExW<'a, P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: ::core::option::Option<&super::super::Security::SECURITY_ATTRIBUTES>, flags: REG_SAVE_FORMAT) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegSaveKeyExW(hkey: HKEY, lpfile: ::windows::core::PCWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flags: REG_SAVE_FORMAT) -> super::super::Foundation::WIN32_ERROR;
    }
    RegSaveKeyExW(hkey.into(), lpfile.into(), ::core::mem::transmute(lpsecurityattributes), flags)
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyW<'a, P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: ::core::option::Option<&super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegSaveKeyW(hkey: HKEY, lpfile: ::windows::core::PCWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::WIN32_ERROR;
    }
    RegSaveKeyW(hkey.into(), lpfile.into(), ::core::mem::transmute(lpsecurityattributes))
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSetKeySecurity<'a, P0, P1>(hkey: P0, securityinformation: u32, psecuritydescriptor: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegSetKeySecurity(hkey: HKEY, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegSetKeySecurity(hkey.into(), securityinformation, psecuritydescriptor.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetKeyValueA<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2, dwtype: u32, lpdata: ::core::option::Option<&[u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegSetKeyValueA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, lpvaluename: ::windows::core::PCSTR, dwtype: u32, lpdata: *const ::core::ffi::c_void, cbdata: u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegSetKeyValueA(hkey.into(), lpsubkey.into(), lpvaluename.into(), dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetKeyValueW<'a, P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2, dwtype: u32, lpdata: ::core::option::Option<&[u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegSetKeyValueW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, lpvaluename: ::windows::core::PCWSTR, dwtype: u32, lpdata: *const ::core::ffi::c_void, cbdata: u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegSetKeyValueW(hkey.into(), lpsubkey.into(), lpvaluename.into(), dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueA<'a, P0, P1>(hkey: P0, lpsubkey: P1, dwtype: REG_VALUE_TYPE, lpdata: ::core::option::Option<&[u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegSetValueA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR, dwtype: REG_VALUE_TYPE, lpdata: ::windows::core::PCSTR, cbdata: u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegSetValueA(hkey.into(), lpsubkey.into(), dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueExA<'a, P0, P1>(hkey: P0, lpvaluename: P1, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: ::core::option::Option<&[u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegSetValueExA(hkey: HKEY, lpvaluename: ::windows::core::PCSTR, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: *const u8, cbdata: u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegSetValueExA(hkey.into(), lpvaluename.into(), reserved, dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueExW<'a, P0, P1>(hkey: P0, lpvaluename: P1, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: ::core::option::Option<&[u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegSetValueExW(hkey: HKEY, lpvaluename: ::windows::core::PCWSTR, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: *const u8, cbdata: u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegSetValueExW(hkey.into(), lpvaluename.into(), reserved, dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueW<'a, P0, P1>(hkey: P0, lpsubkey: P1, dwtype: REG_VALUE_TYPE, lpdata: ::core::option::Option<&[u8]>) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegSetValueW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR, dwtype: REG_VALUE_TYPE, lpdata: ::windows::core::PCWSTR, cbdata: u32) -> super::super::Foundation::WIN32_ERROR;
    }
    RegSetValueW(hkey.into(), lpsubkey.into(), dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegUnLoadKeyA<'a, P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegUnLoadKeyA(hkey: HKEY, lpsubkey: ::windows::core::PCSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegUnLoadKeyA(hkey.into(), lpsubkey.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegUnLoadKeyW<'a, P0, P1>(hkey: P0, lpsubkey: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegUnLoadKeyW(hkey: HKEY, lpsubkey: ::windows::core::PCWSTR) -> super::super::Foundation::WIN32_ERROR;
    }
    RegUnLoadKeyW(hkey.into(), lpsubkey.into())
}
pub const SUF_BATCHINF: i32 = 4i32;
pub const SUF_CLEAN: i32 = 8i32;
pub const SUF_EXPRESS: i32 = 2i32;
pub const SUF_FIRSTTIME: i32 = 1i32;
pub const SUF_INSETUP: i32 = 16i32;
pub const SUF_NETHDBOOT: i32 = 64i32;
pub const SUF_NETRPLBOOT: i32 = 128i32;
pub const SUF_NETSETUP: i32 = 32i32;
pub const SUF_SBSCOPYOK: i32 = 256i32;
#[repr(C)]
pub struct VALENTA {
    pub ve_valuename: ::windows::core::PSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: REG_VALUE_TYPE,
}
impl ::core::marker::Copy for VALENTA {}
impl ::core::clone::Clone for VALENTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VALENTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VALENTA").field("ve_valuename", &self.ve_valuename).field("ve_valuelen", &self.ve_valuelen).field("ve_valueptr", &self.ve_valueptr).field("ve_type", &self.ve_type).finish()
    }
}
unsafe impl ::windows::core::Abi for VALENTA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VALENTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VALENTA>()) == 0 }
    }
}
impl ::core::cmp::Eq for VALENTA {}
impl ::core::default::Default for VALENTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct VALENTW {
    pub ve_valuename: ::windows::core::PWSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: REG_VALUE_TYPE,
}
impl ::core::marker::Copy for VALENTW {}
impl ::core::clone::Clone for VALENTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VALENTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VALENTW").field("ve_valuename", &self.ve_valuename).field("ve_valuelen", &self.ve_valuelen).field("ve_valueptr", &self.ve_valueptr).field("ve_type", &self.ve_type).finish()
    }
}
unsafe impl ::windows::core::Abi for VALENTW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VALENTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VALENTW>()) == 0 }
    }
}
impl ::core::cmp::Eq for VALENTW {}
impl ::core::default::Default for VALENTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const VPDF_DISABLEPWRMGMT: u32 = 1u32;
pub const VPDF_DISABLEPWRSTATUSPOLL: u32 = 8u32;
pub const VPDF_DISABLERINGRESUME: u32 = 16u32;
pub const VPDF_FORCEAPM10MODE: u32 = 2u32;
pub const VPDF_SHOWMULTIBATT: u32 = 32u32;
pub const VPDF_SKIPINTELSLCHECK: u32 = 4u32;
#[repr(C)]
pub struct provider_info {
    pub pi_R0_1val: PQUERYHANDLER,
    pub pi_R0_allvals: PQUERYHANDLER,
    pub pi_R3_1val: PQUERYHANDLER,
    pub pi_R3_allvals: PQUERYHANDLER,
    pub pi_flags: u32,
    pub pi_key_context: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for provider_info {}
impl ::core::clone::Clone for provider_info {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for provider_info {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("provider_info").field("pi_R0_1val", &self.pi_R0_1val.map(|f| f as usize)).field("pi_R0_allvals", &self.pi_R0_allvals.map(|f| f as usize)).field("pi_R3_1val", &self.pi_R3_1val.map(|f| f as usize)).field("pi_R3_allvals", &self.pi_R3_allvals.map(|f| f as usize)).field("pi_flags", &self.pi_flags).field("pi_key_context", &self.pi_key_context).finish()
    }
}
unsafe impl ::windows::core::Abi for provider_info {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for provider_info {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<provider_info>()) == 0 }
    }
}
impl ::core::cmp::Eq for provider_info {}
impl ::core::default::Default for provider_info {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct pvalueA {
    pub pv_valuename: ::windows::core::PSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut ::core::ffi::c_void,
    pub pv_type: u32,
}
impl ::core::marker::Copy for pvalueA {}
impl ::core::clone::Clone for pvalueA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for pvalueA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pvalueA").field("pv_valuename", &self.pv_valuename).field("pv_valuelen", &self.pv_valuelen).field("pv_value_context", &self.pv_value_context).field("pv_type", &self.pv_type).finish()
    }
}
unsafe impl ::windows::core::Abi for pvalueA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for pvalueA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<pvalueA>()) == 0 }
    }
}
impl ::core::cmp::Eq for pvalueA {}
impl ::core::default::Default for pvalueA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct pvalueW {
    pub pv_valuename: ::windows::core::PWSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut ::core::ffi::c_void,
    pub pv_type: u32,
}
impl ::core::marker::Copy for pvalueW {}
impl ::core::clone::Clone for pvalueW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for pvalueW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pvalueW").field("pv_valuename", &self.pv_valuename).field("pv_valuelen", &self.pv_valuelen).field("pv_value_context", &self.pv_value_context).field("pv_type", &self.pv_type).finish()
    }
}
unsafe impl ::windows::core::Abi for pvalueW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for pvalueW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<pvalueW>()) == 0 }
    }
}
impl ::core::cmp::Eq for pvalueW {}
impl ::core::default::Default for pvalueW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct val_context {
    pub valuelen: i32,
    pub value_context: *mut ::core::ffi::c_void,
    pub val_buff_ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for val_context {}
impl ::core::clone::Clone for val_context {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for val_context {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("val_context").field("valuelen", &self.valuelen).field("value_context", &self.value_context).field("val_buff_ptr", &self.val_buff_ptr).finish()
    }
}
unsafe impl ::windows::core::Abi for val_context {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for val_context {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<val_context>()) == 0 }
    }
}
impl ::core::cmp::Eq for val_context {}
impl ::core::default::Default for val_context {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl DSKTLSYSTEMTIME {}
impl ::std::default::Default for DSKTLSYSTEMTIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSKTLSYSTEMTIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSKTLSYSTEMTIME")
            .field("wYear", &self.wYear)
            .field("wMonth", &self.wMonth)
            .field("wDayOfWeek", &self.wDayOfWeek)
            .field("wDay", &self.wDay)
            .field("wHour", &self.wHour)
            .field("wMinute", &self.wMinute)
            .field("wSecond", &self.wSecond)
            .field("wMilliseconds", &self.wMilliseconds)
            .field("wResult", &self.wResult)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DSKTLSYSTEMTIME {
    fn eq(&self, other: &Self) -> bool {
        self.wYear == other.wYear
            && self.wMonth == other.wMonth
            && self.wDayOfWeek == other.wDayOfWeek
            && self.wDay == other.wDay
            && self.wHour == other.wHour
            && self.wMinute == other.wMinute
            && self.wSecond == other.wSecond
            && self.wMilliseconds == other.wMilliseconds
            && self.wResult == other.wResult
    }
}
impl ::std::cmp::Eq for DSKTLSYSTEMTIME {}
unsafe impl ::windows::runtime::Abi for DSKTLSYSTEMTIME {
    type Abi = Self;
    type DefaultType = Self;
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
pub unsafe fn GetRegistryValueWithFallbackW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, HKEY>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkeyprimary: Param0,
    pwszprimarysubkey: Param1,
    hkeyfallback: Param2,
    pwszfallbacksubkey: Param3,
    pwszvalue: Param4,
    dwflags: u32,
    pdwtype: *mut u32,
    pvdata: *mut ::std::ffi::c_void,
    cbdatain: u32,
    pcbdataout: *mut u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRegistryValueWithFallbackW(
                hkeyprimary: HKEY,
                pwszprimarysubkey: super::super::Foundation::PWSTR,
                hkeyfallback: HKEY,
                pwszfallbacksubkey: super::super::Foundation::PWSTR,
                pwszvalue: super::super::Foundation::PWSTR,
                dwflags: u32,
                pdwtype: *mut u32,
                pvdata: *mut ::std::ffi::c_void,
                cbdatain: u32,
                pcbdataout: *mut u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(GetRegistryValueWithFallbackW(
            hkeyprimary.into_param().abi(),
            pwszprimarysubkey.into_param().abi(),
            hkeyfallback.into_param().abi(),
            pwszfallbacksubkey.into_param().abi(),
            pwszvalue.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pdwtype),
            ::std::mem::transmute(pvdata),
            ::std::mem::transmute(cbdatain),
            ::std::mem::transmute(pcbdataout),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HKEY(pub isize);
impl ::std::default::Default for HKEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HKEY {}
unsafe impl ::windows::runtime::Abi for HKEY {
    type Abi = Self;
    type DefaultType = Self;
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
pub type PQUERYHANDLER = unsafe extern "system" fn(
    keycontext: *mut ::std::ffi::c_void,
    val_list: *mut val_context,
    num_vals: u32,
    outputbuffer: *mut ::std::ffi::c_void,
    total_outlen: *mut u32,
    input_blen: u32,
) -> u32;
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
pub const REGSTR_MAX_VALUE_LENGTH: u32 = 256u32;
pub const REGSTR_VAL_MAX_HCID_LEN: u32 = 1024u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct REG_CREATE_KEY_DISPOSITION(pub u32);
pub const REG_CREATED_NEW_KEY: REG_CREATE_KEY_DISPOSITION = REG_CREATE_KEY_DISPOSITION(1u32);
pub const REG_OPENED_EXISTING_KEY: REG_CREATE_KEY_DISPOSITION = REG_CREATE_KEY_DISPOSITION(2u32);
impl ::std::convert::From<u32> for REG_CREATE_KEY_DISPOSITION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REG_CREATE_KEY_DISPOSITION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for REG_CREATE_KEY_DISPOSITION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for REG_CREATE_KEY_DISPOSITION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for REG_CREATE_KEY_DISPOSITION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for REG_CREATE_KEY_DISPOSITION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for REG_CREATE_KEY_DISPOSITION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const REG_MUI_STRING_TRUNCATE: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct REG_NOTIFY_FILTER(pub u32);
pub const REG_NOTIFY_CHANGE_NAME: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(1u32);
pub const REG_NOTIFY_CHANGE_ATTRIBUTES: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(2u32);
pub const REG_NOTIFY_CHANGE_LAST_SET: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(4u32);
pub const REG_NOTIFY_CHANGE_SECURITY: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(8u32);
pub const REG_NOTIFY_THREAD_AGNOSTIC: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(268435456u32);
impl ::std::convert::From<u32> for REG_NOTIFY_FILTER {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REG_NOTIFY_FILTER {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for REG_NOTIFY_FILTER {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for REG_NOTIFY_FILTER {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for REG_NOTIFY_FILTER {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for REG_NOTIFY_FILTER {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for REG_NOTIFY_FILTER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct REG_OPEN_CREATE_OPTIONS(pub u32);
pub const REG_OPTION_RESERVED: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(0u32);
pub const REG_OPTION_NON_VOLATILE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(0u32);
pub const REG_OPTION_VOLATILE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(1u32);
pub const REG_OPTION_CREATE_LINK: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(2u32);
pub const REG_OPTION_BACKUP_RESTORE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(4u32);
pub const REG_OPTION_OPEN_LINK: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(8u32);
pub const REG_OPTION_DONT_VIRTUALIZE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(16u32);
impl ::std::convert::From<u32> for REG_OPEN_CREATE_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REG_OPEN_CREATE_OPTIONS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for REG_OPEN_CREATE_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for REG_OPEN_CREATE_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for REG_OPEN_CREATE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const REG_PROCESS_APPKEY: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct REG_RESTORE_KEY_FLAGS(pub i32);
pub const REG_FORCE_RESTORE: REG_RESTORE_KEY_FLAGS = REG_RESTORE_KEY_FLAGS(8i32);
pub const REG_WHOLE_HIVE_VOLATILE: REG_RESTORE_KEY_FLAGS = REG_RESTORE_KEY_FLAGS(1i32);
impl ::std::convert::From<i32> for REG_RESTORE_KEY_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REG_RESTORE_KEY_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
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
impl ::std::convert::From<u32> for REG_SAM_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REG_SAM_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for REG_SAM_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for REG_SAM_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for REG_SAM_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for REG_SAM_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for REG_SAM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct REG_SAVE_FORMAT(pub u32);
pub const REG_STANDARD_FORMAT: REG_SAVE_FORMAT = REG_SAVE_FORMAT(1u32);
pub const REG_LATEST_FORMAT: REG_SAVE_FORMAT = REG_SAVE_FORMAT(2u32);
pub const REG_NO_COMPRESSION: REG_SAVE_FORMAT = REG_SAVE_FORMAT(4u32);
impl ::std::convert::From<u32> for REG_SAVE_FORMAT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REG_SAVE_FORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for REG_SAVE_FORMAT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for REG_SAVE_FORMAT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for REG_SAVE_FORMAT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for REG_SAVE_FORMAT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for REG_SAVE_FORMAT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const REG_SECURE_CONNECTION: u32 = 1u32;
pub const REG_USE_CURRENT_SECURITY_CONTEXT: u32 = 2u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
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
impl ::std::convert::From<u32> for REG_VALUE_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REG_VALUE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for REG_VALUE_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for REG_VALUE_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for REG_VALUE_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for REG_VALUE_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for REG_VALUE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RRF_NOEXPAND: u32 = 268435456u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
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
impl ::std::convert::From<u32> for RRF_RT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RRF_RT {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RRF_RT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RRF_RT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RRF_RT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RRF_RT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RRF_RT {
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
pub unsafe fn RegCloseKey<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegCloseKey(hkey: HKEY) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegCloseKey(hkey.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegConnectRegistryA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, HKEY>,
>(
    lpmachinename: Param0,
    hkey: Param1,
    phkresult: *mut HKEY,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegConnectRegistryA(
                lpmachinename: super::super::Foundation::PSTR,
                hkey: HKEY,
                phkresult: *mut HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegConnectRegistryA(
            lpmachinename.into_param().abi(),
            hkey.into_param().abi(),
            ::std::mem::transmute(phkresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegConnectRegistryExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, HKEY>,
>(
    lpmachinename: Param0,
    hkey: Param1,
    flags: u32,
    phkresult: *mut HKEY,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegConnectRegistryExA(
                lpmachinename: super::super::Foundation::PSTR,
                hkey: HKEY,
                flags: u32,
                phkresult: *mut HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegConnectRegistryExA(
            lpmachinename.into_param().abi(),
            hkey.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(phkresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegConnectRegistryExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, HKEY>,
>(
    lpmachinename: Param0,
    hkey: Param1,
    flags: u32,
    phkresult: *mut HKEY,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegConnectRegistryExW(
                lpmachinename: super::super::Foundation::PWSTR,
                hkey: HKEY,
                flags: u32,
                phkresult: *mut HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegConnectRegistryExW(
            lpmachinename.into_param().abi(),
            hkey.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(phkresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegConnectRegistryW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, HKEY>,
>(
    lpmachinename: Param0,
    hkey: Param1,
    phkresult: *mut HKEY,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegConnectRegistryW(
                lpmachinename: super::super::Foundation::PWSTR,
                hkey: HKEY,
                phkresult: *mut HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegConnectRegistryW(
            lpmachinename.into_param().abi(),
            hkey.into_param().abi(),
            ::std::mem::transmute(phkresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCopyTreeA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, HKEY>,
>(
    hkeysrc: Param0,
    lpsubkey: Param1,
    hkeydest: Param2,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegCopyTreeA(
                hkeysrc: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                hkeydest: HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegCopyTreeA(
            hkeysrc.into_param().abi(),
            lpsubkey.into_param().abi(),
            hkeydest.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCopyTreeW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, HKEY>,
>(
    hkeysrc: Param0,
    lpsubkey: Param1,
    hkeydest: Param2,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegCopyTreeW(
                hkeysrc: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                hkeydest: HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegCopyTreeW(
            hkeysrc.into_param().abi(),
            lpsubkey.into_param().abi(),
            hkeydest.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCreateKeyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    phkresult: *mut HKEY,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegCreateKeyA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                phkresult: *mut HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegCreateKeyA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(phkresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    reserved: u32,
    lpclass: Param3,
    dwoptions: REG_OPEN_CREATE_OPTIONS,
    samdesired: REG_SAM_FLAGS,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    phkresult: *mut HKEY,
    lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegCreateKeyExA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                reserved: u32,
                lpclass: super::super::Foundation::PSTR,
                dwoptions: REG_OPEN_CREATE_OPTIONS,
                samdesired: REG_SAM_FLAGS,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                phkresult: *mut HKEY,
                lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegCreateKeyExA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(reserved),
            lpclass.into_param().abi(),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(phkresult),
            ::std::mem::transmute(lpdwdisposition),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    reserved: u32,
    lpclass: Param3,
    dwoptions: REG_OPEN_CREATE_OPTIONS,
    samdesired: REG_SAM_FLAGS,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    phkresult: *mut HKEY,
    lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegCreateKeyExW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                reserved: u32,
                lpclass: super::super::Foundation::PWSTR,
                dwoptions: REG_OPEN_CREATE_OPTIONS,
                samdesired: REG_SAM_FLAGS,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                phkresult: *mut HKEY,
                lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegCreateKeyExW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(reserved),
            lpclass.into_param().abi(),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(phkresult),
            ::std::mem::transmute(lpdwdisposition),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    reserved: u32,
    lpclass: Param3,
    dwoptions: REG_OPEN_CREATE_OPTIONS,
    samdesired: REG_SAM_FLAGS,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    phkresult: *mut HKEY,
    lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION,
    htransaction: Param9,
    pextendedparemeter: *mut ::std::ffi::c_void,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegCreateKeyTransactedA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                reserved: u32,
                lpclass: super::super::Foundation::PSTR,
                dwoptions: REG_OPEN_CREATE_OPTIONS,
                samdesired: REG_SAM_FLAGS,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                phkresult: *mut HKEY,
                lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION,
                htransaction: super::super::Foundation::HANDLE,
                pextendedparemeter: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegCreateKeyTransactedA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(reserved),
            lpclass.into_param().abi(),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(phkresult),
            ::std::mem::transmute(lpdwdisposition),
            htransaction.into_param().abi(),
            ::std::mem::transmute(pextendedparemeter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    reserved: u32,
    lpclass: Param3,
    dwoptions: REG_OPEN_CREATE_OPTIONS,
    samdesired: REG_SAM_FLAGS,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    phkresult: *mut HKEY,
    lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION,
    htransaction: Param9,
    pextendedparemeter: *mut ::std::ffi::c_void,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegCreateKeyTransactedW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                reserved: u32,
                lpclass: super::super::Foundation::PWSTR,
                dwoptions: REG_OPEN_CREATE_OPTIONS,
                samdesired: REG_SAM_FLAGS,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                phkresult: *mut HKEY,
                lpdwdisposition: *mut REG_CREATE_KEY_DISPOSITION,
                htransaction: super::super::Foundation::HANDLE,
                pextendedparemeter: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegCreateKeyTransactedW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(reserved),
            lpclass.into_param().abi(),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(phkresult),
            ::std::mem::transmute(lpdwdisposition),
            htransaction.into_param().abi(),
            ::std::mem::transmute(pextendedparemeter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCreateKeyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    phkresult: *mut HKEY,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegCreateKeyW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                phkresult: *mut HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegCreateKeyW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(phkresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDeleteKeyA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDeleteKeyA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    samdesired: u32,
    reserved: u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDeleteKeyExA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                samdesired: u32,
                reserved: u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDeleteKeyExA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(reserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    samdesired: u32,
    reserved: u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDeleteKeyExW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                samdesired: u32,
                reserved: u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDeleteKeyExW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(reserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    samdesired: u32,
    reserved: u32,
    htransaction: Param4,
    pextendedparameter: *mut ::std::ffi::c_void,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDeleteKeyTransactedA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                samdesired: u32,
                reserved: u32,
                htransaction: super::super::Foundation::HANDLE,
                pextendedparameter: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDeleteKeyTransactedA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(reserved),
            htransaction.into_param().abi(),
            ::std::mem::transmute(pextendedparameter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    samdesired: u32,
    reserved: u32,
    htransaction: Param4,
    pextendedparameter: *mut ::std::ffi::c_void,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDeleteKeyTransactedW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                samdesired: u32,
                reserved: u32,
                htransaction: super::super::Foundation::HANDLE,
                pextendedparameter: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDeleteKeyTransactedW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(reserved),
            htransaction.into_param().abi(),
            ::std::mem::transmute(pextendedparameter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyValueA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    lpvaluename: Param2,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDeleteKeyValueA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                lpvaluename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDeleteKeyValueA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            lpvaluename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyValueW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    lpvaluename: Param2,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDeleteKeyValueW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                lpvaluename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDeleteKeyValueW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            lpvaluename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDeleteKeyW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDeleteKeyW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteTreeA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDeleteTreeA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDeleteTreeA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteTreeW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDeleteTreeW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDeleteTreeW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteValueA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpvaluename: Param1,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDeleteValueA(
                hkey: HKEY,
                lpvaluename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDeleteValueA(
            hkey.into_param().abi(),
            lpvaluename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteValueW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpvaluename: Param1,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDeleteValueW(
                hkey: HKEY,
                lpvaluename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDeleteValueW(
            hkey.into_param().abi(),
            lpvaluename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDisablePredefinedCache() -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDisablePredefinedCache() -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDisablePredefinedCache())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDisablePredefinedCacheEx() -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDisablePredefinedCacheEx() -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegDisablePredefinedCacheEx())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegDisableReflectionKey<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hbase: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegDisableReflectionKey(hbase: HKEY) -> i32;
        }
        ::std::mem::transmute(RegDisableReflectionKey(hbase.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegEnableReflectionKey<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hbase: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegEnableReflectionKey(hbase: HKEY) -> i32;
        }
        ::std::mem::transmute(RegEnableReflectionKey(hbase.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyA<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
    dwindex: u32,
    lpname: super::super::Foundation::PSTR,
    cchname: u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegEnumKeyA(
                hkey: HKEY,
                dwindex: u32,
                lpname: super::super::Foundation::PSTR,
                cchname: u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegEnumKeyA(
            hkey.into_param().abi(),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(lpname),
            ::std::mem::transmute(cchname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyExA<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
    dwindex: u32,
    lpname: super::super::Foundation::PSTR,
    lpcchname: *mut u32,
    lpreserved: *mut u32,
    lpclass: super::super::Foundation::PSTR,
    lpcchclass: *mut u32,
    lpftlastwritetime: *mut super::super::Foundation::FILETIME,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegEnumKeyExA(
                hkey: HKEY,
                dwindex: u32,
                lpname: super::super::Foundation::PSTR,
                lpcchname: *mut u32,
                lpreserved: *mut u32,
                lpclass: super::super::Foundation::PSTR,
                lpcchclass: *mut u32,
                lpftlastwritetime: *mut super::super::Foundation::FILETIME,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegEnumKeyExA(
            hkey.into_param().abi(),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(lpname),
            ::std::mem::transmute(lpcchname),
            ::std::mem::transmute(lpreserved),
            ::std::mem::transmute(lpclass),
            ::std::mem::transmute(lpcchclass),
            ::std::mem::transmute(lpftlastwritetime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyExW<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
    dwindex: u32,
    lpname: super::super::Foundation::PWSTR,
    lpcchname: *mut u32,
    lpreserved: *mut u32,
    lpclass: super::super::Foundation::PWSTR,
    lpcchclass: *mut u32,
    lpftlastwritetime: *mut super::super::Foundation::FILETIME,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegEnumKeyExW(
                hkey: HKEY,
                dwindex: u32,
                lpname: super::super::Foundation::PWSTR,
                lpcchname: *mut u32,
                lpreserved: *mut u32,
                lpclass: super::super::Foundation::PWSTR,
                lpcchclass: *mut u32,
                lpftlastwritetime: *mut super::super::Foundation::FILETIME,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegEnumKeyExW(
            hkey.into_param().abi(),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(lpname),
            ::std::mem::transmute(lpcchname),
            ::std::mem::transmute(lpreserved),
            ::std::mem::transmute(lpclass),
            ::std::mem::transmute(lpcchclass),
            ::std::mem::transmute(lpftlastwritetime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyW<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
    dwindex: u32,
    lpname: super::super::Foundation::PWSTR,
    cchname: u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegEnumKeyW(
                hkey: HKEY,
                dwindex: u32,
                lpname: super::super::Foundation::PWSTR,
                cchname: u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegEnumKeyW(
            hkey.into_param().abi(),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(lpname),
            ::std::mem::transmute(cchname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumValueA<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
    dwindex: u32,
    lpvaluename: super::super::Foundation::PSTR,
    lpcchvaluename: *mut u32,
    lpreserved: *mut u32,
    lptype: *mut u32,
    lpdata: *mut u8,
    lpcbdata: *mut u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegEnumValueA(
                hkey: HKEY,
                dwindex: u32,
                lpvaluename: super::super::Foundation::PSTR,
                lpcchvaluename: *mut u32,
                lpreserved: *mut u32,
                lptype: *mut u32,
                lpdata: *mut u8,
                lpcbdata: *mut u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegEnumValueA(
            hkey.into_param().abi(),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(lpvaluename),
            ::std::mem::transmute(lpcchvaluename),
            ::std::mem::transmute(lpreserved),
            ::std::mem::transmute(lptype),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(lpcbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumValueW<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
    dwindex: u32,
    lpvaluename: super::super::Foundation::PWSTR,
    lpcchvaluename: *mut u32,
    lpreserved: *mut u32,
    lptype: *mut u32,
    lpdata: *mut u8,
    lpcbdata: *mut u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegEnumValueW(
                hkey: HKEY,
                dwindex: u32,
                lpvaluename: super::super::Foundation::PWSTR,
                lpcchvaluename: *mut u32,
                lpreserved: *mut u32,
                lptype: *mut u32,
                lpdata: *mut u8,
                lpcbdata: *mut u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegEnumValueW(
            hkey.into_param().abi(),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(lpvaluename),
            ::std::mem::transmute(lpcchvaluename),
            ::std::mem::transmute(lpreserved),
            ::std::mem::transmute(lptype),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(lpcbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegFlushKey<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegFlushKey(hkey: HKEY) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegFlushKey(hkey.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegGetKeySecurity<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
    securityinformation: u32,
    psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
    lpcbsecuritydescriptor: *mut u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegGetKeySecurity(
                hkey: HKEY,
                securityinformation: u32,
                psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
                lpcbsecuritydescriptor: *mut u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegGetKeySecurity(
            hkey.into_param().abi(),
            ::std::mem::transmute(securityinformation),
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(lpcbsecuritydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegGetValueA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    lpvalue: Param2,
    dwflags: RRF_RT,
    pdwtype: *mut u32,
    pvdata: *mut ::std::ffi::c_void,
    pcbdata: *mut u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegGetValueA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                lpvalue: super::super::Foundation::PSTR,
                dwflags: RRF_RT,
                pdwtype: *mut u32,
                pvdata: *mut ::std::ffi::c_void,
                pcbdata: *mut u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegGetValueA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            lpvalue.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pdwtype),
            ::std::mem::transmute(pvdata),
            ::std::mem::transmute(pcbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegGetValueW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    lpvalue: Param2,
    dwflags: RRF_RT,
    pdwtype: *mut u32,
    pvdata: *mut ::std::ffi::c_void,
    pcbdata: *mut u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegGetValueW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                lpvalue: super::super::Foundation::PWSTR,
                dwflags: RRF_RT,
                pdwtype: *mut u32,
                pvdata: *mut ::std::ffi::c_void,
                pcbdata: *mut u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegGetValueW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            lpvalue.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pdwtype),
            ::std::mem::transmute(pvdata),
            ::std::mem::transmute(pcbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadAppKeyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpfile: Param0,
    phkresult: *mut HKEY,
    samdesired: u32,
    dwoptions: u32,
    reserved: u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegLoadAppKeyA(
                lpfile: super::super::Foundation::PSTR,
                phkresult: *mut HKEY,
                samdesired: u32,
                dwoptions: u32,
                reserved: u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegLoadAppKeyA(
            lpfile.into_param().abi(),
            ::std::mem::transmute(phkresult),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(reserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadAppKeyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpfile: Param0,
    phkresult: *mut HKEY,
    samdesired: u32,
    dwoptions: u32,
    reserved: u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegLoadAppKeyW(
                lpfile: super::super::Foundation::PWSTR,
                phkresult: *mut HKEY,
                samdesired: u32,
                dwoptions: u32,
                reserved: u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegLoadAppKeyW(
            lpfile.into_param().abi(),
            ::std::mem::transmute(phkresult),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(reserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadKeyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    lpfile: Param2,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegLoadKeyA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                lpfile: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegLoadKeyA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            lpfile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadKeyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    lpfile: Param2,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegLoadKeyW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                lpfile: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegLoadKeyW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            lpfile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadMUIStringA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    pszvalue: Param1,
    pszoutbuf: super::super::Foundation::PSTR,
    cboutbuf: u32,
    pcbdata: *mut u32,
    flags: u32,
    pszdirectory: Param6,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegLoadMUIStringA(
                hkey: HKEY,
                pszvalue: super::super::Foundation::PSTR,
                pszoutbuf: super::super::Foundation::PSTR,
                cboutbuf: u32,
                pcbdata: *mut u32,
                flags: u32,
                pszdirectory: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegLoadMUIStringA(
            hkey.into_param().abi(),
            pszvalue.into_param().abi(),
            ::std::mem::transmute(pszoutbuf),
            ::std::mem::transmute(cboutbuf),
            ::std::mem::transmute(pcbdata),
            ::std::mem::transmute(flags),
            pszdirectory.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadMUIStringW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    pszvalue: Param1,
    pszoutbuf: super::super::Foundation::PWSTR,
    cboutbuf: u32,
    pcbdata: *mut u32,
    flags: u32,
    pszdirectory: Param6,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegLoadMUIStringW(
                hkey: HKEY,
                pszvalue: super::super::Foundation::PWSTR,
                pszoutbuf: super::super::Foundation::PWSTR,
                cboutbuf: u32,
                pcbdata: *mut u32,
                flags: u32,
                pszdirectory: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegLoadMUIStringW(
            hkey.into_param().abi(),
            pszvalue.into_param().abi(),
            ::std::mem::transmute(pszoutbuf),
            ::std::mem::transmute(cboutbuf),
            ::std::mem::transmute(pcbdata),
            ::std::mem::transmute(flags),
            pszdirectory.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegNotifyChangeKeyValue<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hkey: Param0,
    bwatchsubtree: Param1,
    dwnotifyfilter: REG_NOTIFY_FILTER,
    hevent: Param3,
    fasynchronous: Param4,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegNotifyChangeKeyValue(
                hkey: HKEY,
                bwatchsubtree: super::super::Foundation::BOOL,
                dwnotifyfilter: REG_NOTIFY_FILTER,
                hevent: super::super::Foundation::HANDLE,
                fasynchronous: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegNotifyChangeKeyValue(
            hkey.into_param().abi(),
            bwatchsubtree.into_param().abi(),
            ::std::mem::transmute(dwnotifyfilter),
            hevent.into_param().abi(),
            fasynchronous.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenCurrentUser(
    samdesired: u32,
    phkresult: *mut HKEY,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegOpenCurrentUser(
                samdesired: u32,
                phkresult: *mut HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegOpenCurrentUser(
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(phkresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    phkresult: *mut HKEY,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegOpenKeyA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                phkresult: *mut HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegOpenKeyA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(phkresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    uloptions: u32,
    samdesired: REG_SAM_FLAGS,
    phkresult: *mut HKEY,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegOpenKeyExA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                uloptions: u32,
                samdesired: REG_SAM_FLAGS,
                phkresult: *mut HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegOpenKeyExA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(uloptions),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(phkresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    uloptions: u32,
    samdesired: REG_SAM_FLAGS,
    phkresult: *mut HKEY,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegOpenKeyExW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                uloptions: u32,
                samdesired: REG_SAM_FLAGS,
                phkresult: *mut HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegOpenKeyExW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(uloptions),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(phkresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyTransactedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    uloptions: u32,
    samdesired: REG_SAM_FLAGS,
    phkresult: *mut HKEY,
    htransaction: Param5,
    pextendedparemeter: *mut ::std::ffi::c_void,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegOpenKeyTransactedA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                uloptions: u32,
                samdesired: REG_SAM_FLAGS,
                phkresult: *mut HKEY,
                htransaction: super::super::Foundation::HANDLE,
                pextendedparemeter: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegOpenKeyTransactedA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(uloptions),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(phkresult),
            htransaction.into_param().abi(),
            ::std::mem::transmute(pextendedparemeter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyTransactedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    uloptions: u32,
    samdesired: REG_SAM_FLAGS,
    phkresult: *mut HKEY,
    htransaction: Param5,
    pextendedparemeter: *mut ::std::ffi::c_void,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegOpenKeyTransactedW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                uloptions: u32,
                samdesired: REG_SAM_FLAGS,
                phkresult: *mut HKEY,
                htransaction: super::super::Foundation::HANDLE,
                pextendedparemeter: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegOpenKeyTransactedW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(uloptions),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(phkresult),
            htransaction.into_param().abi(),
            ::std::mem::transmute(pextendedparemeter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    phkresult: *mut HKEY,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegOpenKeyW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                phkresult: *mut HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegOpenKeyW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(phkresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenUserClassesRoot<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    htoken: Param0,
    dwoptions: u32,
    samdesired: u32,
    phkresult: *mut HKEY,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegOpenUserClassesRoot(
                htoken: super::super::Foundation::HANDLE,
                dwoptions: u32,
                samdesired: u32,
                phkresult: *mut HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegOpenUserClassesRoot(
            htoken.into_param().abi(),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(samdesired),
            ::std::mem::transmute(phkresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOverridePredefKey<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, HKEY>,
>(
    hkey: Param0,
    hnewhkey: Param1,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegOverridePredefKey(
                hkey: HKEY,
                hnewhkey: HKEY,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegOverridePredefKey(
            hkey.into_param().abi(),
            hnewhkey.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryInfoKeyA<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
    lpclass: super::super::Foundation::PSTR,
    lpcchclass: *mut u32,
    lpreserved: *mut u32,
    lpcsubkeys: *mut u32,
    lpcbmaxsubkeylen: *mut u32,
    lpcbmaxclasslen: *mut u32,
    lpcvalues: *mut u32,
    lpcbmaxvaluenamelen: *mut u32,
    lpcbmaxvaluelen: *mut u32,
    lpcbsecuritydescriptor: *mut u32,
    lpftlastwritetime: *mut super::super::Foundation::FILETIME,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegQueryInfoKeyA(
                hkey: HKEY,
                lpclass: super::super::Foundation::PSTR,
                lpcchclass: *mut u32,
                lpreserved: *mut u32,
                lpcsubkeys: *mut u32,
                lpcbmaxsubkeylen: *mut u32,
                lpcbmaxclasslen: *mut u32,
                lpcvalues: *mut u32,
                lpcbmaxvaluenamelen: *mut u32,
                lpcbmaxvaluelen: *mut u32,
                lpcbsecuritydescriptor: *mut u32,
                lpftlastwritetime: *mut super::super::Foundation::FILETIME,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegQueryInfoKeyA(
            hkey.into_param().abi(),
            ::std::mem::transmute(lpclass),
            ::std::mem::transmute(lpcchclass),
            ::std::mem::transmute(lpreserved),
            ::std::mem::transmute(lpcsubkeys),
            ::std::mem::transmute(lpcbmaxsubkeylen),
            ::std::mem::transmute(lpcbmaxclasslen),
            ::std::mem::transmute(lpcvalues),
            ::std::mem::transmute(lpcbmaxvaluenamelen),
            ::std::mem::transmute(lpcbmaxvaluelen),
            ::std::mem::transmute(lpcbsecuritydescriptor),
            ::std::mem::transmute(lpftlastwritetime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryInfoKeyW<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
    lpclass: super::super::Foundation::PWSTR,
    lpcchclass: *mut u32,
    lpreserved: *mut u32,
    lpcsubkeys: *mut u32,
    lpcbmaxsubkeylen: *mut u32,
    lpcbmaxclasslen: *mut u32,
    lpcvalues: *mut u32,
    lpcbmaxvaluenamelen: *mut u32,
    lpcbmaxvaluelen: *mut u32,
    lpcbsecuritydescriptor: *mut u32,
    lpftlastwritetime: *mut super::super::Foundation::FILETIME,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegQueryInfoKeyW(
                hkey: HKEY,
                lpclass: super::super::Foundation::PWSTR,
                lpcchclass: *mut u32,
                lpreserved: *mut u32,
                lpcsubkeys: *mut u32,
                lpcbmaxsubkeylen: *mut u32,
                lpcbmaxclasslen: *mut u32,
                lpcvalues: *mut u32,
                lpcbmaxvaluenamelen: *mut u32,
                lpcbmaxvaluelen: *mut u32,
                lpcbsecuritydescriptor: *mut u32,
                lpftlastwritetime: *mut super::super::Foundation::FILETIME,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegQueryInfoKeyW(
            hkey.into_param().abi(),
            ::std::mem::transmute(lpclass),
            ::std::mem::transmute(lpcchclass),
            ::std::mem::transmute(lpreserved),
            ::std::mem::transmute(lpcsubkeys),
            ::std::mem::transmute(lpcbmaxsubkeylen),
            ::std::mem::transmute(lpcbmaxclasslen),
            ::std::mem::transmute(lpcvalues),
            ::std::mem::transmute(lpcbmaxvaluenamelen),
            ::std::mem::transmute(lpcbmaxvaluelen),
            ::std::mem::transmute(lpcbsecuritydescriptor),
            ::std::mem::transmute(lpftlastwritetime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryMultipleValuesA<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
    val_list: *mut VALENTA,
    num_vals: u32,
    lpvaluebuf: super::super::Foundation::PSTR,
    ldwtotsize: *mut u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegQueryMultipleValuesA(
                hkey: HKEY,
                val_list: *mut VALENTA,
                num_vals: u32,
                lpvaluebuf: super::super::Foundation::PSTR,
                ldwtotsize: *mut u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegQueryMultipleValuesA(
            hkey.into_param().abi(),
            ::std::mem::transmute(val_list),
            ::std::mem::transmute(num_vals),
            ::std::mem::transmute(lpvaluebuf),
            ::std::mem::transmute(ldwtotsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryMultipleValuesW<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
    val_list: *mut VALENTW,
    num_vals: u32,
    lpvaluebuf: super::super::Foundation::PWSTR,
    ldwtotsize: *mut u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegQueryMultipleValuesW(
                hkey: HKEY,
                val_list: *mut VALENTW,
                num_vals: u32,
                lpvaluebuf: super::super::Foundation::PWSTR,
                ldwtotsize: *mut u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegQueryMultipleValuesW(
            hkey.into_param().abi(),
            ::std::mem::transmute(val_list),
            ::std::mem::transmute(num_vals),
            ::std::mem::transmute(lpvaluebuf),
            ::std::mem::transmute(ldwtotsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryReflectionKey<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hbase: Param0,
    bisreflectiondisabled: *mut super::super::Foundation::BOOL,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegQueryReflectionKey(
                hbase: HKEY,
                bisreflectiondisabled: *mut super::super::Foundation::BOOL,
            ) -> i32;
        }
        ::std::mem::transmute(RegQueryReflectionKey(
            hbase.into_param().abi(),
            ::std::mem::transmute(bisreflectiondisabled),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    lpdata: super::super::Foundation::PSTR,
    lpcbdata: *mut i32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegQueryValueA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                lpdata: super::super::Foundation::PSTR,
                lpcbdata: *mut i32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegQueryValueA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(lpcbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpvaluename: Param1,
    lpreserved: *mut u32,
    lptype: *mut REG_VALUE_TYPE,
    lpdata: *mut u8,
    lpcbdata: *mut u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegQueryValueExA(
                hkey: HKEY,
                lpvaluename: super::super::Foundation::PSTR,
                lpreserved: *mut u32,
                lptype: *mut REG_VALUE_TYPE,
                lpdata: *mut u8,
                lpcbdata: *mut u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegQueryValueExA(
            hkey.into_param().abi(),
            lpvaluename.into_param().abi(),
            ::std::mem::transmute(lpreserved),
            ::std::mem::transmute(lptype),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(lpcbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpvaluename: Param1,
    lpreserved: *mut u32,
    lptype: *mut REG_VALUE_TYPE,
    lpdata: *mut u8,
    lpcbdata: *mut u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegQueryValueExW(
                hkey: HKEY,
                lpvaluename: super::super::Foundation::PWSTR,
                lpreserved: *mut u32,
                lptype: *mut REG_VALUE_TYPE,
                lpdata: *mut u8,
                lpcbdata: *mut u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegQueryValueExW(
            hkey.into_param().abi(),
            lpvaluename.into_param().abi(),
            ::std::mem::transmute(lpreserved),
            ::std::mem::transmute(lptype),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(lpcbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    lpdata: super::super::Foundation::PWSTR,
    lpcbdata: *mut i32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegQueryValueW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                lpdata: super::super::Foundation::PWSTR,
                lpcbdata: *mut i32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegQueryValueW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(lpcbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegRenameKey<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkeyname: Param1,
    lpnewkeyname: Param2,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegRenameKey(
                hkey: HKEY,
                lpsubkeyname: super::super::Foundation::PWSTR,
                lpnewkeyname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegRenameKey(
            hkey.into_param().abi(),
            lpsubkeyname.into_param().abi(),
            lpnewkeyname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegReplaceKeyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    lpnewfile: Param2,
    lpoldfile: Param3,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegReplaceKeyA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                lpnewfile: super::super::Foundation::PSTR,
                lpoldfile: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegReplaceKeyA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            lpnewfile.into_param().abi(),
            lpoldfile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegReplaceKeyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    lpnewfile: Param2,
    lpoldfile: Param3,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegReplaceKeyW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                lpnewfile: super::super::Foundation::PWSTR,
                lpoldfile: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegReplaceKeyW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            lpnewfile.into_param().abi(),
            lpoldfile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegRestoreKeyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpfile: Param1,
    dwflags: REG_RESTORE_KEY_FLAGS,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegRestoreKeyA(
                hkey: HKEY,
                lpfile: super::super::Foundation::PSTR,
                dwflags: REG_RESTORE_KEY_FLAGS,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegRestoreKeyA(
            hkey.into_param().abi(),
            lpfile.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegRestoreKeyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpfile: Param1,
    dwflags: REG_RESTORE_KEY_FLAGS,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegRestoreKeyW(
                hkey: HKEY,
                lpfile: super::super::Foundation::PWSTR,
                dwflags: REG_RESTORE_KEY_FLAGS,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegRestoreKeyW(
            hkey.into_param().abi(),
            lpfile.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpfile: Param1,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSaveKeyA(
                hkey: HKEY,
                lpfile: super::super::Foundation::PSTR,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegSaveKeyA(
            hkey.into_param().abi(),
            lpfile.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpfile: Param1,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    flags: REG_SAVE_FORMAT,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSaveKeyExA(
                hkey: HKEY,
                lpfile: super::super::Foundation::PSTR,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                flags: REG_SAVE_FORMAT,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegSaveKeyExA(
            hkey.into_param().abi(),
            lpfile.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpfile: Param1,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    flags: REG_SAVE_FORMAT,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSaveKeyExW(
                hkey: HKEY,
                lpfile: super::super::Foundation::PWSTR,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                flags: REG_SAVE_FORMAT,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegSaveKeyExW(
            hkey.into_param().abi(),
            lpfile.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpfile: Param1,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSaveKeyW(
                hkey: HKEY,
                lpfile: super::super::Foundation::PWSTR,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegSaveKeyW(
            hkey.into_param().abi(),
            lpfile.into_param().abi(),
            ::std::mem::transmute(lpsecurityattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSetKeySecurity<'a, Param0: ::windows::runtime::IntoParam<'a, HKEY>>(
    hkey: Param0,
    securityinformation: u32,
    psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSetKeySecurity(
                hkey: HKEY,
                securityinformation: u32,
                psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegSetKeySecurity(
            hkey.into_param().abi(),
            ::std::mem::transmute(securityinformation),
            ::std::mem::transmute(psecuritydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetKeyValueA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    lpvaluename: Param2,
    dwtype: u32,
    lpdata: *const ::std::ffi::c_void,
    cbdata: u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSetKeyValueA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                lpvaluename: super::super::Foundation::PSTR,
                dwtype: u32,
                lpdata: *const ::std::ffi::c_void,
                cbdata: u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegSetKeyValueA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            lpvaluename.into_param().abi(),
            ::std::mem::transmute(dwtype),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(cbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetKeyValueW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    lpvaluename: Param2,
    dwtype: u32,
    lpdata: *const ::std::ffi::c_void,
    cbdata: u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSetKeyValueW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                lpvaluename: super::super::Foundation::PWSTR,
                dwtype: u32,
                lpdata: *const ::std::ffi::c_void,
                cbdata: u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegSetKeyValueW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            lpvaluename.into_param().abi(),
            ::std::mem::transmute(dwtype),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(cbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    dwtype: REG_VALUE_TYPE,
    lpdata: Param3,
    cbdata: u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSetValueA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
                dwtype: REG_VALUE_TYPE,
                lpdata: super::super::Foundation::PSTR,
                cbdata: u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegSetValueA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(dwtype),
            lpdata.into_param().abi(),
            ::std::mem::transmute(cbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpvaluename: Param1,
    reserved: u32,
    dwtype: REG_VALUE_TYPE,
    lpdata: *const u8,
    cbdata: u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSetValueExA(
                hkey: HKEY,
                lpvaluename: super::super::Foundation::PSTR,
                reserved: u32,
                dwtype: REG_VALUE_TYPE,
                lpdata: *const u8,
                cbdata: u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegSetValueExA(
            hkey.into_param().abi(),
            lpvaluename.into_param().abi(),
            ::std::mem::transmute(reserved),
            ::std::mem::transmute(dwtype),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(cbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpvaluename: Param1,
    reserved: u32,
    dwtype: REG_VALUE_TYPE,
    lpdata: *const u8,
    cbdata: u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSetValueExW(
                hkey: HKEY,
                lpvaluename: super::super::Foundation::PWSTR,
                reserved: u32,
                dwtype: REG_VALUE_TYPE,
                lpdata: *const u8,
                cbdata: u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegSetValueExW(
            hkey.into_param().abi(),
            lpvaluename.into_param().abi(),
            ::std::mem::transmute(reserved),
            ::std::mem::transmute(dwtype),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(cbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
    dwtype: REG_VALUE_TYPE,
    lpdata: Param3,
    cbdata: u32,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSetValueW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
                dwtype: REG_VALUE_TYPE,
                lpdata: super::super::Foundation::PWSTR,
                cbdata: u32,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegSetValueW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
            ::std::mem::transmute(dwtype),
            lpdata.into_param().abi(),
            ::std::mem::transmute(cbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegUnLoadKeyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegUnLoadKeyA(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegUnLoadKeyA(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegUnLoadKeyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hkey: Param0,
    lpsubkey: Param1,
) -> super::super::Foundation::LSTATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegUnLoadKeyW(
                hkey: HKEY,
                lpsubkey: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::LSTATUS;
        }
        ::std::mem::transmute(RegUnLoadKeyW(
            hkey.into_param().abi(),
            lpsubkey.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VALENTA {
    pub ve_valuename: super::super::Foundation::PSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: REG_VALUE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl VALENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VALENTA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VALENTA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VALENTA")
            .field("ve_valuename", &self.ve_valuename)
            .field("ve_valuelen", &self.ve_valuelen)
            .field("ve_valueptr", &self.ve_valueptr)
            .field("ve_type", &self.ve_type)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VALENTA {
    fn eq(&self, other: &Self) -> bool {
        self.ve_valuename == other.ve_valuename
            && self.ve_valuelen == other.ve_valuelen
            && self.ve_valueptr == other.ve_valueptr
            && self.ve_type == other.ve_type
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VALENTA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VALENTA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VALENTW {
    pub ve_valuename: super::super::Foundation::PWSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: REG_VALUE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl VALENTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VALENTW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VALENTW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VALENTW")
            .field("ve_valuename", &self.ve_valuename)
            .field("ve_valuelen", &self.ve_valuelen)
            .field("ve_valueptr", &self.ve_valueptr)
            .field("ve_type", &self.ve_type)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VALENTW {
    fn eq(&self, other: &Self) -> bool {
        self.ve_valuename == other.ve_valuename
            && self.ve_valuelen == other.ve_valuelen
            && self.ve_valueptr == other.ve_valueptr
            && self.ve_type == other.ve_type
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VALENTW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VALENTW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VPDF_DISABLEPWRMGMT: u32 = 1u32;
pub const VPDF_DISABLEPWRSTATUSPOLL: u32 = 8u32;
pub const VPDF_DISABLERINGRESUME: u32 = 16u32;
pub const VPDF_FORCEAPM10MODE: u32 = 2u32;
pub const VPDF_SHOWMULTIBATT: u32 = 32u32;
pub const VPDF_SKIPINTELSLCHECK: u32 = 4u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct provider_info {
    pub pi_R0_1val: ::std::option::Option<PQUERYHANDLER>,
    pub pi_R0_allvals: ::std::option::Option<PQUERYHANDLER>,
    pub pi_R3_1val: ::std::option::Option<PQUERYHANDLER>,
    pub pi_R3_allvals: ::std::option::Option<PQUERYHANDLER>,
    pub pi_flags: u32,
    pub pi_key_context: *mut ::std::ffi::c_void,
}
impl provider_info {}
impl ::std::default::Default for provider_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for provider_info {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("provider_info")
            .field("pi_flags", &self.pi_flags)
            .field("pi_key_context", &self.pi_key_context)
            .finish()
    }
}
impl ::std::cmp::PartialEq for provider_info {
    fn eq(&self, other: &Self) -> bool {
        self.pi_R0_1val.map(|f| f as usize) == other.pi_R0_1val.map(|f| f as usize)
            && self.pi_R0_allvals.map(|f| f as usize) == other.pi_R0_allvals.map(|f| f as usize)
            && self.pi_R3_1val.map(|f| f as usize) == other.pi_R3_1val.map(|f| f as usize)
            && self.pi_R3_allvals.map(|f| f as usize) == other.pi_R3_allvals.map(|f| f as usize)
            && self.pi_flags == other.pi_flags
            && self.pi_key_context == other.pi_key_context
    }
}
impl ::std::cmp::Eq for provider_info {}
unsafe impl ::windows::runtime::Abi for provider_info {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct pvalueA {
    pub pv_valuename: super::super::Foundation::PSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut ::std::ffi::c_void,
    pub pv_type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl pvalueA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for pvalueA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for pvalueA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("pvalueA")
            .field("pv_valuename", &self.pv_valuename)
            .field("pv_valuelen", &self.pv_valuelen)
            .field("pv_value_context", &self.pv_value_context)
            .field("pv_type", &self.pv_type)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for pvalueA {
    fn eq(&self, other: &Self) -> bool {
        self.pv_valuename == other.pv_valuename
            && self.pv_valuelen == other.pv_valuelen
            && self.pv_value_context == other.pv_value_context
            && self.pv_type == other.pv_type
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for pvalueA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for pvalueA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct pvalueW {
    pub pv_valuename: super::super::Foundation::PWSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut ::std::ffi::c_void,
    pub pv_type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl pvalueW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for pvalueW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for pvalueW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("pvalueW")
            .field("pv_valuename", &self.pv_valuename)
            .field("pv_valuelen", &self.pv_valuelen)
            .field("pv_value_context", &self.pv_value_context)
            .field("pv_type", &self.pv_type)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for pvalueW {
    fn eq(&self, other: &Self) -> bool {
        self.pv_valuename == other.pv_valuename
            && self.pv_valuelen == other.pv_valuelen
            && self.pv_value_context == other.pv_value_context
            && self.pv_type == other.pv_type
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for pvalueW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for pvalueW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct val_context {
    pub valuelen: i32,
    pub value_context: *mut ::std::ffi::c_void,
    pub val_buff_ptr: *mut ::std::ffi::c_void,
}
impl val_context {}
impl ::std::default::Default for val_context {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for val_context {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("val_context")
            .field("valuelen", &self.valuelen)
            .field("value_context", &self.value_context)
            .field("val_buff_ptr", &self.val_buff_ptr)
            .finish()
    }
}
impl ::std::cmp::PartialEq for val_context {
    fn eq(&self, other: &Self) -> bool {
        self.valuelen == other.valuelen
            && self.value_context == other.value_context
            && self.val_buff_ptr == other.val_buff_ptr
    }
}
impl ::std::cmp::Eq for val_context {}
unsafe impl ::windows::runtime::Abi for val_context {
    type Abi = Self;
    type DefaultType = Self;
}

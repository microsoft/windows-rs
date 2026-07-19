#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn Netbios(pncb: *mut NCB) -> u8 {
    windows_core::link!("netapi32.dll" "system" fn Netbios(pncb : *mut NCB) -> u8);
    unsafe { Netbios(pncb as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTION_HEADER {
    pub transport_id: u32,
    pub action_code: u16,
    pub reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADAPTER_STATUS {
    pub adapter_address: [u8; 6],
    pub rev_major: u8,
    pub reserved0: u8,
    pub adapter_type: u8,
    pub rev_minor: u8,
    pub duration: u16,
    pub frmr_recv: u16,
    pub frmr_xmit: u16,
    pub iframe_recv_err: u16,
    pub xmit_aborts: u16,
    pub xmit_success: u32,
    pub recv_success: u32,
    pub iframe_xmit_err: u16,
    pub recv_buff_unavail: u16,
    pub t1_timeouts: u16,
    pub ti_timeouts: u16,
    pub reserved1: u32,
    pub free_ncbs: u16,
    pub max_cfg_ncbs: u16,
    pub max_ncbs: u16,
    pub xmit_buf_unavail: u16,
    pub max_dgram_size: u16,
    pub pending_sess: u16,
    pub max_cfg_sess: u16,
    pub max_sess: u16,
    pub max_sess_pkt_size: u16,
    pub name_count: u16,
}
impl Default for ADAPTER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ALL_TRANSPORTS: windows_core::PCSTR = windows_core::s!("M\u{0}\u{0}\u{0}");
pub const ASYNCH: u32 = 128;
pub const CALL_PENDING: u32 = 2;
pub const DEREGISTERED: u32 = 5;
pub const DUPLICATE: u32 = 6;
pub const DUPLICATE_DEREG: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FIND_NAME_BUFFER {
    pub length: u8,
    pub access_control: u8,
    pub frame_control: u8,
    pub destination_addr: [u8; 6],
    pub source_addr: [u8; 6],
    pub routing_info: [u8; 18],
}
impl Default for FIND_NAME_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FIND_NAME_HEADER {
    pub node_count: u16,
    pub reserved: u8,
    pub unique_group: u8,
}
pub const GROUP_NAME: u32 = 128;
pub const HANGUP_COMPLETE: u32 = 5;
pub const HANGUP_PENDING: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LANA_ENUM {
    pub length: u8,
    pub lana: [u8; 255],
}
impl Default for LANA_ENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LISTEN_OUTSTANDING: u32 = 1;
pub const MAX_LANA: u32 = 254;
pub const MS_NBF: windows_core::PCSTR = windows_core::s!("MNBF");
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NAME_BUFFER {
    pub name: [u8; 16],
    pub name_num: u8,
    pub name_flags: u8,
}
impl Default for NAME_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NAME_FLAGS_MASK: u32 = 135;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NCB {
    pub ncb_command: u8,
    pub ncb_retcode: u8,
    pub ncb_lsn: u8,
    pub ncb_num: u8,
    pub ncb_buffer: super::PUCHAR,
    pub ncb_length: u16,
    pub ncb_callname: [u8; 16],
    pub ncb_name: [u8; 16],
    pub ncb_rto: u8,
    pub ncb_sto: u8,
    pub ncb_post: *mut u8,
    pub ncb_lana_num: u8,
    pub ncb_cmd_cplt: u8,
    pub ncb_reserve: [u8; 10],
    pub ncb_event: super::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for NCB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NCB {
    pub ncb_command: u8,
    pub ncb_retcode: u8,
    pub ncb_lsn: u8,
    pub ncb_num: u8,
    pub ncb_buffer: super::PUCHAR,
    pub ncb_length: u16,
    pub ncb_callname: [u8; 16],
    pub ncb_name: [u8; 16],
    pub ncb_rto: u8,
    pub ncb_sto: u8,
    pub ncb_post: *mut u8,
    pub ncb_lana_num: u8,
    pub ncb_cmd_cplt: u8,
    pub ncb_reserve: [u8; 18],
    pub ncb_event: super::HANDLE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for NCB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NCBACTION: u32 = 119;
pub const NCBADDGRNAME: u32 = 54;
pub const NCBADDNAME: u32 = 48;
pub const NCBASTAT: u32 = 51;
pub const NCBCALL: u32 = 16;
pub const NCBCANCEL: u32 = 53;
pub const NCBCHAINSEND: u32 = 23;
pub const NCBCHAINSENDNA: u32 = 114;
pub const NCBDELNAME: u32 = 49;
pub const NCBDGRECV: u32 = 33;
pub const NCBDGRECVBC: u32 = 35;
pub const NCBDGSEND: u32 = 32;
pub const NCBDGSENDBC: u32 = 34;
pub const NCBENUM: u32 = 55;
pub const NCBFINDNAME: u32 = 120;
pub const NCBHANGUP: u32 = 18;
pub const NCBLANSTALERT: u32 = 115;
pub const NCBLISTEN: u32 = 17;
pub const NCBNAMSZ: u32 = 16;
pub const NCBRECV: u32 = 21;
pub const NCBRECVANY: u32 = 22;
pub const NCBRESET: u32 = 50;
pub const NCBSEND: u32 = 20;
pub const NCBSENDNA: u32 = 113;
pub const NCBSSTAT: u32 = 52;
pub const NCBTRACE: u32 = 121;
pub const NCBUNLINK: u32 = 112;
pub const NRC_ACTSES: u32 = 15;
pub const NRC_BADDR: u32 = 7;
pub const NRC_BRIDGE: u32 = 35;
pub const NRC_BUFLEN: u32 = 1;
pub const NRC_CANCEL: u32 = 38;
pub const NRC_CANOCCR: u32 = 36;
pub const NRC_CMDCAN: u32 = 11;
pub const NRC_CMDTMO: u32 = 5;
pub const NRC_DUPENV: u32 = 48;
pub const NRC_DUPNAME: u32 = 13;
pub const NRC_ENVNOTDEF: u32 = 52;
pub const NRC_GOODRET: u32 = 0;
pub const NRC_IFBUSY: u32 = 33;
pub const NRC_ILLCMD: u32 = 3;
pub const NRC_ILLNN: u32 = 19;
pub const NRC_INCOMP: u32 = 6;
pub const NRC_INUSE: u32 = 22;
pub const NRC_INVADDRESS: u32 = 57;
pub const NRC_INVDDID: u32 = 59;
pub const NRC_LOCKFAIL: u32 = 60;
pub const NRC_LOCTFUL: u32 = 17;
pub const NRC_MAXAPPS: u32 = 54;
pub const NRC_NAMCONF: u32 = 25;
pub const NRC_NAMERR: u32 = 23;
pub const NRC_NAMTFUL: u32 = 14;
pub const NRC_NOCALL: u32 = 20;
pub const NRC_NORES: u32 = 9;
pub const NRC_NORESOURCES: u32 = 56;
pub const NRC_NOSAPS: u32 = 55;
pub const NRC_NOWILD: u32 = 21;
pub const NRC_OPENERR: u32 = 63;
pub const NRC_OSRESNOTAV: u32 = 53;
pub const NRC_PENDING: u32 = 255;
pub const NRC_REMTFUL: u32 = 18;
pub const NRC_SABORT: u32 = 24;
pub const NRC_SCLOSED: u32 = 10;
pub const NRC_SNUMOUT: u32 = 8;
pub const NRC_SYSTEM: u32 = 64;
pub const NRC_TOOMANY: u32 = 34;
pub type PACTION_HEADER = *mut ACTION_HEADER;
pub type PADAPTER_STATUS = *mut ADAPTER_STATUS;
pub type PFIND_NAME_BUFFER = *mut FIND_NAME_BUFFER;
pub type PFIND_NAME_HEADER = *mut FIND_NAME_HEADER;
pub type PLANA_ENUM = *mut LANA_ENUM;
pub type PNAME_BUFFER = *mut NAME_BUFFER;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PNCB = *mut NCB;
pub type PSESSION_BUFFER = *mut SESSION_BUFFER;
pub type PSESSION_HEADER = *mut SESSION_HEADER;
pub const REGISTERED: u32 = 4;
pub const REGISTERING: u32 = 0;
pub const SESSION_ABORTED: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SESSION_BUFFER {
    pub lsn: u8,
    pub state: u8,
    pub local_name: [u8; 16],
    pub remote_name: [u8; 16],
    pub rcvs_outstanding: u8,
    pub sends_outstanding: u8,
}
impl Default for SESSION_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SESSION_ESTABLISHED: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SESSION_HEADER {
    pub sess_name: u8,
    pub num_sess: u8,
    pub rcv_dg_outstanding: u8,
    pub rcv_any_outstanding: u8,
}
pub const UNIQUE_NAME: u32 = 0;

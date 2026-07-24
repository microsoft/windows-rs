#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("netapi32.dll" "system" fn Netbios(pncb : *mut NCB) -> u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ACTION_HEADER {
    pub transport_id: u32,
    pub action_code: u16,
    pub reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
pub const ALL_TRANSPORTS: windows_sys::core::PCSTR = windows_sys::core::s!("M\u{0}\u{0}\u{0}");
pub const ASYNCH: i32 = 128;
pub const CALL_PENDING: i32 = 2;
pub const DEREGISTERED: i32 = 5;
pub const DUPLICATE: i32 = 6;
pub const DUPLICATE_DEREG: i32 = 7;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct FIND_NAME_HEADER {
    pub node_count: u16,
    pub reserved: u8,
    pub unique_group: u8,
}
pub const GROUP_NAME: i32 = 128;
pub const HANGUP_COMPLETE: i32 = 5;
pub const HANGUP_PENDING: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LANA_ENUM {
    pub length: u8,
    pub lana: [u8; 255],
}
impl Default for LANA_ENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LISTEN_OUTSTANDING: i32 = 1;
pub const MAX_LANA: i32 = 254;
pub const MS_NBF: windows_sys::core::PCSTR = windows_sys::core::s!("MNBF");
#[repr(C)]
#[derive(Clone, Copy)]
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
pub const NAME_FLAGS_MASK: i32 = 135;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
pub const NCBACTION: i32 = 119;
pub const NCBADDGRNAME: i32 = 54;
pub const NCBADDNAME: i32 = 48;
pub const NCBASTAT: i32 = 51;
pub const NCBCALL: i32 = 16;
pub const NCBCANCEL: i32 = 53;
pub const NCBCHAINSEND: i32 = 23;
pub const NCBCHAINSENDNA: i32 = 114;
pub const NCBDELNAME: i32 = 49;
pub const NCBDGRECV: i32 = 33;
pub const NCBDGRECVBC: i32 = 35;
pub const NCBDGSEND: i32 = 32;
pub const NCBDGSENDBC: i32 = 34;
pub const NCBENUM: i32 = 55;
pub const NCBFINDNAME: i32 = 120;
pub const NCBHANGUP: i32 = 18;
pub const NCBLANSTALERT: i32 = 115;
pub const NCBLISTEN: i32 = 17;
pub const NCBNAMSZ: i32 = 16;
pub const NCBRECV: i32 = 21;
pub const NCBRECVANY: i32 = 22;
pub const NCBRESET: i32 = 50;
pub const NCBSEND: i32 = 20;
pub const NCBSENDNA: i32 = 113;
pub const NCBSSTAT: i32 = 52;
pub const NCBTRACE: i32 = 121;
pub const NCBUNLINK: i32 = 112;
pub const NRC_ACTSES: i32 = 15;
pub const NRC_BADDR: i32 = 7;
pub const NRC_BRIDGE: i32 = 35;
pub const NRC_BUFLEN: i32 = 1;
pub const NRC_CANCEL: i32 = 38;
pub const NRC_CANOCCR: i32 = 36;
pub const NRC_CMDCAN: i32 = 11;
pub const NRC_CMDTMO: i32 = 5;
pub const NRC_DUPENV: i32 = 48;
pub const NRC_DUPNAME: i32 = 13;
pub const NRC_ENVNOTDEF: i32 = 52;
pub const NRC_GOODRET: i32 = 0;
pub const NRC_IFBUSY: i32 = 33;
pub const NRC_ILLCMD: i32 = 3;
pub const NRC_ILLNN: i32 = 19;
pub const NRC_INCOMP: i32 = 6;
pub const NRC_INUSE: i32 = 22;
pub const NRC_INVADDRESS: i32 = 57;
pub const NRC_INVDDID: i32 = 59;
pub const NRC_LOCKFAIL: i32 = 60;
pub const NRC_LOCTFUL: i32 = 17;
pub const NRC_MAXAPPS: i32 = 54;
pub const NRC_NAMCONF: i32 = 25;
pub const NRC_NAMERR: i32 = 23;
pub const NRC_NAMTFUL: i32 = 14;
pub const NRC_NOCALL: i32 = 20;
pub const NRC_NORES: i32 = 9;
pub const NRC_NORESOURCES: i32 = 56;
pub const NRC_NOSAPS: i32 = 55;
pub const NRC_NOWILD: i32 = 21;
pub const NRC_OPENERR: i32 = 63;
pub const NRC_OSRESNOTAV: i32 = 53;
pub const NRC_PENDING: i32 = 255;
pub const NRC_REMTFUL: i32 = 18;
pub const NRC_SABORT: i32 = 24;
pub const NRC_SCLOSED: i32 = 10;
pub const NRC_SNUMOUT: i32 = 8;
pub const NRC_SYSTEM: i32 = 64;
pub const NRC_TOOMANY: i32 = 34;
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
pub const REGISTERED: i32 = 4;
pub const REGISTERING: i32 = 0;
pub const SESSION_ABORTED: i32 = 6;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub const SESSION_ESTABLISHED: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SESSION_HEADER {
    pub sess_name: u8,
    pub num_sess: u8,
    pub rcv_dg_outstanding: u8,
    pub rcv_any_outstanding: u8,
}
pub const UNIQUE_NAME: i32 = 0;

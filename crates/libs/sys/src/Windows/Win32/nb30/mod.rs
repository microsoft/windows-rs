#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("netapi32.dll" "system" fn Netbios(pncb : PNCB) -> super::UCHAR);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ACTION_HEADER {
    pub transport_id: super::ULONG,
    pub action_code: super::USHORT,
    pub reserved: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ADAPTER_STATUS {
    pub adapter_address: [super::UCHAR; 6],
    pub rev_major: super::UCHAR,
    pub reserved0: super::UCHAR,
    pub adapter_type: super::UCHAR,
    pub rev_minor: super::UCHAR,
    pub duration: super::WORD,
    pub frmr_recv: super::WORD,
    pub frmr_xmit: super::WORD,
    pub iframe_recv_err: super::WORD,
    pub xmit_aborts: super::WORD,
    pub xmit_success: super::DWORD,
    pub recv_success: super::DWORD,
    pub iframe_xmit_err: super::WORD,
    pub recv_buff_unavail: super::WORD,
    pub t1_timeouts: super::WORD,
    pub ti_timeouts: super::WORD,
    pub reserved1: super::DWORD,
    pub free_ncbs: super::WORD,
    pub max_cfg_ncbs: super::WORD,
    pub max_ncbs: super::WORD,
    pub xmit_buf_unavail: super::WORD,
    pub max_dgram_size: super::WORD,
    pub pending_sess: super::WORD,
    pub max_cfg_sess: super::WORD,
    pub max_sess: super::WORD,
    pub max_sess_pkt_size: super::WORD,
    pub name_count: super::WORD,
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct FIND_NAME_BUFFER {
    pub length: super::UCHAR,
    pub access_control: super::UCHAR,
    pub frame_control: super::UCHAR,
    pub destination_addr: [super::UCHAR; 6],
    pub source_addr: [super::UCHAR; 6],
    pub routing_info: [super::UCHAR; 18],
}
#[cfg(feature = "shared")]
impl Default for FIND_NAME_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FIND_NAME_HEADER {
    pub node_count: super::WORD,
    pub reserved: super::UCHAR,
    pub unique_group: super::UCHAR,
}
pub const GROUP_NAME: i32 = 128;
pub const HANGUP_COMPLETE: i32 = 5;
pub const HANGUP_PENDING: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct LANA_ENUM {
    pub length: super::UCHAR,
    pub lana: [super::UCHAR; 255],
}
#[cfg(feature = "shared")]
impl Default for LANA_ENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LISTEN_OUTSTANDING: i32 = 1;
pub const MAX_LANA: i32 = 254;
pub const MS_NBF: windows_sys::core::PCSTR = windows_sys::core::s!("MNBF");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NAME_BUFFER {
    pub name: [super::UCHAR; 16],
    pub name_num: super::UCHAR,
    pub name_flags: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NAME_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NAME_FLAGS_MASK: i32 = 135;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct NCB {
    pub ncb_command: super::UCHAR,
    pub ncb_retcode: super::UCHAR,
    pub ncb_lsn: super::UCHAR,
    pub ncb_num: super::UCHAR,
    pub ncb_buffer: super::PUCHAR,
    pub ncb_length: super::WORD,
    pub ncb_callname: [super::UCHAR; 16],
    pub ncb_name: [super::UCHAR; 16],
    pub ncb_rto: super::UCHAR,
    pub ncb_sto: super::UCHAR,
    pub ncb_post: Option<unsafe extern "system" fn(*mut Self)>,
    pub ncb_lana_num: super::UCHAR,
    pub ncb_cmd_cplt: super::UCHAR,
    pub ncb_reserve: [super::UCHAR; 10],
    pub ncb_event: super::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for NCB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct NCB {
    pub ncb_command: super::UCHAR,
    pub ncb_retcode: super::UCHAR,
    pub ncb_lsn: super::UCHAR,
    pub ncb_num: super::UCHAR,
    pub ncb_buffer: super::PUCHAR,
    pub ncb_length: super::WORD,
    pub ncb_callname: [super::UCHAR; 16],
    pub ncb_name: [super::UCHAR; 16],
    pub ncb_rto: super::UCHAR,
    pub ncb_sto: super::UCHAR,
    pub ncb_post: Option<unsafe extern "system" fn(*mut Self)>,
    pub ncb_lana_num: super::UCHAR,
    pub ncb_cmd_cplt: super::UCHAR,
    pub ncb_reserve: [super::UCHAR; 18],
    pub ncb_event: super::HANDLE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
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
#[cfg(feature = "shared")]
pub type PACTION_HEADER = *mut ACTION_HEADER;
#[cfg(feature = "shared")]
pub type PADAPTER_STATUS = *mut ADAPTER_STATUS;
#[cfg(feature = "shared")]
pub type PFIND_NAME_BUFFER = *mut FIND_NAME_BUFFER;
#[cfg(feature = "shared")]
pub type PFIND_NAME_HEADER = *mut FIND_NAME_HEADER;
#[cfg(feature = "shared")]
pub type PLANA_ENUM = *mut LANA_ENUM;
#[cfg(feature = "shared")]
pub type PNAME_BUFFER = *mut NAME_BUFFER;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PNCB = *mut NCB;
#[cfg(feature = "shared")]
pub type PSESSION_BUFFER = *mut SESSION_BUFFER;
#[cfg(feature = "shared")]
pub type PSESSION_HEADER = *mut SESSION_HEADER;
pub const REGISTERED: i32 = 4;
pub const REGISTERING: i32 = 0;
pub const SESSION_ABORTED: i32 = 6;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SESSION_BUFFER {
    pub lsn: super::UCHAR,
    pub state: super::UCHAR,
    pub local_name: [super::UCHAR; 16],
    pub remote_name: [super::UCHAR; 16],
    pub rcvs_outstanding: super::UCHAR,
    pub sends_outstanding: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for SESSION_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SESSION_ESTABLISHED: i32 = 3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SESSION_HEADER {
    pub sess_name: super::UCHAR,
    pub num_sess: super::UCHAR,
    pub rcv_dg_outstanding: super::UCHAR,
    pub rcv_any_outstanding: super::UCHAR,
}
pub const UNIQUE_NAME: i32 = 0;

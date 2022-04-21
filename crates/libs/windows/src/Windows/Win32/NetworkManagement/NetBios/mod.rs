#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub struct ACTION_HEADER {
    pub transport_id: u32,
    pub action_code: u16,
    pub reserved: u16,
}
impl ::core::marker::Copy for ACTION_HEADER {}
impl ::core::clone::Clone for ACTION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTION_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTION_HEADER").field("transport_id", &self.transport_id).field("action_code", &self.action_code).field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for ACTION_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTION_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTION_HEADER {}
impl ::core::default::Default for ACTION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
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
impl ::core::marker::Copy for ADAPTER_STATUS {}
impl ::core::clone::Clone for ADAPTER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADAPTER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADAPTER_STATUS")
            .field("adapter_address", &self.adapter_address)
            .field("rev_major", &self.rev_major)
            .field("reserved0", &self.reserved0)
            .field("adapter_type", &self.adapter_type)
            .field("rev_minor", &self.rev_minor)
            .field("duration", &self.duration)
            .field("frmr_recv", &self.frmr_recv)
            .field("frmr_xmit", &self.frmr_xmit)
            .field("iframe_recv_err", &self.iframe_recv_err)
            .field("xmit_aborts", &self.xmit_aborts)
            .field("xmit_success", &self.xmit_success)
            .field("recv_success", &self.recv_success)
            .field("iframe_xmit_err", &self.iframe_xmit_err)
            .field("recv_buff_unavail", &self.recv_buff_unavail)
            .field("t1_timeouts", &self.t1_timeouts)
            .field("ti_timeouts", &self.ti_timeouts)
            .field("reserved1", &self.reserved1)
            .field("free_ncbs", &self.free_ncbs)
            .field("max_cfg_ncbs", &self.max_cfg_ncbs)
            .field("max_ncbs", &self.max_ncbs)
            .field("xmit_buf_unavail", &self.xmit_buf_unavail)
            .field("max_dgram_size", &self.max_dgram_size)
            .field("pending_sess", &self.pending_sess)
            .field("max_cfg_sess", &self.max_cfg_sess)
            .field("max_sess", &self.max_sess)
            .field("max_sess_pkt_size", &self.max_sess_pkt_size)
            .field("name_count", &self.name_count)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for ADAPTER_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ADAPTER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ADAPTER_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for ADAPTER_STATUS {}
impl ::core::default::Default for ADAPTER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const ALL_TRANSPORTS: &str = "M\u{0}\u{0}\u{0}";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const ASYNCH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const CALL_PENDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const DEREGISTERED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const DUPLICATE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const DUPLICATE_DEREG: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub struct FIND_NAME_BUFFER {
    pub length: u8,
    pub access_control: u8,
    pub frame_control: u8,
    pub destination_addr: [u8; 6],
    pub source_addr: [u8; 6],
    pub routing_info: [u8; 18],
}
impl ::core::marker::Copy for FIND_NAME_BUFFER {}
impl ::core::clone::Clone for FIND_NAME_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FIND_NAME_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIND_NAME_BUFFER").field("length", &self.length).field("access_control", &self.access_control).field("frame_control", &self.frame_control).field("destination_addr", &self.destination_addr).field("source_addr", &self.source_addr).field("routing_info", &self.routing_info).finish()
    }
}
unsafe impl ::windows::core::Abi for FIND_NAME_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FIND_NAME_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FIND_NAME_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for FIND_NAME_BUFFER {}
impl ::core::default::Default for FIND_NAME_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub struct FIND_NAME_HEADER {
    pub node_count: u16,
    pub reserved: u8,
    pub unique_group: u8,
}
impl ::core::marker::Copy for FIND_NAME_HEADER {}
impl ::core::clone::Clone for FIND_NAME_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FIND_NAME_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIND_NAME_HEADER").field("node_count", &self.node_count).field("reserved", &self.reserved).field("unique_group", &self.unique_group).finish()
    }
}
unsafe impl ::windows::core::Abi for FIND_NAME_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FIND_NAME_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FIND_NAME_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for FIND_NAME_HEADER {}
impl ::core::default::Default for FIND_NAME_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const GROUP_NAME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const HANGUP_COMPLETE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const HANGUP_PENDING: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub struct LANA_ENUM {
    pub length: u8,
    pub lana: [u8; 255],
}
impl ::core::marker::Copy for LANA_ENUM {}
impl ::core::clone::Clone for LANA_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LANA_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LANA_ENUM").field("length", &self.length).field("lana", &self.lana).finish()
    }
}
unsafe impl ::windows::core::Abi for LANA_ENUM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LANA_ENUM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LANA_ENUM>()) == 0 }
    }
}
impl ::core::cmp::Eq for LANA_ENUM {}
impl ::core::default::Default for LANA_ENUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const LISTEN_OUTSTANDING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const MAX_LANA: u32 = 254u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const MS_NBF: &str = "MNBF";
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub struct NAME_BUFFER {
    pub name: [u8; 16],
    pub name_num: u8,
    pub name_flags: u8,
}
impl ::core::marker::Copy for NAME_BUFFER {}
impl ::core::clone::Clone for NAME_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NAME_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAME_BUFFER").field("name", &self.name).field("name_num", &self.name_num).field("name_flags", &self.name_flags).finish()
    }
}
unsafe impl ::windows::core::Abi for NAME_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NAME_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NAME_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for NAME_BUFFER {}
impl ::core::default::Default for NAME_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NAME_FLAGS_MASK: u32 = 135u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct NCB {
    pub ncb_command: u8,
    pub ncb_retcode: u8,
    pub ncb_lsn: u8,
    pub ncb_num: u8,
    pub ncb_buffer: *mut u8,
    pub ncb_length: u16,
    pub ncb_callname: [u8; 16],
    pub ncb_name: [u8; 16],
    pub ncb_rto: u8,
    pub ncb_sto: u8,
    pub ncb_post: isize,
    pub ncb_lana_num: u8,
    pub ncb_cmd_cplt: u8,
    pub ncb_reserve: [u8; 18],
    pub ncb_event: super::super::Foundation::HANDLE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NCB {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NCB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCB")
            .field("ncb_command", &self.ncb_command)
            .field("ncb_retcode", &self.ncb_retcode)
            .field("ncb_lsn", &self.ncb_lsn)
            .field("ncb_num", &self.ncb_num)
            .field("ncb_buffer", &self.ncb_buffer)
            .field("ncb_length", &self.ncb_length)
            .field("ncb_callname", &self.ncb_callname)
            .field("ncb_name", &self.ncb_name)
            .field("ncb_rto", &self.ncb_rto)
            .field("ncb_sto", &self.ncb_sto)
            .field("ncb_post", &self.ncb_post)
            .field("ncb_lana_num", &self.ncb_lana_num)
            .field("ncb_cmd_cplt", &self.ncb_cmd_cplt)
            .field("ncb_reserve", &self.ncb_reserve)
            .field("ncb_event", &self.ncb_event)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NCB {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NCB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NCB>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NCB {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NCB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct NCB {
    pub ncb_command: u8,
    pub ncb_retcode: u8,
    pub ncb_lsn: u8,
    pub ncb_num: u8,
    pub ncb_buffer: *mut u8,
    pub ncb_length: u16,
    pub ncb_callname: [u8; 16],
    pub ncb_name: [u8; 16],
    pub ncb_rto: u8,
    pub ncb_sto: u8,
    pub ncb_post: isize,
    pub ncb_lana_num: u8,
    pub ncb_cmd_cplt: u8,
    pub ncb_reserve: [u8; 10],
    pub ncb_event: super::super::Foundation::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NCB {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NCB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCB")
            .field("ncb_command", &self.ncb_command)
            .field("ncb_retcode", &self.ncb_retcode)
            .field("ncb_lsn", &self.ncb_lsn)
            .field("ncb_num", &self.ncb_num)
            .field("ncb_buffer", &self.ncb_buffer)
            .field("ncb_length", &self.ncb_length)
            .field("ncb_callname", &self.ncb_callname)
            .field("ncb_name", &self.ncb_name)
            .field("ncb_rto", &self.ncb_rto)
            .field("ncb_sto", &self.ncb_sto)
            .field("ncb_post", &self.ncb_post)
            .field("ncb_lana_num", &self.ncb_lana_num)
            .field("ncb_cmd_cplt", &self.ncb_cmd_cplt)
            .field("ncb_reserve", &self.ncb_reserve)
            .field("ncb_event", &self.ncb_event)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NCB {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NCB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NCB>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NCB {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NCB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBACTION: u32 = 119u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBADDGRNAME: u32 = 54u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBADDNAME: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBASTAT: u32 = 51u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBCALL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBCANCEL: u32 = 53u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBCHAINSEND: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBCHAINSENDNA: u32 = 114u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBDELNAME: u32 = 49u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBDGRECV: u32 = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBDGRECVBC: u32 = 35u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBDGSEND: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBDGSENDBC: u32 = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBENUM: u32 = 55u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBFINDNAME: u32 = 120u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBHANGUP: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBLANSTALERT: u32 = 115u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBLISTEN: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBNAMSZ: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBRECV: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBRECVANY: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBRESET: u32 = 50u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBSEND: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBSENDNA: u32 = 113u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBSSTAT: u32 = 52u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBTRACE: u32 = 121u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NCBUNLINK: u32 = 112u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_ACTSES: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_BADDR: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_BRIDGE: u32 = 35u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_BUFLEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_CANCEL: u32 = 38u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_CANOCCR: u32 = 36u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_CMDCAN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_CMDTMO: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_DUPENV: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_DUPNAME: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_ENVNOTDEF: u32 = 52u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_GOODRET: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_IFBUSY: u32 = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_ILLCMD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_ILLNN: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_INCOMP: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_INUSE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_INVADDRESS: u32 = 57u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_INVDDID: u32 = 59u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_LOCKFAIL: u32 = 60u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_LOCTFUL: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_MAXAPPS: u32 = 54u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_NAMCONF: u32 = 25u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_NAMERR: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_NAMTFUL: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_NOCALL: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_NORES: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_NORESOURCES: u32 = 56u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_NOSAPS: u32 = 55u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_NOWILD: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_OPENERR: u32 = 63u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_OSRESNOTAV: u32 = 53u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_PENDING: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_REMTFUL: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_SABORT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_SCLOSED: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_SNUMOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_SYSTEM: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const NRC_TOOMANY: u32 = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Netbios(pncb: *mut NCB) -> u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Netbios(pncb: *mut NCB) -> u8;
        }
        ::core::mem::transmute(Netbios(::core::mem::transmute(pncb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const REGISTERED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const REGISTERING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const SESSION_ABORTED: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub struct SESSION_BUFFER {
    pub lsn: u8,
    pub state: u8,
    pub local_name: [u8; 16],
    pub remote_name: [u8; 16],
    pub rcvs_outstanding: u8,
    pub sends_outstanding: u8,
}
impl ::core::marker::Copy for SESSION_BUFFER {}
impl ::core::clone::Clone for SESSION_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SESSION_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_BUFFER").field("lsn", &self.lsn).field("state", &self.state).field("local_name", &self.local_name).field("remote_name", &self.remote_name).field("rcvs_outstanding", &self.rcvs_outstanding).field("sends_outstanding", &self.sends_outstanding).finish()
    }
}
unsafe impl ::windows::core::Abi for SESSION_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SESSION_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SESSION_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for SESSION_BUFFER {}
impl ::core::default::Default for SESSION_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const SESSION_ESTABLISHED: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub struct SESSION_HEADER {
    pub sess_name: u8,
    pub num_sess: u8,
    pub rcv_dg_outstanding: u8,
    pub rcv_any_outstanding: u8,
}
impl ::core::marker::Copy for SESSION_HEADER {}
impl ::core::clone::Clone for SESSION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SESSION_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_HEADER").field("sess_name", &self.sess_name).field("num_sess", &self.num_sess).field("rcv_dg_outstanding", &self.rcv_dg_outstanding).field("rcv_any_outstanding", &self.rcv_any_outstanding).finish()
    }
}
unsafe impl ::windows::core::Abi for SESSION_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SESSION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SESSION_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for SESSION_HEADER {}
impl ::core::default::Default for SESSION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetBios\"`*"]
pub const UNIQUE_NAME: u32 = 0u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");

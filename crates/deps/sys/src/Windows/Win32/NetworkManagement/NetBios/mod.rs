#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn Netbios(pncb: *mut NCB) -> u8;
}
#[repr(C)]
pub struct ACTION_HEADER(i32);
#[repr(C)]
pub struct ADAPTER_STATUS(i32);
pub const ASYNCH: u32 = 128u32;
pub const CALL_PENDING: u32 = 2u32;
pub const DEREGISTERED: u32 = 5u32;
pub const DUPLICATE: u32 = 6u32;
pub const DUPLICATE_DEREG: u32 = 7u32;
#[repr(C)]
pub struct FIND_NAME_BUFFER(i32);
#[repr(C)]
pub struct FIND_NAME_HEADER(i32);
pub const GROUP_NAME: u32 = 128u32;
pub const HANGUP_COMPLETE: u32 = 5u32;
pub const HANGUP_PENDING: u32 = 4u32;
#[repr(C)]
pub struct LANA_ENUM(i32);
pub const LISTEN_OUTSTANDING: u32 = 1u32;
pub const MAX_LANA: u32 = 254u32;
#[repr(C)]
pub struct NAME_BUFFER(i32);
pub const NAME_FLAGS_MASK: u32 = 135u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NCB(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NCB(i32);
pub const NCBACTION: u32 = 119u32;
pub const NCBADDGRNAME: u32 = 54u32;
pub const NCBADDNAME: u32 = 48u32;
pub const NCBASTAT: u32 = 51u32;
pub const NCBCALL: u32 = 16u32;
pub const NCBCANCEL: u32 = 53u32;
pub const NCBCHAINSEND: u32 = 23u32;
pub const NCBCHAINSENDNA: u32 = 114u32;
pub const NCBDELNAME: u32 = 49u32;
pub const NCBDGRECV: u32 = 33u32;
pub const NCBDGRECVBC: u32 = 35u32;
pub const NCBDGSEND: u32 = 32u32;
pub const NCBDGSENDBC: u32 = 34u32;
pub const NCBENUM: u32 = 55u32;
pub const NCBFINDNAME: u32 = 120u32;
pub const NCBHANGUP: u32 = 18u32;
pub const NCBLANSTALERT: u32 = 115u32;
pub const NCBLISTEN: u32 = 17u32;
pub const NCBNAMSZ: u32 = 16u32;
pub const NCBRECV: u32 = 21u32;
pub const NCBRECVANY: u32 = 22u32;
pub const NCBRESET: u32 = 50u32;
pub const NCBSEND: u32 = 20u32;
pub const NCBSENDNA: u32 = 113u32;
pub const NCBSSTAT: u32 = 52u32;
pub const NCBTRACE: u32 = 121u32;
pub const NCBUNLINK: u32 = 112u32;
pub const NRC_ACTSES: u32 = 15u32;
pub const NRC_BADDR: u32 = 7u32;
pub const NRC_BRIDGE: u32 = 35u32;
pub const NRC_BUFLEN: u32 = 1u32;
pub const NRC_CANCEL: u32 = 38u32;
pub const NRC_CANOCCR: u32 = 36u32;
pub const NRC_CMDCAN: u32 = 11u32;
pub const NRC_CMDTMO: u32 = 5u32;
pub const NRC_DUPENV: u32 = 48u32;
pub const NRC_DUPNAME: u32 = 13u32;
pub const NRC_ENVNOTDEF: u32 = 52u32;
pub const NRC_GOODRET: u32 = 0u32;
pub const NRC_IFBUSY: u32 = 33u32;
pub const NRC_ILLCMD: u32 = 3u32;
pub const NRC_ILLNN: u32 = 19u32;
pub const NRC_INCOMP: u32 = 6u32;
pub const NRC_INUSE: u32 = 22u32;
pub const NRC_INVADDRESS: u32 = 57u32;
pub const NRC_INVDDID: u32 = 59u32;
pub const NRC_LOCKFAIL: u32 = 60u32;
pub const NRC_LOCTFUL: u32 = 17u32;
pub const NRC_MAXAPPS: u32 = 54u32;
pub const NRC_NAMCONF: u32 = 25u32;
pub const NRC_NAMERR: u32 = 23u32;
pub const NRC_NAMTFUL: u32 = 14u32;
pub const NRC_NOCALL: u32 = 20u32;
pub const NRC_NORES: u32 = 9u32;
pub const NRC_NORESOURCES: u32 = 56u32;
pub const NRC_NOSAPS: u32 = 55u32;
pub const NRC_NOWILD: u32 = 21u32;
pub const NRC_OPENERR: u32 = 63u32;
pub const NRC_OSRESNOTAV: u32 = 53u32;
pub const NRC_PENDING: u32 = 255u32;
pub const NRC_REMTFUL: u32 = 18u32;
pub const NRC_SABORT: u32 = 24u32;
pub const NRC_SCLOSED: u32 = 10u32;
pub const NRC_SNUMOUT: u32 = 8u32;
pub const NRC_SYSTEM: u32 = 64u32;
pub const NRC_TOOMANY: u32 = 34u32;
pub const REGISTERED: u32 = 4u32;
pub const REGISTERING: u32 = 0u32;
pub const SESSION_ABORTED: u32 = 6u32;
#[repr(C)]
pub struct SESSION_BUFFER(i32);
pub const SESSION_ESTABLISHED: u32 = 3u32;
#[repr(C)]
pub struct SESSION_HEADER(i32);
pub const UNIQUE_NAME: u32 = 0u32;

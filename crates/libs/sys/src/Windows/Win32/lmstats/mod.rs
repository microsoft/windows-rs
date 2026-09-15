#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("netapi32.dll" "system" fn NetStatisticsGet(servername : super::LPTSTR, service : super::LPTSTR, level : u32, options : u32, buffer : *mut super::LPBYTE) -> u32);
pub type LPSTAT_SERVER_0 = *mut STAT_SERVER_0;
#[cfg(feature = "winnt")]
pub type LPSTAT_WORKSTATION_0 = *mut STAT_WORKSTATION_0;
pub type PSTAT_SERVER_0 = *mut STAT_SERVER_0;
#[cfg(feature = "winnt")]
pub type PSTAT_WORKSTATION_0 = *mut STAT_WORKSTATION_0;
pub const STATSOPT_CLR: i32 = 1;
pub const STATS_NO_VALUE: u32 = 4294967295;
pub const STATS_OVERFLOW: u32 = 4294967294;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STAT_SERVER_0 {
    pub sts0_start: u32,
    pub sts0_fopens: u32,
    pub sts0_devopens: u32,
    pub sts0_jobsqueued: u32,
    pub sts0_sopens: u32,
    pub sts0_stimedout: u32,
    pub sts0_serrorout: u32,
    pub sts0_pwerrors: u32,
    pub sts0_permerrors: u32,
    pub sts0_syserrors: u32,
    pub sts0_bytessent_low: u32,
    pub sts0_bytessent_high: u32,
    pub sts0_bytesrcvd_low: u32,
    pub sts0_bytesrcvd_high: u32,
    pub sts0_avresponse: u32,
    pub sts0_reqbufneed: u32,
    pub sts0_bigbufneed: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct STAT_WORKSTATION_0 {
    pub StatisticsStartTime: super::LARGE_INTEGER,
    pub BytesReceived: super::LARGE_INTEGER,
    pub SmbsReceived: super::LARGE_INTEGER,
    pub PagingReadBytesRequested: super::LARGE_INTEGER,
    pub NonPagingReadBytesRequested: super::LARGE_INTEGER,
    pub CacheReadBytesRequested: super::LARGE_INTEGER,
    pub NetworkReadBytesRequested: super::LARGE_INTEGER,
    pub BytesTransmitted: super::LARGE_INTEGER,
    pub SmbsTransmitted: super::LARGE_INTEGER,
    pub PagingWriteBytesRequested: super::LARGE_INTEGER,
    pub NonPagingWriteBytesRequested: super::LARGE_INTEGER,
    pub CacheWriteBytesRequested: super::LARGE_INTEGER,
    pub NetworkWriteBytesRequested: super::LARGE_INTEGER,
    pub InitiallyFailedOperations: u32,
    pub FailedCompletionOperations: u32,
    pub ReadOperations: u32,
    pub RandomReadOperations: u32,
    pub ReadSmbs: u32,
    pub LargeReadSmbs: u32,
    pub SmallReadSmbs: u32,
    pub WriteOperations: u32,
    pub RandomWriteOperations: u32,
    pub WriteSmbs: u32,
    pub LargeWriteSmbs: u32,
    pub SmallWriteSmbs: u32,
    pub RawReadsDenied: u32,
    pub RawWritesDenied: u32,
    pub NetworkErrors: u32,
    pub Sessions: u32,
    pub FailedSessions: u32,
    pub Reconnects: u32,
    pub CoreConnects: u32,
    pub Lanman20Connects: u32,
    pub Lanman21Connects: u32,
    pub LanmanNtConnects: u32,
    pub ServerDisconnects: u32,
    pub HungSessions: u32,
    pub UseCount: u32,
    pub FailedUseCount: u32,
    pub CurrentCommands: u32,
}
#[cfg(feature = "winnt")]
impl Default for STAT_WORKSTATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

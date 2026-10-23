#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NetStatisticsGet(servername: super::LPTSTR, service: super::LPTSTR, level: super::DWORD, options: super::DWORD, buffer: *mut super::LPBYTE) -> super::DWORD {
    windows_core::link!("netapi32.dll" "system" fn NetStatisticsGet(servername : super::LPTSTR, service : super::LPTSTR, level : super::DWORD, options : super::DWORD, buffer : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetStatisticsGet(servername, service, level, options, buffer as _) }
}
#[cfg(feature = "shared")]
pub type LPSTAT_SERVER_0 = *mut STAT_SERVER_0;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSTAT_WORKSTATION_0 = *mut STAT_WORKSTATION_0;
#[cfg(feature = "shared")]
pub type PSTAT_SERVER_0 = *mut STAT_SERVER_0;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTAT_WORKSTATION_0 = *mut STAT_WORKSTATION_0;
pub const STATSOPT_CLR: i32 = 1;
pub const STATS_NO_VALUE: u32 = 4294967295;
pub const STATS_OVERFLOW: u32 = 4294967294;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STAT_SERVER_0 {
    pub sts0_start: super::DWORD,
    pub sts0_fopens: super::DWORD,
    pub sts0_devopens: super::DWORD,
    pub sts0_jobsqueued: super::DWORD,
    pub sts0_sopens: super::DWORD,
    pub sts0_stimedout: super::DWORD,
    pub sts0_serrorout: super::DWORD,
    pub sts0_pwerrors: super::DWORD,
    pub sts0_permerrors: super::DWORD,
    pub sts0_syserrors: super::DWORD,
    pub sts0_bytessent_low: super::DWORD,
    pub sts0_bytessent_high: super::DWORD,
    pub sts0_bytesrcvd_low: super::DWORD,
    pub sts0_bytesrcvd_high: super::DWORD,
    pub sts0_avresponse: super::DWORD,
    pub sts0_reqbufneed: super::DWORD,
    pub sts0_bigbufneed: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
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
    pub InitiallyFailedOperations: super::DWORD,
    pub FailedCompletionOperations: super::DWORD,
    pub ReadOperations: super::DWORD,
    pub RandomReadOperations: super::DWORD,
    pub ReadSmbs: super::DWORD,
    pub LargeReadSmbs: super::DWORD,
    pub SmallReadSmbs: super::DWORD,
    pub WriteOperations: super::DWORD,
    pub RandomWriteOperations: super::DWORD,
    pub WriteSmbs: super::DWORD,
    pub LargeWriteSmbs: super::DWORD,
    pub SmallWriteSmbs: super::DWORD,
    pub RawReadsDenied: super::DWORD,
    pub RawWritesDenied: super::DWORD,
    pub NetworkErrors: super::DWORD,
    pub Sessions: super::DWORD,
    pub FailedSessions: super::DWORD,
    pub Reconnects: super::DWORD,
    pub CoreConnects: super::DWORD,
    pub Lanman20Connects: super::DWORD,
    pub Lanman21Connects: super::DWORD,
    pub LanmanNtConnects: super::DWORD,
    pub ServerDisconnects: super::DWORD,
    pub HungSessions: super::DWORD,
    pub UseCount: super::DWORD,
    pub FailedUseCount: super::DWORD,
    pub CurrentCommands: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for STAT_WORKSTATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

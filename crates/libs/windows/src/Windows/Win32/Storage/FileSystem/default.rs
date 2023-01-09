#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BY_HANDLE_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BY_HANDLE_FILE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes && self.ftCreationTime == other.ftCreationTime && self.ftLastAccessTime == other.ftLastAccessTime && self.ftLastWriteTime == other.ftLastWriteTime && self.dwVolumeSerialNumber == other.dwVolumeSerialNumber && self.nFileSizeHigh == other.nFileSizeHigh && self.nFileSizeLow == other.nFileSizeLow && self.nNumberOfLinks == other.nNumberOfLinks && self.nFileIndexHigh == other.nFileIndexHigh && self.nFileIndexLow == other.nFileIndexLow
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BY_HANDLE_FILE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BY_HANDLE_FILE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BY_HANDLE_FILE_INFORMATION")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("dwVolumeSerialNumber", &self.dwVolumeSerialNumber)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .field("nNumberOfLinks", &self.nNumberOfLinks)
            .field("nFileIndexHigh", &self.nFileIndexHigh)
            .field("nFileIndexLow", &self.nFileIndexLow)
            .finish()
    }
}
impl ::core::default::Default for CLFS_CONTEXT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLFS_CONTEXT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_CONTEXT_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLFS_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLFS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CLFS_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CLFS_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CLFS_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CLFS_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CLFS_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CLFS_IOSTATS_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLFS_IOSTATS_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_IOSTATS_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLFS_LOG_ARCHIVE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLFS_LOG_ARCHIVE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_LOG_ARCHIVE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLFS_LOG_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLFS_LOG_NAME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NameLengthInBytes == other.NameLengthInBytes && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for CLFS_LOG_NAME_INFORMATION {}
impl ::core::fmt::Debug for CLFS_LOG_NAME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_LOG_NAME_INFORMATION").field("NameLengthInBytes", &self.NameLengthInBytes).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for CLFS_MGMT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLFS_MGMT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.Notification == other.Notification && self.Lsn == other.Lsn && self.LogIsPinned == other.LogIsPinned
    }
}
impl ::core::cmp::Eq for CLFS_MGMT_NOTIFICATION {}
impl ::core::fmt::Debug for CLFS_MGMT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_MGMT_NOTIFICATION").field("Notification", &self.Notification).field("Lsn", &self.Lsn).field("LogIsPinned", &self.LogIsPinned).finish()
    }
}
impl ::core::default::Default for CLFS_MGMT_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_MGMT_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLFS_MGMT_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CLFS_MGMT_POLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLFS_MGMT_POLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLFS_MGMT_POLICY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLFS_NODE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLFS_NODE_ID {
    fn eq(&self, other: &Self) -> bool {
        self.cType == other.cType && self.cbNode == other.cbNode
    }
}
impl ::core::cmp::Eq for CLFS_NODE_ID {}
impl ::core::fmt::Debug for CLFS_NODE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_NODE_ID").field("cType", &self.cType).field("cbNode", &self.cbNode).finish()
    }
}
impl ::core::default::Default for CLFS_PHYSICAL_LSN_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLFS_PHYSICAL_LSN_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StreamIdentifier == other.StreamIdentifier && self.VirtualLsn == other.VirtualLsn && self.PhysicalLsn == other.PhysicalLsn
    }
}
impl ::core::cmp::Eq for CLFS_PHYSICAL_LSN_INFORMATION {}
impl ::core::fmt::Debug for CLFS_PHYSICAL_LSN_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_PHYSICAL_LSN_INFORMATION").field("StreamIdentifier", &self.StreamIdentifier).field("VirtualLsn", &self.VirtualLsn).field("PhysicalLsn", &self.PhysicalLsn).finish()
    }
}
impl ::core::default::Default for CLFS_STREAM_ID_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLFS_STREAM_ID_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StreamIdentifier == other.StreamIdentifier
    }
}
impl ::core::cmp::Eq for CLFS_STREAM_ID_INFORMATION {}
impl ::core::fmt::Debug for CLFS_STREAM_ID_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLFS_STREAM_ID_INFORMATION").field("StreamIdentifier", &self.StreamIdentifier).finish()
    }
}
impl ::core::default::Default for CLS_ARCHIVE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLS_ARCHIVE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.coffLow == other.coffLow && self.coffHigh == other.coffHigh && self.infoContainer == other.infoContainer
    }
}
impl ::core::cmp::Eq for CLS_ARCHIVE_DESCRIPTOR {}
impl ::core::fmt::Debug for CLS_ARCHIVE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_ARCHIVE_DESCRIPTOR").field("coffLow", &self.coffLow).field("coffHigh", &self.coffHigh).field("infoContainer", &self.infoContainer).finish()
    }
}
impl ::core::default::Default for CLS_CONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLS_CONTAINER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.FileAttributes == other.FileAttributes && self.CreationTime == other.CreationTime && self.LastAccessTime == other.LastAccessTime && self.LastWriteTime == other.LastWriteTime && self.ContainerSize == other.ContainerSize && self.FileNameActualLength == other.FileNameActualLength && self.FileNameLength == other.FileNameLength && self.FileName == other.FileName && self.State == other.State && self.PhysicalContainerId == other.PhysicalContainerId && self.LogicalContainerId == other.LogicalContainerId
    }
}
impl ::core::cmp::Eq for CLS_CONTAINER_INFORMATION {}
impl ::core::fmt::Debug for CLS_CONTAINER_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_CONTAINER_INFORMATION")
            .field("FileAttributes", &self.FileAttributes)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ContainerSize", &self.ContainerSize)
            .field("FileNameActualLength", &self.FileNameActualLength)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileName", &self.FileName)
            .field("State", &self.State)
            .field("PhysicalContainerId", &self.PhysicalContainerId)
            .field("LogicalContainerId", &self.LogicalContainerId)
            .finish()
    }
}
impl ::core::default::Default for CLS_CONTEXT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLS_CONTEXT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLS_CONTEXT_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TotalAvailable == other.TotalAvailable && self.CurrentAvailable == other.CurrentAvailable && self.TotalReservation == other.TotalReservation && self.BaseFileSize == other.BaseFileSize && self.ContainerSize == other.ContainerSize && self.TotalContainers == other.TotalContainers && self.FreeContainers == other.FreeContainers && self.TotalClients == other.TotalClients && self.Attributes == other.Attributes && self.FlushThreshold == other.FlushThreshold && self.SectorSize == other.SectorSize && self.MinArchiveTailLsn == other.MinArchiveTailLsn && self.BaseLsn == other.BaseLsn && self.LastFlushedLsn == other.LastFlushedLsn && self.LastLsn == other.LastLsn && self.RestartLsn == other.RestartLsn && self.Identity == other.Identity
    }
}
impl ::core::cmp::Eq for CLS_INFORMATION {}
impl ::core::fmt::Debug for CLS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_INFORMATION")
            .field("TotalAvailable", &self.TotalAvailable)
            .field("CurrentAvailable", &self.CurrentAvailable)
            .field("TotalReservation", &self.TotalReservation)
            .field("BaseFileSize", &self.BaseFileSize)
            .field("ContainerSize", &self.ContainerSize)
            .field("TotalContainers", &self.TotalContainers)
            .field("FreeContainers", &self.FreeContainers)
            .field("TotalClients", &self.TotalClients)
            .field("Attributes", &self.Attributes)
            .field("FlushThreshold", &self.FlushThreshold)
            .field("SectorSize", &self.SectorSize)
            .field("MinArchiveTailLsn", &self.MinArchiveTailLsn)
            .field("BaseLsn", &self.BaseLsn)
            .field("LastFlushedLsn", &self.LastFlushedLsn)
            .field("LastLsn", &self.LastLsn)
            .field("RestartLsn", &self.RestartLsn)
            .field("Identity", &self.Identity)
            .finish()
    }
}
impl ::core::default::Default for CLS_IOSTATS_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLS_IOSTATS_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLS_IOSTATS_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLS_IO_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLS_IO_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.hdrIoStats == other.hdrIoStats && self.cFlush == other.cFlush && self.cbFlush == other.cbFlush && self.cMetaFlush == other.cMetaFlush && self.cbMetaFlush == other.cbMetaFlush
    }
}
impl ::core::cmp::Eq for CLS_IO_STATISTICS {}
impl ::core::fmt::Debug for CLS_IO_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_IO_STATISTICS").field("hdrIoStats", &self.hdrIoStats).field("cFlush", &self.cFlush).field("cbFlush", &self.cbFlush).field("cMetaFlush", &self.cMetaFlush).field("cbMetaFlush", &self.cbMetaFlush).finish()
    }
}
impl ::core::default::Default for CLS_IO_STATISTICS_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLS_IO_STATISTICS_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.ubMajorVersion == other.ubMajorVersion && self.ubMinorVersion == other.ubMinorVersion && self.eStatsClass == other.eStatsClass && self.cbLength == other.cbLength && self.coffData == other.coffData
    }
}
impl ::core::cmp::Eq for CLS_IO_STATISTICS_HEADER {}
impl ::core::fmt::Debug for CLS_IO_STATISTICS_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_IO_STATISTICS_HEADER").field("ubMajorVersion", &self.ubMajorVersion).field("ubMinorVersion", &self.ubMinorVersion).field("eStatsClass", &self.eStatsClass).field("cbLength", &self.cbLength).field("coffData", &self.coffData).finish()
    }
}
impl ::core::default::Default for CLS_LOG_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLS_LOG_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLS_LOG_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLS_LSN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLS_LSN {
    fn eq(&self, other: &Self) -> bool {
        self.Internal == other.Internal
    }
}
impl ::core::cmp::Eq for CLS_LSN {}
impl ::core::fmt::Debug for CLS_LSN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_LSN").field("Internal", &self.Internal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLS_SCAN_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLS_SCAN_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cidNode == other.cidNode && self.hLog == other.hLog && self.cIndex == other.cIndex && self.cContainers == other.cContainers && self.cContainersReturned == other.cContainersReturned && self.eScanMode == other.eScanMode && self.pinfoContainer == other.pinfoContainer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLS_SCAN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLS_SCAN_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_SCAN_CONTEXT").field("cidNode", &self.cidNode).field("hLog", &self.hLog).field("cIndex", &self.cIndex).field("cContainers", &self.cContainers).field("cContainersReturned", &self.cContainersReturned).field("eScanMode", &self.eScanMode).field("pinfoContainer", &self.pinfoContainer).finish()
    }
}
impl ::core::default::Default for CLS_WRITE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLS_WRITE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.ByteLength == other.ByteLength
    }
}
impl ::core::cmp::Eq for CLS_WRITE_ENTRY {}
impl ::core::fmt::Debug for CLS_WRITE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLS_WRITE_ENTRY").field("Buffer", &self.Buffer).field("ByteLength", &self.ByteLength).finish()
    }
}
impl ::core::default::Default for COMPRESSION_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMPRESSION_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPRESSION_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for CONNECTION_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONNECTION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.coni0_id == other.coni0_id
    }
}
impl ::core::cmp::Eq for CONNECTION_INFO_0 {}
impl ::core::fmt::Debug for CONNECTION_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTION_INFO_0").field("coni0_id", &self.coni0_id).finish()
    }
}
impl ::core::default::Default for CONNECTION_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONNECTION_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.coni1_id == other.coni1_id && self.coni1_type == other.coni1_type && self.coni1_num_opens == other.coni1_num_opens && self.coni1_num_users == other.coni1_num_users && self.coni1_time == other.coni1_time && self.coni1_username == other.coni1_username && self.coni1_netname == other.coni1_netname
    }
}
impl ::core::cmp::Eq for CONNECTION_INFO_1 {}
impl ::core::fmt::Debug for CONNECTION_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTION_INFO_1").field("coni1_id", &self.coni1_id).field("coni1_type", &self.coni1_type).field("coni1_num_opens", &self.coni1_num_opens).field("coni1_num_users", &self.coni1_num_users).field("coni1_time", &self.coni1_time).field("coni1_username", &self.coni1_username).field("coni1_netname", &self.coni1_netname).finish()
    }
}
impl ::core::default::Default for COPYFILE2_COPY_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COPYFILE2_COPY_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COPYFILE2_COPY_PHASE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COPYFILE2_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for COPYFILE2_MESSAGE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COPYFILE2_MESSAGE_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for COPYFILE2_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COPYFILE2_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COPYFILE2_MESSAGE_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for CREATEFILE2_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for CREATEFILE2_EXTENDED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFileAttributes == other.dwFileAttributes && self.dwFileFlags == other.dwFileFlags && self.dwSecurityQosFlags == other.dwSecurityQosFlags && self.lpSecurityAttributes == other.lpSecurityAttributes && self.hTemplateFile == other.hTemplateFile
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for CREATEFILE2_EXTENDED_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for CREATEFILE2_EXTENDED_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATEFILE2_EXTENDED_PARAMETERS").field("dwSize", &self.dwSize).field("dwFileAttributes", &self.dwFileAttributes).field("dwFileFlags", &self.dwFileFlags).field("dwSecurityQosFlags", &self.dwSecurityQosFlags).field("lpSecurityAttributes", &self.lpSecurityAttributes).field("hTemplateFile", &self.hTemplateFile).finish()
    }
}
impl ::core::default::Default for CREATE_TAPE_PARTITION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_TAPE_PARTITION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_TAPE_PARTITION_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEFINE_DOS_DEVICE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEFINE_DOS_DEVICE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEFINE_DOS_DEVICE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DEFINE_DOS_DEVICE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DEFINE_DOS_DEVICE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DEFINE_DOS_DEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DISKQUOTA_USERNAME_RESOLVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISKQUOTA_USERNAME_RESOLVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISKQUOTA_USERNAME_RESOLVE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISKQUOTA_USER_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISKQUOTA_USER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.QuotaUsed == other.QuotaUsed && self.QuotaThreshold == other.QuotaThreshold && self.QuotaLimit == other.QuotaLimit
    }
}
impl ::core::cmp::Eq for DISKQUOTA_USER_INFORMATION {}
impl ::core::fmt::Debug for DISKQUOTA_USER_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISKQUOTA_USER_INFORMATION").field("QuotaUsed", &self.QuotaUsed).field("QuotaThreshold", &self.QuotaThreshold).field("QuotaLimit", &self.QuotaLimit).finish()
    }
}
impl ::core::default::Default for DISK_SPACE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISK_SPACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ActualTotalAllocationUnits == other.ActualTotalAllocationUnits
            && self.ActualAvailableAllocationUnits == other.ActualAvailableAllocationUnits
            && self.ActualPoolUnavailableAllocationUnits == other.ActualPoolUnavailableAllocationUnits
            && self.CallerTotalAllocationUnits == other.CallerTotalAllocationUnits
            && self.CallerAvailableAllocationUnits == other.CallerAvailableAllocationUnits
            && self.CallerPoolUnavailableAllocationUnits == other.CallerPoolUnavailableAllocationUnits
            && self.UsedAllocationUnits == other.UsedAllocationUnits
            && self.TotalReservedAllocationUnits == other.TotalReservedAllocationUnits
            && self.VolumeStorageReserveAllocationUnits == other.VolumeStorageReserveAllocationUnits
            && self.AvailableCommittedAllocationUnits == other.AvailableCommittedAllocationUnits
            && self.PoolAvailableAllocationUnits == other.PoolAvailableAllocationUnits
            && self.SectorsPerAllocationUnit == other.SectorsPerAllocationUnit
            && self.BytesPerSector == other.BytesPerSector
    }
}
impl ::core::cmp::Eq for DISK_SPACE_INFORMATION {}
impl ::core::fmt::Debug for DISK_SPACE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_SPACE_INFORMATION")
            .field("ActualTotalAllocationUnits", &self.ActualTotalAllocationUnits)
            .field("ActualAvailableAllocationUnits", &self.ActualAvailableAllocationUnits)
            .field("ActualPoolUnavailableAllocationUnits", &self.ActualPoolUnavailableAllocationUnits)
            .field("CallerTotalAllocationUnits", &self.CallerTotalAllocationUnits)
            .field("CallerAvailableAllocationUnits", &self.CallerAvailableAllocationUnits)
            .field("CallerPoolUnavailableAllocationUnits", &self.CallerPoolUnavailableAllocationUnits)
            .field("UsedAllocationUnits", &self.UsedAllocationUnits)
            .field("TotalReservedAllocationUnits", &self.TotalReservedAllocationUnits)
            .field("VolumeStorageReserveAllocationUnits", &self.VolumeStorageReserveAllocationUnits)
            .field("AvailableCommittedAllocationUnits", &self.AvailableCommittedAllocationUnits)
            .field("PoolAvailableAllocationUnits", &self.PoolAvailableAllocationUnits)
            .field("SectorsPerAllocationUnit", &self.SectorsPerAllocationUnit)
            .field("BytesPerSector", &self.BytesPerSector)
            .finish()
    }
}
impl ::core::default::Default for EFS_CERTIFICATE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EFS_CERTIFICATE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwCertEncodingType == other.dwCertEncodingType && self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for EFS_CERTIFICATE_BLOB {}
impl ::core::fmt::Debug for EFS_CERTIFICATE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_CERTIFICATE_BLOB").field("dwCertEncodingType", &self.dwCertEncodingType).field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::core::default::Default for EFS_COMPATIBILITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EFS_COMPATIBILITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EfsVersion == other.EfsVersion
    }
}
impl ::core::cmp::Eq for EFS_COMPATIBILITY_INFO {}
impl ::core::fmt::Debug for EFS_COMPATIBILITY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_COMPATIBILITY_INFO").field("EfsVersion", &self.EfsVersion).finish()
    }
}
impl ::core::default::Default for EFS_DECRYPTION_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EFS_DECRYPTION_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwDecryptionError == other.dwDecryptionError && self.dwHashOffset == other.dwHashOffset && self.cbHash == other.cbHash
    }
}
impl ::core::cmp::Eq for EFS_DECRYPTION_STATUS_INFO {}
impl ::core::fmt::Debug for EFS_DECRYPTION_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_DECRYPTION_STATUS_INFO").field("dwDecryptionError", &self.dwDecryptionError).field("dwHashOffset", &self.dwHashOffset).field("cbHash", &self.cbHash).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EFS_ENCRYPTION_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EFS_ENCRYPTION_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.bHasCurrentKey == other.bHasCurrentKey && self.dwEncryptionError == other.dwEncryptionError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EFS_ENCRYPTION_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EFS_ENCRYPTION_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_ENCRYPTION_STATUS_INFO").field("bHasCurrentKey", &self.bHasCurrentKey).field("dwEncryptionError", &self.dwEncryptionError).finish()
    }
}
impl ::core::default::Default for EFS_HASH_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EFS_HASH_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for EFS_HASH_BLOB {}
impl ::core::fmt::Debug for EFS_HASH_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_HASH_BLOB").field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::core::default::Default for EFS_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EFS_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.Entropy == other.Entropy && self.Algorithm == other.Algorithm && self.KeyLength == other.KeyLength
    }
}
impl ::core::cmp::Eq for EFS_KEY_INFO {}
impl ::core::fmt::Debug for EFS_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_KEY_INFO").field("dwVersion", &self.dwVersion).field("Entropy", &self.Entropy).field("Algorithm", &self.Algorithm).field("KeyLength", &self.KeyLength).finish()
    }
}
impl ::core::default::Default for EFS_PIN_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EFS_PIN_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbPadding == other.cbPadding && self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for EFS_PIN_BLOB {}
impl ::core::fmt::Debug for EFS_PIN_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_PIN_BLOB").field("cbPadding", &self.cbPadding).field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::core::default::Default for EFS_RPC_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EFS_RPC_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for EFS_RPC_BLOB {}
impl ::core::fmt::Debug for EFS_RPC_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_RPC_BLOB").field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
impl ::core::default::Default for EFS_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EFS_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EfsVersion == other.EfsVersion && self.SubVersion == other.SubVersion
    }
}
impl ::core::cmp::Eq for EFS_VERSION_INFO {}
impl ::core::fmt::Debug for EFS_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFS_VERSION_INFO").field("EfsVersion", &self.EfsVersion).field("SubVersion", &self.SubVersion).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.dwEfsAccessType == other.dwEfsAccessType && self.pCertificatesAdded == other.pCertificatesAdded && self.pEncryptionCertificate == other.pEncryptionCertificate && self.pEfsStreamSignature == other.pEfsStreamSignature
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTED_FILE_METADATA_SIGNATURE {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTED_FILE_METADATA_SIGNATURE").field("dwEfsAccessType", &self.dwEfsAccessType).field("pCertificatesAdded", &self.pCertificatesAdded).field("pEncryptionCertificate", &self.pEncryptionCertificate).field("pEfsStreamSignature", &self.pEfsStreamSignature).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTION_CERTIFICATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTION_CERTIFICATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalLength == other.cbTotalLength && self.pUserSid == other.pUserSid && self.pCertBlob == other.pCertBlob
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTION_CERTIFICATE {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTION_CERTIFICATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_CERTIFICATE").field("cbTotalLength", &self.cbTotalLength).field("pUserSid", &self.pUserSid).field("pCertBlob", &self.pCertBlob).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTION_CERTIFICATE_HASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTION_CERTIFICATE_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalLength == other.cbTotalLength && self.pUserSid == other.pUserSid && self.pHash == other.pHash && self.lpDisplayInformation == other.lpDisplayInformation
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTION_CERTIFICATE_HASH {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTION_CERTIFICATE_HASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_CERTIFICATE_HASH").field("cbTotalLength", &self.cbTotalLength).field("pUserSid", &self.pUserSid).field("pHash", &self.pHash).field("lpDisplayInformation", &self.lpDisplayInformation).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.nCert_Hash == other.nCert_Hash && self.pUsers == other.pUsers
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTION_CERTIFICATE_HASH_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_CERTIFICATE_HASH_LIST").field("nCert_Hash", &self.nCert_Hash).field("pUsers", &self.pUsers).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTION_CERTIFICATE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTION_CERTIFICATE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.nUsers == other.nUsers && self.pUsers == other.pUsers
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTION_CERTIFICATE_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTION_CERTIFICATE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_CERTIFICATE_LIST").field("nUsers", &self.nUsers).field("pUsers", &self.pUsers).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTION_PROTECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTION_PROTECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalLength == other.cbTotalLength && self.pUserSid == other.pUserSid && self.lpProtectorDescriptor == other.lpProtectorDescriptor
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTION_PROTECTOR {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTION_PROTECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_PROTECTOR").field("cbTotalLength", &self.cbTotalLength).field("pUserSid", &self.pUserSid).field("lpProtectorDescriptor", &self.lpProtectorDescriptor).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for ENCRYPTION_PROTECTOR_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for ENCRYPTION_PROTECTOR_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.nProtectors == other.nProtectors && self.pProtectors == other.pProtectors
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for ENCRYPTION_PROTECTOR_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for ENCRYPTION_PROTECTOR_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_PROTECTOR_LIST").field("nProtectors", &self.nProtectors).field("pProtectors", &self.pProtectors).finish()
    }
}
impl ::core::default::Default for ERASE_TAPE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ERASE_TAPE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERASE_TAPE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FH_OVERLAPPED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FILE_ACCESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_ACCESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_ACCESS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_ACCESS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_ACCESS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_ACCESS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_ACCESS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_ACCESS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for FILE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILE_ALIGNMENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_ALIGNMENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AlignmentRequirement == other.AlignmentRequirement
    }
}
impl ::core::cmp::Eq for FILE_ALIGNMENT_INFO {}
impl ::core::fmt::Debug for FILE_ALIGNMENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ALIGNMENT_INFO").field("AlignmentRequirement", &self.AlignmentRequirement).finish()
    }
}
impl ::core::default::Default for FILE_ALLOCATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_ALLOCATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AllocationSize == other.AllocationSize
    }
}
impl ::core::cmp::Eq for FILE_ALLOCATION_INFO {}
impl ::core::fmt::Debug for FILE_ALLOCATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ALLOCATION_INFO").field("AllocationSize", &self.AllocationSize).finish()
    }
}
impl ::core::default::Default for FILE_ATTRIBUTE_TAG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_ATTRIBUTE_TAG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileAttributes == other.FileAttributes && self.ReparseTag == other.ReparseTag
    }
}
impl ::core::cmp::Eq for FILE_ATTRIBUTE_TAG_INFO {}
impl ::core::fmt::Debug for FILE_ATTRIBUTE_TAG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ATTRIBUTE_TAG_INFO").field("FileAttributes", &self.FileAttributes).field("ReparseTag", &self.ReparseTag).finish()
    }
}
impl ::core::default::Default for FILE_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CreationTime == other.CreationTime && self.LastAccessTime == other.LastAccessTime && self.LastWriteTime == other.LastWriteTime && self.ChangeTime == other.ChangeTime && self.FileAttributes == other.FileAttributes
    }
}
impl ::core::cmp::Eq for FILE_BASIC_INFO {}
impl ::core::fmt::Debug for FILE_BASIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_BASIC_INFO").field("CreationTime", &self.CreationTime).field("LastAccessTime", &self.LastAccessTime).field("LastWriteTime", &self.LastWriteTime).field("ChangeTime", &self.ChangeTime).field("FileAttributes", &self.FileAttributes).finish()
    }
}
impl ::core::default::Default for FILE_COMPRESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_COMPRESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CompressedFileSize == other.CompressedFileSize && self.CompressionFormat == other.CompressionFormat && self.CompressionUnitShift == other.CompressionUnitShift && self.ChunkShift == other.ChunkShift && self.ClusterShift == other.ClusterShift && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for FILE_COMPRESSION_INFO {}
impl ::core::fmt::Debug for FILE_COMPRESSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_COMPRESSION_INFO").field("CompressedFileSize", &self.CompressedFileSize).field("CompressionFormat", &self.CompressionFormat).field("CompressionUnitShift", &self.CompressionUnitShift).field("ChunkShift", &self.ChunkShift).field("ClusterShift", &self.ClusterShift).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for FILE_CREATION_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_CREATION_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_CREATION_DISPOSITION").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILE_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_DEVICE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_DISPOSITION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_DISPOSITION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DeleteFile == other.DeleteFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_DISPOSITION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_DISPOSITION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_DISPOSITION_INFO").field("DeleteFile", &self.DeleteFile).finish()
    }
}
impl ::core::default::Default for FILE_END_OF_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_END_OF_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EndOfFile == other.EndOfFile
    }
}
impl ::core::cmp::Eq for FILE_END_OF_FILE_INFO {}
impl ::core::fmt::Debug for FILE_END_OF_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_END_OF_FILE_INFO").field("EndOfFile", &self.EndOfFile).finish()
    }
}
impl ::core::default::Default for FILE_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeOffset == other.VolumeOffset && self.ExtentLength == other.ExtentLength
    }
}
impl ::core::cmp::Eq for FILE_EXTENT {}
impl ::core::fmt::Debug for FILE_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_EXTENT").field("VolumeOffset", &self.VolumeOffset).field("ExtentLength", &self.ExtentLength).finish()
    }
}
impl ::core::default::Default for FILE_FLAGS_AND_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_FLAGS_AND_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_FLAGS_AND_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_FLAGS_AND_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_FLAGS_AND_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_FLAGS_AND_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for FILE_FULL_DIR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_FULL_DIR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.FileIndex == other.FileIndex && self.CreationTime == other.CreationTime && self.LastAccessTime == other.LastAccessTime && self.LastWriteTime == other.LastWriteTime && self.ChangeTime == other.ChangeTime && self.EndOfFile == other.EndOfFile && self.AllocationSize == other.AllocationSize && self.FileAttributes == other.FileAttributes && self.FileNameLength == other.FileNameLength && self.EaSize == other.EaSize && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_FULL_DIR_INFO {}
impl ::core::fmt::Debug for FILE_FULL_DIR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_FULL_DIR_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FileIndex", &self.FileIndex)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("EndOfFile", &self.EndOfFile)
            .field("AllocationSize", &self.AllocationSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("EaSize", &self.EaSize)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::core::default::Default for FILE_ID_128 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_ID_128 {
    fn eq(&self, other: &Self) -> bool {
        self.Identifier == other.Identifier
    }
}
impl ::core::cmp::Eq for FILE_ID_128 {}
impl ::core::fmt::Debug for FILE_ID_128 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ID_128").field("Identifier", &self.Identifier).finish()
    }
}
impl ::core::default::Default for FILE_ID_BOTH_DIR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_ID_BOTH_DIR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.FileIndex == other.FileIndex && self.CreationTime == other.CreationTime && self.LastAccessTime == other.LastAccessTime && self.LastWriteTime == other.LastWriteTime && self.ChangeTime == other.ChangeTime && self.EndOfFile == other.EndOfFile && self.AllocationSize == other.AllocationSize && self.FileAttributes == other.FileAttributes && self.FileNameLength == other.FileNameLength && self.EaSize == other.EaSize && self.ShortNameLength == other.ShortNameLength && self.ShortName == other.ShortName && self.FileId == other.FileId && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_ID_BOTH_DIR_INFO {}
impl ::core::fmt::Debug for FILE_ID_BOTH_DIR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ID_BOTH_DIR_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FileIndex", &self.FileIndex)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("EndOfFile", &self.EndOfFile)
            .field("AllocationSize", &self.AllocationSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("EaSize", &self.EaSize)
            .field("ShortNameLength", &self.ShortNameLength)
            .field("ShortName", &self.ShortName)
            .field("FileId", &self.FileId)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::core::default::Default for FILE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FILE_ID_EXTD_DIR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_ID_EXTD_DIR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.FileIndex == other.FileIndex && self.CreationTime == other.CreationTime && self.LastAccessTime == other.LastAccessTime && self.LastWriteTime == other.LastWriteTime && self.ChangeTime == other.ChangeTime && self.EndOfFile == other.EndOfFile && self.AllocationSize == other.AllocationSize && self.FileAttributes == other.FileAttributes && self.FileNameLength == other.FileNameLength && self.EaSize == other.EaSize && self.ReparsePointTag == other.ReparsePointTag && self.FileId == other.FileId && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_ID_EXTD_DIR_INFO {}
impl ::core::fmt::Debug for FILE_ID_EXTD_DIR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ID_EXTD_DIR_INFO")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("FileIndex", &self.FileIndex)
            .field("CreationTime", &self.CreationTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("LastWriteTime", &self.LastWriteTime)
            .field("ChangeTime", &self.ChangeTime)
            .field("EndOfFile", &self.EndOfFile)
            .field("AllocationSize", &self.AllocationSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("EaSize", &self.EaSize)
            .field("ReparsePointTag", &self.ReparsePointTag)
            .field("FileId", &self.FileId)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::core::default::Default for FILE_ID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_ID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeSerialNumber == other.VolumeSerialNumber && self.FileId == other.FileId
    }
}
impl ::core::cmp::Eq for FILE_ID_INFO {}
impl ::core::fmt::Debug for FILE_ID_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ID_INFO").field("VolumeSerialNumber", &self.VolumeSerialNumber).field("FileId", &self.FileId).finish()
    }
}
impl ::core::default::Default for FILE_ID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_ID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_ID_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILE_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.fi2_id == other.fi2_id
    }
}
impl ::core::cmp::Eq for FILE_INFO_2 {}
impl ::core::fmt::Debug for FILE_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_INFO_2").field("fi2_id", &self.fi2_id).finish()
    }
}
impl ::core::default::Default for FILE_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.fi3_id == other.fi3_id && self.fi3_permissions == other.fi3_permissions && self.fi3_num_locks == other.fi3_num_locks && self.fi3_pathname == other.fi3_pathname && self.fi3_username == other.fi3_username
    }
}
impl ::core::cmp::Eq for FILE_INFO_3 {}
impl ::core::fmt::Debug for FILE_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_INFO_3").field("fi3_id", &self.fi3_id).field("fi3_permissions", &self.fi3_permissions).field("fi3_num_locks", &self.fi3_num_locks).field("fi3_pathname", &self.fi3_pathname).field("fi3_username", &self.fi3_username).finish()
    }
}
impl ::core::default::Default for FILE_INFO_BY_HANDLE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_INFO_BY_HANDLE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_INFO_BY_HANDLE_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILE_INFO_FLAGS_PERMISSIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_INFO_FLAGS_PERMISSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_INFO_FLAGS_PERMISSIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_INFO_FLAGS_PERMISSIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_INFO_FLAGS_PERMISSIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_INFO_FLAGS_PERMISSIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for FILE_IO_PRIORITY_HINT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_IO_PRIORITY_HINT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PriorityHint == other.PriorityHint
    }
}
impl ::core::cmp::Eq for FILE_IO_PRIORITY_HINT_INFO {}
impl ::core::fmt::Debug for FILE_IO_PRIORITY_HINT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_IO_PRIORITY_HINT_INFO").field("PriorityHint", &self.PriorityHint).finish()
    }
}
impl ::core::default::Default for FILE_NAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_NAME").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILE_NAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_NAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileNameLength == other.FileNameLength && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_NAME_INFO {}
impl ::core::fmt::Debug for FILE_NAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_NAME_INFO").field("FileNameLength", &self.FileNameLength).field("FileName", &self.FileName).finish()
    }
}
impl ::core::default::Default for FILE_NOTIFY_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_NOTIFY_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_NOTIFY_CHANGE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_NOTIFY_CHANGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_NOTIFY_CHANGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_NOTIFY_CHANGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.Action == other.Action && self.CreationTime == other.CreationTime && self.LastModificationTime == other.LastModificationTime && self.LastChangeTime == other.LastChangeTime && self.LastAccessTime == other.LastAccessTime && self.AllocatedLength == other.AllocatedLength && self.FileSize == other.FileSize && self.FileAttributes == other.FileAttributes && self.ReparsePointTag == other.ReparsePointTag && self.FileId == other.FileId && self.ParentFileId == other.ParentFileId && self.FileNameLength == other.FileNameLength && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_NOTIFY_EXTENDED_INFORMATION {}
impl ::core::fmt::Debug for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_NOTIFY_EXTENDED_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("Action", &self.Action)
            .field("CreationTime", &self.CreationTime)
            .field("LastModificationTime", &self.LastModificationTime)
            .field("LastChangeTime", &self.LastChangeTime)
            .field("LastAccessTime", &self.LastAccessTime)
            .field("AllocatedLength", &self.AllocatedLength)
            .field("FileSize", &self.FileSize)
            .field("FileAttributes", &self.FileAttributes)
            .field("ReparsePointTag", &self.ReparsePointTag)
            .field("FileId", &self.FileId)
            .field("ParentFileId", &self.ParentFileId)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileName", &self.FileName)
            .finish()
    }
}
impl ::core::default::Default for FILE_NOTIFY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_NOTIFY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.Action == other.Action && self.FileNameLength == other.FileNameLength && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_NOTIFY_INFORMATION {}
impl ::core::fmt::Debug for FILE_NOTIFY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_NOTIFY_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("Action", &self.Action).field("FileNameLength", &self.FileNameLength).field("FileName", &self.FileName).finish()
    }
}
impl ::core::default::Default for FILE_REMOTE_PROTOCOL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_RENAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FILE_SEGMENT_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FILE_SHARE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_SHARE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_SHARE_MODE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_SHARE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_SHARE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_SHARE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_SHARE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_SHARE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_STANDARD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_STANDARD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.AllocationSize == other.AllocationSize && self.EndOfFile == other.EndOfFile && self.NumberOfLinks == other.NumberOfLinks && self.DeletePending == other.DeletePending && self.Directory == other.Directory
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_STANDARD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_STANDARD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STANDARD_INFO").field("AllocationSize", &self.AllocationSize).field("EndOfFile", &self.EndOfFile).field("NumberOfLinks", &self.NumberOfLinks).field("DeletePending", &self.DeletePending).field("Directory", &self.Directory).finish()
    }
}
impl ::core::default::Default for FILE_STORAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_STORAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LogicalBytesPerSector == other.LogicalBytesPerSector && self.PhysicalBytesPerSectorForAtomicity == other.PhysicalBytesPerSectorForAtomicity && self.PhysicalBytesPerSectorForPerformance == other.PhysicalBytesPerSectorForPerformance && self.FileSystemEffectivePhysicalBytesPerSectorForAtomicity == other.FileSystemEffectivePhysicalBytesPerSectorForAtomicity && self.Flags == other.Flags && self.ByteOffsetForSectorAlignment == other.ByteOffsetForSectorAlignment && self.ByteOffsetForPartitionAlignment == other.ByteOffsetForPartitionAlignment
    }
}
impl ::core::cmp::Eq for FILE_STORAGE_INFO {}
impl ::core::fmt::Debug for FILE_STORAGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STORAGE_INFO")
            .field("LogicalBytesPerSector", &self.LogicalBytesPerSector)
            .field("PhysicalBytesPerSectorForAtomicity", &self.PhysicalBytesPerSectorForAtomicity)
            .field("PhysicalBytesPerSectorForPerformance", &self.PhysicalBytesPerSectorForPerformance)
            .field("FileSystemEffectivePhysicalBytesPerSectorForAtomicity", &self.FileSystemEffectivePhysicalBytesPerSectorForAtomicity)
            .field("Flags", &self.Flags)
            .field("ByteOffsetForSectorAlignment", &self.ByteOffsetForSectorAlignment)
            .field("ByteOffsetForPartitionAlignment", &self.ByteOffsetForPartitionAlignment)
            .finish()
    }
}
impl ::core::default::Default for FILE_STREAM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_STREAM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.StreamNameLength == other.StreamNameLength && self.StreamSize == other.StreamSize && self.StreamAllocationSize == other.StreamAllocationSize && self.StreamName == other.StreamName
    }
}
impl ::core::cmp::Eq for FILE_STREAM_INFO {}
impl ::core::fmt::Debug for FILE_STREAM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STREAM_INFO").field("NextEntryOffset", &self.NextEntryOffset).field("StreamNameLength", &self.StreamNameLength).field("StreamSize", &self.StreamSize).field("StreamAllocationSize", &self.StreamAllocationSize).field("StreamName", &self.StreamName).finish()
    }
}
impl ::core::default::Default for FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FINDEX_INFO_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FINDEX_INFO_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FINDEX_INFO_LEVELS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FINDEX_SEARCH_OPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FINDEX_SEARCH_OPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FINDEX_SEARCH_OPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FIND_FIRST_EX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FIND_FIRST_EX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FIND_FIRST_EX_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FIND_FIRST_EX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FIND_FIRST_EX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FIND_FIRST_EX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FIO_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FIO_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.m_dwTempHack == other.m_dwTempHack && self.m_dwSignature == other.m_dwSignature && self.m_hFile == other.m_hFile && self.m_dwLinesOffset == other.m_dwLinesOffset && self.m_dwHeaderLength == other.m_dwHeaderLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FIO_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FIO_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIO_CONTEXT").field("m_dwTempHack", &self.m_dwTempHack).field("m_dwSignature", &self.m_dwSignature).field("m_hFile", &self.m_hFile).field("m_dwLinesOffset", &self.m_dwLinesOffset).field("m_dwHeaderLength", &self.m_dwHeaderLength).finish()
    }
}
impl ::core::default::Default for GET_FILEEX_INFO_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_FILEEX_INFO_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_FILEEX_INFO_LEVELS").field(&self.0).finish()
    }
}
impl ::core::default::Default for GET_FILE_VERSION_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_FILE_VERSION_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_FILE_VERSION_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_FILE_VERSION_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_FILE_VERSION_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_FILE_VERSION_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_TAPE_DRIVE_PARAMETERS_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_TAPE_DRIVE_PARAMETERS_OPERATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for HIORING__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HIORING__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HIORING__ {}
impl ::core::fmt::Debug for HIORING__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIORING__").field("unused", &self.unused).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDiskQuotaControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDiskQuotaControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDiskQuotaControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiskQuotaControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IDiskQuotaControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumConnectionPoints(&self) -> ::windows::core::Result<super::super::System::Com::IEnumConnectionPoints> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumConnectionPoints)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindConnectionPoint(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::IConnectionPoint> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindConnectionPoint)(::windows::core::Vtable::as_raw(self), riid, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDiskQuotaEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiskQuotaEvents {}
impl ::core::fmt::Debug for IDiskQuotaEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiskQuotaEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDiskQuotaUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiskQuotaUser {}
impl ::core::fmt::Debug for IDiskQuotaUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiskQuotaUser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDiskQuotaUserBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiskQuotaUserBatch {}
impl ::core::fmt::Debug for IDiskQuotaUserBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiskQuotaUserBatch").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDiskQuotaUsers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDiskQuotaUsers {}
impl ::core::fmt::Debug for IEnumDiskQuotaUsers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDiskQuotaUsers").field(&self.0).finish()
    }
}
impl ::core::default::Default for IORING_BUFFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IORING_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for IORING_BUFFER_INFO {}
impl ::core::fmt::Debug for IORING_BUFFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_BUFFER_INFO").field("Address", &self.Address).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for IORING_BUFFER_REF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IORING_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IORING_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.MaxVersion == other.MaxVersion && self.MaxSubmissionQueueSize == other.MaxSubmissionQueueSize && self.MaxCompletionQueueSize == other.MaxCompletionQueueSize && self.FeatureFlags == other.FeatureFlags
    }
}
impl ::core::cmp::Eq for IORING_CAPABILITIES {}
impl ::core::fmt::Debug for IORING_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_CAPABILITIES").field("MaxVersion", &self.MaxVersion).field("MaxSubmissionQueueSize", &self.MaxSubmissionQueueSize).field("MaxCompletionQueueSize", &self.MaxCompletionQueueSize).field("FeatureFlags", &self.FeatureFlags).finish()
    }
}
impl ::core::default::Default for IORING_CQE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IORING_CQE {
    fn eq(&self, other: &Self) -> bool {
        self.UserData == other.UserData && self.ResultCode == other.ResultCode && self.Information == other.Information
    }
}
impl ::core::cmp::Eq for IORING_CQE {}
impl ::core::fmt::Debug for IORING_CQE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_CQE").field("UserData", &self.UserData).field("ResultCode", &self.ResultCode).field("Information", &self.Information).finish()
    }
}
impl ::core::default::Default for IORING_CREATE_ADVISORY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_CREATE_ADVISORY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_CREATE_ADVISORY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IORING_CREATE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IORING_CREATE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.Required == other.Required && self.Advisory == other.Advisory
    }
}
impl ::core::cmp::Eq for IORING_CREATE_FLAGS {}
impl ::core::fmt::Debug for IORING_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_CREATE_FLAGS").field("Required", &self.Required).field("Advisory", &self.Advisory).finish()
    }
}
impl ::core::default::Default for IORING_CREATE_REQUIRED_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_CREATE_REQUIRED_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_CREATE_REQUIRED_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IORING_FEATURE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_FEATURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_FEATURE_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IORING_HANDLE_REF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IORING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IORING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.IoRingVersion == other.IoRingVersion && self.Flags == other.Flags && self.SubmissionQueueSize == other.SubmissionQueueSize && self.CompletionQueueSize == other.CompletionQueueSize
    }
}
impl ::core::cmp::Eq for IORING_INFO {}
impl ::core::fmt::Debug for IORING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_INFO").field("IoRingVersion", &self.IoRingVersion).field("Flags", &self.Flags).field("SubmissionQueueSize", &self.SubmissionQueueSize).field("CompletionQueueSize", &self.CompletionQueueSize).finish()
    }
}
impl ::core::default::Default for IORING_OP_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_OP_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_OP_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IORING_REF_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_REF_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_REF_KIND").field(&self.0).finish()
    }
}
impl ::core::default::Default for IORING_REGISTERED_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IORING_REGISTERED_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.BufferIndex == other.BufferIndex && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for IORING_REGISTERED_BUFFER {}
impl ::core::fmt::Debug for IORING_REGISTERED_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IORING_REGISTERED_BUFFER").field("BufferIndex", &self.BufferIndex).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for IORING_SQE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_SQE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_SQE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IORING_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IORING_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IORING_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for KCRM_MARSHAL_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KCRM_MARSHAL_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.VersionMajor == other.VersionMajor && self.VersionMinor == other.VersionMinor && self.NumProtocols == other.NumProtocols && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for KCRM_MARSHAL_HEADER {}
impl ::core::fmt::Debug for KCRM_MARSHAL_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KCRM_MARSHAL_HEADER").field("VersionMajor", &self.VersionMajor).field("VersionMinor", &self.VersionMinor).field("NumProtocols", &self.NumProtocols).field("Unused", &self.Unused).finish()
    }
}
impl ::core::default::Default for KCRM_PROTOCOL_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KCRM_PROTOCOL_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.ProtocolId == other.ProtocolId && self.StaticInfoLength == other.StaticInfoLength && self.TransactionIdInfoLength == other.TransactionIdInfoLength && self.Unused1 == other.Unused1 && self.Unused2 == other.Unused2
    }
}
impl ::core::cmp::Eq for KCRM_PROTOCOL_BLOB {}
impl ::core::fmt::Debug for KCRM_PROTOCOL_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KCRM_PROTOCOL_BLOB").field("ProtocolId", &self.ProtocolId).field("StaticInfoLength", &self.StaticInfoLength).field("TransactionIdInfoLength", &self.TransactionIdInfoLength).field("Unused1", &self.Unused1).field("Unused2", &self.Unused2).finish()
    }
}
impl ::core::default::Default for KCRM_TRANSACTION_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KCRM_TRANSACTION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.UOW == other.UOW && self.TmIdentity == other.TmIdentity && self.IsolationLevel == other.IsolationLevel && self.IsolationFlags == other.IsolationFlags && self.Timeout == other.Timeout && self.Description == other.Description
    }
}
impl ::core::cmp::Eq for KCRM_TRANSACTION_BLOB {}
impl ::core::fmt::Debug for KCRM_TRANSACTION_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KCRM_TRANSACTION_BLOB").field("UOW", &self.UOW).field("TmIdentity", &self.TmIdentity).field("IsolationLevel", &self.IsolationLevel).field("IsolationFlags", &self.IsolationFlags).field("Timeout", &self.Timeout).field("Description", &self.Description).finish()
    }
}
impl ::core::default::Default for LOCK_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOCK_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCK_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LOCK_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LOCK_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LOCK_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LOCK_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LOCK_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOG_MANAGEMENT_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LPPROGRESS_ROUTINE_CALLBACK_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LPPROGRESS_ROUTINE_CALLBACK_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for LZOPENFILE_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LZOPENFILE_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LZOPENFILE_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LZOPENFILE_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LZOPENFILE_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LZOPENFILE_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LZOPENFILE_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LZOPENFILE_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MOVE_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MOVE_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOVE_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MOVE_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MOVE_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MOVE_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MOVE_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MOVE_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MediaLabelInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MediaLabelInfo {
    fn eq(&self, other: &Self) -> bool {
        self.LabelType == other.LabelType && self.LabelIDSize == other.LabelIDSize && self.LabelID == other.LabelID && self.LabelAppDescr == other.LabelAppDescr
    }
}
impl ::core::cmp::Eq for MediaLabelInfo {}
impl ::core::fmt::Debug for MediaLabelInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MediaLabelInfo").field("LabelType", &self.LabelType).field("LabelIDSize", &self.LabelIDSize).field("LabelID", &self.LabelID).field("LabelAppDescr", &self.LabelAppDescr).finish()
    }
}
impl ::core::default::Default for NAME_CACHE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NAME_CACHE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.m_dwSignature == other.m_dwSignature
    }
}
impl ::core::cmp::Eq for NAME_CACHE_CONTEXT {}
impl ::core::fmt::Debug for NAME_CACHE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAME_CACHE_CONTEXT").field("m_dwSignature", &self.m_dwSignature).finish()
    }
}
impl ::core::default::Default for NTMS_ALLOCATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_ALLOCATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpReserved == other.lpReserved && self.AllocatedFrom == other.AllocatedFrom
    }
}
impl ::core::cmp::Eq for NTMS_ALLOCATION_INFORMATION {}
impl ::core::fmt::Debug for NTMS_ALLOCATION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_ALLOCATION_INFORMATION").field("dwSize", &self.dwSize).field("lpReserved", &self.lpReserved).field("AllocatedFrom", &self.AllocatedFrom).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_ASYNC_IO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_ASYNC_IO {
    fn eq(&self, other: &Self) -> bool {
        self.OperationId == other.OperationId && self.EventId == other.EventId && self.dwOperationType == other.dwOperationType && self.dwResult == other.dwResult && self.dwAsyncState == other.dwAsyncState && self.hEvent == other.hEvent && self.bOnStateChange == other.bOnStateChange
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_ASYNC_IO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_ASYNC_IO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_ASYNC_IO").field("OperationId", &self.OperationId).field("EventId", &self.EventId).field("dwOperationType", &self.dwOperationType).field("dwResult", &self.dwResult).field("dwAsyncState", &self.dwAsyncState).field("hEvent", &self.hEvent).field("bOnStateChange", &self.bOnStateChange).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_CHANGERINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_CHANGERINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.ChangerType == other.ChangerType && self.szSerialNumber == other.szSerialNumber && self.szRevision == other.szRevision && self.szDeviceName == other.szDeviceName && self.ScsiPort == other.ScsiPort && self.ScsiBus == other.ScsiBus && self.ScsiTarget == other.ScsiTarget && self.ScsiLun == other.ScsiLun && self.Library == other.Library
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_CHANGERINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_CHANGERINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_CHANGERINFORMATIONA").field("Number", &self.Number).field("ChangerType", &self.ChangerType).field("szSerialNumber", &self.szSerialNumber).field("szRevision", &self.szRevision).field("szDeviceName", &self.szDeviceName).field("ScsiPort", &self.ScsiPort).field("ScsiBus", &self.ScsiBus).field("ScsiTarget", &self.ScsiTarget).field("ScsiLun", &self.ScsiLun).field("Library", &self.Library).finish()
    }
}
impl ::core::default::Default for NTMS_CHANGERINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_CHANGERINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.ChangerType == other.ChangerType && self.szSerialNumber == other.szSerialNumber && self.szRevision == other.szRevision && self.szDeviceName == other.szDeviceName && self.ScsiPort == other.ScsiPort && self.ScsiBus == other.ScsiBus && self.ScsiTarget == other.ScsiTarget && self.ScsiLun == other.ScsiLun && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_CHANGERINFORMATIONW {}
impl ::core::fmt::Debug for NTMS_CHANGERINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_CHANGERINFORMATIONW").field("Number", &self.Number).field("ChangerType", &self.ChangerType).field("szSerialNumber", &self.szSerialNumber).field("szRevision", &self.szRevision).field("szDeviceName", &self.szDeviceName).field("ScsiPort", &self.ScsiPort).field("ScsiBus", &self.ScsiBus).field("ScsiTarget", &self.ScsiTarget).field("ScsiLun", &self.ScsiLun).field("Library", &self.Library).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_CHANGERTYPEINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_CHANGERTYPEINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor && self.szProduct == other.szProduct && self.DeviceType == other.DeviceType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_CHANGERTYPEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_CHANGERTYPEINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_CHANGERTYPEINFORMATIONA").field("szVendor", &self.szVendor).field("szProduct", &self.szProduct).field("DeviceType", &self.DeviceType).finish()
    }
}
impl ::core::default::Default for NTMS_CHANGERTYPEINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_CHANGERTYPEINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor && self.szProduct == other.szProduct && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_CHANGERTYPEINFORMATIONW {}
impl ::core::fmt::Debug for NTMS_CHANGERTYPEINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_CHANGERTYPEINFORMATIONW").field("szVendor", &self.szVendor).field("szProduct", &self.szProduct).field("DeviceType", &self.DeviceType).finish()
    }
}
impl ::core::default::Default for NTMS_COMPUTERINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_COMPUTERINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwLibRequestPurgeTime == other.dwLibRequestPurgeTime && self.dwOpRequestPurgeTime == other.dwOpRequestPurgeTime && self.dwLibRequestFlags == other.dwLibRequestFlags && self.dwOpRequestFlags == other.dwOpRequestFlags && self.dwMediaPoolPolicy == other.dwMediaPoolPolicy
    }
}
impl ::core::cmp::Eq for NTMS_COMPUTERINFORMATION {}
impl ::core::fmt::Debug for NTMS_COMPUTERINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_COMPUTERINFORMATION").field("dwLibRequestPurgeTime", &self.dwLibRequestPurgeTime).field("dwOpRequestPurgeTime", &self.dwOpRequestPurgeTime).field("dwLibRequestFlags", &self.dwLibRequestFlags).field("dwOpRequestFlags", &self.dwOpRequestFlags).field("dwMediaPoolPolicy", &self.dwMediaPoolPolicy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_DRIVEINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_DRIVEINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.State == other.State && self.DriveType == other.DriveType && self.szDeviceName == other.szDeviceName && self.szSerialNumber == other.szSerialNumber && self.szRevision == other.szRevision && self.ScsiPort == other.ScsiPort && self.ScsiBus == other.ScsiBus && self.ScsiTarget == other.ScsiTarget && self.ScsiLun == other.ScsiLun && self.dwMountCount == other.dwMountCount && self.LastCleanedTs == other.LastCleanedTs && self.SavedPartitionId == other.SavedPartitionId && self.Library == other.Library && self.Reserved == other.Reserved && self.dwDeferDismountDelay == other.dwDeferDismountDelay
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_DRIVEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_DRIVEINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_DRIVEINFORMATIONA")
            .field("Number", &self.Number)
            .field("State", &self.State)
            .field("DriveType", &self.DriveType)
            .field("szDeviceName", &self.szDeviceName)
            .field("szSerialNumber", &self.szSerialNumber)
            .field("szRevision", &self.szRevision)
            .field("ScsiPort", &self.ScsiPort)
            .field("ScsiBus", &self.ScsiBus)
            .field("ScsiTarget", &self.ScsiTarget)
            .field("ScsiLun", &self.ScsiLun)
            .field("dwMountCount", &self.dwMountCount)
            .field("LastCleanedTs", &self.LastCleanedTs)
            .field("SavedPartitionId", &self.SavedPartitionId)
            .field("Library", &self.Library)
            .field("Reserved", &self.Reserved)
            .field("dwDeferDismountDelay", &self.dwDeferDismountDelay)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_DRIVEINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_DRIVEINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.State == other.State && self.DriveType == other.DriveType && self.szDeviceName == other.szDeviceName && self.szSerialNumber == other.szSerialNumber && self.szRevision == other.szRevision && self.ScsiPort == other.ScsiPort && self.ScsiBus == other.ScsiBus && self.ScsiTarget == other.ScsiTarget && self.ScsiLun == other.ScsiLun && self.dwMountCount == other.dwMountCount && self.LastCleanedTs == other.LastCleanedTs && self.SavedPartitionId == other.SavedPartitionId && self.Library == other.Library && self.Reserved == other.Reserved && self.dwDeferDismountDelay == other.dwDeferDismountDelay
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_DRIVEINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_DRIVEINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_DRIVEINFORMATIONW")
            .field("Number", &self.Number)
            .field("State", &self.State)
            .field("DriveType", &self.DriveType)
            .field("szDeviceName", &self.szDeviceName)
            .field("szSerialNumber", &self.szSerialNumber)
            .field("szRevision", &self.szRevision)
            .field("ScsiPort", &self.ScsiPort)
            .field("ScsiBus", &self.ScsiBus)
            .field("ScsiTarget", &self.ScsiTarget)
            .field("ScsiLun", &self.ScsiLun)
            .field("dwMountCount", &self.dwMountCount)
            .field("LastCleanedTs", &self.LastCleanedTs)
            .field("SavedPartitionId", &self.SavedPartitionId)
            .field("Library", &self.Library)
            .field("Reserved", &self.Reserved)
            .field("dwDeferDismountDelay", &self.dwDeferDismountDelay)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_DRIVETYPEINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_DRIVETYPEINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor && self.szProduct == other.szProduct && self.NumberOfHeads == other.NumberOfHeads && self.DeviceType == other.DeviceType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_DRIVETYPEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_DRIVETYPEINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_DRIVETYPEINFORMATIONA").field("szVendor", &self.szVendor).field("szProduct", &self.szProduct).field("NumberOfHeads", &self.NumberOfHeads).field("DeviceType", &self.DeviceType).finish()
    }
}
impl ::core::default::Default for NTMS_DRIVETYPEINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_DRIVETYPEINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.szVendor == other.szVendor && self.szProduct == other.szProduct && self.NumberOfHeads == other.NumberOfHeads && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_DRIVETYPEINFORMATIONW {}
impl ::core::fmt::Debug for NTMS_DRIVETYPEINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_DRIVETYPEINFORMATIONW").field("szVendor", &self.szVendor).field("szProduct", &self.szProduct).field("NumberOfHeads", &self.NumberOfHeads).field("DeviceType", &self.DeviceType).finish()
    }
}
impl ::core::default::Default for NTMS_FILESYSTEM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_FILESYSTEM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileSystemType == other.FileSystemType && self.VolumeName == other.VolumeName && self.SerialNumber == other.SerialNumber
    }
}
impl ::core::cmp::Eq for NTMS_FILESYSTEM_INFO {}
impl ::core::fmt::Debug for NTMS_FILESYSTEM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_FILESYSTEM_INFO").field("FileSystemType", &self.FileSystemType).field("VolumeName", &self.VolumeName).field("SerialNumber", &self.SerialNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_LIBRARYINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_I1_LIBRARYINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LibraryType == other.LibraryType
            && self.CleanerSlot == other.CleanerSlot
            && self.CleanerSlotDefault == other.CleanerSlotDefault
            && self.LibrarySupportsDriveCleaning == other.LibrarySupportsDriveCleaning
            && self.BarCodeReaderInstalled == other.BarCodeReaderInstalled
            && self.InventoryMethod == other.InventoryMethod
            && self.dwCleanerUsesRemaining == other.dwCleanerUsesRemaining
            && self.FirstDriveNumber == other.FirstDriveNumber
            && self.dwNumberOfDrives == other.dwNumberOfDrives
            && self.FirstSlotNumber == other.FirstSlotNumber
            && self.dwNumberOfSlots == other.dwNumberOfSlots
            && self.FirstDoorNumber == other.FirstDoorNumber
            && self.dwNumberOfDoors == other.dwNumberOfDoors
            && self.FirstPortNumber == other.FirstPortNumber
            && self.dwNumberOfPorts == other.dwNumberOfPorts
            && self.FirstChangerNumber == other.FirstChangerNumber
            && self.dwNumberOfChangers == other.dwNumberOfChangers
            && self.dwNumberOfMedia == other.dwNumberOfMedia
            && self.dwNumberOfMediaTypes == other.dwNumberOfMediaTypes
            && self.dwNumberOfLibRequests == other.dwNumberOfLibRequests
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_I1_LIBRARYINFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_I1_LIBRARYINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_LIBRARYINFORMATION")
            .field("LibraryType", &self.LibraryType)
            .field("CleanerSlot", &self.CleanerSlot)
            .field("CleanerSlotDefault", &self.CleanerSlotDefault)
            .field("LibrarySupportsDriveCleaning", &self.LibrarySupportsDriveCleaning)
            .field("BarCodeReaderInstalled", &self.BarCodeReaderInstalled)
            .field("InventoryMethod", &self.InventoryMethod)
            .field("dwCleanerUsesRemaining", &self.dwCleanerUsesRemaining)
            .field("FirstDriveNumber", &self.FirstDriveNumber)
            .field("dwNumberOfDrives", &self.dwNumberOfDrives)
            .field("FirstSlotNumber", &self.FirstSlotNumber)
            .field("dwNumberOfSlots", &self.dwNumberOfSlots)
            .field("FirstDoorNumber", &self.FirstDoorNumber)
            .field("dwNumberOfDoors", &self.dwNumberOfDoors)
            .field("FirstPortNumber", &self.FirstPortNumber)
            .field("dwNumberOfPorts", &self.dwNumberOfPorts)
            .field("FirstChangerNumber", &self.FirstChangerNumber)
            .field("dwNumberOfChangers", &self.dwNumberOfChangers)
            .field("dwNumberOfMedia", &self.dwNumberOfMedia)
            .field("dwNumberOfMediaTypes", &self.dwNumberOfMediaTypes)
            .field("dwNumberOfLibRequests", &self.dwNumberOfLibRequests)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode && self.OperationOption == other.OperationOption && self.State == other.State && self.PartitionId == other.PartitionId && self.DriveId == other.DriveId && self.PhysMediaId == other.PhysMediaId && self.Library == other.Library && self.SlotId == other.SlotId && self.TimeQueued == other.TimeQueued && self.TimeCompleted == other.TimeCompleted && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_I1_LIBREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_LIBREQUESTINFORMATIONA")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode && self.OperationOption == other.OperationOption && self.State == other.State && self.PartitionId == other.PartitionId && self.DriveId == other.DriveId && self.PhysMediaId == other.PhysMediaId && self.Library == other.Library && self.SlotId == other.SlotId && self.TimeQueued == other.TimeQueued && self.TimeCompleted == other.TimeCompleted && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_I1_LIBREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_LIBREQUESTINFORMATIONW")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_OBJECTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_OBJECTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_OPREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_I1_OPREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request && self.Submitted == other.Submitted && self.State == other.State && self.szMessage == other.szMessage && self.Arg1Type == other.Arg1Type && self.Arg1 == other.Arg1 && self.Arg2Type == other.Arg2Type && self.Arg2 == other.Arg2 && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_I1_OPREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_I1_OPREQUESTINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_OPREQUESTINFORMATIONA").field("Request", &self.Request).field("Submitted", &self.Submitted).field("State", &self.State).field("szMessage", &self.szMessage).field("Arg1Type", &self.Arg1Type).field("Arg1", &self.Arg1).field("Arg2Type", &self.Arg2Type).field("Arg2", &self.Arg2).field("szApplication", &self.szApplication).field("szUser", &self.szUser).field("szComputer", &self.szComputer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_OPREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_I1_OPREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request && self.Submitted == other.Submitted && self.State == other.State && self.szMessage == other.szMessage && self.Arg1Type == other.Arg1Type && self.Arg1 == other.Arg1 && self.Arg2Type == other.Arg2Type && self.Arg2 == other.Arg2 && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_I1_OPREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_I1_OPREQUESTINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_OPREQUESTINFORMATIONW").field("Request", &self.Request).field("Submitted", &self.Submitted).field("State", &self.State).field("szMessage", &self.szMessage).field("Arg1Type", &self.Arg1Type).field("Arg1", &self.Arg1).field("Arg2Type", &self.Arg2Type).field("Arg2", &self.Arg2).field("szApplication", &self.szApplication).field("szUser", &self.szUser).field("szComputer", &self.szComputer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_PARTITIONINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_I1_PARTITIONINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia && self.LogicalMedia == other.LogicalMedia && self.State == other.State && self.Side == other.Side && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength && self.OmidLabelId == other.OmidLabelId && self.szOmidLabelType == other.szOmidLabelType && self.szOmidLabelInfo == other.szOmidLabelInfo && self.dwMountCount == other.dwMountCount && self.dwAllocateCount == other.dwAllocateCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_I1_PARTITIONINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_I1_PARTITIONINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_PARTITIONINFORMATIONA").field("PhysicalMedia", &self.PhysicalMedia).field("LogicalMedia", &self.LogicalMedia).field("State", &self.State).field("Side", &self.Side).field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength).field("OmidLabelId", &self.OmidLabelId).field("szOmidLabelType", &self.szOmidLabelType).field("szOmidLabelInfo", &self.szOmidLabelInfo).field("dwMountCount", &self.dwMountCount).field("dwAllocateCount", &self.dwAllocateCount).finish()
    }
}
impl ::core::default::Default for NTMS_I1_PARTITIONINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_PARTITIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia && self.LogicalMedia == other.LogicalMedia && self.State == other.State && self.Side == other.Side && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength && self.OmidLabelId == other.OmidLabelId && self.szOmidLabelType == other.szOmidLabelType && self.szOmidLabelInfo == other.szOmidLabelInfo && self.dwMountCount == other.dwMountCount && self.dwAllocateCount == other.dwAllocateCount
    }
}
impl ::core::cmp::Eq for NTMS_I1_PARTITIONINFORMATIONW {}
impl ::core::fmt::Debug for NTMS_I1_PARTITIONINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_PARTITIONINFORMATIONW").field("PhysicalMedia", &self.PhysicalMedia).field("LogicalMedia", &self.LogicalMedia).field("State", &self.State).field("Side", &self.Side).field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength).field("OmidLabelId", &self.OmidLabelId).field("szOmidLabelType", &self.szOmidLabelType).field("szOmidLabelInfo", &self.szOmidLabelInfo).field("dwMountCount", &self.dwMountCount).field("dwAllocateCount", &self.dwAllocateCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_I1_PMIDINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_I1_PMIDINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary && self.MediaPool == other.MediaPool && self.Location == other.Location && self.LocationType == other.LocationType && self.MediaType == other.MediaType && self.HomeSlot == other.HomeSlot && self.szBarCode == other.szBarCode && self.BarCodeState == other.BarCodeState && self.szSequenceNumber == other.szSequenceNumber && self.MediaState == other.MediaState && self.dwNumberOfPartitions == other.dwNumberOfPartitions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_I1_PMIDINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_I1_PMIDINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_PMIDINFORMATIONA")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .finish()
    }
}
impl ::core::default::Default for NTMS_I1_PMIDINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_I1_PMIDINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary && self.MediaPool == other.MediaPool && self.Location == other.Location && self.LocationType == other.LocationType && self.MediaType == other.MediaType && self.HomeSlot == other.HomeSlot && self.szBarCode == other.szBarCode && self.BarCodeState == other.BarCodeState && self.szSequenceNumber == other.szSequenceNumber && self.MediaState == other.MediaState && self.dwNumberOfPartitions == other.dwNumberOfPartitions
    }
}
impl ::core::cmp::Eq for NTMS_I1_PMIDINFORMATIONW {}
impl ::core::fmt::Debug for NTMS_I1_PMIDINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_I1_PMIDINFORMATIONW")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .finish()
    }
}
impl ::core::default::Default for NTMS_IEDOORINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_IEDOORINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.State == other.State && self.MaxOpenSecs == other.MaxOpenSecs && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_IEDOORINFORMATION {}
impl ::core::fmt::Debug for NTMS_IEDOORINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_IEDOORINFORMATION").field("Number", &self.Number).field("State", &self.State).field("MaxOpenSecs", &self.MaxOpenSecs).field("Library", &self.Library).finish()
    }
}
impl ::core::default::Default for NTMS_IEPORTINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_IEPORTINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.Content == other.Content && self.Position == other.Position && self.MaxExtendSecs == other.MaxExtendSecs && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_IEPORTINFORMATION {}
impl ::core::fmt::Debug for NTMS_IEPORTINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_IEPORTINFORMATION").field("Number", &self.Number).field("Content", &self.Content).field("Position", &self.Position).field("MaxExtendSecs", &self.MaxExtendSecs).field("Library", &self.Library).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_LIBRARYINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_LIBRARYINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LibraryType == other.LibraryType
            && self.CleanerSlot == other.CleanerSlot
            && self.CleanerSlotDefault == other.CleanerSlotDefault
            && self.LibrarySupportsDriveCleaning == other.LibrarySupportsDriveCleaning
            && self.BarCodeReaderInstalled == other.BarCodeReaderInstalled
            && self.InventoryMethod == other.InventoryMethod
            && self.dwCleanerUsesRemaining == other.dwCleanerUsesRemaining
            && self.FirstDriveNumber == other.FirstDriveNumber
            && self.dwNumberOfDrives == other.dwNumberOfDrives
            && self.FirstSlotNumber == other.FirstSlotNumber
            && self.dwNumberOfSlots == other.dwNumberOfSlots
            && self.FirstDoorNumber == other.FirstDoorNumber
            && self.dwNumberOfDoors == other.dwNumberOfDoors
            && self.FirstPortNumber == other.FirstPortNumber
            && self.dwNumberOfPorts == other.dwNumberOfPorts
            && self.FirstChangerNumber == other.FirstChangerNumber
            && self.dwNumberOfChangers == other.dwNumberOfChangers
            && self.dwNumberOfMedia == other.dwNumberOfMedia
            && self.dwNumberOfMediaTypes == other.dwNumberOfMediaTypes
            && self.dwNumberOfLibRequests == other.dwNumberOfLibRequests
            && self.Reserved == other.Reserved
            && self.AutoRecovery == other.AutoRecovery
            && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_LIBRARYINFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_LIBRARYINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_LIBRARYINFORMATION")
            .field("LibraryType", &self.LibraryType)
            .field("CleanerSlot", &self.CleanerSlot)
            .field("CleanerSlotDefault", &self.CleanerSlotDefault)
            .field("LibrarySupportsDriveCleaning", &self.LibrarySupportsDriveCleaning)
            .field("BarCodeReaderInstalled", &self.BarCodeReaderInstalled)
            .field("InventoryMethod", &self.InventoryMethod)
            .field("dwCleanerUsesRemaining", &self.dwCleanerUsesRemaining)
            .field("FirstDriveNumber", &self.FirstDriveNumber)
            .field("dwNumberOfDrives", &self.dwNumberOfDrives)
            .field("FirstSlotNumber", &self.FirstSlotNumber)
            .field("dwNumberOfSlots", &self.dwNumberOfSlots)
            .field("FirstDoorNumber", &self.FirstDoorNumber)
            .field("dwNumberOfDoors", &self.dwNumberOfDoors)
            .field("FirstPortNumber", &self.FirstPortNumber)
            .field("dwNumberOfPorts", &self.dwNumberOfPorts)
            .field("FirstChangerNumber", &self.FirstChangerNumber)
            .field("dwNumberOfChangers", &self.dwNumberOfChangers)
            .field("dwNumberOfMedia", &self.dwNumberOfMedia)
            .field("dwNumberOfMediaTypes", &self.dwNumberOfMediaTypes)
            .field("dwNumberOfLibRequests", &self.dwNumberOfLibRequests)
            .field("Reserved", &self.Reserved)
            .field("AutoRecovery", &self.AutoRecovery)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_LIBREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_LIBREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode && self.OperationOption == other.OperationOption && self.State == other.State && self.PartitionId == other.PartitionId && self.DriveId == other.DriveId && self.PhysMediaId == other.PhysMediaId && self.Library == other.Library && self.SlotId == other.SlotId && self.TimeQueued == other.TimeQueued && self.TimeCompleted == other.TimeCompleted && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer && self.dwErrorCode == other.dwErrorCode && self.WorkItemId == other.WorkItemId && self.dwPriority == other.dwPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_LIBREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_LIBREQUESTINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_LIBREQUESTINFORMATIONA")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .field("dwErrorCode", &self.dwErrorCode)
            .field("WorkItemId", &self.WorkItemId)
            .field("dwPriority", &self.dwPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_LIBREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_LIBREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.OperationCode == other.OperationCode && self.OperationOption == other.OperationOption && self.State == other.State && self.PartitionId == other.PartitionId && self.DriveId == other.DriveId && self.PhysMediaId == other.PhysMediaId && self.Library == other.Library && self.SlotId == other.SlotId && self.TimeQueued == other.TimeQueued && self.TimeCompleted == other.TimeCompleted && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer && self.dwErrorCode == other.dwErrorCode && self.WorkItemId == other.WorkItemId && self.dwPriority == other.dwPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_LIBREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_LIBREQUESTINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_LIBREQUESTINFORMATIONW")
            .field("OperationCode", &self.OperationCode)
            .field("OperationOption", &self.OperationOption)
            .field("State", &self.State)
            .field("PartitionId", &self.PartitionId)
            .field("DriveId", &self.DriveId)
            .field("PhysMediaId", &self.PhysMediaId)
            .field("Library", &self.Library)
            .field("SlotId", &self.SlotId)
            .field("TimeQueued", &self.TimeQueued)
            .field("TimeCompleted", &self.TimeCompleted)
            .field("szApplication", &self.szApplication)
            .field("szUser", &self.szUser)
            .field("szComputer", &self.szComputer)
            .field("dwErrorCode", &self.dwErrorCode)
            .field("WorkItemId", &self.WorkItemId)
            .field("dwPriority", &self.dwPriority)
            .finish()
    }
}
impl ::core::default::Default for NTMS_LMIDINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_LMIDINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MediaPool == other.MediaPool && self.dwNumberOfPartitions == other.dwNumberOfPartitions
    }
}
impl ::core::cmp::Eq for NTMS_LMIDINFORMATION {}
impl ::core::fmt::Debug for NTMS_LMIDINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_LMIDINFORMATION").field("MediaPool", &self.MediaPool).field("dwNumberOfPartitions", &self.dwNumberOfPartitions).finish()
    }
}
impl ::core::default::Default for NTMS_MEDIAPOOLINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_MEDIAPOOLINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PoolType == other.PoolType && self.MediaType == other.MediaType && self.Parent == other.Parent && self.AllocationPolicy == other.AllocationPolicy && self.DeallocationPolicy == other.DeallocationPolicy && self.dwMaxAllocates == other.dwMaxAllocates && self.dwNumberOfPhysicalMedia == other.dwNumberOfPhysicalMedia && self.dwNumberOfLogicalMedia == other.dwNumberOfLogicalMedia && self.dwNumberOfMediaPools == other.dwNumberOfMediaPools
    }
}
impl ::core::cmp::Eq for NTMS_MEDIAPOOLINFORMATION {}
impl ::core::fmt::Debug for NTMS_MEDIAPOOLINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_MEDIAPOOLINFORMATION")
            .field("PoolType", &self.PoolType)
            .field("MediaType", &self.MediaType)
            .field("Parent", &self.Parent)
            .field("AllocationPolicy", &self.AllocationPolicy)
            .field("DeallocationPolicy", &self.DeallocationPolicy)
            .field("dwMaxAllocates", &self.dwMaxAllocates)
            .field("dwNumberOfPhysicalMedia", &self.dwNumberOfPhysicalMedia)
            .field("dwNumberOfLogicalMedia", &self.dwNumberOfLogicalMedia)
            .field("dwNumberOfMediaPools", &self.dwNumberOfMediaPools)
            .finish()
    }
}
impl ::core::default::Default for NTMS_MEDIATYPEINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_MEDIATYPEINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MediaType == other.MediaType && self.NumberOfSides == other.NumberOfSides && self.ReadWriteCharacteristics == other.ReadWriteCharacteristics && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for NTMS_MEDIATYPEINFORMATION {}
impl ::core::fmt::Debug for NTMS_MEDIATYPEINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_MEDIATYPEINFORMATION").field("MediaType", &self.MediaType).field("NumberOfSides", &self.NumberOfSides).field("ReadWriteCharacteristics", &self.ReadWriteCharacteristics).field("DeviceType", &self.DeviceType).finish()
    }
}
impl ::core::default::Default for NTMS_MOUNT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_MOUNT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpReserved == other.lpReserved
    }
}
impl ::core::cmp::Eq for NTMS_MOUNT_INFORMATION {}
impl ::core::fmt::Debug for NTMS_MOUNT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_MOUNT_INFORMATION").field("dwSize", &self.dwSize).field("lpReserved", &self.lpReserved).finish()
    }
}
impl ::core::default::Default for NTMS_NOTIFICATIONINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_NOTIFICATIONINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwOperation == other.dwOperation && self.ObjectId == other.ObjectId
    }
}
impl ::core::cmp::Eq for NTMS_NOTIFICATIONINFORMATION {}
impl ::core::fmt::Debug for NTMS_NOTIFICATIONINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_NOTIFICATIONINFORMATION").field("dwOperation", &self.dwOperation).field("ObjectId", &self.ObjectId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_OBJECTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_OBJECTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NTMS_OMID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NTMS_OMID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NTMS_OMID_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_OPREQUESTINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_OPREQUESTINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request && self.Submitted == other.Submitted && self.State == other.State && self.szMessage == other.szMessage && self.Arg1Type == other.Arg1Type && self.Arg1 == other.Arg1 && self.Arg2Type == other.Arg2Type && self.Arg2 == other.Arg2 && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_OPREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_OPREQUESTINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_OPREQUESTINFORMATIONA").field("Request", &self.Request).field("Submitted", &self.Submitted).field("State", &self.State).field("szMessage", &self.szMessage).field("Arg1Type", &self.Arg1Type).field("Arg1", &self.Arg1).field("Arg2Type", &self.Arg2Type).field("Arg2", &self.Arg2).field("szApplication", &self.szApplication).field("szUser", &self.szUser).field("szComputer", &self.szComputer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_OPREQUESTINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_OPREQUESTINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Request == other.Request && self.Submitted == other.Submitted && self.State == other.State && self.szMessage == other.szMessage && self.Arg1Type == other.Arg1Type && self.Arg1 == other.Arg1 && self.Arg2Type == other.Arg2Type && self.Arg2 == other.Arg2 && self.szApplication == other.szApplication && self.szUser == other.szUser && self.szComputer == other.szComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_OPREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_OPREQUESTINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_OPREQUESTINFORMATIONW").field("Request", &self.Request).field("Submitted", &self.Submitted).field("State", &self.State).field("szMessage", &self.szMessage).field("Arg1Type", &self.Arg1Type).field("Arg1", &self.Arg1).field("Arg2Type", &self.Arg2Type).field("Arg2", &self.Arg2).field("szApplication", &self.szApplication).field("szUser", &self.szUser).field("szComputer", &self.szComputer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_PARTITIONINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_PARTITIONINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia && self.LogicalMedia == other.LogicalMedia && self.State == other.State && self.Side == other.Side && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength && self.OmidLabelId == other.OmidLabelId && self.szOmidLabelType == other.szOmidLabelType && self.szOmidLabelInfo == other.szOmidLabelInfo && self.dwMountCount == other.dwMountCount && self.dwAllocateCount == other.dwAllocateCount && self.Capacity == other.Capacity
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_PARTITIONINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_PARTITIONINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_PARTITIONINFORMATIONA")
            .field("PhysicalMedia", &self.PhysicalMedia)
            .field("LogicalMedia", &self.LogicalMedia)
            .field("State", &self.State)
            .field("Side", &self.Side)
            .field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength)
            .field("OmidLabelId", &self.OmidLabelId)
            .field("szOmidLabelType", &self.szOmidLabelType)
            .field("szOmidLabelInfo", &self.szOmidLabelInfo)
            .field("dwMountCount", &self.dwMountCount)
            .field("dwAllocateCount", &self.dwAllocateCount)
            .field("Capacity", &self.Capacity)
            .finish()
    }
}
impl ::core::default::Default for NTMS_PARTITIONINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_PARTITIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalMedia == other.PhysicalMedia && self.LogicalMedia == other.LogicalMedia && self.State == other.State && self.Side == other.Side && self.dwOmidLabelIdLength == other.dwOmidLabelIdLength && self.OmidLabelId == other.OmidLabelId && self.szOmidLabelType == other.szOmidLabelType && self.szOmidLabelInfo == other.szOmidLabelInfo && self.dwMountCount == other.dwMountCount && self.dwAllocateCount == other.dwAllocateCount && self.Capacity == other.Capacity
    }
}
impl ::core::cmp::Eq for NTMS_PARTITIONINFORMATIONW {}
impl ::core::fmt::Debug for NTMS_PARTITIONINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_PARTITIONINFORMATIONW")
            .field("PhysicalMedia", &self.PhysicalMedia)
            .field("LogicalMedia", &self.LogicalMedia)
            .field("State", &self.State)
            .field("Side", &self.Side)
            .field("dwOmidLabelIdLength", &self.dwOmidLabelIdLength)
            .field("OmidLabelId", &self.OmidLabelId)
            .field("szOmidLabelType", &self.szOmidLabelType)
            .field("szOmidLabelInfo", &self.szOmidLabelInfo)
            .field("dwMountCount", &self.dwMountCount)
            .field("dwAllocateCount", &self.dwAllocateCount)
            .field("Capacity", &self.Capacity)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NTMS_PMIDINFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NTMS_PMIDINFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary && self.MediaPool == other.MediaPool && self.Location == other.Location && self.LocationType == other.LocationType && self.MediaType == other.MediaType && self.HomeSlot == other.HomeSlot && self.szBarCode == other.szBarCode && self.BarCodeState == other.BarCodeState && self.szSequenceNumber == other.szSequenceNumber && self.MediaState == other.MediaState && self.dwNumberOfPartitions == other.dwNumberOfPartitions && self.dwMediaTypeCode == other.dwMediaTypeCode && self.dwDensityCode == other.dwDensityCode && self.MountedPartition == other.MountedPartition
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NTMS_PMIDINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NTMS_PMIDINFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_PMIDINFORMATIONA")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .field("dwMediaTypeCode", &self.dwMediaTypeCode)
            .field("dwDensityCode", &self.dwDensityCode)
            .field("MountedPartition", &self.MountedPartition)
            .finish()
    }
}
impl ::core::default::Default for NTMS_PMIDINFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_PMIDINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentLibrary == other.CurrentLibrary && self.MediaPool == other.MediaPool && self.Location == other.Location && self.LocationType == other.LocationType && self.MediaType == other.MediaType && self.HomeSlot == other.HomeSlot && self.szBarCode == other.szBarCode && self.BarCodeState == other.BarCodeState && self.szSequenceNumber == other.szSequenceNumber && self.MediaState == other.MediaState && self.dwNumberOfPartitions == other.dwNumberOfPartitions && self.dwMediaTypeCode == other.dwMediaTypeCode && self.dwDensityCode == other.dwDensityCode && self.MountedPartition == other.MountedPartition
    }
}
impl ::core::cmp::Eq for NTMS_PMIDINFORMATIONW {}
impl ::core::fmt::Debug for NTMS_PMIDINFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_PMIDINFORMATIONW")
            .field("CurrentLibrary", &self.CurrentLibrary)
            .field("MediaPool", &self.MediaPool)
            .field("Location", &self.Location)
            .field("LocationType", &self.LocationType)
            .field("MediaType", &self.MediaType)
            .field("HomeSlot", &self.HomeSlot)
            .field("szBarCode", &self.szBarCode)
            .field("BarCodeState", &self.BarCodeState)
            .field("szSequenceNumber", &self.szSequenceNumber)
            .field("MediaState", &self.MediaState)
            .field("dwNumberOfPartitions", &self.dwNumberOfPartitions)
            .field("dwMediaTypeCode", &self.dwMediaTypeCode)
            .field("dwDensityCode", &self.dwDensityCode)
            .field("MountedPartition", &self.MountedPartition)
            .finish()
    }
}
impl ::core::default::Default for NTMS_STORAGESLOTINFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTMS_STORAGESLOTINFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Number == other.Number && self.State == other.State && self.Library == other.Library
    }
}
impl ::core::cmp::Eq for NTMS_STORAGESLOTINFORMATION {}
impl ::core::fmt::Debug for NTMS_STORAGESLOTINFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTMS_STORAGESLOTINFORMATION").field("Number", &self.Number).field("State", &self.State).field("Library", &self.Library).finish()
    }
}
impl ::core::default::Default for NT_CREATE_FILE_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NT_CREATE_FILE_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NT_CREATE_FILE_DISPOSITION").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsAccessMask {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsAccessMask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAccessMask").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsAllocateOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsAllocateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAllocateOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsAllocationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsAllocationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAllocationPolicy").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsAsyncOperations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsAsyncOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAsyncOperations").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsAsyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsAsyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsAsyncStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsBarCodeState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsBarCodeState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsBarCodeState").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsCreateNtmsMediaOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsCreateNtmsMediaOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsCreateNtmsMediaOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsCreateOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsCreateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsCreateOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsDeallocationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsDeallocationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDeallocationPolicy").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsDismountOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsDismountOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDismountOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsDoorState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsDoorState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDoorState").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsDriveState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsDriveState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDriveState").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsDriveType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsDriveType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsDriveType").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsEjectOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsEjectOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsEjectOperation").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsEnumerateOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsEnumerateOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsEnumerateOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsInjectOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsInjectOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsInjectOperation").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsInventoryMethod {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsInventoryMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsInventoryMethod").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsLibRequestFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsLibRequestFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLibRequestFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsLibraryFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsLibraryFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLibraryFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsLibraryType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsLibraryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLibraryType").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsLmOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsLmOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLmOperation").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsLmState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsLmState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsLmState").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsMediaPoolPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsMediaPoolPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsMediaPoolPolicy").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsMediaState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsMediaState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsMediaState").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsMountOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsMountOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsMountOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsMountPriority {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsMountPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsMountPriority").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsNotificationOperations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsNotificationOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsNotificationOperations").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsObjectsTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsObjectsTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsObjectsTypes").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsOpRequestFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsOpRequestFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsOpRequestFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsOperationalState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsOperationalState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsOperationalState").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsOpreqCommand {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsOpreqCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsOpreqCommand").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsOpreqState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsOpreqState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsOpreqState").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsPartitionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsPartitionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsPartitionState").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsPoolType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsPoolType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsPoolType").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsPortContent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsPortContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsPortContent").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsPortPosition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsPortPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsPortPosition").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsReadWriteCharacteristics {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsReadWriteCharacteristics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsReadWriteCharacteristics").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsSessionOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsSessionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsSessionOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsSlotState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsSlotState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsSlotState").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsUIOperations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsUIOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsUIOperations").field(&self.0).finish()
    }
}
impl ::core::default::Default for NtmsUITypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NtmsUITypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NtmsUITypes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cBytes == other.cBytes && self.fFixedDisk == other.fFixedDisk && self.nErrCode == other.nErrCode && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.szPathName == other.szPathName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OFSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFSTRUCT").field("cBytes", &self.cBytes).field("fFixedDisk", &self.fFixedDisk).field("nErrCode", &self.nErrCode).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("szPathName", &self.szPathName).finish()
    }
}
impl ::core::default::Default for PREPARE_TAPE_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PREPARE_TAPE_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PREPARE_TAPE_OPERATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for PRIORITY_HINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PRIORITY_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRIORITY_HINT").field(&self.0).finish()
    }
}
impl ::core::default::Default for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for READ_DIRECTORY_NOTIFY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("READ_DIRECTORY_NOTIFY_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for REPARSE_GUID_DATA_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPARSE_GUID_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.ReparseTag == other.ReparseTag && self.ReparseDataLength == other.ReparseDataLength && self.Reserved == other.Reserved && self.ReparseGuid == other.ReparseGuid && self.GenericReparseBuffer == other.GenericReparseBuffer
    }
}
impl ::core::cmp::Eq for REPARSE_GUID_DATA_BUFFER {}
impl ::core::fmt::Debug for REPARSE_GUID_DATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPARSE_GUID_DATA_BUFFER").field("ReparseTag", &self.ReparseTag).field("ReparseDataLength", &self.ReparseDataLength).field("Reserved", &self.Reserved).field("ReparseGuid", &self.ReparseGuid).field("GenericReparseBuffer", &self.GenericReparseBuffer).finish()
    }
}
impl ::core::default::Default for REPARSE_GUID_DATA_BUFFER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPARSE_GUID_DATA_BUFFER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.DataBuffer == other.DataBuffer
    }
}
impl ::core::cmp::Eq for REPARSE_GUID_DATA_BUFFER_0 {}
impl ::core::fmt::Debug for REPARSE_GUID_DATA_BUFFER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPARSE_GUID_DATA_BUFFER_0").field("DataBuffer", &self.DataBuffer).finish()
    }
}
impl ::core::default::Default for REPLACE_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REPLACE_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPLACE_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REPLACE_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REPLACE_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REPLACE_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_ALIAS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_ALIAS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.srvai0_alias == other.srvai0_alias && self.srvai0_target == other.srvai0_target && self.srvai0_default == other.srvai0_default && self.srvai0_reserved == other.srvai0_reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_ALIAS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_ALIAS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_ALIAS_INFO_0").field("srvai0_alias", &self.srvai0_alias).field("srvai0_target", &self.srvai0_target).field("srvai0_default", &self.srvai0_default).field("srvai0_reserved", &self.srvai0_reserved).finish()
    }
}
impl ::core::default::Default for SERVER_CERTIFICATE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_CERTIFICATE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.srvci0_name == other.srvci0_name && self.srvci0_subject == other.srvci0_subject && self.srvci0_issuer == other.srvci0_issuer && self.srvci0_thumbprint == other.srvci0_thumbprint && self.srvci0_friendlyname == other.srvci0_friendlyname && self.srvci0_notbefore == other.srvci0_notbefore && self.srvci0_notafter == other.srvci0_notafter && self.srvci0_storelocation == other.srvci0_storelocation && self.srvci0_storename == other.srvci0_storename && self.srvci0_renewalchain == other.srvci0_renewalchain && self.srvci0_type == other.srvci0_type && self.srvci0_flags == other.srvci0_flags
    }
}
impl ::core::cmp::Eq for SERVER_CERTIFICATE_INFO_0 {}
impl ::core::fmt::Debug for SERVER_CERTIFICATE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_CERTIFICATE_INFO_0")
            .field("srvci0_name", &self.srvci0_name)
            .field("srvci0_subject", &self.srvci0_subject)
            .field("srvci0_issuer", &self.srvci0_issuer)
            .field("srvci0_thumbprint", &self.srvci0_thumbprint)
            .field("srvci0_friendlyname", &self.srvci0_friendlyname)
            .field("srvci0_notbefore", &self.srvci0_notbefore)
            .field("srvci0_notafter", &self.srvci0_notafter)
            .field("srvci0_storelocation", &self.srvci0_storelocation)
            .field("srvci0_storename", &self.srvci0_storename)
            .field("srvci0_renewalchain", &self.srvci0_renewalchain)
            .field("srvci0_type", &self.srvci0_type)
            .field("srvci0_flags", &self.srvci0_flags)
            .finish()
    }
}
impl ::core::default::Default for SERVER_CERTIFICATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SERVER_CERTIFICATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVER_CERTIFICATE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SESSION_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SESSION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi0_cname == other.sesi0_cname
    }
}
impl ::core::cmp::Eq for SESSION_INFO_0 {}
impl ::core::fmt::Debug for SESSION_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_0").field("sesi0_cname", &self.sesi0_cname).finish()
    }
}
impl ::core::default::Default for SESSION_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SESSION_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi1_cname == other.sesi1_cname && self.sesi1_username == other.sesi1_username && self.sesi1_num_opens == other.sesi1_num_opens && self.sesi1_time == other.sesi1_time && self.sesi1_idle_time == other.sesi1_idle_time && self.sesi1_user_flags == other.sesi1_user_flags
    }
}
impl ::core::cmp::Eq for SESSION_INFO_1 {}
impl ::core::fmt::Debug for SESSION_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_1").field("sesi1_cname", &self.sesi1_cname).field("sesi1_username", &self.sesi1_username).field("sesi1_num_opens", &self.sesi1_num_opens).field("sesi1_time", &self.sesi1_time).field("sesi1_idle_time", &self.sesi1_idle_time).field("sesi1_user_flags", &self.sesi1_user_flags).finish()
    }
}
impl ::core::default::Default for SESSION_INFO_10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SESSION_INFO_10 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi10_cname == other.sesi10_cname && self.sesi10_username == other.sesi10_username && self.sesi10_time == other.sesi10_time && self.sesi10_idle_time == other.sesi10_idle_time
    }
}
impl ::core::cmp::Eq for SESSION_INFO_10 {}
impl ::core::fmt::Debug for SESSION_INFO_10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_10").field("sesi10_cname", &self.sesi10_cname).field("sesi10_username", &self.sesi10_username).field("sesi10_time", &self.sesi10_time).field("sesi10_idle_time", &self.sesi10_idle_time).finish()
    }
}
impl ::core::default::Default for SESSION_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SESSION_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi2_cname == other.sesi2_cname && self.sesi2_username == other.sesi2_username && self.sesi2_num_opens == other.sesi2_num_opens && self.sesi2_time == other.sesi2_time && self.sesi2_idle_time == other.sesi2_idle_time && self.sesi2_user_flags == other.sesi2_user_flags && self.sesi2_cltype_name == other.sesi2_cltype_name
    }
}
impl ::core::cmp::Eq for SESSION_INFO_2 {}
impl ::core::fmt::Debug for SESSION_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_2").field("sesi2_cname", &self.sesi2_cname).field("sesi2_username", &self.sesi2_username).field("sesi2_num_opens", &self.sesi2_num_opens).field("sesi2_time", &self.sesi2_time).field("sesi2_idle_time", &self.sesi2_idle_time).field("sesi2_user_flags", &self.sesi2_user_flags).field("sesi2_cltype_name", &self.sesi2_cltype_name).finish()
    }
}
impl ::core::default::Default for SESSION_INFO_502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SESSION_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        self.sesi502_cname == other.sesi502_cname && self.sesi502_username == other.sesi502_username && self.sesi502_num_opens == other.sesi502_num_opens && self.sesi502_time == other.sesi502_time && self.sesi502_idle_time == other.sesi502_idle_time && self.sesi502_user_flags == other.sesi502_user_flags && self.sesi502_cltype_name == other.sesi502_cltype_name && self.sesi502_transport == other.sesi502_transport
    }
}
impl ::core::cmp::Eq for SESSION_INFO_502 {}
impl ::core::fmt::Debug for SESSION_INFO_502 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_INFO_502").field("sesi502_cname", &self.sesi502_cname).field("sesi502_username", &self.sesi502_username).field("sesi502_num_opens", &self.sesi502_num_opens).field("sesi502_time", &self.sesi502_time).field("sesi502_idle_time", &self.sesi502_idle_time).field("sesi502_user_flags", &self.sesi502_user_flags).field("sesi502_cltype_name", &self.sesi502_cltype_name).field("sesi502_transport", &self.sesi502_transport).finish()
    }
}
impl ::core::default::Default for SESSION_INFO_USER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SESSION_INFO_USER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SESSION_INFO_USER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SET_FILE_POINTER_MOVE_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SET_FILE_POINTER_MOVE_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_FILE_POINTER_MOVE_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHARE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.shi0_netname == other.shi0_netname
    }
}
impl ::core::cmp::Eq for SHARE_INFO_0 {}
impl ::core::fmt::Debug for SHARE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_0").field("shi0_netname", &self.shi0_netname).finish()
    }
}
impl ::core::default::Default for SHARE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1_netname == other.shi1_netname && self.shi1_type == other.shi1_type && self.shi1_remark == other.shi1_remark
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1 {}
impl ::core::fmt::Debug for SHARE_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1").field("shi1_netname", &self.shi1_netname).field("shi1_type", &self.shi1_type).field("shi1_remark", &self.shi1_remark).finish()
    }
}
impl ::core::default::Default for SHARE_INFO_1004 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_1004 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1004_remark == other.shi1004_remark
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1004 {}
impl ::core::fmt::Debug for SHARE_INFO_1004 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1004").field("shi1004_remark", &self.shi1004_remark).finish()
    }
}
impl ::core::default::Default for SHARE_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1005_flags == other.shi1005_flags
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1005 {}
impl ::core::fmt::Debug for SHARE_INFO_1005 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1005").field("shi1005_flags", &self.shi1005_flags).finish()
    }
}
impl ::core::default::Default for SHARE_INFO_1006 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_1006 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1006_max_uses == other.shi1006_max_uses
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1006 {}
impl ::core::fmt::Debug for SHARE_INFO_1006 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1006").field("shi1006_max_uses", &self.shi1006_max_uses).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for SHARE_INFO_1501 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for SHARE_INFO_1501 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1501_reserved == other.shi1501_reserved && self.shi1501_security_descriptor == other.shi1501_security_descriptor
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for SHARE_INFO_1501 {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for SHARE_INFO_1501 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1501").field("shi1501_reserved", &self.shi1501_reserved).field("shi1501_security_descriptor", &self.shi1501_security_descriptor).finish()
    }
}
impl ::core::default::Default for SHARE_INFO_1503 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_1503 {
    fn eq(&self, other: &Self) -> bool {
        self.shi1503_sharefilter == other.shi1503_sharefilter
    }
}
impl ::core::cmp::Eq for SHARE_INFO_1503 {}
impl ::core::fmt::Debug for SHARE_INFO_1503 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_1503").field("shi1503_sharefilter", &self.shi1503_sharefilter).finish()
    }
}
impl ::core::default::Default for SHARE_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.shi2_netname == other.shi2_netname && self.shi2_type == other.shi2_type && self.shi2_remark == other.shi2_remark && self.shi2_permissions == other.shi2_permissions && self.shi2_max_uses == other.shi2_max_uses && self.shi2_current_uses == other.shi2_current_uses && self.shi2_path == other.shi2_path && self.shi2_passwd == other.shi2_passwd
    }
}
impl ::core::cmp::Eq for SHARE_INFO_2 {}
impl ::core::fmt::Debug for SHARE_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_2").field("shi2_netname", &self.shi2_netname).field("shi2_type", &self.shi2_type).field("shi2_remark", &self.shi2_remark).field("shi2_permissions", &self.shi2_permissions).field("shi2_max_uses", &self.shi2_max_uses).field("shi2_current_uses", &self.shi2_current_uses).field("shi2_path", &self.shi2_path).field("shi2_passwd", &self.shi2_passwd).finish()
    }
}
impl ::core::default::Default for SHARE_INFO_501 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHARE_INFO_501 {
    fn eq(&self, other: &Self) -> bool {
        self.shi501_netname == other.shi501_netname && self.shi501_type == other.shi501_type && self.shi501_remark == other.shi501_remark && self.shi501_flags == other.shi501_flags
    }
}
impl ::core::cmp::Eq for SHARE_INFO_501 {}
impl ::core::fmt::Debug for SHARE_INFO_501 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_501").field("shi501_netname", &self.shi501_netname).field("shi501_type", &self.shi501_type).field("shi501_remark", &self.shi501_remark).field("shi501_flags", &self.shi501_flags).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for SHARE_INFO_502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for SHARE_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        self.shi502_netname == other.shi502_netname && self.shi502_type == other.shi502_type && self.shi502_remark == other.shi502_remark && self.shi502_permissions == other.shi502_permissions && self.shi502_max_uses == other.shi502_max_uses && self.shi502_current_uses == other.shi502_current_uses && self.shi502_path == other.shi502_path && self.shi502_passwd == other.shi502_passwd && self.shi502_reserved == other.shi502_reserved && self.shi502_security_descriptor == other.shi502_security_descriptor
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for SHARE_INFO_502 {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for SHARE_INFO_502 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_502")
            .field("shi502_netname", &self.shi502_netname)
            .field("shi502_type", &self.shi502_type)
            .field("shi502_remark", &self.shi502_remark)
            .field("shi502_permissions", &self.shi502_permissions)
            .field("shi502_max_uses", &self.shi502_max_uses)
            .field("shi502_current_uses", &self.shi502_current_uses)
            .field("shi502_path", &self.shi502_path)
            .field("shi502_passwd", &self.shi502_passwd)
            .field("shi502_reserved", &self.shi502_reserved)
            .field("shi502_security_descriptor", &self.shi502_security_descriptor)
            .finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for SHARE_INFO_503 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for SHARE_INFO_503 {
    fn eq(&self, other: &Self) -> bool {
        self.shi503_netname == other.shi503_netname && self.shi503_type == other.shi503_type && self.shi503_remark == other.shi503_remark && self.shi503_permissions == other.shi503_permissions && self.shi503_max_uses == other.shi503_max_uses && self.shi503_current_uses == other.shi503_current_uses && self.shi503_path == other.shi503_path && self.shi503_passwd == other.shi503_passwd && self.shi503_servername == other.shi503_servername && self.shi503_reserved == other.shi503_reserved && self.shi503_security_descriptor == other.shi503_security_descriptor
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for SHARE_INFO_503 {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for SHARE_INFO_503 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHARE_INFO_503")
            .field("shi503_netname", &self.shi503_netname)
            .field("shi503_type", &self.shi503_type)
            .field("shi503_remark", &self.shi503_remark)
            .field("shi503_permissions", &self.shi503_permissions)
            .field("shi503_max_uses", &self.shi503_max_uses)
            .field("shi503_current_uses", &self.shi503_current_uses)
            .field("shi503_path", &self.shi503_path)
            .field("shi503_passwd", &self.shi503_passwd)
            .field("shi503_servername", &self.shi503_servername)
            .field("shi503_reserved", &self.shi503_reserved)
            .field("shi503_security_descriptor", &self.shi503_security_descriptor)
            .finish()
    }
}
impl ::core::default::Default for SHARE_INFO_PERMISSIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHARE_INFO_PERMISSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHARE_INFO_PERMISSIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHARE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHARE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHARE_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHARE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHARE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHARE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHARE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHARE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for STAT_SERVER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STAT_SERVER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.sts0_start == other.sts0_start
            && self.sts0_fopens == other.sts0_fopens
            && self.sts0_devopens == other.sts0_devopens
            && self.sts0_jobsqueued == other.sts0_jobsqueued
            && self.sts0_sopens == other.sts0_sopens
            && self.sts0_stimedout == other.sts0_stimedout
            && self.sts0_serrorout == other.sts0_serrorout
            && self.sts0_pwerrors == other.sts0_pwerrors
            && self.sts0_permerrors == other.sts0_permerrors
            && self.sts0_syserrors == other.sts0_syserrors
            && self.sts0_bytessent_low == other.sts0_bytessent_low
            && self.sts0_bytessent_high == other.sts0_bytessent_high
            && self.sts0_bytesrcvd_low == other.sts0_bytesrcvd_low
            && self.sts0_bytesrcvd_high == other.sts0_bytesrcvd_high
            && self.sts0_avresponse == other.sts0_avresponse
            && self.sts0_reqbufneed == other.sts0_reqbufneed
            && self.sts0_bigbufneed == other.sts0_bigbufneed
    }
}
impl ::core::cmp::Eq for STAT_SERVER_0 {}
impl ::core::fmt::Debug for STAT_SERVER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STAT_SERVER_0")
            .field("sts0_start", &self.sts0_start)
            .field("sts0_fopens", &self.sts0_fopens)
            .field("sts0_devopens", &self.sts0_devopens)
            .field("sts0_jobsqueued", &self.sts0_jobsqueued)
            .field("sts0_sopens", &self.sts0_sopens)
            .field("sts0_stimedout", &self.sts0_stimedout)
            .field("sts0_serrorout", &self.sts0_serrorout)
            .field("sts0_pwerrors", &self.sts0_pwerrors)
            .field("sts0_permerrors", &self.sts0_permerrors)
            .field("sts0_syserrors", &self.sts0_syserrors)
            .field("sts0_bytessent_low", &self.sts0_bytessent_low)
            .field("sts0_bytessent_high", &self.sts0_bytessent_high)
            .field("sts0_bytesrcvd_low", &self.sts0_bytesrcvd_low)
            .field("sts0_bytesrcvd_high", &self.sts0_bytesrcvd_high)
            .field("sts0_avresponse", &self.sts0_avresponse)
            .field("sts0_reqbufneed", &self.sts0_reqbufneed)
            .field("sts0_bigbufneed", &self.sts0_bigbufneed)
            .finish()
    }
}
impl ::core::default::Default for STAT_WORKSTATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STAT_WORKSTATION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.StatisticsStartTime == other.StatisticsStartTime
            && self.BytesReceived == other.BytesReceived
            && self.SmbsReceived == other.SmbsReceived
            && self.PagingReadBytesRequested == other.PagingReadBytesRequested
            && self.NonPagingReadBytesRequested == other.NonPagingReadBytesRequested
            && self.CacheReadBytesRequested == other.CacheReadBytesRequested
            && self.NetworkReadBytesRequested == other.NetworkReadBytesRequested
            && self.BytesTransmitted == other.BytesTransmitted
            && self.SmbsTransmitted == other.SmbsTransmitted
            && self.PagingWriteBytesRequested == other.PagingWriteBytesRequested
            && self.NonPagingWriteBytesRequested == other.NonPagingWriteBytesRequested
            && self.CacheWriteBytesRequested == other.CacheWriteBytesRequested
            && self.NetworkWriteBytesRequested == other.NetworkWriteBytesRequested
            && self.InitiallyFailedOperations == other.InitiallyFailedOperations
            && self.FailedCompletionOperations == other.FailedCompletionOperations
            && self.ReadOperations == other.ReadOperations
            && self.RandomReadOperations == other.RandomReadOperations
            && self.ReadSmbs == other.ReadSmbs
            && self.LargeReadSmbs == other.LargeReadSmbs
            && self.SmallReadSmbs == other.SmallReadSmbs
            && self.WriteOperations == other.WriteOperations
            && self.RandomWriteOperations == other.RandomWriteOperations
            && self.WriteSmbs == other.WriteSmbs
            && self.LargeWriteSmbs == other.LargeWriteSmbs
            && self.SmallWriteSmbs == other.SmallWriteSmbs
            && self.RawReadsDenied == other.RawReadsDenied
            && self.RawWritesDenied == other.RawWritesDenied
            && self.NetworkErrors == other.NetworkErrors
            && self.Sessions == other.Sessions
            && self.FailedSessions == other.FailedSessions
            && self.Reconnects == other.Reconnects
            && self.CoreConnects == other.CoreConnects
            && self.Lanman20Connects == other.Lanman20Connects
            && self.Lanman21Connects == other.Lanman21Connects
            && self.LanmanNtConnects == other.LanmanNtConnects
            && self.ServerDisconnects == other.ServerDisconnects
            && self.HungSessions == other.HungSessions
            && self.UseCount == other.UseCount
            && self.FailedUseCount == other.FailedUseCount
            && self.CurrentCommands == other.CurrentCommands
    }
}
impl ::core::cmp::Eq for STAT_WORKSTATION_0 {}
impl ::core::fmt::Debug for STAT_WORKSTATION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STAT_WORKSTATION_0")
            .field("StatisticsStartTime", &self.StatisticsStartTime)
            .field("BytesReceived", &self.BytesReceived)
            .field("SmbsReceived", &self.SmbsReceived)
            .field("PagingReadBytesRequested", &self.PagingReadBytesRequested)
            .field("NonPagingReadBytesRequested", &self.NonPagingReadBytesRequested)
            .field("CacheReadBytesRequested", &self.CacheReadBytesRequested)
            .field("NetworkReadBytesRequested", &self.NetworkReadBytesRequested)
            .field("BytesTransmitted", &self.BytesTransmitted)
            .field("SmbsTransmitted", &self.SmbsTransmitted)
            .field("PagingWriteBytesRequested", &self.PagingWriteBytesRequested)
            .field("NonPagingWriteBytesRequested", &self.NonPagingWriteBytesRequested)
            .field("CacheWriteBytesRequested", &self.CacheWriteBytesRequested)
            .field("NetworkWriteBytesRequested", &self.NetworkWriteBytesRequested)
            .field("InitiallyFailedOperations", &self.InitiallyFailedOperations)
            .field("FailedCompletionOperations", &self.FailedCompletionOperations)
            .field("ReadOperations", &self.ReadOperations)
            .field("RandomReadOperations", &self.RandomReadOperations)
            .field("ReadSmbs", &self.ReadSmbs)
            .field("LargeReadSmbs", &self.LargeReadSmbs)
            .field("SmallReadSmbs", &self.SmallReadSmbs)
            .field("WriteOperations", &self.WriteOperations)
            .field("RandomWriteOperations", &self.RandomWriteOperations)
            .field("WriteSmbs", &self.WriteSmbs)
            .field("LargeWriteSmbs", &self.LargeWriteSmbs)
            .field("SmallWriteSmbs", &self.SmallWriteSmbs)
            .field("RawReadsDenied", &self.RawReadsDenied)
            .field("RawWritesDenied", &self.RawWritesDenied)
            .field("NetworkErrors", &self.NetworkErrors)
            .field("Sessions", &self.Sessions)
            .field("FailedSessions", &self.FailedSessions)
            .field("Reconnects", &self.Reconnects)
            .field("CoreConnects", &self.CoreConnects)
            .field("Lanman20Connects", &self.Lanman20Connects)
            .field("Lanman21Connects", &self.Lanman21Connects)
            .field("LanmanNtConnects", &self.LanmanNtConnects)
            .field("ServerDisconnects", &self.ServerDisconnects)
            .field("HungSessions", &self.HungSessions)
            .field("UseCount", &self.UseCount)
            .field("FailedUseCount", &self.FailedUseCount)
            .field("CurrentCommands", &self.CurrentCommands)
            .finish()
    }
}
impl ::core::default::Default for STORAGE_BUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_BUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_BUS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STREAM_INFO_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STREAM_INFO_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STREAM_INFO_LEVELS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYMBOLIC_LINK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYMBOLIC_LINK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYMBOLIC_LINK_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYMBOLIC_LINK_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYMBOLIC_LINK_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYMBOLIC_LINK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TAPEMARK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPEMARK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPEMARK_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAPE_ERASE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAPE_ERASE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Immediate == other.Immediate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAPE_ERASE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAPE_ERASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_ERASE").field("Type", &self.Type).field("Immediate", &self.Immediate).finish()
    }
}
impl ::core::default::Default for TAPE_GET_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TAPE_GET_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Partition == other.Partition && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for TAPE_GET_POSITION {}
impl ::core::fmt::Debug for TAPE_GET_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_GET_POSITION").field("Type", &self.Type).field("Partition", &self.Partition).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for TAPE_INFORMATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPE_INFORMATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPE_INFORMATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TAPE_POSITION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPE_POSITION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPE_POSITION_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for TAPE_POSITION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPE_POSITION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPE_POSITION_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAPE_PREPARE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAPE_PREPARE {
    fn eq(&self, other: &Self) -> bool {
        self.Operation == other.Operation && self.Immediate == other.Immediate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAPE_PREPARE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAPE_PREPARE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_PREPARE").field("Operation", &self.Operation).field("Immediate", &self.Immediate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAPE_SET_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAPE_SET_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Method == other.Method && self.Partition == other.Partition && self.Offset == other.Offset && self.Immediate == other.Immediate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAPE_SET_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAPE_SET_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_SET_POSITION").field("Method", &self.Method).field("Partition", &self.Partition).field("Offset", &self.Offset).field("Immediate", &self.Immediate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAPE_WRITE_MARKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAPE_WRITE_MARKS {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Count == other.Count && self.Immediate == other.Immediate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAPE_WRITE_MARKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAPE_WRITE_MARKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_WRITE_MARKS").field("Type", &self.Type).field("Count", &self.Count).field("Immediate", &self.Immediate).finish()
    }
}
impl ::core::default::Default for TRANSACTION_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.TransactionKey == other.TransactionKey && self.TransactionNotification == other.TransactionNotification && self.TmVirtualClock == other.TmVirtualClock && self.ArgumentLength == other.ArgumentLength
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION {}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION").field("TransactionKey", &self.TransactionKey).field("TransactionNotification", &self.TransactionNotification).field("TmVirtualClock", &self.TmVirtualClock).field("ArgumentLength", &self.ArgumentLength).finish()
    }
}
impl ::core::default::Default for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.MarshalCookie == other.MarshalCookie && self.UOW == other.UOW
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT").field("MarshalCookie", &self.MarshalCookie).field("UOW", &self.UOW).finish()
    }
}
impl ::core::default::Default for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.PropagationCookie == other.PropagationCookie && self.UOW == other.UOW && self.TmIdentity == other.TmIdentity && self.BufferLength == other.BufferLength
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT").field("PropagationCookie", &self.PropagationCookie).field("UOW", &self.UOW).field("TmIdentity", &self.TmIdentity).field("BufferLength", &self.BufferLength).finish()
    }
}
impl ::core::default::Default for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.EnlistmentId == other.EnlistmentId && self.UOW == other.UOW
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT").field("EnlistmentId", &self.EnlistmentId).field("UOW", &self.UOW).finish()
    }
}
impl ::core::default::Default for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.SavepointId == other.SavepointId
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT").field("SavepointId", &self.SavepointId).finish()
    }
}
impl ::core::default::Default for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.TmIdentity == other.TmIdentity && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {}
impl ::core::fmt::Debug for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT").field("TmIdentity", &self.TmIdentity).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for TRANSACTION_OUTCOME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSACTION_OUTCOME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSACTION_OUTCOME").field(&self.0).finish()
    }
}
impl ::core::default::Default for TXFS_MINIVERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TXFS_MINIVERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXFS_MINIVERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for TXF_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TXF_LOG_RECORD_AFFECTED_FILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TXF_LOG_RECORD_BASE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TXF_LOG_RECORD_TRUNCATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TXF_LOG_RECORD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TXF_LOG_RECORD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXF_LOG_RECORD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TXF_LOG_RECORD_WRITE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VER_FIND_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VER_FIND_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_FIND_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VER_FIND_FILE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VER_FIND_FILE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_FIND_FILE_STATUS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VER_FIND_FILE_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VER_FIND_FILE_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VER_FIND_FILE_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for VER_INSTALL_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VER_INSTALL_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_INSTALL_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VER_INSTALL_FILE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VER_INSTALL_FILE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_INSTALL_FILE_STATUS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VER_INSTALL_FILE_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VER_INSTALL_FILE_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VER_INSTALL_FILE_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.RequestsPerPeriod == other.RequestsPerPeriod && self.Period == other.Period && self.RetryFailures == other.RetryFailures && self.Discardable == other.Discardable && self.Reserved1 == other.Reserved1 && self.LowestByteOffset == other.LowestByteOffset && self.HighestByteOffset == other.HighestByteOffset && self.AccessType == other.AccessType && self.AccessMode == other.AccessMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VOLUME_ALLOCATE_BC_STREAM_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_ALLOCATE_BC_STREAM_INPUT").field("Version", &self.Version).field("RequestsPerPeriod", &self.RequestsPerPeriod).field("Period", &self.Period).field("RetryFailures", &self.RetryFailures).field("Discardable", &self.Discardable).field("Reserved1", &self.Reserved1).field("LowestByteOffset", &self.LowestByteOffset).field("HighestByteOffset", &self.HighestByteOffset).field("AccessType", &self.AccessType).field("AccessMode", &self.AccessMode).finish()
    }
}
impl ::core::default::Default for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.RequestSize == other.RequestSize && self.NumOutStandingRequests == other.NumOutStandingRequests
    }
}
impl ::core::cmp::Eq for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {}
impl ::core::fmt::Debug for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_ALLOCATE_BC_STREAM_OUTPUT").field("RequestSize", &self.RequestSize).field("NumOutStandingRequests", &self.NumOutStandingRequests).finish()
    }
}
impl ::core::default::Default for VOLUME_ALLOCATION_HINT_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_ALLOCATION_HINT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ClusterSize == other.ClusterSize && self.NumberOfClusters == other.NumberOfClusters && self.StartingClusterNumber == other.StartingClusterNumber
    }
}
impl ::core::cmp::Eq for VOLUME_ALLOCATION_HINT_INPUT {}
impl ::core::fmt::Debug for VOLUME_ALLOCATION_HINT_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_ALLOCATION_HINT_INPUT").field("ClusterSize", &self.ClusterSize).field("NumberOfClusters", &self.NumberOfClusters).field("StartingClusterNumber", &self.StartingClusterNumber).finish()
    }
}
impl ::core::default::Default for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Bitmap == other.Bitmap
    }
}
impl ::core::cmp::Eq for VOLUME_ALLOCATION_HINT_OUTPUT {}
impl ::core::fmt::Debug for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_ALLOCATION_HINT_OUTPUT").field("Bitmap", &self.Bitmap).finish()
    }
}
impl ::core::default::Default for VOLUME_CRITICAL_IO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_CRITICAL_IO {
    fn eq(&self, other: &Self) -> bool {
        self.AccessType == other.AccessType && self.ExtentsCount == other.ExtentsCount && self.Extents == other.Extents
    }
}
impl ::core::cmp::Eq for VOLUME_CRITICAL_IO {}
impl ::core::fmt::Debug for VOLUME_CRITICAL_IO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_CRITICAL_IO").field("AccessType", &self.AccessType).field("ExtentsCount", &self.ExtentsCount).field("Extents", &self.Extents).finish()
    }
}
impl ::core::default::Default for VOLUME_FAILOVER_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_FAILOVER_SET {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfDisks == other.NumberOfDisks && self.DiskNumbers == other.DiskNumbers
    }
}
impl ::core::cmp::Eq for VOLUME_FAILOVER_SET {}
impl ::core::fmt::Debug for VOLUME_FAILOVER_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_FAILOVER_SET").field("NumberOfDisks", &self.NumberOfDisks).field("DiskNumbers", &self.DiskNumbers).finish()
    }
}
impl ::core::default::Default for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Reserved1 == other.Reserved1 && self.LowestByteOffset == other.LowestByteOffset && self.HighestByteOffset == other.HighestByteOffset && self.AccessType == other.AccessType && self.AccessMode == other.AccessMode
    }
}
impl ::core::cmp::Eq for VOLUME_GET_BC_PROPERTIES_INPUT {}
impl ::core::fmt::Debug for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_GET_BC_PROPERTIES_INPUT").field("Version", &self.Version).field("Reserved1", &self.Reserved1).field("LowestByteOffset", &self.LowestByteOffset).field("HighestByteOffset", &self.HighestByteOffset).field("AccessType", &self.AccessType).field("AccessMode", &self.AccessMode).finish()
    }
}
impl ::core::default::Default for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumRequestsPerPeriod == other.MaximumRequestsPerPeriod && self.MinimumPeriod == other.MinimumPeriod && self.MaximumRequestSize == other.MaximumRequestSize && self.EstimatedTimePerRequest == other.EstimatedTimePerRequest && self.NumOutStandingRequests == other.NumOutStandingRequests && self.RequestSize == other.RequestSize
    }
}
impl ::core::cmp::Eq for VOLUME_GET_BC_PROPERTIES_OUTPUT {}
impl ::core::fmt::Debug for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_GET_BC_PROPERTIES_OUTPUT").field("MaximumRequestsPerPeriod", &self.MaximumRequestsPerPeriod).field("MinimumPeriod", &self.MinimumPeriod).field("MaximumRequestSize", &self.MaximumRequestSize).field("EstimatedTimePerRequest", &self.EstimatedTimePerRequest).field("NumOutStandingRequests", &self.NumOutStandingRequests).field("RequestSize", &self.RequestSize).finish()
    }
}
impl ::core::default::Default for VOLUME_LOGICAL_OFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_LOGICAL_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.LogicalOffset == other.LogicalOffset
    }
}
impl ::core::cmp::Eq for VOLUME_LOGICAL_OFFSET {}
impl ::core::fmt::Debug for VOLUME_LOGICAL_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_LOGICAL_OFFSET").field("LogicalOffset", &self.LogicalOffset).finish()
    }
}
impl ::core::default::Default for VOLUME_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeNumber == other.VolumeNumber && self.VolumeManagerName == other.VolumeManagerName
    }
}
impl ::core::cmp::Eq for VOLUME_NUMBER {}
impl ::core::fmt::Debug for VOLUME_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_NUMBER").field("VolumeNumber", &self.VolumeNumber).field("VolumeManagerName", &self.VolumeManagerName).finish()
    }
}
impl ::core::default::Default for VOLUME_PHYSICAL_OFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_PHYSICAL_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.DiskNumber == other.DiskNumber && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for VOLUME_PHYSICAL_OFFSET {}
impl ::core::fmt::Debug for VOLUME_PHYSICAL_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_PHYSICAL_OFFSET").field("DiskNumber", &self.DiskNumber).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for VOLUME_PHYSICAL_OFFSETS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_PHYSICAL_OFFSETS {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfPhysicalOffsets == other.NumberOfPhysicalOffsets && self.PhysicalOffset == other.PhysicalOffset
    }
}
impl ::core::cmp::Eq for VOLUME_PHYSICAL_OFFSETS {}
impl ::core::fmt::Debug for VOLUME_PHYSICAL_OFFSETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_PHYSICAL_OFFSETS").field("NumberOfPhysicalOffsets", &self.NumberOfPhysicalOffsets).field("PhysicalOffset", &self.PhysicalOffset).finish()
    }
}
impl ::core::default::Default for VOLUME_READ_PLEX_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_READ_PLEX_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ByteOffset == other.ByteOffset && self.Length == other.Length && self.PlexNumber == other.PlexNumber
    }
}
impl ::core::cmp::Eq for VOLUME_READ_PLEX_INPUT {}
impl ::core::fmt::Debug for VOLUME_READ_PLEX_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_READ_PLEX_INPUT").field("ByteOffset", &self.ByteOffset).field("Length", &self.Length).field("PlexNumber", &self.PlexNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.GptAttributes == other.GptAttributes && self.RevertOnClose == other.RevertOnClose && self.ApplyToAllConnectedVolumes == other.ApplyToAllConnectedVolumes && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_SET_GPT_ATTRIBUTES_INFORMATION").field("GptAttributes", &self.GptAttributes).field("RevertOnClose", &self.RevertOnClose).field("ApplyToAllConnectedVolumes", &self.ApplyToAllConnectedVolumes).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for VOLUME_SHRINK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_SHRINK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeSize == other.VolumeSize
    }
}
impl ::core::cmp::Eq for VOLUME_SHRINK_INFO {}
impl ::core::fmt::Debug for VOLUME_SHRINK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_SHRINK_INFO").field("VolumeSize", &self.VolumeSize).finish()
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VS_FIXEDFILEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSignature == other.dwSignature && self.dwStrucVersion == other.dwStrucVersion && self.dwFileVersionMS == other.dwFileVersionMS && self.dwFileVersionLS == other.dwFileVersionLS && self.dwProductVersionMS == other.dwProductVersionMS && self.dwProductVersionLS == other.dwProductVersionLS && self.dwFileFlagsMask == other.dwFileFlagsMask && self.dwFileFlags == other.dwFileFlags && self.dwFileOS == other.dwFileOS && self.dwFileType == other.dwFileType && self.dwFileSubtype == other.dwFileSubtype && self.dwFileDateMS == other.dwFileDateMS && self.dwFileDateLS == other.dwFileDateLS
    }
}
impl ::core::cmp::Eq for VS_FIXEDFILEINFO {}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VS_FIXEDFILEINFO")
            .field("dwSignature", &self.dwSignature)
            .field("dwStrucVersion", &self.dwStrucVersion)
            .field("dwFileVersionMS", &self.dwFileVersionMS)
            .field("dwFileVersionLS", &self.dwFileVersionLS)
            .field("dwProductVersionMS", &self.dwProductVersionMS)
            .field("dwProductVersionLS", &self.dwProductVersionLS)
            .field("dwFileFlagsMask", &self.dwFileFlagsMask)
            .field("dwFileFlags", &self.dwFileFlags)
            .field("dwFileOS", &self.dwFileOS)
            .field("dwFileType", &self.dwFileType)
            .field("dwFileSubtype", &self.dwFileSubtype)
            .field("dwFileDateMS", &self.dwFileDateMS)
            .field("dwFileDateLS", &self.dwFileDateLS)
            .finish()
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VS_FIXEDFILEINFO_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VS_FIXEDFILEINFO_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO_FILE_OS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO_FILE_OS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_OS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO_FILE_SUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_SUBTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VS_FIXEDFILEINFO_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VS_FIXEDFILEINFO_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VS_FIXEDFILEINFO_FILE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WIM_ENTRY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIM_ENTRY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.WimEntryInfoSize == other.WimEntryInfoSize && self.WimType == other.WimType && self.DataSourceId == other.DataSourceId && self.WimGuid == other.WimGuid && self.WimPath == other.WimPath && self.WimIndex == other.WimIndex && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WIM_ENTRY_INFO {}
impl ::core::fmt::Debug for WIM_ENTRY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_ENTRY_INFO").field("WimEntryInfoSize", &self.WimEntryInfoSize).field("WimType", &self.WimType).field("DataSourceId", &self.DataSourceId).field("WimGuid", &self.WimGuid).field("WimPath", &self.WimPath).field("WimIndex", &self.WimIndex).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for WIM_EXTERNAL_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIM_EXTERNAL_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DataSourceId == other.DataSourceId && self.ResourceHash == other.ResourceHash && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WIM_EXTERNAL_FILE_INFO {}
impl ::core::fmt::Debug for WIM_EXTERNAL_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_EXTERNAL_FILE_INFO").field("DataSourceId", &self.DataSourceId).field("ResourceHash", &self.ResourceHash).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN32_FILE_ATTRIBUTE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN32_FILE_ATTRIBUTE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes && self.ftCreationTime == other.ftCreationTime && self.ftLastAccessTime == other.ftLastAccessTime && self.ftLastWriteTime == other.ftLastWriteTime && self.nFileSizeHigh == other.nFileSizeHigh && self.nFileSizeLow == other.nFileSizeLow
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN32_FILE_ATTRIBUTE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN32_FILE_ATTRIBUTE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_FILE_ATTRIBUTE_DATA").field("dwFileAttributes", &self.dwFileAttributes).field("ftCreationTime", &self.ftCreationTime).field("ftLastAccessTime", &self.ftLastAccessTime).field("ftLastWriteTime", &self.ftLastWriteTime).field("nFileSizeHigh", &self.nFileSizeHigh).field("nFileSizeLow", &self.nFileSizeLow).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN32_FIND_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN32_FIND_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes && self.ftCreationTime == other.ftCreationTime && self.ftLastAccessTime == other.ftLastAccessTime && self.ftLastWriteTime == other.ftLastWriteTime && self.nFileSizeHigh == other.nFileSizeHigh && self.nFileSizeLow == other.nFileSizeLow && self.dwReserved0 == other.dwReserved0 && self.dwReserved1 == other.dwReserved1 && self.cFileName == other.cFileName && self.cAlternateFileName == other.cAlternateFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN32_FIND_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN32_FIND_DATAA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_FIND_DATAA")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .field("dwReserved0", &self.dwReserved0)
            .field("dwReserved1", &self.dwReserved1)
            .field("cFileName", &self.cFileName)
            .field("cAlternateFileName", &self.cAlternateFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIN32_FIND_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIN32_FIND_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileAttributes == other.dwFileAttributes && self.ftCreationTime == other.ftCreationTime && self.ftLastAccessTime == other.ftLastAccessTime && self.ftLastWriteTime == other.ftLastWriteTime && self.nFileSizeHigh == other.nFileSizeHigh && self.nFileSizeLow == other.nFileSizeLow && self.dwReserved0 == other.dwReserved0 && self.dwReserved1 == other.dwReserved1 && self.cFileName == other.cFileName && self.cAlternateFileName == other.cAlternateFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIN32_FIND_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIN32_FIND_DATAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_FIND_DATAW")
            .field("dwFileAttributes", &self.dwFileAttributes)
            .field("ftCreationTime", &self.ftCreationTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastWriteTime", &self.ftLastWriteTime)
            .field("nFileSizeHigh", &self.nFileSizeHigh)
            .field("nFileSizeLow", &self.nFileSizeLow)
            .field("dwReserved0", &self.dwReserved0)
            .field("dwReserved1", &self.dwReserved1)
            .field("cFileName", &self.cFileName)
            .field("cAlternateFileName", &self.cAlternateFileName)
            .finish()
    }
}
impl ::core::default::Default for WIN32_FIND_STREAM_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIN32_FIND_STREAM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.StreamSize == other.StreamSize && self.cStreamName == other.cStreamName
    }
}
impl ::core::cmp::Eq for WIN32_FIND_STREAM_DATA {}
impl ::core::fmt::Debug for WIN32_FIND_STREAM_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_FIND_STREAM_DATA").field("StreamSize", &self.StreamSize).field("cStreamName", &self.cStreamName).finish()
    }
}
impl ::core::default::Default for WIN32_STREAM_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIN32_STREAM_ID {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamId == other.dwStreamId && self.dwStreamAttributes == other.dwStreamAttributes && self.Size == other.Size && self.dwStreamNameSize == other.dwStreamNameSize && self.cStreamName == other.cStreamName
    }
}
impl ::core::cmp::Eq for WIN32_STREAM_ID {}
impl ::core::fmt::Debug for WIN32_STREAM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_STREAM_ID").field("dwStreamId", &self.dwStreamId).field("dwStreamAttributes", &self.dwStreamAttributes).field("Size", &self.Size).field("dwStreamNameSize", &self.dwStreamNameSize).field("cStreamName", &self.cStreamName).finish()
    }
}
impl ::core::default::Default for WIN_STREAM_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WIN_STREAM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN_STREAM_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WOF_FILE_COMPRESSION_INFO_V0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WOF_FILE_COMPRESSION_INFO_V0 {
    fn eq(&self, other: &Self) -> bool {
        self.Algorithm == other.Algorithm
    }
}
impl ::core::cmp::Eq for WOF_FILE_COMPRESSION_INFO_V0 {}
impl ::core::fmt::Debug for WOF_FILE_COMPRESSION_INFO_V0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOF_FILE_COMPRESSION_INFO_V0").field("Algorithm", &self.Algorithm).finish()
    }
}
impl ::core::default::Default for WOF_FILE_COMPRESSION_INFO_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WOF_FILE_COMPRESSION_INFO_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Algorithm == other.Algorithm && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WOF_FILE_COMPRESSION_INFO_V1 {}
impl ::core::fmt::Debug for WOF_FILE_COMPRESSION_INFO_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOF_FILE_COMPRESSION_INFO_V1").field("Algorithm", &self.Algorithm).field("Flags", &self.Flags).finish()
    }
}

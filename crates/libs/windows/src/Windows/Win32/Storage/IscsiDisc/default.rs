impl ::core::default::Default for ATA_PASS_THROUGH_DIRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATA_PASS_THROUGH_DIRECT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.AtaFlags == other.AtaFlags && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.ReservedAsUchar == other.ReservedAsUchar && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.ReservedAsUlong == other.ReservedAsUlong && self.DataBuffer == other.DataBuffer && self.PreviousTaskFile == other.PreviousTaskFile && self.CurrentTaskFile == other.CurrentTaskFile
    }
}
impl ::core::cmp::Eq for ATA_PASS_THROUGH_DIRECT {}
impl ::core::fmt::Debug for ATA_PASS_THROUGH_DIRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATA_PASS_THROUGH_DIRECT")
            .field("Length", &self.Length)
            .field("AtaFlags", &self.AtaFlags)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("ReservedAsUchar", &self.ReservedAsUchar)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ReservedAsUlong", &self.ReservedAsUlong)
            .field("DataBuffer", &self.DataBuffer)
            .field("PreviousTaskFile", &self.PreviousTaskFile)
            .field("CurrentTaskFile", &self.CurrentTaskFile)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for ATA_PASS_THROUGH_DIRECT32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for ATA_PASS_THROUGH_DIRECT32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.AtaFlags == other.AtaFlags && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.ReservedAsUchar == other.ReservedAsUchar && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.ReservedAsUlong == other.ReservedAsUlong && self.DataBuffer == other.DataBuffer && self.PreviousTaskFile == other.PreviousTaskFile && self.CurrentTaskFile == other.CurrentTaskFile
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for ATA_PASS_THROUGH_DIRECT32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for ATA_PASS_THROUGH_DIRECT32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATA_PASS_THROUGH_DIRECT32")
            .field("Length", &self.Length)
            .field("AtaFlags", &self.AtaFlags)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("ReservedAsUchar", &self.ReservedAsUchar)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ReservedAsUlong", &self.ReservedAsUlong)
            .field("DataBuffer", &self.DataBuffer)
            .field("PreviousTaskFile", &self.PreviousTaskFile)
            .field("CurrentTaskFile", &self.CurrentTaskFile)
            .finish()
    }
}
impl ::core::default::Default for ATA_PASS_THROUGH_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ATA_PASS_THROUGH_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.AtaFlags == other.AtaFlags && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.ReservedAsUchar == other.ReservedAsUchar && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.ReservedAsUlong == other.ReservedAsUlong && self.DataBufferOffset == other.DataBufferOffset && self.PreviousTaskFile == other.PreviousTaskFile && self.CurrentTaskFile == other.CurrentTaskFile
    }
}
impl ::core::cmp::Eq for ATA_PASS_THROUGH_EX {}
impl ::core::fmt::Debug for ATA_PASS_THROUGH_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATA_PASS_THROUGH_EX")
            .field("Length", &self.Length)
            .field("AtaFlags", &self.AtaFlags)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("ReservedAsUchar", &self.ReservedAsUchar)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ReservedAsUlong", &self.ReservedAsUlong)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("PreviousTaskFile", &self.PreviousTaskFile)
            .field("CurrentTaskFile", &self.CurrentTaskFile)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for ATA_PASS_THROUGH_EX32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for ATA_PASS_THROUGH_EX32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.AtaFlags == other.AtaFlags && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.ReservedAsUchar == other.ReservedAsUchar && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.ReservedAsUlong == other.ReservedAsUlong && self.DataBufferOffset == other.DataBufferOffset && self.PreviousTaskFile == other.PreviousTaskFile && self.CurrentTaskFile == other.CurrentTaskFile
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for ATA_PASS_THROUGH_EX32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for ATA_PASS_THROUGH_EX32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATA_PASS_THROUGH_EX32")
            .field("Length", &self.Length)
            .field("AtaFlags", &self.AtaFlags)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("ReservedAsUchar", &self.ReservedAsUchar)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ReservedAsUlong", &self.ReservedAsUlong)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("PreviousTaskFile", &self.PreviousTaskFile)
            .field("CurrentTaskFile", &self.CurrentTaskFile)
            .finish()
    }
}
impl ::core::default::Default for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.NotifyFlags == other.NotifyFlags && self.DataSetProfile == other.DataSetProfile && self.Reserved == other.Reserved && self.DataSetRangesCount == other.DataSetRangesCount && self.DataSetRanges == other.DataSetRanges
    }
}
impl ::core::cmp::Eq for DSM_NOTIFICATION_REQUEST_BLOCK {}
impl ::core::fmt::Debug for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSM_NOTIFICATION_REQUEST_BLOCK").field("Size", &self.Size).field("Version", &self.Version).field("NotifyFlags", &self.NotifyFlags).field("DataSetProfile", &self.DataSetProfile).field("Reserved", &self.Reserved).field("DataSetRangesCount", &self.DataSetRangesCount).field("DataSetRanges", &self.DataSetRanges).finish()
    }
}
impl ::core::default::Default for DUMP_DRIVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DUMP_DRIVER {
    fn eq(&self, other: &Self) -> bool {
        self.DumpDriverList == other.DumpDriverList && self.DriverName == other.DriverName && self.BaseName == other.BaseName
    }
}
impl ::core::cmp::Eq for DUMP_DRIVER {}
impl ::core::fmt::Debug for DUMP_DRIVER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUMP_DRIVER").field("DumpDriverList", &self.DumpDriverList).field("DriverName", &self.DriverName).field("BaseName", &self.BaseName).finish()
    }
}
impl ::core::default::Default for DUMP_DRIVER_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DUMP_DRIVER_EX {
    fn eq(&self, other: &Self) -> bool {
        self.DumpDriverList == other.DumpDriverList && self.DriverName == other.DriverName && self.BaseName == other.BaseName && self.DriverFullPath == other.DriverFullPath
    }
}
impl ::core::cmp::Eq for DUMP_DRIVER_EX {}
impl ::core::fmt::Debug for DUMP_DRIVER_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUMP_DRIVER_EX").field("DumpDriverList", &self.DumpDriverList).field("DriverName", &self.DriverName).field("BaseName", &self.BaseName).field("DriverFullPath", &self.DriverFullPath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUMP_POINTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DUMP_POINTERS {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterObject == other.AdapterObject && self.MappedRegisterBase == other.MappedRegisterBase && self.DumpData == other.DumpData && self.CommonBufferVa == other.CommonBufferVa && self.CommonBufferPa == other.CommonBufferPa && self.CommonBufferSize == other.CommonBufferSize && self.AllocateCommonBuffers == other.AllocateCommonBuffers && self.UseDiskDump == other.UseDiskDump && self.Spare1 == other.Spare1 && self.DeviceObject == other.DeviceObject
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DUMP_POINTERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DUMP_POINTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUMP_POINTERS")
            .field("AdapterObject", &self.AdapterObject)
            .field("MappedRegisterBase", &self.MappedRegisterBase)
            .field("DumpData", &self.DumpData)
            .field("CommonBufferVa", &self.CommonBufferVa)
            .field("CommonBufferPa", &self.CommonBufferPa)
            .field("CommonBufferSize", &self.CommonBufferSize)
            .field("AllocateCommonBuffers", &self.AllocateCommonBuffers)
            .field("UseDiskDump", &self.UseDiskDump)
            .field("Spare1", &self.Spare1)
            .field("DeviceObject", &self.DeviceObject)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUMP_POINTERS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DUMP_POINTERS_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DUMP_POINTERS_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for DUMP_POINTERS_VERSION {}
impl ::core::fmt::Debug for DUMP_POINTERS_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUMP_POINTERS_VERSION").field("Version", &self.Version).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for FIRMWARE_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FIRMWARE_REQUEST_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Function == other.Function && self.Flags == other.Flags && self.DataBufferOffset == other.DataBufferOffset && self.DataBufferLength == other.DataBufferLength
    }
}
impl ::core::cmp::Eq for FIRMWARE_REQUEST_BLOCK {}
impl ::core::fmt::Debug for FIRMWARE_REQUEST_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIRMWARE_REQUEST_BLOCK").field("Version", &self.Version).field("Size", &self.Size).field("Function", &self.Function).field("Flags", &self.Flags).field("DataBufferOffset", &self.DataBufferOffset).field("DataBufferLength", &self.DataBufferLength).finish()
    }
}
impl ::core::default::Default for HYBRID_DEMOTE_BY_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HYBRID_DEMOTE_BY_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.SourcePriority == other.SourcePriority && self.TargetPriority == other.TargetPriority && self.Reserved0 == other.Reserved0 && self.Reserved1 == other.Reserved1 && self.LbaCount == other.LbaCount
    }
}
impl ::core::cmp::Eq for HYBRID_DEMOTE_BY_SIZE {}
impl ::core::fmt::Debug for HYBRID_DEMOTE_BY_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_DEMOTE_BY_SIZE").field("Version", &self.Version).field("Size", &self.Size).field("SourcePriority", &self.SourcePriority).field("TargetPriority", &self.TargetPriority).field("Reserved0", &self.Reserved0).field("Reserved1", &self.Reserved1).field("LbaCount", &self.LbaCount).finish()
    }
}
impl ::core::default::Default for HYBRID_DIRTY_THRESHOLDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HYBRID_DIRTY_THRESHOLDS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.DirtyLowThreshold == other.DirtyLowThreshold && self.DirtyHighThreshold == other.DirtyHighThreshold
    }
}
impl ::core::cmp::Eq for HYBRID_DIRTY_THRESHOLDS {}
impl ::core::fmt::Debug for HYBRID_DIRTY_THRESHOLDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_DIRTY_THRESHOLDS").field("Version", &self.Version).field("Size", &self.Size).field("DirtyLowThreshold", &self.DirtyLowThreshold).field("DirtyHighThreshold", &self.DirtyHighThreshold).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HYBRID_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HYBRID_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.HybridSupported == other.HybridSupported && self.Status == other.Status && self.CacheTypeEffective == other.CacheTypeEffective && self.CacheTypeDefault == other.CacheTypeDefault && self.FractionBase == other.FractionBase && self.CacheSize == other.CacheSize && self.Attributes == other.Attributes && self.Priorities == other.Priorities
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HYBRID_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HYBRID_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_INFORMATION").field("Version", &self.Version).field("Size", &self.Size).field("HybridSupported", &self.HybridSupported).field("Status", &self.Status).field("CacheTypeEffective", &self.CacheTypeEffective).field("CacheTypeDefault", &self.CacheTypeDefault).field("FractionBase", &self.FractionBase).field("CacheSize", &self.CacheSize).field("Attributes", &self.Attributes).field("Priorities", &self.Priorities).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HYBRID_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HYBRID_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HYBRID_INFORMATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HYBRID_INFORMATION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_INFORMATION_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HYBRID_INFORMATION_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HYBRID_INFORMATION_1 {
    fn eq(&self, other: &Self) -> bool {
        self.PriorityLevelCount == other.PriorityLevelCount && self.MaxPriorityBehavior == other.MaxPriorityBehavior && self.OptimalWriteGranularity == other.OptimalWriteGranularity && self.Reserved == other.Reserved && self.DirtyThresholdLow == other.DirtyThresholdLow && self.DirtyThresholdHigh == other.DirtyThresholdHigh && self.SupportedCommands == other.SupportedCommands && self.Priority == other.Priority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HYBRID_INFORMATION_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HYBRID_INFORMATION_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_INFORMATION_1").field("PriorityLevelCount", &self.PriorityLevelCount).field("MaxPriorityBehavior", &self.MaxPriorityBehavior).field("OptimalWriteGranularity", &self.OptimalWriteGranularity).field("Reserved", &self.Reserved).field("DirtyThresholdLow", &self.DirtyThresholdLow).field("DirtyThresholdHigh", &self.DirtyThresholdHigh).field("SupportedCommands", &self.SupportedCommands).field("Priority", &self.Priority).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HYBRID_INFORMATION_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HYBRID_INFORMATION_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.MaxEvictCommands == other.MaxEvictCommands && self.MaxLbaRangeCountForEvict == other.MaxLbaRangeCountForEvict && self.MaxLbaRangeCountForChangeLba == other.MaxLbaRangeCountForChangeLba
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HYBRID_INFORMATION_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HYBRID_INFORMATION_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_INFORMATION_1_0").field("_bitfield", &self._bitfield).field("MaxEvictCommands", &self.MaxEvictCommands).field("MaxLbaRangeCountForEvict", &self.MaxLbaRangeCountForEvict).field("MaxLbaRangeCountForChangeLba", &self.MaxLbaRangeCountForChangeLba).finish()
    }
}
impl ::core::default::Default for HYBRID_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HYBRID_REQUEST_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Function == other.Function && self.Flags == other.Flags && self.DataBufferOffset == other.DataBufferOffset && self.DataBufferLength == other.DataBufferLength
    }
}
impl ::core::cmp::Eq for HYBRID_REQUEST_BLOCK {}
impl ::core::fmt::Debug for HYBRID_REQUEST_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_REQUEST_BLOCK").field("Version", &self.Version).field("Size", &self.Size).field("Function", &self.Function).field("Flags", &self.Flags).field("DataBufferOffset", &self.DataBufferOffset).field("DataBufferLength", &self.DataBufferLength).finish()
    }
}
impl ::core::default::Default for IDE_IO_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IDE_IO_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderLength == other.HeaderLength && self.Signature == other.Signature && self.Timeout == other.Timeout && self.ControlCode == other.ControlCode && self.ReturnStatus == other.ReturnStatus && self.DataLength == other.DataLength
    }
}
impl ::core::cmp::Eq for IDE_IO_CONTROL {}
impl ::core::fmt::Debug for IDE_IO_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IDE_IO_CONTROL").field("HeaderLength", &self.HeaderLength).field("Signature", &self.Signature).field("Timeout", &self.Timeout).field("ControlCode", &self.ControlCode).field("ReturnStatus", &self.ReturnStatus).field("DataLength", &self.DataLength).finish()
    }
}
impl ::core::default::Default for IKE_AUTHENTICATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKE_AUTHENTICATION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKE_AUTHENTICATION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKE_AUTHENTICATION_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKE_AUTHENTICATION_PRESHARED_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKE_AUTHENTICATION_PRESHARED_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityFlags == other.SecurityFlags && self.IdType == other.IdType && self.IdLengthInBytes == other.IdLengthInBytes && self.Id == other.Id && self.KeyLengthInBytes == other.KeyLengthInBytes && self.Key == other.Key
    }
}
impl ::core::cmp::Eq for IKE_AUTHENTICATION_PRESHARED_KEY {}
impl ::core::fmt::Debug for IKE_AUTHENTICATION_PRESHARED_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKE_AUTHENTICATION_PRESHARED_KEY").field("SecurityFlags", &self.SecurityFlags).field("IdType", &self.IdType).field("IdLengthInBytes", &self.IdLengthInBytes).field("Id", &self.Id).field("KeyLengthInBytes", &self.KeyLengthInBytes).field("Key", &self.Key).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IO_SCSI_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IO_SCSI_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumTransferLength == other.MaximumTransferLength && self.MaximumPhysicalPages == other.MaximumPhysicalPages && self.SupportedAsynchronousEvents == other.SupportedAsynchronousEvents && self.AlignmentMask == other.AlignmentMask && self.TaggedQueuing == other.TaggedQueuing && self.AdapterScansDown == other.AdapterScansDown && self.AdapterUsesPio == other.AdapterUsesPio
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IO_SCSI_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IO_SCSI_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IO_SCSI_CAPABILITIES").field("Length", &self.Length).field("MaximumTransferLength", &self.MaximumTransferLength).field("MaximumPhysicalPages", &self.MaximumPhysicalPages).field("SupportedAsynchronousEvents", &self.SupportedAsynchronousEvents).field("AlignmentMask", &self.AlignmentMask).field("TaggedQueuing", &self.TaggedQueuing).field("AdapterScansDown", &self.AdapterScansDown).field("AdapterUsesPio", &self.AdapterUsesPio).finish()
    }
}
impl ::core::default::Default for ISCSI_AUTH_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ISCSI_AUTH_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCSI_AUTH_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for ISCSI_CONNECTION_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_CONNECTION_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionId == other.ConnectionId && self.InitiatorAddress == other.InitiatorAddress && self.TargetAddress == other.TargetAddress && self.InitiatorSocket == other.InitiatorSocket && self.TargetSocket == other.TargetSocket && self.CID == other.CID
    }
}
impl ::core::cmp::Eq for ISCSI_CONNECTION_INFOA {}
impl ::core::fmt::Debug for ISCSI_CONNECTION_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_CONNECTION_INFOA").field("ConnectionId", &self.ConnectionId).field("InitiatorAddress", &self.InitiatorAddress).field("TargetAddress", &self.TargetAddress).field("InitiatorSocket", &self.InitiatorSocket).field("TargetSocket", &self.TargetSocket).field("CID", &self.CID).finish()
    }
}
impl ::core::default::Default for ISCSI_CONNECTION_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_CONNECTION_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionId == other.ConnectionId && self.InitiatorAddress == other.InitiatorAddress && self.TargetAddress == other.TargetAddress && self.InitiatorSocket == other.InitiatorSocket && self.TargetSocket == other.TargetSocket && self.CID == other.CID
    }
}
impl ::core::cmp::Eq for ISCSI_CONNECTION_INFOW {}
impl ::core::fmt::Debug for ISCSI_CONNECTION_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_CONNECTION_INFOW").field("ConnectionId", &self.ConnectionId).field("InitiatorAddress", &self.InitiatorAddress).field("TargetAddress", &self.TargetAddress).field("InitiatorSocket", &self.InitiatorSocket).field("TargetSocket", &self.TargetSocket).field("CID", &self.CID).finish()
    }
}
impl ::core::default::Default for ISCSI_CONNECTION_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_CONNECTION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionId == other.ConnectionId && self.State == other.State && self.Protocol == other.Protocol && self.HeaderDigest == other.HeaderDigest && self.DataDigest == other.DataDigest && self.MaxRecvDataSegmentLength == other.MaxRecvDataSegmentLength && self.AuthType == other.AuthType && self.EstimatedThroughput == other.EstimatedThroughput && self.MaxDatagramSize == other.MaxDatagramSize
    }
}
impl ::core::cmp::Eq for ISCSI_CONNECTION_INFO_EX {}
impl ::core::fmt::Debug for ISCSI_CONNECTION_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_CONNECTION_INFO_EX").field("ConnectionId", &self.ConnectionId).field("State", &self.State).field("Protocol", &self.Protocol).field("HeaderDigest", &self.HeaderDigest).field("DataDigest", &self.DataDigest).field("MaxRecvDataSegmentLength", &self.MaxRecvDataSegmentLength).field("AuthType", &self.AuthType).field("EstimatedThroughput", &self.EstimatedThroughput).field("MaxDatagramSize", &self.MaxDatagramSize).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ioctl"))]
impl ::core::default::Default for ISCSI_DEVICE_ON_SESSIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ioctl"))]
impl ::core::cmp::PartialEq for ISCSI_DEVICE_ON_SESSIONA {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.TargetName == other.TargetName && self.ScsiAddress == other.ScsiAddress && self.DeviceInterfaceType == other.DeviceInterfaceType && self.DeviceInterfaceName == other.DeviceInterfaceName && self.LegacyName == other.LegacyName && self.StorageDeviceNumber == other.StorageDeviceNumber && self.DeviceInstance == other.DeviceInstance
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ioctl"))]
impl ::core::cmp::Eq for ISCSI_DEVICE_ON_SESSIONA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ioctl"))]
impl ::core::fmt::Debug for ISCSI_DEVICE_ON_SESSIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_DEVICE_ON_SESSIONA").field("InitiatorName", &self.InitiatorName).field("TargetName", &self.TargetName).field("ScsiAddress", &self.ScsiAddress).field("DeviceInterfaceType", &self.DeviceInterfaceType).field("DeviceInterfaceName", &self.DeviceInterfaceName).field("LegacyName", &self.LegacyName).field("StorageDeviceNumber", &self.StorageDeviceNumber).field("DeviceInstance", &self.DeviceInstance).finish()
    }
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::default::Default for ISCSI_DEVICE_ON_SESSIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::cmp::PartialEq for ISCSI_DEVICE_ON_SESSIONW {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.TargetName == other.TargetName && self.ScsiAddress == other.ScsiAddress && self.DeviceInterfaceType == other.DeviceInterfaceType && self.DeviceInterfaceName == other.DeviceInterfaceName && self.LegacyName == other.LegacyName && self.StorageDeviceNumber == other.StorageDeviceNumber && self.DeviceInstance == other.DeviceInstance
    }
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::cmp::Eq for ISCSI_DEVICE_ON_SESSIONW {}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::fmt::Debug for ISCSI_DEVICE_ON_SESSIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_DEVICE_ON_SESSIONW").field("InitiatorName", &self.InitiatorName).field("TargetName", &self.TargetName).field("ScsiAddress", &self.ScsiAddress).field("DeviceInterfaceType", &self.DeviceInterfaceType).field("DeviceInterfaceName", &self.DeviceInterfaceName).field("LegacyName", &self.LegacyName).field("StorageDeviceNumber", &self.StorageDeviceNumber).field("DeviceInstance", &self.DeviceInstance).finish()
    }
}
impl ::core::default::Default for ISCSI_DIGEST_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ISCSI_DIGEST_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCSI_DIGEST_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for ISCSI_LOGIN_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_LOGIN_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.InformationSpecified == other.InformationSpecified && self.LoginFlags == other.LoginFlags && self.AuthType == other.AuthType && self.HeaderDigest == other.HeaderDigest && self.DataDigest == other.DataDigest && self.MaximumConnections == other.MaximumConnections && self.DefaultTime2Wait == other.DefaultTime2Wait && self.DefaultTime2Retain == other.DefaultTime2Retain && self.UsernameLength == other.UsernameLength && self.PasswordLength == other.PasswordLength && self.Username == other.Username && self.Password == other.Password
    }
}
impl ::core::cmp::Eq for ISCSI_LOGIN_OPTIONS {}
impl ::core::fmt::Debug for ISCSI_LOGIN_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_LOGIN_OPTIONS")
            .field("Version", &self.Version)
            .field("InformationSpecified", &self.InformationSpecified)
            .field("LoginFlags", &self.LoginFlags)
            .field("AuthType", &self.AuthType)
            .field("HeaderDigest", &self.HeaderDigest)
            .field("DataDigest", &self.DataDigest)
            .field("MaximumConnections", &self.MaximumConnections)
            .field("DefaultTime2Wait", &self.DefaultTime2Wait)
            .field("DefaultTime2Retain", &self.DefaultTime2Retain)
            .field("UsernameLength", &self.UsernameLength)
            .field("PasswordLength", &self.PasswordLength)
            .field("Username", &self.Username)
            .field("Password", &self.Password)
            .finish()
    }
}
impl ::core::default::Default for ISCSI_SESSION_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_SESSION_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.InitiatorName == other.InitiatorName && self.TargetNodeName == other.TargetNodeName && self.TargetName == other.TargetName && self.ISID == other.ISID && self.TSID == other.TSID && self.ConnectionCount == other.ConnectionCount && self.Connections == other.Connections
    }
}
impl ::core::cmp::Eq for ISCSI_SESSION_INFOA {}
impl ::core::fmt::Debug for ISCSI_SESSION_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_SESSION_INFOA").field("SessionId", &self.SessionId).field("InitiatorName", &self.InitiatorName).field("TargetNodeName", &self.TargetNodeName).field("TargetName", &self.TargetName).field("ISID", &self.ISID).field("TSID", &self.TSID).field("ConnectionCount", &self.ConnectionCount).field("Connections", &self.Connections).finish()
    }
}
impl ::core::default::Default for ISCSI_SESSION_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_SESSION_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.InitiatorName == other.InitiatorName && self.TargetNodeName == other.TargetNodeName && self.TargetName == other.TargetName && self.ISID == other.ISID && self.TSID == other.TSID && self.ConnectionCount == other.ConnectionCount && self.Connections == other.Connections
    }
}
impl ::core::cmp::Eq for ISCSI_SESSION_INFOW {}
impl ::core::fmt::Debug for ISCSI_SESSION_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_SESSION_INFOW").field("SessionId", &self.SessionId).field("InitiatorName", &self.InitiatorName).field("TargetNodeName", &self.TargetNodeName).field("TargetName", &self.TargetName).field("ISID", &self.ISID).field("TSID", &self.TSID).field("ConnectionCount", &self.ConnectionCount).field("Connections", &self.Connections).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ISCSI_SESSION_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ISCSI_SESSION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.InitialR2t == other.InitialR2t && self.ImmediateData == other.ImmediateData && self.Type == other.Type && self.DataSequenceInOrder == other.DataSequenceInOrder && self.DataPduInOrder == other.DataPduInOrder && self.ErrorRecoveryLevel == other.ErrorRecoveryLevel && self.MaxOutstandingR2t == other.MaxOutstandingR2t && self.FirstBurstLength == other.FirstBurstLength && self.MaxBurstLength == other.MaxBurstLength && self.MaximumConnections == other.MaximumConnections && self.ConnectionCount == other.ConnectionCount && self.Connections == other.Connections
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ISCSI_SESSION_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ISCSI_SESSION_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_SESSION_INFO_EX")
            .field("SessionId", &self.SessionId)
            .field("InitialR2t", &self.InitialR2t)
            .field("ImmediateData", &self.ImmediateData)
            .field("Type", &self.Type)
            .field("DataSequenceInOrder", &self.DataSequenceInOrder)
            .field("DataPduInOrder", &self.DataPduInOrder)
            .field("ErrorRecoveryLevel", &self.ErrorRecoveryLevel)
            .field("MaxOutstandingR2t", &self.MaxOutstandingR2t)
            .field("FirstBurstLength", &self.FirstBurstLength)
            .field("MaxBurstLength", &self.MaxBurstLength)
            .field("MaximumConnections", &self.MaximumConnections)
            .field("ConnectionCount", &self.ConnectionCount)
            .field("Connections", &self.Connections)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ISCSI_TARGET_MAPPINGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ISCSI_TARGET_MAPPINGA {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.TargetName == other.TargetName && self.OSDeviceName == other.OSDeviceName && self.SessionId == other.SessionId && self.OSBusNumber == other.OSBusNumber && self.OSTargetNumber == other.OSTargetNumber && self.LUNCount == other.LUNCount && self.LUNList == other.LUNList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ISCSI_TARGET_MAPPINGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ISCSI_TARGET_MAPPINGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_MAPPINGA").field("InitiatorName", &self.InitiatorName).field("TargetName", &self.TargetName).field("OSDeviceName", &self.OSDeviceName).field("SessionId", &self.SessionId).field("OSBusNumber", &self.OSBusNumber).field("OSTargetNumber", &self.OSTargetNumber).field("LUNCount", &self.LUNCount).field("LUNList", &self.LUNList).finish()
    }
}
impl ::core::default::Default for ISCSI_TARGET_MAPPINGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_MAPPINGW {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.TargetName == other.TargetName && self.OSDeviceName == other.OSDeviceName && self.SessionId == other.SessionId && self.OSBusNumber == other.OSBusNumber && self.OSTargetNumber == other.OSTargetNumber && self.LUNCount == other.LUNCount && self.LUNList == other.LUNList
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_MAPPINGW {}
impl ::core::fmt::Debug for ISCSI_TARGET_MAPPINGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_MAPPINGW").field("InitiatorName", &self.InitiatorName).field("TargetName", &self.TargetName).field("OSDeviceName", &self.OSDeviceName).field("SessionId", &self.SessionId).field("OSBusNumber", &self.OSBusNumber).field("OSTargetNumber", &self.OSTargetNumber).field("LUNCount", &self.LUNCount).field("LUNList", &self.LUNList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ISCSI_TARGET_PORTALA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTALA {
    fn eq(&self, other: &Self) -> bool {
        self.SymbolicName == other.SymbolicName && self.Address == other.Address && self.Socket == other.Socket
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ISCSI_TARGET_PORTALA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ISCSI_TARGET_PORTALA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTALA").field("SymbolicName", &self.SymbolicName).field("Address", &self.Address).field("Socket", &self.Socket).finish()
    }
}
impl ::core::default::Default for ISCSI_TARGET_PORTALW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTALW {
    fn eq(&self, other: &Self) -> bool {
        self.SymbolicName == other.SymbolicName && self.Address == other.Address && self.Socket == other.Socket
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_PORTALW {}
impl ::core::fmt::Debug for ISCSI_TARGET_PORTALW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTALW").field("SymbolicName", &self.SymbolicName).field("Address", &self.Address).field("Socket", &self.Socket).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ISCSI_TARGET_PORTAL_GROUPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTAL_GROUPA {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Portals == other.Portals
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ISCSI_TARGET_PORTAL_GROUPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ISCSI_TARGET_PORTAL_GROUPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTAL_GROUPA").field("Count", &self.Count).field("Portals", &self.Portals).finish()
    }
}
impl ::core::default::Default for ISCSI_TARGET_PORTAL_GROUPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTAL_GROUPW {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Portals == other.Portals
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_PORTAL_GROUPW {}
impl ::core::fmt::Debug for ISCSI_TARGET_PORTAL_GROUPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTAL_GROUPW").field("Count", &self.Count).field("Portals", &self.Portals).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ISCSI_TARGET_PORTAL_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTAL_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.InitiatorPortNumber == other.InitiatorPortNumber && self.SymbolicName == other.SymbolicName && self.Address == other.Address && self.Socket == other.Socket
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ISCSI_TARGET_PORTAL_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ISCSI_TARGET_PORTAL_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTAL_INFOA").field("InitiatorName", &self.InitiatorName).field("InitiatorPortNumber", &self.InitiatorPortNumber).field("SymbolicName", &self.SymbolicName).field("Address", &self.Address).field("Socket", &self.Socket).finish()
    }
}
impl ::core::default::Default for ISCSI_TARGET_PORTAL_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTAL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.InitiatorPortNumber == other.InitiatorPortNumber && self.SymbolicName == other.SymbolicName && self.Address == other.Address && self.Socket == other.Socket
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_PORTAL_INFOW {}
impl ::core::fmt::Debug for ISCSI_TARGET_PORTAL_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTAL_INFOW").field("InitiatorName", &self.InitiatorName).field("InitiatorPortNumber", &self.InitiatorPortNumber).field("SymbolicName", &self.SymbolicName).field("Address", &self.Address).field("Socket", &self.Socket).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ISCSI_TARGET_PORTAL_INFO_EXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTAL_INFO_EXA {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.InitiatorPortNumber == other.InitiatorPortNumber && self.SymbolicName == other.SymbolicName && self.Address == other.Address && self.Socket == other.Socket && self.SecurityFlags == other.SecurityFlags && self.LoginOptions == other.LoginOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ISCSI_TARGET_PORTAL_INFO_EXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ISCSI_TARGET_PORTAL_INFO_EXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTAL_INFO_EXA").field("InitiatorName", &self.InitiatorName).field("InitiatorPortNumber", &self.InitiatorPortNumber).field("SymbolicName", &self.SymbolicName).field("Address", &self.Address).field("Socket", &self.Socket).field("SecurityFlags", &self.SecurityFlags).field("LoginOptions", &self.LoginOptions).finish()
    }
}
impl ::core::default::Default for ISCSI_TARGET_PORTAL_INFO_EXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTAL_INFO_EXW {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.InitiatorPortNumber == other.InitiatorPortNumber && self.SymbolicName == other.SymbolicName && self.Address == other.Address && self.Socket == other.Socket && self.SecurityFlags == other.SecurityFlags && self.LoginOptions == other.LoginOptions
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_PORTAL_INFO_EXW {}
impl ::core::fmt::Debug for ISCSI_TARGET_PORTAL_INFO_EXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTAL_INFO_EXW").field("InitiatorName", &self.InitiatorName).field("InitiatorPortNumber", &self.InitiatorPortNumber).field("SymbolicName", &self.SymbolicName).field("Address", &self.Address).field("Socket", &self.Socket).field("SecurityFlags", &self.SecurityFlags).field("LoginOptions", &self.LoginOptions).finish()
    }
}
impl ::core::default::Default for ISCSI_UNIQUE_SESSION_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_UNIQUE_SESSION_ID {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterUnique == other.AdapterUnique && self.AdapterSpecific == other.AdapterSpecific
    }
}
impl ::core::cmp::Eq for ISCSI_UNIQUE_SESSION_ID {}
impl ::core::fmt::Debug for ISCSI_UNIQUE_SESSION_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_UNIQUE_SESSION_ID").field("AdapterUnique", &self.AdapterUnique).field("AdapterSpecific", &self.AdapterSpecific).finish()
    }
}
impl ::core::default::Default for ISCSI_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISCSI_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.BuildNumber == other.BuildNumber
    }
}
impl ::core::cmp::Eq for ISCSI_VERSION_INFO {}
impl ::core::fmt::Debug for ISCSI_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_VERSION_INFO").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("BuildNumber", &self.BuildNumber).finish()
    }
}
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH {
    fn eq(&self, other: &Self) -> bool {
        self.PassThrough == other.PassThrough && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH {}
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH").field("PassThrough", &self.PassThrough).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH32 {
    fn eq(&self, other: &Self) -> bool {
        self.PassThrough == other.PassThrough && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH32").field("PassThrough", &self.PassThrough).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH32_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH32_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PassThroughOffset == other.PassThroughOffset && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH32_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH32_EX").field("PassThroughOffset", &self.PassThroughOffset).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH_DIRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_DIRECT {
    fn eq(&self, other: &Self) -> bool {
        self.PassThrough == other.PassThrough && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH_DIRECT {}
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH_DIRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH_DIRECT").field("PassThrough", &self.PassThrough).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    fn eq(&self, other: &Self) -> bool {
        self.PassThrough == other.PassThrough && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH_DIRECT32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH_DIRECT32").field("PassThrough", &self.PassThrough).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PassThroughOffset == other.PassThroughOffset && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH_DIRECT32_EX").field("PassThroughOffset", &self.PassThroughOffset).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PassThroughOffset == other.PassThroughOffset && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH_DIRECT_EX {}
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH_DIRECT_EX").field("PassThroughOffset", &self.PassThroughOffset).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PassThroughOffset == other.PassThroughOffset && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH_EX {}
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH_EX").field("PassThroughOffset", &self.PassThroughOffset).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
impl ::core::default::Default for MP_DEVICE_DATA_SET_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MP_DEVICE_DATA_SET_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartingOffset == other.StartingOffset && self.LengthInBytes == other.LengthInBytes
    }
}
impl ::core::cmp::Eq for MP_DEVICE_DATA_SET_RANGE {}
impl ::core::fmt::Debug for MP_DEVICE_DATA_SET_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MP_DEVICE_DATA_SET_RANGE").field("StartingOffset", &self.StartingOffset).field("LengthInBytes", &self.LengthInBytes).finish()
    }
}
impl ::core::default::Default for MP_STORAGE_DIAGNOSTIC_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MP_STORAGE_DIAGNOSTIC_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MP_STORAGE_DIAGNOSTIC_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for MP_STORAGE_DIAGNOSTIC_TARGET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MP_STORAGE_DIAGNOSTIC_TARGET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MP_STORAGE_DIAGNOSTIC_TARGET_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NTSCSI_UNICODE_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTSCSI_UNICODE_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for NTSCSI_UNICODE_STRING {}
impl ::core::fmt::Debug for NTSCSI_UNICODE_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTSCSI_UNICODE_STRING").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for NVCACHE_HINT_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NVCACHE_HINT_PAYLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Command == other.Command && self.Feature7_0 == other.Feature7_0 && self.Feature15_8 == other.Feature15_8 && self.Count15_8 == other.Count15_8 && self.LBA7_0 == other.LBA7_0 && self.LBA15_8 == other.LBA15_8 && self.LBA23_16 == other.LBA23_16 && self.LBA31_24 == other.LBA31_24 && self.LBA39_32 == other.LBA39_32 && self.LBA47_40 == other.LBA47_40 && self.Auxiliary7_0 == other.Auxiliary7_0 && self.Auxiliary23_16 == other.Auxiliary23_16 && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NVCACHE_HINT_PAYLOAD {}
impl ::core::fmt::Debug for NVCACHE_HINT_PAYLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NVCACHE_HINT_PAYLOAD")
            .field("Command", &self.Command)
            .field("Feature7_0", &self.Feature7_0)
            .field("Feature15_8", &self.Feature15_8)
            .field("Count15_8", &self.Count15_8)
            .field("LBA7_0", &self.LBA7_0)
            .field("LBA15_8", &self.LBA15_8)
            .field("LBA23_16", &self.LBA23_16)
            .field("LBA31_24", &self.LBA31_24)
            .field("LBA39_32", &self.LBA39_32)
            .field("LBA47_40", &self.LBA47_40)
            .field("Auxiliary7_0", &self.Auxiliary7_0)
            .field("Auxiliary23_16", &self.Auxiliary23_16)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::default::Default for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.PriorityLevel == other.PriorityLevel && self.Reserved0 == other.Reserved0 && self.ConsumedNVMSizeFraction == other.ConsumedNVMSizeFraction && self.ConsumedMappingResourcesFraction == other.ConsumedMappingResourcesFraction && self.ConsumedNVMSizeForDirtyDataFraction == other.ConsumedNVMSizeForDirtyDataFraction && self.ConsumedMappingResourcesForDirtyDataFraction == other.ConsumedMappingResourcesForDirtyDataFraction && self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {}
impl ::core::fmt::Debug for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NVCACHE_PRIORITY_LEVEL_DESCRIPTOR")
            .field("PriorityLevel", &self.PriorityLevel)
            .field("Reserved0", &self.Reserved0)
            .field("ConsumedNVMSizeFraction", &self.ConsumedNVMSizeFraction)
            .field("ConsumedMappingResourcesFraction", &self.ConsumedMappingResourcesFraction)
            .field("ConsumedNVMSizeForDirtyDataFraction", &self.ConsumedNVMSizeForDirtyDataFraction)
            .field("ConsumedMappingResourcesForDirtyDataFraction", &self.ConsumedMappingResourcesForDirtyDataFraction)
            .field("Reserved1", &self.Reserved1)
            .finish()
    }
}
impl ::core::default::Default for NVCACHE_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NVCACHE_REQUEST_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.NRBSize == other.NRBSize && self.Function == other.Function && self.NRBFlags == other.NRBFlags && self.NRBStatus == other.NRBStatus && self.Count == other.Count && self.LBA == other.LBA && self.DataBufSize == other.DataBufSize && self.NVCacheStatus == other.NVCacheStatus && self.NVCacheSubStatus == other.NVCacheSubStatus
    }
}
impl ::core::cmp::Eq for NVCACHE_REQUEST_BLOCK {}
impl ::core::fmt::Debug for NVCACHE_REQUEST_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NVCACHE_REQUEST_BLOCK").field("NRBSize", &self.NRBSize).field("Function", &self.Function).field("NRBFlags", &self.NRBFlags).field("NRBStatus", &self.NRBStatus).field("Count", &self.Count).field("LBA", &self.LBA).field("DataBufSize", &self.DataBufSize).field("NVCacheStatus", &self.NVCacheStatus).field("NVCacheSubStatus", &self.NVCacheSubStatus).finish()
    }
}
impl ::core::default::Default for NVCACHE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NVCACHE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NVCACHE_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NVCACHE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NVCACHE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NVCACHE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NV_FEATURE_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NV_FEATURE_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.NVPowerModeEnabled == other.NVPowerModeEnabled && self.NVParameterReserv1 == other.NVParameterReserv1 && self.NVCmdEnabled == other.NVCmdEnabled && self.NVParameterReserv2 == other.NVParameterReserv2 && self.NVPowerModeVer == other.NVPowerModeVer && self.NVCmdVer == other.NVCmdVer && self.NVSize == other.NVSize && self.NVReadSpeed == other.NVReadSpeed && self.NVWrtSpeed == other.NVWrtSpeed && self.DeviceSpinUpTime == other.DeviceSpinUpTime
    }
}
impl ::core::cmp::Eq for NV_FEATURE_PARAMETER {}
impl ::core::fmt::Debug for NV_FEATURE_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NV_FEATURE_PARAMETER")
            .field("NVPowerModeEnabled", &self.NVPowerModeEnabled)
            .field("NVParameterReserv1", &self.NVParameterReserv1)
            .field("NVCmdEnabled", &self.NVCmdEnabled)
            .field("NVParameterReserv2", &self.NVParameterReserv2)
            .field("NVPowerModeVer", &self.NVPowerModeVer)
            .field("NVCmdVer", &self.NVCmdVer)
            .field("NVSize", &self.NVSize)
            .field("NVReadSpeed", &self.NVReadSpeed)
            .field("NVWrtSpeed", &self.NVWrtSpeed)
            .field("DeviceSpinUpTime", &self.DeviceSpinUpTime)
            .finish()
    }
}
impl ::core::default::Default for NV_SEP_CACHE_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NV_SEP_WRITE_CACHE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NV_SEP_WRITE_CACHE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NV_SEP_WRITE_CACHE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERSISTENT_ISCSI_LOGIN_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERSISTENT_ISCSI_LOGIN_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName && self.IsInformationalSession == other.IsInformationalSession && self.InitiatorInstance == other.InitiatorInstance && self.InitiatorPortNumber == other.InitiatorPortNumber && self.TargetPortal == other.TargetPortal && self.SecurityFlags == other.SecurityFlags && self.Mappings == other.Mappings && self.LoginOptions == other.LoginOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERSISTENT_ISCSI_LOGIN_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERSISTENT_ISCSI_LOGIN_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERSISTENT_ISCSI_LOGIN_INFOA").field("TargetName", &self.TargetName).field("IsInformationalSession", &self.IsInformationalSession).field("InitiatorInstance", &self.InitiatorInstance).field("InitiatorPortNumber", &self.InitiatorPortNumber).field("TargetPortal", &self.TargetPortal).field("SecurityFlags", &self.SecurityFlags).field("Mappings", &self.Mappings).field("LoginOptions", &self.LoginOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERSISTENT_ISCSI_LOGIN_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERSISTENT_ISCSI_LOGIN_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName && self.IsInformationalSession == other.IsInformationalSession && self.InitiatorInstance == other.InitiatorInstance && self.InitiatorPortNumber == other.InitiatorPortNumber && self.TargetPortal == other.TargetPortal && self.SecurityFlags == other.SecurityFlags && self.Mappings == other.Mappings && self.LoginOptions == other.LoginOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERSISTENT_ISCSI_LOGIN_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERSISTENT_ISCSI_LOGIN_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERSISTENT_ISCSI_LOGIN_INFOW").field("TargetName", &self.TargetName).field("IsInformationalSession", &self.IsInformationalSession).field("InitiatorInstance", &self.InitiatorInstance).field("InitiatorPortNumber", &self.InitiatorPortNumber).field("TargetPortal", &self.TargetPortal).field("SecurityFlags", &self.SecurityFlags).field("Mappings", &self.Mappings).field("LoginOptions", &self.LoginOptions).finish()
    }
}
impl ::core::default::Default for SCSI_ADAPTER_BUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCSI_ADAPTER_BUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfBuses == other.NumberOfBuses && self.BusData == other.BusData
    }
}
impl ::core::cmp::Eq for SCSI_ADAPTER_BUS_INFO {}
impl ::core::fmt::Debug for SCSI_ADAPTER_BUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_ADAPTER_BUS_INFO").field("NumberOfBuses", &self.NumberOfBuses).field("BusData", &self.BusData).finish()
    }
}
impl ::core::default::Default for SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCSI_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.PortNumber == other.PortNumber && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun
    }
}
impl ::core::cmp::Eq for SCSI_ADDRESS {}
impl ::core::fmt::Debug for SCSI_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_ADDRESS").field("Length", &self.Length).field("PortNumber", &self.PortNumber).field("PathId", &self.PathId).field("TargetId", &self.TargetId).field("Lun", &self.Lun).finish()
    }
}
impl ::core::default::Default for SCSI_BUS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCSI_BUS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfLogicalUnits == other.NumberOfLogicalUnits && self.InitiatorBusId == other.InitiatorBusId && self.InquiryDataOffset == other.InquiryDataOffset
    }
}
impl ::core::cmp::Eq for SCSI_BUS_DATA {}
impl ::core::fmt::Debug for SCSI_BUS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_BUS_DATA").field("NumberOfLogicalUnits", &self.NumberOfLogicalUnits).field("InitiatorBusId", &self.InitiatorBusId).field("InquiryDataOffset", &self.InquiryDataOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCSI_INQUIRY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCSI_INQUIRY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.DeviceClaimed == other.DeviceClaimed && self.InquiryDataLength == other.InquiryDataLength && self.NextInquiryDataOffset == other.NextInquiryDataOffset && self.InquiryData == other.InquiryData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCSI_INQUIRY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCSI_INQUIRY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_INQUIRY_DATA").field("PathId", &self.PathId).field("TargetId", &self.TargetId).field("Lun", &self.Lun).field("DeviceClaimed", &self.DeviceClaimed).field("InquiryDataLength", &self.InquiryDataLength).field("NextInquiryDataOffset", &self.NextInquiryDataOffset).field("InquiryData", &self.InquiryData).finish()
    }
}
impl ::core::default::Default for SCSI_LUN_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCSI_LUN_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.OSLUN == other.OSLUN && self.TargetLUN == other.TargetLUN
    }
}
impl ::core::cmp::Eq for SCSI_LUN_LIST {}
impl ::core::fmt::Debug for SCSI_LUN_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_LUN_LIST").field("OSLUN", &self.OSLUN).field("TargetLUN", &self.TargetLUN).finish()
    }
}
impl ::core::default::Default for SCSI_PASS_THROUGH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.ScsiStatus == other.ScsiStatus && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.CdbLength == other.CdbLength && self.SenseInfoLength == other.SenseInfoLength && self.DataIn == other.DataIn && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.DataBufferOffset == other.DataBufferOffset && self.SenseInfoOffset == other.SenseInfoOffset && self.Cdb == other.Cdb
    }
}
impl ::core::cmp::Eq for SCSI_PASS_THROUGH {}
impl ::core::fmt::Debug for SCSI_PASS_THROUGH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH")
            .field("Length", &self.Length)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataIn", &self.DataIn)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SCSI_PASS_THROUGH32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.ScsiStatus == other.ScsiStatus && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.CdbLength == other.CdbLength && self.SenseInfoLength == other.SenseInfoLength && self.DataIn == other.DataIn && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.DataBufferOffset == other.DataBufferOffset && self.SenseInfoOffset == other.SenseInfoOffset && self.Cdb == other.Cdb
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SCSI_PASS_THROUGH32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for SCSI_PASS_THROUGH32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH32")
            .field("Length", &self.Length)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataIn", &self.DataIn)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SCSI_PASS_THROUGH32_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH32_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Length == other.Length && self.CdbLength == other.CdbLength && self.StorAddressLength == other.StorAddressLength && self.ScsiStatus == other.ScsiStatus && self.SenseInfoLength == other.SenseInfoLength && self.DataDirection == other.DataDirection && self.Reserved == other.Reserved && self.TimeOutValue == other.TimeOutValue && self.StorAddressOffset == other.StorAddressOffset && self.SenseInfoOffset == other.SenseInfoOffset && self.DataOutTransferLength == other.DataOutTransferLength && self.DataInTransferLength == other.DataInTransferLength && self.DataOutBufferOffset == other.DataOutBufferOffset && self.DataInBufferOffset == other.DataInBufferOffset && self.Cdb == other.Cdb
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SCSI_PASS_THROUGH32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for SCSI_PASS_THROUGH32_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH32_EX")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("CdbLength", &self.CdbLength)
            .field("StorAddressLength", &self.StorAddressLength)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataDirection", &self.DataDirection)
            .field("Reserved", &self.Reserved)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("StorAddressOffset", &self.StorAddressOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("DataOutTransferLength", &self.DataOutTransferLength)
            .field("DataInTransferLength", &self.DataInTransferLength)
            .field("DataOutBufferOffset", &self.DataOutBufferOffset)
            .field("DataInBufferOffset", &self.DataInBufferOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::core::default::Default for SCSI_PASS_THROUGH_DIRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH_DIRECT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.ScsiStatus == other.ScsiStatus && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.CdbLength == other.CdbLength && self.SenseInfoLength == other.SenseInfoLength && self.DataIn == other.DataIn && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.DataBuffer == other.DataBuffer && self.SenseInfoOffset == other.SenseInfoOffset && self.Cdb == other.Cdb
    }
}
impl ::core::cmp::Eq for SCSI_PASS_THROUGH_DIRECT {}
impl ::core::fmt::Debug for SCSI_PASS_THROUGH_DIRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH_DIRECT")
            .field("Length", &self.Length)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataIn", &self.DataIn)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("DataBuffer", &self.DataBuffer)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SCSI_PASS_THROUGH_DIRECT32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH_DIRECT32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.ScsiStatus == other.ScsiStatus && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.CdbLength == other.CdbLength && self.SenseInfoLength == other.SenseInfoLength && self.DataIn == other.DataIn && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.DataBuffer == other.DataBuffer && self.SenseInfoOffset == other.SenseInfoOffset && self.Cdb == other.Cdb
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SCSI_PASS_THROUGH_DIRECT32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for SCSI_PASS_THROUGH_DIRECT32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH_DIRECT32")
            .field("Length", &self.Length)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataIn", &self.DataIn)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("DataBuffer", &self.DataBuffer)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Length == other.Length && self.CdbLength == other.CdbLength && self.StorAddressLength == other.StorAddressLength && self.ScsiStatus == other.ScsiStatus && self.SenseInfoLength == other.SenseInfoLength && self.DataDirection == other.DataDirection && self.Reserved == other.Reserved && self.TimeOutValue == other.TimeOutValue && self.StorAddressOffset == other.StorAddressOffset && self.SenseInfoOffset == other.SenseInfoOffset && self.DataOutTransferLength == other.DataOutTransferLength && self.DataInTransferLength == other.DataInTransferLength && self.DataOutBuffer == other.DataOutBuffer && self.DataInBuffer == other.DataInBuffer && self.Cdb == other.Cdb
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SCSI_PASS_THROUGH_DIRECT32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH_DIRECT32_EX")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("CdbLength", &self.CdbLength)
            .field("StorAddressLength", &self.StorAddressLength)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataDirection", &self.DataDirection)
            .field("Reserved", &self.Reserved)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("StorAddressOffset", &self.StorAddressOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("DataOutTransferLength", &self.DataOutTransferLength)
            .field("DataInTransferLength", &self.DataInTransferLength)
            .field("DataOutBuffer", &self.DataOutBuffer)
            .field("DataInBuffer", &self.DataInBuffer)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::core::default::Default for SCSI_PASS_THROUGH_DIRECT_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH_DIRECT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Length == other.Length && self.CdbLength == other.CdbLength && self.StorAddressLength == other.StorAddressLength && self.ScsiStatus == other.ScsiStatus && self.SenseInfoLength == other.SenseInfoLength && self.DataDirection == other.DataDirection && self.Reserved == other.Reserved && self.TimeOutValue == other.TimeOutValue && self.StorAddressOffset == other.StorAddressOffset && self.SenseInfoOffset == other.SenseInfoOffset && self.DataOutTransferLength == other.DataOutTransferLength && self.DataInTransferLength == other.DataInTransferLength && self.DataOutBuffer == other.DataOutBuffer && self.DataInBuffer == other.DataInBuffer && self.Cdb == other.Cdb
    }
}
impl ::core::cmp::Eq for SCSI_PASS_THROUGH_DIRECT_EX {}
impl ::core::fmt::Debug for SCSI_PASS_THROUGH_DIRECT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH_DIRECT_EX")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("CdbLength", &self.CdbLength)
            .field("StorAddressLength", &self.StorAddressLength)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataDirection", &self.DataDirection)
            .field("Reserved", &self.Reserved)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("StorAddressOffset", &self.StorAddressOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("DataOutTransferLength", &self.DataOutTransferLength)
            .field("DataInTransferLength", &self.DataInTransferLength)
            .field("DataOutBuffer", &self.DataOutBuffer)
            .field("DataInBuffer", &self.DataInBuffer)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::core::default::Default for SCSI_PASS_THROUGH_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Length == other.Length && self.CdbLength == other.CdbLength && self.StorAddressLength == other.StorAddressLength && self.ScsiStatus == other.ScsiStatus && self.SenseInfoLength == other.SenseInfoLength && self.DataDirection == other.DataDirection && self.Reserved == other.Reserved && self.TimeOutValue == other.TimeOutValue && self.StorAddressOffset == other.StorAddressOffset && self.SenseInfoOffset == other.SenseInfoOffset && self.DataOutTransferLength == other.DataOutTransferLength && self.DataInTransferLength == other.DataInTransferLength && self.DataOutBufferOffset == other.DataOutBufferOffset && self.DataInBufferOffset == other.DataInBufferOffset && self.Cdb == other.Cdb
    }
}
impl ::core::cmp::Eq for SCSI_PASS_THROUGH_EX {}
impl ::core::fmt::Debug for SCSI_PASS_THROUGH_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH_EX")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("CdbLength", &self.CdbLength)
            .field("StorAddressLength", &self.StorAddressLength)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataDirection", &self.DataDirection)
            .field("Reserved", &self.Reserved)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("StorAddressOffset", &self.StorAddressOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("DataOutTransferLength", &self.DataOutTransferLength)
            .field("DataInTransferLength", &self.DataInTransferLength)
            .field("DataOutBufferOffset", &self.DataOutBufferOffset)
            .field("DataInBufferOffset", &self.DataInBufferOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::core::default::Default for SRB_IO_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SRB_IO_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderLength == other.HeaderLength && self.Signature == other.Signature && self.Timeout == other.Timeout && self.ControlCode == other.ControlCode && self.ReturnCode == other.ReturnCode && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for SRB_IO_CONTROL {}
impl ::core::fmt::Debug for SRB_IO_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SRB_IO_CONTROL").field("HeaderLength", &self.HeaderLength).field("Signature", &self.Signature).field("Timeout", &self.Timeout).field("ControlCode", &self.ControlCode).field("ReturnCode", &self.ReturnCode).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.TargetType == other.TargetType && self.Level == other.Level && self.ProviderId == other.ProviderId && self.BufferSize == other.BufferSize && self.Reserved == other.Reserved && self.DataBuffer == other.DataBuffer
    }
}
impl ::core::cmp::Eq for STORAGE_DIAGNOSTIC_MP_REQUEST {}
impl ::core::fmt::Debug for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DIAGNOSTIC_MP_REQUEST").field("Version", &self.Version).field("Size", &self.Size).field("TargetType", &self.TargetType).field("Level", &self.Level).field("ProviderId", &self.ProviderId).field("BufferSize", &self.BufferSize).field("Reserved", &self.Reserved).field("DataBuffer", &self.DataBuffer).finish()
    }
}
impl ::core::default::Default for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.EnduranceInfo == other.EnduranceInfo
    }
}
impl ::core::cmp::Eq for STORAGE_ENDURANCE_DATA_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ENDURANCE_DATA_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("EnduranceInfo", &self.EnduranceInfo).finish()
    }
}
impl ::core::default::Default for STORAGE_ENDURANCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_ENDURANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ValidFields == other.ValidFields && self.GroupId == other.GroupId && self.Flags == other.Flags && self.LifePercentage == other.LifePercentage && self.BytesReadCount == other.BytesReadCount && self.ByteWriteCount == other.ByteWriteCount
    }
}
impl ::core::cmp::Eq for STORAGE_ENDURANCE_INFO {}
impl ::core::fmt::Debug for STORAGE_ENDURANCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ENDURANCE_INFO").field("ValidFields", &self.ValidFields).field("GroupId", &self.GroupId).field("Flags", &self.Flags).field("LifePercentage", &self.LifePercentage).field("BytesReadCount", &self.BytesReadCount).field("ByteWriteCount", &self.ByteWriteCount).finish()
    }
}
impl ::core::default::Default for STORAGE_ENDURANCE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_ENDURANCE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for STORAGE_ENDURANCE_INFO_0 {}
impl ::core::fmt::Debug for STORAGE_ENDURANCE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ENDURANCE_INFO_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for STORAGE_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_FIRMWARE_ACTIVATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.SlotToActivate == other.SlotToActivate && self.Reserved0 == other.Reserved0
    }
}
impl ::core::cmp::Eq for STORAGE_FIRMWARE_ACTIVATE {}
impl ::core::fmt::Debug for STORAGE_FIRMWARE_ACTIVATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FIRMWARE_ACTIVATE").field("Version", &self.Version).field("Size", &self.Size).field("SlotToActivate", &self.SlotToActivate).field("Reserved0", &self.Reserved0).finish()
    }
}
impl ::core::default::Default for STORAGE_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_FIRMWARE_DOWNLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Offset == other.Offset && self.BufferSize == other.BufferSize && self.ImageBuffer == other.ImageBuffer
    }
}
impl ::core::cmp::Eq for STORAGE_FIRMWARE_DOWNLOAD {}
impl ::core::fmt::Debug for STORAGE_FIRMWARE_DOWNLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FIRMWARE_DOWNLOAD").field("Version", &self.Version).field("Size", &self.Size).field("Offset", &self.Offset).field("BufferSize", &self.BufferSize).field("ImageBuffer", &self.ImageBuffer).finish()
    }
}
impl ::core::default::Default for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Offset == other.Offset && self.BufferSize == other.BufferSize && self.Slot == other.Slot && self.Reserved == other.Reserved && self.ImageSize == other.ImageSize && self.ImageBuffer == other.ImageBuffer
    }
}
impl ::core::cmp::Eq for STORAGE_FIRMWARE_DOWNLOAD_V2 {}
impl ::core::fmt::Debug for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FIRMWARE_DOWNLOAD_V2").field("Version", &self.Version).field("Size", &self.Size).field("Offset", &self.Offset).field("BufferSize", &self.BufferSize).field("Slot", &self.Slot).field("Reserved", &self.Reserved).field("ImageSize", &self.ImageSize).field("ImageBuffer", &self.ImageBuffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_FIRMWARE_INFO_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_FIRMWARE_INFO_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.UpgradeSupport == other.UpgradeSupport && self.SlotCount == other.SlotCount && self.ActiveSlot == other.ActiveSlot && self.PendingActivateSlot == other.PendingActivateSlot && self.FirmwareShared == other.FirmwareShared && self.Reserved == other.Reserved && self.ImagePayloadAlignment == other.ImagePayloadAlignment && self.ImagePayloadMaxSize == other.ImagePayloadMaxSize && self.Slot == other.Slot
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_FIRMWARE_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_FIRMWARE_INFO_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FIRMWARE_INFO_V2")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("UpgradeSupport", &self.UpgradeSupport)
            .field("SlotCount", &self.SlotCount)
            .field("ActiveSlot", &self.ActiveSlot)
            .field("PendingActivateSlot", &self.PendingActivateSlot)
            .field("FirmwareShared", &self.FirmwareShared)
            .field("Reserved", &self.Reserved)
            .field("ImagePayloadAlignment", &self.ImagePayloadAlignment)
            .field("ImagePayloadMaxSize", &self.ImagePayloadMaxSize)
            .field("Slot", &self.Slot)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.SlotNumber == other.SlotNumber && self.ReadOnly == other.ReadOnly && self.Reserved == other.Reserved && self.Revision == other.Revision
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_FIRMWARE_SLOT_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FIRMWARE_SLOT_INFO_V2").field("SlotNumber", &self.SlotNumber).field("ReadOnly", &self.ReadOnly).field("Reserved", &self.Reserved).field("Revision", &self.Revision).finish()
    }
}
impl ::core::default::Default for TARGETPROTOCOLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TARGETPROTOCOLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGETPROTOCOLTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TARGET_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TARGET_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_INFORMATION_CLASS").field(&self.0).finish()
    }
}

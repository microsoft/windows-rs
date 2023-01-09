impl ::core::default::Default for FILTER_AGGREGATE_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FILTER_AGGREGATE_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FILTER_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILTER_FULL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.FrameID == other.FrameID && self.NumberOfInstances == other.NumberOfInstances && self.FilterNameLength == other.FilterNameLength && self.FilterNameBuffer == other.FilterNameBuffer
    }
}
impl ::core::cmp::Eq for FILTER_FULL_INFORMATION {}
impl ::core::fmt::Debug for FILTER_FULL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_FULL_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("FrameID", &self.FrameID).field("NumberOfInstances", &self.NumberOfInstances).field("FilterNameLength", &self.FilterNameLength).field("FilterNameBuffer", &self.FilterNameBuffer).finish()
    }
}
impl ::core::default::Default for FILTER_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILTER_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILTER_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILTER_MESSAGE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILTER_MESSAGE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.ReplyLength == other.ReplyLength && self.MessageId == other.MessageId
    }
}
impl ::core::cmp::Eq for FILTER_MESSAGE_HEADER {}
impl ::core::fmt::Debug for FILTER_MESSAGE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_MESSAGE_HEADER").field("ReplyLength", &self.ReplyLength).field("MessageId", &self.MessageId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILTER_REPLY_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILTER_REPLY_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.MessageId == other.MessageId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILTER_REPLY_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILTER_REPLY_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_REPLY_HEADER").field("Status", &self.Status).field("MessageId", &self.MessageId).finish()
    }
}
impl ::core::default::Default for FILTER_VOLUME_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILTER_VOLUME_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.FilterVolumeNameLength == other.FilterVolumeNameLength && self.FilterVolumeName == other.FilterVolumeName
    }
}
impl ::core::cmp::Eq for FILTER_VOLUME_BASIC_INFORMATION {}
impl ::core::fmt::Debug for FILTER_VOLUME_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_VOLUME_BASIC_INFORMATION").field("FilterVolumeNameLength", &self.FilterVolumeNameLength).field("FilterVolumeName", &self.FilterVolumeName).finish()
    }
}
impl ::core::default::Default for FILTER_VOLUME_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILTER_VOLUME_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILTER_VOLUME_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILTER_VOLUME_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILTER_VOLUME_STANDARD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.Flags == other.Flags && self.FrameID == other.FrameID && self.FileSystemType == other.FileSystemType && self.FilterVolumeNameLength == other.FilterVolumeNameLength && self.FilterVolumeName == other.FilterVolumeName
    }
}
impl ::core::cmp::Eq for FILTER_VOLUME_STANDARD_INFORMATION {}
impl ::core::fmt::Debug for FILTER_VOLUME_STANDARD_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_VOLUME_STANDARD_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("Flags", &self.Flags).field("FrameID", &self.FrameID).field("FileSystemType", &self.FileSystemType).field("FilterVolumeNameLength", &self.FilterVolumeNameLength).field("FilterVolumeName", &self.FilterVolumeName).finish()
    }
}
impl ::core::default::Default for FLT_FILESYSTEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FLT_FILESYSTEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FLT_FILESYSTEM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for INSTANCE_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INSTANCE_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.InstanceNameLength == other.InstanceNameLength && self.InstanceNameBufferOffset == other.InstanceNameBufferOffset
    }
}
impl ::core::cmp::Eq for INSTANCE_BASIC_INFORMATION {}
impl ::core::fmt::Debug for INSTANCE_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTANCE_BASIC_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("InstanceNameLength", &self.InstanceNameLength).field("InstanceNameBufferOffset", &self.InstanceNameBufferOffset).finish()
    }
}
impl ::core::default::Default for INSTANCE_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INSTANCE_FULL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.InstanceNameLength == other.InstanceNameLength && self.InstanceNameBufferOffset == other.InstanceNameBufferOffset && self.AltitudeLength == other.AltitudeLength && self.AltitudeBufferOffset == other.AltitudeBufferOffset && self.VolumeNameLength == other.VolumeNameLength && self.VolumeNameBufferOffset == other.VolumeNameBufferOffset && self.FilterNameLength == other.FilterNameLength && self.FilterNameBufferOffset == other.FilterNameBufferOffset
    }
}
impl ::core::cmp::Eq for INSTANCE_FULL_INFORMATION {}
impl ::core::fmt::Debug for INSTANCE_FULL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTANCE_FULL_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("InstanceNameLength", &self.InstanceNameLength)
            .field("InstanceNameBufferOffset", &self.InstanceNameBufferOffset)
            .field("AltitudeLength", &self.AltitudeLength)
            .field("AltitudeBufferOffset", &self.AltitudeBufferOffset)
            .field("VolumeNameLength", &self.VolumeNameLength)
            .field("VolumeNameBufferOffset", &self.VolumeNameBufferOffset)
            .field("FilterNameLength", &self.FilterNameLength)
            .field("FilterNameBufferOffset", &self.FilterNameBufferOffset)
            .finish()
    }
}
impl ::core::default::Default for INSTANCE_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INSTANCE_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTANCE_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for INSTANCE_PARTIAL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INSTANCE_PARTIAL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.InstanceNameLength == other.InstanceNameLength && self.InstanceNameBufferOffset == other.InstanceNameBufferOffset && self.AltitudeLength == other.AltitudeLength && self.AltitudeBufferOffset == other.AltitudeBufferOffset
    }
}
impl ::core::cmp::Eq for INSTANCE_PARTIAL_INFORMATION {}
impl ::core::fmt::Debug for INSTANCE_PARTIAL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTANCE_PARTIAL_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("InstanceNameLength", &self.InstanceNameLength).field("InstanceNameBufferOffset", &self.InstanceNameBufferOffset).field("AltitudeLength", &self.AltitudeLength).field("AltitudeBufferOffset", &self.AltitudeBufferOffset).finish()
    }
}

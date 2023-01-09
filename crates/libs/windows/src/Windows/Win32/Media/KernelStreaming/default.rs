impl ::core::default::Default for ALLOCATOR_PROPERTIES_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ALLOCATOR_PROPERTIES_EX {
    fn eq(&self, other: &Self) -> bool {
        self.cBuffers == other.cBuffers
            && self.cbBuffer == other.cbBuffer
            && self.cbAlign == other.cbAlign
            && self.cbPrefix == other.cbPrefix
            && self.MemoryType == other.MemoryType
            && self.BusType == other.BusType
            && self.State == other.State
            && self.Input == other.Input
            && self.Output == other.Output
            && self.Strategy == other.Strategy
            && self.Flags == other.Flags
            && self.Weight == other.Weight
            && self.LogicalMemoryType == other.LogicalMemoryType
            && self.AllocatorPlace == other.AllocatorPlace
            && self.Dimensions == other.Dimensions
            && self.PhysicalRange == other.PhysicalRange
            && self.PrevSegment == other.PrevSegment
            && self.CountNextSegments == other.CountNextSegments
            && self.NextSegments == other.NextSegments
            && self.InsideFactors == other.InsideFactors
            && self.NumberPins == other.NumberPins
    }
}
impl ::core::cmp::Eq for ALLOCATOR_PROPERTIES_EX {}
impl ::core::fmt::Debug for ALLOCATOR_PROPERTIES_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ALLOCATOR_PROPERTIES_EX")
            .field("cBuffers", &self.cBuffers)
            .field("cbBuffer", &self.cbBuffer)
            .field("cbAlign", &self.cbAlign)
            .field("cbPrefix", &self.cbPrefix)
            .field("MemoryType", &self.MemoryType)
            .field("BusType", &self.BusType)
            .field("State", &self.State)
            .field("Input", &self.Input)
            .field("Output", &self.Output)
            .field("Strategy", &self.Strategy)
            .field("Flags", &self.Flags)
            .field("Weight", &self.Weight)
            .field("LogicalMemoryType", &self.LogicalMemoryType)
            .field("AllocatorPlace", &self.AllocatorPlace)
            .field("Dimensions", &self.Dimensions)
            .field("PhysicalRange", &self.PhysicalRange)
            .field("PrevSegment", &self.PrevSegment)
            .field("CountNextSegments", &self.CountNextSegments)
            .field("NextSegments", &self.NextSegments)
            .field("InsideFactors", &self.InsideFactors)
            .field("NumberPins", &self.NumberPins)
            .finish()
    }
}
impl ::core::default::Default for AUDIOPOSTURE_ORIENTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDIOPOSTURE_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIOPOSTURE_ORIENTATION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    fn eq(&self, other: &Self) -> bool {
        self.ResourceGroupAcquired == other.ResourceGroupAcquired && self.ResourceGroupName == other.ResourceGroupName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIORESOURCEMANAGEMENT_RESOURCEGROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIORESOURCEMANAGEMENT_RESOURCEGROUP").field("ResourceGroupAcquired", &self.ResourceGroupAcquired).field("ResourceGroupName", &self.ResourceGroupName).finish()
    }
}
impl ::core::default::Default for AUDIO_CURVE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDIO_CURVE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_CURVE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CAPTURE_MEMORY_ALLOCATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CAPTURE_MEMORY_ALLOCATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CAPTURE_MEMORY_ALLOCATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CC_BYTE_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CC_BYTE_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.Decoded == other.Decoded && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for CC_BYTE_PAIR {}
impl ::core::fmt::Debug for CC_BYTE_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CC_BYTE_PAIR").field("Decoded", &self.Decoded).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for CC_HW_FIELD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CC_HW_FIELD {
    fn eq(&self, other: &Self) -> bool {
        self.ScanlinesRequested == other.ScanlinesRequested && self.fieldFlags == other.fieldFlags && self.PictureNumber == other.PictureNumber && self.Lines == other.Lines
    }
}
impl ::core::cmp::Eq for CC_HW_FIELD {}
impl ::core::fmt::Debug for CC_HW_FIELD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CC_HW_FIELD").field("ScanlinesRequested", &self.ScanlinesRequested).field("fieldFlags", &self.fieldFlags).field("PictureNumber", &self.PictureNumber).field("Lines", &self.Lines).finish()
    }
}
impl ::core::default::Default for CONSTRICTOR_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONSTRICTOR_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONSTRICTOR_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.CanRecord == other.CanRecord
            && self.CanRecordStrobe == other.CanRecordStrobe
            && self.HasAudio == other.HasAudio
            && self.HasVideo == other.HasVideo
            && self.UsesFiles == other.UsesFiles
            && self.CanSave == other.CanSave
            && self.DeviceType == other.DeviceType
            && self.TCRead == other.TCRead
            && self.TCWrite == other.TCWrite
            && self.CTLRead == other.CTLRead
            && self.IndexRead == other.IndexRead
            && self.Preroll == other.Preroll
            && self.Postroll == other.Postroll
            && self.SyncAcc == other.SyncAcc
            && self.NormRate == other.NormRate
            && self.CanPreview == other.CanPreview
            && self.CanMonitorSrc == other.CanMonitorSrc
            && self.CanTest == other.CanTest
            && self.VideoIn == other.VideoIn
            && self.AudioIn == other.AudioIn
            && self.Calibrate == other.Calibrate
            && self.SeekType == other.SeekType
            && self.SimulatedHardware == other.SimulatedHardware
    }
}
impl ::core::cmp::Eq for DEVCAPS {}
impl ::core::fmt::Debug for DEVCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVCAPS")
            .field("CanRecord", &self.CanRecord)
            .field("CanRecordStrobe", &self.CanRecordStrobe)
            .field("HasAudio", &self.HasAudio)
            .field("HasVideo", &self.HasVideo)
            .field("UsesFiles", &self.UsesFiles)
            .field("CanSave", &self.CanSave)
            .field("DeviceType", &self.DeviceType)
            .field("TCRead", &self.TCRead)
            .field("TCWrite", &self.TCWrite)
            .field("CTLRead", &self.CTLRead)
            .field("IndexRead", &self.IndexRead)
            .field("Preroll", &self.Preroll)
            .field("Postroll", &self.Postroll)
            .field("SyncAcc", &self.SyncAcc)
            .field("NormRate", &self.NormRate)
            .field("CanPreview", &self.CanPreview)
            .field("CanMonitorSrc", &self.CanMonitorSrc)
            .field("CanTest", &self.CanTest)
            .field("VideoIn", &self.VideoIn)
            .field("AudioIn", &self.AudioIn)
            .field("Calibrate", &self.Calibrate)
            .field("SeekType", &self.SeekType)
            .field("SimulatedHardware", &self.SimulatedHardware)
            .finish()
    }
}
impl ::core::default::Default for DS3DVECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for EPcxConnectionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EPcxConnectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPcxConnectionType").field(&self.0).finish()
    }
}
impl ::core::default::Default for EPcxGenLocation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EPcxGenLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPcxGenLocation").field(&self.0).finish()
    }
}
impl ::core::default::Default for EPcxGeoLocation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EPcxGeoLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPcxGeoLocation").field(&self.0).finish()
    }
}
impl ::core::default::Default for EPxcPortConnection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EPxcPortConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPxcPortConnection").field(&self.0).finish()
    }
}
impl ::core::default::Default for FRAMING_CACHE_OPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FRAMING_CACHE_OPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FRAMING_CACHE_OPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FRAMING_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FRAMING_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FRAMING_PROP").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKsAggregateControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsAggregateControl {}
impl ::core::fmt::Debug for IKsAggregateControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsAggregateControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKsControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsControl {}
impl ::core::fmt::Debug for IKsControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKsFormatSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsFormatSupport {}
impl ::core::fmt::Debug for IKsFormatSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsFormatSupport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKsJackContainerId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsJackContainerId {}
impl ::core::fmt::Debug for IKsJackContainerId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsJackContainerId").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKsJackDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsJackDescription {}
impl ::core::fmt::Debug for IKsJackDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsJackDescription").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKsJackDescription2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsJackDescription2 {}
impl ::core::fmt::Debug for IKsJackDescription2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsJackDescription2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKsJackSinkInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsJackSinkInformation {}
impl ::core::fmt::Debug for IKsJackSinkInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsJackSinkInformation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKsPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsPropertySet {}
impl ::core::fmt::Debug for IKsPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsPropertySet").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKsTopology {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKsTopology {}
impl ::core::fmt::Debug for IKsTopology {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKsTopology").field(&self.0).finish()
    }
}
impl ::core::default::Default for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.PrimaryChannelCount == other.PrimaryChannelCount && self.PrimaryChannelStartPosition == other.PrimaryChannelStartPosition && self.PrimaryChannelMask == other.PrimaryChannelMask && self.InterleavedChannelCount == other.InterleavedChannelCount && self.InterleavedChannelStartPosition == other.InterleavedChannelStartPosition && self.InterleavedChannelMask == other.InterleavedChannelMask
    }
}
impl ::core::cmp::Eq for INTERLEAVED_AUDIO_FORMAT_INFORMATION {}
impl ::core::fmt::Debug for INTERLEAVED_AUDIO_FORMAT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERLEAVED_AUDIO_FORMAT_INFORMATION").field("Size", &self.Size).field("PrimaryChannelCount", &self.PrimaryChannelCount).field("PrimaryChannelStartPosition", &self.PrimaryChannelStartPosition).field("PrimaryChannelMask", &self.PrimaryChannelMask).field("InterleavedChannelCount", &self.InterleavedChannelCount).field("InterleavedChannelStartPosition", &self.InterleavedChannelStartPosition).field("InterleavedChannelMask", &self.InterleavedChannelMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAC3_ALTERNATE_AUDIO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAC3_ALTERNATE_AUDIO {
    fn eq(&self, other: &Self) -> bool {
        self.fStereo == other.fStereo && self.DualMode == other.DualMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAC3_ALTERNATE_AUDIO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAC3_ALTERNATE_AUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAC3_ALTERNATE_AUDIO").field("fStereo", &self.fStereo).field("DualMode", &self.DualMode).finish()
    }
}
impl ::core::default::Default for KSAC3_BIT_STREAM_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAC3_BIT_STREAM_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.BitStreamMode == other.BitStreamMode
    }
}
impl ::core::cmp::Eq for KSAC3_BIT_STREAM_MODE {}
impl ::core::fmt::Debug for KSAC3_BIT_STREAM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAC3_BIT_STREAM_MODE").field("BitStreamMode", &self.BitStreamMode).finish()
    }
}
impl ::core::default::Default for KSAC3_DIALOGUE_LEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAC3_DIALOGUE_LEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.DialogueLevel == other.DialogueLevel
    }
}
impl ::core::cmp::Eq for KSAC3_DIALOGUE_LEVEL {}
impl ::core::fmt::Debug for KSAC3_DIALOGUE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAC3_DIALOGUE_LEVEL").field("DialogueLevel", &self.DialogueLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAC3_DOWNMIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAC3_DOWNMIX {
    fn eq(&self, other: &Self) -> bool {
        self.fDownMix == other.fDownMix && self.fDolbySurround == other.fDolbySurround
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAC3_DOWNMIX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAC3_DOWNMIX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAC3_DOWNMIX").field("fDownMix", &self.fDownMix).field("fDolbySurround", &self.fDolbySurround).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAC3_ERROR_CONCEALMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAC3_ERROR_CONCEALMENT {
    fn eq(&self, other: &Self) -> bool {
        self.fRepeatPreviousBlock == other.fRepeatPreviousBlock && self.fErrorInCurrentBlock == other.fErrorInCurrentBlock
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAC3_ERROR_CONCEALMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAC3_ERROR_CONCEALMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAC3_ERROR_CONCEALMENT").field("fRepeatPreviousBlock", &self.fRepeatPreviousBlock).field("fErrorInCurrentBlock", &self.fErrorInCurrentBlock).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAC3_ROOM_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAC3_ROOM_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.fLargeRoom == other.fLargeRoom
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAC3_ROOM_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAC3_ROOM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAC3_ROOM_TYPE").field("fLargeRoom", &self.fLargeRoom).finish()
    }
}
impl ::core::default::Default for KSALLOCATORMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSALLOCATORMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSALLOCATORMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSALLOCATOR_FRAMING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSALLOCATOR_FRAMING_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.Attribute == other.Attribute
    }
}
impl ::core::cmp::Eq for KSATTRIBUTE {}
impl ::core::fmt::Debug for KSATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSATTRIBUTE").field("Size", &self.Size).field("Flags", &self.Flags).field("Attribute", &self.Attribute).finish()
    }
}
impl ::core::default::Default for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.AttributeHeader == other.AttributeHeader && self.SignalProcessingMode == other.SignalProcessingMode
    }
}
impl ::core::cmp::Eq for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {}
impl ::core::fmt::Debug for KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSATTRIBUTE_AUDIOSIGNALPROCESSING_MODE").field("AttributeHeader", &self.AttributeHeader).field("SignalProcessingMode", &self.SignalProcessingMode).finish()
    }
}
impl ::core::default::Default for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.MinBufferBytes == other.MinBufferBytes && self.MaxBufferBytes == other.MaxBufferBytes
    }
}
impl ::core::cmp::Eq for KSAUDIOENGINE_BUFFER_SIZE_RANGE {}
impl ::core::fmt::Debug for KSAUDIOENGINE_BUFFER_SIZE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIOENGINE_BUFFER_SIZE_RANGE").field("MinBufferBytes", &self.MinBufferBytes).field("MaxBufferBytes", &self.MaxBufferBytes).finish()
    }
}
impl ::core::default::Default for KSAUDIOENGINE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIOENGINE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.nHostPinId == other.nHostPinId && self.nOffloadPinId == other.nOffloadPinId && self.nLoopbackPinId == other.nLoopbackPinId
    }
}
impl ::core::cmp::Eq for KSAUDIOENGINE_DESCRIPTOR {}
impl ::core::fmt::Debug for KSAUDIOENGINE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIOENGINE_DESCRIPTOR").field("nHostPinId", &self.nHostPinId).field("nOffloadPinId", &self.nOffloadPinId).field("nLoopbackPinId", &self.nLoopbackPinId).finish()
    }
}
impl ::core::default::Default for KSAUDIOENGINE_VOLUMELEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIOENGINE_VOLUMELEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.TargetVolume == other.TargetVolume && self.CurveType == other.CurveType && self.CurveDuration == other.CurveDuration
    }
}
impl ::core::cmp::Eq for KSAUDIOENGINE_VOLUMELEVEL {}
impl ::core::fmt::Debug for KSAUDIOENGINE_VOLUMELEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIOENGINE_VOLUMELEVEL").field("TargetVolume", &self.TargetVolume).field("CurveType", &self.CurveType).field("CurveDuration", &self.CurveDuration).finish()
    }
}
impl ::core::default::Default for KSAUDIOMODULE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIOMODULE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.ClassId == other.ClassId && self.InstanceId == other.InstanceId && self.VersionMajor == other.VersionMajor && self.VersionMinor == other.VersionMinor && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for KSAUDIOMODULE_DESCRIPTOR {}
impl ::core::fmt::Debug for KSAUDIOMODULE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIOMODULE_DESCRIPTOR").field("ClassId", &self.ClassId).field("InstanceId", &self.InstanceId).field("VersionMajor", &self.VersionMajor).field("VersionMinor", &self.VersionMinor).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for KSAUDIOMODULE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSAUDIOMODULE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSAUDIO_CHANNEL_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_CHANNEL_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.ActiveSpeakerPositions == other.ActiveSpeakerPositions
    }
}
impl ::core::cmp::Eq for KSAUDIO_CHANNEL_CONFIG {}
impl ::core::fmt::Debug for KSAUDIO_CHANNEL_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_CHANNEL_CONFIG").field("ActiveSpeakerPositions", &self.ActiveSpeakerPositions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_COPY_PROTECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAUDIO_COPY_PROTECTION {
    fn eq(&self, other: &Self) -> bool {
        self.fCopyrighted == other.fCopyrighted && self.fOriginal == other.fOriginal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAUDIO_COPY_PROTECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAUDIO_COPY_PROTECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_COPY_PROTECTION").field("fCopyrighted", &self.fCopyrighted).field("fOriginal", &self.fOriginal).finish()
    }
}
impl ::core::default::Default for KSAUDIO_DYNAMIC_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_DYNAMIC_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.QuietCompression == other.QuietCompression && self.LoudCompression == other.LoudCompression
    }
}
impl ::core::cmp::Eq for KSAUDIO_DYNAMIC_RANGE {}
impl ::core::fmt::Debug for KSAUDIO_DYNAMIC_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_DYNAMIC_RANGE").field("QuietCompression", &self.QuietCompression).field("LoudCompression", &self.LoudCompression).finish()
    }
}
impl ::core::default::Default for KSAUDIO_MICROPHONE_COORDINATES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_MICROPHONE_COORDINATES {
    fn eq(&self, other: &Self) -> bool {
        self.usType == other.usType && self.wXCoord == other.wXCoord && self.wYCoord == other.wYCoord && self.wZCoord == other.wZCoord && self.wVerticalAngle == other.wVerticalAngle && self.wHorizontalAngle == other.wHorizontalAngle
    }
}
impl ::core::cmp::Eq for KSAUDIO_MICROPHONE_COORDINATES {}
impl ::core::fmt::Debug for KSAUDIO_MICROPHONE_COORDINATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_MICROPHONE_COORDINATES").field("usType", &self.usType).field("wXCoord", &self.wXCoord).field("wYCoord", &self.wYCoord).field("wZCoord", &self.wZCoord).field("wVerticalAngle", &self.wVerticalAngle).field("wHorizontalAngle", &self.wHorizontalAngle).finish()
    }
}
impl ::core::default::Default for KSAUDIO_MIC_ARRAY_GEOMETRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_MIC_ARRAY_GEOMETRY {
    fn eq(&self, other: &Self) -> bool {
        self.usVersion == other.usVersion && self.usMicArrayType == other.usMicArrayType && self.wVerticalAngleBegin == other.wVerticalAngleBegin && self.wVerticalAngleEnd == other.wVerticalAngleEnd && self.wHorizontalAngleBegin == other.wHorizontalAngleBegin && self.wHorizontalAngleEnd == other.wHorizontalAngleEnd && self.usFrequencyBandLo == other.usFrequencyBandLo && self.usFrequencyBandHi == other.usFrequencyBandHi && self.usNumberOfMicrophones == other.usNumberOfMicrophones && self.KsMicCoord == other.KsMicCoord
    }
}
impl ::core::cmp::Eq for KSAUDIO_MIC_ARRAY_GEOMETRY {}
impl ::core::fmt::Debug for KSAUDIO_MIC_ARRAY_GEOMETRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_MIC_ARRAY_GEOMETRY")
            .field("usVersion", &self.usVersion)
            .field("usMicArrayType", &self.usMicArrayType)
            .field("wVerticalAngleBegin", &self.wVerticalAngleBegin)
            .field("wVerticalAngleEnd", &self.wVerticalAngleEnd)
            .field("wHorizontalAngleBegin", &self.wHorizontalAngleBegin)
            .field("wHorizontalAngleEnd", &self.wHorizontalAngleEnd)
            .field("usFrequencyBandLo", &self.usFrequencyBandLo)
            .field("usFrequencyBandHi", &self.usFrequencyBandHi)
            .field("usNumberOfMicrophones", &self.usNumberOfMicrophones)
            .field("KsMicCoord", &self.KsMicCoord)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_MIXCAP_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_MIXLEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSAUDIO_MIXLEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.Mute == other.Mute && self.Level == other.Level
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSAUDIO_MIXLEVEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSAUDIO_MIXLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_MIXLEVEL").field("Mute", &self.Mute).field("Level", &self.Level).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSAUDIO_MIX_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    fn eq(&self, other: &Self) -> bool {
        self.MinPacketPeriodInHns == other.MinPacketPeriodInHns && self.PacketSizeFileAlignment == other.PacketSizeFileAlignment && self.Reserved == other.Reserved && self.NumProcessingModeConstraints == other.NumProcessingModeConstraints && self.ProcessingModeConstraints == other.ProcessingModeConstraints
    }
}
impl ::core::cmp::Eq for KSAUDIO_PACKETSIZE_CONSTRAINTS {}
impl ::core::fmt::Debug for KSAUDIO_PACKETSIZE_CONSTRAINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_PACKETSIZE_CONSTRAINTS").field("MinPacketPeriodInHns", &self.MinPacketPeriodInHns).field("PacketSizeFileAlignment", &self.PacketSizeFileAlignment).field("Reserved", &self.Reserved).field("NumProcessingModeConstraints", &self.NumProcessingModeConstraints).field("ProcessingModeConstraints", &self.ProcessingModeConstraints).finish()
    }
}
impl ::core::default::Default for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    fn eq(&self, other: &Self) -> bool {
        self.MinPacketPeriodInHns == other.MinPacketPeriodInHns && self.PacketSizeFileAlignment == other.PacketSizeFileAlignment && self.MaxPacketSizeInBytes == other.MaxPacketSizeInBytes && self.NumProcessingModeConstraints == other.NumProcessingModeConstraints && self.ProcessingModeConstraints == other.ProcessingModeConstraints
    }
}
impl ::core::cmp::Eq for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {}
impl ::core::fmt::Debug for KSAUDIO_PACKETSIZE_CONSTRAINTS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_PACKETSIZE_CONSTRAINTS2").field("MinPacketPeriodInHns", &self.MinPacketPeriodInHns).field("PacketSizeFileAlignment", &self.PacketSizeFileAlignment).field("MaxPacketSizeInBytes", &self.MaxPacketSizeInBytes).field("NumProcessingModeConstraints", &self.NumProcessingModeConstraints).field("ProcessingModeConstraints", &self.ProcessingModeConstraints).finish()
    }
}
impl ::core::default::Default for KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessingMode == other.ProcessingMode && self.SamplesPerProcessingPacket == other.SamplesPerProcessingPacket && self.ProcessingPacketDurationInHns == other.ProcessingPacketDurationInHns
    }
}
impl ::core::cmp::Eq for KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT {}
impl ::core::fmt::Debug for KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_PACKETSIZE_PROCESSINGMODE_CONSTRAINT").field("ProcessingMode", &self.ProcessingMode).field("SamplesPerProcessingPacket", &self.SamplesPerProcessingPacket).field("ProcessingPacketDurationInHns", &self.ProcessingPacketDurationInHns).finish()
    }
}
impl ::core::default::Default for KSAUDIO_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.PlayOffset == other.PlayOffset && self.WriteOffset == other.WriteOffset
    }
}
impl ::core::cmp::Eq for KSAUDIO_POSITION {}
impl ::core::fmt::Debug for KSAUDIO_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_POSITION").field("PlayOffset", &self.PlayOffset).field("WriteOffset", &self.WriteOffset).finish()
    }
}
impl ::core::default::Default for KSAUDIO_POSITIONEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_POSITIONEX {
    fn eq(&self, other: &Self) -> bool {
        self.TimerFrequency == other.TimerFrequency && self.TimeStamp1 == other.TimeStamp1 && self.Position == other.Position && self.TimeStamp2 == other.TimeStamp2
    }
}
impl ::core::cmp::Eq for KSAUDIO_POSITIONEX {}
impl ::core::fmt::Debug for KSAUDIO_POSITIONEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_POSITIONEX").field("TimerFrequency", &self.TimerFrequency).field("TimeStamp1", &self.TimeStamp1).field("Position", &self.Position).field("TimeStamp2", &self.TimeStamp2).finish()
    }
}
impl ::core::default::Default for KSAUDIO_PRESENTATION_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSAUDIO_PRESENTATION_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.u64PositionInBlocks == other.u64PositionInBlocks && self.u64QPCPosition == other.u64QPCPosition
    }
}
impl ::core::cmp::Eq for KSAUDIO_PRESENTATION_POSITION {}
impl ::core::fmt::Debug for KSAUDIO_PRESENTATION_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSAUDIO_PRESENTATION_POSITION").field("u64PositionInBlocks", &self.u64PositionInBlocks).field("u64QPCPosition", &self.u64QPCPosition).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Resolution == other.Resolution && self.MaxFrameRate == other.MaxFrameRate && self.MaskResolution == other.MaskResolution && self.SubType == other.SubType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS").field("Resolution", &self.Resolution).field("MaxFrameRate", &self.MaxFrameRate).field("MaskResolution", &self.MaskResolution).field("SubType", &self.SubType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_BACKGROUNDSEGMENTATION_CONFIGCAPS_0").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.PitchAngle == other.PitchAngle && self.YawAngle == other.YawAngle && self.Flag == other.Flag && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_CAMERAOFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_CAMERAOFFSET").field("PitchAngle", &self.PitchAngle).field("YawAngle", &self.YawAngle).field("Flag", &self.Flag).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.ResolutionX == other.ResolutionX && self.ResolutionY == other.ResolutionY && self.PorchTop == other.PorchTop && self.PorchLeft == other.PorchLeft && self.PorchBottom == other.PorchBottom && self.PorchRight == other.PorchRight && self.NonUpscalingWindowSize == other.NonUpscalingWindowSize && self.MinWindowSize == other.MinWindowSize && self.MaxWindowSize == other.MaxWindowSize && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPS")
            .field("ResolutionX", &self.ResolutionX)
            .field("ResolutionY", &self.ResolutionY)
            .field("PorchTop", &self.PorchTop)
            .field("PorchLeft", &self.PorchLeft)
            .field("PorchBottom", &self.PorchBottom)
            .field("PorchRight", &self.PorchRight)
            .field("NonUpscalingWindowSize", &self.NonUpscalingWindowSize)
            .field("MinWindowSize", &self.MinWindowSize)
            .field("MaxWindowSize", &self.MaxWindowSize)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_CONFIGCAPSHEADER").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    fn eq(&self, other: &Self) -> bool {
        self.OriginX == other.OriginX && self.OriginY == other.OriginY && self.WindowSize == other.WindowSize && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_DIGITALWINDOW_SETTING").field("OriginX", &self.OriginX).field("OriginY", &self.OriginY).field("WindowSize", &self.WindowSize).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.Min == other.Min && self.Max == other.Max && self.Value == other.Value && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_EVCOMPENSATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_EVCOMPENSATION").field("Mode", &self.Mode).field("Min", &self.Min).field("Max", &self.Max).field("Value", &self.Value).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    fn eq(&self, other: &Self) -> bool {
        self.NormalizedFocalLengthX == other.NormalizedFocalLengthX && self.NormalizedFocalLengthY == other.NormalizedFocalLengthY && self.Flag == other.Flag && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_FIELDOFVIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_FIELDOFVIEW").field("NormalizedFocalLengthX", &self.NormalizedFocalLengthX).field("NormalizedFocalLengthY", &self.NormalizedFocalLengthY).field("Flag", &self.Flag).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_FOCUSSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_FOCUSSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_EXTENDEDPROP_FOCUSSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.PinId == other.PinId && self.Size == other.Size && self.Result == other.Result && self.Flags == other.Flags && self.Capability == other.Capability
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_HEADER {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_HEADER").field("Version", &self.Version).field("PinId", &self.PinId).field("Size", &self.Size).field("Result", &self.Result).field("Flags", &self.Flags).field("Capability", &self.Capability).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    fn eq(&self, other: &Self) -> bool {
        self.BufferAlignment == other.BufferAlignment && self.MaxMetadataBufferSize == other.MaxMetadataBufferSize
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_METADATAINFO {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_METADATAINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_METADATAINFO").field("BufferAlignment", &self.BufferAlignment).field("MaxMetadataBufferSize", &self.MaxMetadataBufferSize).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_MetadataAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_MetadataAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_EXTENDEDPROP_MetadataAlignment").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    fn eq(&self, other: &Self) -> bool {
        self.RequestedHistoryFrames == other.RequestedHistoryFrames && self.MaxHistoryFrames == other.MaxHistoryFrames && self.SubMode == other.SubMode && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_PHOTOMODE {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_PHOTOMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_PHOTOMODE").field("RequestedHistoryFrames", &self.RequestedHistoryFrames).field("MaxHistoryFrames", &self.MaxHistoryFrames).field("SubMode", &self.SubMode).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.ProfileId == other.ProfileId && self.Index == other.Index && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_PROFILE {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_PROFILE").field("ProfileId", &self.ProfileId).field("Index", &self.Index).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROITYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROITYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_EXTENDEDPROP_ROITYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.ControlId == other.ControlId && self.MaxNumberOfROIs == other.MaxNumberOfROIs && self.Capability == other.Capability
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPS").field("ControlId", &self.ControlId).field("MaxNumberOfROIs", &self.MaxNumberOfROIs).field("Capability", &self.Capability).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ConfigCapCount == other.ConfigCapCount && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_CONFIGCAPSHEADER").field("Size", &self.Size).field("ConfigCapCount", &self.ConfigCapCount).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    fn eq(&self, other: &Self) -> bool {
        self.ROIInfo == other.ROIInfo && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_EXPOSURE").field("ROIInfo", &self.ROIInfo).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    fn eq(&self, other: &Self) -> bool {
        self.ROIInfo == other.ROIInfo && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_FOCUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_FOCUS").field("ROIInfo", &self.ROIInfo).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Region == other.Region && self.Flags == other.Flags && self.Weight == other.Weight && self.RegionOfInterestType == other.RegionOfInterestType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_INFO").field("Region", &self.Region).field("Flags", &self.Flags).field("Weight", &self.Weight).field("RegionOfInterestType", &self.RegionOfInterestType).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.ControlId == other.ControlId && self.ROICount == other.ROICount && self.Result == other.Result && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROL").field("ControlId", &self.ControlId).field("ROICount", &self.ROICount).field("Result", &self.Result).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ControlCount == other.ControlCount && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_ISPCONTROLHEADER").field("Size", &self.Size).field("ControlCount", &self.ControlCount).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    fn eq(&self, other: &Self) -> bool {
        self.ROIInfo == other.ROIInfo && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_EXTENDEDPROP_ROI_WHITEBALANCE").field("ROIInfo", &self.ROIInfo).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_VIDEOPROCSETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_WBPRESET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_WBPRESET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_EXTENDEDPROP_WBPRESET").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_EXTENDEDPROP_WHITEBALANCE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    fn eq(&self, other: &Self) -> bool {
        self.PhotoResWidth == other.PhotoResWidth && self.PhotoResHeight == other.PhotoResHeight && self.PreviewFPSNum == other.PreviewFPSNum && self.PreviewFPSDenom == other.PreviewFPSDenom && self.CaptureFPSNum == other.CaptureFPSNum && self.CaptureFPSDenom == other.CaptureFPSDenom
    }
}
impl ::core::cmp::Eq for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {}
impl ::core::fmt::Debug for KSCAMERA_MAXVIDEOFPS_FORPHOTORES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_MAXVIDEOFPS_FORPHOTORES").field("PhotoResWidth", &self.PhotoResWidth).field("PhotoResHeight", &self.PhotoResHeight).field("PreviewFPSNum", &self.PreviewFPSNum).field("PreviewFPSDenom", &self.PreviewFPSDenom).field("CaptureFPSNum", &self.CaptureFPSNum).field("CaptureFPSDenom", &self.CaptureFPSDenom).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MaskCoverageBoundingBox == other.MaskCoverageBoundingBox && self.MaskResolution == other.MaskResolution && self.ForegroundBoundingBox == other.ForegroundBoundingBox && self.MaskData == other.MaskData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK").field("Header", &self.Header).field("MaskCoverageBoundingBox", &self.MaskCoverageBoundingBox).field("MaskResolution", &self.MaskResolution).field("ForegroundBoundingBox", &self.ForegroundBoundingBox).field("MaskData", &self.MaskData).finish()
    }
}
impl ::core::default::Default for KSCAMERA_METADATA_CAPTURESTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_CAPTURESTATS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.Reserved == other.Reserved && self.ExposureTime == other.ExposureTime && self.ExposureCompensationFlags == other.ExposureCompensationFlags && self.ExposureCompensationValue == other.ExposureCompensationValue && self.IsoSpeed == other.IsoSpeed && self.FocusState == other.FocusState && self.LensPosition == other.LensPosition && self.WhiteBalance == other.WhiteBalance && self.Flash == other.Flash && self.FlashPower == other.FlashPower && self.ZoomFactor == other.ZoomFactor && self.SceneMode == other.SceneMode && self.SensorFramerate == other.SensorFramerate
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_CAPTURESTATS {}
impl ::core::fmt::Debug for KSCAMERA_METADATA_CAPTURESTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_METADATA_CAPTURESTATS")
            .field("Header", &self.Header)
            .field("Flags", &self.Flags)
            .field("Reserved", &self.Reserved)
            .field("ExposureTime", &self.ExposureTime)
            .field("ExposureCompensationFlags", &self.ExposureCompensationFlags)
            .field("ExposureCompensationValue", &self.ExposureCompensationValue)
            .field("IsoSpeed", &self.IsoSpeed)
            .field("FocusState", &self.FocusState)
            .field("LensPosition", &self.LensPosition)
            .field("WhiteBalance", &self.WhiteBalance)
            .field("Flash", &self.Flash)
            .field("FlashPower", &self.FlashPower)
            .field("ZoomFactor", &self.ZoomFactor)
            .field("SceneMode", &self.SceneMode)
            .field("SensorFramerate", &self.SensorFramerate)
            .finish()
    }
}
impl ::core::default::Default for KSCAMERA_METADATA_DIGITALWINDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_DIGITALWINDOW {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Window == other.Window
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_DIGITALWINDOW {}
impl ::core::fmt::Debug for KSCAMERA_METADATA_DIGITALWINDOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_METADATA_DIGITALWINDOW").field("Header", &self.Header).field("Window", &self.Window).finish()
    }
}
impl ::core::default::Default for KSCAMERA_METADATA_FRAMEILLUMINATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_FRAMEILLUMINATION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_FRAMEILLUMINATION {}
impl ::core::fmt::Debug for KSCAMERA_METADATA_FRAMEILLUMINATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_METADATA_FRAMEILLUMINATION").field("Header", &self.Header).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSCAMERA_METADATA_ITEMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_ITEMHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.MetadataId == other.MetadataId && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_ITEMHEADER {}
impl ::core::fmt::Debug for KSCAMERA_METADATA_ITEMHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_METADATA_ITEMHEADER").field("MetadataId", &self.MetadataId).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PhotoConfirmationIndex == other.PhotoConfirmationIndex && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_METADATA_PHOTOCONFIRMATION {}
impl ::core::fmt::Debug for KSCAMERA_METADATA_PHOTOCONFIRMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_METADATA_PHOTOCONFIRMATION").field("Header", &self.Header).field("PhotoConfirmationIndex", &self.PhotoConfirmationIndex).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSCAMERA_MetadataId {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSCAMERA_MetadataId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_MetadataId").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ItemCount == other.ItemCount && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_CAP_HEADER {}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_CAP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PERFRAMESETTING_CAP_HEADER").field("Size", &self.Size).field("ItemCount", &self.ItemCount).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Type == other.Type && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PERFRAMESETTING_CAP_ITEM_HEADER").field("Size", &self.Size).field("Type", &self.Type).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Reserved == other.Reserved && self.Id == other.Id
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PERFRAMESETTING_CUSTOM_ITEM").field("Size", &self.Size).field("Reserved", &self.Reserved).field("Id", &self.Id).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Id == other.Id && self.ItemCount == other.ItemCount && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_FRAME_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PERFRAMESETTING_FRAME_HEADER").field("Size", &self.Size).field("Id", &self.Id).field("ItemCount", &self.ItemCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.FrameCount == other.FrameCount && self.Id == other.Id && self.Flags == other.Flags && self.LoopCount == other.LoopCount && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_HEADER {}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PERFRAMESETTING_HEADER").field("Size", &self.Size).field("FrameCount", &self.FrameCount).field("Id", &self.Id).field("Flags", &self.Flags).field("LoopCount", &self.LoopCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Type == other.Type && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_ITEM_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PERFRAMESETTING_ITEM_HEADER").field("Size", &self.Size).field("Type", &self.Type).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PERFRAMESETTING_ITEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSCAMERA_PERFRAMESETTING_ITEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSCAMERA_PERFRAMESETTING_ITEM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PROFILE_CONCURRENCYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_CONCURRENCYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ReferenceGuid == other.ReferenceGuid && self.Reserved == other.Reserved && self.ProfileCount == other.ProfileCount && self.Profiles == other.Profiles
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_CONCURRENCYINFO {}
impl ::core::fmt::Debug for KSCAMERA_PROFILE_CONCURRENCYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PROFILE_CONCURRENCYINFO").field("ReferenceGuid", &self.ReferenceGuid).field("Reserved", &self.Reserved).field("ProfileCount", &self.ProfileCount).field("Profiles", &self.Profiles).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProfileId == other.ProfileId && self.Index == other.Index && self.PinCount == other.PinCount && self.Pins == other.Pins
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_INFO {}
impl ::core::fmt::Debug for KSCAMERA_PROFILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PROFILE_INFO").field("ProfileId", &self.ProfileId).field("Index", &self.Index).field("PinCount", &self.PinCount).field("Pins", &self.Pins).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PROFILE_MEDIAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_MEDIAINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Resolution == other.Resolution && self.MaxFrameRate == other.MaxFrameRate && self.Flags == other.Flags && self.Data0 == other.Data0 && self.Data1 == other.Data1 && self.Data2 == other.Data2 && self.Data3 == other.Data3
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_MEDIAINFO {}
impl ::core::fmt::Debug for KSCAMERA_PROFILE_MEDIAINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PROFILE_MEDIAINFO").field("Resolution", &self.Resolution).field("MaxFrameRate", &self.MaxFrameRate).field("Flags", &self.Flags).field("Data0", &self.Data0).field("Data1", &self.Data1).field("Data2", &self.Data2).field("Data3", &self.Data3).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PROFILE_MEDIAINFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_MEDIAINFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_MEDIAINFO_0 {}
impl ::core::fmt::Debug for KSCAMERA_PROFILE_MEDIAINFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PROFILE_MEDIAINFO_0").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PROFILE_MEDIAINFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCAMERA_PROFILE_MEDIAINFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for KSCAMERA_PROFILE_MEDIAINFO_1 {}
impl ::core::fmt::Debug for KSCAMERA_PROFILE_MEDIAINFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCAMERA_PROFILE_MEDIAINFO_1").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::core::default::Default for KSCAMERA_PROFILE_PININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSCLOCK_CREATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCLOCK_CREATE {
    fn eq(&self, other: &Self) -> bool {
        self.CreateFlags == other.CreateFlags
    }
}
impl ::core::cmp::Eq for KSCLOCK_CREATE {}
impl ::core::fmt::Debug for KSCLOCK_CREATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCLOCK_CREATE").field("CreateFlags", &self.CreateFlags).finish()
    }
}
impl ::core::default::Default for KSCOMPONENTID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCOMPONENTID {
    fn eq(&self, other: &Self) -> bool {
        self.Manufacturer == other.Manufacturer && self.Product == other.Product && self.Component == other.Component && self.Name == other.Name && self.Version == other.Version && self.Revision == other.Revision
    }
}
impl ::core::cmp::Eq for KSCOMPONENTID {}
impl ::core::fmt::Debug for KSCOMPONENTID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCOMPONENTID").field("Manufacturer", &self.Manufacturer).field("Product", &self.Product).field("Component", &self.Component).field("Name", &self.Name).field("Version", &self.Version).field("Revision", &self.Revision).finish()
    }
}
impl ::core::default::Default for KSCORRELATED_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSCORRELATED_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.Time == other.Time && self.SystemTime == other.SystemTime
    }
}
impl ::core::cmp::Eq for KSCORRELATED_TIME {}
impl ::core::fmt::Debug for KSCORRELATED_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSCORRELATED_TIME").field("Time", &self.Time).field("SystemTime", &self.SystemTime).finish()
    }
}
impl ::core::default::Default for KSDATAFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSDATARANGE_AUDIO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSDATARANGE_MUSIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSDEGRADE_STANDARD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSDEGRADE_STANDARD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSDEGRADE_STANDARD").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSDEVICE_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSDEVICE_THERMAL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSDEVICE_THERMAL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSDEVICE_THERMAL_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSDISPLAYCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDISPLAYCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.PelsWidth == other.PelsWidth && self.PelsHeight == other.PelsHeight && self.BitsPerPel == other.BitsPerPel && self.DeviceID == other.DeviceID
    }
}
impl ::core::cmp::Eq for KSDISPLAYCHANGE {}
impl ::core::fmt::Debug for KSDISPLAYCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDISPLAYCHANGE").field("PelsWidth", &self.PelsWidth).field("PelsHeight", &self.PelsHeight).field("BitsPerPel", &self.BitsPerPel).field("DeviceID", &self.DeviceID).finish()
    }
}
impl ::core::default::Default for KSDS3D_BUFFER_ALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSDS3D_BUFFER_CONE_ANGLES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDS3D_BUFFER_CONE_ANGLES {
    fn eq(&self, other: &Self) -> bool {
        self.InsideConeAngle == other.InsideConeAngle && self.OutsideConeAngle == other.OutsideConeAngle
    }
}
impl ::core::cmp::Eq for KSDS3D_BUFFER_CONE_ANGLES {}
impl ::core::fmt::Debug for KSDS3D_BUFFER_CONE_ANGLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDS3D_BUFFER_CONE_ANGLES").field("InsideConeAngle", &self.InsideConeAngle).field("OutsideConeAngle", &self.OutsideConeAngle).finish()
    }
}
impl ::core::default::Default for KSDS3D_HRTF_COEFF_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSDS3D_HRTF_COEFF_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSDS3D_HRTF_COEFF_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    fn eq(&self, other: &Self) -> bool {
        self.FilterMethod == other.FilterMethod && self.CoeffFormat == other.CoeffFormat && self.Version == other.Version && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSDS3D_HRTF_FILTER_FORMAT_MSG {}
impl ::core::fmt::Debug for KSDS3D_HRTF_FILTER_FORMAT_MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDS3D_HRTF_FILTER_FORMAT_MSG").field("FilterMethod", &self.FilterMethod).field("CoeffFormat", &self.CoeffFormat).field("Version", &self.Version).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSDS3D_HRTF_FILTER_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSDS3D_HRTF_FILTER_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSDS3D_HRTF_FILTER_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSDS3D_HRTF_FILTER_QUALITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSDS3D_HRTF_FILTER_QUALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSDS3D_HRTF_FILTER_QUALITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSDS3D_HRTF_FILTER_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSDS3D_HRTF_FILTER_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSDS3D_HRTF_FILTER_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSDS3D_HRTF_INIT_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDS3D_HRTF_INIT_MSG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Quality == other.Quality && self.SampleRate == other.SampleRate && self.MaxFilterSize == other.MaxFilterSize && self.FilterTransientMuteLength == other.FilterTransientMuteLength && self.FilterOverlapBufferLength == other.FilterOverlapBufferLength && self.OutputOverlapBufferLength == other.OutputOverlapBufferLength && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSDS3D_HRTF_INIT_MSG {}
impl ::core::fmt::Debug for KSDS3D_HRTF_INIT_MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDS3D_HRTF_INIT_MSG").field("Size", &self.Size).field("Quality", &self.Quality).field("SampleRate", &self.SampleRate).field("MaxFilterSize", &self.MaxFilterSize).field("FilterTransientMuteLength", &self.FilterTransientMuteLength).field("FilterOverlapBufferLength", &self.FilterOverlapBufferLength).field("OutputOverlapBufferLength", &self.OutputOverlapBufferLength).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSDS3D_HRTF_PARAMS_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSDS3D_HRTF_PARAMS_MSG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Enabled == other.Enabled && self.SwapChannels == other.SwapChannels && self.ZeroAzimuth == other.ZeroAzimuth && self.CrossFadeOutput == other.CrossFadeOutput && self.FilterSize == other.FilterSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSDS3D_HRTF_PARAMS_MSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSDS3D_HRTF_PARAMS_MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDS3D_HRTF_PARAMS_MSG").field("Size", &self.Size).field("Enabled", &self.Enabled).field("SwapChannels", &self.SwapChannels).field("ZeroAzimuth", &self.ZeroAzimuth).field("CrossFadeOutput", &self.CrossFadeOutput).field("FilterSize", &self.FilterSize).finish()
    }
}
impl ::core::default::Default for KSDS3D_ITD_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDS3D_ITD_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Channel == other.Channel && self.VolSmoothScale == other.VolSmoothScale && self.TotalDryAttenuation == other.TotalDryAttenuation && self.TotalWetAttenuation == other.TotalWetAttenuation && self.SmoothFrequency == other.SmoothFrequency && self.Delay == other.Delay
    }
}
impl ::core::cmp::Eq for KSDS3D_ITD_PARAMS {}
impl ::core::fmt::Debug for KSDS3D_ITD_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDS3D_ITD_PARAMS").field("Channel", &self.Channel).field("VolSmoothScale", &self.VolSmoothScale).field("TotalDryAttenuation", &self.TotalDryAttenuation).field("TotalWetAttenuation", &self.TotalWetAttenuation).field("SmoothFrequency", &self.SmoothFrequency).field("Delay", &self.Delay).finish()
    }
}
impl ::core::default::Default for KSDS3D_ITD_PARAMS_MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSDS3D_ITD_PARAMS_MSG {
    fn eq(&self, other: &Self) -> bool {
        self.Enabled == other.Enabled && self.LeftParams == other.LeftParams && self.RightParams == other.RightParams && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSDS3D_ITD_PARAMS_MSG {}
impl ::core::fmt::Debug for KSDS3D_ITD_PARAMS_MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSDS3D_ITD_PARAMS_MSG").field("Enabled", &self.Enabled).field("LeftParams", &self.LeftParams).field("RightParams", &self.RightParams).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSDS3D_LISTENER_ALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSDS3D_LISTENER_ORIENTATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSERROR {
    fn eq(&self, other: &Self) -> bool {
        self.Context == other.Context && self.Status == other.Status
    }
}
impl ::core::cmp::Eq for KSERROR {}
impl ::core::fmt::Debug for KSERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSERROR").field("Context", &self.Context).field("Status", &self.Status).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSEVENT_AUDIO_CONTROL_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_AUDIO_CONTROL_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_AUDIO_CONTROL_CHANGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_CAMERACONTROL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_CAMERACONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_CAMERACONTROL").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_CAMERAEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_CAMERAEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_CAMERAEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_CLOCK_POSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_CLOCK_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_CLOCK_POSITION").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_CONNECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_CONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_CONNECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_CROSSBAR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_CROSSBAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_CROSSBAR").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_DEVCMD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_DEVCMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_DEVCMD").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_DEVICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_DEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_DEVICE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_DYNAMICFORMATCHANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_DYNAMICFORMATCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_DYNAMICFORMATCHANGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_LOOPEDSTREAMING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_LOOPEDSTREAMING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_LOOPEDSTREAMING").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_PINCAPS_CHANGENOTIFICATIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_PINCAPS_CHANGENOTIFICATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_PINCAPS_CHANGENOTIFICATIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_SOUNDDETECTOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_SOUNDDETECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_SOUNDDETECTOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_STREAMALLOCATOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_STREAMALLOCATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_STREAMALLOCATOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_TELEPHONY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_TELEPHONY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_TELEPHONY").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENT_TIME_INTERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENT_TIME_MARK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSEVENT_TUNER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_TUNER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_TUNER").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSEVENT_TUNER_INITIATE_SCAN_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSEVENT_TVAUDIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_TVAUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_TVAUDIO").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_VIDCAPTOSTI {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_VIDCAPTOSTI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_VIDCAPTOSTI").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_VIDEODECODER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_VIDEODECODER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_VIDEODECODER").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_VOLUMELIMIT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_VOLUMELIMIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_VOLUMELIMIT").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_VPNOTIFY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_VPNOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_VPNOTIFY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSEVENT_VPVBINOTIFY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSEVENT_VPVBINOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSEVENT_VPVBINOTIFY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSE_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSE_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSFRAMETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSFRAMETIME {
    fn eq(&self, other: &Self) -> bool {
        self.Duration == other.Duration && self.FrameFlags == other.FrameFlags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSFRAMETIME {}
impl ::core::fmt::Debug for KSFRAMETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSFRAMETIME").field("Duration", &self.Duration).field("FrameFlags", &self.FrameFlags).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSGOP_USERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSGOP_USERDATA {
    fn eq(&self, other: &Self) -> bool {
        self.sc == other.sc && self.reserved1 == other.reserved1 && self.cFields == other.cFields && self.l21Data == other.l21Data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSGOP_USERDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSGOP_USERDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSGOP_USERDATA").field("sc", &self.sc).field("reserved1", &self.reserved1).field("cFields", &self.cFields).field("l21Data", &self.l21Data).finish()
    }
}
impl ::core::default::Default for KSIDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSINTERFACE_FILEIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSINTERFACE_FILEIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSINTERFACE_FILEIO").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSINTERFACE_MEDIA {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSINTERFACE_MEDIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSINTERFACE_MEDIA").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSINTERFACE_STANDARD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSINTERFACE_STANDARD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSINTERFACE_STANDARD").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSINTERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSINTERVAL {
    fn eq(&self, other: &Self) -> bool {
        self.TimeBase == other.TimeBase && self.Interval == other.Interval
    }
}
impl ::core::cmp::Eq for KSINTERVAL {}
impl ::core::fmt::Debug for KSINTERVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSINTERVAL").field("TimeBase", &self.TimeBase).field("Interval", &self.Interval).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSJACK_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSJACK_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.ChannelMapping == other.ChannelMapping && self.Color == other.Color && self.ConnectionType == other.ConnectionType && self.GeoLocation == other.GeoLocation && self.GenLocation == other.GenLocation && self.PortConnection == other.PortConnection && self.IsConnected == other.IsConnected
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSJACK_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSJACK_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSJACK_DESCRIPTION").field("ChannelMapping", &self.ChannelMapping).field("Color", &self.Color).field("ConnectionType", &self.ConnectionType).field("GeoLocation", &self.GeoLocation).field("GenLocation", &self.GenLocation).field("PortConnection", &self.PortConnection).field("IsConnected", &self.IsConnected).finish()
    }
}
impl ::core::default::Default for KSJACK_DESCRIPTION2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSJACK_DESCRIPTION2 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceStateInfo == other.DeviceStateInfo && self.JackCapabilities == other.JackCapabilities
    }
}
impl ::core::cmp::Eq for KSJACK_DESCRIPTION2 {}
impl ::core::fmt::Debug for KSJACK_DESCRIPTION2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSJACK_DESCRIPTION2").field("DeviceStateInfo", &self.DeviceStateInfo).field("JackCapabilities", &self.JackCapabilities).finish()
    }
}
impl ::core::default::Default for KSJACK_SINK_CONNECTIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSJACK_SINK_CONNECTIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSJACK_SINK_CONNECTIONTYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSJACK_SINK_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSJACK_SINK_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ConnType == other.ConnType && self.ManufacturerId == other.ManufacturerId && self.ProductId == other.ProductId && self.AudioLatency == other.AudioLatency && self.HDCPCapable == other.HDCPCapable && self.AICapable == other.AICapable && self.SinkDescriptionLength == other.SinkDescriptionLength && self.SinkDescription == other.SinkDescription && self.PortId == other.PortId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSJACK_SINK_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSJACK_SINK_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSJACK_SINK_INFORMATION").field("ConnType", &self.ConnType).field("ManufacturerId", &self.ManufacturerId).field("ProductId", &self.ProductId).field("AudioLatency", &self.AudioLatency).field("HDCPCapable", &self.HDCPCapable).field("AICapable", &self.AICapable).field("SinkDescriptionLength", &self.SinkDescriptionLength).field("SinkDescription", &self.SinkDescription).field("PortId", &self.PortId).finish()
    }
}
impl ::core::default::Default for KSMETHOD_STREAMALLOCATOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSMETHOD_STREAMALLOCATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSMETHOD_STREAMALLOCATOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSMETHOD_STREAMIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSMETHOD_STREAMIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSMETHOD_STREAMIO").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSMETHOD_WAVETABLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSMETHOD_WAVETABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSMETHOD_WAVETABLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSMICARRAY_MICARRAYTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSMICARRAY_MICARRAYTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSMICARRAY_MICARRAYTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSMICARRAY_MICTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSMICARRAY_MICTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSMICARRAY_MICTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSMPEGVID_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSMPEGVID_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.StartX == other.StartX && self.StartY == other.StartY && self.EndX == other.EndX && self.EndY == other.EndY
    }
}
impl ::core::cmp::Eq for KSMPEGVID_RECT {}
impl ::core::fmt::Debug for KSMPEGVID_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSMPEGVID_RECT").field("StartX", &self.StartX).field("StartY", &self.StartY).field("EndX", &self.EndX).field("EndY", &self.EndY).finish()
    }
}
impl ::core::default::Default for KSMULTIPLE_DATA_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSMULTIPLE_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSMULTIPLE_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for KSMULTIPLE_ITEM {}
impl ::core::fmt::Debug for KSMULTIPLE_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSMULTIPLE_ITEM").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
impl ::core::default::Default for KSMUSICFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSMUSICFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.TimeDeltaMs == other.TimeDeltaMs && self.ByteCount == other.ByteCount
    }
}
impl ::core::cmp::Eq for KSMUSICFORMAT {}
impl ::core::fmt::Debug for KSMUSICFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSMUSICFORMAT").field("TimeDeltaMs", &self.TimeDeltaMs).field("ByteCount", &self.ByteCount).finish()
    }
}
impl ::core::default::Default for KSM_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSNODEPROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_3D_LISTENER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_DEV_SPECIFIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for KSNODEPROPERTY_AUDIO_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSNODE_CREATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSNODE_CREATE {
    fn eq(&self, other: &Self) -> bool {
        self.CreateFlags == other.CreateFlags && self.Node == other.Node
    }
}
impl ::core::cmp::Eq for KSNODE_CREATE {}
impl ::core::fmt::Debug for KSNODE_CREATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSNODE_CREATE").field("CreateFlags", &self.CreateFlags).field("Node", &self.Node).finish()
    }
}
impl ::core::default::Default for KSPIN_CINSTANCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPIN_CINSTANCES {
    fn eq(&self, other: &Self) -> bool {
        self.PossibleCount == other.PossibleCount && self.CurrentCount == other.CurrentCount
    }
}
impl ::core::cmp::Eq for KSPIN_CINSTANCES {}
impl ::core::fmt::Debug for KSPIN_CINSTANCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPIN_CINSTANCES").field("PossibleCount", &self.PossibleCount).field("CurrentCount", &self.CurrentCount).finish()
    }
}
impl ::core::default::Default for KSPIN_COMMUNICATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPIN_COMMUNICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPIN_COMMUNICATION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPIN_CONNECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPIN_DATAFLOW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPIN_DATAFLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPIN_DATAFLOW").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPIN_MDL_CACHING_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPIN_MDL_CACHING_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPIN_MDL_CACHING_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPIN_MDL_CACHING_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPIN_MDL_CACHING_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.Event == other.Event && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for KSPIN_MDL_CACHING_NOTIFICATION {}
impl ::core::fmt::Debug for KSPIN_MDL_CACHING_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPIN_MDL_CACHING_NOTIFICATION").field("Event", &self.Event).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for KSPIN_MDL_CACHING_NOTIFICATION32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPIN_MDL_CACHING_NOTIFICATION32 {
    fn eq(&self, other: &Self) -> bool {
        self.Event == other.Event && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for KSPIN_MDL_CACHING_NOTIFICATION32 {}
impl ::core::fmt::Debug for KSPIN_MDL_CACHING_NOTIFICATION32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPIN_MDL_CACHING_NOTIFICATION32").field("Event", &self.Event).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for KSPIN_PHYSICALCONNECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPIN_PHYSICALCONNECTION {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Pin == other.Pin && self.SymbolicLinkName == other.SymbolicLinkName
    }
}
impl ::core::cmp::Eq for KSPIN_PHYSICALCONNECTION {}
impl ::core::fmt::Debug for KSPIN_PHYSICALCONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPIN_PHYSICALCONNECTION").field("Size", &self.Size).field("Pin", &self.Pin).field("SymbolicLinkName", &self.SymbolicLinkName).finish()
    }
}
impl ::core::default::Default for KSPPROPERTY_ALLOCATOR_MDLCACHING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPPROPERTY_ALLOCATOR_MDLCACHING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPPROPERTY_ALLOCATOR_MDLCACHING").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPRIORITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPRIORITY {
    fn eq(&self, other: &Self) -> bool {
        self.PriorityClass == other.PriorityClass && self.PrioritySubClass == other.PrioritySubClass
    }
}
impl ::core::cmp::Eq for KSPRIORITY {}
impl ::core::fmt::Debug for KSPRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPRIORITY").field("PriorityClass", &self.PriorityClass).field("PrioritySubClass", &self.PrioritySubClass).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_AC3 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_AC3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AC3").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_ALLOCATOR_CONTROL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_ALLOCATOR_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_ALLOCATOR_CONTROL").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        self.InterleavedCapSupported == other.InterleavedCapSupported
    }
}
impl ::core::cmp::Eq for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {}
impl ::core::fmt::Debug for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_CAPS_S").field("InterleavedCapSupported", &self.InterleavedCapSupported).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    fn eq(&self, other: &Self) -> bool {
        self.InterleavedCapPossible == other.InterleavedCapPossible
    }
}
impl ::core::cmp::Eq for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {}
impl ::core::fmt::Debug for KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_ALLOCATOR_CONTROL_CAPTURE_INTERLEAVE_S").field("InterleavedCapPossible", &self.InterleavedCapPossible).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    fn eq(&self, other: &Self) -> bool {
        self.CX == other.CX && self.CY == other.CY
    }
}
impl ::core::cmp::Eq for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {}
impl ::core::fmt::Debug for KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_ALLOCATOR_CONTROL_SURFACE_SIZE_S").field("CX", &self.CX).field("CY", &self.CY).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_AUDDECOUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_AUDDECOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDDECOUT").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_AUDIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_AUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDIO").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_AUDIOENGINE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_AUDIOENGINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDIOENGINE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_AUDIOMODULE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_AUDIOMODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDIOMODULE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_AUDIOPOSTURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_AUDIOPOSTURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDIOPOSTURE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_AUDIORESOURCEMANAGEMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_AUDIORESOURCEMANAGEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDIORESOURCEMANAGEMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_AUDIOSIGNALPROCESSING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_AUDIOSIGNALPROCESSING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_AUDIOSIGNALPROCESSING").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_BIBLIOGRAPHIC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_BIBLIOGRAPHIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_BIBLIOGRAPHIC").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_BOUNDS_LONGLONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_BTAUDIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_BTAUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_BTAUDIO").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERACONTROL_EXTENDED_PROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_FLASH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_FLASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERACONTROL_FLASH").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_FLASH_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_FLASH_S {
    fn eq(&self, other: &Self) -> bool {
        self.Flash == other.Flash && self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_FLASH_S {}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_FLASH_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_CAMERACONTROL_FLASH_S").field("Flash", &self.Flash).field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_FOCAL_LENGTH_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.Reserved0 == other.Reserved0
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_CAMERACONTROL_IMAGE_PIN_CAPABILITY_S").field("Capabilities", &self.Capabilities).field("Reserved0", &self.Reserved0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_NODE_FOCAL_LENGTH_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_NODE_S2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERACONTROL_PERFRAMESETTING_PROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_REGION_OF_INTEREST_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_S2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_S_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    fn eq(&self, other: &Self) -> bool {
        self.VideoStabilizationMode == other.VideoStabilizationMode && self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_CAMERACONTROL_VIDEOSTABILIZATION_MODE_S").field("VideoStabilizationMode", &self.VideoStabilizationMode).field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERACONTROL_VIDEO_STABILIZATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CAMERA_PHOTOTRIGGERTIME_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_CLOCK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CLOCK").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_CONNECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CONNECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_COPYPROT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_COPYPROT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_COPYPROT").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_CROSSBAR_ACTIVE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_CROSSBAR_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_CROSSBAR_PININFO_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_CROSSBAR_ROUTE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_CYCLIC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_CYCLIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_CYCLIC").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_DIRECTSOUND3DBUFFER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_DIRECTSOUND3DBUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_DIRECTSOUND3DBUFFER").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_DIRECTSOUND3DLISTENER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_DIRECTSOUND3DLISTENER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_DIRECTSOUND3DLISTENER").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_DRMAUDIOSTREAM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_DRMAUDIOSTREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_DRMAUDIOSTREAM").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_DROPPEDFRAMES_CURRENT_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_DVDSUBPIC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_DVDSUBPIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_DVDSUBPIC").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_EXTDEVICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_EXTDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_EXTDEVICE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_EXTDEVICE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_EXTENSION_UNIT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_EXTENSION_UNIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_EXTENSION_UNIT").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_EXTXPORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_EXTXPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_EXTXPORT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_EXTXPORT_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_FMRX_CONTROL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_FMRX_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_FMRX_CONTROL").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_FMRX_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_FMRX_TOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_FMRX_TOPOLOGY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_GENERAL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_GENERAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_GENERAL").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_HRTF3D {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_HRTF3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_HRTF3D").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_INTERLEAVEDAUDIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_INTERLEAVEDAUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_INTERLEAVEDAUDIO").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_ITD3D {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_ITD3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_ITD3D").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_JACK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_JACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_JACK").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_MEDIAAVAILABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_MEDIAAVAILABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Earliest == other.Earliest && self.Latest == other.Latest
    }
}
impl ::core::cmp::Eq for KSPROPERTY_MEDIAAVAILABLE {}
impl ::core::fmt::Debug for KSPROPERTY_MEDIAAVAILABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_MEDIAAVAILABLE").field("Earliest", &self.Earliest).field("Latest", &self.Latest).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_MEDIASEEKING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_MEDIASEEKING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_MEDIASEEKING").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_MEMBERSHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_MEMBERSHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.MembersFlags == other.MembersFlags && self.MembersSize == other.MembersSize && self.MembersCount == other.MembersCount && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for KSPROPERTY_MEMBERSHEADER {}
impl ::core::fmt::Debug for KSPROPERTY_MEMBERSHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_MEMBERSHEADER").field("MembersFlags", &self.MembersFlags).field("MembersSize", &self.MembersSize).field("MembersCount", &self.MembersCount).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_MPEG2VID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_MPEG2VID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_MPEG2VID").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_MPEG4_MEDIATYPE_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.EventFilter == other.EventFilter
    }
}
impl ::core::cmp::Eq for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {}
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_NETWORKCAMERACONTROL_EVENT_INFO").field("Header", &self.Header).field("EventFilter", &self.EventFilter).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MetadataItems == other.MetadataItems && self.Size == other.Size && self.PTZStatus == other.PTZStatus && self.Events == other.Events && self.Analytics == other.Analytics && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_NETWORKCAMERACONTROL_METADATA_INFO").field("MetadataItems", &self.MetadataItems).field("Size", &self.Size).field("PTZStatus", &self.PTZStatus).field("Events", &self.Events).field("Analytics", &self.Analytics).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_NETWORKCAMERACONTROL_METADATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {}
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_HEADER").field("Size", &self.Size).field("Type", &self.Type).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_NETWORKCAMERACONTROL_NTPINFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_NETWORKCAMERACONTROL_PROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_OVERLAYUPDATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_OVERLAYUPDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_OVERLAYUPDATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_PIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_PIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_PIN").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_POSITIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_POSITIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Current == other.Current && self.Stop == other.Stop && self.CurrentFlags == other.CurrentFlags && self.StopFlags == other.StopFlags
    }
}
impl ::core::cmp::Eq for KSPROPERTY_POSITIONS {}
impl ::core::fmt::Debug for KSPROPERTY_POSITIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_POSITIONS").field("Current", &self.Current).field("Stop", &self.Stop).field("CurrentFlags", &self.CurrentFlags).field("StopFlags", &self.StopFlags).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_QUALITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_QUALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_QUALITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_RTAUDIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_RTAUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_RTAUDIO").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_SELECTOR_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_SELECTOR_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_SERIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_SERIALHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_SOUNDDETECTOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_SOUNDDETECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_SOUNDDETECTOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_SPHLI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_SPHLI {
    fn eq(&self, other: &Self) -> bool {
        self.HLISS == other.HLISS && self.Reserved == other.Reserved && self.StartPTM == other.StartPTM && self.EndPTM == other.EndPTM && self.StartX == other.StartX && self.StartY == other.StartY && self.StopX == other.StopX && self.StopY == other.StopY && self.ColCon == other.ColCon
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SPHLI {}
impl ::core::fmt::Debug for KSPROPERTY_SPHLI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_SPHLI").field("HLISS", &self.HLISS).field("Reserved", &self.Reserved).field("StartPTM", &self.StartPTM).field("EndPTM", &self.EndPTM).field("StartX", &self.StartX).field("StartY", &self.StartY).field("StopX", &self.StopX).field("StopY", &self.StopY).field("ColCon", &self.ColCon).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_SPPAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSPROPERTY_SPPAL {
    fn eq(&self, other: &Self) -> bool {
        self.sppal == other.sppal
    }
}
impl ::core::cmp::Eq for KSPROPERTY_SPPAL {}
impl ::core::fmt::Debug for KSPROPERTY_SPPAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSPROPERTY_SPPAL").field("sppal", &self.sppal).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_STEPPING_LONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_STEPPING_LONGLONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_STREAM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_STREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_STREAM").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_STREAMINTERFACE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_STREAMINTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_STREAMINTERFACE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_TELEPHONY_CONTROL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_TELEPHONY_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TELEPHONY_CONTROL").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_TELEPHONY_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_TELEPHONY_TOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TELEPHONY_TOPOLOGY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_TIMECODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_TIMECODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TIMECODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_TIMECODE_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TIMECODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_TOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TOPOLOGY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_TOPOLOGYNODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_TOPOLOGYNODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TOPOLOGYNODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_TUNER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TUNER").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER_FREQUENCY_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER_IF_MEDIUM_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER_INPUT_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_TUNER_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_TUNER_MODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER_MODE_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER_MODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER_NETWORKTYPE_SCAN_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_TUNER_SCAN_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER_SCAN_STATUS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_TUNER_STANDARD_MODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER_STANDARD_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TUNER_STATUS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TVAUDIO_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_TVAUDIO_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VBICAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VBICAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VBICAP").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VBICODECFILTERING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VBICODECFILTERING").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_CC_SUBSTREAMS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_NABTS_SUBSTREAMS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_SCANLINES_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_PIN_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_CC_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_PIN_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_COMMON_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_PIN_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VBICODECFILTERING_STATISTICS_NABTS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_CAMERACONTROL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_CAMERACONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_CAMERACONTROL").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_CROSSBAR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_CROSSBAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_CROSSBAR").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_DROPPEDFRAMES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_DROPPEDFRAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_DROPPEDFRAMES").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_SELECTOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_SELECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_SELECTOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_TVAUDIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_TVAUDIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_TVAUDIO").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_VIDEOCOMPRESSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_VIDEOCOMPRESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_VIDEOCOMPRESSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_VIDEOCONTROL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_VIDEOCONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_VIDEOCONTROL").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_VIDEODECODER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_VIDEODECODER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_VIDEODECODER").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_VIDEOENCODER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_VIDEOENCODER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_VIDEOENCODER").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VIDCAP_VIDEOPROCAMP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VIDCAP_VIDEOPROCAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDCAP_VIDEOPROCAMP").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEOCOMPRESSION_GETINFO_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEOCOMPRESSION_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEOCOMPRESSION_S1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_VIDEOCONTROL_ACTUAL_FRAME_RATE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEOCONTROL_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSPROPERTY_VIDEOCONTROL_FRAME_RATES_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEOCONTROL_MODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEODECODER_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEODECODER_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEODECODER_STATUS2_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEODECODER_STATUS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEOENCODER_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEOPROCAMP_NODE_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEOPROCAMP_NODE_S2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEOPROCAMP_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDEOPROCAMP_S2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSPROPERTY_VIDMEM_TRANSPORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VIDMEM_TRANSPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VIDMEM_TRANSPORT").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_VPCONFIG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_VPCONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_VPCONFIG").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSPROPERTY_WAVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSPROPERTY_WAVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSPROPERTY_WAVE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSP_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSP_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSP_TIMEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSQUALITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSQUALITY {
    fn eq(&self, other: &Self) -> bool {
        self.Context == other.Context && self.Proportion == other.Proportion && self.DeltaTime == other.DeltaTime
    }
}
impl ::core::cmp::Eq for KSQUALITY {}
impl ::core::fmt::Debug for KSQUALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSQUALITY").field("Context", &self.Context).field("Proportion", &self.Proportion).field("DeltaTime", &self.DeltaTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSQUALITY_MANAGER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSQUALITY_MANAGER {
    fn eq(&self, other: &Self) -> bool {
        self.QualityManager == other.QualityManager && self.Context == other.Context
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSQUALITY_MANAGER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSQUALITY_MANAGER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSQUALITY_MANAGER").field("QualityManager", &self.QualityManager).field("Context", &self.Context).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSQUERYBUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSRATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSRATE_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRELATIVEEVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSRESET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSRESET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSRESET").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSRESOLUTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRESOLUTION {
    fn eq(&self, other: &Self) -> bool {
        self.Granularity == other.Granularity && self.Error == other.Error
    }
}
impl ::core::cmp::Eq for KSRESOLUTION {}
impl ::core::fmt::Debug for KSRESOLUTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRESOLUTION").field("Granularity", &self.Granularity).field("Error", &self.Error).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRTAUDIO_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.BufferAddress == other.BufferAddress && self.ActualBufferSize == other.ActualBufferSize && self.CallMemoryBarrier == other.CallMemoryBarrier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSRTAUDIO_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_BUFFER").field("BufferAddress", &self.BufferAddress).field("ActualBufferSize", &self.ActualBufferSize).field("CallMemoryBarrier", &self.CallMemoryBarrier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRTAUDIO_BUFFER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRTAUDIO_BUFFER32 {
    fn eq(&self, other: &Self) -> bool {
        self.BufferAddress == other.BufferAddress && self.ActualBufferSize == other.ActualBufferSize && self.CallMemoryBarrier == other.CallMemoryBarrier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRTAUDIO_BUFFER32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSRTAUDIO_BUFFER32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_BUFFER32").field("BufferAddress", &self.BufferAddress).field("ActualBufferSize", &self.ActualBufferSize).field("CallMemoryBarrier", &self.CallMemoryBarrier).finish()
    }
}
impl ::core::default::Default for KSRTAUDIO_BUFFER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSRTAUDIO_BUFFER_PROPERTY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSRTAUDIO_BUFFER_PROPERTY_WITH_NOTIFICATION32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRTAUDIO_GETREADPACKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KSRTAUDIO_GETREADPACKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PacketNumber == other.PacketNumber && self.Flags == other.Flags && self.PerformanceCounterValue == other.PerformanceCounterValue && self.MoreData == other.MoreData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KSRTAUDIO_GETREADPACKET_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KSRTAUDIO_GETREADPACKET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_GETREADPACKET_INFO").field("PacketNumber", &self.PacketNumber).field("Flags", &self.Flags).field("PerformanceCounterValue", &self.PerformanceCounterValue).field("MoreData", &self.MoreData).finish()
    }
}
impl ::core::default::Default for KSRTAUDIO_HWLATENCY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWLATENCY {
    fn eq(&self, other: &Self) -> bool {
        self.FifoSize == other.FifoSize && self.ChipsetDelay == other.ChipsetDelay && self.CodecDelay == other.CodecDelay
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWLATENCY {}
impl ::core::fmt::Debug for KSRTAUDIO_HWLATENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_HWLATENCY").field("FifoSize", &self.FifoSize).field("ChipsetDelay", &self.ChipsetDelay).field("CodecDelay", &self.CodecDelay).finish()
    }
}
impl ::core::default::Default for KSRTAUDIO_HWREGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWREGISTER {
    fn eq(&self, other: &Self) -> bool {
        self.Register == other.Register && self.Width == other.Width && self.Numerator == other.Numerator && self.Denominator == other.Denominator && self.Accuracy == other.Accuracy
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWREGISTER {}
impl ::core::fmt::Debug for KSRTAUDIO_HWREGISTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_HWREGISTER").field("Register", &self.Register).field("Width", &self.Width).field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).field("Accuracy", &self.Accuracy).finish()
    }
}
impl ::core::default::Default for KSRTAUDIO_HWREGISTER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_HWREGISTER32 {
    fn eq(&self, other: &Self) -> bool {
        self.Register == other.Register && self.Width == other.Width && self.Numerator == other.Numerator && self.Denominator == other.Denominator && self.Accuracy == other.Accuracy
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_HWREGISTER32 {}
impl ::core::fmt::Debug for KSRTAUDIO_HWREGISTER32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_HWREGISTER32").field("Register", &self.Register).field("Width", &self.Width).field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).field("Accuracy", &self.Accuracy).finish()
    }
}
impl ::core::default::Default for KSRTAUDIO_HWREGISTER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSRTAUDIO_HWREGISTER_PROPERTY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSRTAUDIO_NOTIFICATION_EVENT_PROPERTY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSRTAUDIO_PACKETVREGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_PACKETVREGISTER {
    fn eq(&self, other: &Self) -> bool {
        self.CompletedPacketCount == other.CompletedPacketCount && self.CompletedPacketQPC == other.CompletedPacketQPC && self.CompletedPacketHash == other.CompletedPacketHash
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_PACKETVREGISTER {}
impl ::core::fmt::Debug for KSRTAUDIO_PACKETVREGISTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_PACKETVREGISTER").field("CompletedPacketCount", &self.CompletedPacketCount).field("CompletedPacketQPC", &self.CompletedPacketQPC).field("CompletedPacketHash", &self.CompletedPacketHash).finish()
    }
}
impl ::core::default::Default for KSRTAUDIO_PACKETVREGISTER_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSRTAUDIO_SETWRITEPACKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSRTAUDIO_SETWRITEPACKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PacketNumber == other.PacketNumber && self.Flags == other.Flags && self.EosPacketLength == other.EosPacketLength
    }
}
impl ::core::cmp::Eq for KSRTAUDIO_SETWRITEPACKET_INFO {}
impl ::core::fmt::Debug for KSRTAUDIO_SETWRITEPACKET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSRTAUDIO_SETWRITEPACKET_INFO").field("PacketNumber", &self.PacketNumber).field("Flags", &self.Flags).field("EosPacketLength", &self.EosPacketLength).finish()
    }
}
impl ::core::default::Default for KSSOUNDDETECTORPROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSSTREAMALLOCATOR_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSSTREAMALLOCATOR_STATUS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for KSSTREAM_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for KSSTREAM_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.TypeSpecificFlags == other.TypeSpecificFlags && self.PresentationTime == other.PresentationTime && self.Duration == other.Duration && self.FrameExtent == other.FrameExtent && self.DataUsed == other.DataUsed && self.Data == other.Data && self.OptionsFlags == other.OptionsFlags && self.Reserved == other.Reserved
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for KSSTREAM_HEADER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for KSSTREAM_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSSTREAM_HEADER").field("Size", &self.Size).field("TypeSpecificFlags", &self.TypeSpecificFlags).field("PresentationTime", &self.PresentationTime).field("Duration", &self.Duration).field("FrameExtent", &self.FrameExtent).field("DataUsed", &self.DataUsed).field("Data", &self.Data).field("OptionsFlags", &self.OptionsFlags).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for KSSTREAM_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for KSSTREAM_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.TypeSpecificFlags == other.TypeSpecificFlags && self.PresentationTime == other.PresentationTime && self.Duration == other.Duration && self.FrameExtent == other.FrameExtent && self.DataUsed == other.DataUsed && self.Data == other.Data && self.OptionsFlags == other.OptionsFlags
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for KSSTREAM_HEADER {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for KSSTREAM_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSSTREAM_HEADER").field("Size", &self.Size).field("TypeSpecificFlags", &self.TypeSpecificFlags).field("PresentationTime", &self.PresentationTime).field("Duration", &self.Duration).field("FrameExtent", &self.FrameExtent).field("DataUsed", &self.DataUsed).field("Data", &self.Data).field("OptionsFlags", &self.OptionsFlags).finish()
    }
}
impl ::core::default::Default for KSSTREAM_METADATA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSSTREAM_METADATA_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.BufferSize == other.BufferSize && self.UsedSize == other.UsedSize && self.Data == other.Data && self.SystemVa == other.SystemVa && self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSSTREAM_METADATA_INFO {}
impl ::core::fmt::Debug for KSSTREAM_METADATA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSSTREAM_METADATA_INFO").field("BufferSize", &self.BufferSize).field("UsedSize", &self.UsedSize).field("Data", &self.Data).field("SystemVa", &self.SystemVa).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSSTREAM_UVC_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSTELEPHONY_CALLCONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSTELEPHONY_CALLCONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.CallType == other.CallType && self.CallControlOp == other.CallControlOp
    }
}
impl ::core::cmp::Eq for KSTELEPHONY_CALLCONTROL {}
impl ::core::fmt::Debug for KSTELEPHONY_CALLCONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTELEPHONY_CALLCONTROL").field("CallType", &self.CallType).field("CallControlOp", &self.CallControlOp).finish()
    }
}
impl ::core::default::Default for KSTELEPHONY_CALLINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSTELEPHONY_CALLINFO {
    fn eq(&self, other: &Self) -> bool {
        self.CallType == other.CallType && self.CallState == other.CallState
    }
}
impl ::core::cmp::Eq for KSTELEPHONY_CALLINFO {}
impl ::core::fmt::Debug for KSTELEPHONY_CALLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTELEPHONY_CALLINFO").field("CallType", &self.CallType).field("CallState", &self.CallState).finish()
    }
}
impl ::core::default::Default for KSTELEPHONY_PROVIDERCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSTELEPHONY_PROVIDERCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.CallType == other.CallType && self.ProviderChangeOp == other.ProviderChangeOp
    }
}
impl ::core::cmp::Eq for KSTELEPHONY_PROVIDERCHANGE {}
impl ::core::fmt::Debug for KSTELEPHONY_PROVIDERCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTELEPHONY_PROVIDERCHANGE").field("CallType", &self.CallType).field("ProviderChangeOp", &self.ProviderChangeOp).finish()
    }
}
impl ::core::default::Default for KSTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSTIME {
    fn eq(&self, other: &Self) -> bool {
        self.Time == other.Time && self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for KSTIME {}
impl ::core::fmt::Debug for KSTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTIME").field("Time", &self.Time).field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::default::Default for KSTOPOLOGY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSTOPOLOGY {
    fn eq(&self, other: &Self) -> bool {
        self.CategoriesCount == other.CategoriesCount && self.Categories == other.Categories && self.TopologyNodesCount == other.TopologyNodesCount && self.TopologyNodes == other.TopologyNodes && self.TopologyConnectionsCount == other.TopologyConnectionsCount && self.TopologyConnections == other.TopologyConnections && self.TopologyNodesNames == other.TopologyNodesNames && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSTOPOLOGY {}
impl ::core::fmt::Debug for KSTOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTOPOLOGY").field("CategoriesCount", &self.CategoriesCount).field("Categories", &self.Categories).field("TopologyNodesCount", &self.TopologyNodesCount).field("TopologyNodes", &self.TopologyNodes).field("TopologyConnectionsCount", &self.TopologyConnectionsCount).field("TopologyConnections", &self.TopologyConnections).field("TopologyNodesNames", &self.TopologyNodesNames).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSTOPOLOGY_CONNECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSTOPOLOGY_CONNECTION {
    fn eq(&self, other: &Self) -> bool {
        self.FromNode == other.FromNode && self.FromNodePin == other.FromNodePin && self.ToNode == other.ToNode && self.ToNodePin == other.ToNodePin
    }
}
impl ::core::cmp::Eq for KSTOPOLOGY_CONNECTION {}
impl ::core::fmt::Debug for KSTOPOLOGY_CONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTOPOLOGY_CONNECTION").field("FromNode", &self.FromNode).field("FromNodePin", &self.FromNodePin).field("ToNode", &self.ToNode).field("ToNodePin", &self.ToNodePin).finish()
    }
}
impl ::core::default::Default for KSTOPOLOGY_ENDPOINTID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSTOPOLOGY_ENDPOINTID {
    fn eq(&self, other: &Self) -> bool {
        self.TopologyName == other.TopologyName && self.PinId == other.PinId
    }
}
impl ::core::cmp::Eq for KSTOPOLOGY_ENDPOINTID {}
impl ::core::fmt::Debug for KSTOPOLOGY_ENDPOINTID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTOPOLOGY_ENDPOINTID").field("TopologyName", &self.TopologyName).field("PinId", &self.PinId).finish()
    }
}
impl ::core::default::Default for KSTOPOLOGY_ENDPOINTIDPAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSTOPOLOGY_ENDPOINTIDPAIR {
    fn eq(&self, other: &Self) -> bool {
        self.RenderEndpoint == other.RenderEndpoint && self.CaptureEndpoint == other.CaptureEndpoint
    }
}
impl ::core::cmp::Eq for KSTOPOLOGY_ENDPOINTIDPAIR {}
impl ::core::fmt::Debug for KSTOPOLOGY_ENDPOINTIDPAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSTOPOLOGY_ENDPOINTIDPAIR").field("RenderEndpoint", &self.RenderEndpoint).field("CaptureEndpoint", &self.CaptureEndpoint).finish()
    }
}
impl ::core::default::Default for KSVPMAXPIXELRATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSVPMAXPIXELRATE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.MaxPixelsPerSecond == other.MaxPixelsPerSecond && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KSVPMAXPIXELRATE {}
impl ::core::fmt::Debug for KSVPMAXPIXELRATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSVPMAXPIXELRATE").field("Size", &self.Size).field("MaxPixelsPerSecond", &self.MaxPixelsPerSecond).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KSVPSIZE_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSVPSURFACEPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSVPSURFACEPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwPitch == other.dwPitch && self.dwXOrigin == other.dwXOrigin && self.dwYOrigin == other.dwYOrigin
    }
}
impl ::core::cmp::Eq for KSVPSURFACEPARAMS {}
impl ::core::fmt::Debug for KSVPSURFACEPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSVPSURFACEPARAMS").field("dwPitch", &self.dwPitch).field("dwXOrigin", &self.dwXOrigin).field("dwYOrigin", &self.dwYOrigin).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KSWAVETABLE_WAVE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KSWAVE_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSWAVE_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.Attributes == other.Attributes && self.BufferSize == other.BufferSize && self.BufferAddress == other.BufferAddress
    }
}
impl ::core::cmp::Eq for KSWAVE_BUFFER {}
impl ::core::fmt::Debug for KSWAVE_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSWAVE_BUFFER").field("Attributes", &self.Attributes).field("BufferSize", &self.BufferSize).field("BufferAddress", &self.BufferAddress).finish()
    }
}
impl ::core::default::Default for KSWAVE_COMPATCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSWAVE_COMPATCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.ulDeviceType == other.ulDeviceType
    }
}
impl ::core::cmp::Eq for KSWAVE_COMPATCAPS {}
impl ::core::fmt::Debug for KSWAVE_COMPATCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSWAVE_COMPATCAPS").field("ulDeviceType", &self.ulDeviceType).finish()
    }
}
impl ::core::default::Default for KSWAVE_INPUT_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSWAVE_INPUT_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumChannelsPerConnection == other.MaximumChannelsPerConnection && self.MinimumBitsPerSample == other.MinimumBitsPerSample && self.MaximumBitsPerSample == other.MaximumBitsPerSample && self.MinimumSampleFrequency == other.MinimumSampleFrequency && self.MaximumSampleFrequency == other.MaximumSampleFrequency && self.TotalConnections == other.TotalConnections && self.ActiveConnections == other.ActiveConnections
    }
}
impl ::core::cmp::Eq for KSWAVE_INPUT_CAPABILITIES {}
impl ::core::fmt::Debug for KSWAVE_INPUT_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSWAVE_INPUT_CAPABILITIES").field("MaximumChannelsPerConnection", &self.MaximumChannelsPerConnection).field("MinimumBitsPerSample", &self.MinimumBitsPerSample).field("MaximumBitsPerSample", &self.MaximumBitsPerSample).field("MinimumSampleFrequency", &self.MinimumSampleFrequency).field("MaximumSampleFrequency", &self.MaximumSampleFrequency).field("TotalConnections", &self.TotalConnections).field("ActiveConnections", &self.ActiveConnections).finish()
    }
}
impl ::core::default::Default for KSWAVE_OUTPUT_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSWAVE_OUTPUT_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumChannelsPerConnection == other.MaximumChannelsPerConnection
            && self.MinimumBitsPerSample == other.MinimumBitsPerSample
            && self.MaximumBitsPerSample == other.MaximumBitsPerSample
            && self.MinimumSampleFrequency == other.MinimumSampleFrequency
            && self.MaximumSampleFrequency == other.MaximumSampleFrequency
            && self.TotalConnections == other.TotalConnections
            && self.StaticConnections == other.StaticConnections
            && self.StreamingConnections == other.StreamingConnections
            && self.ActiveConnections == other.ActiveConnections
            && self.ActiveStaticConnections == other.ActiveStaticConnections
            && self.ActiveStreamingConnections == other.ActiveStreamingConnections
            && self.Total3DConnections == other.Total3DConnections
            && self.Static3DConnections == other.Static3DConnections
            && self.Streaming3DConnections == other.Streaming3DConnections
            && self.Active3DConnections == other.Active3DConnections
            && self.ActiveStatic3DConnections == other.ActiveStatic3DConnections
            && self.ActiveStreaming3DConnections == other.ActiveStreaming3DConnections
            && self.TotalSampleMemory == other.TotalSampleMemory
            && self.FreeSampleMemory == other.FreeSampleMemory
            && self.LargestFreeContiguousSampleMemory == other.LargestFreeContiguousSampleMemory
    }
}
impl ::core::cmp::Eq for KSWAVE_OUTPUT_CAPABILITIES {}
impl ::core::fmt::Debug for KSWAVE_OUTPUT_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSWAVE_OUTPUT_CAPABILITIES")
            .field("MaximumChannelsPerConnection", &self.MaximumChannelsPerConnection)
            .field("MinimumBitsPerSample", &self.MinimumBitsPerSample)
            .field("MaximumBitsPerSample", &self.MaximumBitsPerSample)
            .field("MinimumSampleFrequency", &self.MinimumSampleFrequency)
            .field("MaximumSampleFrequency", &self.MaximumSampleFrequency)
            .field("TotalConnections", &self.TotalConnections)
            .field("StaticConnections", &self.StaticConnections)
            .field("StreamingConnections", &self.StreamingConnections)
            .field("ActiveConnections", &self.ActiveConnections)
            .field("ActiveStaticConnections", &self.ActiveStaticConnections)
            .field("ActiveStreamingConnections", &self.ActiveStreamingConnections)
            .field("Total3DConnections", &self.Total3DConnections)
            .field("Static3DConnections", &self.Static3DConnections)
            .field("Streaming3DConnections", &self.Streaming3DConnections)
            .field("Active3DConnections", &self.Active3DConnections)
            .field("ActiveStatic3DConnections", &self.ActiveStatic3DConnections)
            .field("ActiveStreaming3DConnections", &self.ActiveStreaming3DConnections)
            .field("TotalSampleMemory", &self.TotalSampleMemory)
            .field("FreeSampleMemory", &self.FreeSampleMemory)
            .field("LargestFreeContiguousSampleMemory", &self.LargestFreeContiguousSampleMemory)
            .finish()
    }
}
impl ::core::default::Default for KSWAVE_VOLUME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KSWAVE_VOLUME {
    fn eq(&self, other: &Self) -> bool {
        self.LeftAttenuation == other.LeftAttenuation && self.RightAttenuation == other.RightAttenuation
    }
}
impl ::core::cmp::Eq for KSWAVE_VOLUME {}
impl ::core::fmt::Debug for KSWAVE_VOLUME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KSWAVE_VOLUME").field("LeftAttenuation", &self.LeftAttenuation).field("RightAttenuation", &self.RightAttenuation).finish()
    }
}
impl ::core::default::Default for KS_AMPixAspectRatio {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_AMPixAspectRatio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_AMPixAspectRatio").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_AMVPDATAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_AMVPDATAINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwMicrosecondsPerField == other.dwMicrosecondsPerField && self.amvpDimInfo == other.amvpDimInfo && self.dwPictAspectRatioX == other.dwPictAspectRatioX && self.dwPictAspectRatioY == other.dwPictAspectRatioY && self.bEnableDoubleClock == other.bEnableDoubleClock && self.bEnableVACT == other.bEnableVACT && self.bDataIsInterlaced == other.bDataIsInterlaced && self.lHalfLinesOdd == other.lHalfLinesOdd && self.bFieldPolarityInverted == other.bFieldPolarityInverted && self.dwNumLinesInVREF == other.dwNumLinesInVREF && self.lHalfLinesEven == other.lHalfLinesEven && self.dwReserved1 == other.dwReserved1
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_AMVPDATAINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_AMVPDATAINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_AMVPDATAINFO")
            .field("dwSize", &self.dwSize)
            .field("dwMicrosecondsPerField", &self.dwMicrosecondsPerField)
            .field("amvpDimInfo", &self.amvpDimInfo)
            .field("dwPictAspectRatioX", &self.dwPictAspectRatioX)
            .field("dwPictAspectRatioY", &self.dwPictAspectRatioY)
            .field("bEnableDoubleClock", &self.bEnableDoubleClock)
            .field("bEnableVACT", &self.bEnableVACT)
            .field("bDataIsInterlaced", &self.bDataIsInterlaced)
            .field("lHalfLinesOdd", &self.lHalfLinesOdd)
            .field("bFieldPolarityInverted", &self.bFieldPolarityInverted)
            .field("dwNumLinesInVREF", &self.dwNumLinesInVREF)
            .field("lHalfLinesEven", &self.lHalfLinesEven)
            .field("dwReserved1", &self.dwReserved1)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_AMVPDIMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_AMVPDIMINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFieldWidth == other.dwFieldWidth && self.dwFieldHeight == other.dwFieldHeight && self.dwVBIWidth == other.dwVBIWidth && self.dwVBIHeight == other.dwVBIHeight && self.rcValidRegion == other.rcValidRegion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_AMVPDIMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_AMVPDIMINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_AMVPDIMINFO").field("dwFieldWidth", &self.dwFieldWidth).field("dwFieldHeight", &self.dwFieldHeight).field("dwVBIWidth", &self.dwVBIWidth).field("dwVBIHeight", &self.dwVBIHeight).field("rcValidRegion", &self.rcValidRegion).finish()
    }
}
impl ::core::default::Default for KS_AMVPSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_AMVPSIZE {
    fn eq(&self, other: &Self) -> bool {
        self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight
    }
}
impl ::core::cmp::Eq for KS_AMVPSIZE {}
impl ::core::fmt::Debug for KS_AMVPSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_AMVPSIZE").field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).finish()
    }
}
impl ::core::default::Default for KS_AMVP_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_AMVP_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_AMVP_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_AMVP_SELECTFORMATBY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_AMVP_SELECTFORMATBY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_AMVP_SELECTFORMATBY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_AM_ExactRateChange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_AM_ExactRateChange {
    fn eq(&self, other: &Self) -> bool {
        self.OutputZeroTime == other.OutputZeroTime && self.Rate == other.Rate
    }
}
impl ::core::cmp::Eq for KS_AM_ExactRateChange {}
impl ::core::fmt::Debug for KS_AM_ExactRateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_AM_ExactRateChange").field("OutputZeroTime", &self.OutputZeroTime).field("Rate", &self.Rate).finish()
    }
}
impl ::core::default::Default for KS_AM_PROPERTY_TS_RATE_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_AM_PROPERTY_TS_RATE_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_AM_PROPERTY_TS_RATE_CHANGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_AM_SimpleRateChange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_AM_SimpleRateChange {
    fn eq(&self, other: &Self) -> bool {
        self.StartTime == other.StartTime && self.Rate == other.Rate
    }
}
impl ::core::cmp::Eq for KS_AM_SimpleRateChange {}
impl ::core::fmt::Debug for KS_AM_SimpleRateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_AM_SimpleRateChange").field("StartTime", &self.StartTime).field("Rate", &self.Rate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_ANALOGVIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_ANALOGVIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        self.rcSource == other.rcSource && self.rcTarget == other.rcTarget && self.dwActiveWidth == other.dwActiveWidth && self.dwActiveHeight == other.dwActiveHeight && self.AvgTimePerFrame == other.AvgTimePerFrame
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_ANALOGVIDEOINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_ANALOGVIDEOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_ANALOGVIDEOINFO").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwActiveWidth", &self.dwActiveWidth).field("dwActiveHeight", &self.dwActiveHeight).field("AvgTimePerFrame", &self.AvgTimePerFrame).finish()
    }
}
impl ::core::default::Default for KS_AnalogVideoStandard {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_AnalogVideoStandard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_AnalogVideoStandard").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_BITMAPINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_BITMAPINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.biSize == other.biSize && self.biWidth == other.biWidth && self.biHeight == other.biHeight && self.biPlanes == other.biPlanes && self.biBitCount == other.biBitCount && self.biCompression == other.biCompression && self.biSizeImage == other.biSizeImage && self.biXPelsPerMeter == other.biXPelsPerMeter && self.biYPelsPerMeter == other.biYPelsPerMeter && self.biClrUsed == other.biClrUsed && self.biClrImportant == other.biClrImportant
    }
}
impl ::core::cmp::Eq for KS_BITMAPINFOHEADER {}
impl ::core::fmt::Debug for KS_BITMAPINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_BITMAPINFOHEADER").field("biSize", &self.biSize).field("biWidth", &self.biWidth).field("biHeight", &self.biHeight).field("biPlanes", &self.biPlanes).field("biBitCount", &self.biBitCount).field("biCompression", &self.biCompression).field("biSizeImage", &self.biSizeImage).field("biXPelsPerMeter", &self.biXPelsPerMeter).field("biYPelsPerMeter", &self.biYPelsPerMeter).field("biClrUsed", &self.biClrUsed).field("biClrImportant", &self.biClrImportant).finish()
    }
}
impl ::core::default::Default for KS_COLCON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_COLCON {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield1 == other._bitfield1 && self._bitfield2 == other._bitfield2 && self._bitfield3 == other._bitfield3 && self._bitfield4 == other._bitfield4
    }
}
impl ::core::cmp::Eq for KS_COLCON {}
impl ::core::fmt::Debug for KS_COLCON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_COLCON").field("_bitfield1", &self._bitfield1).field("_bitfield2", &self._bitfield2).field("_bitfield3", &self._bitfield3).field("_bitfield4", &self._bitfield4).finish()
    }
}
impl ::core::default::Default for KS_COMPRESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_COMPRESSION {
    fn eq(&self, other: &Self) -> bool {
        self.RatioNumerator == other.RatioNumerator && self.RatioDenominator == other.RatioDenominator && self.RatioConstantMargin == other.RatioConstantMargin
    }
}
impl ::core::cmp::Eq for KS_COMPRESSION {}
impl ::core::fmt::Debug for KS_COMPRESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_COMPRESSION").field("RatioNumerator", &self.RatioNumerator).field("RatioDenominator", &self.RatioDenominator).field("RatioConstantMargin", &self.RatioConstantMargin).finish()
    }
}
impl ::core::default::Default for KS_COPY_MACROVISION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_COPY_MACROVISION {
    fn eq(&self, other: &Self) -> bool {
        self.MACROVISIONLevel == other.MACROVISIONLevel
    }
}
impl ::core::cmp::Eq for KS_COPY_MACROVISION {}
impl ::core::fmt::Debug for KS_COPY_MACROVISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_COPY_MACROVISION").field("MACROVISIONLevel", &self.MACROVISIONLevel).finish()
    }
}
impl ::core::default::Default for KS_COPY_MACROVISION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_COPY_MACROVISION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_COPY_MACROVISION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_CameraControlAsyncOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_CameraControlAsyncOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_CameraControlAsyncOperation").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_CompressionCaps {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_CompressionCaps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_CompressionCaps").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_DATAFORMAT_H264VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KS_DATAFORMAT_IMAGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATAFORMAT_MPEGVIDEOINFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KS_DATAFORMAT_VBIINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATAFORMAT_VIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATAFORMAT_VIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATAFORMAT_VIDEOINFO_PALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_ANALOGVIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_H264_VIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_IMAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_MPEG1_VIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_MPEG2_VIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_VIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_VIDEO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_VIDEO_PALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_DATARANGE_VIDEO_VBI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KS_DVDCOPYSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_DVDCOPYSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_DVDCOPYSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_DVDCOPY_BUSKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_BUSKEY {
    fn eq(&self, other: &Self) -> bool {
        self.BusKey == other.BusKey && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_BUSKEY {}
impl ::core::fmt::Debug for KS_DVDCOPY_BUSKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVDCOPY_BUSKEY").field("BusKey", &self.BusKey).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KS_DVDCOPY_CHLGKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_CHLGKEY {
    fn eq(&self, other: &Self) -> bool {
        self.ChlgKey == other.ChlgKey && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_CHLGKEY {}
impl ::core::fmt::Debug for KS_DVDCOPY_CHLGKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVDCOPY_CHLGKEY").field("ChlgKey", &self.ChlgKey).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KS_DVDCOPY_DISCKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_DISCKEY {
    fn eq(&self, other: &Self) -> bool {
        self.DiscKey == other.DiscKey
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_DISCKEY {}
impl ::core::fmt::Debug for KS_DVDCOPY_DISCKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVDCOPY_DISCKEY").field("DiscKey", &self.DiscKey).finish()
    }
}
impl ::core::default::Default for KS_DVDCOPY_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved && self.RegionData == other.RegionData && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_REGION {}
impl ::core::fmt::Debug for KS_DVDCOPY_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVDCOPY_REGION").field("Reserved", &self.Reserved).field("RegionData", &self.RegionData).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for KS_DVDCOPY_SET_COPY_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_SET_COPY_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.DVDCopyState == other.DVDCopyState
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_SET_COPY_STATE {}
impl ::core::fmt::Debug for KS_DVDCOPY_SET_COPY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVDCOPY_SET_COPY_STATE").field("DVDCopyState", &self.DVDCopyState).finish()
    }
}
impl ::core::default::Default for KS_DVDCOPY_TITLEKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_DVDCOPY_TITLEKEY {
    fn eq(&self, other: &Self) -> bool {
        self.KeyFlags == other.KeyFlags && self.ReservedNT == other.ReservedNT && self.TitleKey == other.TitleKey && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for KS_DVDCOPY_TITLEKEY {}
impl ::core::fmt::Debug for KS_DVDCOPY_TITLEKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVDCOPY_TITLEKEY").field("KeyFlags", &self.KeyFlags).field("ReservedNT", &self.ReservedNT).field("TitleKey", &self.TitleKey).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for KS_DVD_YCrCb {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_DVD_YCrCb {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved && self.Y == other.Y && self.Cr == other.Cr && self.Cb == other.Cb
    }
}
impl ::core::cmp::Eq for KS_DVD_YCrCb {}
impl ::core::fmt::Debug for KS_DVD_YCrCb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVD_YCrCb").field("Reserved", &self.Reserved).field("Y", &self.Y).field("Cr", &self.Cr).field("Cb", &self.Cb).finish()
    }
}
impl ::core::default::Default for KS_DVD_YUV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_DVD_YUV {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved && self.Y == other.Y && self.V == other.V && self.U == other.U
    }
}
impl ::core::cmp::Eq for KS_DVD_YUV {}
impl ::core::fmt::Debug for KS_DVD_YUV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_DVD_YUV").field("Reserved", &self.Reserved).field("Y", &self.Y).field("V", &self.V).field("U", &self.U).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_FRAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KS_FRAMING_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KS_FRAMING_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_FRAMING_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.MinFrameSize == other.MinFrameSize && self.MaxFrameSize == other.MaxFrameSize && self.Stepping == other.Stepping
    }
}
impl ::core::cmp::Eq for KS_FRAMING_RANGE {}
impl ::core::fmt::Debug for KS_FRAMING_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_FRAMING_RANGE").field("MinFrameSize", &self.MinFrameSize).field("MaxFrameSize", &self.MaxFrameSize).field("Stepping", &self.Stepping).finish()
    }
}
impl ::core::default::Default for KS_FRAMING_RANGE_WEIGHTED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_FRAMING_RANGE_WEIGHTED {
    fn eq(&self, other: &Self) -> bool {
        self.Range == other.Range && self.InPlaceWeight == other.InPlaceWeight && self.NotInPlaceWeight == other.NotInPlaceWeight
    }
}
impl ::core::cmp::Eq for KS_FRAMING_RANGE_WEIGHTED {}
impl ::core::fmt::Debug for KS_FRAMING_RANGE_WEIGHTED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_FRAMING_RANGE_WEIGHTED").field("Range", &self.Range).field("InPlaceWeight", &self.InPlaceWeight).field("NotInPlaceWeight", &self.NotInPlaceWeight).finish()
    }
}
impl ::core::default::Default for KS_H264VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_H264VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        self.wWidth == other.wWidth
            && self.wHeight == other.wHeight
            && self.wSARwidth == other.wSARwidth
            && self.wSARheight == other.wSARheight
            && self.wProfile == other.wProfile
            && self.bLevelIDC == other.bLevelIDC
            && self.wConstrainedToolset == other.wConstrainedToolset
            && self.bmSupportedUsages == other.bmSupportedUsages
            && self.bmCapabilities == other.bmCapabilities
            && self.bmSVCCapabilities == other.bmSVCCapabilities
            && self.bmMVCCapabilities == other.bmMVCCapabilities
            && self.dwFrameInterval == other.dwFrameInterval
            && self.bMaxCodecConfigDelay == other.bMaxCodecConfigDelay
            && self.bmSupportedSliceModes == other.bmSupportedSliceModes
            && self.bmSupportedSyncFrameTypes == other.bmSupportedSyncFrameTypes
            && self.bResolutionScaling == other.bResolutionScaling
            && self.bSimulcastSupport == other.bSimulcastSupport
            && self.bmSupportedRateControlModes == other.bmSupportedRateControlModes
            && self.wMaxMBperSecOneResolutionNoScalability == other.wMaxMBperSecOneResolutionNoScalability
            && self.wMaxMBperSecTwoResolutionsNoScalability == other.wMaxMBperSecTwoResolutionsNoScalability
            && self.wMaxMBperSecThreeResolutionsNoScalability == other.wMaxMBperSecThreeResolutionsNoScalability
            && self.wMaxMBperSecFourResolutionsNoScalability == other.wMaxMBperSecFourResolutionsNoScalability
            && self.wMaxMBperSecOneResolutionTemporalScalability == other.wMaxMBperSecOneResolutionTemporalScalability
            && self.wMaxMBperSecTwoResolutionsTemporalScalablility == other.wMaxMBperSecTwoResolutionsTemporalScalablility
            && self.wMaxMBperSecThreeResolutionsTemporalScalability == other.wMaxMBperSecThreeResolutionsTemporalScalability
            && self.wMaxMBperSecFourResolutionsTemporalScalability == other.wMaxMBperSecFourResolutionsTemporalScalability
            && self.wMaxMBperSecOneResolutionTemporalQualityScalability == other.wMaxMBperSecOneResolutionTemporalQualityScalability
            && self.wMaxMBperSecTwoResolutionsTemporalQualityScalability == other.wMaxMBperSecTwoResolutionsTemporalQualityScalability
            && self.wMaxMBperSecThreeResolutionsTemporalQualityScalablity == other.wMaxMBperSecThreeResolutionsTemporalQualityScalablity
            && self.wMaxMBperSecFourResolutionsTemporalQualityScalability == other.wMaxMBperSecFourResolutionsTemporalQualityScalability
            && self.wMaxMBperSecOneResolutionTemporalSpatialScalability == other.wMaxMBperSecOneResolutionTemporalSpatialScalability
            && self.wMaxMBperSecTwoResolutionsTemporalSpatialScalability == other.wMaxMBperSecTwoResolutionsTemporalSpatialScalability
            && self.wMaxMBperSecThreeResolutionsTemporalSpatialScalablity == other.wMaxMBperSecThreeResolutionsTemporalSpatialScalablity
            && self.wMaxMBperSecFourResolutionsTemporalSpatialScalability == other.wMaxMBperSecFourResolutionsTemporalSpatialScalability
            && self.wMaxMBperSecOneResolutionFullScalability == other.wMaxMBperSecOneResolutionFullScalability
            && self.wMaxMBperSecTwoResolutionsFullScalability == other.wMaxMBperSecTwoResolutionsFullScalability
            && self.wMaxMBperSecThreeResolutionsFullScalability == other.wMaxMBperSecThreeResolutionsFullScalability
            && self.wMaxMBperSecFourResolutionsFullScalability == other.wMaxMBperSecFourResolutionsFullScalability
    }
}
impl ::core::cmp::Eq for KS_H264VIDEOINFO {}
impl ::core::fmt::Debug for KS_H264VIDEOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_H264VIDEOINFO")
            .field("wWidth", &self.wWidth)
            .field("wHeight", &self.wHeight)
            .field("wSARwidth", &self.wSARwidth)
            .field("wSARheight", &self.wSARheight)
            .field("wProfile", &self.wProfile)
            .field("bLevelIDC", &self.bLevelIDC)
            .field("wConstrainedToolset", &self.wConstrainedToolset)
            .field("bmSupportedUsages", &self.bmSupportedUsages)
            .field("bmCapabilities", &self.bmCapabilities)
            .field("bmSVCCapabilities", &self.bmSVCCapabilities)
            .field("bmMVCCapabilities", &self.bmMVCCapabilities)
            .field("dwFrameInterval", &self.dwFrameInterval)
            .field("bMaxCodecConfigDelay", &self.bMaxCodecConfigDelay)
            .field("bmSupportedSliceModes", &self.bmSupportedSliceModes)
            .field("bmSupportedSyncFrameTypes", &self.bmSupportedSyncFrameTypes)
            .field("bResolutionScaling", &self.bResolutionScaling)
            .field("bSimulcastSupport", &self.bSimulcastSupport)
            .field("bmSupportedRateControlModes", &self.bmSupportedRateControlModes)
            .field("wMaxMBperSecOneResolutionNoScalability", &self.wMaxMBperSecOneResolutionNoScalability)
            .field("wMaxMBperSecTwoResolutionsNoScalability", &self.wMaxMBperSecTwoResolutionsNoScalability)
            .field("wMaxMBperSecThreeResolutionsNoScalability", &self.wMaxMBperSecThreeResolutionsNoScalability)
            .field("wMaxMBperSecFourResolutionsNoScalability", &self.wMaxMBperSecFourResolutionsNoScalability)
            .field("wMaxMBperSecOneResolutionTemporalScalability", &self.wMaxMBperSecOneResolutionTemporalScalability)
            .field("wMaxMBperSecTwoResolutionsTemporalScalablility", &self.wMaxMBperSecTwoResolutionsTemporalScalablility)
            .field("wMaxMBperSecThreeResolutionsTemporalScalability", &self.wMaxMBperSecThreeResolutionsTemporalScalability)
            .field("wMaxMBperSecFourResolutionsTemporalScalability", &self.wMaxMBperSecFourResolutionsTemporalScalability)
            .field("wMaxMBperSecOneResolutionTemporalQualityScalability", &self.wMaxMBperSecOneResolutionTemporalQualityScalability)
            .field("wMaxMBperSecTwoResolutionsTemporalQualityScalability", &self.wMaxMBperSecTwoResolutionsTemporalQualityScalability)
            .field("wMaxMBperSecThreeResolutionsTemporalQualityScalablity", &self.wMaxMBperSecThreeResolutionsTemporalQualityScalablity)
            .field("wMaxMBperSecFourResolutionsTemporalQualityScalability", &self.wMaxMBperSecFourResolutionsTemporalQualityScalability)
            .field("wMaxMBperSecOneResolutionTemporalSpatialScalability", &self.wMaxMBperSecOneResolutionTemporalSpatialScalability)
            .field("wMaxMBperSecTwoResolutionsTemporalSpatialScalability", &self.wMaxMBperSecTwoResolutionsTemporalSpatialScalability)
            .field("wMaxMBperSecThreeResolutionsTemporalSpatialScalablity", &self.wMaxMBperSecThreeResolutionsTemporalSpatialScalablity)
            .field("wMaxMBperSecFourResolutionsTemporalSpatialScalability", &self.wMaxMBperSecFourResolutionsTemporalSpatialScalability)
            .field("wMaxMBperSecOneResolutionFullScalability", &self.wMaxMBperSecOneResolutionFullScalability)
            .field("wMaxMBperSecTwoResolutionsFullScalability", &self.wMaxMBperSecTwoResolutionsFullScalability)
            .field("wMaxMBperSecThreeResolutionsFullScalability", &self.wMaxMBperSecThreeResolutionsFullScalability)
            .field("wMaxMBperSecFourResolutionsFullScalability", &self.wMaxMBperSecFourResolutionsFullScalability)
            .finish()
    }
}
impl ::core::default::Default for KS_LogicalMemoryType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_LogicalMemoryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_LogicalMemoryType").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_MPEG1VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_MPEG1VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwStartTimeCode == other.dwStartTimeCode && self.cbSequenceHeader == other.cbSequenceHeader && self.bSequenceHeader == other.bSequenceHeader
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_MPEG1VIDEOINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_MPEG1VIDEOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_MPEG1VIDEOINFO").field("hdr", &self.hdr).field("dwStartTimeCode", &self.dwStartTimeCode).field("cbSequenceHeader", &self.cbSequenceHeader).field("bSequenceHeader", &self.bSequenceHeader).finish()
    }
}
impl ::core::default::Default for KS_MPEG2Level {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_MPEG2Level {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_MPEG2Level").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_MPEG2Profile {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_MPEG2Profile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_MPEG2Profile").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_MPEGAUDIOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_MPEGAUDIOINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3
    }
}
impl ::core::cmp::Eq for KS_MPEGAUDIOINFO {}
impl ::core::fmt::Debug for KS_MPEGAUDIOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_MPEGAUDIOINFO").field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).field("dwReserved3", &self.dwReserved3).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_MPEGVIDEOINFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for KS_PhysicalConnectorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_PhysicalConnectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_PhysicalConnectorType").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_RGBQUAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_RGBQUAD {
    fn eq(&self, other: &Self) -> bool {
        self.rgbBlue == other.rgbBlue && self.rgbGreen == other.rgbGreen && self.rgbRed == other.rgbRed && self.rgbReserved == other.rgbReserved
    }
}
impl ::core::cmp::Eq for KS_RGBQUAD {}
impl ::core::fmt::Debug for KS_RGBQUAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_RGBQUAD").field("rgbBlue", &self.rgbBlue).field("rgbGreen", &self.rgbGreen).field("rgbRed", &self.rgbRed).field("rgbReserved", &self.rgbReserved).finish()
    }
}
impl ::core::default::Default for KS_SEEKING_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_SEEKING_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_SEEKING_CAPABILITIES").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_SEEKING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_SEEKING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_SEEKING_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_TRUECOLORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_TRUECOLORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwBitMasks == other.dwBitMasks && self.bmiColors == other.bmiColors
    }
}
impl ::core::cmp::Eq for KS_TRUECOLORINFO {}
impl ::core::fmt::Debug for KS_TRUECOLORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_TRUECOLORINFO").field("dwBitMasks", &self.dwBitMasks).field("bmiColors", &self.bmiColors).finish()
    }
}
impl ::core::default::Default for KS_TUNER_STRATEGY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_TUNER_STRATEGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_TUNER_STRATEGY").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_TUNER_TUNING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_TUNER_TUNING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_TUNER_TUNING_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_TVTUNER_CHANGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_TVTUNER_CHANGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwCountryCode == other.dwCountryCode && self.dwAnalogVideoStandard == other.dwAnalogVideoStandard && self.dwChannel == other.dwChannel
    }
}
impl ::core::cmp::Eq for KS_TVTUNER_CHANGE_INFO {}
impl ::core::fmt::Debug for KS_TVTUNER_CHANGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_TVTUNER_CHANGE_INFO").field("dwFlags", &self.dwFlags).field("dwCountryCode", &self.dwCountryCode).field("dwAnalogVideoStandard", &self.dwAnalogVideoStandard).field("dwChannel", &self.dwChannel).finish()
    }
}
impl ::core::default::Default for KS_VBIINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_VBIINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.StartLine == other.StartLine && self.EndLine == other.EndLine && self.SamplingFrequency == other.SamplingFrequency && self.MinLineStartTime == other.MinLineStartTime && self.MaxLineStartTime == other.MaxLineStartTime && self.ActualLineStartTime == other.ActualLineStartTime && self.ActualLineEndTime == other.ActualLineEndTime && self.VideoStandard == other.VideoStandard && self.SamplesPerLine == other.SamplesPerLine && self.StrideInBytes == other.StrideInBytes && self.BufferSize == other.BufferSize
    }
}
impl ::core::cmp::Eq for KS_VBIINFOHEADER {}
impl ::core::fmt::Debug for KS_VBIINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_VBIINFOHEADER")
            .field("StartLine", &self.StartLine)
            .field("EndLine", &self.EndLine)
            .field("SamplingFrequency", &self.SamplingFrequency)
            .field("MinLineStartTime", &self.MinLineStartTime)
            .field("MaxLineStartTime", &self.MaxLineStartTime)
            .field("ActualLineStartTime", &self.ActualLineStartTime)
            .field("ActualLineEndTime", &self.ActualLineEndTime)
            .field("VideoStandard", &self.VideoStandard)
            .field("SamplesPerLine", &self.SamplesPerLine)
            .field("StrideInBytes", &self.StrideInBytes)
            .field("BufferSize", &self.BufferSize)
            .finish()
    }
}
impl ::core::default::Default for KS_VBI_FRAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KS_VBI_FRAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ExtendedHeaderSize == other.ExtendedHeaderSize && self.dwFrameFlags == other.dwFrameFlags && self.PictureNumber == other.PictureNumber && self.DropCount == other.DropCount && self.dwSamplingFrequency == other.dwSamplingFrequency && self.TvTunerChangeInfo == other.TvTunerChangeInfo && self.VBIInfoHeader == other.VBIInfoHeader
    }
}
impl ::core::cmp::Eq for KS_VBI_FRAME_INFO {}
impl ::core::fmt::Debug for KS_VBI_FRAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_VBI_FRAME_INFO").field("ExtendedHeaderSize", &self.ExtendedHeaderSize).field("dwFrameFlags", &self.dwFrameFlags).field("PictureNumber", &self.PictureNumber).field("DropCount", &self.DropCount).field("dwSamplingFrequency", &self.dwSamplingFrequency).field("TvTunerChangeInfo", &self.TvTunerChangeInfo).field("VBIInfoHeader", &self.VBIInfoHeader).finish()
    }
}
impl ::core::default::Default for KS_VIDEODECODER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_VIDEODECODER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_VIDEODECODER_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.rcSource == other.rcSource && self.rcTarget == other.rcTarget && self.dwBitRate == other.dwBitRate && self.dwBitErrorRate == other.dwBitErrorRate && self.AvgTimePerFrame == other.AvgTimePerFrame && self.bmiHeader == other.bmiHeader
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_VIDEOINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_VIDEOINFOHEADER").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwBitRate", &self.dwBitRate).field("dwBitErrorRate", &self.dwBitErrorRate).field("AvgTimePerFrame", &self.AvgTimePerFrame).field("bmiHeader", &self.bmiHeader).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KS_VIDEO_STREAM_CONFIG_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KS_VIDEO_STREAM_CONFIG_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid
            && self.VideoStandard == other.VideoStandard
            && self.InputSize == other.InputSize
            && self.MinCroppingSize == other.MinCroppingSize
            && self.MaxCroppingSize == other.MaxCroppingSize
            && self.CropGranularityX == other.CropGranularityX
            && self.CropGranularityY == other.CropGranularityY
            && self.CropAlignX == other.CropAlignX
            && self.CropAlignY == other.CropAlignY
            && self.MinOutputSize == other.MinOutputSize
            && self.MaxOutputSize == other.MaxOutputSize
            && self.OutputGranularityX == other.OutputGranularityX
            && self.OutputGranularityY == other.OutputGranularityY
            && self.StretchTapsX == other.StretchTapsX
            && self.StretchTapsY == other.StretchTapsY
            && self.ShrinkTapsX == other.ShrinkTapsX
            && self.ShrinkTapsY == other.ShrinkTapsY
            && self.MinFrameInterval == other.MinFrameInterval
            && self.MaxFrameInterval == other.MaxFrameInterval
            && self.MinBitsPerSecond == other.MinBitsPerSecond
            && self.MaxBitsPerSecond == other.MaxBitsPerSecond
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KS_VIDEO_STREAM_CONFIG_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KS_VIDEO_STREAM_CONFIG_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KS_VIDEO_STREAM_CONFIG_CAPS")
            .field("guid", &self.guid)
            .field("VideoStandard", &self.VideoStandard)
            .field("InputSize", &self.InputSize)
            .field("MinCroppingSize", &self.MinCroppingSize)
            .field("MaxCroppingSize", &self.MaxCroppingSize)
            .field("CropGranularityX", &self.CropGranularityX)
            .field("CropGranularityY", &self.CropGranularityY)
            .field("CropAlignX", &self.CropAlignX)
            .field("CropAlignY", &self.CropAlignY)
            .field("MinOutputSize", &self.MinOutputSize)
            .field("MaxOutputSize", &self.MaxOutputSize)
            .field("OutputGranularityX", &self.OutputGranularityX)
            .field("OutputGranularityY", &self.OutputGranularityY)
            .field("StretchTapsX", &self.StretchTapsX)
            .field("StretchTapsY", &self.StretchTapsY)
            .field("ShrinkTapsX", &self.ShrinkTapsX)
            .field("ShrinkTapsY", &self.ShrinkTapsY)
            .field("MinFrameInterval", &self.MinFrameInterval)
            .field("MaxFrameInterval", &self.MaxFrameInterval)
            .field("MinBitsPerSecond", &self.MinBitsPerSecond)
            .field("MaxBitsPerSecond", &self.MaxBitsPerSecond)
            .finish()
    }
}
impl ::core::default::Default for KS_VideoControlFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_VideoControlFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_VideoControlFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for KS_VideoStreamingHints {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KS_VideoStreamingHints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KS_VideoStreamingHints").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOOPEDSTREAMING_POSITION_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MEDIUM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MEDIUM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MediaPresent == other.MediaPresent && self.MediaType == other.MediaType && self.RecordInhibit == other.RecordInhibit
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MEDIUM_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MEDIUM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEDIUM_INFO").field("MediaPresent", &self.MediaPresent).field("MediaType", &self.MediaType).field("RecordInhibit", &self.RecordInhibit).finish()
    }
}
impl ::core::default::Default for MF_MDL_SHARED_PAYLOAD_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NABTSFEC_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NABTSFEC_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.dataSize == other.dataSize && self.groupID == other.groupID && self.Reserved == other.Reserved && self.data == other.data
    }
}
impl ::core::cmp::Eq for NABTSFEC_BUFFER {}
impl ::core::fmt::Debug for NABTSFEC_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NABTSFEC_BUFFER").field("dataSize", &self.dataSize).field("groupID", &self.groupID).field("Reserved", &self.Reserved).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for NABTS_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NABTS_BUFFER_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NABTS_BUFFER_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Confidence == other.Confidence && self.Bytes == other.Bytes
    }
}
impl ::core::cmp::Eq for NABTS_BUFFER_LINE {}
impl ::core::fmt::Debug for NABTS_BUFFER_LINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NABTS_BUFFER_LINE").field("Confidence", &self.Confidence).field("Bytes", &self.Bytes).finish()
    }
}
impl ::core::default::Default for OPTIMAL_WEIGHT_TOTALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OPTIMAL_WEIGHT_TOTALS {
    fn eq(&self, other: &Self) -> bool {
        self.MinTotalNominator == other.MinTotalNominator && self.MaxTotalNominator == other.MaxTotalNominator && self.TotalDenominator == other.TotalDenominator
    }
}
impl ::core::cmp::Eq for OPTIMAL_WEIGHT_TOTALS {}
impl ::core::fmt::Debug for OPTIMAL_WEIGHT_TOTALS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPTIMAL_WEIGHT_TOTALS").field("MinTotalNominator", &self.MinTotalNominator).field("MaxTotalNominator", &self.MaxTotalNominator).field("TotalDenominator", &self.TotalDenominator).finish()
    }
}
impl ::core::default::Default for PIPE_ALLOCATOR_PLACE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PIPE_ALLOCATOR_PLACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PIPE_ALLOCATOR_PLACE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PIPE_DIMENSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PIPE_DIMENSIONS {
    fn eq(&self, other: &Self) -> bool {
        self.AllocatorPin == other.AllocatorPin && self.MaxExpansionPin == other.MaxExpansionPin && self.EndPin == other.EndPin
    }
}
impl ::core::cmp::Eq for PIPE_DIMENSIONS {}
impl ::core::fmt::Debug for PIPE_DIMENSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PIPE_DIMENSIONS").field("AllocatorPin", &self.AllocatorPin).field("MaxExpansionPin", &self.MaxExpansionPin).field("EndPin", &self.EndPin).finish()
    }
}
impl ::core::default::Default for PIPE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PIPE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PIPE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PIPE_TERMINATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PIPE_TERMINATION {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.OutsideFactors == other.OutsideFactors && self.Weigth == other.Weigth && self.PhysicalRange == other.PhysicalRange && self.OptimalRange == other.OptimalRange && self.Compression == other.Compression
    }
}
impl ::core::cmp::Eq for PIPE_TERMINATION {}
impl ::core::fmt::Debug for PIPE_TERMINATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PIPE_TERMINATION").field("Flags", &self.Flags).field("OutsideFactors", &self.OutsideFactors).field("Weigth", &self.Weigth).field("PhysicalRange", &self.PhysicalRange).field("OptimalRange", &self.OptimalRange).field("Compression", &self.Compression).finish()
    }
}
impl ::core::default::Default for SECURE_BUFFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SECURE_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.guidBufferIdentifier == other.guidBufferIdentifier && self.cbBufferSize == other.cbBufferSize && self.cbCaptured == other.cbCaptured && self.ullReserved == other.ullReserved
    }
}
impl ::core::cmp::Eq for SECURE_BUFFER_INFO {}
impl ::core::fmt::Debug for SECURE_BUFFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURE_BUFFER_INFO").field("guidBufferIdentifier", &self.guidBufferIdentifier).field("cbBufferSize", &self.cbBufferSize).field("cbCaptured", &self.cbCaptured).field("ullReserved", &self.ullReserved).finish()
    }
}
impl ::core::default::Default for SOUNDDETECTOR_PATTERNHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOUNDDETECTOR_PATTERNHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.PatternType == other.PatternType
    }
}
impl ::core::cmp::Eq for SOUNDDETECTOR_PATTERNHEADER {}
impl ::core::fmt::Debug for SOUNDDETECTOR_PATTERNHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOUNDDETECTOR_PATTERNHEADER").field("Size", &self.Size).field("PatternType", &self.PatternType).finish()
    }
}
impl ::core::default::Default for TELEPHONY_CALLCONTROLOP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TELEPHONY_CALLCONTROLOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TELEPHONY_CALLCONTROLOP").field(&self.0).finish()
    }
}
impl ::core::default::Default for TELEPHONY_CALLSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TELEPHONY_CALLSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TELEPHONY_CALLSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TELEPHONY_CALLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TELEPHONY_CALLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TELEPHONY_CALLTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TELEPHONY_PROVIDERCHANGEOP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TELEPHONY_PROVIDERCHANGEOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TELEPHONY_PROVIDERCHANGEOP").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRANSPORTAUDIOPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSPORTAUDIOPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.EnableOutput == other.EnableOutput && self.EnableRecord == other.EnableRecord && self.EnableSelsync == other.EnableSelsync && self.Input == other.Input && self.MonitorSource == other.MonitorSource
    }
}
impl ::core::cmp::Eq for TRANSPORTAUDIOPARMS {}
impl ::core::fmt::Debug for TRANSPORTAUDIOPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORTAUDIOPARMS").field("EnableOutput", &self.EnableOutput).field("EnableRecord", &self.EnableRecord).field("EnableSelsync", &self.EnableSelsync).field("Input", &self.Input).field("MonitorSource", &self.MonitorSource).finish()
    }
}
impl ::core::default::Default for TRANSPORTBASICPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSPORTBASICPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.TimeFormat == other.TimeFormat
            && self.TimeReference == other.TimeReference
            && self.Superimpose == other.Superimpose
            && self.EndStopAction == other.EndStopAction
            && self.RecordFormat == other.RecordFormat
            && self.StepFrames == other.StepFrames
            && self.SetpField == other.SetpField
            && self.Preroll == other.Preroll
            && self.RecPreroll == other.RecPreroll
            && self.Postroll == other.Postroll
            && self.EditDelay == other.EditDelay
            && self.PlayTCDelay == other.PlayTCDelay
            && self.RecTCDelay == other.RecTCDelay
            && self.EditField == other.EditField
            && self.FrameServo == other.FrameServo
            && self.ColorFrameServo == other.ColorFrameServo
            && self.ServoRef == other.ServoRef
            && self.WarnGenlock == other.WarnGenlock
            && self.SetTracking == other.SetTracking
            && self.VolumeName == other.VolumeName
            && self.Ballistic == other.Ballistic
            && self.Speed == other.Speed
            && self.CounterFormat == other.CounterFormat
            && self.TunerChannel == other.TunerChannel
            && self.TunerNumber == other.TunerNumber
            && self.TimerEvent == other.TimerEvent
            && self.TimerStartDay == other.TimerStartDay
            && self.TimerStartTime == other.TimerStartTime
            && self.TimerStopDay == other.TimerStopDay
            && self.TimerStopTime == other.TimerStopTime
    }
}
impl ::core::cmp::Eq for TRANSPORTBASICPARMS {}
impl ::core::fmt::Debug for TRANSPORTBASICPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORTBASICPARMS")
            .field("TimeFormat", &self.TimeFormat)
            .field("TimeReference", &self.TimeReference)
            .field("Superimpose", &self.Superimpose)
            .field("EndStopAction", &self.EndStopAction)
            .field("RecordFormat", &self.RecordFormat)
            .field("StepFrames", &self.StepFrames)
            .field("SetpField", &self.SetpField)
            .field("Preroll", &self.Preroll)
            .field("RecPreroll", &self.RecPreroll)
            .field("Postroll", &self.Postroll)
            .field("EditDelay", &self.EditDelay)
            .field("PlayTCDelay", &self.PlayTCDelay)
            .field("RecTCDelay", &self.RecTCDelay)
            .field("EditField", &self.EditField)
            .field("FrameServo", &self.FrameServo)
            .field("ColorFrameServo", &self.ColorFrameServo)
            .field("ServoRef", &self.ServoRef)
            .field("WarnGenlock", &self.WarnGenlock)
            .field("SetTracking", &self.SetTracking)
            .field("VolumeName", &self.VolumeName)
            .field("Ballistic", &self.Ballistic)
            .field("Speed", &self.Speed)
            .field("CounterFormat", &self.CounterFormat)
            .field("TunerChannel", &self.TunerChannel)
            .field("TunerNumber", &self.TunerNumber)
            .field("TimerEvent", &self.TimerEvent)
            .field("TimerStartDay", &self.TimerStartDay)
            .field("TimerStartTime", &self.TimerStartTime)
            .field("TimerStopDay", &self.TimerStopDay)
            .field("TimerStopTime", &self.TimerStopTime)
            .finish()
    }
}
impl ::core::default::Default for TRANSPORTSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSPORTSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.LastError == other.LastError && self.RecordInhibit == other.RecordInhibit && self.ServoLock == other.ServoLock && self.MediaPresent == other.MediaPresent && self.MediaLength == other.MediaLength && self.MediaSize == other.MediaSize && self.MediaTrackCount == other.MediaTrackCount && self.MediaTrackLength == other.MediaTrackLength && self.MediaTrackSide == other.MediaTrackSide && self.MediaType == other.MediaType && self.LinkMode == other.LinkMode && self.NotifyOn == other.NotifyOn
    }
}
impl ::core::cmp::Eq for TRANSPORTSTATUS {}
impl ::core::fmt::Debug for TRANSPORTSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORTSTATUS")
            .field("Mode", &self.Mode)
            .field("LastError", &self.LastError)
            .field("RecordInhibit", &self.RecordInhibit)
            .field("ServoLock", &self.ServoLock)
            .field("MediaPresent", &self.MediaPresent)
            .field("MediaLength", &self.MediaLength)
            .field("MediaSize", &self.MediaSize)
            .field("MediaTrackCount", &self.MediaTrackCount)
            .field("MediaTrackLength", &self.MediaTrackLength)
            .field("MediaTrackSide", &self.MediaTrackSide)
            .field("MediaType", &self.MediaType)
            .field("LinkMode", &self.LinkMode)
            .field("NotifyOn", &self.NotifyOn)
            .finish()
    }
}
impl ::core::default::Default for TRANSPORTVIDEOPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSPORTVIDEOPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.OutputMode == other.OutputMode && self.Input == other.Input
    }
}
impl ::core::cmp::Eq for TRANSPORTVIDEOPARMS {}
impl ::core::fmt::Debug for TRANSPORTVIDEOPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORTVIDEOPARMS").field("OutputMode", &self.OutputMode).field("Input", &self.Input).finish()
    }
}
impl ::core::default::Default for TRANSPORT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSPORT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.State == other.State
    }
}
impl ::core::cmp::Eq for TRANSPORT_STATE {}
impl ::core::fmt::Debug for TRANSPORT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORT_STATE").field("Mode", &self.Mode).field("State", &self.State).finish()
    }
}
impl ::core::default::Default for TUNER_ANALOG_CAPS_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TUNER_ANALOG_CAPS_S {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.StandardsSupported == other.StandardsSupported && self.MinFrequency == other.MinFrequency && self.MaxFrequency == other.MaxFrequency && self.TuningGranularity == other.TuningGranularity && self.SettlingTime == other.SettlingTime && self.ScanSensingRange == other.ScanSensingRange && self.FineTuneSensingRange == other.FineTuneSensingRange
    }
}
impl ::core::cmp::Eq for TUNER_ANALOG_CAPS_S {}
impl ::core::fmt::Debug for TUNER_ANALOG_CAPS_S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TUNER_ANALOG_CAPS_S").field("Mode", &self.Mode).field("StandardsSupported", &self.StandardsSupported).field("MinFrequency", &self.MinFrequency).field("MaxFrequency", &self.MaxFrequency).field("TuningGranularity", &self.TuningGranularity).field("SettlingTime", &self.SettlingTime).field("ScanSensingRange", &self.ScanSensingRange).field("FineTuneSensingRange", &self.FineTuneSensingRange).finish()
    }
}
impl ::core::default::Default for TunerLockType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TunerLockType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TunerLockType").field(&self.0).finish()
    }
}
impl ::core::default::Default for VBICAP_PROPERTIES_PROTECTION_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VBICODECFILTERING_CC_SUBSTREAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_CC_SUBSTREAMS {
    fn eq(&self, other: &Self) -> bool {
        self.SubstreamMask == other.SubstreamMask
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_CC_SUBSTREAMS {}
impl ::core::fmt::Debug for VBICODECFILTERING_CC_SUBSTREAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_CC_SUBSTREAMS").field("SubstreamMask", &self.SubstreamMask).finish()
    }
}
impl ::core::default::Default for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn eq(&self, other: &Self) -> bool {
        self.SubstreamMask == other.SubstreamMask
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_NABTS_SUBSTREAMS {}
impl ::core::fmt::Debug for VBICODECFILTERING_NABTS_SUBSTREAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_NABTS_SUBSTREAMS").field("SubstreamMask", &self.SubstreamMask).finish()
    }
}
impl ::core::default::Default for VBICODECFILTERING_SCANLINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_SCANLINES {
    fn eq(&self, other: &Self) -> bool {
        self.DwordBitArray == other.DwordBitArray
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_SCANLINES {}
impl ::core::fmt::Debug for VBICODECFILTERING_SCANLINES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_SCANLINES").field("DwordBitArray", &self.DwordBitArray).finish()
    }
}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_CC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_CC {
    fn eq(&self, other: &Self) -> bool {
        self.Common == other.Common
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_CC {}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_CC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_CC").field("Common", &self.Common).finish()
    }
}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_CC_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_CC_PIN {
    fn eq(&self, other: &Self) -> bool {
        self.Common == other.Common
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_CC_PIN {}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_CC_PIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_CC_PIN").field("Common", &self.Common).finish()
    }
}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_COMMON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_COMMON {
    fn eq(&self, other: &Self) -> bool {
        self.InputSRBsProcessed == other.InputSRBsProcessed && self.OutputSRBsProcessed == other.OutputSRBsProcessed && self.SRBsIgnored == other.SRBsIgnored && self.InputSRBsMissing == other.InputSRBsMissing && self.OutputSRBsMissing == other.OutputSRBsMissing && self.OutputFailures == other.OutputFailures && self.InternalErrors == other.InternalErrors && self.ExternalErrors == other.ExternalErrors && self.InputDiscontinuities == other.InputDiscontinuities && self.DSPFailures == other.DSPFailures && self.TvTunerChanges == other.TvTunerChanges && self.VBIHeaderChanges == other.VBIHeaderChanges && self.LineConfidenceAvg == other.LineConfidenceAvg && self.BytesOutput == other.BytesOutput
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_COMMON {}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_COMMON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_COMMON")
            .field("InputSRBsProcessed", &self.InputSRBsProcessed)
            .field("OutputSRBsProcessed", &self.OutputSRBsProcessed)
            .field("SRBsIgnored", &self.SRBsIgnored)
            .field("InputSRBsMissing", &self.InputSRBsMissing)
            .field("OutputSRBsMissing", &self.OutputSRBsMissing)
            .field("OutputFailures", &self.OutputFailures)
            .field("InternalErrors", &self.InternalErrors)
            .field("ExternalErrors", &self.ExternalErrors)
            .field("InputDiscontinuities", &self.InputDiscontinuities)
            .field("DSPFailures", &self.DSPFailures)
            .field("TvTunerChanges", &self.TvTunerChanges)
            .field("VBIHeaderChanges", &self.VBIHeaderChanges)
            .field("LineConfidenceAvg", &self.LineConfidenceAvg)
            .field("BytesOutput", &self.BytesOutput)
            .finish()
    }
}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    fn eq(&self, other: &Self) -> bool {
        self.SRBsProcessed == other.SRBsProcessed && self.SRBsIgnored == other.SRBsIgnored && self.SRBsMissing == other.SRBsMissing && self.InternalErrors == other.InternalErrors && self.ExternalErrors == other.ExternalErrors && self.Discontinuities == other.Discontinuities && self.LineConfidenceAvg == other.LineConfidenceAvg && self.BytesOutput == other.BytesOutput
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_COMMON_PIN {}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_COMMON_PIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_COMMON_PIN").field("SRBsProcessed", &self.SRBsProcessed).field("SRBsIgnored", &self.SRBsIgnored).field("SRBsMissing", &self.SRBsMissing).field("InternalErrors", &self.InternalErrors).field("ExternalErrors", &self.ExternalErrors).field("Discontinuities", &self.Discontinuities).field("LineConfidenceAvg", &self.LineConfidenceAvg).field("BytesOutput", &self.BytesOutput).finish()
    }
}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_NABTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_NABTS {
    fn eq(&self, other: &Self) -> bool {
        self.Common == other.Common && self.FECBundleBadLines == other.FECBundleBadLines && self.FECQueueOverflows == other.FECQueueOverflows && self.FECCorrectedLines == other.FECCorrectedLines && self.FECUncorrectableLines == other.FECUncorrectableLines && self.BundlesProcessed == other.BundlesProcessed && self.BundlesSent2IP == other.BundlesSent2IP && self.FilteredLines == other.FilteredLines
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_NABTS {}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_NABTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_NABTS").field("Common", &self.Common).field("FECBundleBadLines", &self.FECBundleBadLines).field("FECQueueOverflows", &self.FECQueueOverflows).field("FECCorrectedLines", &self.FECCorrectedLines).field("FECUncorrectableLines", &self.FECUncorrectableLines).field("BundlesProcessed", &self.BundlesProcessed).field("BundlesSent2IP", &self.BundlesSent2IP).field("FilteredLines", &self.FilteredLines).finish()
    }
}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    fn eq(&self, other: &Self) -> bool {
        self.Common == other.Common
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_NABTS_PIN {}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_NABTS_PIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_NABTS_PIN").field("Common", &self.Common).finish()
    }
}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_TELETEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_TELETEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Common == other.Common
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_TELETEXT {}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_TELETEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_TELETEXT").field("Common", &self.Common).finish()
    }
}
impl ::core::default::Default for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    fn eq(&self, other: &Self) -> bool {
        self.Common == other.Common
    }
}
impl ::core::cmp::Eq for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {}
impl ::core::fmt::Debug for VBICODECFILTERING_STATISTICS_TELETEXT_PIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBICODECFILTERING_STATISTICS_TELETEXT_PIN").field("Common", &self.Common).finish()
    }
}
impl ::core::default::Default for VRAM_SURFACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VRAM_SURFACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.hSurface == other.hSurface && self.VramPhysicalAddress == other.VramPhysicalAddress && self.cbCaptured == other.cbCaptured && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.dwLinearSize == other.dwLinearSize && self.lPitch == other.lPitch && self.ullReserved == other.ullReserved
    }
}
impl ::core::cmp::Eq for VRAM_SURFACE_INFO {}
impl ::core::fmt::Debug for VRAM_SURFACE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VRAM_SURFACE_INFO").field("hSurface", &self.hSurface).field("VramPhysicalAddress", &self.VramPhysicalAddress).field("cbCaptured", &self.cbCaptured).field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).field("dwLinearSize", &self.dwLinearSize).field("lPitch", &self.lPitch).field("ullReserved", &self.ullReserved).finish()
    }
}
impl ::core::default::Default for VRAM_SURFACE_INFO_PROPERTY_S {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WNF_KSCAMERA_STREAMSTATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WNF_KSCAMERA_STREAMSTATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessId == other.ProcessId && self.SessionId == other.SessionId && self.StreamState == other.StreamState && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WNF_KSCAMERA_STREAMSTATE_INFO {}
impl ::core::fmt::Debug for WNF_KSCAMERA_STREAMSTATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNF_KSCAMERA_STREAMSTATE_INFO").field("ProcessId", &self.ProcessId).field("SessionId", &self.SessionId).field("StreamState", &self.StreamState).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for WST_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WST_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.ScanlinesRequested == other.ScanlinesRequested && self.WstLines == other.WstLines
    }
}
impl ::core::cmp::Eq for WST_BUFFER {}
impl ::core::fmt::Debug for WST_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WST_BUFFER").field("ScanlinesRequested", &self.ScanlinesRequested).field("WstLines", &self.WstLines).finish()
    }
}
impl ::core::default::Default for WST_BUFFER_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WST_BUFFER_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Confidence == other.Confidence && self.Bytes == other.Bytes
    }
}
impl ::core::cmp::Eq for WST_BUFFER_LINE {}
impl ::core::fmt::Debug for WST_BUFFER_LINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WST_BUFFER_LINE").field("Confidence", &self.Confidence).field("Bytes", &self.Bytes).finish()
    }
}

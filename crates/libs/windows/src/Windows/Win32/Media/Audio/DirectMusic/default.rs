impl ::core::default::Default for CONNECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONNECTION {
    fn eq(&self, other: &Self) -> bool {
        self.usSource == other.usSource && self.usControl == other.usControl && self.usDestination == other.usDestination && self.usTransform == other.usTransform && self.lScale == other.lScale
    }
}
impl ::core::cmp::Eq for CONNECTION {}
impl ::core::fmt::Debug for CONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTION").field("usSource", &self.usSource).field("usControl", &self.usControl).field("usDestination", &self.usDestination).field("usTransform", &self.usTransform).field("lScale", &self.lScale).finish()
    }
}
impl ::core::default::Default for CONNECTIONLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONNECTIONLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cConnections == other.cConnections
    }
}
impl ::core::cmp::Eq for CONNECTIONLIST {}
impl ::core::fmt::Debug for CONNECTIONLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTIONLIST").field("cbSize", &self.cbSize).field("cConnections", &self.cConnections).finish()
    }
}
impl ::core::default::Default for DIRECTSOUNDDEVICE_DATAFLOW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DIRECTSOUNDDEVICE_DATAFLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTSOUNDDEVICE_DATAFLOW").field(&self.0).finish()
    }
}
impl ::core::default::Default for DIRECTSOUNDDEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DIRECTSOUNDDEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTSOUNDDEVICE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DLSHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DLSHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cInstruments == other.cInstruments
    }
}
impl ::core::cmp::Eq for DLSHEADER {}
impl ::core::fmt::Debug for DLSHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DLSHEADER").field("cInstruments", &self.cInstruments).finish()
    }
}
impl ::core::default::Default for DLSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DLSID {
    fn eq(&self, other: &Self) -> bool {
        self.ulData1 == other.ulData1 && self.usData2 == other.usData2 && self.usData3 == other.usData3 && self.abData4 == other.abData4
    }
}
impl ::core::cmp::Eq for DLSID {}
impl ::core::fmt::Debug for DLSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DLSID").field("ulData1", &self.ulData1).field("usData2", &self.usData2).field("usData3", &self.usData3).field("abData4", &self.abData4).finish()
    }
}
impl ::core::default::Default for DLSVERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DLSVERSION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersionMS == other.dwVersionMS && self.dwVersionLS == other.dwVersionLS
    }
}
impl ::core::cmp::Eq for DLSVERSION {}
impl ::core::fmt::Debug for DLSVERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DLSVERSION").field("dwVersionMS", &self.dwVersionMS).field("dwVersionLS", &self.dwVersionLS).finish()
    }
}
impl ::core::default::Default for DMUS_ARTICPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_ARTICPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.LFO == other.LFO && self.VolEG == other.VolEG && self.PitchEG == other.PitchEG && self.Misc == other.Misc
    }
}
impl ::core::cmp::Eq for DMUS_ARTICPARAMS {}
impl ::core::fmt::Debug for DMUS_ARTICPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_ARTICPARAMS").field("LFO", &self.LFO).field("VolEG", &self.VolEG).field("PitchEG", &self.PitchEG).field("Misc", &self.Misc).finish()
    }
}
impl ::core::default::Default for DMUS_ARTICULATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_ARTICULATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulArt1Idx == other.ulArt1Idx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx
    }
}
impl ::core::cmp::Eq for DMUS_ARTICULATION {}
impl ::core::fmt::Debug for DMUS_ARTICULATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_ARTICULATION").field("ulArt1Idx", &self.ulArt1Idx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).finish()
    }
}
impl ::core::default::Default for DMUS_ARTICULATION2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_ARTICULATION2 {
    fn eq(&self, other: &Self) -> bool {
        self.ulArtIdx == other.ulArtIdx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx && self.ulNextArtIdx == other.ulNextArtIdx
    }
}
impl ::core::cmp::Eq for DMUS_ARTICULATION2 {}
impl ::core::fmt::Debug for DMUS_ARTICULATION2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_ARTICULATION2").field("ulArtIdx", &self.ulArtIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("ulNextArtIdx", &self.ulNextArtIdx).finish()
    }
}
impl ::core::default::Default for DMUS_BUFFERDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_BUFFERDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidBufferFormat == other.guidBufferFormat && self.cbBuffer == other.cbBuffer
    }
}
impl ::core::cmp::Eq for DMUS_BUFFERDESC {}
impl ::core::fmt::Debug for DMUS_BUFFERDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_BUFFERDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidBufferFormat", &self.guidBufferFormat).field("cbBuffer", &self.cbBuffer).finish()
    }
}
impl ::core::default::Default for DMUS_CLOCKINFO7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_CLOCKINFO7 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ctType == other.ctType && self.guidClock == other.guidClock && self.wszDescription == other.wszDescription
    }
}
impl ::core::cmp::Eq for DMUS_CLOCKINFO7 {}
impl ::core::fmt::Debug for DMUS_CLOCKINFO7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_CLOCKINFO7").field("dwSize", &self.dwSize).field("ctType", &self.ctType).field("guidClock", &self.guidClock).field("wszDescription", &self.wszDescription).finish()
    }
}
impl ::core::default::Default for DMUS_CLOCKINFO8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_CLOCKINFO8 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ctType == other.ctType && self.guidClock == other.guidClock && self.wszDescription == other.wszDescription && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DMUS_CLOCKINFO8 {}
impl ::core::fmt::Debug for DMUS_CLOCKINFO8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_CLOCKINFO8").field("dwSize", &self.dwSize).field("ctType", &self.ctType).field("guidClock", &self.guidClock).field("wszDescription", &self.wszDescription).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for DMUS_CLOCKTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DMUS_CLOCKTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DMUS_CLOCKTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DMUS_COPYRIGHT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_COPYRIGHT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.byCopyright == other.byCopyright
    }
}
impl ::core::cmp::Eq for DMUS_COPYRIGHT {}
impl ::core::fmt::Debug for DMUS_COPYRIGHT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_COPYRIGHT").field("cbSize", &self.cbSize).field("byCopyright", &self.byCopyright).finish()
    }
}
impl ::core::default::Default for DMUS_DOWNLOADINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_DOWNLOADINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwDLType == other.dwDLType && self.dwDLId == other.dwDLId && self.dwNumOffsetTableEntries == other.dwNumOffsetTableEntries && self.cbSize == other.cbSize
    }
}
impl ::core::cmp::Eq for DMUS_DOWNLOADINFO {}
impl ::core::fmt::Debug for DMUS_DOWNLOADINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_DOWNLOADINFO").field("dwDLType", &self.dwDLType).field("dwDLId", &self.dwDLId).field("dwNumOffsetTableEntries", &self.dwNumOffsetTableEntries).field("cbSize", &self.cbSize).finish()
    }
}
impl ::core::default::Default for DMUS_EVENTHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DMUS_EXTENSIONCHUNK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_EXTENSIONCHUNK {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulNextExtCkIdx == other.ulNextExtCkIdx && self.ExtCkID == other.ExtCkID && self.byExtCk == other.byExtCk
    }
}
impl ::core::cmp::Eq for DMUS_EXTENSIONCHUNK {}
impl ::core::fmt::Debug for DMUS_EXTENSIONCHUNK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_EXTENSIONCHUNK").field("cbSize", &self.cbSize).field("ulNextExtCkIdx", &self.ulNextExtCkIdx).field("ExtCkID", &self.ExtCkID).field("byExtCk", &self.byExtCk).finish()
    }
}
impl ::core::default::Default for DMUS_INSTRUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_INSTRUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.ulPatch == other.ulPatch && self.ulFirstRegionIdx == other.ulFirstRegionIdx && self.ulGlobalArtIdx == other.ulGlobalArtIdx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx && self.ulCopyrightIdx == other.ulCopyrightIdx && self.ulFlags == other.ulFlags
    }
}
impl ::core::cmp::Eq for DMUS_INSTRUMENT {}
impl ::core::fmt::Debug for DMUS_INSTRUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_INSTRUMENT").field("ulPatch", &self.ulPatch).field("ulFirstRegionIdx", &self.ulFirstRegionIdx).field("ulGlobalArtIdx", &self.ulGlobalArtIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("ulCopyrightIdx", &self.ulCopyrightIdx).field("ulFlags", &self.ulFlags).finish()
    }
}
impl ::core::default::Default for DMUS_LFOPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_LFOPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.pcFrequency == other.pcFrequency && self.tcDelay == other.tcDelay && self.gcVolumeScale == other.gcVolumeScale && self.pcPitchScale == other.pcPitchScale && self.gcMWToVolume == other.gcMWToVolume && self.pcMWToPitch == other.pcMWToPitch
    }
}
impl ::core::cmp::Eq for DMUS_LFOPARAMS {}
impl ::core::fmt::Debug for DMUS_LFOPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_LFOPARAMS").field("pcFrequency", &self.pcFrequency).field("tcDelay", &self.tcDelay).field("gcVolumeScale", &self.gcVolumeScale).field("pcPitchScale", &self.pcPitchScale).field("gcMWToVolume", &self.gcMWToVolume).field("pcMWToPitch", &self.pcMWToPitch).finish()
    }
}
impl ::core::default::Default for DMUS_MSCPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_MSCPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.ptDefaultPan == other.ptDefaultPan
    }
}
impl ::core::cmp::Eq for DMUS_MSCPARAMS {}
impl ::core::fmt::Debug for DMUS_MSCPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_MSCPARAMS").field("ptDefaultPan", &self.ptDefaultPan).finish()
    }
}
impl ::core::default::Default for DMUS_NOTERANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_NOTERANGE {
    fn eq(&self, other: &Self) -> bool {
        self.dwLowNote == other.dwLowNote && self.dwHighNote == other.dwHighNote
    }
}
impl ::core::cmp::Eq for DMUS_NOTERANGE {}
impl ::core::fmt::Debug for DMUS_NOTERANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_NOTERANGE").field("dwLowNote", &self.dwLowNote).field("dwHighNote", &self.dwHighNote).finish()
    }
}
impl ::core::default::Default for DMUS_OFFSETTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_OFFSETTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.ulOffsetTable == other.ulOffsetTable
    }
}
impl ::core::cmp::Eq for DMUS_OFFSETTABLE {}
impl ::core::fmt::Debug for DMUS_OFFSETTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_OFFSETTABLE").field("ulOffsetTable", &self.ulOffsetTable).finish()
    }
}
impl ::core::default::Default for DMUS_PEGPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_PEGPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.tcAttack == other.tcAttack && self.tcDecay == other.tcDecay && self.ptSustain == other.ptSustain && self.tcRelease == other.tcRelease && self.tcVel2Attack == other.tcVel2Attack && self.tcKey2Decay == other.tcKey2Decay && self.pcRange == other.pcRange
    }
}
impl ::core::cmp::Eq for DMUS_PEGPARAMS {}
impl ::core::fmt::Debug for DMUS_PEGPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_PEGPARAMS").field("tcAttack", &self.tcAttack).field("tcDecay", &self.tcDecay).field("ptSustain", &self.ptSustain).field("tcRelease", &self.tcRelease).field("tcVel2Attack", &self.tcVel2Attack).field("tcKey2Decay", &self.tcKey2Decay).field("pcRange", &self.pcRange).finish()
    }
}
impl ::core::default::Default for DMUS_PORTCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_PORTCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidPort == other.guidPort && self.dwClass == other.dwClass && self.dwType == other.dwType && self.dwMemorySize == other.dwMemorySize && self.dwMaxChannelGroups == other.dwMaxChannelGroups && self.dwMaxVoices == other.dwMaxVoices && self.dwMaxAudioChannels == other.dwMaxAudioChannels && self.dwEffectFlags == other.dwEffectFlags && self.wszDescription == other.wszDescription
    }
}
impl ::core::cmp::Eq for DMUS_PORTCAPS {}
impl ::core::fmt::Debug for DMUS_PORTCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_PORTCAPS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidPort", &self.guidPort).field("dwClass", &self.dwClass).field("dwType", &self.dwType).field("dwMemorySize", &self.dwMemorySize).field("dwMaxChannelGroups", &self.dwMaxChannelGroups).field("dwMaxVoices", &self.dwMaxVoices).field("dwMaxAudioChannels", &self.dwMaxAudioChannels).field("dwEffectFlags", &self.dwEffectFlags).field("wszDescription", &self.wszDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DMUS_PORTPARAMS7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DMUS_PORTPARAMS7 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidParams == other.dwValidParams && self.dwVoices == other.dwVoices && self.dwChannelGroups == other.dwChannelGroups && self.dwAudioChannels == other.dwAudioChannels && self.dwSampleRate == other.dwSampleRate && self.dwEffectFlags == other.dwEffectFlags && self.fShare == other.fShare
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DMUS_PORTPARAMS7 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DMUS_PORTPARAMS7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_PORTPARAMS7").field("dwSize", &self.dwSize).field("dwValidParams", &self.dwValidParams).field("dwVoices", &self.dwVoices).field("dwChannelGroups", &self.dwChannelGroups).field("dwAudioChannels", &self.dwAudioChannels).field("dwSampleRate", &self.dwSampleRate).field("dwEffectFlags", &self.dwEffectFlags).field("fShare", &self.fShare).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DMUS_PORTPARAMS8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DMUS_PORTPARAMS8 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidParams == other.dwValidParams && self.dwVoices == other.dwVoices && self.dwChannelGroups == other.dwChannelGroups && self.dwAudioChannels == other.dwAudioChannels && self.dwSampleRate == other.dwSampleRate && self.dwEffectFlags == other.dwEffectFlags && self.fShare == other.fShare && self.dwFeatures == other.dwFeatures
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DMUS_PORTPARAMS8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DMUS_PORTPARAMS8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_PORTPARAMS8").field("dwSize", &self.dwSize).field("dwValidParams", &self.dwValidParams).field("dwVoices", &self.dwVoices).field("dwChannelGroups", &self.dwChannelGroups).field("dwAudioChannels", &self.dwAudioChannels).field("dwSampleRate", &self.dwSampleRate).field("dwEffectFlags", &self.dwEffectFlags).field("fShare", &self.fShare).field("dwFeatures", &self.dwFeatures).finish()
    }
}
impl ::core::default::Default for DMUS_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.RangeKey == other.RangeKey && self.RangeVelocity == other.RangeVelocity && self.fusOptions == other.fusOptions && self.usKeyGroup == other.usKeyGroup && self.ulRegionArtIdx == other.ulRegionArtIdx && self.ulNextRegionIdx == other.ulNextRegionIdx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx && self.WaveLink == other.WaveLink && self.WSMP == other.WSMP && self.WLOOP == other.WLOOP
    }
}
impl ::core::cmp::Eq for DMUS_REGION {}
impl ::core::fmt::Debug for DMUS_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_REGION").field("RangeKey", &self.RangeKey).field("RangeVelocity", &self.RangeVelocity).field("fusOptions", &self.fusOptions).field("usKeyGroup", &self.usKeyGroup).field("ulRegionArtIdx", &self.ulRegionArtIdx).field("ulNextRegionIdx", &self.ulNextRegionIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("WaveLink", &self.WaveLink).field("WSMP", &self.WSMP).field("WLOOP", &self.WLOOP).finish()
    }
}
impl ::core::default::Default for DMUS_SYNTHSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_SYNTHSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidStats == other.dwValidStats && self.dwVoices == other.dwVoices && self.dwTotalCPU == other.dwTotalCPU && self.dwCPUPerVoice == other.dwCPUPerVoice && self.dwLostNotes == other.dwLostNotes && self.dwFreeMemory == other.dwFreeMemory && self.lPeakVolume == other.lPeakVolume
    }
}
impl ::core::cmp::Eq for DMUS_SYNTHSTATS {}
impl ::core::fmt::Debug for DMUS_SYNTHSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_SYNTHSTATS").field("dwSize", &self.dwSize).field("dwValidStats", &self.dwValidStats).field("dwVoices", &self.dwVoices).field("dwTotalCPU", &self.dwTotalCPU).field("dwCPUPerVoice", &self.dwCPUPerVoice).field("dwLostNotes", &self.dwLostNotes).field("dwFreeMemory", &self.dwFreeMemory).field("lPeakVolume", &self.lPeakVolume).finish()
    }
}
impl ::core::default::Default for DMUS_SYNTHSTATS8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_SYNTHSTATS8 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidStats == other.dwValidStats && self.dwVoices == other.dwVoices && self.dwTotalCPU == other.dwTotalCPU && self.dwCPUPerVoice == other.dwCPUPerVoice && self.dwLostNotes == other.dwLostNotes && self.dwFreeMemory == other.dwFreeMemory && self.lPeakVolume == other.lPeakVolume && self.dwSynthMemUse == other.dwSynthMemUse
    }
}
impl ::core::cmp::Eq for DMUS_SYNTHSTATS8 {}
impl ::core::fmt::Debug for DMUS_SYNTHSTATS8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_SYNTHSTATS8").field("dwSize", &self.dwSize).field("dwValidStats", &self.dwValidStats).field("dwVoices", &self.dwVoices).field("dwTotalCPU", &self.dwTotalCPU).field("dwCPUPerVoice", &self.dwCPUPerVoice).field("dwLostNotes", &self.dwLostNotes).field("dwFreeMemory", &self.dwFreeMemory).field("lPeakVolume", &self.lPeakVolume).field("dwSynthMemUse", &self.dwSynthMemUse).finish()
    }
}
impl ::core::default::Default for DMUS_VEGPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_VEGPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.tcAttack == other.tcAttack && self.tcDecay == other.tcDecay && self.ptSustain == other.ptSustain && self.tcRelease == other.tcRelease && self.tcVel2Attack == other.tcVel2Attack && self.tcKey2Decay == other.tcKey2Decay
    }
}
impl ::core::cmp::Eq for DMUS_VEGPARAMS {}
impl ::core::fmt::Debug for DMUS_VEGPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_VEGPARAMS").field("tcAttack", &self.tcAttack).field("tcDecay", &self.tcDecay).field("ptSustain", &self.ptSustain).field("tcRelease", &self.tcRelease).field("tcVel2Attack", &self.tcVel2Attack).field("tcKey2Decay", &self.tcKey2Decay).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DMUS_VOICE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DMUS_VOICE_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.bExists == other.bExists && self.spPosition == other.spPosition
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DMUS_VOICE_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DMUS_VOICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_VOICE_STATE").field("bExists", &self.bExists).field("spPosition", &self.spPosition).finish()
    }
}
impl ::core::default::Default for DMUS_WAVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DMUS_WAVEARTDL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_WAVEARTDL {
    fn eq(&self, other: &Self) -> bool {
        self.ulDownloadIdIdx == other.ulDownloadIdIdx && self.ulBus == other.ulBus && self.ulBuffers == other.ulBuffers && self.ulMasterDLId == other.ulMasterDLId && self.usOptions == other.usOptions
    }
}
impl ::core::cmp::Eq for DMUS_WAVEARTDL {}
impl ::core::fmt::Debug for DMUS_WAVEARTDL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_WAVEARTDL").field("ulDownloadIdIdx", &self.ulDownloadIdIdx).field("ulBus", &self.ulBus).field("ulBuffers", &self.ulBuffers).field("ulMasterDLId", &self.ulMasterDLId).field("usOptions", &self.usOptions).finish()
    }
}
impl ::core::default::Default for DMUS_WAVEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_WAVEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.byData == other.byData
    }
}
impl ::core::cmp::Eq for DMUS_WAVEDATA {}
impl ::core::fmt::Debug for DMUS_WAVEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_WAVEDATA").field("cbSize", &self.cbSize).field("byData", &self.byData).finish()
    }
}
impl ::core::default::Default for DMUS_WAVEDL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_WAVEDL {
    fn eq(&self, other: &Self) -> bool {
        self.cbWaveData == other.cbWaveData
    }
}
impl ::core::cmp::Eq for DMUS_WAVEDL {}
impl ::core::fmt::Debug for DMUS_WAVEDL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_WAVEDL").field("cbWaveData", &self.cbWaveData).finish()
    }
}
impl ::core::default::Default for DMUS_WAVES_REVERB_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_WAVES_REVERB_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.fInGain == other.fInGain && self.fReverbMix == other.fReverbMix && self.fReverbTime == other.fReverbTime && self.fHighFreqRTRatio == other.fHighFreqRTRatio
    }
}
impl ::core::cmp::Eq for DMUS_WAVES_REVERB_PARAMS {}
impl ::core::fmt::Debug for DMUS_WAVES_REVERB_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_WAVES_REVERB_PARAMS").field("fInGain", &self.fInGain).field("fReverbMix", &self.fReverbMix).field("fReverbTime", &self.fReverbTime).field("fHighFreqRTRatio", &self.fHighFreqRTRatio).finish()
    }
}
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSPROPERTY_DIRECTSOUNDDEVICE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceId == other.DeviceId && self.DescriptionA == other.DescriptionA && self.DescriptionW == other.DescriptionW && self.ModuleA == other.ModuleA && self.ModuleW == other.ModuleW && self.Type == other.Type && self.DataFlow == other.DataFlow && self.WaveDeviceId == other.WaveDeviceId && self.Devnode == other.Devnode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA").field("DeviceId", &self.DeviceId).field("DescriptionA", &self.DescriptionA).field("DescriptionW", &self.DescriptionW).field("ModuleA", &self.ModuleA).field("ModuleW", &self.ModuleW).field("Type", &self.Type).field("DataFlow", &self.DataFlow).field("WaveDeviceId", &self.WaveDeviceId).field("Devnode", &self.Devnode).finish()
    }
}
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId && self.Description == other.Description && self.Module == other.Module && self.Interface == other.Interface && self.WaveDeviceId == other.WaveDeviceId
    }
}
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {}
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA").field("Type", &self.Type).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).field("Description", &self.Description).field("Module", &self.Module).field("Interface", &self.Interface).field("WaveDeviceId", &self.WaveDeviceId).finish()
    }
}
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId && self.Description == other.Description && self.Module == other.Module && self.Interface == other.Interface && self.WaveDeviceId == other.WaveDeviceId
    }
}
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {}
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA").field("Type", &self.Type).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).field("Description", &self.Description).field("Module", &self.Module).field("Interface", &self.Interface).field("WaveDeviceId", &self.WaveDeviceId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId
    }
}
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {}
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA").field("DeviceName", &self.DeviceName).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).finish()
    }
}
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId
    }
}
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {}
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA").field("DeviceName", &self.DeviceName).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).finish()
    }
}
impl ::core::default::Default for DVAudInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DVAudInfo {
    fn eq(&self, other: &Self) -> bool {
        self.bAudStyle == other.bAudStyle && self.bAudQu == other.bAudQu && self.bNumAudPin == other.bNumAudPin && self.wAvgSamplesPerPinPerFrm == other.wAvgSamplesPerPinPerFrm && self.wBlkMode == other.wBlkMode && self.wDIFMode == other.wDIFMode && self.wBlkDiv == other.wBlkDiv
    }
}
impl ::core::cmp::Eq for DVAudInfo {}
impl ::core::fmt::Debug for DVAudInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DVAudInfo").field("bAudStyle", &self.bAudStyle).field("bAudQu", &self.bAudQu).field("bNumAudPin", &self.bNumAudPin).field("wAvgSamplesPerPinPerFrm", &self.wAvgSamplesPerPinPerFrm).field("wBlkMode", &self.wBlkMode).field("wDIFMode", &self.wDIFMode).field("wBlkDiv", &self.wBlkDiv).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectMusic {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusic {}
impl ::core::fmt::Debug for IDirectMusic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusic").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectMusic8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusic8 {}
impl ::core::fmt::Debug for IDirectMusic8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusic8").field(&self.0).finish()
    }
}
impl IDirectMusic8 {
    pub unsafe fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumPort)(::windows::core::Vtable::as_raw(self), dwindex, pportcaps).ok()
    }
    pub unsafe fn CreateMusicBuffer<P0>(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::core::option::Option<IDirectMusicBuffer>, punkouter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateMusicBuffer)(::windows::core::Vtable::as_raw(self), pbufferdesc, ::core::mem::transmute(ppbuffer), punkouter.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePort<P0>(&self, rclsidport: *const ::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::core::option::Option<IDirectMusicPort>, punkouter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreatePort)(::windows::core::Vtable::as_raw(self), rclsidport, pportparams, ::core::mem::transmute(ppport), punkouter.into().abi()).ok()
    }
    pub unsafe fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumMasterClock)(::windows::core::Vtable::as_raw(self), dwindex, lpclockinfo).ok()
    }
    pub unsafe fn GetMasterClock(&self, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut ::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMasterClock)(::windows::core::Vtable::as_raw(self), pguidclock, ::core::mem::transmute(ppreferenceclock)).ok()
    }
    pub unsafe fn SetMasterClock(&self, rguidclock: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMasterClock)(::windows::core::Vtable::as_raw(self), rguidclock).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Activate)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn GetDefaultPort(&self, pguidport: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDefaultPort)(::windows::core::Vtable::as_raw(self), pguidport).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio_DirectSound\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
    pub unsafe fn SetDirectSound<P0, P1>(&self, pdirectsound: P0, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectSound::IDirectSound>>,
        P1: ::std::convert::Into<super::super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDirectSound)(::windows::core::Vtable::as_raw(self), pdirectsound.into().abi(), hwnd.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IDirectMusicBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicBuffer {}
impl ::core::fmt::Debug for IDirectMusicBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectMusicCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicCollection {}
impl ::core::fmt::Debug for IDirectMusicCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectMusicDownload {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicDownload {}
impl ::core::fmt::Debug for IDirectMusicDownload {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicDownload").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectMusicDownloadedInstrument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicDownloadedInstrument {}
impl ::core::fmt::Debug for IDirectMusicDownloadedInstrument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicDownloadedInstrument").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectMusicInstrument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicInstrument {}
impl ::core::fmt::Debug for IDirectMusicInstrument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicInstrument").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectMusicPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicPort {}
impl ::core::fmt::Debug for IDirectMusicPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicPort").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectMusicPortDownload {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicPortDownload {}
impl ::core::fmt::Debug for IDirectMusicPortDownload {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicPortDownload").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectMusicSynth {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicSynth {}
impl ::core::fmt::Debug for IDirectMusicSynth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicSynth").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectMusicSynth8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicSynth8 {}
impl ::core::fmt::Debug for IDirectMusicSynth8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicSynth8").field(&self.0).finish()
    }
}
impl IDirectMusicSynth8 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Open)(::windows::core::Vtable::as_raw(self), pportparams).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetNumChannelGroups(&self, dwgroups: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNumChannelGroups)(::windows::core::Vtable::as_raw(self), dwgroups).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Download)(::windows::core::Vtable::as_raw(self), phdownload, pvdata, pbfree).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unload<P0, P1>(&self, hdownload: P0, lpfreehandle: isize, huserdata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
        P1: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.Unload)(::windows::core::Vtable::as_raw(self), hdownload.into(), lpfreehandle, huserdata.into()).ok()
    }
    pub unsafe fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PlayBuffer)(::windows::core::Vtable::as_raw(self), rt, pbbuffer, cbbuffer).ok()
    }
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRunningStats)(::windows::core::Vtable::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPortCaps)(::windows::core::Vtable::as_raw(self), pcaps).ok()
    }
    pub unsafe fn SetMasterClock<P0>(&self, pclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::IReferenceClock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMasterClock)(::windows::core::Vtable::as_raw(self), pclock.into().abi()).ok()
    }
    pub unsafe fn GetLatencyClock(&self) -> ::windows::core::Result<super::super::IReferenceClock> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLatencyClock)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Activate)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn SetSynthSink<P0>(&self, psynthsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirectMusicSynthSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSynthSink)(::windows::core::Vtable::as_raw(self), psynthsink.into().abi()).ok()
    }
    pub unsafe fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Render)(::windows::core::Vtable::as_raw(self), pbuffer, dwlength, llposition).ok()
    }
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetChannelPriority)(::windows::core::Vtable::as_raw(self), dwchannelgroup, dwchannel, dwpriority).ok()
    }
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetChannelPriority)(::windows::core::Vtable::as_raw(self), dwchannelgroup, dwchannel, pdwpriority).ok()
    }
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFormat)(::windows::core::Vtable::as_raw(self), pwaveformatex, pdwwaveformatexsize).ok()
    }
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAppend)(::windows::core::Vtable::as_raw(self), pdwappend).ok()
    }
}
impl ::core::cmp::PartialEq for IDirectMusicSynthSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicSynthSink {}
impl ::core::fmt::Debug for IDirectMusicSynthSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicSynthSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectMusicThru {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicThru {}
impl ::core::fmt::Debug for IDirectMusicThru {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicThru").field(&self.0).finish()
    }
}
impl ::core::default::Default for INSTHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INSTHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cRegions == other.cRegions && self.Locale == other.Locale
    }
}
impl ::core::cmp::Eq for INSTHEADER {}
impl ::core::fmt::Debug for INSTHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTHEADER").field("cRegions", &self.cRegions).field("Locale", &self.Locale).finish()
    }
}
impl ::core::default::Default for MDEVICECAPSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIDILOCALE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDILOCALE {
    fn eq(&self, other: &Self) -> bool {
        self.ulBank == other.ulBank && self.ulInstrument == other.ulInstrument
    }
}
impl ::core::cmp::Eq for MIDILOCALE {}
impl ::core::fmt::Debug for MIDILOCALE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDILOCALE").field("ulBank", &self.ulBank).field("ulInstrument", &self.ulInstrument).finish()
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::core::default::Default for MIDIOPENDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for POOLCUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POOLCUE {
    fn eq(&self, other: &Self) -> bool {
        self.ulOffset == other.ulOffset
    }
}
impl ::core::cmp::Eq for POOLCUE {}
impl ::core::fmt::Debug for POOLCUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POOLCUE").field("ulOffset", &self.ulOffset).finish()
    }
}
impl ::core::default::Default for POOLTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POOLTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cCues == other.cCues
    }
}
impl ::core::cmp::Eq for POOLTABLE {}
impl ::core::fmt::Debug for POOLTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POOLTABLE").field("cbSize", &self.cbSize).field("cCues", &self.cCues).finish()
    }
}
impl ::core::default::Default for RGNHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RGNHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.RangeKey == other.RangeKey && self.RangeVelocity == other.RangeVelocity && self.fusOptions == other.fusOptions && self.usKeyGroup == other.usKeyGroup
    }
}
impl ::core::cmp::Eq for RGNHEADER {}
impl ::core::fmt::Debug for RGNHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGNHEADER").field("RangeKey", &self.RangeKey).field("RangeVelocity", &self.RangeVelocity).field("fusOptions", &self.fusOptions).field("usKeyGroup", &self.usKeyGroup).finish()
    }
}
impl ::core::default::Default for RGNRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RGNRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.usLow == other.usLow && self.usHigh == other.usHigh
    }
}
impl ::core::cmp::Eq for RGNRANGE {}
impl ::core::fmt::Debug for RGNRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGNRANGE").field("usLow", &self.usLow).field("usHigh", &self.usHigh).finish()
    }
}
impl ::core::default::Default for WAVELINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WAVELINK {
    fn eq(&self, other: &Self) -> bool {
        self.fusOptions == other.fusOptions && self.usPhaseGroup == other.usPhaseGroup && self.ulChannel == other.ulChannel && self.ulTableIndex == other.ulTableIndex
    }
}
impl ::core::cmp::Eq for WAVELINK {}
impl ::core::fmt::Debug for WAVELINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVELINK").field("fusOptions", &self.fusOptions).field("usPhaseGroup", &self.usPhaseGroup).field("ulChannel", &self.ulChannel).field("ulTableIndex", &self.ulTableIndex).finish()
    }
}
impl ::core::default::Default for WLOOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLOOP {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulType == other.ulType && self.ulStart == other.ulStart && self.ulLength == other.ulLength
    }
}
impl ::core::cmp::Eq for WLOOP {}
impl ::core::fmt::Debug for WLOOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLOOP").field("cbSize", &self.cbSize).field("ulType", &self.ulType).field("ulStart", &self.ulStart).field("ulLength", &self.ulLength).finish()
    }
}
impl ::core::default::Default for WSMPL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMPL {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.usUnityNote == other.usUnityNote && self.sFineTune == other.sFineTune && self.lAttenuation == other.lAttenuation && self.fulOptions == other.fulOptions && self.cSampleLoops == other.cSampleLoops
    }
}
impl ::core::cmp::Eq for WSMPL {}
impl ::core::fmt::Debug for WSMPL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMPL").field("cbSize", &self.cbSize).field("usUnityNote", &self.usUnityNote).field("sFineTune", &self.sFineTune).field("lAttenuation", &self.lAttenuation).field("fulOptions", &self.fulOptions).field("cSampleLoops", &self.cSampleLoops).finish()
    }
}

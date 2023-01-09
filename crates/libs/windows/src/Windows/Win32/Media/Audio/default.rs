#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for ACMDRIVERDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for ACMDRIVERDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ACMDRVFORMATSUGGEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ACMDRVOPENDESCA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ACMDRVOPENDESCW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ACMDRVSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ACMDRVSTREAMINSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ACMDRVSTREAMSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFILTERCHOOSEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFILTERCHOOSEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFILTERDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ACMFILTERDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFILTERTAGDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ACMFILTERTAGDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFORMATCHOOSEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFORMATCHOOSEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFORMATDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFORMATTAGDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ACMFORMATTAGDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for AMBISONICS_CHANNEL_ORDERING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AMBISONICS_CHANNEL_ORDERING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AMBISONICS_CHANNEL_ORDERING").field(&self.0).finish()
    }
}
impl ::core::default::Default for AMBISONICS_NORMALIZATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AMBISONICS_NORMALIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AMBISONICS_NORMALIZATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for AMBISONICS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AMBISONICS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.u32Size == other.u32Size && self.u32Version == other.u32Version && self.u32Type == other.u32Type && self.u32ChannelOrdering == other.u32ChannelOrdering && self.u32Normalization == other.u32Normalization && self.u32Order == other.u32Order && self.u32NumChannels == other.u32NumChannels && self.pu32ChannelMap == other.pu32ChannelMap
    }
}
impl ::core::cmp::Eq for AMBISONICS_PARAMS {}
impl ::core::fmt::Debug for AMBISONICS_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AMBISONICS_PARAMS").field("u32Size", &self.u32Size).field("u32Version", &self.u32Version).field("u32Type", &self.u32Type).field("u32ChannelOrdering", &self.u32ChannelOrdering).field("u32Normalization", &self.u32Normalization).field("u32Order", &self.u32Order).field("u32NumChannels", &self.u32NumChannels).field("pu32ChannelMap", &self.pu32ChannelMap).finish()
    }
}
impl ::core::default::Default for AMBISONICS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AMBISONICS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AMBISONICS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for AUDCLNT_SHAREMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDCLNT_SHAREMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDCLNT_SHAREMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for AUDCLNT_STREAMOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDCLNT_STREAMOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDCLNT_STREAMOPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AUDCLNT_STREAMOPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AUDCLNT_STREAMOPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for AUDIOCLIENT_ACTIVATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDIOCLIENT_ACTIVATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIOCLIENT_ACTIVATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.TargetProcessId == other.TargetProcessId && self.ProcessLoopbackMode == other.ProcessLoopbackMode
    }
}
impl ::core::cmp::Eq for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {}
impl ::core::fmt::Debug for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS").field("TargetProcessId", &self.TargetProcessId).field("ProcessLoopbackMode", &self.ProcessLoopbackMode).finish()
    }
}
impl ::core::default::Default for AUDIO_DUCKING_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDIO_DUCKING_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_DUCKING_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AUDIO_DUCKING_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AUDIO_DUCKING_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_EFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_EFFECT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.canSetState == other.canSetState && self.state == other.state
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_EFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_EFFECT").field("id", &self.id).field("canSetState", &self.canSetState).field("state", &self.state).finish()
    }
}
impl ::core::default::Default for AUDIO_EFFECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDIO_EFFECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_EFFECT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for AUDIO_STREAM_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDIO_STREAM_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_STREAM_CATEGORY").field(&self.0).finish()
    }
}
impl ::core::default::Default for AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.guidEventContext == other.guidEventContext && self.bMuted == other.bMuted && self.fMasterVolume == other.fMasterVolume && self.nChannels == other.nChannels && self.afChannelVolumes == other.afChannelVolumes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_VOLUME_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_VOLUME_NOTIFICATION_DATA").field("guidEventContext", &self.guidEventContext).field("bMuted", &self.bMuted).field("fMasterVolume", &self.fMasterVolume).field("nChannels", &self.nChannels).field("afChannelVolumes", &self.afChannelVolumes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUXCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for AUXCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUXCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for AUXCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for AudioClient3ActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AudioClient3ActivationParams {
    fn eq(&self, other: &Self) -> bool {
        self.tracingContextId == other.tracingContextId
    }
}
impl ::core::cmp::Eq for AudioClient3ActivationParams {}
impl ::core::fmt::Debug for AudioClient3ActivationParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioClient3ActivationParams").field("tracingContextId", &self.tracingContextId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AudioClientProperties {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AudioClientProperties {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.bIsOffload == other.bIsOffload && self.eCategory == other.eCategory && self.Options == other.Options
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AudioClientProperties {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AudioClientProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioClientProperties").field("cbSize", &self.cbSize).field("bIsOffload", &self.bIsOffload).field("eCategory", &self.eCategory).field("Options", &self.Options).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AudioExtensionParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AudioExtensionParams {
    fn eq(&self, other: &Self) -> bool {
        self.AddPageParam == other.AddPageParam && self.pEndpoint == other.pEndpoint && self.pPnpInterface == other.pPnpInterface && self.pPnpDevnode == other.pPnpDevnode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AudioExtensionParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AudioExtensionParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioExtensionParams").field("AddPageParam", &self.AddPageParam).field("pEndpoint", &self.pEndpoint).field("pPnpInterface", &self.pPnpInterface).field("pPnpDevnode", &self.pPnpDevnode).finish()
    }
}
impl ::core::default::Default for AudioObjectType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AudioObjectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioObjectType").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AudioObjectType {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AudioObjectType {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AudioObjectType {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AudioObjectType {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AudioObjectType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for AudioSessionDisconnectReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AudioSessionDisconnectReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioSessionDisconnectReason").field(&self.0).finish()
    }
}
impl ::core::default::Default for AudioSessionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AudioSessionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioSessionState").field(&self.0).finish()
    }
}
impl ::core::default::Default for AudioStateMonitorSoundLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AudioStateMonitorSoundLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioStateMonitorSoundLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for ConnectorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ConnectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectorType").field(&self.0).finish()
    }
}
impl ::core::default::Default for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbDirectXAudioActivationParams == other.cbDirectXAudioActivationParams && self.guidAudioSession == other.guidAudioSession && self.dwAudioStreamFlags == other.dwAudioStreamFlags
    }
}
impl ::core::cmp::Eq for DIRECTX_AUDIO_ACTIVATION_PARAMS {}
impl ::core::fmt::Debug for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIRECTX_AUDIO_ACTIVATION_PARAMS").field("cbDirectXAudioActivationParams", &self.cbDirectXAudioActivationParams).field("guidAudioSession", &self.guidAudioSession).field("dwAudioStreamFlags", &self.dwAudioStreamFlags).finish()
    }
}
impl ::core::default::Default for DataFlow {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DataFlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataFlow").field(&self.0).finish()
    }
}
impl ::core::default::Default for ECHOWAVEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for EDataFlow {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDataFlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDataFlow").field(&self.0).finish()
    }
}
impl ::core::default::Default for ERole {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ERole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERole").field(&self.0).finish()
    }
}
impl ::core::default::Default for EndpointFormFactor {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EndpointFormFactor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EndpointFormFactor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActivateAudioInterfaceAsyncOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivateAudioInterfaceAsyncOperation {}
impl ::core::fmt::Debug for IActivateAudioInterfaceAsyncOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivateAudioInterfaceAsyncOperation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActivateAudioInterfaceCompletionHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivateAudioInterfaceCompletionHandler {}
impl ::core::fmt::Debug for IActivateAudioInterfaceCompletionHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivateAudioInterfaceCompletionHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioAmbisonicsControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioAmbisonicsControl {}
impl ::core::fmt::Debug for IAudioAmbisonicsControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioAmbisonicsControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioAutoGainControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioAutoGainControl {}
impl ::core::fmt::Debug for IAudioAutoGainControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioAutoGainControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioBass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioBass {}
impl ::core::fmt::Debug for IAudioBass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioBass").field(&self.0).finish()
    }
}
impl IAudioBass {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetChannelCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLevelRange)(::windows::core::Vtable::as_raw(self), nchannel, pfminleveldb, pfmaxleveldb, pfstepping).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLevel)(::windows::core::Vtable::as_raw(self), nchannel, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLevel)(::windows::core::Vtable::as_raw(self), nchannel, fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLevelUniform)(::windows::core::Vtable::as_raw(self), fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLevelAllChannels)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(alevelsdb.as_ptr()), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
impl ::core::cmp::PartialEq for IAudioCaptureClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioCaptureClient {}
impl ::core::fmt::Debug for IAudioCaptureClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioCaptureClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioChannelConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioChannelConfig {}
impl ::core::fmt::Debug for IAudioChannelConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioChannelConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClient {}
impl ::core::fmt::Debug for IAudioClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioClient2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClient2 {}
impl ::core::fmt::Debug for IAudioClient2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClient2").field(&self.0).finish()
    }
}
impl IAudioClient2 {
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), sharemode, streamflags, hnsbufferduration, hnsperiodicity, pformat, ::core::mem::transmute(audiosessionguid.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetBufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBufferSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStreamLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamLatency)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentPadding)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: ::core::option::Option<*mut *mut WAVEFORMATEX>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.IsFormatSupported)(::windows::core::Vtable::as_raw(self), sharemode, pformat, ::core::mem::transmute(ppclosestmatch.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GetMixFormat(&self) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMixFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: ::core::option::Option<*mut i64>, phnsminimumdeviceperiod: ::core::option::Option<*mut i64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDevicePeriod)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(phnsdefaultdeviceperiod.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(phnsminimumdeviceperiod.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<P0>(&self, eventhandle: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEventHandle)(::windows::core::Vtable::as_raw(self), eventhandle.into()).ok()
    }
    pub unsafe fn GetService<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetService)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IAudioClient3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClient3 {}
impl ::core::fmt::Debug for IAudioClient3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClient3").field(&self.0).finish()
    }
}
impl IAudioClient3 {
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), sharemode, streamflags, hnsbufferduration, hnsperiodicity, pformat, ::core::mem::transmute(audiosessionguid.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetBufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBufferSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStreamLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStreamLatency)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCurrentPadding)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: ::core::option::Option<*mut *mut WAVEFORMATEX>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.IsFormatSupported)(::windows::core::Vtable::as_raw(self), sharemode, pformat, ::core::mem::transmute(ppclosestmatch.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GetMixFormat(&self) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMixFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: ::core::option::Option<*mut i64>, phnsminimumdeviceperiod: ::core::option::Option<*mut i64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevicePeriod)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(phnsdefaultdeviceperiod.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(phnsminimumdeviceperiod.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<P0>(&self, eventhandle: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetEventHandle)(::windows::core::Vtable::as_raw(self), eventhandle.into()).ok()
    }
    pub unsafe fn GetService<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetService)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsOffloadCapable)(::windows::core::Vtable::as_raw(self), category, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetClientProperties)(::windows::core::Vtable::as_raw(self), pproperties).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBufferSizeLimits<P0>(&self, pformat: *const WAVEFORMATEX, beventdriven: P0, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetBufferSizeLimits)(::windows::core::Vtable::as_raw(self), pformat, beventdriven.into(), phnsminbufferduration, phnsmaxbufferduration).ok()
    }
}
impl ::core::cmp::PartialEq for IAudioClientDuckingControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClientDuckingControl {}
impl ::core::fmt::Debug for IAudioClientDuckingControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClientDuckingControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClock {}
impl ::core::fmt::Debug for IAudioClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClock").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioClock2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClock2 {}
impl ::core::fmt::Debug for IAudioClock2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClock2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioClockAdjustment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClockAdjustment {}
impl ::core::fmt::Debug for IAudioClockAdjustment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClockAdjustment").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioEffectsChangedNotificationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEffectsChangedNotificationClient {}
impl ::core::fmt::Debug for IAudioEffectsChangedNotificationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEffectsChangedNotificationClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioEffectsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEffectsManager {}
impl ::core::fmt::Debug for IAudioEffectsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEffectsManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioFormatEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioFormatEnumerator {}
impl ::core::fmt::Debug for IAudioFormatEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioFormatEnumerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioInputSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioInputSelector {}
impl ::core::fmt::Debug for IAudioInputSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioInputSelector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioLoudness {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioLoudness {}
impl ::core::fmt::Debug for IAudioLoudness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioLoudness").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioMidrange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMidrange {}
impl ::core::fmt::Debug for IAudioMidrange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMidrange").field(&self.0).finish()
    }
}
impl IAudioMidrange {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetChannelCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLevelRange)(::windows::core::Vtable::as_raw(self), nchannel, pfminleveldb, pfmaxleveldb, pfstepping).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLevel)(::windows::core::Vtable::as_raw(self), nchannel, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLevel)(::windows::core::Vtable::as_raw(self), nchannel, fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLevelUniform)(::windows::core::Vtable::as_raw(self), fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLevelAllChannels)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(alevelsdb.as_ptr()), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
impl ::core::cmp::PartialEq for IAudioMute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMute {}
impl ::core::fmt::Debug for IAudioMute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMute").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioOutputSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioOutputSelector {}
impl ::core::fmt::Debug for IAudioOutputSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioOutputSelector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioPeakMeter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioPeakMeter {}
impl ::core::fmt::Debug for IAudioPeakMeter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioPeakMeter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioRenderClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioRenderClient {}
impl ::core::fmt::Debug for IAudioRenderClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioRenderClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioSessionControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionControl {}
impl ::core::fmt::Debug for IAudioSessionControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioSessionControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionControl2 {}
impl ::core::fmt::Debug for IAudioSessionControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionControl2").field(&self.0).finish()
    }
}
impl IAudioSessionControl2 {
    pub unsafe fn GetState(&self) -> ::windows::core::Result<AudioSessionState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, value: P0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDisplayName)(::windows::core::Vtable::as_raw(self), value.into().abi(), eventcontext).ok()
    }
    pub unsafe fn GetIconPath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetIconPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetIconPath<P0>(&self, value: P0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetIconPath)(::windows::core::Vtable::as_raw(self), value.into().abi(), eventcontext).ok()
    }
    pub unsafe fn GetGroupingParam(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGroupingParam)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGroupingParam(&self, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGroupingParam)(::windows::core::Vtable::as_raw(self), r#override, eventcontext).ok()
    }
    pub unsafe fn RegisterAudioSessionNotification<P0>(&self, newnotifications: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAudioSessionEvents>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RegisterAudioSessionNotification)(::windows::core::Vtable::as_raw(self), newnotifications.into().abi()).ok()
    }
    pub unsafe fn UnregisterAudioSessionNotification<P0>(&self, newnotifications: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAudioSessionEvents>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UnregisterAudioSessionNotification)(::windows::core::Vtable::as_raw(self), newnotifications.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IAudioSessionEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionEnumerator {}
impl ::core::fmt::Debug for IAudioSessionEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionEnumerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioSessionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionEvents {}
impl ::core::fmt::Debug for IAudioSessionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioSessionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionManager {}
impl ::core::fmt::Debug for IAudioSessionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioSessionManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionManager2 {}
impl ::core::fmt::Debug for IAudioSessionManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionManager2").field(&self.0).finish()
    }
}
impl IAudioSessionManager2 {
    pub unsafe fn GetAudioSessionControl(&self, audiosessionguid: ::core::option::Option<*const ::windows::core::GUID>, streamflags: u32) -> ::windows::core::Result<IAudioSessionControl> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAudioSessionControl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(audiosessionguid.unwrap_or(::std::ptr::null())), streamflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSimpleAudioVolume(&self, audiosessionguid: ::core::option::Option<*const ::windows::core::GUID>, streamflags: u32) -> ::windows::core::Result<ISimpleAudioVolume> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSimpleAudioVolume)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(audiosessionguid.unwrap_or(::std::ptr::null())), streamflags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IAudioSessionNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionNotification {}
impl ::core::fmt::Debug for IAudioSessionNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioStateMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioStateMonitor {}
impl ::core::fmt::Debug for IAudioStateMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioStateMonitor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioStreamVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioStreamVolume {}
impl ::core::fmt::Debug for IAudioStreamVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioStreamVolume").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffectsPropertyChangeNotificationClient {}
impl ::core::fmt::Debug for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffectsPropertyChangeNotificationClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioSystemEffectsPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffectsPropertyStore {}
impl ::core::fmt::Debug for IAudioSystemEffectsPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffectsPropertyStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioTreble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioTreble {}
impl ::core::fmt::Debug for IAudioTreble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioTreble").field(&self.0).finish()
    }
}
impl IAudioTreble {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetChannelCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLevelRange)(::windows::core::Vtable::as_raw(self), nchannel, pfminleveldb, pfmaxleveldb, pfstepping).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLevel)(::windows::core::Vtable::as_raw(self), nchannel, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLevel)(::windows::core::Vtable::as_raw(self), nchannel, fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLevelUniform)(::windows::core::Vtable::as_raw(self), fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLevelAllChannels)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(alevelsdb.as_ptr()), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
impl ::core::cmp::PartialEq for IAudioVolumeDuckNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioVolumeDuckNotification {}
impl ::core::fmt::Debug for IAudioVolumeDuckNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioVolumeDuckNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioVolumeLevel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioVolumeLevel {}
impl ::core::fmt::Debug for IAudioVolumeLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioVolumeLevel").field(&self.0).finish()
    }
}
impl IAudioVolumeLevel {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetChannelCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLevelRange)(::windows::core::Vtable::as_raw(self), nchannel, pfminleveldb, pfmaxleveldb, pfstepping).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLevel)(::windows::core::Vtable::as_raw(self), nchannel, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLevel)(::windows::core::Vtable::as_raw(self), nchannel, fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLevelUniform)(::windows::core::Vtable::as_raw(self), fleveldb, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLevelAllChannels)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(alevelsdb.as_ptr()), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext.unwrap_or(::std::ptr::null()))).ok()
    }
}
impl ::core::cmp::PartialEq for IChannelAudioVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChannelAudioVolume {}
impl ::core::fmt::Debug for IChannelAudioVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChannelAudioVolume").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnector {}
impl ::core::fmt::Debug for IConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IControlChangeNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlChangeNotify {}
impl ::core::fmt::Debug for IControlChangeNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlChangeNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IControlInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlInterface {}
impl ::core::fmt::Debug for IControlInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlInterface").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDeviceSpecificProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceSpecificProperty {}
impl ::core::fmt::Debug for IDeviceSpecificProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceSpecificProperty").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDeviceTopology {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceTopology {}
impl ::core::fmt::Debug for IDeviceTopology {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceTopology").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMMDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMDevice {}
impl ::core::fmt::Debug for IMMDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMMDeviceActivator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMDeviceActivator {}
impl ::core::fmt::Debug for IMMDeviceActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDeviceActivator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMMDeviceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMDeviceCollection {}
impl ::core::fmt::Debug for IMMDeviceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDeviceCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMMDeviceEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMDeviceEnumerator {}
impl ::core::fmt::Debug for IMMDeviceEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDeviceEnumerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMMEndpoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMEndpoint {}
impl ::core::fmt::Debug for IMMEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMEndpoint").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMMNotificationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMNotificationClient {}
impl ::core::fmt::Debug for IMMNotificationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMNotificationClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMessageFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMessageFilter {}
impl ::core::fmt::Debug for IMessageFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessageFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPart {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPart {}
impl ::core::fmt::Debug for IPart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPart").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPartsList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartsList {}
impl ::core::fmt::Debug for IPartsList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartsList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPerChannelDbLevel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPerChannelDbLevel {}
impl ::core::fmt::Debug for IPerChannelDbLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerChannelDbLevel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISimpleAudioVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimpleAudioVolume {}
impl ::core::fmt::Debug for ISimpleAudioVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimpleAudioVolume").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioClient {}
impl ::core::fmt::Debug for ISpatialAudioClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioClient2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioClient2 {}
impl ::core::fmt::Debug for ISpatialAudioClient2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioClient2").field(&self.0).finish()
    }
}
impl ISpatialAudioClient2 {
    pub unsafe fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStaticObjectPosition)(::windows::core::Vtable::as_raw(self), r#type, x, y, z).ok()
    }
    pub unsafe fn GetNativeStaticObjectTypeMask(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNativeStaticObjectTypeMask)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMaxDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaxDynamicObjectCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSupportedAudioObjectFormatEnumerator(&self) -> ::windows::core::Result<IAudioFormatEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSupportedAudioObjectFormatEnumerator)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaxFrameCount)(::windows::core::Vtable::as_raw(self), objectformat, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsAudioObjectFormatSupported)(::windows::core::Vtable::as_raw(self), objectformat).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: ::core::option::Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsSpatialAudioStreamAvailable)(::windows::core::Vtable::as_raw(self), streamuuid, ::core::mem::transmute(auxiliaryinfo.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ActivateSpatialAudioStream<T>(&self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ActivateSpatialAudioStream)(::windows::core::Vtable::as_raw(self), activationparams, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataClient {}
impl ::core::fmt::Debug for ISpatialAudioMetadataClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataCopier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataCopier {}
impl ::core::fmt::Debug for ISpatialAudioMetadataCopier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataCopier").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataItems {}
impl ::core::fmt::Debug for ISpatialAudioMetadataItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataItems").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataItemsBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataItemsBuffer {}
impl ::core::fmt::Debug for ISpatialAudioMetadataItemsBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataItemsBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataReader {}
impl ::core::fmt::Debug for ISpatialAudioMetadataReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataWriter {}
impl ::core::fmt::Debug for ISpatialAudioMetadataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataWriter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObject {}
impl ::core::fmt::Debug for ISpatialAudioObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObject").field(&self.0).finish()
    }
}
impl ISpatialAudioObject {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBuffer)(::windows::core::Vtable::as_raw(self), buffer, bufferlength).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEndOfStream)(::windows::core::Vtable::as_raw(self), framecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsActive)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAudioObjectType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectBase {}
impl ::core::fmt::Debug for ISpatialAudioObjectBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectBase").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectForHrtf {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectForHrtf {}
impl ::core::fmt::Debug for ISpatialAudioObjectForHrtf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectForHrtf").field(&self.0).finish()
    }
}
impl ISpatialAudioObjectForHrtf {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBuffer)(::windows::core::Vtable::as_raw(self), buffer, bufferlength).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEndOfStream)(::windows::core::Vtable::as_raw(self), framecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsActive)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAudioObjectType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectForMetadataCommands {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectForMetadataCommands {}
impl ::core::fmt::Debug for ISpatialAudioObjectForMetadataCommands {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectForMetadataCommands").field(&self.0).finish()
    }
}
impl ISpatialAudioObjectForMetadataCommands {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBuffer)(::windows::core::Vtable::as_raw(self), buffer, bufferlength).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEndOfStream)(::windows::core::Vtable::as_raw(self), framecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsActive)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAudioObjectType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectForMetadataItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectForMetadataItems {}
impl ::core::fmt::Debug for ISpatialAudioObjectForMetadataItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectForMetadataItems").field(&self.0).finish()
    }
}
impl ISpatialAudioObjectForMetadataItems {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBuffer)(::windows::core::Vtable::as_raw(self), buffer, bufferlength).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEndOfStream)(::windows::core::Vtable::as_raw(self), framecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsActive)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAudioObjectType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStream {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStream").field(&self.0).finish()
    }
}
impl ISpatialAudioObjectRenderStream {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAvailableDynamicObjectCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetService<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetService)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginUpdatingAudioObjects)(::windows::core::Vtable::as_raw(self), availabledynamicobjectcount, framecountperbuffer).ok()
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndUpdatingAudioObjects)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamBase {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamBase").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamForHrtf {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamForHrtf {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamForHrtf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamForHrtf").field(&self.0).finish()
    }
}
impl ISpatialAudioObjectRenderStreamForHrtf {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAvailableDynamicObjectCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetService<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetService)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginUpdatingAudioObjects)(::windows::core::Vtable::as_raw(self), availabledynamicobjectcount, framecountperbuffer).ok()
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndUpdatingAudioObjects)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamForMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamForMetadata {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamForMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamForMetadata").field(&self.0).finish()
    }
}
impl ISpatialAudioObjectRenderStreamForMetadata {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAvailableDynamicObjectCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetService<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetService)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginUpdatingAudioObjects)(::windows::core::Vtable::as_raw(self), availabledynamicobjectcount, framecountperbuffer).ok()
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndUpdatingAudioObjects)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamNotify {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISubunit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISubunit {}
impl ::core::fmt::Debug for ISubunit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISubunit").field(&self.0).finish()
    }
}
impl ::core::default::Default for MIDIEVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIDIHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIDIINCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIDIINCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIDIINCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIDIINCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIDIOUTCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIDIOUTCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIDIOUTCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIDIOUTCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIDIPROPTEMPO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIDIPROPTIMEDIV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIDISTRMBUFFVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIDI_WAVE_OPEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIDI_WAVE_OPEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIDI_WAVE_OPEN_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIXERCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIXERCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLDETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIXERCONTROLDETAILS_BOOLEAN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLDETAILS_LISTTEXTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIXERCONTROLDETAILS_LISTTEXTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIXERCONTROLDETAILS_SIGNED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIXERCONTROLDETAILS_UNSIGNED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIXERCONTROLW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERLINEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERLINECONTROLSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIXERLINECONTROLSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIXERLINEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIXERLINE_COMPONENTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIXERLINE_COMPONENTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIXERLINE_COMPONENTTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROCESS_LOOPBACK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_LOOPBACK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_LOOPBACK_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PartType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PartType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PartType").field(&self.0).finish()
    }
}
impl ::core::default::Default for SND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SND_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPATIAL_AUDIO_STREAM_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SpatialAudioClientActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SpatialAudioClientActivationParams {
    fn eq(&self, other: &Self) -> bool {
        self.tracingContextId == other.tracingContextId && self.appId == other.appId && self.majorVersion == other.majorVersion && self.minorVersion1 == other.minorVersion1 && self.minorVersion2 == other.minorVersion2 && self.minorVersion3 == other.minorVersion3
    }
}
impl ::core::cmp::Eq for SpatialAudioClientActivationParams {}
impl ::core::fmt::Debug for SpatialAudioClientActivationParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioClientActivationParams").field("tracingContextId", &self.tracingContextId).field("appId", &self.appId).field("majorVersion", &self.majorVersion).field("minorVersion1", &self.minorVersion1).field("minorVersion2", &self.minorVersion2).field("minorVersion3", &self.minorVersion3).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioHrtfActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioHrtfActivationParams2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SpatialAudioHrtfDirectivity {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SpatialAudioHrtfDirectivityCardioid {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SpatialAudioHrtfDirectivityCone {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SpatialAudioHrtfDirectivityType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfDirectivityType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfDirectivityType").field(&self.0).finish()
    }
}
impl ::core::default::Default for SpatialAudioHrtfDirectivityUnion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SpatialAudioHrtfDistanceDecay {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SpatialAudioHrtfDistanceDecayType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfDistanceDecayType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfDistanceDecayType").field(&self.0).finish()
    }
}
impl ::core::default::Default for SpatialAudioHrtfEnvironmentType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialAudioHrtfEnvironmentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfEnvironmentType").field(&self.0).finish()
    }
}
impl ::core::default::Default for SpatialAudioMetadataCopyMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialAudioMetadataCopyMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioMetadataCopyMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for SpatialAudioMetadataItemsInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SpatialAudioMetadataWriterOverflowMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialAudioMetadataWriterOverflowMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioMetadataWriterOverflowMode").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioObjectRenderStreamActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioObjectRenderStreamActivationParams2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VOLUMEWAVEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WAVEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WAVEFORMATEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WAVEFORMATEXTENSIBLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WAVEHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAVEINCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WAVEINCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAVEINCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WAVEINCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAVEOUTCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WAVEOUTCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAVEOUTCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WAVEOUTCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for _AUDCLNT_BUFFERFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _AUDCLNT_BUFFERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_AUDCLNT_BUFFERFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for tACMFORMATDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}

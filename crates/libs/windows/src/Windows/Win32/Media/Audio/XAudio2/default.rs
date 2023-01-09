impl ::core::default::Default for FXECHO_INITDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FXECHO_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FXEQ_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FXMASTERINGLIMITER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FXREVERB_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HrtfApoInit {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HrtfApoInit {
    fn eq(&self, other: &Self) -> bool {
        self.distanceDecay == other.distanceDecay && self.directivity == other.directivity
    }
}
impl ::core::cmp::Eq for HrtfApoInit {}
impl ::core::fmt::Debug for HrtfApoInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfApoInit").field("distanceDecay", &self.distanceDecay).field("directivity", &self.directivity).finish()
    }
}
impl ::core::default::Default for HrtfDirectivity {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HrtfDirectivity {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.scaling == other.scaling
    }
}
impl ::core::cmp::Eq for HrtfDirectivity {}
impl ::core::fmt::Debug for HrtfDirectivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfDirectivity").field("type", &self.r#type).field("scaling", &self.scaling).finish()
    }
}
impl ::core::default::Default for HrtfDirectivityCardioid {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HrtfDirectivityCardioid {
    fn eq(&self, other: &Self) -> bool {
        self.directivity == other.directivity && self.order == other.order
    }
}
impl ::core::cmp::Eq for HrtfDirectivityCardioid {}
impl ::core::fmt::Debug for HrtfDirectivityCardioid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfDirectivityCardioid").field("directivity", &self.directivity).field("order", &self.order).finish()
    }
}
impl ::core::default::Default for HrtfDirectivityCone {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HrtfDirectivityCone {
    fn eq(&self, other: &Self) -> bool {
        self.directivity == other.directivity && self.innerAngle == other.innerAngle && self.outerAngle == other.outerAngle
    }
}
impl ::core::cmp::Eq for HrtfDirectivityCone {}
impl ::core::fmt::Debug for HrtfDirectivityCone {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfDirectivityCone").field("directivity", &self.directivity).field("innerAngle", &self.innerAngle).field("outerAngle", &self.outerAngle).finish()
    }
}
impl ::core::default::Default for HrtfDirectivityType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HrtfDirectivityType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HrtfDirectivityType").field(&self.0).finish()
    }
}
impl ::core::default::Default for HrtfDistanceDecay {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HrtfDistanceDecay {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.maxGain == other.maxGain && self.minGain == other.minGain && self.unityGainDistance == other.unityGainDistance && self.cutoffDistance == other.cutoffDistance
    }
}
impl ::core::cmp::Eq for HrtfDistanceDecay {}
impl ::core::fmt::Debug for HrtfDistanceDecay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfDistanceDecay").field("type", &self.r#type).field("maxGain", &self.maxGain).field("minGain", &self.minGain).field("unityGainDistance", &self.unityGainDistance).field("cutoffDistance", &self.cutoffDistance).finish()
    }
}
impl ::core::default::Default for HrtfDistanceDecayType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HrtfDistanceDecayType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HrtfDistanceDecayType").field(&self.0).finish()
    }
}
impl ::core::default::Default for HrtfEnvironment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HrtfEnvironment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HrtfEnvironment").field(&self.0).finish()
    }
}
impl ::core::default::Default for HrtfOrientation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HrtfOrientation {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
    }
}
impl ::core::cmp::Eq for HrtfOrientation {}
impl ::core::fmt::Debug for HrtfOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfOrientation").field("element", &self.element).finish()
    }
}
impl ::core::default::Default for HrtfPosition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HrtfPosition {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl ::core::cmp::Eq for HrtfPosition {}
impl ::core::fmt::Debug for HrtfPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfPosition").field("x", &self.x).field("y", &self.y).field("z", &self.z).finish()
    }
}
impl ::core::cmp::PartialEq for IXAPO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAPO {}
impl ::core::fmt::Debug for IXAPO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAPO").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXAPOHrtfParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAPOHrtfParameters {}
impl ::core::fmt::Debug for IXAPOHrtfParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAPOHrtfParameters").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXAPOParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAPOParameters {}
impl ::core::fmt::Debug for IXAPOParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAPOParameters").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXAudio2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAudio2 {}
impl ::core::fmt::Debug for IXAudio2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAudio2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXAudio2EngineCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAudio2EngineCallback {}
impl ::core::fmt::Debug for IXAudio2EngineCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAudio2EngineCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXAudio2Extension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAudio2Extension {}
impl ::core::fmt::Debug for IXAudio2Extension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAudio2Extension").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXAudio2MasteringVoice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAudio2MasteringVoice {}
impl ::core::fmt::Debug for IXAudio2MasteringVoice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAudio2MasteringVoice").field(&self.0).finish()
    }
}
impl IXAudio2MasteringVoice {
    pub unsafe fn GetVoiceDetails(&self) -> XAUDIO2_VOICE_DETAILS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVoiceDetails)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetOutputVoices(&self, psendlist: ::core::option::Option<*const XAUDIO2_VOICE_SENDS>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOutputVoices)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psendlist.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEffectChain(&self, peffectchain: ::core::option::Option<*const XAUDIO2_EFFECT_CHAIN>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEffectChain)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(peffectchain.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn EnableEffect(&self, effectindex: u32, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnableEffect)(::windows::core::Vtable::as_raw(self), effectindex, operationset).ok()
    }
    pub unsafe fn DisableEffect(&self, effectindex: u32, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DisableEffect)(::windows::core::Vtable::as_raw(self), effectindex, operationset).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectState(&self, effectindex: u32) -> super::super::super::Foundation::BOOL {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEffectState)(::windows::core::Vtable::as_raw(self), effectindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEffectParameters(&self, effectindex: u32, pparameters: *const ::core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEffectParameters)(::windows::core::Vtable::as_raw(self), effectindex, pparameters, parametersbytesize, operationset).ok()
    }
    pub unsafe fn GetEffectParameters(&self, effectindex: u32, pparameters: *mut ::core::ffi::c_void, parametersbytesize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetEffectParameters)(::windows::core::Vtable::as_raw(self), effectindex, pparameters, parametersbytesize).ok()
    }
    pub unsafe fn SetFilterParameters(&self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFilterParameters)(::windows::core::Vtable::as_raw(self), pparameters, operationset).ok()
    }
    pub unsafe fn GetFilterParameters(&self) -> XAUDIO2_FILTER_PARAMETERS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFilterParameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetOutputFilterParameters<P0>(&self, pdestinationvoice: P0, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXAudio2Voice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOutputFilterParameters)(::windows::core::Vtable::as_raw(self), pdestinationvoice.into().abi(), pparameters, operationset).ok()
    }
    pub unsafe fn GetOutputFilterParameters<P0>(&self, pdestinationvoice: P0) -> XAUDIO2_FILTER_PARAMETERS
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXAudio2Voice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutputFilterParameters)(::windows::core::Vtable::as_raw(self), pdestinationvoice.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetVolume(&self, volume: f32, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVolume)(::windows::core::Vtable::as_raw(self), volume, operationset).ok()
    }
    pub unsafe fn GetVolume(&self) -> f32 {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVolume)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetChannelVolumes(&self, pvolumes: &[f32], operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetChannelVolumes)(::windows::core::Vtable::as_raw(self), pvolumes.len() as _, ::core::mem::transmute(pvolumes.as_ptr()), operationset).ok()
    }
    pub unsafe fn GetChannelVolumes(&self, pvolumes: &mut [f32]) {
        (::windows::core::Vtable::vtable(self).base__.GetChannelVolumes)(::windows::core::Vtable::as_raw(self), pvolumes.len() as _, ::core::mem::transmute(pvolumes.as_ptr()))
    }
    pub unsafe fn SetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXAudio2Voice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOutputMatrix)(::windows::core::Vtable::as_raw(self), pdestinationvoice.into().abi(), sourcechannels, destinationchannels, plevelmatrix, operationset).ok()
    }
    pub unsafe fn GetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32) -> f32
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXAudio2Voice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutputMatrix)(::windows::core::Vtable::as_raw(self), pdestinationvoice.into().abi(), sourcechannels, destinationchannels, result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn DestroyVoice(&self) {
        (::windows::core::Vtable::vtable(self).base__.DestroyVoice)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IXAudio2SourceVoice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAudio2SourceVoice {}
impl ::core::fmt::Debug for IXAudio2SourceVoice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAudio2SourceVoice").field(&self.0).finish()
    }
}
impl IXAudio2SourceVoice {
    pub unsafe fn GetVoiceDetails(&self) -> XAUDIO2_VOICE_DETAILS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVoiceDetails)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetOutputVoices(&self, psendlist: ::core::option::Option<*const XAUDIO2_VOICE_SENDS>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOutputVoices)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psendlist.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEffectChain(&self, peffectchain: ::core::option::Option<*const XAUDIO2_EFFECT_CHAIN>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEffectChain)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(peffectchain.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn EnableEffect(&self, effectindex: u32, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnableEffect)(::windows::core::Vtable::as_raw(self), effectindex, operationset).ok()
    }
    pub unsafe fn DisableEffect(&self, effectindex: u32, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DisableEffect)(::windows::core::Vtable::as_raw(self), effectindex, operationset).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectState(&self, effectindex: u32) -> super::super::super::Foundation::BOOL {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEffectState)(::windows::core::Vtable::as_raw(self), effectindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEffectParameters(&self, effectindex: u32, pparameters: *const ::core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEffectParameters)(::windows::core::Vtable::as_raw(self), effectindex, pparameters, parametersbytesize, operationset).ok()
    }
    pub unsafe fn GetEffectParameters(&self, effectindex: u32, pparameters: *mut ::core::ffi::c_void, parametersbytesize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetEffectParameters)(::windows::core::Vtable::as_raw(self), effectindex, pparameters, parametersbytesize).ok()
    }
    pub unsafe fn SetFilterParameters(&self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFilterParameters)(::windows::core::Vtable::as_raw(self), pparameters, operationset).ok()
    }
    pub unsafe fn GetFilterParameters(&self) -> XAUDIO2_FILTER_PARAMETERS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFilterParameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetOutputFilterParameters<P0>(&self, pdestinationvoice: P0, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXAudio2Voice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOutputFilterParameters)(::windows::core::Vtable::as_raw(self), pdestinationvoice.into().abi(), pparameters, operationset).ok()
    }
    pub unsafe fn GetOutputFilterParameters<P0>(&self, pdestinationvoice: P0) -> XAUDIO2_FILTER_PARAMETERS
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXAudio2Voice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutputFilterParameters)(::windows::core::Vtable::as_raw(self), pdestinationvoice.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetVolume(&self, volume: f32, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVolume)(::windows::core::Vtable::as_raw(self), volume, operationset).ok()
    }
    pub unsafe fn GetVolume(&self) -> f32 {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVolume)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetChannelVolumes(&self, pvolumes: &[f32], operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetChannelVolumes)(::windows::core::Vtable::as_raw(self), pvolumes.len() as _, ::core::mem::transmute(pvolumes.as_ptr()), operationset).ok()
    }
    pub unsafe fn GetChannelVolumes(&self, pvolumes: &mut [f32]) {
        (::windows::core::Vtable::vtable(self).base__.GetChannelVolumes)(::windows::core::Vtable::as_raw(self), pvolumes.len() as _, ::core::mem::transmute(pvolumes.as_ptr()))
    }
    pub unsafe fn SetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXAudio2Voice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOutputMatrix)(::windows::core::Vtable::as_raw(self), pdestinationvoice.into().abi(), sourcechannels, destinationchannels, plevelmatrix, operationset).ok()
    }
    pub unsafe fn GetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32) -> f32
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXAudio2Voice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutputMatrix)(::windows::core::Vtable::as_raw(self), pdestinationvoice.into().abi(), sourcechannels, destinationchannels, result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn DestroyVoice(&self) {
        (::windows::core::Vtable::vtable(self).base__.DestroyVoice)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IXAudio2SubmixVoice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAudio2SubmixVoice {}
impl ::core::fmt::Debug for IXAudio2SubmixVoice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAudio2SubmixVoice").field(&self.0).finish()
    }
}
impl IXAudio2SubmixVoice {
    pub unsafe fn GetVoiceDetails(&self) -> XAUDIO2_VOICE_DETAILS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVoiceDetails)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetOutputVoices(&self, psendlist: ::core::option::Option<*const XAUDIO2_VOICE_SENDS>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOutputVoices)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psendlist.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEffectChain(&self, peffectchain: ::core::option::Option<*const XAUDIO2_EFFECT_CHAIN>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEffectChain)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(peffectchain.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn EnableEffect(&self, effectindex: u32, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnableEffect)(::windows::core::Vtable::as_raw(self), effectindex, operationset).ok()
    }
    pub unsafe fn DisableEffect(&self, effectindex: u32, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DisableEffect)(::windows::core::Vtable::as_raw(self), effectindex, operationset).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectState(&self, effectindex: u32) -> super::super::super::Foundation::BOOL {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEffectState)(::windows::core::Vtable::as_raw(self), effectindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEffectParameters(&self, effectindex: u32, pparameters: *const ::core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEffectParameters)(::windows::core::Vtable::as_raw(self), effectindex, pparameters, parametersbytesize, operationset).ok()
    }
    pub unsafe fn GetEffectParameters(&self, effectindex: u32, pparameters: *mut ::core::ffi::c_void, parametersbytesize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetEffectParameters)(::windows::core::Vtable::as_raw(self), effectindex, pparameters, parametersbytesize).ok()
    }
    pub unsafe fn SetFilterParameters(&self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFilterParameters)(::windows::core::Vtable::as_raw(self), pparameters, operationset).ok()
    }
    pub unsafe fn GetFilterParameters(&self) -> XAUDIO2_FILTER_PARAMETERS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFilterParameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetOutputFilterParameters<P0>(&self, pdestinationvoice: P0, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXAudio2Voice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOutputFilterParameters)(::windows::core::Vtable::as_raw(self), pdestinationvoice.into().abi(), pparameters, operationset).ok()
    }
    pub unsafe fn GetOutputFilterParameters<P0>(&self, pdestinationvoice: P0) -> XAUDIO2_FILTER_PARAMETERS
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXAudio2Voice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutputFilterParameters)(::windows::core::Vtable::as_raw(self), pdestinationvoice.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetVolume(&self, volume: f32, operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVolume)(::windows::core::Vtable::as_raw(self), volume, operationset).ok()
    }
    pub unsafe fn GetVolume(&self) -> f32 {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVolume)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetChannelVolumes(&self, pvolumes: &[f32], operationset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetChannelVolumes)(::windows::core::Vtable::as_raw(self), pvolumes.len() as _, ::core::mem::transmute(pvolumes.as_ptr()), operationset).ok()
    }
    pub unsafe fn GetChannelVolumes(&self, pvolumes: &mut [f32]) {
        (::windows::core::Vtable::vtable(self).base__.GetChannelVolumes)(::windows::core::Vtable::as_raw(self), pvolumes.len() as _, ::core::mem::transmute(pvolumes.as_ptr()))
    }
    pub unsafe fn SetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXAudio2Voice>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOutputMatrix)(::windows::core::Vtable::as_raw(self), pdestinationvoice.into().abi(), sourcechannels, destinationchannels, plevelmatrix, operationset).ok()
    }
    pub unsafe fn GetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32) -> f32
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXAudio2Voice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutputMatrix)(::windows::core::Vtable::as_raw(self), pdestinationvoice.into().abi(), sourcechannels, destinationchannels, result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn DestroyVoice(&self) {
        (::windows::core::Vtable::vtable(self).base__.DestroyVoice)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IXAudio2Voice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAudio2Voice {}
impl ::core::fmt::Debug for IXAudio2Voice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAudio2Voice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXAudio2VoiceCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAudio2VoiceCallback {}
impl ::core::fmt::Debug for IXAudio2VoiceCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAudio2VoiceCallback").field(&self.0).finish()
    }
}
impl ::core::default::Default for XAPO_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XAPO_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XAPO_BUFFER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for XAPO_LOCKFORPROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XAPO_PROCESS_BUFFER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XAPO_REGISTRATION_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XAUDIO2FX_REVERB_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XAUDIO2FX_VOLUMEMETER_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XAUDIO2_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XAUDIO2_BUFFER_WMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XAUDIO2_DEBUG_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XAUDIO2_EFFECT_CHAIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XAUDIO2_EFFECT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XAUDIO2_FILTER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XAUDIO2_FILTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XAUDIO2_FILTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XAUDIO2_FILTER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XAUDIO2_PERFORMANCE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XAUDIO2_SEND_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XAUDIO2_VOICE_DETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XAUDIO2_VOICE_SENDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XAUDIO2_VOICE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}

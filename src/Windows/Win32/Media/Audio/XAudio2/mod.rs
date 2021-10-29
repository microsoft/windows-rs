#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const AudioReverb: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3261283094,
    18203,
    17560,
    [184, 197, 79, 9, 89, 226, 236, 9],
);
pub const AudioVolumeMeter: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1338224998,
    38698,
    16591,
    [188, 55, 125, 176, 61, 178, 251, 163],
);
#[inline]
pub unsafe fn CreateAudioReverb() -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAudioReverb(
                ppapo: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        CreateAudioReverb(&mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateAudioVolumeMeter() -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAudioVolumeMeter(
                ppapo: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        CreateAudioVolumeMeter(&mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateFX(
    clsid: *const ::windows::runtime::GUID,
    peffect: *mut ::std::option::Option<::windows::runtime::IUnknown>,
    pinitdat: *const ::std::ffi::c_void,
    initdatabytesize: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFX(
                clsid: *const ::windows::runtime::GUID,
                peffect: *mut ::windows::runtime::RawPtr,
                pinitdat: *const ::std::ffi::c_void,
                initdatabytesize: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        CreateFX(
            ::std::mem::transmute(clsid),
            ::std::mem::transmute(peffect),
            ::std::mem::transmute(pinitdat),
            ::std::mem::transmute(initdatabytesize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateHrtfApo(init: *const HrtfApoInit) -> ::windows::runtime::Result<IXAPO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateHrtfApo(
                init: *const HrtfApoInit,
                xapo: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IXAPO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateHrtfApo(::std::mem::transmute(init), &mut result__).from_abi::<IXAPO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FACILITY_XAPO: u32 = 2199u32;
pub const FACILITY_XAUDIO2: u32 = 2198u32;
pub const FXECHO_DEFAULT_DELAY: f32 = 500f32;
pub const FXECHO_DEFAULT_FEEDBACK: f32 = 0.5f32;
pub const FXECHO_DEFAULT_WETDRYMIX: f32 = 0.5f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct FXECHO_INITDATA {
    pub MaxDelay: f32,
}
impl FXECHO_INITDATA {}
impl ::std::default::Default for FXECHO_INITDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FXECHO_INITDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FXECHO_INITDATA {}
unsafe impl ::windows::runtime::Abi for FXECHO_INITDATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FXECHO_MAX_DELAY: f32 = 2000f32;
pub const FXECHO_MAX_FEEDBACK: f32 = 1f32;
pub const FXECHO_MAX_WETDRYMIX: f32 = 1f32;
pub const FXECHO_MIN_DELAY: f32 = 1f32;
pub const FXECHO_MIN_FEEDBACK: f32 = 0f32;
pub const FXECHO_MIN_WETDRYMIX: f32 = 0f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct FXECHO_PARAMETERS {
    pub WetDryMix: f32,
    pub Feedback: f32,
    pub Delay: f32,
}
impl FXECHO_PARAMETERS {}
impl ::std::default::Default for FXECHO_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FXECHO_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FXECHO_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for FXECHO_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FXEQ: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4125102359,
    54980,
    18522,
    [163, 245, 105, 81, 150, 243, 219, 250],
);
pub const FXEQ_DEFAULT_BANDWIDTH: f32 = 1f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_0: f32 = 100f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_1: f32 = 800f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_2: f32 = 2000f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_3: f32 = 10000f32;
pub const FXEQ_DEFAULT_GAIN: f32 = 1f32;
pub const FXEQ_MAX_BANDWIDTH: f32 = 2f32;
pub const FXEQ_MAX_FRAMERATE: u32 = 48000u32;
pub const FXEQ_MAX_FREQUENCY_CENTER: f32 = 20000f32;
pub const FXEQ_MAX_GAIN: f32 = 7.94f32;
pub const FXEQ_MIN_BANDWIDTH: f32 = 0.1f32;
pub const FXEQ_MIN_FRAMERATE: u32 = 22000u32;
pub const FXEQ_MIN_FREQUENCY_CENTER: f32 = 20f32;
pub const FXEQ_MIN_GAIN: f32 = 0.126f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct FXEQ_PARAMETERS {
    pub FrequencyCenter0: f32,
    pub Gain0: f32,
    pub Bandwidth0: f32,
    pub FrequencyCenter1: f32,
    pub Gain1: f32,
    pub Bandwidth1: f32,
    pub FrequencyCenter2: f32,
    pub Gain2: f32,
    pub Bandwidth2: f32,
    pub FrequencyCenter3: f32,
    pub Gain3: f32,
    pub Bandwidth3: f32,
}
impl FXEQ_PARAMETERS {}
impl ::std::default::Default for FXEQ_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FXEQ_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FXEQ_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for FXEQ_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FXEcho: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1345967936,
    63286,
    17562,
    [132, 211, 165, 98, 2, 85, 123, 135],
);
pub const FXLOUDNESS_DEFAULT_MOMENTARY_MS: u32 = 400u32;
pub const FXLOUDNESS_DEFAULT_SHORTTERM_MS: u32 = 3000u32;
pub const FXMASTERINGLIMITER_DEFAULT_LOUDNESS: u32 = 1000u32;
pub const FXMASTERINGLIMITER_DEFAULT_RELEASE: u32 = 6u32;
pub const FXMASTERINGLIMITER_MAX_LOUDNESS: u32 = 1800u32;
pub const FXMASTERINGLIMITER_MAX_RELEASE: u32 = 20u32;
pub const FXMASTERINGLIMITER_MIN_LOUDNESS: u32 = 1u32;
pub const FXMASTERINGLIMITER_MIN_RELEASE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct FXMASTERINGLIMITER_PARAMETERS {
    pub Release: u32,
    pub Loudness: u32,
}
impl FXMASTERINGLIMITER_PARAMETERS {}
impl ::std::default::Default for FXMASTERINGLIMITER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FXMASTERINGLIMITER_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FXMASTERINGLIMITER_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for FXMASTERINGLIMITER_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FXMasteringLimiter: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3289610518,
    11233,
    18173,
    [133, 153, 68, 21, 54, 244, 152, 86],
);
pub const FXREVERB_DEFAULT_DIFFUSION: f32 = 0.9f32;
pub const FXREVERB_DEFAULT_ROOMSIZE: f32 = 0.6f32;
pub const FXREVERB_MAX_DIFFUSION: f32 = 1f32;
pub const FXREVERB_MAX_ROOMSIZE: f32 = 1f32;
pub const FXREVERB_MIN_DIFFUSION: f32 = 0f32;
pub const FXREVERB_MIN_ROOMSIZE: f32 = 0.0001f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct FXREVERB_PARAMETERS {
    pub Diffusion: f32,
    pub RoomSize: f32,
}
impl FXREVERB_PARAMETERS {}
impl ::std::default::Default for FXREVERB_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FXREVERB_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FXREVERB_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for FXREVERB_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FXReverb: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2107296342,
    52072,
    18439,
    [182, 50, 177, 55, 53, 46, 133, 150],
);
pub const HRTF_DEFAULT_UNITY_GAIN_DISTANCE: f32 = 1f32;
pub const HRTF_MAX_GAIN_LIMIT: f32 = 12f32;
pub const HRTF_MIN_GAIN_LIMIT: f32 = -96f32;
pub const HRTF_MIN_UNITY_GAIN_DISTANCE: f32 = 0.05f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HrtfApoInit {
    pub distanceDecay: *mut HrtfDistanceDecay,
    pub directivity: *mut HrtfDirectivity,
}
impl HrtfApoInit {}
impl ::std::default::Default for HrtfApoInit {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HrtfApoInit {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HrtfApoInit")
            .field("distanceDecay", &self.distanceDecay)
            .field("directivity", &self.directivity)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HrtfApoInit {
    fn eq(&self, other: &Self) -> bool {
        self.distanceDecay == other.distanceDecay && self.directivity == other.directivity
    }
}
impl ::std::cmp::Eq for HrtfApoInit {}
unsafe impl ::windows::runtime::Abi for HrtfApoInit {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HrtfDirectivity {
    pub r#type: HrtfDirectivityType,
    pub scaling: f32,
}
impl HrtfDirectivity {}
impl ::std::default::Default for HrtfDirectivity {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HrtfDirectivity {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HrtfDirectivity")
            .field("r#type", &self.r#type)
            .field("scaling", &self.scaling)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HrtfDirectivity {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.scaling == other.scaling
    }
}
impl ::std::cmp::Eq for HrtfDirectivity {}
unsafe impl ::windows::runtime::Abi for HrtfDirectivity {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HrtfDirectivityCardioid {
    pub directivity: HrtfDirectivity,
    pub order: f32,
}
impl HrtfDirectivityCardioid {}
impl ::std::default::Default for HrtfDirectivityCardioid {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HrtfDirectivityCardioid {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HrtfDirectivityCardioid")
            .field("directivity", &self.directivity)
            .field("order", &self.order)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HrtfDirectivityCardioid {
    fn eq(&self, other: &Self) -> bool {
        self.directivity == other.directivity && self.order == other.order
    }
}
impl ::std::cmp::Eq for HrtfDirectivityCardioid {}
unsafe impl ::windows::runtime::Abi for HrtfDirectivityCardioid {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HrtfDirectivityCone {
    pub directivity: HrtfDirectivity,
    pub innerAngle: f32,
    pub outerAngle: f32,
}
impl HrtfDirectivityCone {}
impl ::std::default::Default for HrtfDirectivityCone {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HrtfDirectivityCone {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HrtfDirectivityCone")
            .field("directivity", &self.directivity)
            .field("innerAngle", &self.innerAngle)
            .field("outerAngle", &self.outerAngle)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HrtfDirectivityCone {
    fn eq(&self, other: &Self) -> bool {
        self.directivity == other.directivity
            && self.innerAngle == other.innerAngle
            && self.outerAngle == other.outerAngle
    }
}
impl ::std::cmp::Eq for HrtfDirectivityCone {}
unsafe impl ::windows::runtime::Abi for HrtfDirectivityCone {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HrtfDirectivityType(pub i32);
pub const OmniDirectional: HrtfDirectivityType = HrtfDirectivityType(0i32);
pub const Cardioid: HrtfDirectivityType = HrtfDirectivityType(1i32);
pub const Cone: HrtfDirectivityType = HrtfDirectivityType(2i32);
impl ::std::convert::From<i32> for HrtfDirectivityType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HrtfDirectivityType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HrtfDistanceDecay {
    pub r#type: HrtfDistanceDecayType,
    pub maxGain: f32,
    pub minGain: f32,
    pub unityGainDistance: f32,
    pub cutoffDistance: f32,
}
impl HrtfDistanceDecay {}
impl ::std::default::Default for HrtfDistanceDecay {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HrtfDistanceDecay {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HrtfDistanceDecay")
            .field("r#type", &self.r#type)
            .field("maxGain", &self.maxGain)
            .field("minGain", &self.minGain)
            .field("unityGainDistance", &self.unityGainDistance)
            .field("cutoffDistance", &self.cutoffDistance)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HrtfDistanceDecay {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
            && self.maxGain == other.maxGain
            && self.minGain == other.minGain
            && self.unityGainDistance == other.unityGainDistance
            && self.cutoffDistance == other.cutoffDistance
    }
}
impl ::std::cmp::Eq for HrtfDistanceDecay {}
unsafe impl ::windows::runtime::Abi for HrtfDistanceDecay {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HrtfDistanceDecayType(pub i32);
pub const NaturalDecay: HrtfDistanceDecayType = HrtfDistanceDecayType(0i32);
pub const CustomDecay: HrtfDistanceDecayType = HrtfDistanceDecayType(1i32);
impl ::std::convert::From<i32> for HrtfDistanceDecayType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HrtfDistanceDecayType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HrtfEnvironment(pub i32);
pub const Small: HrtfEnvironment = HrtfEnvironment(0i32);
pub const Medium: HrtfEnvironment = HrtfEnvironment(1i32);
pub const Large: HrtfEnvironment = HrtfEnvironment(2i32);
pub const Outdoors: HrtfEnvironment = HrtfEnvironment(3i32);
impl ::std::convert::From<i32> for HrtfEnvironment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HrtfEnvironment {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HrtfOrientation {
    pub element: [f32; 9],
}
impl HrtfOrientation {}
impl ::std::default::Default for HrtfOrientation {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HrtfOrientation {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HrtfOrientation")
            .field("element", &self.element)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HrtfOrientation {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
    }
}
impl ::std::cmp::Eq for HrtfOrientation {}
unsafe impl ::windows::runtime::Abi for HrtfOrientation {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HrtfPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl HrtfPosition {}
impl ::std::default::Default for HrtfPosition {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HrtfPosition {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HrtfPosition")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HrtfPosition {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl ::std::cmp::Eq for HrtfPosition {}
unsafe impl ::windows::runtime::Abi for HrtfPosition {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXAPO(::windows::runtime::IUnknown);
impl IXAPO {
    pub unsafe fn GetRegistrationProperties(
        &self,
    ) -> ::windows::runtime::Result<*mut XAPO_REGISTRATION_PROPERTIES> {
        let mut result__: <*mut XAPO_REGISTRATION_PROPERTIES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut XAPO_REGISTRATION_PROPERTIES>(result__)
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    pub unsafe fn IsInputFormatSupported(
        &self,
        poutputformat: *const super::super::Multimedia::WAVEFORMATEX,
        prequestedinputformat: *const super::super::Multimedia::WAVEFORMATEX,
    ) -> ::windows::runtime::Result<*mut super::super::Multimedia::WAVEFORMATEX> {
        let mut result__ : < * mut super::super::Multimedia:: WAVEFORMATEX as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(poutputformat),
            ::std::mem::transmute(prequestedinputformat),
            &mut result__,
        )
        .from_abi::<*mut super::super::Multimedia::WAVEFORMATEX>(result__)
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    pub unsafe fn IsOutputFormatSupported(
        &self,
        pinputformat: *const super::super::Multimedia::WAVEFORMATEX,
        prequestedoutputformat: *const super::super::Multimedia::WAVEFORMATEX,
    ) -> ::windows::runtime::Result<*mut super::super::Multimedia::WAVEFORMATEX> {
        let mut result__ : < * mut super::super::Multimedia:: WAVEFORMATEX as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pinputformat),
            ::std::mem::transmute(prequestedoutputformat),
            &mut result__,
        )
        .from_abi::<*mut super::super::Multimedia::WAVEFORMATEX>(result__)
    }
    pub unsafe fn Initialize(
        &self,
        pdata: *const ::std::ffi::c_void,
        databytesize: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(databytesize),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    pub unsafe fn LockForProcess(
        &self,
        inputlockedparametercount: u32,
        pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS,
        outputlockedparametercount: u32,
        poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(inputlockedparametercount),
            ::std::mem::transmute(pinputlockedparameters),
            ::std::mem::transmute(outputlockedparametercount),
            ::std::mem::transmute(poutputlockedparameters),
        )
        .ok()
    }
    pub unsafe fn UnlockForProcess(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process<
        'a,
        Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>,
    >(
        &self,
        inputprocessparametercount: u32,
        pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS,
        outputprocessparametercount: u32,
        poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS,
        isenabled: Param4,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(inputprocessparametercount),
            ::std::mem::transmute(pinputprocessparameters),
            ::std::mem::transmute(outputprocessparametercount),
            ::std::mem::transmute(poutputprocessparameters),
            isenabled.into_param().abi(),
        ))
    }
    pub unsafe fn CalcInputFrames(&self, outputframecount: u32) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(outputframecount),
        ))
    }
    pub unsafe fn CalcOutputFrames(&self, inputframecount: u32) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(inputframecount),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IXAPO {
    type Vtable = IXAPO_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2752559492,
        38969,
        18457,
        [160, 190, 40, 86, 174, 107, 58, 219],
    );
}
impl ::std::convert::From<IXAPO> for ::windows::runtime::IUnknown {
    fn from(value: IXAPO) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAPO> for ::windows::runtime::IUnknown {
    fn from(value: &IXAPO) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXAPO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXAPO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAPO_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppregistrationproperties: *mut *mut XAPO_REGISTRATION_PROPERTIES,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        poutputformat: *const super::super::Multimedia::WAVEFORMATEX,
        prequestedinputformat: *const super::super::Multimedia::WAVEFORMATEX,
        ppsupportedinputformat: *mut *mut super::super::Multimedia::WAVEFORMATEX,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    #[cfg(feature = "Win32_Media_Multimedia")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pinputformat: *const super::super::Multimedia::WAVEFORMATEX,
        prequestedoutputformat: *const super::super::Multimedia::WAVEFORMATEX,
        ppsupportedoutputformat: *mut *mut super::super::Multimedia::WAVEFORMATEX,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdata: *const ::std::ffi::c_void,
        databytesize: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Media_Multimedia")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        inputlockedparametercount: u32,
        pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS,
        outputlockedparametercount: u32,
        poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        inputprocessparametercount: u32,
        pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS,
        outputprocessparametercount: u32,
        poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS,
        isenabled: super::super::super::Foundation::BOOL,
    ),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputframecount: u32) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputframecount: u32) -> u32,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXAPOHrtfParameters(::windows::runtime::IUnknown);
impl IXAPOHrtfParameters {
    pub unsafe fn SetSourcePosition(
        &self,
        position: *const HrtfPosition,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(position),
        )
        .ok()
    }
    pub unsafe fn SetSourceOrientation(
        &self,
        orientation: *const HrtfOrientation,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(orientation),
        )
        .ok()
    }
    pub unsafe fn SetSourceGain(&self, gain: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(gain),
        )
        .ok()
    }
    pub unsafe fn SetEnvironment(
        &self,
        environment: HrtfEnvironment,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(environment),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXAPOHrtfParameters {
    type Vtable = IXAPOHrtfParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        364105062,
        59870,
        17508,
        [182, 230, 43, 195, 207, 99, 212, 85],
    );
}
impl ::std::convert::From<IXAPOHrtfParameters> for ::windows::runtime::IUnknown {
    fn from(value: IXAPOHrtfParameters) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAPOHrtfParameters> for ::windows::runtime::IUnknown {
    fn from(value: &IXAPOHrtfParameters) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXAPOHrtfParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXAPOHrtfParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAPOHrtfParameters_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        position: *const HrtfPosition,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        orientation: *const HrtfOrientation,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        gain: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        environment: HrtfEnvironment,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXAPOParameters(::windows::runtime::IUnknown);
impl IXAPOParameters {
    pub unsafe fn SetParameters(
        &self,
        pparameters: *const ::std::ffi::c_void,
        parameterbytesize: u32,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(parameterbytesize),
        ))
    }
    pub unsafe fn GetParameters(
        &self,
        pparameters: *mut ::std::ffi::c_void,
        parameterbytesize: u32,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(parameterbytesize),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IXAPOParameters {
    type Vtable = IXAPOParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        651779174,
        33010,
        18842,
        [173, 84, 90, 231, 240, 28, 109, 152],
    );
}
impl ::std::convert::From<IXAPOParameters> for ::windows::runtime::IUnknown {
    fn from(value: IXAPOParameters) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAPOParameters> for ::windows::runtime::IUnknown {
    fn from(value: &IXAPOParameters) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXAPOParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXAPOParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAPOParameters_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pparameters: *const ::std::ffi::c_void,
        parameterbytesize: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pparameters: *mut ::std::ffi::c_void,
        parameterbytesize: u32,
    ),
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXAudio2(::windows::runtime::IUnknown);
impl IXAudio2 {
    pub unsafe fn RegisterForCallbacks<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IXAudio2EngineCallback>,
    >(
        &self,
        pcallback: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pcallback.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn UnregisterForCallbacks<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IXAudio2EngineCallback>,
    >(
        &self,
        pcallback: Param0,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pcallback.into_param().abi(),
        ))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Multimedia"))]
    pub unsafe fn CreateSourceVoice<
        'a,
        Param4: ::windows::runtime::IntoParam<'a, IXAudio2VoiceCallback>,
    >(
        &self,
        ppsourcevoice: *mut ::std::option::Option<IXAudio2SourceVoice>,
        psourceformat: *const super::super::Multimedia::WAVEFORMATEX,
        flags: u32,
        maxfrequencyratio: f32,
        pcallback: Param4,
        psendlist: *const XAUDIO2_VOICE_SENDS,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppsourcevoice),
            ::std::mem::transmute(psourceformat),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(maxfrequencyratio),
            pcallback.into_param().abi(),
            ::std::mem::transmute(psendlist),
            ::std::mem::transmute(peffectchain),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSubmixVoice(
        &self,
        ppsubmixvoice: *mut ::std::option::Option<IXAudio2SubmixVoice>,
        inputchannels: u32,
        inputsamplerate: u32,
        flags: u32,
        processingstage: u32,
        psendlist: *const XAUDIO2_VOICE_SENDS,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppsubmixvoice),
            ::std::mem::transmute(inputchannels),
            ::std::mem::transmute(inputsamplerate),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(processingstage),
            ::std::mem::transmute(psendlist),
            ::std::mem::transmute(peffectchain),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
    pub unsafe fn CreateMasteringVoice<
        'a,
        Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>,
    >(
        &self,
        ppmasteringvoice: *mut ::std::option::Option<IXAudio2MasteringVoice>,
        inputchannels: u32,
        inputsamplerate: u32,
        flags: u32,
        szdeviceid: Param4,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
        streamcategory: super::CoreAudio::AUDIO_STREAM_CATEGORY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppmasteringvoice),
            ::std::mem::transmute(inputchannels),
            ::std::mem::transmute(inputsamplerate),
            ::std::mem::transmute(flags),
            szdeviceid.into_param().abi(),
            ::std::mem::transmute(peffectchain),
            ::std::mem::transmute(streamcategory),
        )
        .ok()
    }
    pub unsafe fn StartEngine(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn StopEngine(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn CommitChanges(&self, operationset: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetPerformanceData(&self, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pperfdata),
        ))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDebugConfiguration(
        &self,
        pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION,
        preserved: *mut ::std::ffi::c_void,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdebugconfiguration),
            ::std::mem::transmute(preserved),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IXAudio2 {
    type Vtable = IXAudio2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        721609679,
        11787,
        20163,
        [190, 69, 27, 42, 63, 231, 33, 13],
    );
}
impl ::std::convert::From<IXAudio2> for ::windows::runtime::IUnknown {
    fn from(value: IXAudio2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAudio2> for ::windows::runtime::IUnknown {
    fn from(value: &IXAudio2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXAudio2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXAudio2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcallback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcallback: ::windows::runtime::RawPtr,
    ),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Multimedia"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppsourcevoice: *mut ::windows::runtime::RawPtr,
        psourceformat: *const super::super::Multimedia::WAVEFORMATEX,
        flags: u32,
        maxfrequencyratio: f32,
        pcallback: ::windows::runtime::RawPtr,
        psendlist: *const XAUDIO2_VOICE_SENDS,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_Multimedia")))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppsubmixvoice: *mut ::windows::runtime::RawPtr,
        inputchannels: u32,
        inputsamplerate: u32,
        flags: u32,
        processingstage: u32,
        psendlist: *const XAUDIO2_VOICE_SENDS,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppmasteringvoice: *mut ::windows::runtime::RawPtr,
        inputchannels: u32,
        inputsamplerate: u32,
        flags: u32,
        szdeviceid: super::super::super::Foundation::PWSTR,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
        streamcategory: super::CoreAudio::AUDIO_STREAM_CATEGORY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pperfdata: *mut XAUDIO2_PERFORMANCE_DATA,
    ),
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION,
        preserved: *mut ::std::ffi::c_void,
    ),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXAudio2EngineCallback(::windows::runtime::IUnknown);
impl IXAudio2EngineCallback {
    pub unsafe fn OnProcessingPassStart(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn OnProcessingPassEnd(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn OnCriticalError(&self, error: ::windows::runtime::HRESULT) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(error),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IXAudio2EngineCallback {
    type Vtable = IXAudio2EngineCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IXAudio2EngineCallback> for ::windows::runtime::IUnknown {
    fn from(value: IXAudio2EngineCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAudio2EngineCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IXAudio2EngineCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IXAudio2EngineCallback
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IXAudio2EngineCallback
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2EngineCallback_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        error: ::windows::runtime::HRESULT,
    ),
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXAudio2Extension(::windows::runtime::IUnknown);
impl IXAudio2Extension {
    pub unsafe fn GetProcessingQuantum(
        &self,
        quantumnumerator: *mut u32,
        quantumdenominator: *mut u32,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(quantumnumerator),
            ::std::mem::transmute(quantumdenominator),
        ))
    }
    pub unsafe fn GetProcessor(&self, processor: *mut u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(processor),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IXAudio2Extension {
    type Vtable = IXAudio2Extension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2225875387,
        54809,
        17618,
        [177, 151, 228, 172, 247, 223, 62, 214],
    );
}
impl ::std::convert::From<IXAudio2Extension> for ::windows::runtime::IUnknown {
    fn from(value: IXAudio2Extension) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAudio2Extension> for ::windows::runtime::IUnknown {
    fn from(value: &IXAudio2Extension) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXAudio2Extension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXAudio2Extension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2Extension_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        quantumnumerator: *mut u32,
        quantumdenominator: *mut u32,
    ),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, processor: *mut u32),
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXAudio2MasteringVoice(::windows::runtime::IUnknown);
impl IXAudio2MasteringVoice {
    pub unsafe fn GetVoiceDetails(&self, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvoicedetails),
        ))
    }
    pub unsafe fn SetOutputVoices(
        &self,
        psendlist: *const XAUDIO2_VOICE_SENDS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(psendlist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEffectChain(
        &self,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(peffectchain),
        )
        .ok()
    }
    pub unsafe fn EnableEffect(
        &self,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn DisableEffect(
        &self,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectState(
        &self,
        effectindex: u32,
        penabled: *mut super::super::super::Foundation::BOOL,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(penabled),
        ))
    }
    pub unsafe fn SetEffectParameters(
        &self,
        effectindex: u32,
        pparameters: *const ::std::ffi::c_void,
        parametersbytesize: u32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(parametersbytesize),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetEffectParameters(
        &self,
        effectindex: u32,
        pparameters: *mut ::std::ffi::c_void,
        parametersbytesize: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(parametersbytesize),
        )
        .ok()
    }
    pub unsafe fn SetFilterParameters(
        &self,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetFilterParameters(&self, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pparameters),
        ))
    }
    pub unsafe fn SetOutputFilterParameters<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>,
    >(
        &self,
        pdestinationvoice: Param0,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetOutputFilterParameters<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>,
    >(
        &self,
        pdestinationvoice: Param0,
        pparameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(pparameters),
        ))
    }
    pub unsafe fn SetVolume(
        &self,
        volume: f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(volume),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetVolume(&self, pvolume: *mut f32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvolume),
        ))
    }
    pub unsafe fn SetChannelVolumes(
        &self,
        channels: u32,
        pvolumes: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(channels),
            ::std::mem::transmute(pvolumes),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetChannelVolumes(&self, channels: u32, pvolumes: *mut f32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(channels),
            ::std::mem::transmute(pvolumes),
        ))
    }
    pub unsafe fn SetOutputMatrix<'a, Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>>(
        &self,
        pdestinationvoice: Param0,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(sourcechannels),
            ::std::mem::transmute(destinationchannels),
            ::std::mem::transmute(plevelmatrix),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetOutputMatrix<'a, Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>>(
        &self,
        pdestinationvoice: Param0,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *mut f32,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(sourcechannels),
            ::std::mem::transmute(destinationchannels),
            ::std::mem::transmute(plevelmatrix),
        ))
    }
    pub unsafe fn DestroyVoice(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn GetChannelMask(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXAudio2MasteringVoice {
    type Vtable = IXAudio2MasteringVoice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IXAudio2MasteringVoice> for ::windows::runtime::IUnknown {
    fn from(value: IXAudio2MasteringVoice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAudio2MasteringVoice> for ::windows::runtime::IUnknown {
    fn from(value: &IXAudio2MasteringVoice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IXAudio2MasteringVoice
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IXAudio2MasteringVoice
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IXAudio2MasteringVoice> for IXAudio2Voice {
    fn from(value: IXAudio2MasteringVoice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAudio2MasteringVoice> for IXAudio2Voice {
    fn from(value: &IXAudio2MasteringVoice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXAudio2Voice> for IXAudio2MasteringVoice {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXAudio2Voice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IXAudio2Voice>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXAudio2Voice> for &IXAudio2MasteringVoice {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXAudio2Voice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IXAudio2Voice>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2MasteringVoice_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvoicedetails: *mut XAUDIO2_VOICE_DETAILS,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psendlist: *const XAUDIO2_VOICE_SENDS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        penabled: *mut super::super::super::Foundation::BOOL,
    ),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        pparameters: *const ::std::ffi::c_void,
        parametersbytesize: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        pparameters: *mut ::std::ffi::c_void,
        parametersbytesize: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pparameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        pparameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        volume: f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvolume: *mut f32),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        channels: u32,
        pvolumes: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        channels: u32,
        pvolumes: *mut f32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *mut f32,
    ),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pchannelmask: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXAudio2SourceVoice(::windows::runtime::IUnknown);
impl IXAudio2SourceVoice {
    pub unsafe fn GetVoiceDetails(&self, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvoicedetails),
        ))
    }
    pub unsafe fn SetOutputVoices(
        &self,
        psendlist: *const XAUDIO2_VOICE_SENDS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(psendlist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEffectChain(
        &self,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(peffectchain),
        )
        .ok()
    }
    pub unsafe fn EnableEffect(
        &self,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn DisableEffect(
        &self,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectState(
        &self,
        effectindex: u32,
        penabled: *mut super::super::super::Foundation::BOOL,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(penabled),
        ))
    }
    pub unsafe fn SetEffectParameters(
        &self,
        effectindex: u32,
        pparameters: *const ::std::ffi::c_void,
        parametersbytesize: u32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(parametersbytesize),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetEffectParameters(
        &self,
        effectindex: u32,
        pparameters: *mut ::std::ffi::c_void,
        parametersbytesize: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(parametersbytesize),
        )
        .ok()
    }
    pub unsafe fn SetFilterParameters(
        &self,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetFilterParameters(&self, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pparameters),
        ))
    }
    pub unsafe fn SetOutputFilterParameters<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>,
    >(
        &self,
        pdestinationvoice: Param0,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetOutputFilterParameters<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>,
    >(
        &self,
        pdestinationvoice: Param0,
        pparameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(pparameters),
        ))
    }
    pub unsafe fn SetVolume(
        &self,
        volume: f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(volume),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetVolume(&self, pvolume: *mut f32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvolume),
        ))
    }
    pub unsafe fn SetChannelVolumes(
        &self,
        channels: u32,
        pvolumes: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(channels),
            ::std::mem::transmute(pvolumes),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetChannelVolumes(&self, channels: u32, pvolumes: *mut f32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(channels),
            ::std::mem::transmute(pvolumes),
        ))
    }
    pub unsafe fn SetOutputMatrix<'a, Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>>(
        &self,
        pdestinationvoice: Param0,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(sourcechannels),
            ::std::mem::transmute(destinationchannels),
            ::std::mem::transmute(plevelmatrix),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetOutputMatrix<'a, Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>>(
        &self,
        pdestinationvoice: Param0,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *mut f32,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(sourcechannels),
            ::std::mem::transmute(destinationchannels),
            ::std::mem::transmute(plevelmatrix),
        ))
    }
    pub unsafe fn DestroyVoice(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn Start(&self, flags: u32, operationset: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn Stop(&self, flags: u32, operationset: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn SubmitSourceBuffer(
        &self,
        pbuffer: *const XAUDIO2_BUFFER,
        pbufferwma: *const XAUDIO2_BUFFER_WMA,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbuffer),
            ::std::mem::transmute(pbufferwma),
        )
        .ok()
    }
    pub unsafe fn FlushSourceBuffers(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Discontinuity(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ExitLoop(&self, operationset: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetState(&self, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvoicestate),
            ::std::mem::transmute(flags),
        ))
    }
    pub unsafe fn SetFrequencyRatio(
        &self,
        ratio: f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ratio),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetFrequencyRatio(&self, pratio: *mut f32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pratio),
        ))
    }
    pub unsafe fn SetSourceSampleRate(
        &self,
        newsourcesamplerate: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(newsourcesamplerate),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IXAudio2SourceVoice {
    type Vtable = IXAudio2SourceVoice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IXAudio2SourceVoice> for ::windows::runtime::IUnknown {
    fn from(value: IXAudio2SourceVoice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAudio2SourceVoice> for ::windows::runtime::IUnknown {
    fn from(value: &IXAudio2SourceVoice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXAudio2SourceVoice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXAudio2SourceVoice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IXAudio2SourceVoice> for IXAudio2Voice {
    fn from(value: IXAudio2SourceVoice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAudio2SourceVoice> for IXAudio2Voice {
    fn from(value: &IXAudio2SourceVoice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXAudio2Voice> for IXAudio2SourceVoice {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXAudio2Voice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IXAudio2Voice>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXAudio2Voice> for &IXAudio2SourceVoice {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXAudio2Voice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IXAudio2Voice>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2SourceVoice_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvoicedetails: *mut XAUDIO2_VOICE_DETAILS,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psendlist: *const XAUDIO2_VOICE_SENDS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        penabled: *mut super::super::super::Foundation::BOOL,
    ),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        pparameters: *const ::std::ffi::c_void,
        parametersbytesize: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        pparameters: *mut ::std::ffi::c_void,
        parametersbytesize: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pparameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        pparameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        volume: f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvolume: *mut f32),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        channels: u32,
        pvolumes: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        channels: u32,
        pvolumes: *mut f32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *mut f32,
    ),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        flags: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        flags: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbuffer: *const XAUDIO2_BUFFER,
        pbufferwma: *const XAUDIO2_BUFFER_WMA,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvoicestate: *mut XAUDIO2_VOICE_STATE,
        flags: u32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ratio: f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pratio: *mut f32),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        newsourcesamplerate: u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXAudio2SubmixVoice(::windows::runtime::IUnknown);
impl IXAudio2SubmixVoice {
    pub unsafe fn GetVoiceDetails(&self, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvoicedetails),
        ))
    }
    pub unsafe fn SetOutputVoices(
        &self,
        psendlist: *const XAUDIO2_VOICE_SENDS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(psendlist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEffectChain(
        &self,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(peffectchain),
        )
        .ok()
    }
    pub unsafe fn EnableEffect(
        &self,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn DisableEffect(
        &self,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectState(
        &self,
        effectindex: u32,
        penabled: *mut super::super::super::Foundation::BOOL,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(penabled),
        ))
    }
    pub unsafe fn SetEffectParameters(
        &self,
        effectindex: u32,
        pparameters: *const ::std::ffi::c_void,
        parametersbytesize: u32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(parametersbytesize),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetEffectParameters(
        &self,
        effectindex: u32,
        pparameters: *mut ::std::ffi::c_void,
        parametersbytesize: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(parametersbytesize),
        )
        .ok()
    }
    pub unsafe fn SetFilterParameters(
        &self,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetFilterParameters(&self, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pparameters),
        ))
    }
    pub unsafe fn SetOutputFilterParameters<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>,
    >(
        &self,
        pdestinationvoice: Param0,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetOutputFilterParameters<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>,
    >(
        &self,
        pdestinationvoice: Param0,
        pparameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(pparameters),
        ))
    }
    pub unsafe fn SetVolume(
        &self,
        volume: f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(volume),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetVolume(&self, pvolume: *mut f32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvolume),
        ))
    }
    pub unsafe fn SetChannelVolumes(
        &self,
        channels: u32,
        pvolumes: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(channels),
            ::std::mem::transmute(pvolumes),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetChannelVolumes(&self, channels: u32, pvolumes: *mut f32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(channels),
            ::std::mem::transmute(pvolumes),
        ))
    }
    pub unsafe fn SetOutputMatrix<'a, Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>>(
        &self,
        pdestinationvoice: Param0,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(sourcechannels),
            ::std::mem::transmute(destinationchannels),
            ::std::mem::transmute(plevelmatrix),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetOutputMatrix<'a, Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>>(
        &self,
        pdestinationvoice: Param0,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *mut f32,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(sourcechannels),
            ::std::mem::transmute(destinationchannels),
            ::std::mem::transmute(plevelmatrix),
        ))
    }
    pub unsafe fn DestroyVoice(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IXAudio2SubmixVoice {
    type Vtable = IXAudio2SubmixVoice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IXAudio2SubmixVoice> for ::windows::runtime::IUnknown {
    fn from(value: IXAudio2SubmixVoice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAudio2SubmixVoice> for ::windows::runtime::IUnknown {
    fn from(value: &IXAudio2SubmixVoice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXAudio2SubmixVoice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXAudio2SubmixVoice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IXAudio2SubmixVoice> for IXAudio2Voice {
    fn from(value: IXAudio2SubmixVoice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAudio2SubmixVoice> for IXAudio2Voice {
    fn from(value: &IXAudio2SubmixVoice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXAudio2Voice> for IXAudio2SubmixVoice {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXAudio2Voice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IXAudio2Voice>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IXAudio2Voice> for &IXAudio2SubmixVoice {
    fn into_param(self) -> ::windows::runtime::Param<'a, IXAudio2Voice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IXAudio2Voice>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2SubmixVoice_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvoicedetails: *mut XAUDIO2_VOICE_DETAILS,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psendlist: *const XAUDIO2_VOICE_SENDS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        penabled: *mut super::super::super::Foundation::BOOL,
    ),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        pparameters: *const ::std::ffi::c_void,
        parametersbytesize: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        pparameters: *mut ::std::ffi::c_void,
        parametersbytesize: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pparameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        pparameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        volume: f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvolume: *mut f32),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        channels: u32,
        pvolumes: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        channels: u32,
        pvolumes: *mut f32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *mut f32,
    ),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXAudio2Voice(::windows::runtime::IUnknown);
impl IXAudio2Voice {
    pub unsafe fn GetVoiceDetails(&self, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvoicedetails),
        ))
    }
    pub unsafe fn SetOutputVoices(
        &self,
        psendlist: *const XAUDIO2_VOICE_SENDS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(psendlist),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEffectChain(
        &self,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(peffectchain),
        )
        .ok()
    }
    pub unsafe fn EnableEffect(
        &self,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn DisableEffect(
        &self,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectState(
        &self,
        effectindex: u32,
        penabled: *mut super::super::super::Foundation::BOOL,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(penabled),
        ))
    }
    pub unsafe fn SetEffectParameters(
        &self,
        effectindex: u32,
        pparameters: *const ::std::ffi::c_void,
        parametersbytesize: u32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(parametersbytesize),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetEffectParameters(
        &self,
        effectindex: u32,
        pparameters: *mut ::std::ffi::c_void,
        parametersbytesize: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(effectindex),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(parametersbytesize),
        )
        .ok()
    }
    pub unsafe fn SetFilterParameters(
        &self,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetFilterParameters(&self, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pparameters),
        ))
    }
    pub unsafe fn SetOutputFilterParameters<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>,
    >(
        &self,
        pdestinationvoice: Param0,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(pparameters),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetOutputFilterParameters<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>,
    >(
        &self,
        pdestinationvoice: Param0,
        pparameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(pparameters),
        ))
    }
    pub unsafe fn SetVolume(
        &self,
        volume: f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(volume),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetVolume(&self, pvolume: *mut f32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvolume),
        ))
    }
    pub unsafe fn SetChannelVolumes(
        &self,
        channels: u32,
        pvolumes: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(channels),
            ::std::mem::transmute(pvolumes),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetChannelVolumes(&self, channels: u32, pvolumes: *mut f32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(channels),
            ::std::mem::transmute(pvolumes),
        ))
    }
    pub unsafe fn SetOutputMatrix<'a, Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>>(
        &self,
        pdestinationvoice: Param0,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(sourcechannels),
            ::std::mem::transmute(destinationchannels),
            ::std::mem::transmute(plevelmatrix),
            ::std::mem::transmute(operationset),
        )
        .ok()
    }
    pub unsafe fn GetOutputMatrix<'a, Param0: ::windows::runtime::IntoParam<'a, IXAudio2Voice>>(
        &self,
        pdestinationvoice: Param0,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *mut f32,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            pdestinationvoice.into_param().abi(),
            ::std::mem::transmute(sourcechannels),
            ::std::mem::transmute(destinationchannels),
            ::std::mem::transmute(plevelmatrix),
        ))
    }
    pub unsafe fn DestroyVoice(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IXAudio2Voice {
    type Vtable = IXAudio2Voice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IXAudio2Voice> for ::windows::runtime::IUnknown {
    fn from(value: IXAudio2Voice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAudio2Voice> for ::windows::runtime::IUnknown {
    fn from(value: &IXAudio2Voice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXAudio2Voice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IXAudio2Voice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2Voice_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvoicedetails: *mut XAUDIO2_VOICE_DETAILS,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psendlist: *const XAUDIO2_VOICE_SENDS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        peffectchain: *const XAUDIO2_EFFECT_CHAIN,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        penabled: *mut super::super::super::Foundation::BOOL,
    ),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        pparameters: *const ::std::ffi::c_void,
        parametersbytesize: u32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectindex: u32,
        pparameters: *mut ::std::ffi::c_void,
        parametersbytesize: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pparameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        pparameters: *const XAUDIO2_FILTER_PARAMETERS,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        pparameters: *mut XAUDIO2_FILTER_PARAMETERS,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        volume: f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvolume: *mut f32),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        channels: u32,
        pvolumes: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        channels: u32,
        pvolumes: *mut f32,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *const f32,
        operationset: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdestinationvoice: ::windows::runtime::RawPtr,
        sourcechannels: u32,
        destinationchannels: u32,
        plevelmatrix: *mut f32,
    ),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IXAudio2VoiceCallback(::windows::runtime::IUnknown);
impl IXAudio2VoiceCallback {
    pub unsafe fn OnVoiceProcessingPassStart(&self, bytesrequired: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bytesrequired),
        ))
    }
    pub unsafe fn OnVoiceProcessingPassEnd(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn OnStreamEnd(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn OnBufferStart(&self, pbuffercontext: *mut ::std::ffi::c_void) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbuffercontext),
        ))
    }
    pub unsafe fn OnBufferEnd(&self, pbuffercontext: *mut ::std::ffi::c_void) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbuffercontext),
        ))
    }
    pub unsafe fn OnLoopEnd(&self, pbuffercontext: *mut ::std::ffi::c_void) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbuffercontext),
        ))
    }
    pub unsafe fn OnVoiceError(
        &self,
        pbuffercontext: *mut ::std::ffi::c_void,
        error: ::windows::runtime::HRESULT,
    ) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbuffercontext),
            ::std::mem::transmute(error),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for IXAudio2VoiceCallback {
    type Vtable = IXAudio2VoiceCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
impl ::std::convert::From<IXAudio2VoiceCallback> for ::windows::runtime::IUnknown {
    fn from(value: IXAudio2VoiceCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IXAudio2VoiceCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IXAudio2VoiceCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXAudio2VoiceCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IXAudio2VoiceCallback
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2VoiceCallback_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bytesrequired: u32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbuffercontext: *mut ::std::ffi::c_void,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbuffercontext: *mut ::std::ffi::c_void,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbuffercontext: *mut ::std::ffi::c_void,
    ),
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbuffercontext: *mut ::std::ffi::c_void,
        error: ::windows::runtime::HRESULT,
    ),
);
pub const Processor1: u32 = 1u32;
pub const Processor10: u32 = 512u32;
pub const Processor11: u32 = 1024u32;
pub const Processor12: u32 = 2048u32;
pub const Processor13: u32 = 4096u32;
pub const Processor14: u32 = 8192u32;
pub const Processor15: u32 = 16384u32;
pub const Processor16: u32 = 32768u32;
pub const Processor17: u32 = 65536u32;
pub const Processor18: u32 = 131072u32;
pub const Processor19: u32 = 262144u32;
pub const Processor2: u32 = 2u32;
pub const Processor20: u32 = 524288u32;
pub const Processor21: u32 = 1048576u32;
pub const Processor22: u32 = 2097152u32;
pub const Processor23: u32 = 4194304u32;
pub const Processor24: u32 = 8388608u32;
pub const Processor25: u32 = 16777216u32;
pub const Processor26: u32 = 33554432u32;
pub const Processor27: u32 = 67108864u32;
pub const Processor28: u32 = 134217728u32;
pub const Processor29: u32 = 268435456u32;
pub const Processor3: u32 = 4u32;
pub const Processor30: u32 = 536870912u32;
pub const Processor31: u32 = 1073741824u32;
pub const Processor32: u32 = 2147483648u32;
pub const Processor4: u32 = 8u32;
pub const Processor5: u32 = 16u32;
pub const Processor6: u32 = 32u32;
pub const Processor7: u32 = 64u32;
pub const Processor8: u32 = 128u32;
pub const Processor9: u32 = 256u32;
pub const SPEAKER_MONO: u32 = 4u32;
pub const X3DAUDIO_2PI: f32 = 6.2831855f32;
pub const X3DAUDIO_CALCULATE_DELAY: u32 = 2u32;
pub const X3DAUDIO_CALCULATE_DOPPLER: u32 = 32u32;
pub const X3DAUDIO_CALCULATE_EMITTER_ANGLE: u32 = 64u32;
pub const X3DAUDIO_CALCULATE_LPF_DIRECT: u32 = 4u32;
pub const X3DAUDIO_CALCULATE_LPF_REVERB: u32 = 8u32;
pub const X3DAUDIO_CALCULATE_MATRIX: u32 = 1u32;
pub const X3DAUDIO_CALCULATE_REDIRECT_TO_LFE: u32 = 131072u32;
pub const X3DAUDIO_CALCULATE_REVERB: u32 = 16u32;
pub const X3DAUDIO_CALCULATE_ZEROCENTER: u32 = 65536u32;
pub const X3DAUDIO_HANDLE_BYTESIZE: u32 = 20u32;
pub const X3DAUDIO_PI: f32 = 3.1415927f32;
pub const X3DAUDIO_SPEED_OF_SOUND: f32 = 343.5f32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct XAPO_BUFFER_FLAGS(pub i32);
pub const XAPO_BUFFER_SILENT: XAPO_BUFFER_FLAGS = XAPO_BUFFER_FLAGS(0i32);
pub const XAPO_BUFFER_VALID: XAPO_BUFFER_FLAGS = XAPO_BUFFER_FLAGS(1i32);
impl ::std::convert::From<i32> for XAPO_BUFFER_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XAPO_BUFFER_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const XAPO_E_FORMAT_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2003369983i32 as _);
pub const XAPO_FLAG_BITSPERSAMPLE_MUST_MATCH: u32 = 4u32;
pub const XAPO_FLAG_BUFFERCOUNT_MUST_MATCH: u32 = 8u32;
pub const XAPO_FLAG_CHANNELS_MUST_MATCH: u32 = 1u32;
pub const XAPO_FLAG_FRAMERATE_MUST_MATCH: u32 = 2u32;
pub const XAPO_FLAG_INPLACE_REQUIRED: u32 = 32u32;
pub const XAPO_FLAG_INPLACE_SUPPORTED: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Multimedia")]
pub struct XAPO_LOCKFORPROCESS_PARAMETERS {
    pub pFormat: *mut super::super::Multimedia::WAVEFORMATEX,
    pub MaxFrameCount: u32,
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl XAPO_LOCKFORPROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::default::Default for XAPO_LOCKFORPROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::PartialEq for XAPO_LOCKFORPROCESS_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::Eq for XAPO_LOCKFORPROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Media_Multimedia")]
unsafe impl ::windows::runtime::Abi for XAPO_LOCKFORPROCESS_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const XAPO_MAX_CHANNELS: u32 = 64u32;
pub const XAPO_MAX_FRAMERATE: u32 = 200000u32;
pub const XAPO_MIN_CHANNELS: u32 = 1u32;
pub const XAPO_MIN_FRAMERATE: u32 = 1000u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct XAPO_PROCESS_BUFFER_PARAMETERS {
    pub pBuffer: *mut ::std::ffi::c_void,
    pub BufferFlags: XAPO_BUFFER_FLAGS,
    pub ValidFrameCount: u32,
}
impl XAPO_PROCESS_BUFFER_PARAMETERS {}
impl ::std::default::Default for XAPO_PROCESS_BUFFER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for XAPO_PROCESS_BUFFER_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for XAPO_PROCESS_BUFFER_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for XAPO_PROCESS_BUFFER_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct XAPO_REGISTRATION_PROPERTIES {
    pub clsid: ::windows::runtime::GUID,
    pub FriendlyName: [u16; 256],
    pub CopyrightInfo: [u16; 256],
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub Flags: u32,
    pub MinInputBufferCount: u32,
    pub MaxInputBufferCount: u32,
    pub MinOutputBufferCount: u32,
    pub MaxOutputBufferCount: u32,
}
impl XAPO_REGISTRATION_PROPERTIES {}
impl ::std::default::Default for XAPO_REGISTRATION_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for XAPO_REGISTRATION_PROPERTIES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for XAPO_REGISTRATION_PROPERTIES {}
unsafe impl ::windows::runtime::Abi for XAPO_REGISTRATION_PROPERTIES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const XAPO_REGISTRATION_STRING_LENGTH: u32 = 256u32;
pub const XAUDIO2FX_REVERB_DEFAULT_7POINT1_REAR_DELAY: u32 = 20u32;
pub const XAUDIO2FX_REVERB_DEFAULT_7POINT1_SIDE_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_DECAY_TIME: f32 = 1f32;
pub const XAUDIO2FX_REVERB_DEFAULT_DENSITY: f32 = 100f32;
pub const XAUDIO2FX_REVERB_DEFAULT_DISABLE_LATE_FIELD: u32 = 0u32;
pub const XAUDIO2FX_REVERB_DEFAULT_EARLY_DIFFUSION: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_HIGH_EQ_CUTOFF: u32 = 4u32;
pub const XAUDIO2FX_REVERB_DEFAULT_HIGH_EQ_GAIN: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_LATE_DIFFUSION: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_LOW_EQ_CUTOFF: u32 = 4u32;
pub const XAUDIO2FX_REVERB_DEFAULT_LOW_EQ_GAIN: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_POSITION: u32 = 6u32;
pub const XAUDIO2FX_REVERB_DEFAULT_POSITION_MATRIX: u32 = 27u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REAR_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REFLECTIONS_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REFLECTIONS_GAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_REVERB_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REVERB_GAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_FREQ: f32 = 5000f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_HF: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_MAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_SIZE: f32 = 100f32;
pub const XAUDIO2FX_REVERB_DEFAULT_WET_DRY_MIX: f32 = 100f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    pub WetDryMix: f32,
    pub Room: i32,
    pub RoomHF: i32,
    pub RoomRolloffFactor: f32,
    pub DecayTime: f32,
    pub DecayHFRatio: f32,
    pub Reflections: i32,
    pub ReflectionsDelay: f32,
    pub Reverb: i32,
    pub ReverbDelay: f32,
    pub Diffusion: f32,
    pub Density: f32,
    pub HFReference: f32,
}
impl XAUDIO2FX_REVERB_I3DL2_PARAMETERS {}
impl ::std::default::Default for XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for XAUDIO2FX_REVERB_I3DL2_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const XAUDIO2FX_REVERB_MAX_7POINT1_REAR_DELAY: u32 = 20u32;
pub const XAUDIO2FX_REVERB_MAX_7POINT1_SIDE_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_MAX_DENSITY: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MAX_DIFFUSION: u32 = 15u32;
pub const XAUDIO2FX_REVERB_MAX_FRAMERATE: u32 = 48000u32;
pub const XAUDIO2FX_REVERB_MAX_HIGH_EQ_CUTOFF: u32 = 14u32;
pub const XAUDIO2FX_REVERB_MAX_HIGH_EQ_GAIN: u32 = 8u32;
pub const XAUDIO2FX_REVERB_MAX_LOW_EQ_CUTOFF: u32 = 9u32;
pub const XAUDIO2FX_REVERB_MAX_LOW_EQ_GAIN: u32 = 12u32;
pub const XAUDIO2FX_REVERB_MAX_POSITION: u32 = 30u32;
pub const XAUDIO2FX_REVERB_MAX_REAR_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_MAX_REFLECTIONS_DELAY: u32 = 300u32;
pub const XAUDIO2FX_REVERB_MAX_REFLECTIONS_GAIN: f32 = 20f32;
pub const XAUDIO2FX_REVERB_MAX_REVERB_DELAY: u32 = 85u32;
pub const XAUDIO2FX_REVERB_MAX_REVERB_GAIN: f32 = 20f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_FREQ: f32 = 20000f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_HF: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_MAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_SIZE: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MAX_WET_DRY_MIX: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MIN_7POINT1_REAR_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_7POINT1_SIDE_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_DECAY_TIME: f32 = 0.1f32;
pub const XAUDIO2FX_REVERB_MIN_DENSITY: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MIN_DIFFUSION: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_FRAMERATE: u32 = 20000u32;
pub const XAUDIO2FX_REVERB_MIN_HIGH_EQ_CUTOFF: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_HIGH_EQ_GAIN: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_LOW_EQ_CUTOFF: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_LOW_EQ_GAIN: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_POSITION: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REAR_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REFLECTIONS_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REFLECTIONS_GAIN: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_REVERB_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REVERB_GAIN: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_FREQ: f32 = 20f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_HF: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_MAIN: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_SIZE: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MIN_WET_DRY_MIX: f32 = 0f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct XAUDIO2FX_REVERB_PARAMETERS {
    pub WetDryMix: f32,
    pub ReflectionsDelay: u32,
    pub ReverbDelay: u8,
    pub RearDelay: u8,
    pub SideDelay: u8,
    pub PositionLeft: u8,
    pub PositionRight: u8,
    pub PositionMatrixLeft: u8,
    pub PositionMatrixRight: u8,
    pub EarlyDiffusion: u8,
    pub LateDiffusion: u8,
    pub LowEQGain: u8,
    pub LowEQCutoff: u8,
    pub HighEQGain: u8,
    pub HighEQCutoff: u8,
    pub RoomFilterFreq: f32,
    pub RoomFilterMain: f32,
    pub RoomFilterHF: f32,
    pub ReflectionsGain: f32,
    pub ReverbGain: f32,
    pub DecayTime: f32,
    pub Density: f32,
    pub RoomSize: f32,
    pub DisableLateField: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl XAUDIO2FX_REVERB_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for XAUDIO2FX_REVERB_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for XAUDIO2FX_REVERB_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for XAUDIO2FX_REVERB_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for XAUDIO2FX_REVERB_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct XAUDIO2FX_VOLUMEMETER_LEVELS {
    pub pPeakLevels: *mut f32,
    pub pRMSLevels: *mut f32,
    pub ChannelCount: u32,
}
impl XAUDIO2FX_VOLUMEMETER_LEVELS {}
impl ::std::default::Default for XAUDIO2FX_VOLUMEMETER_LEVELS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for XAUDIO2FX_VOLUMEMETER_LEVELS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for XAUDIO2FX_VOLUMEMETER_LEVELS {}
unsafe impl ::windows::runtime::Abi for XAUDIO2FX_VOLUMEMETER_LEVELS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const XAUDIO2_1024_QUANTUM: u32 = 32768u32;
pub const XAUDIO2_ANY_PROCESSOR: u32 = 4294967295u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct XAUDIO2_BUFFER {
    pub Flags: u32,
    pub AudioBytes: u32,
    pub pAudioData: *mut u8,
    pub PlayBegin: u32,
    pub PlayLength: u32,
    pub LoopBegin: u32,
    pub LoopLength: u32,
    pub LoopCount: u32,
    pub pContext: *mut ::std::ffi::c_void,
}
impl XAUDIO2_BUFFER {}
impl ::std::default::Default for XAUDIO2_BUFFER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for XAUDIO2_BUFFER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for XAUDIO2_BUFFER {}
unsafe impl ::windows::runtime::Abi for XAUDIO2_BUFFER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct XAUDIO2_BUFFER_WMA {
    pub pDecodedPacketCumulativeBytes: *mut u32,
    pub PacketCount: u32,
}
impl XAUDIO2_BUFFER_WMA {}
impl ::std::default::Default for XAUDIO2_BUFFER_WMA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for XAUDIO2_BUFFER_WMA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for XAUDIO2_BUFFER_WMA {}
unsafe impl ::windows::runtime::Abi for XAUDIO2_BUFFER_WMA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const XAUDIO2_COMMIT_ALL: u32 = 0u32;
pub const XAUDIO2_COMMIT_NOW: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct XAUDIO2_DEBUG_CONFIGURATION {
    pub TraceMask: u32,
    pub BreakMask: u32,
    pub LogThreadID: super::super::super::Foundation::BOOL,
    pub LogFileline: super::super::super::Foundation::BOOL,
    pub LogFunctionName: super::super::super::Foundation::BOOL,
    pub LogTiming: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl XAUDIO2_DEBUG_CONFIGURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for XAUDIO2_DEBUG_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for XAUDIO2_DEBUG_CONFIGURATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for XAUDIO2_DEBUG_CONFIGURATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for XAUDIO2_DEBUG_CONFIGURATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const XAUDIO2_DEBUG_ENGINE: u32 = 1u32;
pub const XAUDIO2_DEFAULT_CHANNELS: u32 = 0u32;
pub const XAUDIO2_DEFAULT_FILTER_FREQUENCY: f32 = 1f32;
pub const XAUDIO2_DEFAULT_FILTER_ONEOVERQ: f32 = 1f32;
pub const XAUDIO2_DEFAULT_FREQ_RATIO: f32 = 2f32;
pub const XAUDIO2_DEFAULT_PROCESSOR: u32 = 1u32;
pub const XAUDIO2_DEFAULT_SAMPLERATE: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct XAUDIO2_EFFECT_CHAIN {
    pub EffectCount: u32,
    pub pEffectDescriptors: *mut XAUDIO2_EFFECT_DESCRIPTOR,
}
#[cfg(feature = "Win32_Foundation")]
impl XAUDIO2_EFFECT_CHAIN {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for XAUDIO2_EFFECT_CHAIN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for XAUDIO2_EFFECT_CHAIN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for XAUDIO2_EFFECT_CHAIN {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for XAUDIO2_EFFECT_CHAIN {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for XAUDIO2_EFFECT_DESCRIPTOR {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct XAUDIO2_EFFECT_DESCRIPTOR {
    pub pEffect: ::std::option::Option<::windows::runtime::IUnknown>,
    pub InitialState: super::super::super::Foundation::BOOL,
    pub OutputChannels: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl XAUDIO2_EFFECT_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for XAUDIO2_EFFECT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for XAUDIO2_EFFECT_DESCRIPTOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for XAUDIO2_EFFECT_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for XAUDIO2_EFFECT_DESCRIPTOR {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const XAUDIO2_END_OF_STREAM: u32 = 64u32;
pub const XAUDIO2_E_DEVICE_INVALIDATED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2003435516i32 as _);
pub const XAUDIO2_E_INVALID_CALL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2003435519i32 as _);
pub const XAUDIO2_E_XAPO_CREATION_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2003435517i32 as _);
pub const XAUDIO2_E_XMA_DECODER_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2003435518i32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct XAUDIO2_FILTER_PARAMETERS {
    pub Type: XAUDIO2_FILTER_TYPE,
    pub Frequency: f32,
    pub OneOverQ: f32,
}
impl XAUDIO2_FILTER_PARAMETERS {}
impl ::std::default::Default for XAUDIO2_FILTER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for XAUDIO2_FILTER_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for XAUDIO2_FILTER_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for XAUDIO2_FILTER_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct XAUDIO2_FILTER_TYPE(pub i32);
pub const LowPassFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(0i32);
pub const BandPassFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(1i32);
pub const HighPassFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(2i32);
pub const NotchFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(3i32);
pub const LowPassOnePoleFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(4i32);
pub const HighPassOnePoleFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(5i32);
impl ::std::convert::From<i32> for XAUDIO2_FILTER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XAUDIO2_FILTER_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const XAUDIO2_LOG_API_CALLS: u32 = 16u32;
pub const XAUDIO2_LOG_DETAIL: u32 = 8u32;
pub const XAUDIO2_LOG_ERRORS: u32 = 1u32;
pub const XAUDIO2_LOG_FUNC_CALLS: u32 = 32u32;
pub const XAUDIO2_LOG_INFO: u32 = 4u32;
pub const XAUDIO2_LOG_LOCKS: u32 = 128u32;
pub const XAUDIO2_LOG_MEMORY: u32 = 256u32;
pub const XAUDIO2_LOG_STREAMING: u32 = 4096u32;
pub const XAUDIO2_LOG_TIMING: u32 = 64u32;
pub const XAUDIO2_LOG_WARNINGS: u32 = 2u32;
pub const XAUDIO2_LOOP_INFINITE: u32 = 255u32;
pub const XAUDIO2_MAX_AUDIO_CHANNELS: u32 = 64u32;
pub const XAUDIO2_MAX_BUFFERS_SYSTEM: u32 = 2u32;
pub const XAUDIO2_MAX_BUFFER_BYTES: u32 = 2147483648u32;
pub const XAUDIO2_MAX_FILTER_FREQUENCY: f32 = 1f32;
pub const XAUDIO2_MAX_FILTER_ONEOVERQ: f32 = 1.5f32;
pub const XAUDIO2_MAX_FREQ_RATIO: f32 = 1024f32;
pub const XAUDIO2_MAX_INSTANCES: u32 = 8u32;
pub const XAUDIO2_MAX_LOOP_COUNT: u32 = 254u32;
pub const XAUDIO2_MAX_QUEUED_BUFFERS: u32 = 64u32;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO: u32 = 600000u32;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL: u32 = 300000u32;
pub const XAUDIO2_MAX_SAMPLE_RATE: u32 = 200000u32;
pub const XAUDIO2_MAX_VOLUME_LEVEL: f32 = 16777216f32;
pub const XAUDIO2_MIN_SAMPLE_RATE: u32 = 1000u32;
pub const XAUDIO2_NO_LOOP_REGION: u32 = 0u32;
pub const XAUDIO2_NO_VIRTUAL_AUDIO_CLIENT: u32 = 65536u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct XAUDIO2_PERFORMANCE_DATA {
    pub AudioCyclesSinceLastQuery: u64,
    pub TotalCyclesSinceLastQuery: u64,
    pub MinimumCyclesPerQuantum: u32,
    pub MaximumCyclesPerQuantum: u32,
    pub MemoryUsageInBytes: u32,
    pub CurrentLatencyInSamples: u32,
    pub GlitchesSinceEngineStarted: u32,
    pub ActiveSourceVoiceCount: u32,
    pub TotalSourceVoiceCount: u32,
    pub ActiveSubmixVoiceCount: u32,
    pub ActiveResamplerCount: u32,
    pub ActiveMatrixMixCount: u32,
    pub ActiveXmaSourceVoices: u32,
    pub ActiveXmaStreams: u32,
}
impl XAUDIO2_PERFORMANCE_DATA {}
impl ::std::default::Default for XAUDIO2_PERFORMANCE_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for XAUDIO2_PERFORMANCE_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for XAUDIO2_PERFORMANCE_DATA {}
unsafe impl ::windows::runtime::Abi for XAUDIO2_PERFORMANCE_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const XAUDIO2_PLAY_TAILS: u32 = 32u32;
pub const XAUDIO2_QUANTUM_DENOMINATOR: u32 = 100u32;
pub const XAUDIO2_QUANTUM_NUMERATOR: u32 = 1u32;
impl ::std::clone::Clone for XAUDIO2_SEND_DESCRIPTOR {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
pub struct XAUDIO2_SEND_DESCRIPTOR {
    pub Flags: u32,
    pub pOutputVoice: ::std::option::Option<IXAudio2Voice>,
}
impl XAUDIO2_SEND_DESCRIPTOR {}
impl ::std::default::Default for XAUDIO2_SEND_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for XAUDIO2_SEND_DESCRIPTOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for XAUDIO2_SEND_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for XAUDIO2_SEND_DESCRIPTOR {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const XAUDIO2_SEND_USEFILTER: u32 = 128u32;
pub const XAUDIO2_STOP_ENGINE_WHEN_IDLE: u32 = 8192u32;
pub const XAUDIO2_USE_DEFAULT_PROCESSOR: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct XAUDIO2_VOICE_DETAILS {
    pub CreationFlags: u32,
    pub ActiveFlags: u32,
    pub InputChannels: u32,
    pub InputSampleRate: u32,
}
impl XAUDIO2_VOICE_DETAILS {}
impl ::std::default::Default for XAUDIO2_VOICE_DETAILS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for XAUDIO2_VOICE_DETAILS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for XAUDIO2_VOICE_DETAILS {}
unsafe impl ::windows::runtime::Abi for XAUDIO2_VOICE_DETAILS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const XAUDIO2_VOICE_NOPITCH: u32 = 2u32;
pub const XAUDIO2_VOICE_NOSAMPLESPLAYED: u32 = 256u32;
pub const XAUDIO2_VOICE_NOSRC: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct XAUDIO2_VOICE_SENDS {
    pub SendCount: u32,
    pub pSends: *mut XAUDIO2_SEND_DESCRIPTOR,
}
impl XAUDIO2_VOICE_SENDS {}
impl ::std::default::Default for XAUDIO2_VOICE_SENDS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for XAUDIO2_VOICE_SENDS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for XAUDIO2_VOICE_SENDS {}
unsafe impl ::windows::runtime::Abi for XAUDIO2_VOICE_SENDS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct XAUDIO2_VOICE_STATE {
    pub pCurrentBufferContext: *mut ::std::ffi::c_void,
    pub BuffersQueued: u32,
    pub SamplesPlayed: u64,
}
impl XAUDIO2_VOICE_STATE {}
impl ::std::default::Default for XAUDIO2_VOICE_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for XAUDIO2_VOICE_STATE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for XAUDIO2_VOICE_STATE {}
unsafe impl ::windows::runtime::Abi for XAUDIO2_VOICE_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const XAUDIO2_VOICE_USEFILTER: u32 = 8u32;
#[inline]
pub unsafe fn XAudio2CreateWithVersionInfo(
    ppxaudio2: *mut ::std::option::Option<IXAudio2>,
    flags: u32,
    xaudio2processor: u32,
    ntddiversion: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XAudio2CreateWithVersionInfo(
                ppxaudio2: *mut ::windows::runtime::RawPtr,
                flags: u32,
                xaudio2processor: u32,
                ntddiversion: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        XAudio2CreateWithVersionInfo(
            ::std::mem::transmute(ppxaudio2),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(xaudio2processor),
            ::std::mem::transmute(ntddiversion),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}

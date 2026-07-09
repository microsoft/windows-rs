pub const ACM_MPEG_COPYRIGHT: u32 = 2;
pub const ACM_MPEG_DUALCHANNEL: u32 = 4;
pub const ACM_MPEG_ID_MPEG1: u32 = 16;
pub const ACM_MPEG_JOINTSTEREO: u32 = 2;
pub const ACM_MPEG_LAYER1: u32 = 1;
pub const ACM_MPEG_LAYER2: u32 = 2;
pub const ACM_MPEG_LAYER3: u32 = 4;
pub const ACM_MPEG_ORIGINALHOME: u32 = 4;
pub const ACM_MPEG_PRIVATEBIT: u32 = 1;
pub const ACM_MPEG_PROTECTIONBIT: u32 = 8;
pub const ACM_MPEG_SINGLECHANNEL: u32 = 8;
pub const ACM_MPEG_STEREO: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ADPCMCOEFSET {
    pub iCoef1: i16,
    pub iCoef2: i16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct ADPCMEWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy)]
pub struct ADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
    pub wNumCoef: u16,
    pub aCoef: [ADPCMCOEFSET; 0],
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct APTXWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct AUDIOFILE_AF10WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct AUDIOFILE_AF36WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
pub const AVIIF_CONTROLFRAME: u32 = 512;
pub const BICOMP_CREATIVEYUV: u32 = 1987410275;
pub const BICOMP_IBMPHOTOMOTION: u32 = 1330464848;
pub const BICOMP_IBMULTIMOTION: u32 = 1230261333;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct CONTRESCR10WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct CONTRESVQLPCWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct CREATIVEADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wRevision: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct CREATIVEFASTSPEECH10WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wRevision: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct CREATIVEFASTSPEECH8WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wRevision: u16,
}
pub const CRYSTAL_NET_SFM_CODEC: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct CSIMAADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DIALOGICOKIADPCMWAVEFORMAT {
    pub ewf: super::mmeapi::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DIGIADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DIGIFIXWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DIGIREALWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DIGISTDWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DOLBYAC2WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub nAuxBitsCode: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DRMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wReserved: u16,
    pub ulContentId: u32,
    pub wfxSecure: super::mmeapi::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DVIADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct ECHOSC1WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ECHOWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
    pub dwDelay: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_wingdi")]
#[derive(Clone, Copy, Default)]
pub struct EXBMINFOHEADER {
    pub bmi: super::wingdi::BITMAPINFOHEADER,
    pub biExtDataOffset: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct FMTOWNS_SND_WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wRevision: u16,
}
pub const FOURCC_RDSP: u32 = 1347634258;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct G721_ADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub nAuxBlockSize: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct G723_ADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub cbExtraSize: u16,
    pub nAuxBlockSize: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct GSM610WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy)]
pub struct HEAACWAVEFORMAT {
    pub wfInfo: HEAACWAVEINFO,
    pub pbAudioSpecificConfig: [u8; 1],
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for HEAACWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct HEAACWAVEINFO {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wPayloadType: u16,
    pub wAudioProfileLevelIndication: u16,
    pub wStructType: u16,
    pub wReserved1: u16,
    pub dwReserved2: u32,
}
pub const ICTYPE_AUDIO: u32 = 1667528033;
pub const ICTYPE_VIDEO: u32 = 1667524982;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct IMAADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
pub const JIFMK_00: u32 = 65280;
pub const JIFMK_APP0: u32 = 65504;
pub const JIFMK_APP1: u32 = 65505;
pub const JIFMK_APP2: u32 = 65506;
pub const JIFMK_APP3: u32 = 65507;
pub const JIFMK_APP4: u32 = 65508;
pub const JIFMK_APP5: u32 = 65509;
pub const JIFMK_APP6: u32 = 65510;
pub const JIFMK_APP7: u32 = 65511;
pub const JIFMK_COM: u32 = 65534;
pub const JIFMK_DAC: u32 = 65484;
pub const JIFMK_DHP: u32 = 65502;
pub const JIFMK_DHT: u32 = 65476;
pub const JIFMK_DNL: u32 = 65500;
pub const JIFMK_DQT: u32 = 65499;
pub const JIFMK_DRI: u32 = 65501;
pub const JIFMK_EOI: u32 = 65497;
pub const JIFMK_EXP: u32 = 65503;
pub const JIFMK_FF: u32 = 65535;
pub const JIFMK_JPG: u32 = 65480;
pub const JIFMK_JPG0: u32 = 65520;
pub const JIFMK_JPG1: u32 = 65521;
pub const JIFMK_JPG10: u32 = 65530;
pub const JIFMK_JPG11: u32 = 65531;
pub const JIFMK_JPG12: u32 = 65532;
pub const JIFMK_JPG13: u32 = 65533;
pub const JIFMK_JPG2: u32 = 65522;
pub const JIFMK_JPG3: u32 = 65523;
pub const JIFMK_JPG4: u32 = 65524;
pub const JIFMK_JPG5: u32 = 65525;
pub const JIFMK_JPG6: u32 = 65526;
pub const JIFMK_JPG7: u32 = 65527;
pub const JIFMK_JPG8: u32 = 65528;
pub const JIFMK_JPG9: u32 = 65529;
pub const JIFMK_RES: u32 = 65282;
pub const JIFMK_RST0: u32 = 65488;
pub const JIFMK_RST1: u32 = 65489;
pub const JIFMK_RST2: u32 = 65490;
pub const JIFMK_RST3: u32 = 65491;
pub const JIFMK_RST4: u32 = 65492;
pub const JIFMK_RST5: u32 = 65493;
pub const JIFMK_RST6: u32 = 65494;
pub const JIFMK_RST7: u32 = 65495;
pub const JIFMK_SOF0: u32 = 65472;
pub const JIFMK_SOF1: u32 = 65473;
pub const JIFMK_SOF10: u32 = 65482;
pub const JIFMK_SOF11: u32 = 65483;
pub const JIFMK_SOF13: u32 = 65485;
pub const JIFMK_SOF14: u32 = 65486;
pub const JIFMK_SOF15: u32 = 65487;
pub const JIFMK_SOF2: u32 = 65474;
pub const JIFMK_SOF3: u32 = 65475;
pub const JIFMK_SOF5: u32 = 65477;
pub const JIFMK_SOF6: u32 = 65478;
pub const JIFMK_SOF7: u32 = 65479;
pub const JIFMK_SOF9: u32 = 65481;
pub const JIFMK_SOI: u32 = 65496;
pub const JIFMK_SOS: u32 = 65498;
pub const JIFMK_TEM: u32 = 65281;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct JPEGINFOHEADER {
    pub JPEGSize: u32,
    pub JPEGProcess: u32,
    pub JPEGColorSpaceID: u32,
    pub JPEGBitsPerSample: u32,
    pub JPEGHSubSampling: u32,
    pub JPEGVSubSampling: u32,
}
pub const JPEG_DIB: u32 = 1195724874;
pub const JPEG_PROCESS_BASELINE: u32 = 0;
pub const JPEG_RGB: u32 = 3;
pub const JPEG_Y: u32 = 1;
pub const JPEG_YCbCr: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KSDATAFORMAT_SUBTYPE_IEEE_FLOAT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KSDATAFORMAT_SUBTYPE_PCM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KSDATAFORMAT_SUBTYPE_WAVEFORMATEX(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADPCMCOEFSET(pub *mut ADPCMCOEFSET);
impl LPADPCMCOEFSET {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPADPCMCOEFSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADPCMEWAVEFORMAT(pub *mut ADPCMEWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPADPCMEWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPADPCMEWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADPCMWAVEFORMAT(pub *mut ADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPAPTXWAVEFORMAT(pub *mut APTXWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPAPTXWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPAPTXWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPAUDIOFILE_AF10WAVEFORMAT(pub *mut AUDIOFILE_AF10WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPAUDIOFILE_AF10WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPAUDIOFILE_AF10WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPAUDIOFILE_AF36WAVEFORMAT(pub *mut AUDIOFILE_AF36WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPAUDIOFILE_AF36WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPAUDIOFILE_AF36WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCONTRESCR10WAVEFORMAT(pub *mut CONTRESCR10WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPCONTRESCR10WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPCONTRESCR10WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCONTRESVQLPCWAVEFORMAT(pub *mut CONTRESVQLPCWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPCONTRESVQLPCWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPCONTRESVQLPCWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCREATIVEADPCMWAVEFORMAT(pub *mut CREATIVEADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPCREATIVEADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPCREATIVEADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCREATIVEFASTSPEECH10WAVEFORMAT(pub *mut CREATIVEFASTSPEECH10WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPCREATIVEFASTSPEECH10WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPCREATIVEFASTSPEECH10WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCREATIVEFASTSPEECH8WAVEFORMAT(pub *mut CREATIVEFASTSPEECH8WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPCREATIVEFASTSPEECH8WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPCREATIVEFASTSPEECH8WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCSIMAADPCMWAVEFORMAT(pub *mut CSIMAADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPCSIMAADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPCSIMAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDIALOGICOKIADPCMWAVEFORMAT(pub *mut DIALOGICOKIADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPDIALOGICOKIADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPDIALOGICOKIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDIGIADPCMWAVEFORMAT(pub *mut DIGIADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPDIGIADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPDIGIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDIGIFIXWAVEFORMAT(pub *mut DIGIFIXWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPDIGIFIXWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPDIGIFIXWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDIGIREALWAVEFORMAT(pub *mut DIGIREALWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPDIGIREALWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPDIGIREALWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDIGISTDWAVEFORMAT(pub *mut DIGISTDWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPDIGISTDWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPDIGISTDWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDRMWAVEFORMAT(pub *mut DRMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPDRMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPDRMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDVIADPCMWAVEFORMAT(pub *mut DVIADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPDVIADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPDVIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPECHOSC1WAVEFORMAT(pub *mut ECHOSC1WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPECHOSC1WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPECHOSC1WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPECHOWAVEFILTER(pub *mut ECHOWAVEFILTER);
impl LPECHOWAVEFILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPECHOWAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPFMTOWNS_SND_WAVEFORMAT(pub *mut FMTOWNS_SND_WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPFMTOWNS_SND_WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPFMTOWNS_SND_WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPG721_ADPCMWAVEFORMAT(pub *mut G721_ADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPG721_ADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPG721_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPG723_ADPCMWAVEFORMAT(pub *mut G723_ADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPG723_ADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPG723_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGSM610WAVEFORMAT(pub *mut GSM610WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPGSM610WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPGSM610WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHEAACWAVEFORMAT(pub *mut HEAACWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPHEAACWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPHEAACWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPHEAACWAVEINFO(pub *mut HEAACWAVEINFO);
#[cfg(feature = "Win32_mmeapi")]
impl LPHEAACWAVEINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPHEAACWAVEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPIMAADPCMWAVEFORMAT(pub *mut IMAADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPIMAADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPIMAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMEDIASPACEADPCMWAVEFORMAT(pub *mut MEDIASPACEADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPMEDIASPACEADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPMEDIASPACEADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMPEG1WAVEFORMAT(pub *mut MPEG1WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPMPEG1WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPMPEG1WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMPEGLAYER3WAVEFORMAT(pub *mut MPEGLAYER3WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPMPEGLAYER3WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPMPEGLAYER3WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMSAUDIO1WAVEFORMAT(pub *mut MSAUDIO1WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPMSAUDIO1WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPMSAUDIO1WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNMS_VBXADPCMWAVEFORMAT(pub *mut NMS_VBXADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPNMS_VBXADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPNMS_VBXADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOLIADPCMWAVEFORMAT(pub *mut OLIADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPOLIADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPOLIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOLICELPWAVEFORMAT(pub *mut OLICELPWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPOLICELPWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPOLICELPWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOLIGSMWAVEFORMAT(pub *mut OLIGSMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPOLIGSMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPOLIGSMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOLIOPRWAVEFORMAT(pub *mut OLIOPRWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPOLIOPRWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPOLIOPRWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOLISBCWAVEFORMAT(pub *mut OLISBCWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPOLISBCWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPOLISBCWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSIERRAADPCMWAVEFORMAT(pub *mut SIERRAADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPSIERRAADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPSIERRAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSONARCWAVEFORMAT(pub *mut SONARCWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPSONARCWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPSONARCWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTRUESPEECHWAVEFORMAT(pub *mut TRUESPEECHWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPTRUESPEECHWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPTRUESPEECHWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPVOLUMEWAVEFILTER(pub *mut VOLUMEWAVEFILTER);
impl LPVOLUMEWAVEFILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPVOLUMEWAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEFILTER(pub *mut WAVEFILTER);
impl LPWAVEFILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPWAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEFORMATIEEEFLOATEX(pub *mut WAVEFORMATIEEEFLOATEX);
#[cfg(feature = "Win32_mmeapi")]
impl LPWAVEFORMATIEEEFLOATEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPWAVEFORMATIEEEFLOATEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWAVEFORMATPCMEX(pub *mut WAVEFORMATPCMEX);
#[cfg(feature = "Win32_mmeapi")]
impl LPWAVEFORMATPCMEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPWAVEFORMATPCMEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWMAUDIO2WAVEFORMAT(pub *mut WMAUDIO2WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPWMAUDIO2WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPWMAUDIO2WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWMAUDIO3WAVEFORMAT(pub *mut WMAUDIO3WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPWMAUDIO3WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPWMAUDIO3WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPYAMAHA_ADPCMWAVEFORMAT(pub *mut YAMAHA_ADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl LPYAMAHA_ADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for LPYAMAHA_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct MEDIASPACEADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wRevision: u16,
}
pub const MIXERCONTROL_CONTROLTYPE_SRS_MTS: u32 = 536936454;
pub const MIXERCONTROL_CONTROLTYPE_SRS_ONOFF: u32 = 536936455;
pub const MIXERCONTROL_CONTROLTYPE_SRS_SYNTHSELECT: u32 = 536936456;
pub const MJPG_DIB: u32 = 1196444237;
pub const MM_3COM: u32 = 260;
pub const MM_3COM_CB_MIXER: u32 = 1;
pub const MM_3COM_CB_WAVEIN: u32 = 2;
pub const MM_3COM_CB_WAVEOUT: u32 = 3;
pub const MM_3DFX: u32 = 262;
pub const MM_AARDVARK: u32 = 11;
pub const MM_AARDVARK_STUDIO12_WAVEIN: u32 = 2;
pub const MM_AARDVARK_STUDIO12_WAVEOUT: u32 = 1;
pub const MM_AARDVARK_STUDIO88_WAVEIN: u32 = 4;
pub const MM_AARDVARK_STUDIO88_WAVEOUT: u32 = 3;
pub const MM_ACTIVEVOICE: u32 = 225;
pub const MM_ACTIVEVOICE_ACM_VOXADPCM: u32 = 1;
pub const MM_ACULAB: u32 = 14;
pub const MM_ADDX: u32 = 118;
pub const MM_ADDX_PCTV_AUX_CD: u32 = 5;
pub const MM_ADDX_PCTV_AUX_LINE: u32 = 6;
pub const MM_ADDX_PCTV_DIGITALMIX: u32 = 1;
pub const MM_ADDX_PCTV_MIXER: u32 = 4;
pub const MM_ADDX_PCTV_WAVEIN: u32 = 2;
pub const MM_ADDX_PCTV_WAVEOUT: u32 = 3;
pub const MM_ADLACC: u32 = 91;
pub const MM_ADLIB: u32 = 9;
pub const MM_ADMOS: u32 = 235;
pub const MM_ADMOS_FM_SYNTH: u32 = 1;
pub const MM_ADMOS_QS3AMIDIIN: u32 = 3;
pub const MM_ADMOS_QS3AMIDIOUT: u32 = 2;
pub const MM_ADMOS_QS3AWAVEIN: u32 = 5;
pub const MM_ADMOS_QS3AWAVEOUT: u32 = 4;
pub const MM_AHEAD: u32 = 77;
pub const MM_AHEAD_GENERIC: u32 = 4;
pub const MM_AHEAD_MULTISOUND: u32 = 1;
pub const MM_AHEAD_PROAUDIO: u32 = 3;
pub const MM_AHEAD_SOUNDBLASTER: u32 = 2;
pub const MM_ALARIS: u32 = 174;
pub const MM_ALDIGITAL: u32 = 143;
pub const MM_ALESIS: u32 = 243;
pub const MM_ALGOVISION: u32 = 266;
pub const MM_ALGOVISION_VB80AUX: u32 = 4;
pub const MM_ALGOVISION_VB80AUX2: u32 = 5;
pub const MM_ALGOVISION_VB80MIXER: u32 = 3;
pub const MM_ALGOVISION_VB80WAVEIN: u32 = 2;
pub const MM_ALGOVISION_VB80WAVEOUT: u32 = 1;
pub const MM_AMD: u32 = 146;
pub const MM_AMD_INTERWAVE_AUX1: u32 = 10;
pub const MM_AMD_INTERWAVE_AUX2: u32 = 11;
pub const MM_AMD_INTERWAVE_AUX_CD: u32 = 13;
pub const MM_AMD_INTERWAVE_AUX_MIC: u32 = 12;
pub const MM_AMD_INTERWAVE_EX_CD: u32 = 7;
pub const MM_AMD_INTERWAVE_EX_TELEPHONY: u32 = 16;
pub const MM_AMD_INTERWAVE_JOYSTICK: u32 = 6;
pub const MM_AMD_INTERWAVE_MIDIIN: u32 = 8;
pub const MM_AMD_INTERWAVE_MIDIOUT: u32 = 9;
pub const MM_AMD_INTERWAVE_MIXER1: u32 = 4;
pub const MM_AMD_INTERWAVE_MIXER2: u32 = 5;
pub const MM_AMD_INTERWAVE_MONO_IN: u32 = 14;
pub const MM_AMD_INTERWAVE_MONO_OUT: u32 = 15;
pub const MM_AMD_INTERWAVE_STEREO_ENHANCED: u32 = 19;
pub const MM_AMD_INTERWAVE_SYNTH: u32 = 3;
pub const MM_AMD_INTERWAVE_WAVEIN: u32 = 1;
pub const MM_AMD_INTERWAVE_WAVEOUT: u32 = 2;
pub const MM_AMD_INTERWAVE_WAVEOUT_BASE: u32 = 17;
pub const MM_AMD_INTERWAVE_WAVEOUT_TREBLE: u32 = 18;
pub const MM_ANALOGDEVICES: u32 = 252;
pub const MM_ANTEX: u32 = 31;
pub const MM_ANTEX_AUDIOPORT22_FEEDTHRU: u32 = 9;
pub const MM_ANTEX_AUDIOPORT22_WAVEIN: u32 = 7;
pub const MM_ANTEX_AUDIOPORT22_WAVEOUT: u32 = 8;
pub const MM_ANTEX_SX12_WAVEIN: u32 = 1;
pub const MM_ANTEX_SX12_WAVEOUT: u32 = 2;
pub const MM_ANTEX_SX15_WAVEIN: u32 = 3;
pub const MM_ANTEX_SX15_WAVEOUT: u32 = 4;
pub const MM_ANTEX_VP625_WAVEIN: u32 = 5;
pub const MM_ANTEX_VP625_WAVEOUT: u32 = 6;
pub const MM_APICOM: u32 = 116;
pub const MM_APPLE: u32 = 99;
pub const MM_APPS: u32 = 42;
pub const MM_APT: u32 = 56;
pub const MM_APT_ACE100CD: u32 = 1;
pub const MM_ARRAY: u32 = 231;
pub const MM_ARTISOFT: u32 = 20;
pub const MM_ARTISOFT_SBWAVEIN: u32 = 1;
pub const MM_ARTISOFT_SBWAVEOUT: u32 = 2;
pub const MM_AST: u32 = 64;
pub const MM_AST_MODEMWAVE_WAVEIN: u32 = 13;
pub const MM_AST_MODEMWAVE_WAVEOUT: u32 = 14;
pub const MM_ATI: u32 = 27;
pub const MM_ATT: u32 = 185;
pub const MM_ATT_G729A: u32 = 1;
pub const MM_ATT_MICROELECTRONICS: u32 = 139;
pub const MM_AU8820_AUX: u32 = 21;
pub const MM_AU8820_MIDIIN: u32 = 23;
pub const MM_AU8820_MIDIOUT: u32 = 22;
pub const MM_AU8820_MIXER: u32 = 20;
pub const MM_AU8820_SYNTH: u32 = 17;
pub const MM_AU8820_WAVEIN: u32 = 19;
pub const MM_AU8820_WAVEOUT: u32 = 18;
pub const MM_AU8830_AUX: u32 = 37;
pub const MM_AU8830_MIDIIN: u32 = 39;
pub const MM_AU8830_MIDIOUT: u32 = 38;
pub const MM_AU8830_MIXER: u32 = 36;
pub const MM_AU8830_SYNTH: u32 = 33;
pub const MM_AU8830_WAVEIN: u32 = 35;
pub const MM_AU8830_WAVEOUT: u32 = 34;
pub const MM_AUDIOFILE: u32 = 47;
pub const MM_AUDIOPT: u32 = 74;
pub const MM_AUDIOSCIENCE: u32 = 217;
pub const MM_AURAVISION: u32 = 80;
pub const MM_AUREAL: u32 = 181;
pub const MM_AUREAL_AU8820: u32 = 16;
pub const MM_AUREAL_AU8830: u32 = 32;
pub const MM_AZTECH: u32 = 52;
pub const MM_AZTECH_AUX: u32 = 404;
pub const MM_AZTECH_AUX_CD: u32 = 401;
pub const MM_AZTECH_AUX_LINE: u32 = 402;
pub const MM_AZTECH_AUX_MIC: u32 = 403;
pub const MM_AZTECH_DSP16_FMSYNTH: u32 = 68;
pub const MM_AZTECH_DSP16_WAVEIN: u32 = 65;
pub const MM_AZTECH_DSP16_WAVEOUT: u32 = 66;
pub const MM_AZTECH_DSP16_WAVESYNTH: u32 = 70;
pub const MM_AZTECH_FMSYNTH: u32 = 20;
pub const MM_AZTECH_MIDIIN: u32 = 4;
pub const MM_AZTECH_MIDIOUT: u32 = 3;
pub const MM_AZTECH_MIXER: u32 = 21;
pub const MM_AZTECH_NOVA16_MIXER: u32 = 73;
pub const MM_AZTECH_NOVA16_WAVEIN: u32 = 71;
pub const MM_AZTECH_NOVA16_WAVEOUT: u32 = 72;
pub const MM_AZTECH_PRO16_FMSYNTH: u32 = 38;
pub const MM_AZTECH_PRO16_WAVEIN: u32 = 33;
pub const MM_AZTECH_PRO16_WAVEOUT: u32 = 34;
pub const MM_AZTECH_WASH16_MIXER: u32 = 76;
pub const MM_AZTECH_WASH16_WAVEIN: u32 = 74;
pub const MM_AZTECH_WASH16_WAVEOUT: u32 = 75;
pub const MM_AZTECH_WAVEIN: u32 = 17;
pub const MM_AZTECH_WAVEOUT: u32 = 18;
pub const MM_BCB: u32 = 192;
pub const MM_BCB_NETBOARD_10: u32 = 1;
pub const MM_BCB_TT75_10: u32 = 2;
pub const MM_BECUBED: u32 = 10;
pub const MM_BERCOS: u32 = 199;
pub const MM_BERCOS_MIXER: u32 = 2;
pub const MM_BERCOS_WAVEIN: u32 = 1;
pub const MM_BERCOS_WAVEOUT: u32 = 3;
pub const MM_BERKOM: u32 = 189;
pub const MM_BINTEC: u32 = 12;
pub const MM_BINTEC_TAPI_WAVE: u32 = 1;
pub const MM_BROOKTREE: u32 = 121;
pub const MM_BTV_AUX_CD: u32 = 8;
pub const MM_BTV_AUX_LINE: u32 = 6;
pub const MM_BTV_AUX_MIC: u32 = 7;
pub const MM_BTV_DIGITALIN: u32 = 9;
pub const MM_BTV_DIGITALOUT: u32 = 10;
pub const MM_BTV_MIDIIN: u32 = 3;
pub const MM_BTV_MIDIOUT: u32 = 4;
pub const MM_BTV_MIDISYNTH: u32 = 5;
pub const MM_BTV_MIDIWAVESTREAM: u32 = 11;
pub const MM_BTV_MIXER: u32 = 12;
pub const MM_BTV_WAVEIN: u32 = 1;
pub const MM_BTV_WAVEOUT: u32 = 2;
pub const MM_CANAM: u32 = 148;
pub const MM_CANAM_CBXWAVEIN: u32 = 2;
pub const MM_CANAM_CBXWAVEOUT: u32 = 1;
pub const MM_CANOPUS: u32 = 49;
pub const MM_CANOPUS_ACM_DVREX: u32 = 1;
pub const MM_CASIO: u32 = 162;
pub const MM_CASIO_LSG_MIDIOUT: u32 = 3;
pub const MM_CASIO_WP150_MIDIIN: u32 = 2;
pub const MM_CASIO_WP150_MIDIOUT: u32 = 1;
pub const MM_CAT: u32 = 41;
pub const MM_CAT_WAVEOUT: u32 = 1;
pub const MM_CDPC_AUX: u32 = 119;
pub const MM_CDPC_MIDIIN: u32 = 114;
pub const MM_CDPC_MIDIOUT: u32 = 113;
pub const MM_CDPC_MIXER: u32 = 118;
pub const MM_CDPC_SYNTH: u32 = 115;
pub const MM_CDPC_WAVEIN: u32 = 117;
pub const MM_CDPC_WAVEOUT: u32 = 116;
pub const MM_CHROMATIC: u32 = 155;
pub const MM_CHROMATIC_M1: u32 = 1;
pub const MM_CHROMATIC_M1_AUX: u32 = 6;
pub const MM_CHROMATIC_M1_AUX_CD: u32 = 7;
pub const MM_CHROMATIC_M1_FMSYNTH: u32 = 4;
pub const MM_CHROMATIC_M1_MIDIIN: u32 = 8;
pub const MM_CHROMATIC_M1_MIDIOUT: u32 = 9;
pub const MM_CHROMATIC_M1_MIXER: u32 = 5;
pub const MM_CHROMATIC_M1_MPEGWAVEIN: u32 = 17;
pub const MM_CHROMATIC_M1_MPEGWAVEOUT: u32 = 18;
pub const MM_CHROMATIC_M1_WAVEIN: u32 = 2;
pub const MM_CHROMATIC_M1_WAVEOUT: u32 = 3;
pub const MM_CHROMATIC_M1_WTSYNTH: u32 = 16;
pub const MM_CHROMATIC_M2: u32 = 19;
pub const MM_CHROMATIC_M2_AUX: u32 = 24;
pub const MM_CHROMATIC_M2_AUX_CD: u32 = 25;
pub const MM_CHROMATIC_M2_FMSYNTH: u32 = 22;
pub const MM_CHROMATIC_M2_MIDIIN: u32 = 32;
pub const MM_CHROMATIC_M2_MIDIOUT: u32 = 33;
pub const MM_CHROMATIC_M2_MIXER: u32 = 23;
pub const MM_CHROMATIC_M2_MPEGWAVEIN: u32 = 35;
pub const MM_CHROMATIC_M2_MPEGWAVEOUT: u32 = 36;
pub const MM_CHROMATIC_M2_WAVEIN: u32 = 20;
pub const MM_CHROMATIC_M2_WAVEOUT: u32 = 21;
pub const MM_CHROMATIC_M2_WTSYNTH: u32 = 34;
pub const MM_CIRRUSLOGIC: u32 = 105;
pub const MM_COLORGRAPH: u32 = 179;
pub const MM_COMPAQ: u32 = 92;
pub const MM_COMPAQ_BB_WAVEAUX: u32 = 3;
pub const MM_COMPAQ_BB_WAVEIN: u32 = 1;
pub const MM_COMPAQ_BB_WAVEOUT: u32 = 2;
pub const MM_COMPUSIC: u32 = 89;
pub const MM_COMPUTER_FRIENDS: u32 = 45;
pub const MM_CONCEPTS: u32 = 108;
pub const MM_CONNECTIX: u32 = 158;
pub const MM_CONNECTIX_VIDEC_CODEC: u32 = 1;
pub const MM_CONTROLRES: u32 = 84;
pub const MM_COREDYNAMICS: u32 = 147;
pub const MM_COREDYNAMICS_DYNAGRAFX_VGA: u32 = 9;
pub const MM_COREDYNAMICS_DYNAGRAFX_WAVE_IN: u32 = 10;
pub const MM_COREDYNAMICS_DYNAGRAFX_WAVE_OUT: u32 = 11;
pub const MM_COREDYNAMICS_DYNAMIXHR: u32 = 1;
pub const MM_COREDYNAMICS_DYNASONIX_AUDIO_IN: u32 = 7;
pub const MM_COREDYNAMICS_DYNASONIX_AUDIO_OUT: u32 = 8;
pub const MM_COREDYNAMICS_DYNASONIX_MIDI_IN: u32 = 3;
pub const MM_COREDYNAMICS_DYNASONIX_MIDI_OUT: u32 = 4;
pub const MM_COREDYNAMICS_DYNASONIX_SYNTH: u32 = 2;
pub const MM_COREDYNAMICS_DYNASONIX_WAVE_IN: u32 = 5;
pub const MM_COREDYNAMICS_DYNASONIX_WAVE_OUT: u32 = 6;
pub const MM_CREATIVE: u32 = 2;
pub const MM_CREATIVE_AUX_CD: u32 = 401;
pub const MM_CREATIVE_AUX_LINE: u32 = 402;
pub const MM_CREATIVE_AUX_MASTER: u32 = 404;
pub const MM_CREATIVE_AUX_MIC: u32 = 403;
pub const MM_CREATIVE_AUX_MIDI: u32 = 407;
pub const MM_CREATIVE_AUX_PCSPK: u32 = 405;
pub const MM_CREATIVE_AUX_WAVE: u32 = 406;
pub const MM_CREATIVE_FMSYNTH_MONO: u32 = 301;
pub const MM_CREATIVE_FMSYNTH_STEREO: u32 = 302;
pub const MM_CREATIVE_MIDIIN: u32 = 202;
pub const MM_CREATIVE_MIDIOUT: u32 = 201;
pub const MM_CREATIVE_MIDI_AWE32: u32 = 303;
pub const MM_CREATIVE_PHNBLST_WAVEIN: u32 = 5;
pub const MM_CREATIVE_PHNBLST_WAVEOUT: u32 = 105;
pub const MM_CREATIVE_SB15_WAVEIN: u32 = 1;
pub const MM_CREATIVE_SB15_WAVEOUT: u32 = 101;
pub const MM_CREATIVE_SB16_MIXER: u32 = 409;
pub const MM_CREATIVE_SB20_WAVEIN: u32 = 2;
pub const MM_CREATIVE_SB20_WAVEOUT: u32 = 102;
pub const MM_CREATIVE_SBP16_WAVEIN: u32 = 4;
pub const MM_CREATIVE_SBP16_WAVEOUT: u32 = 104;
pub const MM_CREATIVE_SBPRO_MIXER: u32 = 408;
pub const MM_CREATIVE_SBPRO_WAVEIN: u32 = 3;
pub const MM_CREATIVE_SBPRO_WAVEOUT: u32 = 103;
pub const MM_CRYSTAL: u32 = 132;
pub const MM_CRYSTAL_CS4232_INPUTGAIN_AUX1: u32 = 13;
pub const MM_CRYSTAL_CS4232_INPUTGAIN_LOOP: u32 = 14;
pub const MM_CRYSTAL_CS4232_MIDIIN: u32 = 9;
pub const MM_CRYSTAL_CS4232_MIDIOUT: u32 = 10;
pub const MM_CRYSTAL_CS4232_WAVEAUX_AUX1: u32 = 4;
pub const MM_CRYSTAL_CS4232_WAVEAUX_AUX2: u32 = 5;
pub const MM_CRYSTAL_CS4232_WAVEAUX_LINE: u32 = 6;
pub const MM_CRYSTAL_CS4232_WAVEAUX_MASTER: u32 = 8;
pub const MM_CRYSTAL_CS4232_WAVEAUX_MONO: u32 = 7;
pub const MM_CRYSTAL_CS4232_WAVEIN: u32 = 1;
pub const MM_CRYSTAL_CS4232_WAVEMIXER: u32 = 3;
pub const MM_CRYSTAL_CS4232_WAVEOUT: u32 = 2;
pub const MM_CRYSTAL_NET: u32 = 154;
pub const MM_CRYSTAL_SOUND_FUSION_JOYSTICK: u32 = 26;
pub const MM_CRYSTAL_SOUND_FUSION_MIDIIN: u32 = 24;
pub const MM_CRYSTAL_SOUND_FUSION_MIDIOUT: u32 = 25;
pub const MM_CRYSTAL_SOUND_FUSION_MIXER: u32 = 23;
pub const MM_CRYSTAL_SOUND_FUSION_WAVEIN: u32 = 21;
pub const MM_CRYSTAL_SOUND_FUSION_WAVEOUT: u32 = 22;
pub const MM_CS: u32 = 242;
pub const MM_CYRIX: u32 = 6;
pub const MM_CYRIX_XAAUX: u32 = 6;
pub const MM_CYRIX_XAMIDIIN: u32 = 2;
pub const MM_CYRIX_XAMIDIOUT: u32 = 3;
pub const MM_CYRIX_XAMIXER: u32 = 7;
pub const MM_CYRIX_XASYNTH: u32 = 1;
pub const MM_CYRIX_XAWAVEIN: u32 = 4;
pub const MM_CYRIX_XAWAVEOUT: u32 = 5;
pub const MM_DATAFUSION: u32 = 196;
pub const MM_DATARAN: u32 = 232;
pub const MM_DDD: u32 = 151;
pub const MM_DDD_MIDILINK_MIDIIN: u32 = 1;
pub const MM_DDD_MIDILINK_MIDIOUT: u32 = 2;
pub const MM_DF_ACM_G726: u32 = 1;
pub const MM_DF_ACM_GSM610: u32 = 2;
pub const MM_DIACOUSTICS: u32 = 129;
pub const MM_DIACOUSTICS_DRUM_ACTION: u32 = 1;
pub const MM_DIALOGIC: u32 = 93;
pub const MM_DIAMONDMM: u32 = 163;
pub const MM_DICTAPHONE: u32 = 214;
pub const MM_DICTAPHONE_G726: u32 = 1;
pub const MM_DIGIGRAM: u32 = 227;
pub const MM_DIGITAL: u32 = 100;
pub const MM_DIGITAL_ACM_G723: u32 = 3;
pub const MM_DIGITAL_AUDIO_LABS: u32 = 136;
pub const MM_DIGITAL_AUDIO_LABS_CDLX: u32 = 19;
pub const MM_DIGITAL_AUDIO_LABS_CPRO: u32 = 17;
pub const MM_DIGITAL_AUDIO_LABS_CTDIF: u32 = 20;
pub const MM_DIGITAL_AUDIO_LABS_DOC: u32 = 2;
pub const MM_DIGITAL_AUDIO_LABS_TC: u32 = 1;
pub const MM_DIGITAL_AUDIO_LABS_V8: u32 = 16;
pub const MM_DIGITAL_AUDIO_LABS_VP: u32 = 18;
pub const MM_DIGITAL_AV320_WAVEIN: u32 = 1;
pub const MM_DIGITAL_AV320_WAVEOUT: u32 = 2;
pub const MM_DIGITAL_ICM_H261: u32 = 5;
pub const MM_DIGITAL_ICM_H263: u32 = 4;
pub const MM_DIMD_AUX_LINE: u32 = 9;
pub const MM_DIMD_DIRSOUND: u32 = 1;
pub const MM_DIMD_MIDIIN: u32 = 7;
pub const MM_DIMD_MIDIOUT: u32 = 8;
pub const MM_DIMD_MIXER: u32 = 10;
pub const MM_DIMD_PLATFORM: u32 = 0;
pub const MM_DIMD_VIRTJOY: u32 = 4;
pub const MM_DIMD_VIRTMPU: u32 = 2;
pub const MM_DIMD_VIRTSB: u32 = 3;
pub const MM_DIMD_WAVEIN: u32 = 5;
pub const MM_DIMD_WAVEOUT: u32 = 6;
pub const MM_DIMD_WSS_AUX: u32 = 21;
pub const MM_DIMD_WSS_MIXER: u32 = 17;
pub const MM_DIMD_WSS_SYNTH: u32 = 76;
pub const MM_DIMD_WSS_WAVEIN: u32 = 14;
pub const MM_DIMD_WSS_WAVEOUT: u32 = 15;
pub const MM_DOLBY: u32 = 78;
pub const MM_DPSINC: u32 = 191;
pub const MM_DSP_GROUP: u32 = 43;
pub const MM_DSP_GROUP_TRUESPEECH: u32 = 1;
pub const MM_DSP_SOLUTIONS: u32 = 25;
pub const MM_DSP_SOLUTIONS_AUX: u32 = 4;
pub const MM_DSP_SOLUTIONS_SYNTH: u32 = 3;
pub const MM_DSP_SOLUTIONS_WAVEIN: u32 = 2;
pub const MM_DSP_SOLUTIONS_WAVEOUT: u32 = 1;
pub const MM_DTS: u32 = 226;
pub const MM_DTS_DS: u32 = 1;
pub const MM_DUCK: u32 = 197;
pub const MM_DVISION: u32 = 165;
pub const MM_ECHO: u32 = 39;
pub const MM_ECHO_AUX: u32 = 6;
pub const MM_ECHO_MIDIIN: u32 = 5;
pub const MM_ECHO_MIDIOUT: u32 = 4;
pub const MM_ECHO_SYNTH: u32 = 1;
pub const MM_ECHO_WAVEIN: u32 = 3;
pub const MM_ECHO_WAVEOUT: u32 = 2;
pub const MM_ECS: u32 = 145;
pub const MM_ECS_AADF_MIDI_IN: u32 = 10;
pub const MM_ECS_AADF_MIDI_OUT: u32 = 11;
pub const MM_ECS_AADF_WAVE2MIDI_IN: u32 = 12;
pub const MM_EES: u32 = 219;
pub const MM_EES_PCMIDI14: u32 = 1;
pub const MM_EES_PCMIDI14_IN: u32 = 2;
pub const MM_EES_PCMIDI14_OUT1: u32 = 3;
pub const MM_EES_PCMIDI14_OUT2: u32 = 4;
pub const MM_EES_PCMIDI14_OUT3: u32 = 5;
pub const MM_EES_PCMIDI14_OUT4: u32 = 6;
pub const MM_EMAGIC: u32 = 208;
pub const MM_EMAGIC_UNITOR8: u32 = 1;
pub const MM_EMU: u32 = 19;
pub const MM_EMU_APSMIDIIN: u32 = 2;
pub const MM_EMU_APSMIDIOUT: u32 = 3;
pub const MM_EMU_APSSYNTH: u32 = 1;
pub const MM_EMU_APSWAVEIN: u32 = 4;
pub const MM_EMU_APSWAVEOUT: u32 = 5;
pub const MM_ENET: u32 = 206;
pub const MM_ENET_T2000_HANDSETIN: u32 = 3;
pub const MM_ENET_T2000_HANDSETOUT: u32 = 4;
pub const MM_ENET_T2000_LINEIN: u32 = 1;
pub const MM_ENET_T2000_LINEOUT: u32 = 2;
pub const MM_ENSONIQ: u32 = 125;
pub const MM_ENSONIQ_SOUNDSCAPE: u32 = 16;
pub const MM_EPSON: u32 = 50;
pub const MM_EPS_FMSND: u32 = 1;
pub const MM_ESS: u32 = 46;
pub const MM_ESS_AMAUX: u32 = 3;
pub const MM_ESS_AMMIDIIN: u32 = 6;
pub const MM_ESS_AMMIDIOUT: u32 = 5;
pub const MM_ESS_AMSYNTH: u32 = 4;
pub const MM_ESS_AMWAVEIN: u32 = 2;
pub const MM_ESS_AMWAVEOUT: u32 = 1;
pub const MM_ESS_AUX_CD: u32 = 8;
pub const MM_ESS_ES1488_MIXER: u32 = 24;
pub const MM_ESS_ES1488_WAVEIN: u32 = 23;
pub const MM_ESS_ES1488_WAVEOUT: u32 = 22;
pub const MM_ESS_ES1688_MIXER: u32 = 27;
pub const MM_ESS_ES1688_WAVEIN: u32 = 26;
pub const MM_ESS_ES1688_WAVEOUT: u32 = 25;
pub const MM_ESS_ES1788_MIXER: u32 = 30;
pub const MM_ESS_ES1788_WAVEIN: u32 = 29;
pub const MM_ESS_ES1788_WAVEOUT: u32 = 28;
pub const MM_ESS_ES1868_MIXER: u32 = 36;
pub const MM_ESS_ES1868_WAVEIN: u32 = 35;
pub const MM_ESS_ES1868_WAVEOUT: u32 = 34;
pub const MM_ESS_ES1878_MIXER: u32 = 39;
pub const MM_ESS_ES1878_WAVEIN: u32 = 38;
pub const MM_ESS_ES1878_WAVEOUT: u32 = 37;
pub const MM_ESS_ES1888_MIXER: u32 = 33;
pub const MM_ESS_ES1888_WAVEIN: u32 = 32;
pub const MM_ESS_ES1888_WAVEOUT: u32 = 31;
pub const MM_ESS_ES488_MIXER: u32 = 18;
pub const MM_ESS_ES488_WAVEIN: u32 = 17;
pub const MM_ESS_ES488_WAVEOUT: u32 = 16;
pub const MM_ESS_ES688_MIXER: u32 = 21;
pub const MM_ESS_ES688_WAVEIN: u32 = 20;
pub const MM_ESS_ES688_WAVEOUT: u32 = 19;
pub const MM_ESS_MIXER: u32 = 7;
pub const MM_ESS_MPU401_MIDIIN: u32 = 10;
pub const MM_ESS_MPU401_MIDIOUT: u32 = 9;
pub const MM_ETEK: u32 = 241;
pub const MM_ETEK_KWIKMIDI_MIDIIN: u32 = 1;
pub const MM_ETEK_KWIKMIDI_MIDIOUT: u32 = 2;
pub const MM_EUPHONICS: u32 = 152;
pub const MM_EUPHONICS_AUX_CD: u32 = 1;
pub const MM_EUPHONICS_AUX_LINE: u32 = 2;
pub const MM_EUPHONICS_AUX_MASTER: u32 = 3;
pub const MM_EUPHONICS_AUX_MIC: u32 = 4;
pub const MM_EUPHONICS_AUX_MIDI: u32 = 5;
pub const MM_EUPHONICS_AUX_WAVE: u32 = 6;
pub const MM_EUPHONICS_EUSYNTH: u32 = 14;
pub const MM_EUPHONICS_FMSYNTH_MONO: u32 = 7;
pub const MM_EUPHONICS_FMSYNTH_STEREO: u32 = 8;
pub const MM_EUPHONICS_MIDIIN: u32 = 9;
pub const MM_EUPHONICS_MIDIOUT: u32 = 10;
pub const MM_EUPHONICS_MIXER: u32 = 11;
pub const MM_EUPHONICS_WAVEIN: u32 = 12;
pub const MM_EUPHONICS_WAVEOUT: u32 = 13;
pub const MM_EVEREX: u32 = 38;
pub const MM_EVEREX_CARRIER: u32 = 1;
pub const MM_EXAN: u32 = 63;
pub const MM_FAITH: u32 = 15;
pub const MM_FAST: u32 = 126;
pub const MM_FHGIIS_MPEGLAYER3: u32 = 10;
pub const MM_FHGIIS_MPEGLAYER3_ADVANCED: u32 = 12;
pub const MM_FHGIIS_MPEGLAYER3_ADVANCEDPLUS: u32 = 14;
pub const MM_FHGIIS_MPEGLAYER3_BASIC: u32 = 11;
pub const MM_FHGIIS_MPEGLAYER3_DECODE: u32 = 9;
pub const MM_FHGIIS_MPEGLAYER3_LITE: u32 = 10;
pub const MM_FHGIIS_MPEGLAYER3_PROFESSIONAL: u32 = 13;
pub const MM_FLEXION: u32 = 249;
pub const MM_FLEXION_X300_WAVEIN: u32 = 1;
pub const MM_FLEXION_X300_WAVEOUT: u32 = 2;
pub const MM_FORTEMEDIA: u32 = 229;
pub const MM_FORTEMEDIA_AUX: u32 = 5;
pub const MM_FORTEMEDIA_FMSYNC: u32 = 3;
pub const MM_FORTEMEDIA_MIXER: u32 = 4;
pub const MM_FORTEMEDIA_WAVEIN: u32 = 1;
pub const MM_FORTEMEDIA_WAVEOUT: u32 = 2;
pub const MM_FRAUNHOFER_IIS: u32 = 172;
pub const MM_FRONTIER: u32 = 160;
pub const MM_FRONTIER_WAVECENTER_MIDIIN: u32 = 1;
pub const MM_FRONTIER_WAVECENTER_MIDIOUT: u32 = 2;
pub const MM_FRONTIER_WAVECENTER_WAVEIN: u32 = 3;
pub const MM_FRONTIER_WAVECENTER_WAVEOUT: u32 = 4;
pub const MM_FTR: u32 = 198;
pub const MM_FTR_ACM: u32 = 2;
pub const MM_FTR_ENCODER_WAVEIN: u32 = 1;
pub const MM_FUJITSU: u32 = 4;
pub const MM_GADGETLABS: u32 = 159;
pub const MM_GADGETLABS_WAVE42_WAVEIN: u32 = 3;
pub const MM_GADGETLABS_WAVE42_WAVEOUT: u32 = 4;
pub const MM_GADGETLABS_WAVE44_WAVEIN: u32 = 1;
pub const MM_GADGETLABS_WAVE44_WAVEOUT: u32 = 2;
pub const MM_GADGETLABS_WAVE4_MIDIIN: u32 = 5;
pub const MM_GADGETLABS_WAVE4_MIDIOUT: u32 = 6;
pub const MM_GRANDE: u32 = 117;
pub const MM_GRAVIS: u32 = 34;
pub const MM_GUILLEMOT: u32 = 207;
pub const MM_GULBRANSEN: u32 = 130;
pub const MM_HAFTMANN: u32 = 220;
pub const MM_HAFTMANN_LPTDAC2: u32 = 1;
pub const MM_HEADSPACE: u32 = 222;
pub const MM_HEADSPACE_HAEMIXER: u32 = 4;
pub const MM_HEADSPACE_HAESYNTH: u32 = 1;
pub const MM_HEADSPACE_HAEWAVEIN: u32 = 3;
pub const MM_HEADSPACE_HAEWAVEOUT: u32 = 2;
pub const MM_HEWLETT_PACKARD: u32 = 13;
pub const MM_HEWLETT_PACKARD_CU_CODEC: u32 = 1;
pub const MM_HORIZONS: u32 = 107;
pub const MM_HP: u32 = 253;
pub const MM_HP_WAVEIN: u32 = 2;
pub const MM_HP_WAVEOUT: u32 = 1;
pub const MM_HYPERACTIVE: u32 = 246;
pub const MM_IBM: u32 = 22;
pub const MM_IBM_MWAVE_AUX: u32 = 23;
pub const MM_IBM_MWAVE_MIDIIN: u32 = 21;
pub const MM_IBM_MWAVE_MIDIOUT: u32 = 22;
pub const MM_IBM_MWAVE_MIXER: u32 = 20;
pub const MM_IBM_MWAVE_WAVEIN: u32 = 18;
pub const MM_IBM_MWAVE_WAVEOUT: u32 = 19;
pub const MM_IBM_PCMCIA_AUX: u32 = 16;
pub const MM_IBM_PCMCIA_MIDIIN: u32 = 14;
pub const MM_IBM_PCMCIA_MIDIOUT: u32 = 15;
pub const MM_IBM_PCMCIA_SYNTH: u32 = 13;
pub const MM_IBM_PCMCIA_WAVEIN: u32 = 11;
pub const MM_IBM_PCMCIA_WAVEOUT: u32 = 12;
pub const MM_IBM_THINKPAD200: u32 = 17;
pub const MM_IBM_WC_MIDIOUT: u32 = 30;
pub const MM_IBM_WC_MIXEROUT: u32 = 33;
pub const MM_IBM_WC_WAVEOUT: u32 = 31;
pub const MM_ICCC: u32 = 259;
pub const MM_ICCC_UNA3_AUX: u32 = 3;
pub const MM_ICCC_UNA3_MIXER: u32 = 4;
pub const MM_ICCC_UNA3_WAVEIN: u32 = 1;
pub const MM_ICCC_UNA3_WAVEOUT: u32 = 2;
pub const MM_ICE: u32 = 239;
pub const MM_ICE_AUX: u32 = 11;
pub const MM_ICE_MIDIIN1: u32 = 6;
pub const MM_ICE_MIDIIN2: u32 = 8;
pub const MM_ICE_MIDIOUT1: u32 = 5;
pub const MM_ICE_MIDIOUT2: u32 = 7;
pub const MM_ICE_MIXER: u32 = 10;
pub const MM_ICE_MTWAVEIN: u32 = 4;
pub const MM_ICE_MTWAVEOUT: u32 = 3;
pub const MM_ICE_SYNTH: u32 = 9;
pub const MM_ICE_WAVEIN: u32 = 2;
pub const MM_ICE_WAVEOUT: u32 = 1;
pub const MM_ICL_PS: u32 = 32;
pub const MM_ICOM_AUX: u32 = 6;
pub const MM_ICOM_LINE: u32 = 7;
pub const MM_ICOM_MIXER: u32 = 5;
pub const MM_ICOM_WAVEIN: u32 = 3;
pub const MM_ICOM_WAVEOUT: u32 = 4;
pub const MM_ICS: u32 = 57;
pub const MM_ICS_2115_LITE_MIDIOUT: u32 = 13;
pub const MM_ICS_2120_LITE_MIDIOUT: u32 = 14;
pub const MM_ICS_WAVEDECK_AUX: u32 = 4;
pub const MM_ICS_WAVEDECK_MIXER: u32 = 3;
pub const MM_ICS_WAVEDECK_SYNTH: u32 = 5;
pub const MM_ICS_WAVEDECK_WAVEIN: u32 = 2;
pub const MM_ICS_WAVEDECK_WAVEOUT: u32 = 1;
pub const MM_ICS_WAVEDEC_SB_AUX: u32 = 12;
pub const MM_ICS_WAVEDEC_SB_FM_MIDIOUT: u32 = 8;
pub const MM_ICS_WAVEDEC_SB_MIXER: u32 = 11;
pub const MM_ICS_WAVEDEC_SB_MPU401_MIDIIN: u32 = 10;
pub const MM_ICS_WAVEDEC_SB_MPU401_MIDIOUT: u32 = 9;
pub const MM_ICS_WAVEDEC_SB_WAVEIN: u32 = 7;
pub const MM_ICS_WAVEDEC_SB_WAVEOUT: u32 = 6;
pub const MM_INSOFT: u32 = 94;
pub const MM_INTEL: u32 = 33;
pub const MM_INTELOPD_AUX: u32 = 401;
pub const MM_INTELOPD_WAVEIN: u32 = 1;
pub const MM_INTELOPD_WAVEOUT: u32 = 101;
pub const MM_INTEL_NSPMODEMLINEIN: u32 = 501;
pub const MM_INTEL_NSPMODEMLINEOUT: u32 = 502;
pub const MM_INTERACTIVE: u32 = 36;
pub const MM_INTERACTIVE_WAVEIN: u32 = 69;
pub const MM_INTERACTIVE_WAVEOUT: u32 = 69;
pub const MM_INTERNET: u32 = 244;
pub const MM_INTERNET_SSW_MIDIIN: u32 = 11;
pub const MM_INTERNET_SSW_MIDIOUT: u32 = 10;
pub const MM_INTERNET_SSW_WAVEIN: u32 = 13;
pub const MM_INTERNET_SSW_WAVEOUT: u32 = 12;
pub const MM_INVISION: u32 = 188;
pub const MM_IODD: u32 = 258;
pub const MM_IOMAGIC: u32 = 82;
pub const MM_IOMAGIC_TEMPO_AUXOUT: u32 = 6;
pub const MM_IOMAGIC_TEMPO_MIDIOUT: u32 = 4;
pub const MM_IOMAGIC_TEMPO_MXDOUT: u32 = 5;
pub const MM_IOMAGIC_TEMPO_SYNTH: u32 = 3;
pub const MM_IOMAGIC_TEMPO_WAVEIN: u32 = 2;
pub const MM_IOMAGIC_TEMPO_WAVEOUT: u32 = 1;
pub const MM_IPI: u32 = 238;
pub const MM_IPI_ACM_HSX: u32 = 1;
pub const MM_IPI_ACM_RPELP: u32 = 2;
pub const MM_IPI_AT_MIXER: u32 = 6;
pub const MM_IPI_AT_WAVEIN: u32 = 5;
pub const MM_IPI_AT_WAVEOUT: u32 = 4;
pub const MM_IPI_WF_ASSS: u32 = 3;
pub const MM_ISOLUTION: u32 = 106;
pub const MM_ISOLUTION_PASCAL: u32 = 1;
pub const MM_ITERATEDSYS: u32 = 58;
pub const MM_ITERATEDSYS_FUFCODEC: u32 = 1;
pub const MM_I_LINK: u32 = 233;
pub const MM_I_LINK_VOICE_CODER: u32 = 1;
pub const MM_KAY_ELEMETRICS: u32 = 131;
pub const MM_KAY_ELEMETRICS_CSL: u32 = 17152;
pub const MM_KAY_ELEMETRICS_CSL_4CHANNEL: u32 = 17161;
pub const MM_KAY_ELEMETRICS_CSL_DAT: u32 = 17160;
pub const MM_KORG: u32 = 55;
pub const MM_KORG_1212IO_MSWAVEIN: u32 = 3;
pub const MM_KORG_1212IO_MSWAVEOUT: u32 = 4;
pub const MM_KORG_PCIF_MIDIIN: u32 = 2;
pub const MM_KORG_PCIF_MIDIOUT: u32 = 1;
pub const MM_LERNOUT_ANDHAUSPIE_LHCODECACM: u32 = 1;
pub const MM_LERNOUT_AND_HAUSPIE: u32 = 97;
pub const MM_LEXICON: u32 = 236;
pub const MM_LEXICON_STUDIO_WAVE_IN: u32 = 2;
pub const MM_LEXICON_STUDIO_WAVE_OUT: u32 = 1;
pub const MM_LOGITECH: u32 = 60;
pub const MM_LUCENT: u32 = 184;
pub const MM_LUCENT_ACM_G723: u32 = 0;
pub const MM_LUCID: u32 = 221;
pub const MM_LUCID_PCI24WAVEIN: u32 = 1;
pub const MM_LUCID_PCI24WAVEOUT: u32 = 2;
pub const MM_LUMINOSITI: u32 = 224;
pub const MM_LUMINOSITI_SCWAVEIN: u32 = 1;
pub const MM_LUMINOSITI_SCWAVEMIX: u32 = 3;
pub const MM_LUMINOSITI_SCWAVEOUT: u32 = 2;
pub const MM_LYNX: u32 = 212;
pub const MM_LYRRUS: u32 = 88;
pub const MM_LYRRUS_BRIDGE_GUITAR: u32 = 1;
pub const MM_MALDEN: u32 = 261;
pub const MM_MARIAN: u32 = 190;
pub const MM_MARIAN_ARC44WAVEIN: u32 = 1;
pub const MM_MARIAN_ARC44WAVEOUT: u32 = 2;
pub const MM_MARIAN_ARC88WAVEIN: u32 = 5;
pub const MM_MARIAN_ARC88WAVEOUT: u32 = 6;
pub const MM_MARIAN_PRODIF24WAVEIN: u32 = 3;
pub const MM_MARIAN_PRODIF24WAVEOUT: u32 = 4;
pub const MM_MATROX_DIV: u32 = 254;
pub const MM_MATSUSHITA: u32 = 83;
pub const MM_MATSUSHITA_AUX: u32 = 5;
pub const MM_MATSUSHITA_FMSYNTH_STEREO: u32 = 3;
pub const MM_MATSUSHITA_MIXER: u32 = 4;
pub const MM_MATSUSHITA_WAVEIN: u32 = 1;
pub const MM_MATSUSHITA_WAVEOUT: u32 = 2;
pub const MM_MEDIASONIC: u32 = 71;
pub const MM_MEDIASONIC_ACM_G723: u32 = 1;
pub const MM_MEDIASONIC_ICOM: u32 = 2;
pub const MM_MEDIATRIX: u32 = 141;
pub const MM_MEDIAVISION: u32 = 3;
pub const MM_MEDIAVISION_CDPC: u32 = 112;
pub const MM_MEDIAVISION_OPUS1208: u32 = 128;
pub const MM_MEDIAVISION_OPUS1216: u32 = 144;
pub const MM_MEDIAVISION_PROAUDIO: u32 = 16;
pub const MM_MEDIAVISION_PROAUDIO_16: u32 = 96;
pub const MM_MEDIAVISION_PROAUDIO_PLUS: u32 = 80;
pub const MM_MEDIAVISION_PROSTUDIO_16: u32 = 96;
pub const MM_MEDIAVISION_THUNDER: u32 = 32;
pub const MM_MEDIAVISION_TPORT: u32 = 64;
pub const MM_MELABS: u32 = 44;
pub const MM_MELABS_MIDI2GO: u32 = 1;
pub const MM_MERGING_MPEGL3: u32 = 1;
pub const MM_MERGING_TECHNOLOGIES: u32 = 177;
pub const MM_METHEUS: u32 = 59;
pub const MM_METHEUS_ZIPPER: u32 = 1;
pub const MM_MICRONAS: u32 = 251;
pub const MM_MICRONAS_CLP833: u32 = 2;
pub const MM_MICRONAS_SC4: u32 = 1;
pub const MM_MICROSOFT: u32 = 1;
pub const MM_MIDI_MAPPER: u32 = 1;
pub const MM_MINDMAKER: u32 = 263;
pub const MM_MINDMAKER_GC_MIXER: u32 = 3;
pub const MM_MINDMAKER_GC_WAVEIN: u32 = 1;
pub const MM_MINDMAKER_GC_WAVEOUT: u32 = 2;
pub const MM_MIRO: u32 = 104;
pub const MM_MIRO_DC30_MIX: u32 = 7;
pub const MM_MIRO_DC30_WAVEIN: u32 = 6;
pub const MM_MIRO_DC30_WAVEOUT: u32 = 5;
pub const MM_MIRO_MOVIEPRO: u32 = 1;
pub const MM_MIRO_VIDEOD1: u32 = 2;
pub const MM_MIRO_VIDEODC1TV: u32 = 3;
pub const MM_MIRO_VIDEOTD: u32 = 4;
pub const MM_MITEL: u32 = 16;
pub const MM_MITEL_MEDIAPATH_WAVEIN: u32 = 301;
pub const MM_MITEL_MEDIAPATH_WAVEOUT: u32 = 300;
pub const MM_MITEL_MPA_HANDSET_WAVEIN: u32 = 201;
pub const MM_MITEL_MPA_HANDSET_WAVEOUT: u32 = 200;
pub const MM_MITEL_MPA_HANDSFREE_WAVEIN: u32 = 203;
pub const MM_MITEL_MPA_HANDSFREE_WAVEOUT: u32 = 202;
pub const MM_MITEL_MPA_LINE1_WAVEIN: u32 = 205;
pub const MM_MITEL_MPA_LINE1_WAVEOUT: u32 = 204;
pub const MM_MITEL_MPA_LINE2_WAVEIN: u32 = 207;
pub const MM_MITEL_MPA_LINE2_WAVEOUT: u32 = 206;
pub const MM_MITEL_TALKTO_BRIDGED_WAVEIN: u32 = 105;
pub const MM_MITEL_TALKTO_BRIDGED_WAVEOUT: u32 = 104;
pub const MM_MITEL_TALKTO_HANDSET_WAVEIN: u32 = 103;
pub const MM_MITEL_TALKTO_HANDSET_WAVEOUT: u32 = 102;
pub const MM_MITEL_TALKTO_LINE_WAVEIN: u32 = 101;
pub const MM_MITEL_TALKTO_LINE_WAVEOUT: u32 = 100;
pub const MM_MMOTION_WAVEAUX: u32 = 1;
pub const MM_MMOTION_WAVEIN: u32 = 3;
pub const MM_MMOTION_WAVEOUT: u32 = 2;
pub const MM_MOSCOM: u32 = 68;
pub const MM_MOSCOM_VPC2400_IN: u32 = 1;
pub const MM_MOSCOM_VPC2400_OUT: u32 = 2;
pub const MM_MOTIONPIXELS: u32 = 193;
pub const MM_MOTIONPIXELS_MVI2: u32 = 1;
pub const MM_MOTOROLA: u32 = 48;
pub const MM_MOTU: u32 = 101;
pub const MM_MOTU_DTX_MIDI_IN_A: u32 = 801;
pub const MM_MOTU_DTX_MIDI_IN_B: u32 = 802;
pub const MM_MOTU_DTX_MIDI_IN_SYNC: u32 = 800;
pub const MM_MOTU_DTX_MIDI_OUT_A: u32 = 801;
pub const MM_MOTU_DTX_MIDI_OUT_B: u32 = 802;
pub const MM_MOTU_FLYER_MIDI_IN_A: u32 = 601;
pub const MM_MOTU_FLYER_MIDI_IN_B: u32 = 602;
pub const MM_MOTU_FLYER_MIDI_IN_SYNC: u32 = 600;
pub const MM_MOTU_FLYER_MIDI_OUT_A: u32 = 601;
pub const MM_MOTU_FLYER_MIDI_OUT_B: u32 = 602;
pub const MM_MOTU_MTPAV_MIDIIN_1: u32 = 901;
pub const MM_MOTU_MTPAV_MIDIIN_2: u32 = 902;
pub const MM_MOTU_MTPAV_MIDIIN_3: u32 = 903;
pub const MM_MOTU_MTPAV_MIDIIN_4: u32 = 904;
pub const MM_MOTU_MTPAV_MIDIIN_5: u32 = 905;
pub const MM_MOTU_MTPAV_MIDIIN_6: u32 = 906;
pub const MM_MOTU_MTPAV_MIDIIN_7: u32 = 907;
pub const MM_MOTU_MTPAV_MIDIIN_8: u32 = 908;
pub const MM_MOTU_MTPAV_MIDIIN_ADAT: u32 = 917;
pub const MM_MOTU_MTPAV_MIDIIN_SYNC: u32 = 900;
pub const MM_MOTU_MTPAV_MIDIOUT_1: u32 = 901;
pub const MM_MOTU_MTPAV_MIDIOUT_2: u32 = 902;
pub const MM_MOTU_MTPAV_MIDIOUT_3: u32 = 903;
pub const MM_MOTU_MTPAV_MIDIOUT_4: u32 = 904;
pub const MM_MOTU_MTPAV_MIDIOUT_5: u32 = 905;
pub const MM_MOTU_MTPAV_MIDIOUT_6: u32 = 906;
pub const MM_MOTU_MTPAV_MIDIOUT_7: u32 = 907;
pub const MM_MOTU_MTPAV_MIDIOUT_8: u32 = 908;
pub const MM_MOTU_MTPAV_MIDIOUT_ADAT: u32 = 917;
pub const MM_MOTU_MTPAV_MIDIOUT_ALL: u32 = 900;
pub const MM_MOTU_MTPAV_NET_MIDIIN_1: u32 = 909;
pub const MM_MOTU_MTPAV_NET_MIDIIN_2: u32 = 910;
pub const MM_MOTU_MTPAV_NET_MIDIIN_3: u32 = 911;
pub const MM_MOTU_MTPAV_NET_MIDIIN_4: u32 = 912;
pub const MM_MOTU_MTPAV_NET_MIDIIN_5: u32 = 913;
pub const MM_MOTU_MTPAV_NET_MIDIIN_6: u32 = 914;
pub const MM_MOTU_MTPAV_NET_MIDIIN_7: u32 = 915;
pub const MM_MOTU_MTPAV_NET_MIDIIN_8: u32 = 916;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_1: u32 = 909;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_2: u32 = 910;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_3: u32 = 911;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_4: u32 = 912;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_5: u32 = 913;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_6: u32 = 914;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_7: u32 = 915;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_8: u32 = 916;
pub const MM_MOTU_MTPII_MIDIIN_1: u32 = 201;
pub const MM_MOTU_MTPII_MIDIIN_2: u32 = 202;
pub const MM_MOTU_MTPII_MIDIIN_3: u32 = 203;
pub const MM_MOTU_MTPII_MIDIIN_4: u32 = 204;
pub const MM_MOTU_MTPII_MIDIIN_5: u32 = 205;
pub const MM_MOTU_MTPII_MIDIIN_6: u32 = 206;
pub const MM_MOTU_MTPII_MIDIIN_7: u32 = 207;
pub const MM_MOTU_MTPII_MIDIIN_8: u32 = 208;
pub const MM_MOTU_MTPII_MIDIIN_SYNC: u32 = 200;
pub const MM_MOTU_MTPII_MIDIOUT_1: u32 = 201;
pub const MM_MOTU_MTPII_MIDIOUT_2: u32 = 202;
pub const MM_MOTU_MTPII_MIDIOUT_3: u32 = 203;
pub const MM_MOTU_MTPII_MIDIOUT_4: u32 = 204;
pub const MM_MOTU_MTPII_MIDIOUT_5: u32 = 205;
pub const MM_MOTU_MTPII_MIDIOUT_6: u32 = 206;
pub const MM_MOTU_MTPII_MIDIOUT_7: u32 = 207;
pub const MM_MOTU_MTPII_MIDIOUT_8: u32 = 208;
pub const MM_MOTU_MTPII_MIDIOUT_ALL: u32 = 200;
pub const MM_MOTU_MTPII_NET_MIDIIN_1: u32 = 209;
pub const MM_MOTU_MTPII_NET_MIDIIN_2: u32 = 210;
pub const MM_MOTU_MTPII_NET_MIDIIN_3: u32 = 211;
pub const MM_MOTU_MTPII_NET_MIDIIN_4: u32 = 212;
pub const MM_MOTU_MTPII_NET_MIDIIN_5: u32 = 213;
pub const MM_MOTU_MTPII_NET_MIDIIN_6: u32 = 214;
pub const MM_MOTU_MTPII_NET_MIDIIN_7: u32 = 215;
pub const MM_MOTU_MTPII_NET_MIDIIN_8: u32 = 216;
pub const MM_MOTU_MTPII_NET_MIDIOUT_1: u32 = 209;
pub const MM_MOTU_MTPII_NET_MIDIOUT_2: u32 = 210;
pub const MM_MOTU_MTPII_NET_MIDIOUT_3: u32 = 211;
pub const MM_MOTU_MTPII_NET_MIDIOUT_4: u32 = 212;
pub const MM_MOTU_MTPII_NET_MIDIOUT_5: u32 = 213;
pub const MM_MOTU_MTPII_NET_MIDIOUT_6: u32 = 214;
pub const MM_MOTU_MTPII_NET_MIDIOUT_7: u32 = 215;
pub const MM_MOTU_MTPII_NET_MIDIOUT_8: u32 = 216;
pub const MM_MOTU_MTP_MIDIIN_1: u32 = 101;
pub const MM_MOTU_MTP_MIDIIN_2: u32 = 102;
pub const MM_MOTU_MTP_MIDIIN_3: u32 = 103;
pub const MM_MOTU_MTP_MIDIIN_4: u32 = 104;
pub const MM_MOTU_MTP_MIDIIN_5: u32 = 105;
pub const MM_MOTU_MTP_MIDIIN_6: u32 = 106;
pub const MM_MOTU_MTP_MIDIIN_7: u32 = 107;
pub const MM_MOTU_MTP_MIDIIN_8: u32 = 108;
pub const MM_MOTU_MTP_MIDIOUT_1: u32 = 101;
pub const MM_MOTU_MTP_MIDIOUT_2: u32 = 102;
pub const MM_MOTU_MTP_MIDIOUT_3: u32 = 103;
pub const MM_MOTU_MTP_MIDIOUT_4: u32 = 104;
pub const MM_MOTU_MTP_MIDIOUT_5: u32 = 105;
pub const MM_MOTU_MTP_MIDIOUT_6: u32 = 106;
pub const MM_MOTU_MTP_MIDIOUT_7: u32 = 107;
pub const MM_MOTU_MTP_MIDIOUT_8: u32 = 108;
pub const MM_MOTU_MTP_MIDIOUT_ALL: u32 = 100;
pub const MM_MOTU_MXN_MIDIIN_1: u32 = 501;
pub const MM_MOTU_MXN_MIDIIN_2: u32 = 502;
pub const MM_MOTU_MXN_MIDIIN_3: u32 = 503;
pub const MM_MOTU_MXN_MIDIIN_4: u32 = 504;
pub const MM_MOTU_MXN_MIDIIN_SYNC: u32 = 500;
pub const MM_MOTU_MXN_MIDIOUT_1: u32 = 501;
pub const MM_MOTU_MXN_MIDIOUT_2: u32 = 502;
pub const MM_MOTU_MXN_MIDIOUT_3: u32 = 503;
pub const MM_MOTU_MXN_MIDIOUT_4: u32 = 504;
pub const MM_MOTU_MXN_MIDIOUT_ALL: u32 = 500;
pub const MM_MOTU_MXPMPU_MIDIIN_1: u32 = 401;
pub const MM_MOTU_MXPMPU_MIDIIN_2: u32 = 402;
pub const MM_MOTU_MXPMPU_MIDIIN_3: u32 = 403;
pub const MM_MOTU_MXPMPU_MIDIIN_4: u32 = 404;
pub const MM_MOTU_MXPMPU_MIDIIN_5: u32 = 405;
pub const MM_MOTU_MXPMPU_MIDIIN_6: u32 = 406;
pub const MM_MOTU_MXPMPU_MIDIIN_SYNC: u32 = 400;
pub const MM_MOTU_MXPMPU_MIDIOUT_1: u32 = 401;
pub const MM_MOTU_MXPMPU_MIDIOUT_2: u32 = 402;
pub const MM_MOTU_MXPMPU_MIDIOUT_3: u32 = 403;
pub const MM_MOTU_MXPMPU_MIDIOUT_4: u32 = 404;
pub const MM_MOTU_MXPMPU_MIDIOUT_5: u32 = 405;
pub const MM_MOTU_MXPMPU_MIDIOUT_6: u32 = 406;
pub const MM_MOTU_MXPMPU_MIDIOUT_ALL: u32 = 400;
pub const MM_MOTU_MXPXT_MIDIIN_1: u32 = 1001;
pub const MM_MOTU_MXPXT_MIDIIN_2: u32 = 1002;
pub const MM_MOTU_MXPXT_MIDIIN_3: u32 = 1003;
pub const MM_MOTU_MXPXT_MIDIIN_4: u32 = 1004;
pub const MM_MOTU_MXPXT_MIDIIN_5: u32 = 1005;
pub const MM_MOTU_MXPXT_MIDIIN_6: u32 = 1006;
pub const MM_MOTU_MXPXT_MIDIIN_7: u32 = 1007;
pub const MM_MOTU_MXPXT_MIDIIN_8: u32 = 1008;
pub const MM_MOTU_MXPXT_MIDIIN_SYNC: u32 = 1000;
pub const MM_MOTU_MXPXT_MIDIOUT_1: u32 = 1001;
pub const MM_MOTU_MXPXT_MIDIOUT_2: u32 = 1002;
pub const MM_MOTU_MXPXT_MIDIOUT_3: u32 = 1003;
pub const MM_MOTU_MXPXT_MIDIOUT_4: u32 = 1004;
pub const MM_MOTU_MXPXT_MIDIOUT_5: u32 = 1005;
pub const MM_MOTU_MXPXT_MIDIOUT_6: u32 = 1006;
pub const MM_MOTU_MXPXT_MIDIOUT_7: u32 = 1007;
pub const MM_MOTU_MXPXT_MIDIOUT_8: u32 = 1008;
pub const MM_MOTU_MXPXT_MIDIOUT_ALL: u32 = 1000;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_1: u32 = 301;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_2: u32 = 302;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_3: u32 = 303;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_4: u32 = 304;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_5: u32 = 305;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_6: u32 = 306;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_1: u32 = 301;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_2: u32 = 302;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_3: u32 = 303;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_4: u32 = 304;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_5: u32 = 305;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_6: u32 = 306;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_ALL: u32 = 300;
pub const MM_MOTU_MXP_MIDIIN_SYNC: u32 = 300;
pub const MM_MOTU_PKX_MIDI_IN_A: u32 = 701;
pub const MM_MOTU_PKX_MIDI_IN_B: u32 = 702;
pub const MM_MOTU_PKX_MIDI_IN_SYNC: u32 = 700;
pub const MM_MOTU_PKX_MIDI_OUT_A: u32 = 701;
pub const MM_MOTU_PKX_MIDI_OUT_B: u32 = 702;
pub const MM_MPTUS: u32 = 95;
pub const MM_MPTUS_SPWAVEOUT: u32 = 1;
pub const MM_MPU401_MIDIIN: u32 = 11;
pub const MM_MPU401_MIDIOUT: u32 = 10;
pub const MM_MSFT_ACM_G711: u32 = 37;
pub const MM_MSFT_ACM_GSM610: u32 = 36;
pub const MM_MSFT_ACM_IMAADPCM: u32 = 34;
pub const MM_MSFT_ACM_MSADPCM: u32 = 33;
pub const MM_MSFT_ACM_MSAUDIO1: u32 = 39;
pub const MM_MSFT_ACM_MSFILTER: u32 = 35;
pub const MM_MSFT_ACM_MSG723: u32 = 92;
pub const MM_MSFT_ACM_MSNAUDIO: u32 = 91;
pub const MM_MSFT_ACM_MSRT24: u32 = 93;
pub const MM_MSFT_ACM_PCM: u32 = 38;
pub const MM_MSFT_ACM_WMAUDIO: u32 = 39;
pub const MM_MSFT_ACM_WMAUDIO2: u32 = 101;
pub const MM_MSFT_GENERIC_AUX_CD: u32 = 30;
pub const MM_MSFT_GENERIC_AUX_LINE: u32 = 28;
pub const MM_MSFT_GENERIC_AUX_MIC: u32 = 29;
pub const MM_MSFT_GENERIC_MIDIIN: u32 = 25;
pub const MM_MSFT_GENERIC_MIDIOUT: u32 = 26;
pub const MM_MSFT_GENERIC_MIDISYNTH: u32 = 27;
pub const MM_MSFT_GENERIC_WAVEIN: u32 = 23;
pub const MM_MSFT_GENERIC_WAVEOUT: u32 = 24;
pub const MM_MSFT_MSACM: u32 = 32;
pub const MM_MSFT_MSOPL_SYNTH: u32 = 76;
pub const MM_MSFT_SB16_AUX_CD: u32 = 66;
pub const MM_MSFT_SB16_AUX_LINE: u32 = 65;
pub const MM_MSFT_SB16_MIDIIN: u32 = 62;
pub const MM_MSFT_SB16_MIDIOUT: u32 = 63;
pub const MM_MSFT_SB16_MIXER: u32 = 67;
pub const MM_MSFT_SB16_SYNTH: u32 = 64;
pub const MM_MSFT_SB16_WAVEIN: u32 = 60;
pub const MM_MSFT_SB16_WAVEOUT: u32 = 61;
pub const MM_MSFT_SBPRO_AUX_CD: u32 = 74;
pub const MM_MSFT_SBPRO_AUX_LINE: u32 = 73;
pub const MM_MSFT_SBPRO_MIDIIN: u32 = 70;
pub const MM_MSFT_SBPRO_MIDIOUT: u32 = 71;
pub const MM_MSFT_SBPRO_MIXER: u32 = 75;
pub const MM_MSFT_SBPRO_SYNTH: u32 = 72;
pub const MM_MSFT_SBPRO_WAVEIN: u32 = 68;
pub const MM_MSFT_SBPRO_WAVEOUT: u32 = 69;
pub const MM_MSFT_VMDMS_HANDSET_WAVEIN: u32 = 82;
pub const MM_MSFT_VMDMS_HANDSET_WAVEOUT: u32 = 83;
pub const MM_MSFT_VMDMS_LINE_WAVEIN: u32 = 80;
pub const MM_MSFT_VMDMS_LINE_WAVEOUT: u32 = 81;
pub const MM_MSFT_VMDMW_HANDSET_WAVEIN: u32 = 86;
pub const MM_MSFT_VMDMW_HANDSET_WAVEOUT: u32 = 87;
pub const MM_MSFT_VMDMW_LINE_WAVEIN: u32 = 84;
pub const MM_MSFT_VMDMW_LINE_WAVEOUT: u32 = 85;
pub const MM_MSFT_VMDMW_MIXER: u32 = 88;
pub const MM_MSFT_VMDM_GAME_WAVEIN: u32 = 90;
pub const MM_MSFT_VMDM_GAME_WAVEOUT: u32 = 89;
pub const MM_MSFT_WDMAUDIO_AUX: u32 = 105;
pub const MM_MSFT_WDMAUDIO_MIDIIN: u32 = 103;
pub const MM_MSFT_WDMAUDIO_MIDIOUT: u32 = 102;
pub const MM_MSFT_WDMAUDIO_MIXER: u32 = 104;
pub const MM_MSFT_WDMAUDIO_WAVEIN: u32 = 101;
pub const MM_MSFT_WDMAUDIO_WAVEOUT: u32 = 100;
pub const MM_MSFT_WSS_AUX: u32 = 21;
pub const MM_MSFT_WSS_FMSYNTH_STEREO: u32 = 16;
pub const MM_MSFT_WSS_MIXER: u32 = 17;
pub const MM_MSFT_WSS_NT_AUX: u32 = 59;
pub const MM_MSFT_WSS_NT_FMSYNTH_STEREO: u32 = 57;
pub const MM_MSFT_WSS_NT_MIXER: u32 = 58;
pub const MM_MSFT_WSS_NT_WAVEIN: u32 = 55;
pub const MM_MSFT_WSS_NT_WAVEOUT: u32 = 56;
pub const MM_MSFT_WSS_OEM_AUX: u32 = 22;
pub const MM_MSFT_WSS_OEM_FMSYNTH_STEREO: u32 = 20;
pub const MM_MSFT_WSS_OEM_MIXER: u32 = 31;
pub const MM_MSFT_WSS_OEM_WAVEIN: u32 = 18;
pub const MM_MSFT_WSS_OEM_WAVEOUT: u32 = 19;
pub const MM_MSFT_WSS_WAVEIN: u32 = 14;
pub const MM_MSFT_WSS_WAVEOUT: u32 = 15;
pub const MM_MWM: u32 = 209;
pub const MM_NCR: u32 = 62;
pub const MM_NCR_BA_AUX: u32 = 4;
pub const MM_NCR_BA_MIXER: u32 = 5;
pub const MM_NCR_BA_SYNTH: u32 = 3;
pub const MM_NCR_BA_WAVEIN: u32 = 1;
pub const MM_NCR_BA_WAVEOUT: u32 = 2;
pub const MM_NEC: u32 = 26;
pub const MM_NEC_26_SYNTH: u32 = 9;
pub const MM_NEC_73_86_SYNTH: u32 = 5;
pub const MM_NEC_73_86_WAVEIN: u32 = 7;
pub const MM_NEC_73_86_WAVEOUT: u32 = 6;
pub const MM_NEC_JOYSTICK: u32 = 12;
pub const MM_NEC_MPU401_MIDIIN: u32 = 11;
pub const MM_NEC_MPU401_MIDIOUT: u32 = 10;
pub const MM_NEOMAGIC: u32 = 176;
pub const MM_NEOMAGIC_AUX: u32 = 6;
pub const MM_NEOMAGIC_MIDIIN: u32 = 5;
pub const MM_NEOMAGIC_MIDIOUT: u32 = 4;
pub const MM_NEOMAGIC_MW3DX_AUX: u32 = 17;
pub const MM_NEOMAGIC_MW3DX_FMSYNTH: u32 = 14;
pub const MM_NEOMAGIC_MW3DX_GMSYNTH: u32 = 15;
pub const MM_NEOMAGIC_MW3DX_MIDIIN: u32 = 13;
pub const MM_NEOMAGIC_MW3DX_MIDIOUT: u32 = 12;
pub const MM_NEOMAGIC_MW3DX_MIXER: u32 = 16;
pub const MM_NEOMAGIC_MW3DX_WAVEIN: u32 = 11;
pub const MM_NEOMAGIC_MW3DX_WAVEOUT: u32 = 10;
pub const MM_NEOMAGIC_MWAVE_AUX: u32 = 25;
pub const MM_NEOMAGIC_MWAVE_MIDIIN: u32 = 23;
pub const MM_NEOMAGIC_MWAVE_MIDIOUT: u32 = 22;
pub const MM_NEOMAGIC_MWAVE_MIXER: u32 = 24;
pub const MM_NEOMAGIC_MWAVE_WAVEIN: u32 = 21;
pub const MM_NEOMAGIC_MWAVE_WAVEOUT: u32 = 20;
pub const MM_NEOMAGIC_SYNTH: u32 = 1;
pub const MM_NEOMAGIC_WAVEIN: u32 = 3;
pub const MM_NEOMAGIC_WAVEOUT: u32 = 2;
pub const MM_NETSCAPE: u32 = 166;
pub const MM_NETXL: u32 = 8;
pub const MM_NETXL_XLVIDEO: u32 = 1;
pub const MM_NEWMEDIA: u32 = 86;
pub const MM_NEWMEDIA_WAVJAMMER: u32 = 1;
pub const MM_NMP: u32 = 195;
pub const MM_NMP_ACM_AMR: u32 = 10;
pub const MM_NMP_CCP_WAVEIN: u32 = 1;
pub const MM_NMP_CCP_WAVEOUT: u32 = 2;
pub const MM_NMS: u32 = 87;
pub const MM_NOGATECH: u32 = 75;
pub const MM_NORRIS: u32 = 150;
pub const MM_NORRIS_VOICELINK: u32 = 1;
pub const MM_NORTEL_MPXAC_WAVEIN: u32 = 1;
pub const MM_NORTEL_MPXAC_WAVEOUT: u32 = 2;
pub const MM_NORTHERN_TELECOM: u32 = 115;
pub const MM_NVIDIA: u32 = 127;
pub const MM_NVIDIA_AUX: u32 = 7;
pub const MM_NVIDIA_GAMEPORT: u32 = 5;
pub const MM_NVIDIA_MIDIIN: u32 = 4;
pub const MM_NVIDIA_MIDIOUT: u32 = 3;
pub const MM_NVIDIA_MIXER: u32 = 6;
pub const MM_NVIDIA_WAVEIN: u32 = 2;
pub const MM_NVIDIA_WAVEOUT: u32 = 1;
pub const MM_OKI: u32 = 79;
pub const MM_OKSORI: u32 = 128;
pub const MM_OKSORI_BASE: u32 = 0;
pub const MM_OKSORI_EXT_MIC1: u32 = 15;
pub const MM_OKSORI_EXT_MIC2: u32 = 16;
pub const MM_OKSORI_FM_OPL4: u32 = 5;
pub const MM_OKSORI_MIDIIN: u32 = 18;
pub const MM_OKSORI_MIDIOUT: u32 = 17;
pub const MM_OKSORI_MIX_AUX1: u32 = 13;
pub const MM_OKSORI_MIX_CD: u32 = 10;
pub const MM_OKSORI_MIX_ECHO: u32 = 12;
pub const MM_OKSORI_MIX_FM: u32 = 8;
pub const MM_OKSORI_MIX_LINE: u32 = 9;
pub const MM_OKSORI_MIX_LINE1: u32 = 14;
pub const MM_OKSORI_MIX_MASTER: u32 = 6;
pub const MM_OKSORI_MIX_MIC: u32 = 11;
pub const MM_OKSORI_MIX_WAVE: u32 = 7;
pub const MM_OKSORI_MPEG_CDVISION: u32 = 19;
pub const MM_OKSORI_OSR16_WAVEIN: u32 = 4;
pub const MM_OKSORI_OSR16_WAVEOUT: u32 = 3;
pub const MM_OKSORI_OSR8_WAVEIN: u32 = 2;
pub const MM_OKSORI_OSR8_WAVEOUT: u32 = 1;
pub const MM_OLIVETTI: u32 = 81;
pub const MM_OLIVETTI_ACM_ADPCM: u32 = 10;
pub const MM_OLIVETTI_ACM_CELP: u32 = 11;
pub const MM_OLIVETTI_ACM_GSM: u32 = 9;
pub const MM_OLIVETTI_ACM_OPR: u32 = 13;
pub const MM_OLIVETTI_ACM_SBC: u32 = 12;
pub const MM_OLIVETTI_AUX: u32 = 4;
pub const MM_OLIVETTI_JOYSTICK: u32 = 8;
pub const MM_OLIVETTI_MIDIIN: u32 = 5;
pub const MM_OLIVETTI_MIDIOUT: u32 = 6;
pub const MM_OLIVETTI_MIXER: u32 = 3;
pub const MM_OLIVETTI_SYNTH: u32 = 7;
pub const MM_OLIVETTI_WAVEIN: u32 = 1;
pub const MM_OLIVETTI_WAVEOUT: u32 = 2;
pub const MM_ONLIVE: u32 = 200;
pub const MM_ONLIVE_MPCODEC: u32 = 1;
pub const MM_OPCODE: u32 = 113;
pub const MM_OPTI: u32 = 90;
pub const MM_OPTI_M16_AUX: u32 = 7;
pub const MM_OPTI_M16_FMSYNTH_STEREO: u32 = 1;
pub const MM_OPTI_M16_MIDIIN: u32 = 2;
pub const MM_OPTI_M16_MIDIOUT: u32 = 3;
pub const MM_OPTI_M16_MIXER: u32 = 6;
pub const MM_OPTI_M16_WAVEIN: u32 = 4;
pub const MM_OPTI_M16_WAVEOUT: u32 = 5;
pub const MM_OPTI_M32_AUX: u32 = 38;
pub const MM_OPTI_M32_MIDIIN: u32 = 34;
pub const MM_OPTI_M32_MIDIOUT: u32 = 35;
pub const MM_OPTI_M32_MIXER: u32 = 37;
pub const MM_OPTI_M32_SYNTH_STEREO: u32 = 36;
pub const MM_OPTI_M32_WAVEIN: u32 = 32;
pub const MM_OPTI_M32_WAVEOUT: u32 = 33;
pub const MM_OPTI_P16_AUX: u32 = 22;
pub const MM_OPTI_P16_FMSYNTH_STEREO: u32 = 16;
pub const MM_OPTI_P16_MIDIIN: u32 = 17;
pub const MM_OPTI_P16_MIDIOUT: u32 = 18;
pub const MM_OPTI_P16_MIXER: u32 = 21;
pub const MM_OPTI_P16_WAVEIN: u32 = 19;
pub const MM_OPTI_P16_WAVEOUT: u32 = 20;
pub const MM_OPUS1208_AUX: u32 = 135;
pub const MM_OPUS1208_MIXER: u32 = 134;
pub const MM_OPUS1208_SYNTH: u32 = 131;
pub const MM_OPUS1208_WAVEIN: u32 = 133;
pub const MM_OPUS1208_WAVEOUT: u32 = 132;
pub const MM_OPUS1216_AUX: u32 = 151;
pub const MM_OPUS1216_MIDIIN: u32 = 146;
pub const MM_OPUS1216_MIDIOUT: u32 = 145;
pub const MM_OPUS1216_MIXER: u32 = 150;
pub const MM_OPUS1216_SYNTH: u32 = 147;
pub const MM_OPUS1216_WAVEIN: u32 = 149;
pub const MM_OPUS1216_WAVEOUT: u32 = 148;
pub const MM_OPUS401_MIDIIN: u32 = 130;
pub const MM_OPUS401_MIDIOUT: u32 = 129;
pub const MM_OSITECH: u32 = 103;
pub const MM_OSITECH_TRUMPCARD: u32 = 1;
pub const MM_OSPREY: u32 = 140;
pub const MM_OSPREY_1000WAVEIN: u32 = 1;
pub const MM_OSPREY_1000WAVEOUT: u32 = 2;
pub const MM_OTI: u32 = 180;
pub const MM_OTI_611MIDIN: u32 = 18;
pub const MM_OTI_611MIDIOUT: u32 = 19;
pub const MM_OTI_611MIXER: u32 = 7;
pub const MM_OTI_611WAVEIN: u32 = 5;
pub const MM_OTI_611WAVEOUT: u32 = 6;
pub const MM_PACIFICRESEARCH: u32 = 210;
pub const MM_PCSPEAKER_WAVEOUT: u32 = 13;
pub const MM_PC_JOYSTICK: u32 = 12;
pub const MM_PHILIPS_ACM_LPCBB: u32 = 1;
pub const MM_PHILIPS_SPEECH_PROCESSING: u32 = 7;
pub const MM_PHONET: u32 = 203;
pub const MM_PHONET_PP_MIXER: u32 = 3;
pub const MM_PHONET_PP_WAVEIN: u32 = 2;
pub const MM_PHONET_PP_WAVEOUT: u32 = 1;
pub const MM_PICTURETEL: u32 = 138;
pub const MM_PID_UNMAPPED: u32 = 65535;
pub const MM_PINNACLE: u32 = 218;
pub const MM_PRAGMATRAX: u32 = 5;
pub const MM_PRECEPT: u32 = 153;
pub const MM_PROAUD_16_AUX: u32 = 103;
pub const MM_PROAUD_16_MIDIIN: u32 = 98;
pub const MM_PROAUD_16_MIDIOUT: u32 = 97;
pub const MM_PROAUD_16_MIXER: u32 = 102;
pub const MM_PROAUD_16_SYNTH: u32 = 99;
pub const MM_PROAUD_16_WAVEIN: u32 = 101;
pub const MM_PROAUD_16_WAVEOUT: u32 = 100;
pub const MM_PROAUD_AUX: u32 = 23;
pub const MM_PROAUD_MIDIIN: u32 = 18;
pub const MM_PROAUD_MIDIOUT: u32 = 17;
pub const MM_PROAUD_MIXER: u32 = 22;
pub const MM_PROAUD_PLUS_AUX: u32 = 87;
pub const MM_PROAUD_PLUS_MIDIIN: u32 = 82;
pub const MM_PROAUD_PLUS_MIDIOUT: u32 = 81;
pub const MM_PROAUD_PLUS_MIXER: u32 = 86;
pub const MM_PROAUD_PLUS_SYNTH: u32 = 83;
pub const MM_PROAUD_PLUS_WAVEIN: u32 = 85;
pub const MM_PROAUD_PLUS_WAVEOUT: u32 = 84;
pub const MM_PROAUD_SYNTH: u32 = 19;
pub const MM_PROAUD_WAVEIN: u32 = 21;
pub const MM_PROAUD_WAVEOUT: u32 = 20;
pub const MM_QCIAR: u32 = 98;
pub const MM_QDESIGN: u32 = 194;
pub const MM_QDESIGN_ACM_MPEG: u32 = 1;
pub const MM_QDESIGN_ACM_QDESIGN_MUSIC: u32 = 2;
pub const MM_QTEAM: u32 = 169;
pub const MM_QUALCOMM: u32 = 215;
pub const MM_QUANTUM3D: u32 = 17;
pub const MM_QUARTERDECK: u32 = 134;
pub const MM_QUARTERDECK_LHWAVEIN: u32 = 0;
pub const MM_QUARTERDECK_LHWAVEOUT: u32 = 1;
pub const MM_QUICKAUDIO: u32 = 255;
pub const MM_QUICKAUDIO_MAXIMIDI: u32 = 2;
pub const MM_QUICKAUDIO_MINIMIDI: u32 = 1;
pub const MM_QUICKNET: u32 = 173;
pub const MM_QUICKNET_PJWAVEIN: u32 = 1;
pub const MM_QUICKNET_PJWAVEOUT: u32 = 2;
pub const MM_RADIUS: u32 = 110;
pub const MM_RHETOREX: u32 = 120;
pub const MM_RHETOREX_WAVEIN: u32 = 1;
pub const MM_RHETOREX_WAVEOUT: u32 = 2;
pub const MM_RICHMOND: u32 = 257;
pub const MM_ROCKWELL: u32 = 111;
pub const MM_ROLAND: u32 = 24;
pub const MM_ROLAND_MPU401_MIDIIN: u32 = 16;
pub const MM_ROLAND_MPU401_MIDIOUT: u32 = 15;
pub const MM_ROLAND_RAP10_MIDIIN: u32 = 11;
pub const MM_ROLAND_RAP10_MIDIOUT: u32 = 10;
pub const MM_ROLAND_RAP10_SYNTH: u32 = 12;
pub const MM_ROLAND_RAP10_WAVEIN: u32 = 14;
pub const MM_ROLAND_RAP10_WAVEOUT: u32 = 13;
pub const MM_ROLAND_SC7_MIDIIN: u32 = 22;
pub const MM_ROLAND_SC7_MIDIOUT: u32 = 21;
pub const MM_ROLAND_SCP_AUX: u32 = 48;
pub const MM_ROLAND_SCP_MIDIIN: u32 = 39;
pub const MM_ROLAND_SCP_MIDIOUT: u32 = 38;
pub const MM_ROLAND_SCP_MIXER: u32 = 42;
pub const MM_ROLAND_SCP_WAVEIN: u32 = 41;
pub const MM_ROLAND_SCP_WAVEOUT: u32 = 40;
pub const MM_ROLAND_SERIAL_MIDIIN: u32 = 24;
pub const MM_ROLAND_SERIAL_MIDIOUT: u32 = 23;
pub const MM_ROLAND_SMPU_MIDIINA: u32 = 19;
pub const MM_ROLAND_SMPU_MIDIINB: u32 = 20;
pub const MM_ROLAND_SMPU_MIDIOUTA: u32 = 17;
pub const MM_ROLAND_SMPU_MIDIOUTB: u32 = 18;
pub const MM_RZS: u32 = 216;
pub const MM_RZS_ACM_TUBGSM: u32 = 1;
pub const MM_S3: u32 = 164;
pub const MM_S3_AUX: u32 = 7;
pub const MM_S3_FMSYNTH: u32 = 5;
pub const MM_S3_MIDIIN: u32 = 4;
pub const MM_S3_MIDIOUT: u32 = 3;
pub const MM_S3_MIXER: u32 = 6;
pub const MM_S3_WAVEIN: u32 = 2;
pub const MM_S3_WAVEOUT: u32 = 1;
pub const MM_SANYO: u32 = 72;
pub const MM_SANYO_ACM_LD_ADPCM: u32 = 1;
pub const MM_SCALACS: u32 = 54;
pub const MM_SEERSYS: u32 = 137;
pub const MM_SEERSYS_REALITY: u32 = 6;
pub const MM_SEERSYS_SEERMIX: u32 = 3;
pub const MM_SEERSYS_SEERSYNTH: u32 = 1;
pub const MM_SEERSYS_SEERWAVE: u32 = 2;
pub const MM_SEERSYS_WAVESYNTH: u32 = 4;
pub const MM_SEERSYS_WAVESYNTH_WG: u32 = 5;
pub const MM_SELSIUS_SYSTEMS: u32 = 234;
pub const MM_SELSIUS_SYSTEMS_RTPWAVEIN: u32 = 2;
pub const MM_SELSIUS_SYSTEMS_RTPWAVEOUT: u32 = 1;
pub const MM_SGI: u32 = 237;
pub const MM_SGI_320_MIXER: u32 = 3;
pub const MM_SGI_320_WAVEIN: u32 = 1;
pub const MM_SGI_320_WAVEOUT: u32 = 2;
pub const MM_SGI_540_MIXER: u32 = 6;
pub const MM_SGI_540_WAVEIN: u32 = 4;
pub const MM_SGI_540_WAVEOUT: u32 = 5;
pub const MM_SGI_RAD_ADAT8CHAN_WAVEIN: u32 = 19;
pub const MM_SGI_RAD_ADAT8CHAN_WAVEOUT: u32 = 32;
pub const MM_SGI_RAD_ADATMONO1_WAVEIN: u32 = 7;
pub const MM_SGI_RAD_ADATMONO1_WAVEOUT: u32 = 20;
pub const MM_SGI_RAD_ADATMONO2_WAVEIN: u32 = 8;
pub const MM_SGI_RAD_ADATMONO2_WAVEOUT: u32 = 21;
pub const MM_SGI_RAD_ADATMONO3_WAVEIN: u32 = 9;
pub const MM_SGI_RAD_ADATMONO3_WAVEOUT: u32 = 22;
pub const MM_SGI_RAD_ADATMONO4_WAVEIN: u32 = 10;
pub const MM_SGI_RAD_ADATMONO4_WAVEOUT: u32 = 23;
pub const MM_SGI_RAD_ADATMONO5_WAVEIN: u32 = 11;
pub const MM_SGI_RAD_ADATMONO5_WAVEOUT: u32 = 24;
pub const MM_SGI_RAD_ADATMONO6_WAVEIN: u32 = 12;
pub const MM_SGI_RAD_ADATMONO6_WAVEOUT: u32 = 25;
pub const MM_SGI_RAD_ADATMONO7_WAVEIN: u32 = 13;
pub const MM_SGI_RAD_ADATMONO7_WAVEOUT: u32 = 26;
pub const MM_SGI_RAD_ADATMONO8_WAVEIN: u32 = 14;
pub const MM_SGI_RAD_ADATMONO8_WAVEOUT: u32 = 27;
pub const MM_SGI_RAD_ADATSTEREO12_WAVEIN: u32 = 15;
pub const MM_SGI_RAD_ADATSTEREO12_WAVEOUT: u32 = 28;
pub const MM_SGI_RAD_ADATSTEREO32_WAVEOUT: u32 = 29;
pub const MM_SGI_RAD_ADATSTEREO34_WAVEIN: u32 = 16;
pub const MM_SGI_RAD_ADATSTEREO56_WAVEIN: u32 = 17;
pub const MM_SGI_RAD_ADATSTEREO56_WAVEOUT: u32 = 30;
pub const MM_SGI_RAD_ADATSTEREO78_WAVEIN: u32 = 18;
pub const MM_SGI_RAD_ADATSTEREO78_WAVEOUT: u32 = 31;
pub const MM_SGI_RAD_AESMONO1_WAVEIN: u32 = 33;
pub const MM_SGI_RAD_AESMONO1_WAVEOUT: u32 = 36;
pub const MM_SGI_RAD_AESMONO2_WAVEIN: u32 = 34;
pub const MM_SGI_RAD_AESMONO2_WAVEOUT: u32 = 37;
pub const MM_SGI_RAD_AESSTEREO_WAVEIN: u32 = 35;
pub const MM_SGI_RAD_AESSTEREO_WAVEOUT: u32 = 38;
pub const MM_SHARP: u32 = 183;
pub const MM_SHARP_MDC_AUX: u32 = 6;
pub const MM_SHARP_MDC_AUX_BASS: u32 = 101;
pub const MM_SHARP_MDC_AUX_CHR: u32 = 109;
pub const MM_SHARP_MDC_AUX_MASTER: u32 = 100;
pub const MM_SHARP_MDC_AUX_MIDI_VOL: u32 = 103;
pub const MM_SHARP_MDC_AUX_RVB: u32 = 108;
pub const MM_SHARP_MDC_AUX_TREBLE: u32 = 102;
pub const MM_SHARP_MDC_AUX_VOL: u32 = 107;
pub const MM_SHARP_MDC_AUX_WAVE_CHR: u32 = 106;
pub const MM_SHARP_MDC_AUX_WAVE_RVB: u32 = 105;
pub const MM_SHARP_MDC_AUX_WAVE_VOL: u32 = 104;
pub const MM_SHARP_MDC_MIDI_IN: u32 = 2;
pub const MM_SHARP_MDC_MIDI_OUT: u32 = 3;
pub const MM_SHARP_MDC_MIDI_SYNTH: u32 = 1;
pub const MM_SHARP_MDC_MIXER: u32 = 10;
pub const MM_SHARP_MDC_WAVE_IN: u32 = 4;
pub const MM_SHARP_MDC_WAVE_OUT: u32 = 5;
pub const MM_SICRESOURCE: u32 = 175;
pub const MM_SICRESOURCE_SSO3D: u32 = 2;
pub const MM_SICRESOURCE_SSOW3DI: u32 = 3;
pub const MM_SIEMENS_SBC: u32 = 201;
pub const MM_SIERRA: u32 = 40;
pub const MM_SIERRA_ARIA_AUX: u32 = 25;
pub const MM_SIERRA_ARIA_AUX2: u32 = 32;
pub const MM_SIERRA_ARIA_MIDIIN: u32 = 21;
pub const MM_SIERRA_ARIA_MIDIOUT: u32 = 20;
pub const MM_SIERRA_ARIA_SYNTH: u32 = 22;
pub const MM_SIERRA_ARIA_WAVEIN: u32 = 24;
pub const MM_SIERRA_ARIA_WAVEOUT: u32 = 23;
pub const MM_SIERRA_QUARTET_AUX_CD: u32 = 85;
pub const MM_SIERRA_QUARTET_AUX_LINE: u32 = 86;
pub const MM_SIERRA_QUARTET_AUX_MODEM: u32 = 87;
pub const MM_SIERRA_QUARTET_MIDIIN: u32 = 82;
pub const MM_SIERRA_QUARTET_MIDIOUT: u32 = 83;
pub const MM_SIERRA_QUARTET_MIXER: u32 = 88;
pub const MM_SIERRA_QUARTET_SYNTH: u32 = 84;
pub const MM_SIERRA_QUARTET_WAVEIN: u32 = 80;
pub const MM_SIERRA_QUARTET_WAVEOUT: u32 = 81;
pub const MM_SILICONSOFT: u32 = 69;
pub const MM_SILICONSOFT_SC1_WAVEIN: u32 = 1;
pub const MM_SILICONSOFT_SC1_WAVEOUT: u32 = 2;
pub const MM_SILICONSOFT_SC2_WAVEIN: u32 = 3;
pub const MM_SILICONSOFT_SC2_WAVEOUT: u32 = 4;
pub const MM_SILICONSOFT_SOUNDJR2PR_WAVEIN: u32 = 6;
pub const MM_SILICONSOFT_SOUNDJR2PR_WAVEOUT: u32 = 7;
pub const MM_SILICONSOFT_SOUNDJR2_WAVEOUT: u32 = 5;
pub const MM_SILICONSOFT_SOUNDJR3_WAVEOUT: u32 = 8;
pub const MM_SIPROLAB: u32 = 211;
pub const MM_SIPROLAB_ACELPNET: u32 = 1;
pub const MM_SNDBLST_MIDIIN: u32 = 4;
pub const MM_SNDBLST_MIDIOUT: u32 = 3;
pub const MM_SNDBLST_SYNTH: u32 = 5;
pub const MM_SNDBLST_WAVEIN: u32 = 7;
pub const MM_SNDBLST_WAVEOUT: u32 = 6;
pub const MM_SNI: u32 = 18;
pub const MM_SNI_ACM_G721: u32 = 1;
pub const MM_SOFTLAB_NSK: u32 = 228;
pub const MM_SOFTLAB_NSK_FRW_AUX: u32 = 4;
pub const MM_SOFTLAB_NSK_FRW_MIXER: u32 = 3;
pub const MM_SOFTLAB_NSK_FRW_WAVEIN: u32 = 1;
pub const MM_SOFTLAB_NSK_FRW_WAVEOUT: u32 = 2;
pub const MM_SOFTSOUND: u32 = 149;
pub const MM_SOFTSOUND_CODEC: u32 = 1;
pub const MM_SONICFOUNDRY: u32 = 66;
pub const MM_SONORUS: u32 = 230;
pub const MM_SONORUS_STUDIO: u32 = 1;
pub const MM_SONY: u32 = 245;
pub const MM_SONY_ACM_SCX: u32 = 1;
pub const MM_SORVIS: u32 = 187;
pub const MM_SOUNDESIGNS: u32 = 142;
pub const MM_SOUNDESIGNS_WAVEIN: u32 = 1;
pub const MM_SOUNDESIGNS_WAVEOUT: u32 = 2;
pub const MM_SOUNDSCAPE_AUX: u32 = 24;
pub const MM_SOUNDSCAPE_MIDIIN: u32 = 21;
pub const MM_SOUNDSCAPE_MIDIOUT: u32 = 20;
pub const MM_SOUNDSCAPE_MIXER: u32 = 23;
pub const MM_SOUNDSCAPE_SYNTH: u32 = 22;
pub const MM_SOUNDSCAPE_WAVEIN: u32 = 19;
pub const MM_SOUNDSCAPE_WAVEOUT: u32 = 17;
pub const MM_SOUNDSCAPE_WAVEOUT_AUX: u32 = 18;
pub const MM_SOUNDSPACE: u32 = 167;
pub const MM_SPECTRUM_PRODUCTIONS: u32 = 213;
pub const MM_SPECTRUM_SIGNAL_PROCESSING: u32 = 144;
pub const MM_SPEECHCOMP: u32 = 76;
pub const MM_SPLASH_STUDIOS: u32 = 133;
pub const MM_SSP_SNDFESAUX: u32 = 7;
pub const MM_SSP_SNDFESMIDIIN: u32 = 3;
pub const MM_SSP_SNDFESMIDIOUT: u32 = 4;
pub const MM_SSP_SNDFESMIX: u32 = 6;
pub const MM_SSP_SNDFESSYNTH: u32 = 5;
pub const MM_SSP_SNDFESWAVEIN: u32 = 1;
pub const MM_SSP_SNDFESWAVEOUT: u32 = 2;
pub const MM_STUDER: u32 = 171;
pub const MM_STUDIO_16_AUX: u32 = 103;
pub const MM_STUDIO_16_MIDIIN: u32 = 98;
pub const MM_STUDIO_16_MIDIOUT: u32 = 97;
pub const MM_STUDIO_16_MIXER: u32 = 102;
pub const MM_STUDIO_16_SYNTH: u32 = 99;
pub const MM_STUDIO_16_WAVEIN: u32 = 101;
pub const MM_STUDIO_16_WAVEOUT: u32 = 100;
pub const MM_ST_MICROELECTRONICS: u32 = 265;
pub const MM_SUNCOM: u32 = 186;
pub const MM_SUPERMAC: u32 = 73;
pub const MM_SYDEC_NV: u32 = 248;
pub const MM_SYDEC_NV_WAVEIN: u32 = 1;
pub const MM_SYDEC_NV_WAVEOUT: u32 = 2;
pub const MM_TANDY: u32 = 29;
pub const MM_TANDY_PSSJWAVEIN: u32 = 9;
pub const MM_TANDY_PSSJWAVEOUT: u32 = 10;
pub const MM_TANDY_SENS_MMAMIDIIN: u32 = 6;
pub const MM_TANDY_SENS_MMAMIDIOUT: u32 = 7;
pub const MM_TANDY_SENS_MMAWAVEIN: u32 = 4;
pub const MM_TANDY_SENS_MMAWAVEOUT: u32 = 5;
pub const MM_TANDY_SENS_VISWAVEOUT: u32 = 8;
pub const MM_TANDY_VISBIOSSYNTH: u32 = 3;
pub const MM_TANDY_VISWAVEIN: u32 = 1;
pub const MM_TANDY_VISWAVEOUT: u32 = 2;
pub const MM_TBS_TROPEZ_AUX1: u32 = 39;
pub const MM_TBS_TROPEZ_AUX2: u32 = 40;
pub const MM_TBS_TROPEZ_LINE: u32 = 41;
pub const MM_TBS_TROPEZ_WAVEIN: u32 = 37;
pub const MM_TBS_TROPEZ_WAVEOUT: u32 = 38;
pub const MM_TDK: u32 = 135;
pub const MM_TDK_MW_AUX: u32 = 6;
pub const MM_TDK_MW_AUX_BASS: u32 = 101;
pub const MM_TDK_MW_AUX_CHR: u32 = 109;
pub const MM_TDK_MW_AUX_MASTER: u32 = 100;
pub const MM_TDK_MW_AUX_MIDI_VOL: u32 = 103;
pub const MM_TDK_MW_AUX_RVB: u32 = 108;
pub const MM_TDK_MW_AUX_TREBLE: u32 = 102;
pub const MM_TDK_MW_AUX_VOL: u32 = 107;
pub const MM_TDK_MW_AUX_WAVE_CHR: u32 = 106;
pub const MM_TDK_MW_AUX_WAVE_RVB: u32 = 105;
pub const MM_TDK_MW_AUX_WAVE_VOL: u32 = 104;
pub const MM_TDK_MW_MIDI_IN: u32 = 2;
pub const MM_TDK_MW_MIDI_OUT: u32 = 3;
pub const MM_TDK_MW_MIDI_SYNTH: u32 = 1;
pub const MM_TDK_MW_MIXER: u32 = 10;
pub const MM_TDK_MW_WAVE_IN: u32 = 4;
pub const MM_TDK_MW_WAVE_OUT: u32 = 5;
pub const MM_TELEKOL: u32 = 264;
pub const MM_TELEKOL_WAVEIN: u32 = 2;
pub const MM_TELEKOL_WAVEOUT: u32 = 1;
pub const MM_TERALOGIC: u32 = 202;
pub const MM_TERRATEC: u32 = 70;
pub const MM_THUNDER_AUX: u32 = 39;
pub const MM_THUNDER_SYNTH: u32 = 35;
pub const MM_THUNDER_WAVEIN: u32 = 37;
pub const MM_THUNDER_WAVEOUT: u32 = 36;
pub const MM_TPORT_SYNTH: u32 = 67;
pub const MM_TPORT_WAVEIN: u32 = 66;
pub const MM_TPORT_WAVEOUT: u32 = 65;
pub const MM_TRUEVISION: u32 = 51;
pub const MM_TRUEVISION_WAVEIN1: u32 = 1;
pub const MM_TRUEVISION_WAVEOUT1: u32 = 2;
pub const MM_TTEWS_AUX: u32 = 9;
pub const MM_TTEWS_MIDIIN: u32 = 3;
pub const MM_TTEWS_MIDIMONITOR: u32 = 6;
pub const MM_TTEWS_MIDIOUT: u32 = 4;
pub const MM_TTEWS_MIDISYNTH: u32 = 5;
pub const MM_TTEWS_MIXER: u32 = 10;
pub const MM_TTEWS_VMIDIIN: u32 = 7;
pub const MM_TTEWS_VMIDIOUT: u32 = 8;
pub const MM_TTEWS_WAVEIN: u32 = 1;
pub const MM_TTEWS_WAVEOUT: u32 = 2;
pub const MM_TURTLE_BEACH: u32 = 21;
pub const MM_UHER_INFORMATIC: u32 = 247;
pub const MM_UH_ACM_ADPCM: u32 = 1;
pub const MM_UNISYS: u32 = 223;
pub const MM_UNISYS_ACM_NAP: u32 = 1;
pub const MM_UNMAPPED: u32 = 65535;
pub const MM_VAL: u32 = 35;
pub const MM_VAL_MICROKEY_AP_WAVEIN: u32 = 1;
pub const MM_VAL_MICROKEY_AP_WAVEOUT: u32 = 2;
pub const MM_VANKOEVERING: u32 = 168;
pub const MM_VIA: u32 = 250;
pub const MM_VIA_AUX: u32 = 4;
pub const MM_VIA_MIXER: u32 = 3;
pub const MM_VIA_MPU401_MIDIIN: u32 = 6;
pub const MM_VIA_MPU401_MIDIOUT: u32 = 5;
pub const MM_VIA_SWFM_SYNTH: u32 = 7;
pub const MM_VIA_WAVEIN: u32 = 2;
pub const MM_VIA_WAVEOUT: u32 = 1;
pub const MM_VIA_WDM_MIXER: u32 = 10;
pub const MM_VIA_WDM_MPU401_MIDIIN: u32 = 12;
pub const MM_VIA_WDM_MPU401_MIDIOUT: u32 = 11;
pub const MM_VIA_WDM_WAVEIN: u32 = 9;
pub const MM_VIA_WDM_WAVEOUT: u32 = 8;
pub const MM_VIDEOLOGIC: u32 = 53;
pub const MM_VIDEOLOGIC_MSWAVEIN: u32 = 1;
pub const MM_VIDEOLOGIC_MSWAVEOUT: u32 = 2;
pub const MM_VIENNASYS: u32 = 157;
pub const MM_VIENNASYS_TSP_WAVE_DRIVER: u32 = 1;
pub const MM_VIONA: u32 = 161;
pub const MM_VIONAQVINPCI_WAVEOUT: u32 = 3;
pub const MM_VIONA_BUSTER_MIXER: u32 = 4;
pub const MM_VIONA_CINEMASTER_MIXER: u32 = 5;
pub const MM_VIONA_CONCERTO_MIXER: u32 = 6;
pub const MM_VIONA_QVINPCI_MIXER: u32 = 1;
pub const MM_VIONA_QVINPCI_WAVEIN: u32 = 2;
pub const MM_VIRTUALMUSIC: u32 = 205;
pub const MM_VITEC: u32 = 67;
pub const MM_VITEC_VMAKER: u32 = 1;
pub const MM_VITEC_VMPRO: u32 = 2;
pub const MM_VIVO: u32 = 182;
pub const MM_VIVO_AUDIO_CODEC: u32 = 1;
pub const MM_VKC_MPU401_MIDIIN: u32 = 256;
pub const MM_VKC_MPU401_MIDIOUT: u32 = 512;
pub const MM_VKC_SERIAL_MIDIIN: u32 = 257;
pub const MM_VKC_SERIAL_MIDIOUT: u32 = 513;
pub const MM_VOCALTEC: u32 = 23;
pub const MM_VOCALTEC_WAVEIN: u32 = 2;
pub const MM_VOCALTEC_WAVEOUT: u32 = 1;
pub const MM_VOICEINFO: u32 = 156;
pub const MM_VOICEMIXER: u32 = 1;
pub const MM_VOXWARE: u32 = 114;
pub const MM_VOXWARE_CODEC: u32 = 1;
pub const MM_VOYETRA: u32 = 30;
pub const MM_VQST: u32 = 240;
pub const MM_VQST_VQC1: u32 = 1;
pub const MM_VQST_VQC2: u32 = 2;
pub const MM_VTG: u32 = 109;
pub const MM_WANGLABS: u32 = 28;
pub const MM_WANGLABS_WAVEIN1: u32 = 1;
pub const MM_WANGLABS_WAVEOUT1: u32 = 2;
pub const MM_WAVE_MAPPER: u32 = 2;
pub const MM_WEITEK: u32 = 96;
pub const MM_WILDCAT: u32 = 119;
pub const MM_WILDCAT_AUTOSCOREMIDIIN: u32 = 1;
pub const MM_WILLOPOND_SNDCOMM_WAVEIN: u32 = 108;
pub const MM_WILLOWPOND: u32 = 65;
pub const MM_WILLOWPOND_FMSYNTH_STEREO: u32 = 20;
pub const MM_WILLOWPOND_GENERIC_AUX: u32 = 115;
pub const MM_WILLOWPOND_GENERIC_MIXER: u32 = 114;
pub const MM_WILLOWPOND_GENERIC_WAVEIN: u32 = 112;
pub const MM_WILLOWPOND_GENERIC_WAVEOUT: u32 = 113;
pub const MM_WILLOWPOND_MPU401: u32 = 21;
pub const MM_WILLOWPOND_PH_AUX: u32 = 107;
pub const MM_WILLOWPOND_PH_MIXER: u32 = 106;
pub const MM_WILLOWPOND_PH_WAVEIN: u32 = 104;
pub const MM_WILLOWPOND_PH_WAVEOUT: u32 = 105;
pub const MM_WILLOWPOND_SNDCOMM_AUX: u32 = 111;
pub const MM_WILLOWPOND_SNDCOMM_MIXER: u32 = 110;
pub const MM_WILLOWPOND_SNDCOMM_WAVEOUT: u32 = 109;
pub const MM_WILLOWPOND_SNDPORT_AUX: u32 = 103;
pub const MM_WILLOWPOND_SNDPORT_MIXER: u32 = 102;
pub const MM_WILLOWPOND_SNDPORT_WAVEIN: u32 = 100;
pub const MM_WILLOWPOND_SNDPORT_WAVEOUT: u32 = 101;
pub const MM_WINBOND: u32 = 204;
pub const MM_WINNOV: u32 = 61;
pub const MM_WINNOV_CAVIAR_CHAMPAGNE: u32 = 4;
pub const MM_WINNOV_CAVIAR_VIDC: u32 = 3;
pub const MM_WINNOV_CAVIAR_WAVEIN: u32 = 1;
pub const MM_WINNOV_CAVIAR_WAVEOUT: u32 = 2;
pub const MM_WINNOV_CAVIAR_YUV8: u32 = 5;
pub const MM_WORKBIT: u32 = 102;
pub const MM_WORKBIT_AUX: u32 = 7;
pub const MM_WORKBIT_FMSYNTH: u32 = 6;
pub const MM_WORKBIT_JOYSTICK: u32 = 8;
pub const MM_WORKBIT_MIDIIN: u32 = 4;
pub const MM_WORKBIT_MIDIOUT: u32 = 5;
pub const MM_WORKBIT_MIXER: u32 = 1;
pub const MM_WORKBIT_WAVEIN: u32 = 3;
pub const MM_WORKBIT_WAVEOUT: u32 = 2;
pub const MM_WSS_SB16_AUX_CD: u32 = 45;
pub const MM_WSS_SB16_AUX_LINE: u32 = 44;
pub const MM_WSS_SB16_MIDIIN: u32 = 41;
pub const MM_WSS_SB16_MIDIOUT: u32 = 42;
pub const MM_WSS_SB16_MIXER: u32 = 46;
pub const MM_WSS_SB16_SYNTH: u32 = 43;
pub const MM_WSS_SB16_WAVEIN: u32 = 39;
pub const MM_WSS_SB16_WAVEOUT: u32 = 40;
pub const MM_WSS_SBPRO_AUX_CD: u32 = 53;
pub const MM_WSS_SBPRO_AUX_LINE: u32 = 52;
pub const MM_WSS_SBPRO_MIDIIN: u32 = 49;
pub const MM_WSS_SBPRO_MIDIOUT: u32 = 50;
pub const MM_WSS_SBPRO_MIXER: u32 = 54;
pub const MM_WSS_SBPRO_SYNTH: u32 = 51;
pub const MM_WSS_SBPRO_WAVEIN: u32 = 47;
pub const MM_WSS_SBPRO_WAVEOUT: u32 = 48;
pub const MM_XEBEC: u32 = 85;
pub const MM_XIRLINK: u32 = 178;
pub const MM_XIRLINK_VISIONLINK: u32 = 1;
pub const MM_XYZ: u32 = 112;
pub const MM_YAMAHA: u32 = 37;
pub const MM_YAMAHA_ACXG_AUX: u32 = 41;
pub const MM_YAMAHA_ACXG_MIDIOUT: u32 = 39;
pub const MM_YAMAHA_ACXG_MIXER: u32 = 40;
pub const MM_YAMAHA_ACXG_WAVEIN: u32 = 37;
pub const MM_YAMAHA_ACXG_WAVEOUT: u32 = 38;
pub const MM_YAMAHA_GSS_AUX: u32 = 6;
pub const MM_YAMAHA_GSS_MIDIIN: u32 = 5;
pub const MM_YAMAHA_GSS_MIDIOUT: u32 = 4;
pub const MM_YAMAHA_GSS_SYNTH: u32 = 1;
pub const MM_YAMAHA_GSS_WAVEIN: u32 = 3;
pub const MM_YAMAHA_GSS_WAVEOUT: u32 = 2;
pub const MM_YAMAHA_OPL3SA_FMSYNTH: u32 = 18;
pub const MM_YAMAHA_OPL3SA_JOYSTICK: u32 = 24;
pub const MM_YAMAHA_OPL3SA_MIDIIN: u32 = 21;
pub const MM_YAMAHA_OPL3SA_MIDIOUT: u32 = 20;
pub const MM_YAMAHA_OPL3SA_MIXER: u32 = 23;
pub const MM_YAMAHA_OPL3SA_WAVEIN: u32 = 17;
pub const MM_YAMAHA_OPL3SA_WAVEOUT: u32 = 16;
pub const MM_YAMAHA_OPL3SA_YSYNTH: u32 = 19;
pub const MM_YAMAHA_SERIAL_MIDIIN: u32 = 8;
pub const MM_YAMAHA_SERIAL_MIDIOUT: u32 = 7;
pub const MM_YAMAHA_SXG_MIDIOUT: u32 = 34;
pub const MM_YAMAHA_SXG_MIXER: u32 = 36;
pub const MM_YAMAHA_SXG_WAVEOUT: u32 = 35;
pub const MM_YAMAHA_YMF724LEG_FMSYNTH: u32 = 32;
pub const MM_YAMAHA_YMF724LEG_MIDIIN: u32 = 26;
pub const MM_YAMAHA_YMF724LEG_MIDIOUT: u32 = 25;
pub const MM_YAMAHA_YMF724LEG_MIXER: u32 = 33;
pub const MM_YAMAHA_YMF724_AUX: u32 = 30;
pub const MM_YAMAHA_YMF724_MIDIOUT: u32 = 29;
pub const MM_YAMAHA_YMF724_MIXER: u32 = 31;
pub const MM_YAMAHA_YMF724_WAVEIN: u32 = 28;
pub const MM_YAMAHA_YMF724_WAVEOUT: u32 = 27;
pub const MM_YOUCOM: u32 = 256;
pub const MM_ZEFIRO: u32 = 170;
pub const MM_ZEFIRO_ZA2: u32 = 2;
pub const MM_ZYXEL: u32 = 9;
pub const MM_ZYXEL_ACM_ADPCM: u32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct MPEG1WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub fwHeadLayer: u16,
    pub dwHeadBitrate: u32,
    pub fwHeadMode: u16,
    pub fwHeadModeExt: u16,
    pub wHeadEmphasis: u16,
    pub fwHeadFlags: u16,
    pub dwPTSLow: u32,
    pub dwPTSHigh: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct MPEGLAYER3WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wID: u16,
    pub fdwFlags: u32,
    pub nBlockSize: u16,
    pub nFramesPerBlock: u16,
    pub nCodecDelay: u16,
}
pub const MPEGLAYER3_FLAG_PADDING_ISO: u32 = 0;
pub const MPEGLAYER3_FLAG_PADDING_OFF: u32 = 2;
pub const MPEGLAYER3_FLAG_PADDING_ON: u32 = 1;
pub const MPEGLAYER3_ID_CONSTANTFRAMESIZE: u32 = 2;
pub const MPEGLAYER3_ID_MPEG: u32 = 1;
pub const MPEGLAYER3_ID_UNKNOWN: u32 = 0;
pub const MPEGLAYER3_WFX_EXTRA_BYTES: u32 = 12;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct MSAUDIO1WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
    pub wEncodeOptions: u16,
}
pub const MSAUDIO1_BITS_PER_SAMPLE: u32 = 16;
pub const MSAUDIO1_MAX_CHANNELS: u32 = 2;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct NMS_VBXADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPADPCMCOEFSET(pub *mut ADPCMCOEFSET);
impl NPADPCMCOEFSET {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPADPCMCOEFSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPADPCMEWAVEFORMAT(pub *mut ADPCMEWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPADPCMEWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPADPCMEWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPADPCMWAVEFORMAT(pub *mut ADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPAPTXWAVEFORMAT(pub *mut APTXWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPAPTXWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPAPTXWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPAUDIOFILE_AF10WAVEFORMAT(pub *mut AUDIOFILE_AF10WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPAUDIOFILE_AF10WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPAUDIOFILE_AF10WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPAUDIOFILE_AF36WAVEFORMAT(pub *mut AUDIOFILE_AF36WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPAUDIOFILE_AF36WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPAUDIOFILE_AF36WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPCONTRESCR10WAVEFORMAT(pub *mut CONTRESCR10WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPCONTRESCR10WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPCONTRESCR10WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPCONTRESVQLPCWAVEFORMAT(pub *mut CONTRESVQLPCWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPCONTRESVQLPCWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPCONTRESVQLPCWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPCREATIVEADPCMWAVEFORMAT(pub *mut CREATIVEADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPCREATIVEADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPCREATIVEADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPCREATIVEFASTSPEECH10WAVEFORMAT(pub *mut CREATIVEFASTSPEECH10WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPCREATIVEFASTSPEECH10WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPCREATIVEFASTSPEECH10WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPCREATIVEFASTSPEECH8WAVEFORMAT(pub *mut CREATIVEFASTSPEECH8WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPCREATIVEFASTSPEECH8WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPCREATIVEFASTSPEECH8WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPCSIMAADPCMWAVEFORMAT(pub *mut CSIMAADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPCSIMAADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPCSIMAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPDIALOGICOKIADPCMWAVEFORMAT(pub *mut DIALOGICOKIADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPDIALOGICOKIADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPDIALOGICOKIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPDIGIADPCMWAVEFORMAT(pub *mut DIGIADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPDIGIADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPDIGIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPDIGIFIXWAVEFORMAT(pub *mut DIGIFIXWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPDIGIFIXWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPDIGIFIXWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPDIGIREALWAVEFORMAT(pub *mut DIGIREALWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPDIGIREALWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPDIGIREALWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPDIGISTDWAVEFORMAT(pub *mut DIGISTDWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPDIGISTDWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPDIGISTDWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPDRMWAVEFORMAT(pub *mut DRMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPDRMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPDRMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPDVIADPCMWAVEFORMAT(pub *mut DVIADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPDVIADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPDVIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPECHOSC1WAVEFORMAT(pub *mut ECHOSC1WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPECHOSC1WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPECHOSC1WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPECHOWAVEFILTER(pub *mut ECHOWAVEFILTER);
impl NPECHOWAVEFILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPECHOWAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPFMTOWNS_SND_WAVEFORMAT(pub *mut FMTOWNS_SND_WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPFMTOWNS_SND_WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPFMTOWNS_SND_WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPG721_ADPCMWAVEFORMAT(pub *mut G721_ADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPG721_ADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPG721_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPG723_ADPCMWAVEFORMAT(pub *mut G723_ADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPG723_ADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPG723_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPGSM610WAVEFORMAT(pub *mut GSM610WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPGSM610WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPGSM610WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPHEAACWAVEFORMAT(pub *mut HEAACWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPHEAACWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPHEAACWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPHEAACWAVEINFO(pub *mut HEAACWAVEINFO);
#[cfg(feature = "Win32_mmeapi")]
impl NPHEAACWAVEINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPHEAACWAVEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPIMAADPCMWAVEFORMAT(pub *mut IMAADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPIMAADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPIMAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMEDIASPACEADPCMWAVEFORMAT(pub *mut MEDIASPACEADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPMEDIASPACEADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPMEDIASPACEADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMPEG1WAVEFORMAT(pub *mut MPEG1WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPMPEG1WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPMPEG1WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMPEGLAYER3WAVEFORMAT(pub *mut MPEGLAYER3WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPMPEGLAYER3WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPMPEGLAYER3WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPNMS_VBXADPCMWAVEFORMAT(pub *mut NMS_VBXADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPNMS_VBXADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPNMS_VBXADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPOLIADPCMWAVEFORMAT(pub *mut OLIADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPOLIADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPOLIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPOLICELPWAVEFORMAT(pub *mut OLICELPWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPOLICELPWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPOLICELPWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPOLIGSMWAVEFORMAT(pub *mut OLIGSMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPOLIGSMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPOLIGSMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPOLIOPRWAVEFORMAT(pub *mut OLIOPRWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPOLIOPRWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPOLIOPRWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPOLISBCWAVEFORMAT(pub *mut OLISBCWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPOLISBCWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPOLISBCWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPSIERRAADPCMWAVEFORMAT(pub *mut SIERRAADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPSIERRAADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPSIERRAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPSONARCWAVEFORMAT(pub *mut SONARCWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPSONARCWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPSONARCWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPTRUESPEECHWAVEFORMAT(pub *mut TRUESPEECHWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPTRUESPEECHWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPTRUESPEECHWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPVOLUMEWAVEFILTER(pub *mut VOLUMEWAVEFILTER);
impl NPVOLUMEWAVEFILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPVOLUMEWAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEFILTER(pub *mut WAVEFILTER);
impl NPWAVEFILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPWAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEFORMATIEEEFLOATEX(pub *mut WAVEFORMATIEEEFLOATEX);
#[cfg(feature = "Win32_mmeapi")]
impl NPWAVEFORMATIEEEFLOATEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPWAVEFORMATIEEEFLOATEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPWAVEFORMATPCMEX(pub *mut WAVEFORMATPCMEX);
#[cfg(feature = "Win32_mmeapi")]
impl NPWAVEFORMATPCMEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPWAVEFORMATPCMEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPYAMAHA_ADPCMWAVEFORMAT(pub *mut YAMAHA_ADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl NPYAMAHA_ADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for NPYAMAHA_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct OLIADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct OLICELPWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct OLIGSMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct OLIOPRWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct OLISBCWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADPCMCOEFSET(pub *mut ADPCMCOEFSET);
impl PADPCMCOEFSET {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADPCMCOEFSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADPCMEWAVEFORMAT(pub *mut ADPCMEWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PADPCMEWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PADPCMEWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADPCMWAVEFORMAT(pub *mut ADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAPTXWAVEFORMAT(pub *mut APTXWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PAPTXWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PAPTXWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUDIOFILE_AF10WAVEFORMAT(pub *mut AUDIOFILE_AF10WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PAUDIOFILE_AF10WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PAUDIOFILE_AF10WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUDIOFILE_AF36WAVEFORMAT(pub *mut AUDIOFILE_AF36WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PAUDIOFILE_AF36WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PAUDIOFILE_AF36WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONTRESCR10WAVEFORMAT(pub *mut CONTRESCR10WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PCONTRESCR10WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PCONTRESCR10WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONTRESVQLPCWAVEFORMAT(pub *mut CONTRESVQLPCWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PCONTRESVQLPCWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PCONTRESVQLPCWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCREATIVEADPCMWAVEFORMAT(pub *mut CREATIVEADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PCREATIVEADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PCREATIVEADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCREATIVEFASTSPEECH10WAVEFORMAT(pub *mut CREATIVEFASTSPEECH10WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PCREATIVEFASTSPEECH10WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PCREATIVEFASTSPEECH10WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCREATIVEFASTSPEECH8WAVEFORMAT(pub *mut CREATIVEFASTSPEECH8WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PCREATIVEFASTSPEECH8WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PCREATIVEFASTSPEECH8WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCSIMAADPCMWAVEFORMAT(pub *mut CSIMAADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PCSIMAADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PCSIMAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDIALOGICOKIADPCMWAVEFORMAT(pub *mut DIALOGICOKIADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PDIALOGICOKIADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PDIALOGICOKIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDIGIADPCMWAVEFORMAT(pub *mut DIGIADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PDIGIADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PDIGIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDIGIFIXWAVEFORMAT(pub *mut DIGIFIXWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PDIGIFIXWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PDIGIFIXWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDIGIREALWAVEFORMAT(pub *mut DIGIREALWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PDIGIREALWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PDIGIREALWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDIGISTDWAVEFORMAT(pub *mut DIGISTDWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PDIGISTDWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PDIGISTDWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDRMWAVEFORMAT(pub *mut DRMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PDRMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PDRMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDVIADPCMWAVEFORMAT(pub *mut DVIADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PDVIADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PDVIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PECHOSC1WAVEFORMAT(pub *mut ECHOSC1WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PECHOSC1WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PECHOSC1WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PECHOWAVEFILTER(pub *mut ECHOWAVEFILTER);
impl PECHOWAVEFILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PECHOWAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFMTOWNS_SND_WAVEFORMAT(pub *mut FMTOWNS_SND_WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PFMTOWNS_SND_WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PFMTOWNS_SND_WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PG721_ADPCMWAVEFORMAT(pub *mut G721_ADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PG721_ADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PG721_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PG723_ADPCMWAVEFORMAT(pub *mut G723_ADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PG723_ADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PG723_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGSM610WAVEFORMAT(pub *mut GSM610WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PGSM610WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PGSM610WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHEAACWAVEFORMAT(pub *mut HEAACWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PHEAACWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PHEAACWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHEAACWAVEINFO(pub *mut HEAACWAVEINFO);
#[cfg(feature = "Win32_mmeapi")]
impl PHEAACWAVEINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PHEAACWAVEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAADPCMWAVEFORMAT(pub *mut IMAADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PIMAADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PIMAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEDIASPACEADPCMWAVEFORMAT(pub *mut MEDIASPACEADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PMEDIASPACEADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PMEDIASPACEADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMPEG1WAVEFORMAT(pub *mut MPEG1WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PMPEG1WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PMPEG1WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMPEGLAYER3WAVEFORMAT(pub *mut MPEGLAYER3WAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PMPEGLAYER3WAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PMPEGLAYER3WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNMS_VBXADPCMWAVEFORMAT(pub *mut NMS_VBXADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PNMS_VBXADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PNMS_VBXADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POLIADPCMWAVEFORMAT(pub *mut OLIADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl POLIADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for POLIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POLICELPWAVEFORMAT(pub *mut OLICELPWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl POLICELPWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for POLICELPWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POLIGSMWAVEFORMAT(pub *mut OLIGSMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl POLIGSMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for POLIGSMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POLIOPRWAVEFORMAT(pub *mut OLIOPRWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl POLIOPRWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for POLIOPRWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POLISBCWAVEFORMAT(pub *mut OLISBCWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl POLISBCWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for POLISBCWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSIERRAADPCMWAVEFORMAT(pub *mut SIERRAADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PSIERRAADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PSIERRAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSONARCWAVEFORMAT(pub *mut SONARCWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PSONARCWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PSONARCWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRUESPEECHWAVEFORMAT(pub *mut TRUESPEECHWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PTRUESPEECHWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PTRUESPEECHWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PVOLUMEWAVEFILTER(pub *mut VOLUMEWAVEFILTER);
impl PVOLUMEWAVEFILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PVOLUMEWAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEFILTER(pub *mut WAVEFILTER);
impl PWAVEFILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEFORMATEXTENSIBLE(pub *mut WAVEFORMATEXTENSIBLE);
#[cfg(feature = "Win32_mmeapi")]
impl PWAVEFORMATEXTENSIBLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PWAVEFORMATEXTENSIBLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEFORMATIEEEFLOATEX(pub *mut WAVEFORMATIEEEFLOATEX);
#[cfg(feature = "Win32_mmeapi")]
impl PWAVEFORMATIEEEFLOATEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PWAVEFORMATIEEEFLOATEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWAVEFORMATPCMEX(pub *mut WAVEFORMATPCMEX);
#[cfg(feature = "Win32_mmeapi")]
impl PWAVEFORMATPCMEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PWAVEFORMATPCMEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PYAMAHA_ADPCMWAVEFORMAT(pub *mut YAMAHA_ADPCMWAVEFORMAT);
#[cfg(feature = "Win32_mmeapi")]
impl PYAMAHA_ADPCMWAVEFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for PYAMAHA_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RIFFCPPO: u32 = 1330663491;
pub const RIFFCPPO_byte: u32 = 1702132066;
pub const RIFFCPPO_char: u32 = 1918986339;
pub const RIFFCPPO_clsi: u32 = 1769172067;
pub const RIFFCPPO_clsr: u32 = 1920167011;
pub const RIFFCPPO_dbl: u32 = 543973988;
pub const RIFFCPPO_dwrd: u32 = 1685223268;
pub const RIFFCPPO_flt: u32 = 544500838;
pub const RIFFCPPO_int: u32 = 544501353;
pub const RIFFCPPO_long: u32 = 1735290732;
pub const RIFFCPPO_mbr: u32 = 544367213;
pub const RIFFCPPO_obji: u32 = 1768579695;
pub const RIFFCPPO_objr: u32 = 1919574639;
pub const RIFFCPPO_str: u32 = 544371827;
pub const RIFFCPPO_word: u32 = 1685221239;
pub const RIFFINFO_IARL: u32 = 1280459081;
pub const RIFFINFO_IART: u32 = 1414676809;
pub const RIFFINFO_ICMS: u32 = 1397572425;
pub const RIFFINFO_ICMT: u32 = 1414349641;
pub const RIFFINFO_ICOP: u32 = 1347371849;
pub const RIFFINFO_ICRD: u32 = 1146241865;
pub const RIFFINFO_ICRP: u32 = 1347568457;
pub const RIFFINFO_IDIM: u32 = 1296647241;
pub const RIFFINFO_IDIT: u32 = 1414087753;
pub const RIFFINFO_IDPI: u32 = 1229997129;
pub const RIFFINFO_IENG: u32 = 1196311881;
pub const RIFFINFO_IGNR: u32 = 1380861769;
pub const RIFFINFO_IKEY: u32 = 1497713481;
pub const RIFFINFO_ILGT: u32 = 1413958729;
pub const RIFFINFO_IMED: u32 = 1145392457;
pub const RIFFINFO_INAM: u32 = 1296125513;
pub const RIFFINFO_IPLT: u32 = 1414287433;
pub const RIFFINFO_IPRD: u32 = 1146245193;
pub const RIFFINFO_ISBJ: u32 = 1245860681;
pub const RIFFINFO_ISFT: u32 = 1413894985;
pub const RIFFINFO_ISHP: u32 = 1346917193;
pub const RIFFINFO_ISMP: u32 = 1347244873;
pub const RIFFINFO_ISRC: u32 = 1129468745;
pub const RIFFINFO_ISRF: u32 = 1179800393;
pub const RIFFINFO_ITCH: u32 = 1212372041;
pub const RIFFINFO_ITOC: u32 = 1129272393;
pub const RIFFINFO_ITRK: u32 = 1263686729;
pub const RIFFWAVE_inst: u32 = 1953721961;
pub const ROCKWELL_WA1_MIXER: u32 = 103;
pub const ROCKWELL_WA1_MPU401_IN: u32 = 104;
pub const ROCKWELL_WA1_MPU401_OUT: u32 = 105;
pub const ROCKWELL_WA1_SYNTH: u32 = 102;
pub const ROCKWELL_WA1_WAVEIN: u32 = 100;
pub const ROCKWELL_WA1_WAVEOUT: u32 = 101;
pub const ROCKWELL_WA2_MIXER: u32 = 203;
pub const ROCKWELL_WA2_MPU401_IN: u32 = 204;
pub const ROCKWELL_WA2_MPU401_OUT: u32 = 205;
pub const ROCKWELL_WA2_SYNTH: u32 = 202;
pub const ROCKWELL_WA2_WAVEIN: u32 = 200;
pub const ROCKWELL_WA2_WAVEOUT: u32 = 201;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct SIERRAADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wRevision: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct SONARCWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wCompType: u16,
}
pub const SPEAKER_ALL: u32 = 2147483648;
pub const SPEAKER_BACK_CENTER: u32 = 256;
pub const SPEAKER_BACK_LEFT: u32 = 16;
pub const SPEAKER_BACK_RIGHT: u32 = 32;
pub const SPEAKER_FRONT_CENTER: u32 = 4;
pub const SPEAKER_FRONT_LEFT: u32 = 1;
pub const SPEAKER_FRONT_LEFT_OF_CENTER: u32 = 64;
pub const SPEAKER_FRONT_RIGHT: u32 = 2;
pub const SPEAKER_FRONT_RIGHT_OF_CENTER: u32 = 128;
pub const SPEAKER_LOW_FREQUENCY: u32 = 8;
pub const SPEAKER_RESERVED: u32 = 2147221504;
pub const SPEAKER_SIDE_LEFT: u32 = 512;
pub const SPEAKER_SIDE_RIGHT: u32 = 1024;
pub const SPEAKER_TOP_BACK_CENTER: u32 = 65536;
pub const SPEAKER_TOP_BACK_LEFT: u32 = 32768;
pub const SPEAKER_TOP_BACK_RIGHT: u32 = 131072;
pub const SPEAKER_TOP_CENTER: u32 = 2048;
pub const SPEAKER_TOP_FRONT_CENTER: u32 = 8192;
pub const SPEAKER_TOP_FRONT_LEFT: u32 = 4096;
pub const SPEAKER_TOP_FRONT_RIGHT: u32 = 16384;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy)]
pub struct TRUESPEECHWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wRevision: u16,
    pub nSamplesPerBlock: u16,
    pub abReserved: [u8; 28],
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for TRUESPEECHWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct VOLUMEWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEFILTER {
    pub cbStruct: u32,
    pub dwFilterTag: u32,
    pub fdwFilter: u32,
    pub dwReserved: [u32; 5],
}
impl Default for WAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy)]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: super::mmeapi::WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: windows_core::GUID,
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for WAVEFORMATEXTENSIBLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy)]
pub union WAVEFORMATEXTENSIBLE_0 {
    pub wValidBitsPerSample: u16,
    pub wSamplesPerBlock: u16,
    pub wReserved: u16,
}
#[cfg(feature = "Win32_mmeapi")]
impl Default for WAVEFORMATEXTENSIBLE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_mmeapi")]
pub type WAVEFORMATIEEEFLOATEX = WAVEFORMATEXTENSIBLE;
#[cfg(feature = "Win32_mmeapi")]
pub type WAVEFORMATPCMEX = WAVEFORMATEXTENSIBLE;
pub const WAVE_FILTER_DEVELOPMENT: u32 = 65535;
pub const WAVE_FILTER_ECHO: u32 = 2;
pub const WAVE_FILTER_UNKNOWN: u32 = 0;
pub const WAVE_FILTER_VOLUME: u32 = 1;
pub const WAVE_FORMAT_3COM_NBX: u32 = 28672;
pub const WAVE_FORMAT_ADPCM: u32 = 2;
pub const WAVE_FORMAT_ALAC: u32 = 27745;
pub const WAVE_FORMAT_ALAW: u32 = 6;
pub const WAVE_FORMAT_AMR_NB: u32 = 29537;
pub const WAVE_FORMAT_AMR_WB: u32 = 29538;
pub const WAVE_FORMAT_AMR_WP: u32 = 29539;
pub const WAVE_FORMAT_ANTEX_ADPCME: u32 = 51;
pub const WAVE_FORMAT_APTX: u32 = 37;
pub const WAVE_FORMAT_AUDIOFILE_AF10: u32 = 38;
pub const WAVE_FORMAT_AUDIOFILE_AF36: u32 = 36;
pub const WAVE_FORMAT_BTV_DIGITAL: u32 = 1024;
pub const WAVE_FORMAT_CANOPUS_ATRAC: u32 = 99;
pub const WAVE_FORMAT_CIRRUS: u32 = 96;
pub const WAVE_FORMAT_CODIAN: u32 = 41252;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_AVQSBC: u32 = 41217;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_G723_1: u32 = 41216;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_SBC: u32 = 41218;
pub const WAVE_FORMAT_CONGRUENCY: u32 = 141;
pub const WAVE_FORMAT_CONTROL_RES_CR10: u32 = 55;
pub const WAVE_FORMAT_CONTROL_RES_VQLPC: u32 = 52;
pub const WAVE_FORMAT_CONVEDIA_G729: u32 = 140;
pub const WAVE_FORMAT_CREATIVE_ADPCM: u32 = 512;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH10: u32 = 515;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH8: u32 = 514;
pub const WAVE_FORMAT_CS2: u32 = 608;
pub const WAVE_FORMAT_CS_IMAADPCM: u32 = 57;
pub const WAVE_FORMAT_CUSEEME: u32 = 7939;
pub const WAVE_FORMAT_CU_CODEC: u32 = 25;
pub const WAVE_FORMAT_DEVELOPMENT: u32 = 65535;
pub const WAVE_FORMAT_DF_G726: u32 = 133;
pub const WAVE_FORMAT_DF_GSM610: u32 = 134;
pub const WAVE_FORMAT_DIALOGIC_OKI_ADPCM: u32 = 23;
pub const WAVE_FORMAT_DICTAPHONE_CELP54: u32 = 322;
pub const WAVE_FORMAT_DICTAPHONE_CELP68: u32 = 321;
pub const WAVE_FORMAT_DIGIADPCM: u32 = 54;
pub const WAVE_FORMAT_DIGIFIX: u32 = 22;
pub const WAVE_FORMAT_DIGIREAL: u32 = 53;
pub const WAVE_FORMAT_DIGISTD: u32 = 21;
pub const WAVE_FORMAT_DIGITAL_G723: u32 = 291;
pub const WAVE_FORMAT_DIVIO_G726: u32 = 16963;
pub const WAVE_FORMAT_DIVIO_MPEG4_AAC: u32 = 16707;
pub const WAVE_FORMAT_DOLBY_AC2: u32 = 48;
pub const WAVE_FORMAT_DOLBY_AC3_SPDIF: u32 = 146;
pub const WAVE_FORMAT_DOLBY_AC4: u32 = 44096;
pub const WAVE_FORMAT_DRM: u32 = 9;
pub const WAVE_FORMAT_DSAT: u32 = 102;
pub const WAVE_FORMAT_DSAT_DISPLAY: u32 = 103;
pub const WAVE_FORMAT_DSPGROUP_TRUESPEECH: u32 = 34;
pub const WAVE_FORMAT_DTS: u32 = 8;
pub const WAVE_FORMAT_DTS2: u32 = 8193;
pub const WAVE_FORMAT_DTS_DS: u32 = 400;
pub const WAVE_FORMAT_DVI_ADPCM: u32 = 17;
pub const WAVE_FORMAT_DVM: u32 = 8192;
pub const WAVE_FORMAT_ECHOSC1: u32 = 35;
pub const WAVE_FORMAT_ECHOSC3: u32 = 58;
pub const WAVE_FORMAT_ENCORE_G726: u32 = 41223;
pub const WAVE_FORMAT_ESPCM: u32 = 97;
pub const WAVE_FORMAT_ESST_AC3: u32 = 577;
pub const WAVE_FORMAT_EXTENSIBLE: u32 = 65534;
pub const WAVE_FORMAT_FAAD_AAC: u32 = 28781;
pub const WAVE_FORMAT_FLAC: u32 = 61868;
pub const WAVE_FORMAT_FM_TOWNS_SND: u32 = 768;
pub const WAVE_FORMAT_FRACE_TELECOM_G729: u32 = 41251;
pub const WAVE_FORMAT_FRAUNHOFER_IIS_MPEG2_AAC: u32 = 384;
pub const WAVE_FORMAT_G721_ADPCM: u32 = 64;
pub const WAVE_FORMAT_G722_ADPCM: u32 = 101;
pub const WAVE_FORMAT_G723_ADPCM: u32 = 20;
pub const WAVE_FORMAT_G726ADPCM: u32 = 320;
pub const WAVE_FORMAT_G726_ADPCM: u32 = 100;
pub const WAVE_FORMAT_G728_CELP: u32 = 65;
pub const WAVE_FORMAT_G729A: u32 = 131;
pub const WAVE_FORMAT_GENERIC_PASSTHRU: u32 = 585;
pub const WAVE_FORMAT_GLOBAL_IP_ILBC: u32 = 41238;
pub const WAVE_FORMAT_GSM610: u32 = 49;
pub const WAVE_FORMAT_GSM_610: u32 = 41229;
pub const WAVE_FORMAT_GSM_620: u32 = 41230;
pub const WAVE_FORMAT_GSM_660: u32 = 41231;
pub const WAVE_FORMAT_GSM_690: u32 = 41232;
pub const WAVE_FORMAT_GSM_ADAPTIVE_MULTIRATE_WB: u32 = 41233;
pub const WAVE_FORMAT_GSM_AMR_CBR: u32 = 31265;
pub const WAVE_FORMAT_GSM_AMR_VBR_SID: u32 = 31266;
pub const WAVE_FORMAT_HP_DYN_VOICE: u32 = 26;
pub const WAVE_FORMAT_IBM_CVSD: u32 = 5;
pub const WAVE_FORMAT_IEEE_FLOAT: u32 = 3;
pub const WAVE_FORMAT_ILINK_VC: u32 = 560;
pub const WAVE_FORMAT_IMA_ADPCM: u32 = 17;
pub const WAVE_FORMAT_INDEO_AUDIO: u32 = 1026;
pub const WAVE_FORMAT_INFOCOM_ITS_G721_ADPCM: u32 = 139;
pub const WAVE_FORMAT_INGENIENT_G726: u32 = 41221;
pub const WAVE_FORMAT_INNINGS_TELECOM_ADPCM: u32 = 6521;
pub const WAVE_FORMAT_INTEL_G723_1: u32 = 67;
pub const WAVE_FORMAT_INTEL_G729: u32 = 68;
pub const WAVE_FORMAT_INTEL_MUSIC_CODER: u32 = 1025;
pub const WAVE_FORMAT_IPI_HSX: u32 = 592;
pub const WAVE_FORMAT_IPI_RPELP: u32 = 593;
pub const WAVE_FORMAT_IRAT: u32 = 257;
pub const WAVE_FORMAT_ISIAUDIO: u32 = 136;
pub const WAVE_FORMAT_ISIAUDIO_2: u32 = 5121;
pub const WAVE_FORMAT_KNOWLEDGE_ADVENTURE_ADPCM: u32 = 376;
pub const WAVE_FORMAT_LEAD_SPEECH: u32 = 17228;
pub const WAVE_FORMAT_LEAD_VORBIS: u32 = 22092;
pub const WAVE_FORMAT_LH_CODEC: u32 = 4352;
pub const WAVE_FORMAT_LH_CODEC_CELP: u32 = 4353;
pub const WAVE_FORMAT_LH_CODEC_SBC12: u32 = 4355;
pub const WAVE_FORMAT_LH_CODEC_SBC16: u32 = 4356;
pub const WAVE_FORMAT_LH_CODEC_SBC8: u32 = 4354;
pub const WAVE_FORMAT_LIGHTWAVE_LOSSLESS: u32 = 2222;
pub const WAVE_FORMAT_LRC: u32 = 40;
pub const WAVE_FORMAT_LUCENT_G723: u32 = 89;
pub const WAVE_FORMAT_LUCENT_SX5363S: u32 = 7180;
pub const WAVE_FORMAT_LUCENT_SX8300P: u32 = 7175;
pub const WAVE_FORMAT_MAKEAVIS: u32 = 13075;
pub const WAVE_FORMAT_MALDEN_PHONYTALK: u32 = 160;
pub const WAVE_FORMAT_MEDIASONIC_G723: u32 = 147;
pub const WAVE_FORMAT_MEDIASPACE_ADPCM: u32 = 18;
pub const WAVE_FORMAT_MEDIAVISION_ADPCM: u32 = 24;
pub const WAVE_FORMAT_MICRONAS: u32 = 848;
pub const WAVE_FORMAT_MICRONAS_CELP833: u32 = 849;
pub const WAVE_FORMAT_MPEG: u32 = 80;
pub const WAVE_FORMAT_MPEG4_AAC: u32 = 41222;
pub const WAVE_FORMAT_MPEGLAYER3: u32 = 85;
pub const WAVE_FORMAT_MPEG_ADTS_AAC: u32 = 5632;
pub const WAVE_FORMAT_MPEG_HEAAC: u32 = 5648;
pub const WAVE_FORMAT_MPEG_LOAS: u32 = 5634;
pub const WAVE_FORMAT_MPEG_RAW_AAC: u32 = 5633;
pub const WAVE_FORMAT_MSAUDIO1: u32 = 352;
pub const WAVE_FORMAT_MSG723: u32 = 66;
pub const WAVE_FORMAT_MSNAUDIO: u32 = 50;
pub const WAVE_FORMAT_MSRT24: u32 = 130;
pub const WAVE_FORMAT_MULAW: u32 = 7;
pub const WAVE_FORMAT_MULTITUDE_FT_SX20: u32 = 138;
pub const WAVE_FORMAT_MVI_MVI2: u32 = 132;
pub const WAVE_FORMAT_NEC_AAC: u32 = 176;
pub const WAVE_FORMAT_NICE_ACA: u32 = 41240;
pub const WAVE_FORMAT_NICE_ADPCM: u32 = 41241;
pub const WAVE_FORMAT_NICE_G728: u32 = 41250;
pub const WAVE_FORMAT_NMS_VBXADPCM: u32 = 56;
pub const WAVE_FORMAT_NOKIA_ADAPTIVE_MULTIRATE: u32 = 16897;
pub const WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC: u32 = 5640;
pub const WAVE_FORMAT_NOKIA_MPEG_RAW_AAC: u32 = 5641;
pub const WAVE_FORMAT_NORCOM_VOICE_SYSTEMS_ADPCM: u32 = 645;
pub const WAVE_FORMAT_NORRIS: u32 = 5120;
pub const WAVE_FORMAT_NTCSOFT_ALF2CM_ACM: u32 = 8132;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1: u32 = 26447;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1_PLUS: u32 = 26479;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2: u32 = 26448;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2_PLUS: u32 = 26480;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3: u32 = 26449;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3_PLUS: u32 = 26481;
pub const WAVE_FORMAT_OKI_ADPCM: u32 = 16;
pub const WAVE_FORMAT_OLIADPCM: u32 = 4097;
pub const WAVE_FORMAT_OLICELP: u32 = 4098;
pub const WAVE_FORMAT_OLIGSM: u32 = 4096;
pub const WAVE_FORMAT_OLIOPR: u32 = 4100;
pub const WAVE_FORMAT_OLISBC: u32 = 4099;
pub const WAVE_FORMAT_ON2_VP6_AUDIO: u32 = 1281;
pub const WAVE_FORMAT_ON2_VP7_AUDIO: u32 = 1280;
pub const WAVE_FORMAT_ONLIVE: u32 = 137;
pub const WAVE_FORMAT_OPUS: u32 = 28751;
pub const WAVE_FORMAT_PAC: u32 = 83;
pub const WAVE_FORMAT_PACKED: u32 = 153;
pub const WAVE_FORMAT_PHILIPS_CELP: u32 = 288;
pub const WAVE_FORMAT_PHILIPS_GRUNDIG: u32 = 289;
pub const WAVE_FORMAT_PHILIPS_LPCBB: u32 = 152;
pub const WAVE_FORMAT_POLYCOM_G722: u32 = 41234;
pub const WAVE_FORMAT_POLYCOM_G728: u32 = 41235;
pub const WAVE_FORMAT_POLYCOM_G729_A: u32 = 41236;
pub const WAVE_FORMAT_POLYCOM_SIREN: u32 = 41237;
pub const WAVE_FORMAT_PROSODY_1612: u32 = 39;
pub const WAVE_FORMAT_PROSODY_8KBPS: u32 = 148;
pub const WAVE_FORMAT_QDESIGN_MUSIC: u32 = 1104;
pub const WAVE_FORMAT_QUALCOMM_HALFRATE: u32 = 337;
pub const WAVE_FORMAT_QUALCOMM_PUREVOICE: u32 = 336;
pub const WAVE_FORMAT_QUARTERDECK: u32 = 544;
pub const WAVE_FORMAT_RACAL_RECORDER_G720_A: u32 = 162;
pub const WAVE_FORMAT_RACAL_RECORDER_G723_1: u32 = 163;
pub const WAVE_FORMAT_RACAL_RECORDER_GSM: u32 = 161;
pub const WAVE_FORMAT_RACAL_RECORDER_TETRA_ACELP: u32 = 164;
pub const WAVE_FORMAT_RADIOTIME_TIME_SHIFT_RADIO: u32 = 41239;
pub const WAVE_FORMAT_RAW_AAC1: u32 = 255;
pub const WAVE_FORMAT_RAW_SPORT: u32 = 576;
pub const WAVE_FORMAT_RHETOREX_ADPCM: u32 = 256;
pub const WAVE_FORMAT_ROCKWELL_ADPCM: u32 = 59;
pub const WAVE_FORMAT_ROCKWELL_DIGITALK: u32 = 60;
pub const WAVE_FORMAT_RT24: u32 = 82;
pub const WAVE_FORMAT_SANYO_LD_ADPCM: u32 = 293;
pub const WAVE_FORMAT_SBC24: u32 = 145;
pub const WAVE_FORMAT_SHARP_G726: u32 = 69;
pub const WAVE_FORMAT_SIERRA_ADPCM: u32 = 19;
pub const WAVE_FORMAT_SIPROLAB_ACELP4800: u32 = 305;
pub const WAVE_FORMAT_SIPROLAB_ACELP8V3: u32 = 306;
pub const WAVE_FORMAT_SIPROLAB_ACEPLNET: u32 = 304;
pub const WAVE_FORMAT_SIPROLAB_G729: u32 = 307;
pub const WAVE_FORMAT_SIPROLAB_G729A: u32 = 308;
pub const WAVE_FORMAT_SIPROLAB_KELVIN: u32 = 309;
pub const WAVE_FORMAT_SOFTSOUND: u32 = 128;
pub const WAVE_FORMAT_SONARC: u32 = 33;
pub const WAVE_FORMAT_SONICFOUNDRY_LOSSLESS: u32 = 6513;
pub const WAVE_FORMAT_SONY_ATRAC3: u32 = 626;
pub const WAVE_FORMAT_SONY_SCX: u32 = 624;
pub const WAVE_FORMAT_SONY_SCY: u32 = 625;
pub const WAVE_FORMAT_SONY_SPC: u32 = 627;
pub const WAVE_FORMAT_SOUNDSPACE_MUSICOMPRESS: u32 = 5376;
pub const WAVE_FORMAT_SPEEX_VOICE: u32 = 41225;
pub const WAVE_FORMAT_SYCOM_ACM_SYC008: u32 = 372;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP54: u32 = 374;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP68: u32 = 375;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_G726L: u32 = 373;
pub const WAVE_FORMAT_SYMBOL_G729_A: u32 = 41219;
pub const WAVE_FORMAT_TELUM_AUDIO: u32 = 640;
pub const WAVE_FORMAT_TELUM_IA_AUDIO: u32 = 641;
pub const WAVE_FORMAT_TPC: u32 = 1665;
pub const WAVE_FORMAT_TUBGSM: u32 = 341;
pub const WAVE_FORMAT_UHER_ADPCM: u32 = 528;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO: u32 = 533;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO_1: u32 = 534;
pub const WAVE_FORMAT_UNISYS_NAP_16K: u32 = 371;
pub const WAVE_FORMAT_UNISYS_NAP_ADPCM: u32 = 368;
pub const WAVE_FORMAT_UNISYS_NAP_ALAW: u32 = 370;
pub const WAVE_FORMAT_UNISYS_NAP_ULAW: u32 = 369;
pub const WAVE_FORMAT_UNKNOWN: u32 = 0;
pub const WAVE_FORMAT_VIANIX_MASC: u32 = 41226;
pub const WAVE_FORMAT_VIVO_G723: u32 = 273;
pub const WAVE_FORMAT_VIVO_SIREN: u32 = 274;
pub const WAVE_FORMAT_VME_VMPCM: u32 = 1664;
pub const WAVE_FORMAT_VOCORD_G721: u32 = 41242;
pub const WAVE_FORMAT_VOCORD_G722_1: u32 = 41244;
pub const WAVE_FORMAT_VOCORD_G723_1: u32 = 41248;
pub const WAVE_FORMAT_VOCORD_G726: u32 = 41243;
pub const WAVE_FORMAT_VOCORD_G728: u32 = 41245;
pub const WAVE_FORMAT_VOCORD_G729: u32 = 41246;
pub const WAVE_FORMAT_VOCORD_G729_A: u32 = 41247;
pub const WAVE_FORMAT_VOCORD_LBC: u32 = 41249;
pub const WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC: u32 = 5642;
pub const WAVE_FORMAT_VODAFONE_MPEG_RAW_AAC: u32 = 5643;
pub const WAVE_FORMAT_VOICEAGE_AMR: u32 = 310;
pub const WAVE_FORMAT_VOICEAGE_AMR_WB: u32 = 41220;
pub const WAVE_FORMAT_VOXWARE: u32 = 98;
pub const WAVE_FORMAT_VOXWARE_AC10: u32 = 113;
pub const WAVE_FORMAT_VOXWARE_AC16: u32 = 114;
pub const WAVE_FORMAT_VOXWARE_AC20: u32 = 115;
pub const WAVE_FORMAT_VOXWARE_AC8: u32 = 112;
pub const WAVE_FORMAT_VOXWARE_BYTE_ALIGNED: u32 = 105;
pub const WAVE_FORMAT_VOXWARE_RT24: u32 = 116;
pub const WAVE_FORMAT_VOXWARE_RT24_SPEECH: u32 = 6172;
pub const WAVE_FORMAT_VOXWARE_RT29: u32 = 117;
pub const WAVE_FORMAT_VOXWARE_RT29HW: u32 = 118;
pub const WAVE_FORMAT_VOXWARE_SC3: u32 = 122;
pub const WAVE_FORMAT_VOXWARE_SC3_1: u32 = 123;
pub const WAVE_FORMAT_VOXWARE_TQ40: u32 = 121;
pub const WAVE_FORMAT_VOXWARE_TQ60: u32 = 129;
pub const WAVE_FORMAT_VOXWARE_VR12: u32 = 119;
pub const WAVE_FORMAT_VOXWARE_VR18: u32 = 120;
pub const WAVE_FORMAT_VSELP: u32 = 4;
pub const WAVE_FORMAT_WAVPACK_AUDIO: u32 = 22358;
pub const WAVE_FORMAT_WM9_SPECTRUM_ANALYZER: u32 = 41227;
pub const WAVE_FORMAT_WMASPDIF: u32 = 356;
pub const WAVE_FORMAT_WMAUDIO2: u32 = 353;
pub const WAVE_FORMAT_WMAUDIO3: u32 = 354;
pub const WAVE_FORMAT_WMAUDIO_LOSSLESS: u32 = 355;
pub const WAVE_FORMAT_WMAVOICE10: u32 = 11;
pub const WAVE_FORMAT_WMAVOICE9: u32 = 10;
pub const WAVE_FORMAT_WMF_SPECTRUM_ANAYZER: u32 = 41228;
pub const WAVE_FORMAT_XEBEC: u32 = 61;
pub const WAVE_FORMAT_YAMAHA_ADPCM: u32 = 32;
pub const WAVE_FORMAT_ZOLL_ASAO: u32 = 41224;
pub const WAVE_FORMAT_ZYXEL_ADPCM: u32 = 151;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct WMAUDIO2WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub dwSamplesPerBlock: u32,
    pub wEncodeOptions: u16,
    pub dwSuperBlockAlign: u32,
}
pub const WMAUDIO2_BITS_PER_SAMPLE: u32 = 16;
pub const WMAUDIO2_MAX_CHANNELS: u32 = 2;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct WMAUDIO3WAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
    pub wValidBitsPerSample: u16,
    pub dwChannelMask: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub wEncodeOptions: u16,
    pub wReserved3: u16,
}
pub const WMAUDIO_BITS_PER_SAMPLE: u32 = 16;
pub const WMAUDIO_MAX_CHANNELS: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct YAMAHA_ADPCMWAVEFORMAT {
    pub wfx: super::mmeapi::WAVEFORMATEX,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct s_RIFFWAVE_inst(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct tag_s_RIFFWAVE_inst {
    pub bUnshiftedNote: u8,
    pub chFineTune: i8,
    pub chGain: i8,
    pub bLowNote: u8,
    pub bHighNote: u8,
    pub bLowVelocity: u8,
    pub bHighVelocity: u8,
}

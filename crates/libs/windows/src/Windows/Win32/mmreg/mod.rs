pub const ACM_MPEG_COPYRIGHT: i32 = 2;
pub const ACM_MPEG_DUALCHANNEL: i32 = 4;
pub const ACM_MPEG_ID_MPEG1: i32 = 16;
pub const ACM_MPEG_JOINTSTEREO: i32 = 2;
pub const ACM_MPEG_LAYER1: i32 = 1;
pub const ACM_MPEG_LAYER2: i32 = 2;
pub const ACM_MPEG_LAYER3: i32 = 4;
pub const ACM_MPEG_ORIGINALHOME: i32 = 4;
pub const ACM_MPEG_PRIVATEBIT: i32 = 1;
pub const ACM_MPEG_PROTECTIONBIT: i32 = 8;
pub const ACM_MPEG_SINGLECHANNEL: i32 = 8;
pub const ACM_MPEG_STEREO: i32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ADPCMCOEFSET {
    pub iCoef1: i16,
    pub iCoef2: i16,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct ADPCMEWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub struct ADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
    pub wNumCoef: u16,
    pub aCoef: [ADPCMCOEFSET; 0],
}
#[cfg(feature = "mmeapi")]
impl Default for ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct APTXWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct AUDIOFILE_AF10WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct AUDIOFILE_AF36WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
pub const AVIIF_CONTROLFRAME: i32 = 512;
pub const BICOMP_CREATIVEYUV: u32 = 1987410275;
pub const BICOMP_IBMPHOTOMOTION: u32 = 1330464848;
pub const BICOMP_IBMULTIMOTION: u32 = 1230261333;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct CONTRESCR10WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct CONTRESVQLPCWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct CREATIVEADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wRevision: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct CREATIVEFASTSPEECH10WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wRevision: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct CREATIVEFASTSPEECH8WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wRevision: u16,
}
pub const CRYSTAL_NET_SFM_CODEC: i32 = 1;
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct CSIMAADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DIALOGICOKIADPCMWAVEFORMAT {
    pub ewf: super::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DIGIADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DIGIFIXWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DIGIREALWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DIGISTDWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DOLBYAC2WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub nAuxBitsCode: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DRMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wReserved: u16,
    pub ulContentId: u32,
    pub wfxSecure: super::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DVIADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct ECHOSC1WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ECHOWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
    pub dwDelay: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Default)]
pub struct EXBMINFOHEADER {
    pub bmi: super::BITMAPINFOHEADER,
    pub biExtDataOffset: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct FMTOWNS_SND_WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wRevision: u16,
}
pub const FOURCC_RDSP: u32 = 1347634258;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct G721_ADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub nAuxBlockSize: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct G723_ADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub cbExtraSize: u16,
    pub nAuxBlockSize: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct GSM610WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub struct HEAACWAVEFORMAT {
    pub wfInfo: HEAACWAVEINFO,
    pub pbAudioSpecificConfig: [u8; 1],
}
#[cfg(feature = "mmeapi")]
impl Default for HEAACWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct HEAACWAVEINFO {
    pub wfx: super::WAVEFORMATEX,
    pub wPayloadType: u16,
    pub wAudioProfileLevelIndication: u16,
    pub wStructType: u16,
    pub wReserved1: u16,
    pub dwReserved2: u32,
}
pub const ICTYPE_AUDIO: u32 = 1667528033;
pub const ICTYPE_VIDEO: u32 = 1667524982;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct IMAADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
pub const JIFMK_00: i32 = 65280;
pub const JIFMK_APP0: i32 = 65504;
pub const JIFMK_APP1: i32 = 65505;
pub const JIFMK_APP2: i32 = 65506;
pub const JIFMK_APP3: i32 = 65507;
pub const JIFMK_APP4: i32 = 65508;
pub const JIFMK_APP5: i32 = 65509;
pub const JIFMK_APP6: i32 = 65510;
pub const JIFMK_APP7: i32 = 65511;
pub const JIFMK_COM: i32 = 65534;
pub const JIFMK_DAC: i32 = 65484;
pub const JIFMK_DHP: i32 = 65502;
pub const JIFMK_DHT: i32 = 65476;
pub const JIFMK_DNL: i32 = 65500;
pub const JIFMK_DQT: i32 = 65499;
pub const JIFMK_DRI: i32 = 65501;
pub const JIFMK_EOI: i32 = 65497;
pub const JIFMK_EXP: i32 = 65503;
pub const JIFMK_FF: i32 = 65535;
pub const JIFMK_JPG: i32 = 65480;
pub const JIFMK_JPG0: i32 = 65520;
pub const JIFMK_JPG1: i32 = 65521;
pub const JIFMK_JPG10: i32 = 65530;
pub const JIFMK_JPG11: i32 = 65531;
pub const JIFMK_JPG12: i32 = 65532;
pub const JIFMK_JPG13: i32 = 65533;
pub const JIFMK_JPG2: i32 = 65522;
pub const JIFMK_JPG3: i32 = 65523;
pub const JIFMK_JPG4: i32 = 65524;
pub const JIFMK_JPG5: i32 = 65525;
pub const JIFMK_JPG6: i32 = 65526;
pub const JIFMK_JPG7: i32 = 65527;
pub const JIFMK_JPG8: i32 = 65528;
pub const JIFMK_JPG9: i32 = 65529;
pub const JIFMK_RES: i32 = 65282;
pub const JIFMK_RST0: i32 = 65488;
pub const JIFMK_RST1: i32 = 65489;
pub const JIFMK_RST2: i32 = 65490;
pub const JIFMK_RST3: i32 = 65491;
pub const JIFMK_RST4: i32 = 65492;
pub const JIFMK_RST5: i32 = 65493;
pub const JIFMK_RST6: i32 = 65494;
pub const JIFMK_RST7: i32 = 65495;
pub const JIFMK_SOF0: i32 = 65472;
pub const JIFMK_SOF1: i32 = 65473;
pub const JIFMK_SOF10: i32 = 65482;
pub const JIFMK_SOF11: i32 = 65483;
pub const JIFMK_SOF13: i32 = 65485;
pub const JIFMK_SOF14: i32 = 65486;
pub const JIFMK_SOF15: i32 = 65487;
pub const JIFMK_SOF2: i32 = 65474;
pub const JIFMK_SOF3: i32 = 65475;
pub const JIFMK_SOF5: i32 = 65477;
pub const JIFMK_SOF6: i32 = 65478;
pub const JIFMK_SOF7: i32 = 65479;
pub const JIFMK_SOF9: i32 = 65481;
pub const JIFMK_SOI: i32 = 65496;
pub const JIFMK_SOS: i32 = 65498;
pub const JIFMK_TEM: i32 = 65281;
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
pub const JPEG_PROCESS_BASELINE: i32 = 0;
pub const JPEG_RGB: i32 = 3;
pub const JPEG_Y: i32 = 1;
pub const JPEG_YCbCr: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDATAFORMAT_SUBTYPE_IEEE_FLOAT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDATAFORMAT_SUBTYPE_PCM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDATAFORMAT_SUBTYPE_WAVEFORMATEX(pub u8);
pub type LPADPCMCOEFSET = *mut ADPCMCOEFSET;
#[cfg(feature = "mmeapi")]
pub type LPADPCMEWAVEFORMAT = *mut ADPCMEWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPADPCMWAVEFORMAT = *mut ADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPAPTXWAVEFORMAT = *mut APTXWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPAUDIOFILE_AF10WAVEFORMAT = *mut AUDIOFILE_AF10WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPAUDIOFILE_AF36WAVEFORMAT = *mut AUDIOFILE_AF36WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPCONTRESCR10WAVEFORMAT = *mut CONTRESCR10WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPCONTRESVQLPCWAVEFORMAT = *mut CONTRESVQLPCWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPCREATIVEADPCMWAVEFORMAT = *mut CREATIVEADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPCREATIVEFASTSPEECH10WAVEFORMAT = *mut CREATIVEFASTSPEECH10WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPCREATIVEFASTSPEECH8WAVEFORMAT = *mut CREATIVEFASTSPEECH8WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPCSIMAADPCMWAVEFORMAT = *mut CSIMAADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPDIALOGICOKIADPCMWAVEFORMAT = *mut DIALOGICOKIADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPDIGIADPCMWAVEFORMAT = *mut DIGIADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPDIGIFIXWAVEFORMAT = *mut DIGIFIXWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPDIGIREALWAVEFORMAT = *mut DIGIREALWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPDIGISTDWAVEFORMAT = *mut DIGISTDWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPDRMWAVEFORMAT = *mut DRMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPDVIADPCMWAVEFORMAT = *mut DVIADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPECHOSC1WAVEFORMAT = *mut ECHOSC1WAVEFORMAT;
pub type LPECHOWAVEFILTER = *mut ECHOWAVEFILTER;
#[cfg(feature = "mmeapi")]
pub type LPFMTOWNS_SND_WAVEFORMAT = *mut FMTOWNS_SND_WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPG721_ADPCMWAVEFORMAT = *mut G721_ADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPG723_ADPCMWAVEFORMAT = *mut G723_ADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPGSM610WAVEFORMAT = *mut GSM610WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPHEAACWAVEFORMAT = *mut HEAACWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPHEAACWAVEINFO = *mut HEAACWAVEINFO;
#[cfg(feature = "mmeapi")]
pub type LPIMAADPCMWAVEFORMAT = *mut IMAADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPMEDIASPACEADPCMWAVEFORMAT = *mut MEDIASPACEADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPMPEG1WAVEFORMAT = *mut MPEG1WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPMPEGLAYER3WAVEFORMAT = *mut MPEGLAYER3WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPMSAUDIO1WAVEFORMAT = *mut MSAUDIO1WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPNMS_VBXADPCMWAVEFORMAT = *mut NMS_VBXADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPOLIADPCMWAVEFORMAT = *mut OLIADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPOLICELPWAVEFORMAT = *mut OLICELPWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPOLIGSMWAVEFORMAT = *mut OLIGSMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPOLIOPRWAVEFORMAT = *mut OLIOPRWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPOLISBCWAVEFORMAT = *mut OLISBCWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPSIERRAADPCMWAVEFORMAT = *mut SIERRAADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPSONARCWAVEFORMAT = *mut SONARCWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPTRUESPEECHWAVEFORMAT = *mut TRUESPEECHWAVEFORMAT;
pub type LPVOLUMEWAVEFILTER = *mut VOLUMEWAVEFILTER;
pub type LPWAVEFILTER = *mut WAVEFILTER;
#[cfg(feature = "mmeapi")]
pub type LPWAVEFORMATIEEEFLOATEX = *mut WAVEFORMATIEEEFLOATEX;
#[cfg(feature = "mmeapi")]
pub type LPWAVEFORMATPCMEX = *mut WAVEFORMATPCMEX;
#[cfg(feature = "mmeapi")]
pub type LPWMAUDIO2WAVEFORMAT = *mut WMAUDIO2WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPWMAUDIO3WAVEFORMAT = *mut WMAUDIO3WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type LPYAMAHA_ADPCMWAVEFORMAT = *mut YAMAHA_ADPCMWAVEFORMAT;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct MEDIASPACEADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wRevision: u16,
}
pub const MIXERCONTROL_CONTROLTYPE_SRS_MTS: i32 = 536936454;
pub const MIXERCONTROL_CONTROLTYPE_SRS_ONOFF: i32 = 536936455;
pub const MIXERCONTROL_CONTROLTYPE_SRS_SYNTHSELECT: i32 = 536936456;
pub const MJPG_DIB: u32 = 1196444237;
pub const MM_3COM: i32 = 260;
pub const MM_3COM_CB_MIXER: i32 = 1;
pub const MM_3COM_CB_WAVEIN: i32 = 2;
pub const MM_3COM_CB_WAVEOUT: i32 = 3;
pub const MM_3DFX: i32 = 262;
pub const MM_AARDVARK: i32 = 11;
pub const MM_AARDVARK_STUDIO12_WAVEIN: i32 = 2;
pub const MM_AARDVARK_STUDIO12_WAVEOUT: i32 = 1;
pub const MM_AARDVARK_STUDIO88_WAVEIN: i32 = 4;
pub const MM_AARDVARK_STUDIO88_WAVEOUT: i32 = 3;
pub const MM_ACTIVEVOICE: i32 = 225;
pub const MM_ACTIVEVOICE_ACM_VOXADPCM: i32 = 1;
pub const MM_ACULAB: i32 = 14;
pub const MM_ADDX: i32 = 118;
pub const MM_ADDX_PCTV_AUX_CD: i32 = 5;
pub const MM_ADDX_PCTV_AUX_LINE: i32 = 6;
pub const MM_ADDX_PCTV_DIGITALMIX: i32 = 1;
pub const MM_ADDX_PCTV_MIXER: i32 = 4;
pub const MM_ADDX_PCTV_WAVEIN: i32 = 2;
pub const MM_ADDX_PCTV_WAVEOUT: i32 = 3;
pub const MM_ADLACC: i32 = 91;
pub const MM_ADLIB: i32 = 9;
pub const MM_ADMOS: i32 = 235;
pub const MM_ADMOS_FM_SYNTH: i32 = 1;
pub const MM_ADMOS_QS3AMIDIIN: i32 = 3;
pub const MM_ADMOS_QS3AMIDIOUT: i32 = 2;
pub const MM_ADMOS_QS3AWAVEIN: i32 = 5;
pub const MM_ADMOS_QS3AWAVEOUT: i32 = 4;
pub const MM_AHEAD: i32 = 77;
pub const MM_AHEAD_GENERIC: i32 = 4;
pub const MM_AHEAD_MULTISOUND: i32 = 1;
pub const MM_AHEAD_PROAUDIO: i32 = 3;
pub const MM_AHEAD_SOUNDBLASTER: i32 = 2;
pub const MM_ALARIS: i32 = 174;
pub const MM_ALDIGITAL: i32 = 143;
pub const MM_ALESIS: i32 = 243;
pub const MM_ALGOVISION: i32 = 266;
pub const MM_ALGOVISION_VB80AUX: i32 = 4;
pub const MM_ALGOVISION_VB80AUX2: i32 = 5;
pub const MM_ALGOVISION_VB80MIXER: i32 = 3;
pub const MM_ALGOVISION_VB80WAVEIN: i32 = 2;
pub const MM_ALGOVISION_VB80WAVEOUT: i32 = 1;
pub const MM_AMD: i32 = 146;
pub const MM_AMD_INTERWAVE_AUX1: i32 = 10;
pub const MM_AMD_INTERWAVE_AUX2: i32 = 11;
pub const MM_AMD_INTERWAVE_AUX_CD: i32 = 13;
pub const MM_AMD_INTERWAVE_AUX_MIC: i32 = 12;
pub const MM_AMD_INTERWAVE_EX_CD: i32 = 7;
pub const MM_AMD_INTERWAVE_EX_TELEPHONY: i32 = 16;
pub const MM_AMD_INTERWAVE_JOYSTICK: i32 = 6;
pub const MM_AMD_INTERWAVE_MIDIIN: i32 = 8;
pub const MM_AMD_INTERWAVE_MIDIOUT: i32 = 9;
pub const MM_AMD_INTERWAVE_MIXER1: i32 = 4;
pub const MM_AMD_INTERWAVE_MIXER2: i32 = 5;
pub const MM_AMD_INTERWAVE_MONO_IN: i32 = 14;
pub const MM_AMD_INTERWAVE_MONO_OUT: i32 = 15;
pub const MM_AMD_INTERWAVE_STEREO_ENHANCED: i32 = 19;
pub const MM_AMD_INTERWAVE_SYNTH: i32 = 3;
pub const MM_AMD_INTERWAVE_WAVEIN: i32 = 1;
pub const MM_AMD_INTERWAVE_WAVEOUT: i32 = 2;
pub const MM_AMD_INTERWAVE_WAVEOUT_BASE: i32 = 17;
pub const MM_AMD_INTERWAVE_WAVEOUT_TREBLE: i32 = 18;
pub const MM_ANALOGDEVICES: i32 = 252;
pub const MM_ANTEX: i32 = 31;
pub const MM_ANTEX_AUDIOPORT22_FEEDTHRU: i32 = 9;
pub const MM_ANTEX_AUDIOPORT22_WAVEIN: i32 = 7;
pub const MM_ANTEX_AUDIOPORT22_WAVEOUT: i32 = 8;
pub const MM_ANTEX_SX12_WAVEIN: i32 = 1;
pub const MM_ANTEX_SX12_WAVEOUT: i32 = 2;
pub const MM_ANTEX_SX15_WAVEIN: i32 = 3;
pub const MM_ANTEX_SX15_WAVEOUT: i32 = 4;
pub const MM_ANTEX_VP625_WAVEIN: i32 = 5;
pub const MM_ANTEX_VP625_WAVEOUT: i32 = 6;
pub const MM_APICOM: i32 = 116;
pub const MM_APPLE: i32 = 99;
pub const MM_APPS: i32 = 42;
pub const MM_APT: i32 = 56;
pub const MM_APT_ACE100CD: i32 = 1;
pub const MM_ARRAY: i32 = 231;
pub const MM_ARTISOFT: i32 = 20;
pub const MM_ARTISOFT_SBWAVEIN: i32 = 1;
pub const MM_ARTISOFT_SBWAVEOUT: i32 = 2;
pub const MM_AST: i32 = 64;
pub const MM_AST_MODEMWAVE_WAVEIN: i32 = 13;
pub const MM_AST_MODEMWAVE_WAVEOUT: i32 = 14;
pub const MM_ATI: i32 = 27;
pub const MM_ATT: i32 = 185;
pub const MM_ATT_G729A: i32 = 1;
pub const MM_ATT_MICROELECTRONICS: i32 = 139;
pub const MM_AU8820_AUX: i32 = 21;
pub const MM_AU8820_MIDIIN: i32 = 23;
pub const MM_AU8820_MIDIOUT: i32 = 22;
pub const MM_AU8820_MIXER: i32 = 20;
pub const MM_AU8820_SYNTH: i32 = 17;
pub const MM_AU8820_WAVEIN: i32 = 19;
pub const MM_AU8820_WAVEOUT: i32 = 18;
pub const MM_AU8830_AUX: i32 = 37;
pub const MM_AU8830_MIDIIN: i32 = 39;
pub const MM_AU8830_MIDIOUT: i32 = 38;
pub const MM_AU8830_MIXER: i32 = 36;
pub const MM_AU8830_SYNTH: i32 = 33;
pub const MM_AU8830_WAVEIN: i32 = 35;
pub const MM_AU8830_WAVEOUT: i32 = 34;
pub const MM_AUDIOFILE: i32 = 47;
pub const MM_AUDIOPT: i32 = 74;
pub const MM_AUDIOSCIENCE: i32 = 217;
pub const MM_AURAVISION: i32 = 80;
pub const MM_AUREAL: i32 = 181;
pub const MM_AUREAL_AU8820: i32 = 16;
pub const MM_AUREAL_AU8830: i32 = 32;
pub const MM_AZTECH: i32 = 52;
pub const MM_AZTECH_AUX: i32 = 404;
pub const MM_AZTECH_AUX_CD: i32 = 401;
pub const MM_AZTECH_AUX_LINE: i32 = 402;
pub const MM_AZTECH_AUX_MIC: i32 = 403;
pub const MM_AZTECH_DSP16_FMSYNTH: i32 = 68;
pub const MM_AZTECH_DSP16_WAVEIN: i32 = 65;
pub const MM_AZTECH_DSP16_WAVEOUT: i32 = 66;
pub const MM_AZTECH_DSP16_WAVESYNTH: i32 = 70;
pub const MM_AZTECH_FMSYNTH: i32 = 20;
pub const MM_AZTECH_MIDIIN: i32 = 4;
pub const MM_AZTECH_MIDIOUT: i32 = 3;
pub const MM_AZTECH_MIXER: i32 = 21;
pub const MM_AZTECH_NOVA16_MIXER: i32 = 73;
pub const MM_AZTECH_NOVA16_WAVEIN: i32 = 71;
pub const MM_AZTECH_NOVA16_WAVEOUT: i32 = 72;
pub const MM_AZTECH_PRO16_FMSYNTH: i32 = 38;
pub const MM_AZTECH_PRO16_WAVEIN: i32 = 33;
pub const MM_AZTECH_PRO16_WAVEOUT: i32 = 34;
pub const MM_AZTECH_WASH16_MIXER: i32 = 76;
pub const MM_AZTECH_WASH16_WAVEIN: i32 = 74;
pub const MM_AZTECH_WASH16_WAVEOUT: i32 = 75;
pub const MM_AZTECH_WAVEIN: i32 = 17;
pub const MM_AZTECH_WAVEOUT: i32 = 18;
pub const MM_BCB: i32 = 192;
pub const MM_BCB_NETBOARD_10: i32 = 1;
pub const MM_BCB_TT75_10: i32 = 2;
pub const MM_BECUBED: i32 = 10;
pub const MM_BERCOS: i32 = 199;
pub const MM_BERCOS_MIXER: i32 = 2;
pub const MM_BERCOS_WAVEIN: i32 = 1;
pub const MM_BERCOS_WAVEOUT: i32 = 3;
pub const MM_BERKOM: i32 = 189;
pub const MM_BINTEC: i32 = 12;
pub const MM_BINTEC_TAPI_WAVE: i32 = 1;
pub const MM_BROOKTREE: i32 = 121;
pub const MM_BTV_AUX_CD: i32 = 8;
pub const MM_BTV_AUX_LINE: i32 = 6;
pub const MM_BTV_AUX_MIC: i32 = 7;
pub const MM_BTV_DIGITALIN: i32 = 9;
pub const MM_BTV_DIGITALOUT: i32 = 10;
pub const MM_BTV_MIDIIN: i32 = 3;
pub const MM_BTV_MIDIOUT: i32 = 4;
pub const MM_BTV_MIDISYNTH: i32 = 5;
pub const MM_BTV_MIDIWAVESTREAM: i32 = 11;
pub const MM_BTV_MIXER: i32 = 12;
pub const MM_BTV_WAVEIN: i32 = 1;
pub const MM_BTV_WAVEOUT: i32 = 2;
pub const MM_CANAM: i32 = 148;
pub const MM_CANAM_CBXWAVEIN: i32 = 2;
pub const MM_CANAM_CBXWAVEOUT: i32 = 1;
pub const MM_CANOPUS: i32 = 49;
pub const MM_CANOPUS_ACM_DVREX: i32 = 1;
pub const MM_CASIO: i32 = 162;
pub const MM_CASIO_LSG_MIDIOUT: i32 = 3;
pub const MM_CASIO_WP150_MIDIIN: i32 = 2;
pub const MM_CASIO_WP150_MIDIOUT: i32 = 1;
pub const MM_CAT: i32 = 41;
pub const MM_CAT_WAVEOUT: i32 = 1;
pub const MM_CDPC_AUX: i32 = 119;
pub const MM_CDPC_MIDIIN: i32 = 114;
pub const MM_CDPC_MIDIOUT: i32 = 113;
pub const MM_CDPC_MIXER: i32 = 118;
pub const MM_CDPC_SYNTH: i32 = 115;
pub const MM_CDPC_WAVEIN: i32 = 117;
pub const MM_CDPC_WAVEOUT: i32 = 116;
pub const MM_CHROMATIC: i32 = 155;
pub const MM_CHROMATIC_M1: i32 = 1;
pub const MM_CHROMATIC_M1_AUX: i32 = 6;
pub const MM_CHROMATIC_M1_AUX_CD: i32 = 7;
pub const MM_CHROMATIC_M1_FMSYNTH: i32 = 4;
pub const MM_CHROMATIC_M1_MIDIIN: i32 = 8;
pub const MM_CHROMATIC_M1_MIDIOUT: i32 = 9;
pub const MM_CHROMATIC_M1_MIXER: i32 = 5;
pub const MM_CHROMATIC_M1_MPEGWAVEIN: i32 = 17;
pub const MM_CHROMATIC_M1_MPEGWAVEOUT: i32 = 18;
pub const MM_CHROMATIC_M1_WAVEIN: i32 = 2;
pub const MM_CHROMATIC_M1_WAVEOUT: i32 = 3;
pub const MM_CHROMATIC_M1_WTSYNTH: i32 = 16;
pub const MM_CHROMATIC_M2: i32 = 19;
pub const MM_CHROMATIC_M2_AUX: i32 = 24;
pub const MM_CHROMATIC_M2_AUX_CD: i32 = 25;
pub const MM_CHROMATIC_M2_FMSYNTH: i32 = 22;
pub const MM_CHROMATIC_M2_MIDIIN: i32 = 32;
pub const MM_CHROMATIC_M2_MIDIOUT: i32 = 33;
pub const MM_CHROMATIC_M2_MIXER: i32 = 23;
pub const MM_CHROMATIC_M2_MPEGWAVEIN: i32 = 35;
pub const MM_CHROMATIC_M2_MPEGWAVEOUT: i32 = 36;
pub const MM_CHROMATIC_M2_WAVEIN: i32 = 20;
pub const MM_CHROMATIC_M2_WAVEOUT: i32 = 21;
pub const MM_CHROMATIC_M2_WTSYNTH: i32 = 34;
pub const MM_CIRRUSLOGIC: i32 = 105;
pub const MM_COLORGRAPH: i32 = 179;
pub const MM_COMPAQ: i32 = 92;
pub const MM_COMPAQ_BB_WAVEAUX: i32 = 3;
pub const MM_COMPAQ_BB_WAVEIN: i32 = 1;
pub const MM_COMPAQ_BB_WAVEOUT: i32 = 2;
pub const MM_COMPUSIC: i32 = 89;
pub const MM_COMPUTER_FRIENDS: i32 = 45;
pub const MM_CONCEPTS: i32 = 108;
pub const MM_CONNECTIX: i32 = 158;
pub const MM_CONNECTIX_VIDEC_CODEC: i32 = 1;
pub const MM_CONTROLRES: i32 = 84;
pub const MM_COREDYNAMICS: i32 = 147;
pub const MM_COREDYNAMICS_DYNAGRAFX_VGA: i32 = 9;
pub const MM_COREDYNAMICS_DYNAGRAFX_WAVE_IN: i32 = 10;
pub const MM_COREDYNAMICS_DYNAGRAFX_WAVE_OUT: i32 = 11;
pub const MM_COREDYNAMICS_DYNAMIXHR: i32 = 1;
pub const MM_COREDYNAMICS_DYNASONIX_AUDIO_IN: i32 = 7;
pub const MM_COREDYNAMICS_DYNASONIX_AUDIO_OUT: i32 = 8;
pub const MM_COREDYNAMICS_DYNASONIX_MIDI_IN: i32 = 3;
pub const MM_COREDYNAMICS_DYNASONIX_MIDI_OUT: i32 = 4;
pub const MM_COREDYNAMICS_DYNASONIX_SYNTH: i32 = 2;
pub const MM_COREDYNAMICS_DYNASONIX_WAVE_IN: i32 = 5;
pub const MM_COREDYNAMICS_DYNASONIX_WAVE_OUT: i32 = 6;
pub const MM_CREATIVE: i32 = 2;
pub const MM_CREATIVE_AUX_CD: i32 = 401;
pub const MM_CREATIVE_AUX_LINE: i32 = 402;
pub const MM_CREATIVE_AUX_MASTER: i32 = 404;
pub const MM_CREATIVE_AUX_MIC: i32 = 403;
pub const MM_CREATIVE_AUX_MIDI: i32 = 407;
pub const MM_CREATIVE_AUX_PCSPK: i32 = 405;
pub const MM_CREATIVE_AUX_WAVE: i32 = 406;
pub const MM_CREATIVE_FMSYNTH_MONO: i32 = 301;
pub const MM_CREATIVE_FMSYNTH_STEREO: i32 = 302;
pub const MM_CREATIVE_MIDIIN: i32 = 202;
pub const MM_CREATIVE_MIDIOUT: i32 = 201;
pub const MM_CREATIVE_MIDI_AWE32: i32 = 303;
pub const MM_CREATIVE_PHNBLST_WAVEIN: i32 = 5;
pub const MM_CREATIVE_PHNBLST_WAVEOUT: i32 = 105;
pub const MM_CREATIVE_SB15_WAVEIN: i32 = 1;
pub const MM_CREATIVE_SB15_WAVEOUT: i32 = 101;
pub const MM_CREATIVE_SB16_MIXER: i32 = 409;
pub const MM_CREATIVE_SB20_WAVEIN: i32 = 2;
pub const MM_CREATIVE_SB20_WAVEOUT: i32 = 102;
pub const MM_CREATIVE_SBP16_WAVEIN: i32 = 4;
pub const MM_CREATIVE_SBP16_WAVEOUT: i32 = 104;
pub const MM_CREATIVE_SBPRO_MIXER: i32 = 408;
pub const MM_CREATIVE_SBPRO_WAVEIN: i32 = 3;
pub const MM_CREATIVE_SBPRO_WAVEOUT: i32 = 103;
pub const MM_CRYSTAL: i32 = 132;
pub const MM_CRYSTAL_CS4232_INPUTGAIN_AUX1: i32 = 13;
pub const MM_CRYSTAL_CS4232_INPUTGAIN_LOOP: i32 = 14;
pub const MM_CRYSTAL_CS4232_MIDIIN: i32 = 9;
pub const MM_CRYSTAL_CS4232_MIDIOUT: i32 = 10;
pub const MM_CRYSTAL_CS4232_WAVEAUX_AUX1: i32 = 4;
pub const MM_CRYSTAL_CS4232_WAVEAUX_AUX2: i32 = 5;
pub const MM_CRYSTAL_CS4232_WAVEAUX_LINE: i32 = 6;
pub const MM_CRYSTAL_CS4232_WAVEAUX_MASTER: i32 = 8;
pub const MM_CRYSTAL_CS4232_WAVEAUX_MONO: i32 = 7;
pub const MM_CRYSTAL_CS4232_WAVEIN: i32 = 1;
pub const MM_CRYSTAL_CS4232_WAVEMIXER: i32 = 3;
pub const MM_CRYSTAL_CS4232_WAVEOUT: i32 = 2;
pub const MM_CRYSTAL_NET: i32 = 154;
pub const MM_CRYSTAL_SOUND_FUSION_JOYSTICK: i32 = 26;
pub const MM_CRYSTAL_SOUND_FUSION_MIDIIN: i32 = 24;
pub const MM_CRYSTAL_SOUND_FUSION_MIDIOUT: i32 = 25;
pub const MM_CRYSTAL_SOUND_FUSION_MIXER: i32 = 23;
pub const MM_CRYSTAL_SOUND_FUSION_WAVEIN: i32 = 21;
pub const MM_CRYSTAL_SOUND_FUSION_WAVEOUT: i32 = 22;
pub const MM_CS: i32 = 242;
pub const MM_CYRIX: i32 = 6;
pub const MM_CYRIX_XAAUX: i32 = 6;
pub const MM_CYRIX_XAMIDIIN: i32 = 2;
pub const MM_CYRIX_XAMIDIOUT: i32 = 3;
pub const MM_CYRIX_XAMIXER: i32 = 7;
pub const MM_CYRIX_XASYNTH: i32 = 1;
pub const MM_CYRIX_XAWAVEIN: i32 = 4;
pub const MM_CYRIX_XAWAVEOUT: i32 = 5;
pub const MM_DATAFUSION: i32 = 196;
pub const MM_DATARAN: i32 = 232;
pub const MM_DDD: i32 = 151;
pub const MM_DDD_MIDILINK_MIDIIN: i32 = 1;
pub const MM_DDD_MIDILINK_MIDIOUT: i32 = 2;
pub const MM_DF_ACM_G726: i32 = 1;
pub const MM_DF_ACM_GSM610: i32 = 2;
pub const MM_DIACOUSTICS: i32 = 129;
pub const MM_DIACOUSTICS_DRUM_ACTION: i32 = 1;
pub const MM_DIALOGIC: i32 = 93;
pub const MM_DIAMONDMM: i32 = 163;
pub const MM_DICTAPHONE: i32 = 214;
pub const MM_DICTAPHONE_G726: i32 = 1;
pub const MM_DIGIGRAM: i32 = 227;
pub const MM_DIGITAL: i32 = 100;
pub const MM_DIGITAL_ACM_G723: i32 = 3;
pub const MM_DIGITAL_AUDIO_LABS: i32 = 136;
pub const MM_DIGITAL_AUDIO_LABS_CDLX: i32 = 19;
pub const MM_DIGITAL_AUDIO_LABS_CPRO: i32 = 17;
pub const MM_DIGITAL_AUDIO_LABS_CTDIF: i32 = 20;
pub const MM_DIGITAL_AUDIO_LABS_DOC: i32 = 2;
pub const MM_DIGITAL_AUDIO_LABS_TC: i32 = 1;
pub const MM_DIGITAL_AUDIO_LABS_V8: i32 = 16;
pub const MM_DIGITAL_AUDIO_LABS_VP: i32 = 18;
pub const MM_DIGITAL_AV320_WAVEIN: i32 = 1;
pub const MM_DIGITAL_AV320_WAVEOUT: i32 = 2;
pub const MM_DIGITAL_ICM_H261: i32 = 5;
pub const MM_DIGITAL_ICM_H263: i32 = 4;
pub const MM_DIMD_AUX_LINE: i32 = 9;
pub const MM_DIMD_DIRSOUND: i32 = 1;
pub const MM_DIMD_MIDIIN: i32 = 7;
pub const MM_DIMD_MIDIOUT: i32 = 8;
pub const MM_DIMD_MIXER: i32 = 10;
pub const MM_DIMD_PLATFORM: i32 = 0;
pub const MM_DIMD_VIRTJOY: i32 = 4;
pub const MM_DIMD_VIRTMPU: i32 = 2;
pub const MM_DIMD_VIRTSB: i32 = 3;
pub const MM_DIMD_WAVEIN: i32 = 5;
pub const MM_DIMD_WAVEOUT: i32 = 6;
pub const MM_DIMD_WSS_AUX: i32 = 21;
pub const MM_DIMD_WSS_MIXER: i32 = 17;
pub const MM_DIMD_WSS_SYNTH: i32 = 76;
pub const MM_DIMD_WSS_WAVEIN: i32 = 14;
pub const MM_DIMD_WSS_WAVEOUT: i32 = 15;
pub const MM_DOLBY: i32 = 78;
pub const MM_DPSINC: i32 = 191;
pub const MM_DSP_GROUP: i32 = 43;
pub const MM_DSP_GROUP_TRUESPEECH: i32 = 1;
pub const MM_DSP_SOLUTIONS: i32 = 25;
pub const MM_DSP_SOLUTIONS_AUX: i32 = 4;
pub const MM_DSP_SOLUTIONS_SYNTH: i32 = 3;
pub const MM_DSP_SOLUTIONS_WAVEIN: i32 = 2;
pub const MM_DSP_SOLUTIONS_WAVEOUT: i32 = 1;
pub const MM_DTS: i32 = 226;
pub const MM_DTS_DS: i32 = 1;
pub const MM_DUCK: i32 = 197;
pub const MM_DVISION: i32 = 165;
pub const MM_ECHO: i32 = 39;
pub const MM_ECHO_AUX: i32 = 6;
pub const MM_ECHO_MIDIIN: i32 = 5;
pub const MM_ECHO_MIDIOUT: i32 = 4;
pub const MM_ECHO_SYNTH: i32 = 1;
pub const MM_ECHO_WAVEIN: i32 = 3;
pub const MM_ECHO_WAVEOUT: i32 = 2;
pub const MM_ECS: i32 = 145;
pub const MM_ECS_AADF_MIDI_IN: i32 = 10;
pub const MM_ECS_AADF_MIDI_OUT: i32 = 11;
pub const MM_ECS_AADF_WAVE2MIDI_IN: i32 = 12;
pub const MM_EES: i32 = 219;
pub const MM_EES_PCMIDI14: i32 = 1;
pub const MM_EES_PCMIDI14_IN: i32 = 2;
pub const MM_EES_PCMIDI14_OUT1: i32 = 3;
pub const MM_EES_PCMIDI14_OUT2: i32 = 4;
pub const MM_EES_PCMIDI14_OUT3: i32 = 5;
pub const MM_EES_PCMIDI14_OUT4: i32 = 6;
pub const MM_EMAGIC: i32 = 208;
pub const MM_EMAGIC_UNITOR8: i32 = 1;
pub const MM_EMU: i32 = 19;
pub const MM_EMU_APSMIDIIN: i32 = 2;
pub const MM_EMU_APSMIDIOUT: i32 = 3;
pub const MM_EMU_APSSYNTH: i32 = 1;
pub const MM_EMU_APSWAVEIN: i32 = 4;
pub const MM_EMU_APSWAVEOUT: i32 = 5;
pub const MM_ENET: i32 = 206;
pub const MM_ENET_T2000_HANDSETIN: i32 = 3;
pub const MM_ENET_T2000_HANDSETOUT: i32 = 4;
pub const MM_ENET_T2000_LINEIN: i32 = 1;
pub const MM_ENET_T2000_LINEOUT: i32 = 2;
pub const MM_ENSONIQ: i32 = 125;
pub const MM_ENSONIQ_SOUNDSCAPE: i32 = 16;
pub const MM_EPSON: i32 = 50;
pub const MM_EPS_FMSND: i32 = 1;
pub const MM_ESS: i32 = 46;
pub const MM_ESS_AMAUX: i32 = 3;
pub const MM_ESS_AMMIDIIN: i32 = 6;
pub const MM_ESS_AMMIDIOUT: i32 = 5;
pub const MM_ESS_AMSYNTH: i32 = 4;
pub const MM_ESS_AMWAVEIN: i32 = 2;
pub const MM_ESS_AMWAVEOUT: i32 = 1;
pub const MM_ESS_AUX_CD: i32 = 8;
pub const MM_ESS_ES1488_MIXER: i32 = 24;
pub const MM_ESS_ES1488_WAVEIN: i32 = 23;
pub const MM_ESS_ES1488_WAVEOUT: i32 = 22;
pub const MM_ESS_ES1688_MIXER: i32 = 27;
pub const MM_ESS_ES1688_WAVEIN: i32 = 26;
pub const MM_ESS_ES1688_WAVEOUT: i32 = 25;
pub const MM_ESS_ES1788_MIXER: i32 = 30;
pub const MM_ESS_ES1788_WAVEIN: i32 = 29;
pub const MM_ESS_ES1788_WAVEOUT: i32 = 28;
pub const MM_ESS_ES1868_MIXER: i32 = 36;
pub const MM_ESS_ES1868_WAVEIN: i32 = 35;
pub const MM_ESS_ES1868_WAVEOUT: i32 = 34;
pub const MM_ESS_ES1878_MIXER: i32 = 39;
pub const MM_ESS_ES1878_WAVEIN: i32 = 38;
pub const MM_ESS_ES1878_WAVEOUT: i32 = 37;
pub const MM_ESS_ES1888_MIXER: i32 = 33;
pub const MM_ESS_ES1888_WAVEIN: i32 = 32;
pub const MM_ESS_ES1888_WAVEOUT: i32 = 31;
pub const MM_ESS_ES488_MIXER: i32 = 18;
pub const MM_ESS_ES488_WAVEIN: i32 = 17;
pub const MM_ESS_ES488_WAVEOUT: i32 = 16;
pub const MM_ESS_ES688_MIXER: i32 = 21;
pub const MM_ESS_ES688_WAVEIN: i32 = 20;
pub const MM_ESS_ES688_WAVEOUT: i32 = 19;
pub const MM_ESS_MIXER: i32 = 7;
pub const MM_ESS_MPU401_MIDIIN: i32 = 10;
pub const MM_ESS_MPU401_MIDIOUT: i32 = 9;
pub const MM_ETEK: i32 = 241;
pub const MM_ETEK_KWIKMIDI_MIDIIN: i32 = 1;
pub const MM_ETEK_KWIKMIDI_MIDIOUT: i32 = 2;
pub const MM_EUPHONICS: i32 = 152;
pub const MM_EUPHONICS_AUX_CD: i32 = 1;
pub const MM_EUPHONICS_AUX_LINE: i32 = 2;
pub const MM_EUPHONICS_AUX_MASTER: i32 = 3;
pub const MM_EUPHONICS_AUX_MIC: i32 = 4;
pub const MM_EUPHONICS_AUX_MIDI: i32 = 5;
pub const MM_EUPHONICS_AUX_WAVE: i32 = 6;
pub const MM_EUPHONICS_EUSYNTH: i32 = 14;
pub const MM_EUPHONICS_FMSYNTH_MONO: i32 = 7;
pub const MM_EUPHONICS_FMSYNTH_STEREO: i32 = 8;
pub const MM_EUPHONICS_MIDIIN: i32 = 9;
pub const MM_EUPHONICS_MIDIOUT: i32 = 10;
pub const MM_EUPHONICS_MIXER: i32 = 11;
pub const MM_EUPHONICS_WAVEIN: i32 = 12;
pub const MM_EUPHONICS_WAVEOUT: i32 = 13;
pub const MM_EVEREX: i32 = 38;
pub const MM_EVEREX_CARRIER: i32 = 1;
pub const MM_EXAN: i32 = 63;
pub const MM_FAITH: i32 = 15;
pub const MM_FAST: i32 = 126;
pub const MM_FHGIIS_MPEGLAYER3: i32 = 10;
pub const MM_FHGIIS_MPEGLAYER3_ADVANCED: i32 = 12;
pub const MM_FHGIIS_MPEGLAYER3_ADVANCEDPLUS: i32 = 14;
pub const MM_FHGIIS_MPEGLAYER3_BASIC: i32 = 11;
pub const MM_FHGIIS_MPEGLAYER3_DECODE: i32 = 9;
pub const MM_FHGIIS_MPEGLAYER3_LITE: i32 = 10;
pub const MM_FHGIIS_MPEGLAYER3_PROFESSIONAL: i32 = 13;
pub const MM_FLEXION: i32 = 249;
pub const MM_FLEXION_X300_WAVEIN: i32 = 1;
pub const MM_FLEXION_X300_WAVEOUT: i32 = 2;
pub const MM_FORTEMEDIA: i32 = 229;
pub const MM_FORTEMEDIA_AUX: i32 = 5;
pub const MM_FORTEMEDIA_FMSYNC: i32 = 3;
pub const MM_FORTEMEDIA_MIXER: i32 = 4;
pub const MM_FORTEMEDIA_WAVEIN: i32 = 1;
pub const MM_FORTEMEDIA_WAVEOUT: i32 = 2;
pub const MM_FRAUNHOFER_IIS: i32 = 172;
pub const MM_FRONTIER: i32 = 160;
pub const MM_FRONTIER_WAVECENTER_MIDIIN: i32 = 1;
pub const MM_FRONTIER_WAVECENTER_MIDIOUT: i32 = 2;
pub const MM_FRONTIER_WAVECENTER_WAVEIN: i32 = 3;
pub const MM_FRONTIER_WAVECENTER_WAVEOUT: i32 = 4;
pub const MM_FTR: i32 = 198;
pub const MM_FTR_ACM: i32 = 2;
pub const MM_FTR_ENCODER_WAVEIN: i32 = 1;
pub const MM_FUJITSU: i32 = 4;
pub const MM_GADGETLABS: i32 = 159;
pub const MM_GADGETLABS_WAVE42_WAVEIN: i32 = 3;
pub const MM_GADGETLABS_WAVE42_WAVEOUT: i32 = 4;
pub const MM_GADGETLABS_WAVE44_WAVEIN: i32 = 1;
pub const MM_GADGETLABS_WAVE44_WAVEOUT: i32 = 2;
pub const MM_GADGETLABS_WAVE4_MIDIIN: i32 = 5;
pub const MM_GADGETLABS_WAVE4_MIDIOUT: i32 = 6;
pub const MM_GRANDE: i32 = 117;
pub const MM_GRAVIS: i32 = 34;
pub const MM_GUILLEMOT: i32 = 207;
pub const MM_GULBRANSEN: i32 = 130;
pub const MM_HAFTMANN: i32 = 220;
pub const MM_HAFTMANN_LPTDAC2: i32 = 1;
pub const MM_HEADSPACE: i32 = 222;
pub const MM_HEADSPACE_HAEMIXER: i32 = 4;
pub const MM_HEADSPACE_HAESYNTH: i32 = 1;
pub const MM_HEADSPACE_HAEWAVEIN: i32 = 3;
pub const MM_HEADSPACE_HAEWAVEOUT: i32 = 2;
pub const MM_HEWLETT_PACKARD: i32 = 13;
pub const MM_HEWLETT_PACKARD_CU_CODEC: i32 = 1;
pub const MM_HORIZONS: i32 = 107;
pub const MM_HP: i32 = 253;
pub const MM_HP_WAVEIN: i32 = 2;
pub const MM_HP_WAVEOUT: i32 = 1;
pub const MM_HYPERACTIVE: i32 = 246;
pub const MM_IBM: i32 = 22;
pub const MM_IBM_MWAVE_AUX: i32 = 23;
pub const MM_IBM_MWAVE_MIDIIN: i32 = 21;
pub const MM_IBM_MWAVE_MIDIOUT: i32 = 22;
pub const MM_IBM_MWAVE_MIXER: i32 = 20;
pub const MM_IBM_MWAVE_WAVEIN: i32 = 18;
pub const MM_IBM_MWAVE_WAVEOUT: i32 = 19;
pub const MM_IBM_PCMCIA_AUX: i32 = 16;
pub const MM_IBM_PCMCIA_MIDIIN: i32 = 14;
pub const MM_IBM_PCMCIA_MIDIOUT: i32 = 15;
pub const MM_IBM_PCMCIA_SYNTH: i32 = 13;
pub const MM_IBM_PCMCIA_WAVEIN: i32 = 11;
pub const MM_IBM_PCMCIA_WAVEOUT: i32 = 12;
pub const MM_IBM_THINKPAD200: i32 = 17;
pub const MM_IBM_WC_MIDIOUT: i32 = 30;
pub const MM_IBM_WC_MIXEROUT: i32 = 33;
pub const MM_IBM_WC_WAVEOUT: i32 = 31;
pub const MM_ICCC: i32 = 259;
pub const MM_ICCC_UNA3_AUX: i32 = 3;
pub const MM_ICCC_UNA3_MIXER: i32 = 4;
pub const MM_ICCC_UNA3_WAVEIN: i32 = 1;
pub const MM_ICCC_UNA3_WAVEOUT: i32 = 2;
pub const MM_ICE: i32 = 239;
pub const MM_ICE_AUX: i32 = 11;
pub const MM_ICE_MIDIIN1: i32 = 6;
pub const MM_ICE_MIDIIN2: i32 = 8;
pub const MM_ICE_MIDIOUT1: i32 = 5;
pub const MM_ICE_MIDIOUT2: i32 = 7;
pub const MM_ICE_MIXER: i32 = 10;
pub const MM_ICE_MTWAVEIN: i32 = 4;
pub const MM_ICE_MTWAVEOUT: i32 = 3;
pub const MM_ICE_SYNTH: i32 = 9;
pub const MM_ICE_WAVEIN: i32 = 2;
pub const MM_ICE_WAVEOUT: i32 = 1;
pub const MM_ICL_PS: i32 = 32;
pub const MM_ICOM_AUX: i32 = 6;
pub const MM_ICOM_LINE: i32 = 7;
pub const MM_ICOM_MIXER: i32 = 5;
pub const MM_ICOM_WAVEIN: i32 = 3;
pub const MM_ICOM_WAVEOUT: i32 = 4;
pub const MM_ICS: i32 = 57;
pub const MM_ICS_2115_LITE_MIDIOUT: i32 = 13;
pub const MM_ICS_2120_LITE_MIDIOUT: i32 = 14;
pub const MM_ICS_WAVEDECK_AUX: i32 = 4;
pub const MM_ICS_WAVEDECK_MIXER: i32 = 3;
pub const MM_ICS_WAVEDECK_SYNTH: i32 = 5;
pub const MM_ICS_WAVEDECK_WAVEIN: i32 = 2;
pub const MM_ICS_WAVEDECK_WAVEOUT: i32 = 1;
pub const MM_ICS_WAVEDEC_SB_AUX: i32 = 12;
pub const MM_ICS_WAVEDEC_SB_FM_MIDIOUT: i32 = 8;
pub const MM_ICS_WAVEDEC_SB_MIXER: i32 = 11;
pub const MM_ICS_WAVEDEC_SB_MPU401_MIDIIN: i32 = 10;
pub const MM_ICS_WAVEDEC_SB_MPU401_MIDIOUT: i32 = 9;
pub const MM_ICS_WAVEDEC_SB_WAVEIN: i32 = 7;
pub const MM_ICS_WAVEDEC_SB_WAVEOUT: i32 = 6;
pub const MM_INSOFT: i32 = 94;
pub const MM_INTEL: i32 = 33;
pub const MM_INTELOPD_AUX: i32 = 401;
pub const MM_INTELOPD_WAVEIN: i32 = 1;
pub const MM_INTELOPD_WAVEOUT: i32 = 101;
pub const MM_INTEL_NSPMODEMLINEIN: i32 = 501;
pub const MM_INTEL_NSPMODEMLINEOUT: i32 = 502;
pub const MM_INTERACTIVE: i32 = 36;
pub const MM_INTERACTIVE_WAVEIN: i32 = 69;
pub const MM_INTERACTIVE_WAVEOUT: i32 = 69;
pub const MM_INTERNET: i32 = 244;
pub const MM_INTERNET_SSW_MIDIIN: i32 = 11;
pub const MM_INTERNET_SSW_MIDIOUT: i32 = 10;
pub const MM_INTERNET_SSW_WAVEIN: i32 = 13;
pub const MM_INTERNET_SSW_WAVEOUT: i32 = 12;
pub const MM_INVISION: i32 = 188;
pub const MM_IODD: i32 = 258;
pub const MM_IOMAGIC: i32 = 82;
pub const MM_IOMAGIC_TEMPO_AUXOUT: i32 = 6;
pub const MM_IOMAGIC_TEMPO_MIDIOUT: i32 = 4;
pub const MM_IOMAGIC_TEMPO_MXDOUT: i32 = 5;
pub const MM_IOMAGIC_TEMPO_SYNTH: i32 = 3;
pub const MM_IOMAGIC_TEMPO_WAVEIN: i32 = 2;
pub const MM_IOMAGIC_TEMPO_WAVEOUT: i32 = 1;
pub const MM_IPI: i32 = 238;
pub const MM_IPI_ACM_HSX: i32 = 1;
pub const MM_IPI_ACM_RPELP: i32 = 2;
pub const MM_IPI_AT_MIXER: i32 = 6;
pub const MM_IPI_AT_WAVEIN: i32 = 5;
pub const MM_IPI_AT_WAVEOUT: i32 = 4;
pub const MM_IPI_WF_ASSS: i32 = 3;
pub const MM_ISOLUTION: i32 = 106;
pub const MM_ISOLUTION_PASCAL: i32 = 1;
pub const MM_ITERATEDSYS: i32 = 58;
pub const MM_ITERATEDSYS_FUFCODEC: i32 = 1;
pub const MM_I_LINK: i32 = 233;
pub const MM_I_LINK_VOICE_CODER: i32 = 1;
pub const MM_KAY_ELEMETRICS: i32 = 131;
pub const MM_KAY_ELEMETRICS_CSL: i32 = 17152;
pub const MM_KAY_ELEMETRICS_CSL_4CHANNEL: i32 = 17161;
pub const MM_KAY_ELEMETRICS_CSL_DAT: i32 = 17160;
pub const MM_KORG: i32 = 55;
pub const MM_KORG_1212IO_MSWAVEIN: i32 = 3;
pub const MM_KORG_1212IO_MSWAVEOUT: i32 = 4;
pub const MM_KORG_PCIF_MIDIIN: i32 = 2;
pub const MM_KORG_PCIF_MIDIOUT: i32 = 1;
pub const MM_LERNOUT_ANDHAUSPIE_LHCODECACM: i32 = 1;
pub const MM_LERNOUT_AND_HAUSPIE: i32 = 97;
pub const MM_LEXICON: i32 = 236;
pub const MM_LEXICON_STUDIO_WAVE_IN: i32 = 2;
pub const MM_LEXICON_STUDIO_WAVE_OUT: i32 = 1;
pub const MM_LOGITECH: i32 = 60;
pub const MM_LUCENT: i32 = 184;
pub const MM_LUCENT_ACM_G723: i32 = 0;
pub const MM_LUCID: i32 = 221;
pub const MM_LUCID_PCI24WAVEIN: i32 = 1;
pub const MM_LUCID_PCI24WAVEOUT: i32 = 2;
pub const MM_LUMINOSITI: i32 = 224;
pub const MM_LUMINOSITI_SCWAVEIN: i32 = 1;
pub const MM_LUMINOSITI_SCWAVEMIX: i32 = 3;
pub const MM_LUMINOSITI_SCWAVEOUT: i32 = 2;
pub const MM_LYNX: i32 = 212;
pub const MM_LYRRUS: i32 = 88;
pub const MM_LYRRUS_BRIDGE_GUITAR: i32 = 1;
pub const MM_MALDEN: i32 = 261;
pub const MM_MARIAN: i32 = 190;
pub const MM_MARIAN_ARC44WAVEIN: i32 = 1;
pub const MM_MARIAN_ARC44WAVEOUT: i32 = 2;
pub const MM_MARIAN_ARC88WAVEIN: i32 = 5;
pub const MM_MARIAN_ARC88WAVEOUT: i32 = 6;
pub const MM_MARIAN_PRODIF24WAVEIN: i32 = 3;
pub const MM_MARIAN_PRODIF24WAVEOUT: i32 = 4;
pub const MM_MATROX_DIV: i32 = 254;
pub const MM_MATSUSHITA: i32 = 83;
pub const MM_MATSUSHITA_AUX: i32 = 5;
pub const MM_MATSUSHITA_FMSYNTH_STEREO: i32 = 3;
pub const MM_MATSUSHITA_MIXER: i32 = 4;
pub const MM_MATSUSHITA_WAVEIN: i32 = 1;
pub const MM_MATSUSHITA_WAVEOUT: i32 = 2;
pub const MM_MEDIASONIC: i32 = 71;
pub const MM_MEDIASONIC_ACM_G723: i32 = 1;
pub const MM_MEDIASONIC_ICOM: i32 = 2;
pub const MM_MEDIATRIX: i32 = 141;
pub const MM_MEDIAVISION: i32 = 3;
pub const MM_MEDIAVISION_CDPC: i32 = 112;
pub const MM_MEDIAVISION_OPUS1208: i32 = 128;
pub const MM_MEDIAVISION_OPUS1216: i32 = 144;
pub const MM_MEDIAVISION_PROAUDIO: i32 = 16;
pub const MM_MEDIAVISION_PROAUDIO_16: i32 = 96;
pub const MM_MEDIAVISION_PROAUDIO_PLUS: i32 = 80;
pub const MM_MEDIAVISION_PROSTUDIO_16: i32 = 96;
pub const MM_MEDIAVISION_THUNDER: i32 = 32;
pub const MM_MEDIAVISION_TPORT: i32 = 64;
pub const MM_MELABS: i32 = 44;
pub const MM_MELABS_MIDI2GO: i32 = 1;
pub const MM_MERGING_MPEGL3: i32 = 1;
pub const MM_MERGING_TECHNOLOGIES: i32 = 177;
pub const MM_METHEUS: i32 = 59;
pub const MM_METHEUS_ZIPPER: i32 = 1;
pub const MM_MICRONAS: i32 = 251;
pub const MM_MICRONAS_CLP833: i32 = 2;
pub const MM_MICRONAS_SC4: i32 = 1;
pub const MM_MICROSOFT: i32 = 1;
pub const MM_MIDI_MAPPER: i32 = 1;
pub const MM_MINDMAKER: i32 = 263;
pub const MM_MINDMAKER_GC_MIXER: i32 = 3;
pub const MM_MINDMAKER_GC_WAVEIN: i32 = 1;
pub const MM_MINDMAKER_GC_WAVEOUT: i32 = 2;
pub const MM_MIRO: i32 = 104;
pub const MM_MIRO_DC30_MIX: i32 = 7;
pub const MM_MIRO_DC30_WAVEIN: i32 = 6;
pub const MM_MIRO_DC30_WAVEOUT: i32 = 5;
pub const MM_MIRO_MOVIEPRO: i32 = 1;
pub const MM_MIRO_VIDEOD1: i32 = 2;
pub const MM_MIRO_VIDEODC1TV: i32 = 3;
pub const MM_MIRO_VIDEOTD: i32 = 4;
pub const MM_MITEL: i32 = 16;
pub const MM_MITEL_MEDIAPATH_WAVEIN: i32 = 301;
pub const MM_MITEL_MEDIAPATH_WAVEOUT: i32 = 300;
pub const MM_MITEL_MPA_HANDSET_WAVEIN: i32 = 201;
pub const MM_MITEL_MPA_HANDSET_WAVEOUT: i32 = 200;
pub const MM_MITEL_MPA_HANDSFREE_WAVEIN: i32 = 203;
pub const MM_MITEL_MPA_HANDSFREE_WAVEOUT: i32 = 202;
pub const MM_MITEL_MPA_LINE1_WAVEIN: i32 = 205;
pub const MM_MITEL_MPA_LINE1_WAVEOUT: i32 = 204;
pub const MM_MITEL_MPA_LINE2_WAVEIN: i32 = 207;
pub const MM_MITEL_MPA_LINE2_WAVEOUT: i32 = 206;
pub const MM_MITEL_TALKTO_BRIDGED_WAVEIN: i32 = 105;
pub const MM_MITEL_TALKTO_BRIDGED_WAVEOUT: i32 = 104;
pub const MM_MITEL_TALKTO_HANDSET_WAVEIN: i32 = 103;
pub const MM_MITEL_TALKTO_HANDSET_WAVEOUT: i32 = 102;
pub const MM_MITEL_TALKTO_LINE_WAVEIN: i32 = 101;
pub const MM_MITEL_TALKTO_LINE_WAVEOUT: i32 = 100;
pub const MM_MMOTION_WAVEAUX: i32 = 1;
pub const MM_MMOTION_WAVEIN: i32 = 3;
pub const MM_MMOTION_WAVEOUT: i32 = 2;
pub const MM_MOSCOM: i32 = 68;
pub const MM_MOSCOM_VPC2400_IN: i32 = 1;
pub const MM_MOSCOM_VPC2400_OUT: i32 = 2;
pub const MM_MOTIONPIXELS: i32 = 193;
pub const MM_MOTIONPIXELS_MVI2: i32 = 1;
pub const MM_MOTOROLA: i32 = 48;
pub const MM_MOTU: i32 = 101;
pub const MM_MOTU_DTX_MIDI_IN_A: i32 = 801;
pub const MM_MOTU_DTX_MIDI_IN_B: i32 = 802;
pub const MM_MOTU_DTX_MIDI_IN_SYNC: i32 = 800;
pub const MM_MOTU_DTX_MIDI_OUT_A: i32 = 801;
pub const MM_MOTU_DTX_MIDI_OUT_B: i32 = 802;
pub const MM_MOTU_FLYER_MIDI_IN_A: i32 = 601;
pub const MM_MOTU_FLYER_MIDI_IN_B: i32 = 602;
pub const MM_MOTU_FLYER_MIDI_IN_SYNC: i32 = 600;
pub const MM_MOTU_FLYER_MIDI_OUT_A: i32 = 601;
pub const MM_MOTU_FLYER_MIDI_OUT_B: i32 = 602;
pub const MM_MOTU_MTPAV_MIDIIN_1: i32 = 901;
pub const MM_MOTU_MTPAV_MIDIIN_2: i32 = 902;
pub const MM_MOTU_MTPAV_MIDIIN_3: i32 = 903;
pub const MM_MOTU_MTPAV_MIDIIN_4: i32 = 904;
pub const MM_MOTU_MTPAV_MIDIIN_5: i32 = 905;
pub const MM_MOTU_MTPAV_MIDIIN_6: i32 = 906;
pub const MM_MOTU_MTPAV_MIDIIN_7: i32 = 907;
pub const MM_MOTU_MTPAV_MIDIIN_8: i32 = 908;
pub const MM_MOTU_MTPAV_MIDIIN_ADAT: i32 = 917;
pub const MM_MOTU_MTPAV_MIDIIN_SYNC: i32 = 900;
pub const MM_MOTU_MTPAV_MIDIOUT_1: i32 = 901;
pub const MM_MOTU_MTPAV_MIDIOUT_2: i32 = 902;
pub const MM_MOTU_MTPAV_MIDIOUT_3: i32 = 903;
pub const MM_MOTU_MTPAV_MIDIOUT_4: i32 = 904;
pub const MM_MOTU_MTPAV_MIDIOUT_5: i32 = 905;
pub const MM_MOTU_MTPAV_MIDIOUT_6: i32 = 906;
pub const MM_MOTU_MTPAV_MIDIOUT_7: i32 = 907;
pub const MM_MOTU_MTPAV_MIDIOUT_8: i32 = 908;
pub const MM_MOTU_MTPAV_MIDIOUT_ADAT: i32 = 917;
pub const MM_MOTU_MTPAV_MIDIOUT_ALL: i32 = 900;
pub const MM_MOTU_MTPAV_NET_MIDIIN_1: i32 = 909;
pub const MM_MOTU_MTPAV_NET_MIDIIN_2: i32 = 910;
pub const MM_MOTU_MTPAV_NET_MIDIIN_3: i32 = 911;
pub const MM_MOTU_MTPAV_NET_MIDIIN_4: i32 = 912;
pub const MM_MOTU_MTPAV_NET_MIDIIN_5: i32 = 913;
pub const MM_MOTU_MTPAV_NET_MIDIIN_6: i32 = 914;
pub const MM_MOTU_MTPAV_NET_MIDIIN_7: i32 = 915;
pub const MM_MOTU_MTPAV_NET_MIDIIN_8: i32 = 916;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_1: i32 = 909;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_2: i32 = 910;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_3: i32 = 911;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_4: i32 = 912;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_5: i32 = 913;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_6: i32 = 914;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_7: i32 = 915;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_8: i32 = 916;
pub const MM_MOTU_MTPII_MIDIIN_1: i32 = 201;
pub const MM_MOTU_MTPII_MIDIIN_2: i32 = 202;
pub const MM_MOTU_MTPII_MIDIIN_3: i32 = 203;
pub const MM_MOTU_MTPII_MIDIIN_4: i32 = 204;
pub const MM_MOTU_MTPII_MIDIIN_5: i32 = 205;
pub const MM_MOTU_MTPII_MIDIIN_6: i32 = 206;
pub const MM_MOTU_MTPII_MIDIIN_7: i32 = 207;
pub const MM_MOTU_MTPII_MIDIIN_8: i32 = 208;
pub const MM_MOTU_MTPII_MIDIIN_SYNC: i32 = 200;
pub const MM_MOTU_MTPII_MIDIOUT_1: i32 = 201;
pub const MM_MOTU_MTPII_MIDIOUT_2: i32 = 202;
pub const MM_MOTU_MTPII_MIDIOUT_3: i32 = 203;
pub const MM_MOTU_MTPII_MIDIOUT_4: i32 = 204;
pub const MM_MOTU_MTPII_MIDIOUT_5: i32 = 205;
pub const MM_MOTU_MTPII_MIDIOUT_6: i32 = 206;
pub const MM_MOTU_MTPII_MIDIOUT_7: i32 = 207;
pub const MM_MOTU_MTPII_MIDIOUT_8: i32 = 208;
pub const MM_MOTU_MTPII_MIDIOUT_ALL: i32 = 200;
pub const MM_MOTU_MTPII_NET_MIDIIN_1: i32 = 209;
pub const MM_MOTU_MTPII_NET_MIDIIN_2: i32 = 210;
pub const MM_MOTU_MTPII_NET_MIDIIN_3: i32 = 211;
pub const MM_MOTU_MTPII_NET_MIDIIN_4: i32 = 212;
pub const MM_MOTU_MTPII_NET_MIDIIN_5: i32 = 213;
pub const MM_MOTU_MTPII_NET_MIDIIN_6: i32 = 214;
pub const MM_MOTU_MTPII_NET_MIDIIN_7: i32 = 215;
pub const MM_MOTU_MTPII_NET_MIDIIN_8: i32 = 216;
pub const MM_MOTU_MTPII_NET_MIDIOUT_1: i32 = 209;
pub const MM_MOTU_MTPII_NET_MIDIOUT_2: i32 = 210;
pub const MM_MOTU_MTPII_NET_MIDIOUT_3: i32 = 211;
pub const MM_MOTU_MTPII_NET_MIDIOUT_4: i32 = 212;
pub const MM_MOTU_MTPII_NET_MIDIOUT_5: i32 = 213;
pub const MM_MOTU_MTPII_NET_MIDIOUT_6: i32 = 214;
pub const MM_MOTU_MTPII_NET_MIDIOUT_7: i32 = 215;
pub const MM_MOTU_MTPII_NET_MIDIOUT_8: i32 = 216;
pub const MM_MOTU_MTP_MIDIIN_1: i32 = 101;
pub const MM_MOTU_MTP_MIDIIN_2: i32 = 102;
pub const MM_MOTU_MTP_MIDIIN_3: i32 = 103;
pub const MM_MOTU_MTP_MIDIIN_4: i32 = 104;
pub const MM_MOTU_MTP_MIDIIN_5: i32 = 105;
pub const MM_MOTU_MTP_MIDIIN_6: i32 = 106;
pub const MM_MOTU_MTP_MIDIIN_7: i32 = 107;
pub const MM_MOTU_MTP_MIDIIN_8: i32 = 108;
pub const MM_MOTU_MTP_MIDIOUT_1: i32 = 101;
pub const MM_MOTU_MTP_MIDIOUT_2: i32 = 102;
pub const MM_MOTU_MTP_MIDIOUT_3: i32 = 103;
pub const MM_MOTU_MTP_MIDIOUT_4: i32 = 104;
pub const MM_MOTU_MTP_MIDIOUT_5: i32 = 105;
pub const MM_MOTU_MTP_MIDIOUT_6: i32 = 106;
pub const MM_MOTU_MTP_MIDIOUT_7: i32 = 107;
pub const MM_MOTU_MTP_MIDIOUT_8: i32 = 108;
pub const MM_MOTU_MTP_MIDIOUT_ALL: i32 = 100;
pub const MM_MOTU_MXN_MIDIIN_1: i32 = 501;
pub const MM_MOTU_MXN_MIDIIN_2: i32 = 502;
pub const MM_MOTU_MXN_MIDIIN_3: i32 = 503;
pub const MM_MOTU_MXN_MIDIIN_4: i32 = 504;
pub const MM_MOTU_MXN_MIDIIN_SYNC: i32 = 500;
pub const MM_MOTU_MXN_MIDIOUT_1: i32 = 501;
pub const MM_MOTU_MXN_MIDIOUT_2: i32 = 502;
pub const MM_MOTU_MXN_MIDIOUT_3: i32 = 503;
pub const MM_MOTU_MXN_MIDIOUT_4: i32 = 504;
pub const MM_MOTU_MXN_MIDIOUT_ALL: i32 = 500;
pub const MM_MOTU_MXPMPU_MIDIIN_1: i32 = 401;
pub const MM_MOTU_MXPMPU_MIDIIN_2: i32 = 402;
pub const MM_MOTU_MXPMPU_MIDIIN_3: i32 = 403;
pub const MM_MOTU_MXPMPU_MIDIIN_4: i32 = 404;
pub const MM_MOTU_MXPMPU_MIDIIN_5: i32 = 405;
pub const MM_MOTU_MXPMPU_MIDIIN_6: i32 = 406;
pub const MM_MOTU_MXPMPU_MIDIIN_SYNC: i32 = 400;
pub const MM_MOTU_MXPMPU_MIDIOUT_1: i32 = 401;
pub const MM_MOTU_MXPMPU_MIDIOUT_2: i32 = 402;
pub const MM_MOTU_MXPMPU_MIDIOUT_3: i32 = 403;
pub const MM_MOTU_MXPMPU_MIDIOUT_4: i32 = 404;
pub const MM_MOTU_MXPMPU_MIDIOUT_5: i32 = 405;
pub const MM_MOTU_MXPMPU_MIDIOUT_6: i32 = 406;
pub const MM_MOTU_MXPMPU_MIDIOUT_ALL: i32 = 400;
pub const MM_MOTU_MXPXT_MIDIIN_1: i32 = 1001;
pub const MM_MOTU_MXPXT_MIDIIN_2: i32 = 1002;
pub const MM_MOTU_MXPXT_MIDIIN_3: i32 = 1003;
pub const MM_MOTU_MXPXT_MIDIIN_4: i32 = 1004;
pub const MM_MOTU_MXPXT_MIDIIN_5: i32 = 1005;
pub const MM_MOTU_MXPXT_MIDIIN_6: i32 = 1006;
pub const MM_MOTU_MXPXT_MIDIIN_7: i32 = 1007;
pub const MM_MOTU_MXPXT_MIDIIN_8: i32 = 1008;
pub const MM_MOTU_MXPXT_MIDIIN_SYNC: i32 = 1000;
pub const MM_MOTU_MXPXT_MIDIOUT_1: i32 = 1001;
pub const MM_MOTU_MXPXT_MIDIOUT_2: i32 = 1002;
pub const MM_MOTU_MXPXT_MIDIOUT_3: i32 = 1003;
pub const MM_MOTU_MXPXT_MIDIOUT_4: i32 = 1004;
pub const MM_MOTU_MXPXT_MIDIOUT_5: i32 = 1005;
pub const MM_MOTU_MXPXT_MIDIOUT_6: i32 = 1006;
pub const MM_MOTU_MXPXT_MIDIOUT_7: i32 = 1007;
pub const MM_MOTU_MXPXT_MIDIOUT_8: i32 = 1008;
pub const MM_MOTU_MXPXT_MIDIOUT_ALL: i32 = 1000;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_1: i32 = 301;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_2: i32 = 302;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_3: i32 = 303;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_4: i32 = 304;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_5: i32 = 305;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_6: i32 = 306;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_1: i32 = 301;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_2: i32 = 302;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_3: i32 = 303;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_4: i32 = 304;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_5: i32 = 305;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_6: i32 = 306;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_ALL: i32 = 300;
pub const MM_MOTU_MXP_MIDIIN_SYNC: i32 = 300;
pub const MM_MOTU_PKX_MIDI_IN_A: i32 = 701;
pub const MM_MOTU_PKX_MIDI_IN_B: i32 = 702;
pub const MM_MOTU_PKX_MIDI_IN_SYNC: i32 = 700;
pub const MM_MOTU_PKX_MIDI_OUT_A: i32 = 701;
pub const MM_MOTU_PKX_MIDI_OUT_B: i32 = 702;
pub const MM_MPTUS: i32 = 95;
pub const MM_MPTUS_SPWAVEOUT: i32 = 1;
pub const MM_MPU401_MIDIIN: i32 = 11;
pub const MM_MPU401_MIDIOUT: i32 = 10;
pub const MM_MSFT_ACM_G711: i32 = 37;
pub const MM_MSFT_ACM_GSM610: i32 = 36;
pub const MM_MSFT_ACM_IMAADPCM: i32 = 34;
pub const MM_MSFT_ACM_MSADPCM: i32 = 33;
pub const MM_MSFT_ACM_MSAUDIO1: i32 = 39;
pub const MM_MSFT_ACM_MSFILTER: i32 = 35;
pub const MM_MSFT_ACM_MSG723: i32 = 92;
pub const MM_MSFT_ACM_MSNAUDIO: i32 = 91;
pub const MM_MSFT_ACM_MSRT24: i32 = 93;
pub const MM_MSFT_ACM_PCM: i32 = 38;
pub const MM_MSFT_ACM_WMAUDIO: i32 = 39;
pub const MM_MSFT_ACM_WMAUDIO2: i32 = 101;
pub const MM_MSFT_GENERIC_AUX_CD: i32 = 30;
pub const MM_MSFT_GENERIC_AUX_LINE: i32 = 28;
pub const MM_MSFT_GENERIC_AUX_MIC: i32 = 29;
pub const MM_MSFT_GENERIC_MIDIIN: i32 = 25;
pub const MM_MSFT_GENERIC_MIDIOUT: i32 = 26;
pub const MM_MSFT_GENERIC_MIDISYNTH: i32 = 27;
pub const MM_MSFT_GENERIC_WAVEIN: i32 = 23;
pub const MM_MSFT_GENERIC_WAVEOUT: i32 = 24;
pub const MM_MSFT_MSACM: i32 = 32;
pub const MM_MSFT_MSOPL_SYNTH: i32 = 76;
pub const MM_MSFT_SB16_AUX_CD: i32 = 66;
pub const MM_MSFT_SB16_AUX_LINE: i32 = 65;
pub const MM_MSFT_SB16_MIDIIN: i32 = 62;
pub const MM_MSFT_SB16_MIDIOUT: i32 = 63;
pub const MM_MSFT_SB16_MIXER: i32 = 67;
pub const MM_MSFT_SB16_SYNTH: i32 = 64;
pub const MM_MSFT_SB16_WAVEIN: i32 = 60;
pub const MM_MSFT_SB16_WAVEOUT: i32 = 61;
pub const MM_MSFT_SBPRO_AUX_CD: i32 = 74;
pub const MM_MSFT_SBPRO_AUX_LINE: i32 = 73;
pub const MM_MSFT_SBPRO_MIDIIN: i32 = 70;
pub const MM_MSFT_SBPRO_MIDIOUT: i32 = 71;
pub const MM_MSFT_SBPRO_MIXER: i32 = 75;
pub const MM_MSFT_SBPRO_SYNTH: i32 = 72;
pub const MM_MSFT_SBPRO_WAVEIN: i32 = 68;
pub const MM_MSFT_SBPRO_WAVEOUT: i32 = 69;
pub const MM_MSFT_VMDMS_HANDSET_WAVEIN: i32 = 82;
pub const MM_MSFT_VMDMS_HANDSET_WAVEOUT: i32 = 83;
pub const MM_MSFT_VMDMS_LINE_WAVEIN: i32 = 80;
pub const MM_MSFT_VMDMS_LINE_WAVEOUT: i32 = 81;
pub const MM_MSFT_VMDMW_HANDSET_WAVEIN: i32 = 86;
pub const MM_MSFT_VMDMW_HANDSET_WAVEOUT: i32 = 87;
pub const MM_MSFT_VMDMW_LINE_WAVEIN: i32 = 84;
pub const MM_MSFT_VMDMW_LINE_WAVEOUT: i32 = 85;
pub const MM_MSFT_VMDMW_MIXER: i32 = 88;
pub const MM_MSFT_VMDM_GAME_WAVEIN: i32 = 90;
pub const MM_MSFT_VMDM_GAME_WAVEOUT: i32 = 89;
pub const MM_MSFT_WDMAUDIO_AUX: i32 = 105;
pub const MM_MSFT_WDMAUDIO_MIDIIN: i32 = 103;
pub const MM_MSFT_WDMAUDIO_MIDIOUT: i32 = 102;
pub const MM_MSFT_WDMAUDIO_MIXER: i32 = 104;
pub const MM_MSFT_WDMAUDIO_WAVEIN: i32 = 101;
pub const MM_MSFT_WDMAUDIO_WAVEOUT: i32 = 100;
pub const MM_MSFT_WSS_AUX: i32 = 21;
pub const MM_MSFT_WSS_FMSYNTH_STEREO: i32 = 16;
pub const MM_MSFT_WSS_MIXER: i32 = 17;
pub const MM_MSFT_WSS_NT_AUX: i32 = 59;
pub const MM_MSFT_WSS_NT_FMSYNTH_STEREO: i32 = 57;
pub const MM_MSFT_WSS_NT_MIXER: i32 = 58;
pub const MM_MSFT_WSS_NT_WAVEIN: i32 = 55;
pub const MM_MSFT_WSS_NT_WAVEOUT: i32 = 56;
pub const MM_MSFT_WSS_OEM_AUX: i32 = 22;
pub const MM_MSFT_WSS_OEM_FMSYNTH_STEREO: i32 = 20;
pub const MM_MSFT_WSS_OEM_MIXER: i32 = 31;
pub const MM_MSFT_WSS_OEM_WAVEIN: i32 = 18;
pub const MM_MSFT_WSS_OEM_WAVEOUT: i32 = 19;
pub const MM_MSFT_WSS_WAVEIN: i32 = 14;
pub const MM_MSFT_WSS_WAVEOUT: i32 = 15;
pub const MM_MWM: i32 = 209;
pub const MM_NCR: i32 = 62;
pub const MM_NCR_BA_AUX: i32 = 4;
pub const MM_NCR_BA_MIXER: i32 = 5;
pub const MM_NCR_BA_SYNTH: i32 = 3;
pub const MM_NCR_BA_WAVEIN: i32 = 1;
pub const MM_NCR_BA_WAVEOUT: i32 = 2;
pub const MM_NEC: i32 = 26;
pub const MM_NEC_26_SYNTH: i32 = 9;
pub const MM_NEC_73_86_SYNTH: i32 = 5;
pub const MM_NEC_73_86_WAVEIN: i32 = 7;
pub const MM_NEC_73_86_WAVEOUT: i32 = 6;
pub const MM_NEC_JOYSTICK: i32 = 12;
pub const MM_NEC_MPU401_MIDIIN: i32 = 11;
pub const MM_NEC_MPU401_MIDIOUT: i32 = 10;
pub const MM_NEOMAGIC: i32 = 176;
pub const MM_NEOMAGIC_AUX: i32 = 6;
pub const MM_NEOMAGIC_MIDIIN: i32 = 5;
pub const MM_NEOMAGIC_MIDIOUT: i32 = 4;
pub const MM_NEOMAGIC_MW3DX_AUX: i32 = 17;
pub const MM_NEOMAGIC_MW3DX_FMSYNTH: i32 = 14;
pub const MM_NEOMAGIC_MW3DX_GMSYNTH: i32 = 15;
pub const MM_NEOMAGIC_MW3DX_MIDIIN: i32 = 13;
pub const MM_NEOMAGIC_MW3DX_MIDIOUT: i32 = 12;
pub const MM_NEOMAGIC_MW3DX_MIXER: i32 = 16;
pub const MM_NEOMAGIC_MW3DX_WAVEIN: i32 = 11;
pub const MM_NEOMAGIC_MW3DX_WAVEOUT: i32 = 10;
pub const MM_NEOMAGIC_MWAVE_AUX: i32 = 25;
pub const MM_NEOMAGIC_MWAVE_MIDIIN: i32 = 23;
pub const MM_NEOMAGIC_MWAVE_MIDIOUT: i32 = 22;
pub const MM_NEOMAGIC_MWAVE_MIXER: i32 = 24;
pub const MM_NEOMAGIC_MWAVE_WAVEIN: i32 = 21;
pub const MM_NEOMAGIC_MWAVE_WAVEOUT: i32 = 20;
pub const MM_NEOMAGIC_SYNTH: i32 = 1;
pub const MM_NEOMAGIC_WAVEIN: i32 = 3;
pub const MM_NEOMAGIC_WAVEOUT: i32 = 2;
pub const MM_NETSCAPE: i32 = 166;
pub const MM_NETXL: i32 = 8;
pub const MM_NETXL_XLVIDEO: i32 = 1;
pub const MM_NEWMEDIA: i32 = 86;
pub const MM_NEWMEDIA_WAVJAMMER: i32 = 1;
pub const MM_NMP: i32 = 195;
pub const MM_NMP_ACM_AMR: i32 = 10;
pub const MM_NMP_CCP_WAVEIN: i32 = 1;
pub const MM_NMP_CCP_WAVEOUT: i32 = 2;
pub const MM_NMS: i32 = 87;
pub const MM_NOGATECH: i32 = 75;
pub const MM_NORRIS: i32 = 150;
pub const MM_NORRIS_VOICELINK: i32 = 1;
pub const MM_NORTEL_MPXAC_WAVEIN: i32 = 1;
pub const MM_NORTEL_MPXAC_WAVEOUT: i32 = 2;
pub const MM_NORTHERN_TELECOM: i32 = 115;
pub const MM_NVIDIA: i32 = 127;
pub const MM_NVIDIA_AUX: i32 = 7;
pub const MM_NVIDIA_GAMEPORT: i32 = 5;
pub const MM_NVIDIA_MIDIIN: i32 = 4;
pub const MM_NVIDIA_MIDIOUT: i32 = 3;
pub const MM_NVIDIA_MIXER: i32 = 6;
pub const MM_NVIDIA_WAVEIN: i32 = 2;
pub const MM_NVIDIA_WAVEOUT: i32 = 1;
pub const MM_OKI: i32 = 79;
pub const MM_OKSORI: i32 = 128;
pub const MM_OKSORI_BASE: i32 = 0;
pub const MM_OKSORI_EXT_MIC1: i32 = 15;
pub const MM_OKSORI_EXT_MIC2: i32 = 16;
pub const MM_OKSORI_FM_OPL4: i32 = 5;
pub const MM_OKSORI_MIDIIN: i32 = 18;
pub const MM_OKSORI_MIDIOUT: i32 = 17;
pub const MM_OKSORI_MIX_AUX1: i32 = 13;
pub const MM_OKSORI_MIX_CD: i32 = 10;
pub const MM_OKSORI_MIX_ECHO: i32 = 12;
pub const MM_OKSORI_MIX_FM: i32 = 8;
pub const MM_OKSORI_MIX_LINE: i32 = 9;
pub const MM_OKSORI_MIX_LINE1: i32 = 14;
pub const MM_OKSORI_MIX_MASTER: i32 = 6;
pub const MM_OKSORI_MIX_MIC: i32 = 11;
pub const MM_OKSORI_MIX_WAVE: i32 = 7;
pub const MM_OKSORI_MPEG_CDVISION: i32 = 19;
pub const MM_OKSORI_OSR16_WAVEIN: i32 = 4;
pub const MM_OKSORI_OSR16_WAVEOUT: i32 = 3;
pub const MM_OKSORI_OSR8_WAVEIN: i32 = 2;
pub const MM_OKSORI_OSR8_WAVEOUT: i32 = 1;
pub const MM_OLIVETTI: i32 = 81;
pub const MM_OLIVETTI_ACM_ADPCM: i32 = 10;
pub const MM_OLIVETTI_ACM_CELP: i32 = 11;
pub const MM_OLIVETTI_ACM_GSM: i32 = 9;
pub const MM_OLIVETTI_ACM_OPR: i32 = 13;
pub const MM_OLIVETTI_ACM_SBC: i32 = 12;
pub const MM_OLIVETTI_AUX: i32 = 4;
pub const MM_OLIVETTI_JOYSTICK: i32 = 8;
pub const MM_OLIVETTI_MIDIIN: i32 = 5;
pub const MM_OLIVETTI_MIDIOUT: i32 = 6;
pub const MM_OLIVETTI_MIXER: i32 = 3;
pub const MM_OLIVETTI_SYNTH: i32 = 7;
pub const MM_OLIVETTI_WAVEIN: i32 = 1;
pub const MM_OLIVETTI_WAVEOUT: i32 = 2;
pub const MM_ONLIVE: i32 = 200;
pub const MM_ONLIVE_MPCODEC: i32 = 1;
pub const MM_OPCODE: i32 = 113;
pub const MM_OPTI: i32 = 90;
pub const MM_OPTI_M16_AUX: i32 = 7;
pub const MM_OPTI_M16_FMSYNTH_STEREO: i32 = 1;
pub const MM_OPTI_M16_MIDIIN: i32 = 2;
pub const MM_OPTI_M16_MIDIOUT: i32 = 3;
pub const MM_OPTI_M16_MIXER: i32 = 6;
pub const MM_OPTI_M16_WAVEIN: i32 = 4;
pub const MM_OPTI_M16_WAVEOUT: i32 = 5;
pub const MM_OPTI_M32_AUX: i32 = 38;
pub const MM_OPTI_M32_MIDIIN: i32 = 34;
pub const MM_OPTI_M32_MIDIOUT: i32 = 35;
pub const MM_OPTI_M32_MIXER: i32 = 37;
pub const MM_OPTI_M32_SYNTH_STEREO: i32 = 36;
pub const MM_OPTI_M32_WAVEIN: i32 = 32;
pub const MM_OPTI_M32_WAVEOUT: i32 = 33;
pub const MM_OPTI_P16_AUX: i32 = 22;
pub const MM_OPTI_P16_FMSYNTH_STEREO: i32 = 16;
pub const MM_OPTI_P16_MIDIIN: i32 = 17;
pub const MM_OPTI_P16_MIDIOUT: i32 = 18;
pub const MM_OPTI_P16_MIXER: i32 = 21;
pub const MM_OPTI_P16_WAVEIN: i32 = 19;
pub const MM_OPTI_P16_WAVEOUT: i32 = 20;
pub const MM_OPUS1208_AUX: i32 = 135;
pub const MM_OPUS1208_MIXER: i32 = 134;
pub const MM_OPUS1208_SYNTH: i32 = 131;
pub const MM_OPUS1208_WAVEIN: i32 = 133;
pub const MM_OPUS1208_WAVEOUT: i32 = 132;
pub const MM_OPUS1216_AUX: i32 = 151;
pub const MM_OPUS1216_MIDIIN: i32 = 146;
pub const MM_OPUS1216_MIDIOUT: i32 = 145;
pub const MM_OPUS1216_MIXER: i32 = 150;
pub const MM_OPUS1216_SYNTH: i32 = 147;
pub const MM_OPUS1216_WAVEIN: i32 = 149;
pub const MM_OPUS1216_WAVEOUT: i32 = 148;
pub const MM_OPUS401_MIDIIN: i32 = 130;
pub const MM_OPUS401_MIDIOUT: i32 = 129;
pub const MM_OSITECH: i32 = 103;
pub const MM_OSITECH_TRUMPCARD: i32 = 1;
pub const MM_OSPREY: i32 = 140;
pub const MM_OSPREY_1000WAVEIN: i32 = 1;
pub const MM_OSPREY_1000WAVEOUT: i32 = 2;
pub const MM_OTI: i32 = 180;
pub const MM_OTI_611MIDIN: i32 = 18;
pub const MM_OTI_611MIDIOUT: i32 = 19;
pub const MM_OTI_611MIXER: i32 = 7;
pub const MM_OTI_611WAVEIN: i32 = 5;
pub const MM_OTI_611WAVEOUT: i32 = 6;
pub const MM_PACIFICRESEARCH: i32 = 210;
pub const MM_PCSPEAKER_WAVEOUT: i32 = 13;
pub const MM_PC_JOYSTICK: i32 = 12;
pub const MM_PHILIPS_ACM_LPCBB: i32 = 1;
pub const MM_PHILIPS_SPEECH_PROCESSING: i32 = 7;
pub const MM_PHONET: i32 = 203;
pub const MM_PHONET_PP_MIXER: i32 = 3;
pub const MM_PHONET_PP_WAVEIN: i32 = 2;
pub const MM_PHONET_PP_WAVEOUT: i32 = 1;
pub const MM_PICTURETEL: i32 = 138;
pub const MM_PID_UNMAPPED: i32 = 65535;
pub const MM_PINNACLE: i32 = 218;
pub const MM_PRAGMATRAX: i32 = 5;
pub const MM_PRECEPT: i32 = 153;
pub const MM_PROAUD_16_AUX: i32 = 103;
pub const MM_PROAUD_16_MIDIIN: i32 = 98;
pub const MM_PROAUD_16_MIDIOUT: i32 = 97;
pub const MM_PROAUD_16_MIXER: i32 = 102;
pub const MM_PROAUD_16_SYNTH: i32 = 99;
pub const MM_PROAUD_16_WAVEIN: i32 = 101;
pub const MM_PROAUD_16_WAVEOUT: i32 = 100;
pub const MM_PROAUD_AUX: i32 = 23;
pub const MM_PROAUD_MIDIIN: i32 = 18;
pub const MM_PROAUD_MIDIOUT: i32 = 17;
pub const MM_PROAUD_MIXER: i32 = 22;
pub const MM_PROAUD_PLUS_AUX: i32 = 87;
pub const MM_PROAUD_PLUS_MIDIIN: i32 = 82;
pub const MM_PROAUD_PLUS_MIDIOUT: i32 = 81;
pub const MM_PROAUD_PLUS_MIXER: i32 = 86;
pub const MM_PROAUD_PLUS_SYNTH: i32 = 83;
pub const MM_PROAUD_PLUS_WAVEIN: i32 = 85;
pub const MM_PROAUD_PLUS_WAVEOUT: i32 = 84;
pub const MM_PROAUD_SYNTH: i32 = 19;
pub const MM_PROAUD_WAVEIN: i32 = 21;
pub const MM_PROAUD_WAVEOUT: i32 = 20;
pub const MM_QCIAR: i32 = 98;
pub const MM_QDESIGN: i32 = 194;
pub const MM_QDESIGN_ACM_MPEG: i32 = 1;
pub const MM_QDESIGN_ACM_QDESIGN_MUSIC: i32 = 2;
pub const MM_QTEAM: i32 = 169;
pub const MM_QUALCOMM: i32 = 215;
pub const MM_QUANTUM3D: i32 = 17;
pub const MM_QUARTERDECK: i32 = 134;
pub const MM_QUARTERDECK_LHWAVEIN: i32 = 0;
pub const MM_QUARTERDECK_LHWAVEOUT: i32 = 1;
pub const MM_QUICKAUDIO: i32 = 255;
pub const MM_QUICKAUDIO_MAXIMIDI: i32 = 2;
pub const MM_QUICKAUDIO_MINIMIDI: i32 = 1;
pub const MM_QUICKNET: i32 = 173;
pub const MM_QUICKNET_PJWAVEIN: i32 = 1;
pub const MM_QUICKNET_PJWAVEOUT: i32 = 2;
pub const MM_RADIUS: i32 = 110;
pub const MM_RHETOREX: i32 = 120;
pub const MM_RHETOREX_WAVEIN: i32 = 1;
pub const MM_RHETOREX_WAVEOUT: i32 = 2;
pub const MM_RICHMOND: i32 = 257;
pub const MM_ROCKWELL: i32 = 111;
pub const MM_ROLAND: i32 = 24;
pub const MM_ROLAND_MPU401_MIDIIN: i32 = 16;
pub const MM_ROLAND_MPU401_MIDIOUT: i32 = 15;
pub const MM_ROLAND_RAP10_MIDIIN: i32 = 11;
pub const MM_ROLAND_RAP10_MIDIOUT: i32 = 10;
pub const MM_ROLAND_RAP10_SYNTH: i32 = 12;
pub const MM_ROLAND_RAP10_WAVEIN: i32 = 14;
pub const MM_ROLAND_RAP10_WAVEOUT: i32 = 13;
pub const MM_ROLAND_SC7_MIDIIN: i32 = 22;
pub const MM_ROLAND_SC7_MIDIOUT: i32 = 21;
pub const MM_ROLAND_SCP_AUX: i32 = 48;
pub const MM_ROLAND_SCP_MIDIIN: i32 = 39;
pub const MM_ROLAND_SCP_MIDIOUT: i32 = 38;
pub const MM_ROLAND_SCP_MIXER: i32 = 42;
pub const MM_ROLAND_SCP_WAVEIN: i32 = 41;
pub const MM_ROLAND_SCP_WAVEOUT: i32 = 40;
pub const MM_ROLAND_SERIAL_MIDIIN: i32 = 24;
pub const MM_ROLAND_SERIAL_MIDIOUT: i32 = 23;
pub const MM_ROLAND_SMPU_MIDIINA: i32 = 19;
pub const MM_ROLAND_SMPU_MIDIINB: i32 = 20;
pub const MM_ROLAND_SMPU_MIDIOUTA: i32 = 17;
pub const MM_ROLAND_SMPU_MIDIOUTB: i32 = 18;
pub const MM_RZS: i32 = 216;
pub const MM_RZS_ACM_TUBGSM: i32 = 1;
pub const MM_S3: i32 = 164;
pub const MM_S3_AUX: i32 = 7;
pub const MM_S3_FMSYNTH: i32 = 5;
pub const MM_S3_MIDIIN: i32 = 4;
pub const MM_S3_MIDIOUT: i32 = 3;
pub const MM_S3_MIXER: i32 = 6;
pub const MM_S3_WAVEIN: i32 = 2;
pub const MM_S3_WAVEOUT: i32 = 1;
pub const MM_SANYO: i32 = 72;
pub const MM_SANYO_ACM_LD_ADPCM: i32 = 1;
pub const MM_SCALACS: i32 = 54;
pub const MM_SEERSYS: i32 = 137;
pub const MM_SEERSYS_REALITY: i32 = 6;
pub const MM_SEERSYS_SEERMIX: i32 = 3;
pub const MM_SEERSYS_SEERSYNTH: i32 = 1;
pub const MM_SEERSYS_SEERWAVE: i32 = 2;
pub const MM_SEERSYS_WAVESYNTH: i32 = 4;
pub const MM_SEERSYS_WAVESYNTH_WG: i32 = 5;
pub const MM_SELSIUS_SYSTEMS: i32 = 234;
pub const MM_SELSIUS_SYSTEMS_RTPWAVEIN: i32 = 2;
pub const MM_SELSIUS_SYSTEMS_RTPWAVEOUT: i32 = 1;
pub const MM_SGI: i32 = 237;
pub const MM_SGI_320_MIXER: i32 = 3;
pub const MM_SGI_320_WAVEIN: i32 = 1;
pub const MM_SGI_320_WAVEOUT: i32 = 2;
pub const MM_SGI_540_MIXER: i32 = 6;
pub const MM_SGI_540_WAVEIN: i32 = 4;
pub const MM_SGI_540_WAVEOUT: i32 = 5;
pub const MM_SGI_RAD_ADAT8CHAN_WAVEIN: i32 = 19;
pub const MM_SGI_RAD_ADAT8CHAN_WAVEOUT: i32 = 32;
pub const MM_SGI_RAD_ADATMONO1_WAVEIN: i32 = 7;
pub const MM_SGI_RAD_ADATMONO1_WAVEOUT: i32 = 20;
pub const MM_SGI_RAD_ADATMONO2_WAVEIN: i32 = 8;
pub const MM_SGI_RAD_ADATMONO2_WAVEOUT: i32 = 21;
pub const MM_SGI_RAD_ADATMONO3_WAVEIN: i32 = 9;
pub const MM_SGI_RAD_ADATMONO3_WAVEOUT: i32 = 22;
pub const MM_SGI_RAD_ADATMONO4_WAVEIN: i32 = 10;
pub const MM_SGI_RAD_ADATMONO4_WAVEOUT: i32 = 23;
pub const MM_SGI_RAD_ADATMONO5_WAVEIN: i32 = 11;
pub const MM_SGI_RAD_ADATMONO5_WAVEOUT: i32 = 24;
pub const MM_SGI_RAD_ADATMONO6_WAVEIN: i32 = 12;
pub const MM_SGI_RAD_ADATMONO6_WAVEOUT: i32 = 25;
pub const MM_SGI_RAD_ADATMONO7_WAVEIN: i32 = 13;
pub const MM_SGI_RAD_ADATMONO7_WAVEOUT: i32 = 26;
pub const MM_SGI_RAD_ADATMONO8_WAVEIN: i32 = 14;
pub const MM_SGI_RAD_ADATMONO8_WAVEOUT: i32 = 27;
pub const MM_SGI_RAD_ADATSTEREO12_WAVEIN: i32 = 15;
pub const MM_SGI_RAD_ADATSTEREO12_WAVEOUT: i32 = 28;
pub const MM_SGI_RAD_ADATSTEREO32_WAVEOUT: i32 = 29;
pub const MM_SGI_RAD_ADATSTEREO34_WAVEIN: i32 = 16;
pub const MM_SGI_RAD_ADATSTEREO56_WAVEIN: i32 = 17;
pub const MM_SGI_RAD_ADATSTEREO56_WAVEOUT: i32 = 30;
pub const MM_SGI_RAD_ADATSTEREO78_WAVEIN: i32 = 18;
pub const MM_SGI_RAD_ADATSTEREO78_WAVEOUT: i32 = 31;
pub const MM_SGI_RAD_AESMONO1_WAVEIN: i32 = 33;
pub const MM_SGI_RAD_AESMONO1_WAVEOUT: i32 = 36;
pub const MM_SGI_RAD_AESMONO2_WAVEIN: i32 = 34;
pub const MM_SGI_RAD_AESMONO2_WAVEOUT: i32 = 37;
pub const MM_SGI_RAD_AESSTEREO_WAVEIN: i32 = 35;
pub const MM_SGI_RAD_AESSTEREO_WAVEOUT: i32 = 38;
pub const MM_SHARP: i32 = 183;
pub const MM_SHARP_MDC_AUX: i32 = 6;
pub const MM_SHARP_MDC_AUX_BASS: i32 = 101;
pub const MM_SHARP_MDC_AUX_CHR: i32 = 109;
pub const MM_SHARP_MDC_AUX_MASTER: i32 = 100;
pub const MM_SHARP_MDC_AUX_MIDI_VOL: i32 = 103;
pub const MM_SHARP_MDC_AUX_RVB: i32 = 108;
pub const MM_SHARP_MDC_AUX_TREBLE: i32 = 102;
pub const MM_SHARP_MDC_AUX_VOL: i32 = 107;
pub const MM_SHARP_MDC_AUX_WAVE_CHR: i32 = 106;
pub const MM_SHARP_MDC_AUX_WAVE_RVB: i32 = 105;
pub const MM_SHARP_MDC_AUX_WAVE_VOL: i32 = 104;
pub const MM_SHARP_MDC_MIDI_IN: i32 = 2;
pub const MM_SHARP_MDC_MIDI_OUT: i32 = 3;
pub const MM_SHARP_MDC_MIDI_SYNTH: i32 = 1;
pub const MM_SHARP_MDC_MIXER: i32 = 10;
pub const MM_SHARP_MDC_WAVE_IN: i32 = 4;
pub const MM_SHARP_MDC_WAVE_OUT: i32 = 5;
pub const MM_SICRESOURCE: i32 = 175;
pub const MM_SICRESOURCE_SSO3D: i32 = 2;
pub const MM_SICRESOURCE_SSOW3DI: i32 = 3;
pub const MM_SIEMENS_SBC: i32 = 201;
pub const MM_SIERRA: i32 = 40;
pub const MM_SIERRA_ARIA_AUX: i32 = 25;
pub const MM_SIERRA_ARIA_AUX2: i32 = 32;
pub const MM_SIERRA_ARIA_MIDIIN: i32 = 21;
pub const MM_SIERRA_ARIA_MIDIOUT: i32 = 20;
pub const MM_SIERRA_ARIA_SYNTH: i32 = 22;
pub const MM_SIERRA_ARIA_WAVEIN: i32 = 24;
pub const MM_SIERRA_ARIA_WAVEOUT: i32 = 23;
pub const MM_SIERRA_QUARTET_AUX_CD: i32 = 85;
pub const MM_SIERRA_QUARTET_AUX_LINE: i32 = 86;
pub const MM_SIERRA_QUARTET_AUX_MODEM: i32 = 87;
pub const MM_SIERRA_QUARTET_MIDIIN: i32 = 82;
pub const MM_SIERRA_QUARTET_MIDIOUT: i32 = 83;
pub const MM_SIERRA_QUARTET_MIXER: i32 = 88;
pub const MM_SIERRA_QUARTET_SYNTH: i32 = 84;
pub const MM_SIERRA_QUARTET_WAVEIN: i32 = 80;
pub const MM_SIERRA_QUARTET_WAVEOUT: i32 = 81;
pub const MM_SILICONSOFT: i32 = 69;
pub const MM_SILICONSOFT_SC1_WAVEIN: i32 = 1;
pub const MM_SILICONSOFT_SC1_WAVEOUT: i32 = 2;
pub const MM_SILICONSOFT_SC2_WAVEIN: i32 = 3;
pub const MM_SILICONSOFT_SC2_WAVEOUT: i32 = 4;
pub const MM_SILICONSOFT_SOUNDJR2PR_WAVEIN: i32 = 6;
pub const MM_SILICONSOFT_SOUNDJR2PR_WAVEOUT: i32 = 7;
pub const MM_SILICONSOFT_SOUNDJR2_WAVEOUT: i32 = 5;
pub const MM_SILICONSOFT_SOUNDJR3_WAVEOUT: i32 = 8;
pub const MM_SIPROLAB: i32 = 211;
pub const MM_SIPROLAB_ACELPNET: i32 = 1;
pub const MM_SNDBLST_MIDIIN: i32 = 4;
pub const MM_SNDBLST_MIDIOUT: i32 = 3;
pub const MM_SNDBLST_SYNTH: i32 = 5;
pub const MM_SNDBLST_WAVEIN: i32 = 7;
pub const MM_SNDBLST_WAVEOUT: i32 = 6;
pub const MM_SNI: i32 = 18;
pub const MM_SNI_ACM_G721: i32 = 1;
pub const MM_SOFTLAB_NSK: i32 = 228;
pub const MM_SOFTLAB_NSK_FRW_AUX: i32 = 4;
pub const MM_SOFTLAB_NSK_FRW_MIXER: i32 = 3;
pub const MM_SOFTLAB_NSK_FRW_WAVEIN: i32 = 1;
pub const MM_SOFTLAB_NSK_FRW_WAVEOUT: i32 = 2;
pub const MM_SOFTSOUND: i32 = 149;
pub const MM_SOFTSOUND_CODEC: i32 = 1;
pub const MM_SONICFOUNDRY: i32 = 66;
pub const MM_SONORUS: i32 = 230;
pub const MM_SONORUS_STUDIO: i32 = 1;
pub const MM_SONY: i32 = 245;
pub const MM_SONY_ACM_SCX: i32 = 1;
pub const MM_SORVIS: i32 = 187;
pub const MM_SOUNDESIGNS: i32 = 142;
pub const MM_SOUNDESIGNS_WAVEIN: i32 = 1;
pub const MM_SOUNDESIGNS_WAVEOUT: i32 = 2;
pub const MM_SOUNDSCAPE_AUX: i32 = 24;
pub const MM_SOUNDSCAPE_MIDIIN: i32 = 21;
pub const MM_SOUNDSCAPE_MIDIOUT: i32 = 20;
pub const MM_SOUNDSCAPE_MIXER: i32 = 23;
pub const MM_SOUNDSCAPE_SYNTH: i32 = 22;
pub const MM_SOUNDSCAPE_WAVEIN: i32 = 19;
pub const MM_SOUNDSCAPE_WAVEOUT: i32 = 17;
pub const MM_SOUNDSCAPE_WAVEOUT_AUX: i32 = 18;
pub const MM_SOUNDSPACE: i32 = 167;
pub const MM_SPECTRUM_PRODUCTIONS: i32 = 213;
pub const MM_SPECTRUM_SIGNAL_PROCESSING: i32 = 144;
pub const MM_SPEECHCOMP: i32 = 76;
pub const MM_SPLASH_STUDIOS: i32 = 133;
pub const MM_SSP_SNDFESAUX: i32 = 7;
pub const MM_SSP_SNDFESMIDIIN: i32 = 3;
pub const MM_SSP_SNDFESMIDIOUT: i32 = 4;
pub const MM_SSP_SNDFESMIX: i32 = 6;
pub const MM_SSP_SNDFESSYNTH: i32 = 5;
pub const MM_SSP_SNDFESWAVEIN: i32 = 1;
pub const MM_SSP_SNDFESWAVEOUT: i32 = 2;
pub const MM_STUDER: i32 = 171;
pub const MM_STUDIO_16_AUX: i32 = 103;
pub const MM_STUDIO_16_MIDIIN: i32 = 98;
pub const MM_STUDIO_16_MIDIOUT: i32 = 97;
pub const MM_STUDIO_16_MIXER: i32 = 102;
pub const MM_STUDIO_16_SYNTH: i32 = 99;
pub const MM_STUDIO_16_WAVEIN: i32 = 101;
pub const MM_STUDIO_16_WAVEOUT: i32 = 100;
pub const MM_ST_MICROELECTRONICS: i32 = 265;
pub const MM_SUNCOM: i32 = 186;
pub const MM_SUPERMAC: i32 = 73;
pub const MM_SYDEC_NV: i32 = 248;
pub const MM_SYDEC_NV_WAVEIN: i32 = 1;
pub const MM_SYDEC_NV_WAVEOUT: i32 = 2;
pub const MM_TANDY: i32 = 29;
pub const MM_TANDY_PSSJWAVEIN: i32 = 9;
pub const MM_TANDY_PSSJWAVEOUT: i32 = 10;
pub const MM_TANDY_SENS_MMAMIDIIN: i32 = 6;
pub const MM_TANDY_SENS_MMAMIDIOUT: i32 = 7;
pub const MM_TANDY_SENS_MMAWAVEIN: i32 = 4;
pub const MM_TANDY_SENS_MMAWAVEOUT: i32 = 5;
pub const MM_TANDY_SENS_VISWAVEOUT: i32 = 8;
pub const MM_TANDY_VISBIOSSYNTH: i32 = 3;
pub const MM_TANDY_VISWAVEIN: i32 = 1;
pub const MM_TANDY_VISWAVEOUT: i32 = 2;
pub const MM_TBS_TROPEZ_AUX1: i32 = 39;
pub const MM_TBS_TROPEZ_AUX2: i32 = 40;
pub const MM_TBS_TROPEZ_LINE: i32 = 41;
pub const MM_TBS_TROPEZ_WAVEIN: i32 = 37;
pub const MM_TBS_TROPEZ_WAVEOUT: i32 = 38;
pub const MM_TDK: i32 = 135;
pub const MM_TDK_MW_AUX: i32 = 6;
pub const MM_TDK_MW_AUX_BASS: i32 = 101;
pub const MM_TDK_MW_AUX_CHR: i32 = 109;
pub const MM_TDK_MW_AUX_MASTER: i32 = 100;
pub const MM_TDK_MW_AUX_MIDI_VOL: i32 = 103;
pub const MM_TDK_MW_AUX_RVB: i32 = 108;
pub const MM_TDK_MW_AUX_TREBLE: i32 = 102;
pub const MM_TDK_MW_AUX_VOL: i32 = 107;
pub const MM_TDK_MW_AUX_WAVE_CHR: i32 = 106;
pub const MM_TDK_MW_AUX_WAVE_RVB: i32 = 105;
pub const MM_TDK_MW_AUX_WAVE_VOL: i32 = 104;
pub const MM_TDK_MW_MIDI_IN: i32 = 2;
pub const MM_TDK_MW_MIDI_OUT: i32 = 3;
pub const MM_TDK_MW_MIDI_SYNTH: i32 = 1;
pub const MM_TDK_MW_MIXER: i32 = 10;
pub const MM_TDK_MW_WAVE_IN: i32 = 4;
pub const MM_TDK_MW_WAVE_OUT: i32 = 5;
pub const MM_TELEKOL: i32 = 264;
pub const MM_TELEKOL_WAVEIN: i32 = 2;
pub const MM_TELEKOL_WAVEOUT: i32 = 1;
pub const MM_TERALOGIC: i32 = 202;
pub const MM_TERRATEC: i32 = 70;
pub const MM_THUNDER_AUX: i32 = 39;
pub const MM_THUNDER_SYNTH: i32 = 35;
pub const MM_THUNDER_WAVEIN: i32 = 37;
pub const MM_THUNDER_WAVEOUT: i32 = 36;
pub const MM_TPORT_SYNTH: i32 = 67;
pub const MM_TPORT_WAVEIN: i32 = 66;
pub const MM_TPORT_WAVEOUT: i32 = 65;
pub const MM_TRUEVISION: i32 = 51;
pub const MM_TRUEVISION_WAVEIN1: i32 = 1;
pub const MM_TRUEVISION_WAVEOUT1: i32 = 2;
pub const MM_TTEWS_AUX: i32 = 9;
pub const MM_TTEWS_MIDIIN: i32 = 3;
pub const MM_TTEWS_MIDIMONITOR: i32 = 6;
pub const MM_TTEWS_MIDIOUT: i32 = 4;
pub const MM_TTEWS_MIDISYNTH: i32 = 5;
pub const MM_TTEWS_MIXER: i32 = 10;
pub const MM_TTEWS_VMIDIIN: i32 = 7;
pub const MM_TTEWS_VMIDIOUT: i32 = 8;
pub const MM_TTEWS_WAVEIN: i32 = 1;
pub const MM_TTEWS_WAVEOUT: i32 = 2;
pub const MM_TURTLE_BEACH: i32 = 21;
pub const MM_UHER_INFORMATIC: i32 = 247;
pub const MM_UH_ACM_ADPCM: i32 = 1;
pub const MM_UNISYS: i32 = 223;
pub const MM_UNISYS_ACM_NAP: i32 = 1;
pub const MM_UNMAPPED: i32 = 65535;
pub const MM_VAL: i32 = 35;
pub const MM_VAL_MICROKEY_AP_WAVEIN: i32 = 1;
pub const MM_VAL_MICROKEY_AP_WAVEOUT: i32 = 2;
pub const MM_VANKOEVERING: i32 = 168;
pub const MM_VIA: i32 = 250;
pub const MM_VIA_AUX: i32 = 4;
pub const MM_VIA_MIXER: i32 = 3;
pub const MM_VIA_MPU401_MIDIIN: i32 = 6;
pub const MM_VIA_MPU401_MIDIOUT: i32 = 5;
pub const MM_VIA_SWFM_SYNTH: i32 = 7;
pub const MM_VIA_WAVEIN: i32 = 2;
pub const MM_VIA_WAVEOUT: i32 = 1;
pub const MM_VIA_WDM_MIXER: i32 = 10;
pub const MM_VIA_WDM_MPU401_MIDIIN: i32 = 12;
pub const MM_VIA_WDM_MPU401_MIDIOUT: i32 = 11;
pub const MM_VIA_WDM_WAVEIN: i32 = 9;
pub const MM_VIA_WDM_WAVEOUT: i32 = 8;
pub const MM_VIDEOLOGIC: i32 = 53;
pub const MM_VIDEOLOGIC_MSWAVEIN: i32 = 1;
pub const MM_VIDEOLOGIC_MSWAVEOUT: i32 = 2;
pub const MM_VIENNASYS: i32 = 157;
pub const MM_VIENNASYS_TSP_WAVE_DRIVER: i32 = 1;
pub const MM_VIONA: i32 = 161;
pub const MM_VIONAQVINPCI_WAVEOUT: i32 = 3;
pub const MM_VIONA_BUSTER_MIXER: i32 = 4;
pub const MM_VIONA_CINEMASTER_MIXER: i32 = 5;
pub const MM_VIONA_CONCERTO_MIXER: i32 = 6;
pub const MM_VIONA_QVINPCI_MIXER: i32 = 1;
pub const MM_VIONA_QVINPCI_WAVEIN: i32 = 2;
pub const MM_VIRTUALMUSIC: i32 = 205;
pub const MM_VITEC: i32 = 67;
pub const MM_VITEC_VMAKER: i32 = 1;
pub const MM_VITEC_VMPRO: i32 = 2;
pub const MM_VIVO: i32 = 182;
pub const MM_VIVO_AUDIO_CODEC: i32 = 1;
pub const MM_VKC_MPU401_MIDIIN: i32 = 256;
pub const MM_VKC_MPU401_MIDIOUT: i32 = 512;
pub const MM_VKC_SERIAL_MIDIIN: i32 = 257;
pub const MM_VKC_SERIAL_MIDIOUT: i32 = 513;
pub const MM_VOCALTEC: i32 = 23;
pub const MM_VOCALTEC_WAVEIN: i32 = 2;
pub const MM_VOCALTEC_WAVEOUT: i32 = 1;
pub const MM_VOICEINFO: i32 = 156;
pub const MM_VOICEMIXER: i32 = 1;
pub const MM_VOXWARE: i32 = 114;
pub const MM_VOXWARE_CODEC: i32 = 1;
pub const MM_VOYETRA: i32 = 30;
pub const MM_VQST: i32 = 240;
pub const MM_VQST_VQC1: i32 = 1;
pub const MM_VQST_VQC2: i32 = 2;
pub const MM_VTG: i32 = 109;
pub const MM_WANGLABS: i32 = 28;
pub const MM_WANGLABS_WAVEIN1: i32 = 1;
pub const MM_WANGLABS_WAVEOUT1: i32 = 2;
pub const MM_WAVE_MAPPER: i32 = 2;
pub const MM_WEITEK: i32 = 96;
pub const MM_WILDCAT: i32 = 119;
pub const MM_WILDCAT_AUTOSCOREMIDIIN: i32 = 1;
pub const MM_WILLOPOND_SNDCOMM_WAVEIN: i32 = 108;
pub const MM_WILLOWPOND: i32 = 65;
pub const MM_WILLOWPOND_FMSYNTH_STEREO: i32 = 20;
pub const MM_WILLOWPOND_GENERIC_AUX: i32 = 115;
pub const MM_WILLOWPOND_GENERIC_MIXER: i32 = 114;
pub const MM_WILLOWPOND_GENERIC_WAVEIN: i32 = 112;
pub const MM_WILLOWPOND_GENERIC_WAVEOUT: i32 = 113;
pub const MM_WILLOWPOND_MPU401: i32 = 21;
pub const MM_WILLOWPOND_PH_AUX: i32 = 107;
pub const MM_WILLOWPOND_PH_MIXER: i32 = 106;
pub const MM_WILLOWPOND_PH_WAVEIN: i32 = 104;
pub const MM_WILLOWPOND_PH_WAVEOUT: i32 = 105;
pub const MM_WILLOWPOND_SNDCOMM_AUX: i32 = 111;
pub const MM_WILLOWPOND_SNDCOMM_MIXER: i32 = 110;
pub const MM_WILLOWPOND_SNDCOMM_WAVEOUT: i32 = 109;
pub const MM_WILLOWPOND_SNDPORT_AUX: i32 = 103;
pub const MM_WILLOWPOND_SNDPORT_MIXER: i32 = 102;
pub const MM_WILLOWPOND_SNDPORT_WAVEIN: i32 = 100;
pub const MM_WILLOWPOND_SNDPORT_WAVEOUT: i32 = 101;
pub const MM_WINBOND: i32 = 204;
pub const MM_WINNOV: i32 = 61;
pub const MM_WINNOV_CAVIAR_CHAMPAGNE: i32 = 4;
pub const MM_WINNOV_CAVIAR_VIDC: i32 = 3;
pub const MM_WINNOV_CAVIAR_WAVEIN: i32 = 1;
pub const MM_WINNOV_CAVIAR_WAVEOUT: i32 = 2;
pub const MM_WINNOV_CAVIAR_YUV8: i32 = 5;
pub const MM_WORKBIT: i32 = 102;
pub const MM_WORKBIT_AUX: i32 = 7;
pub const MM_WORKBIT_FMSYNTH: i32 = 6;
pub const MM_WORKBIT_JOYSTICK: i32 = 8;
pub const MM_WORKBIT_MIDIIN: i32 = 4;
pub const MM_WORKBIT_MIDIOUT: i32 = 5;
pub const MM_WORKBIT_MIXER: i32 = 1;
pub const MM_WORKBIT_WAVEIN: i32 = 3;
pub const MM_WORKBIT_WAVEOUT: i32 = 2;
pub const MM_WSS_SB16_AUX_CD: i32 = 45;
pub const MM_WSS_SB16_AUX_LINE: i32 = 44;
pub const MM_WSS_SB16_MIDIIN: i32 = 41;
pub const MM_WSS_SB16_MIDIOUT: i32 = 42;
pub const MM_WSS_SB16_MIXER: i32 = 46;
pub const MM_WSS_SB16_SYNTH: i32 = 43;
pub const MM_WSS_SB16_WAVEIN: i32 = 39;
pub const MM_WSS_SB16_WAVEOUT: i32 = 40;
pub const MM_WSS_SBPRO_AUX_CD: i32 = 53;
pub const MM_WSS_SBPRO_AUX_LINE: i32 = 52;
pub const MM_WSS_SBPRO_MIDIIN: i32 = 49;
pub const MM_WSS_SBPRO_MIDIOUT: i32 = 50;
pub const MM_WSS_SBPRO_MIXER: i32 = 54;
pub const MM_WSS_SBPRO_SYNTH: i32 = 51;
pub const MM_WSS_SBPRO_WAVEIN: i32 = 47;
pub const MM_WSS_SBPRO_WAVEOUT: i32 = 48;
pub const MM_XEBEC: i32 = 85;
pub const MM_XIRLINK: i32 = 178;
pub const MM_XIRLINK_VISIONLINK: i32 = 1;
pub const MM_XYZ: i32 = 112;
pub const MM_YAMAHA: i32 = 37;
pub const MM_YAMAHA_ACXG_AUX: i32 = 41;
pub const MM_YAMAHA_ACXG_MIDIOUT: i32 = 39;
pub const MM_YAMAHA_ACXG_MIXER: i32 = 40;
pub const MM_YAMAHA_ACXG_WAVEIN: i32 = 37;
pub const MM_YAMAHA_ACXG_WAVEOUT: i32 = 38;
pub const MM_YAMAHA_GSS_AUX: i32 = 6;
pub const MM_YAMAHA_GSS_MIDIIN: i32 = 5;
pub const MM_YAMAHA_GSS_MIDIOUT: i32 = 4;
pub const MM_YAMAHA_GSS_SYNTH: i32 = 1;
pub const MM_YAMAHA_GSS_WAVEIN: i32 = 3;
pub const MM_YAMAHA_GSS_WAVEOUT: i32 = 2;
pub const MM_YAMAHA_OPL3SA_FMSYNTH: i32 = 18;
pub const MM_YAMAHA_OPL3SA_JOYSTICK: i32 = 24;
pub const MM_YAMAHA_OPL3SA_MIDIIN: i32 = 21;
pub const MM_YAMAHA_OPL3SA_MIDIOUT: i32 = 20;
pub const MM_YAMAHA_OPL3SA_MIXER: i32 = 23;
pub const MM_YAMAHA_OPL3SA_WAVEIN: i32 = 17;
pub const MM_YAMAHA_OPL3SA_WAVEOUT: i32 = 16;
pub const MM_YAMAHA_OPL3SA_YSYNTH: i32 = 19;
pub const MM_YAMAHA_SERIAL_MIDIIN: i32 = 8;
pub const MM_YAMAHA_SERIAL_MIDIOUT: i32 = 7;
pub const MM_YAMAHA_SXG_MIDIOUT: i32 = 34;
pub const MM_YAMAHA_SXG_MIXER: i32 = 36;
pub const MM_YAMAHA_SXG_WAVEOUT: i32 = 35;
pub const MM_YAMAHA_YMF724LEG_FMSYNTH: i32 = 32;
pub const MM_YAMAHA_YMF724LEG_MIDIIN: i32 = 26;
pub const MM_YAMAHA_YMF724LEG_MIDIOUT: i32 = 25;
pub const MM_YAMAHA_YMF724LEG_MIXER: i32 = 33;
pub const MM_YAMAHA_YMF724_AUX: i32 = 30;
pub const MM_YAMAHA_YMF724_MIDIOUT: i32 = 29;
pub const MM_YAMAHA_YMF724_MIXER: i32 = 31;
pub const MM_YAMAHA_YMF724_WAVEIN: i32 = 28;
pub const MM_YAMAHA_YMF724_WAVEOUT: i32 = 27;
pub const MM_YOUCOM: i32 = 256;
pub const MM_ZEFIRO: i32 = 170;
pub const MM_ZEFIRO_ZA2: i32 = 2;
pub const MM_ZYXEL: i32 = 9;
pub const MM_ZYXEL_ACM_ADPCM: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct MPEG1WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
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
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct MPEGLAYER3WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wID: u16,
    pub fdwFlags: u32,
    pub nBlockSize: u16,
    pub nFramesPerBlock: u16,
    pub nCodecDelay: u16,
}
pub const MPEGLAYER3_FLAG_PADDING_ISO: i32 = 0;
pub const MPEGLAYER3_FLAG_PADDING_OFF: i32 = 2;
pub const MPEGLAYER3_FLAG_PADDING_ON: i32 = 1;
pub const MPEGLAYER3_ID_CONSTANTFRAMESIZE: i32 = 2;
pub const MPEGLAYER3_ID_MPEG: i32 = 1;
pub const MPEGLAYER3_ID_UNKNOWN: i32 = 0;
pub const MPEGLAYER3_WFX_EXTRA_BYTES: i32 = 12;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct MSAUDIO1WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
    pub wEncodeOptions: u16,
}
pub const MSAUDIO1_BITS_PER_SAMPLE: i32 = 16;
pub const MSAUDIO1_MAX_CHANNELS: i32 = 2;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct NMS_VBXADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
pub type NPADPCMCOEFSET = *mut ADPCMCOEFSET;
#[cfg(feature = "mmeapi")]
pub type NPADPCMEWAVEFORMAT = *mut ADPCMEWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPADPCMWAVEFORMAT = *mut ADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPAPTXWAVEFORMAT = *mut APTXWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPAUDIOFILE_AF10WAVEFORMAT = *mut AUDIOFILE_AF10WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPAUDIOFILE_AF36WAVEFORMAT = *mut AUDIOFILE_AF36WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPCONTRESCR10WAVEFORMAT = *mut CONTRESCR10WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPCONTRESVQLPCWAVEFORMAT = *mut CONTRESVQLPCWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPCREATIVEADPCMWAVEFORMAT = *mut CREATIVEADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPCREATIVEFASTSPEECH10WAVEFORMAT = *mut CREATIVEFASTSPEECH10WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPCREATIVEFASTSPEECH8WAVEFORMAT = *mut CREATIVEFASTSPEECH8WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPCSIMAADPCMWAVEFORMAT = *mut CSIMAADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPDIALOGICOKIADPCMWAVEFORMAT = *mut DIALOGICOKIADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPDIGIADPCMWAVEFORMAT = *mut DIGIADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPDIGIFIXWAVEFORMAT = *mut DIGIFIXWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPDIGIREALWAVEFORMAT = *mut DIGIREALWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPDIGISTDWAVEFORMAT = *mut DIGISTDWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPDRMWAVEFORMAT = *mut DRMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPDVIADPCMWAVEFORMAT = *mut DVIADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPECHOSC1WAVEFORMAT = *mut ECHOSC1WAVEFORMAT;
pub type NPECHOWAVEFILTER = *mut ECHOWAVEFILTER;
#[cfg(feature = "mmeapi")]
pub type NPFMTOWNS_SND_WAVEFORMAT = *mut FMTOWNS_SND_WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPG721_ADPCMWAVEFORMAT = *mut G721_ADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPG723_ADPCMWAVEFORMAT = *mut G723_ADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPGSM610WAVEFORMAT = *mut GSM610WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPHEAACWAVEFORMAT = *mut HEAACWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPHEAACWAVEINFO = *mut HEAACWAVEINFO;
#[cfg(feature = "mmeapi")]
pub type NPIMAADPCMWAVEFORMAT = *mut IMAADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPMEDIASPACEADPCMWAVEFORMAT = *mut MEDIASPACEADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPMPEG1WAVEFORMAT = *mut MPEG1WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPMPEGLAYER3WAVEFORMAT = *mut MPEGLAYER3WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPNMS_VBXADPCMWAVEFORMAT = *mut NMS_VBXADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPOLIADPCMWAVEFORMAT = *mut OLIADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPOLICELPWAVEFORMAT = *mut OLICELPWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPOLIGSMWAVEFORMAT = *mut OLIGSMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPOLIOPRWAVEFORMAT = *mut OLIOPRWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPOLISBCWAVEFORMAT = *mut OLISBCWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPSIERRAADPCMWAVEFORMAT = *mut SIERRAADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPSONARCWAVEFORMAT = *mut SONARCWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type NPTRUESPEECHWAVEFORMAT = *mut TRUESPEECHWAVEFORMAT;
pub type NPVOLUMEWAVEFILTER = *mut VOLUMEWAVEFILTER;
pub type NPWAVEFILTER = *mut WAVEFILTER;
#[cfg(feature = "mmeapi")]
pub type NPWAVEFORMATIEEEFLOATEX = *mut WAVEFORMATIEEEFLOATEX;
#[cfg(feature = "mmeapi")]
pub type NPWAVEFORMATPCMEX = *mut WAVEFORMATPCMEX;
#[cfg(feature = "mmeapi")]
pub type NPYAMAHA_ADPCMWAVEFORMAT = *mut YAMAHA_ADPCMWAVEFORMAT;
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct OLIADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct OLICELPWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct OLIGSMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct OLIOPRWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct OLISBCWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
pub type PADPCMCOEFSET = *mut ADPCMCOEFSET;
#[cfg(feature = "mmeapi")]
pub type PADPCMEWAVEFORMAT = *mut ADPCMEWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PADPCMWAVEFORMAT = *mut ADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PAPTXWAVEFORMAT = *mut APTXWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PAUDIOFILE_AF10WAVEFORMAT = *mut AUDIOFILE_AF10WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PAUDIOFILE_AF36WAVEFORMAT = *mut AUDIOFILE_AF36WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PCONTRESCR10WAVEFORMAT = *mut CONTRESCR10WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PCONTRESVQLPCWAVEFORMAT = *mut CONTRESVQLPCWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PCREATIVEADPCMWAVEFORMAT = *mut CREATIVEADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PCREATIVEFASTSPEECH10WAVEFORMAT = *mut CREATIVEFASTSPEECH10WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PCREATIVEFASTSPEECH8WAVEFORMAT = *mut CREATIVEFASTSPEECH8WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PCSIMAADPCMWAVEFORMAT = *mut CSIMAADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PDIALOGICOKIADPCMWAVEFORMAT = *mut DIALOGICOKIADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PDIGIADPCMWAVEFORMAT = *mut DIGIADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PDIGIFIXWAVEFORMAT = *mut DIGIFIXWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PDIGIREALWAVEFORMAT = *mut DIGIREALWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PDIGISTDWAVEFORMAT = *mut DIGISTDWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PDRMWAVEFORMAT = *mut DRMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PDVIADPCMWAVEFORMAT = *mut DVIADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PECHOSC1WAVEFORMAT = *mut ECHOSC1WAVEFORMAT;
pub type PECHOWAVEFILTER = *mut ECHOWAVEFILTER;
#[cfg(feature = "mmeapi")]
pub type PFMTOWNS_SND_WAVEFORMAT = *mut FMTOWNS_SND_WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PG721_ADPCMWAVEFORMAT = *mut G721_ADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PG723_ADPCMWAVEFORMAT = *mut G723_ADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PGSM610WAVEFORMAT = *mut GSM610WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PHEAACWAVEFORMAT = *mut HEAACWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PHEAACWAVEINFO = *mut HEAACWAVEINFO;
#[cfg(feature = "mmeapi")]
pub type PIMAADPCMWAVEFORMAT = *mut IMAADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PMEDIASPACEADPCMWAVEFORMAT = *mut MEDIASPACEADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PMPEG1WAVEFORMAT = *mut MPEG1WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PMPEGLAYER3WAVEFORMAT = *mut MPEGLAYER3WAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PNMS_VBXADPCMWAVEFORMAT = *mut NMS_VBXADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type POLIADPCMWAVEFORMAT = *mut OLIADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type POLICELPWAVEFORMAT = *mut OLICELPWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type POLIGSMWAVEFORMAT = *mut OLIGSMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type POLIOPRWAVEFORMAT = *mut OLIOPRWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type POLISBCWAVEFORMAT = *mut OLISBCWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PSIERRAADPCMWAVEFORMAT = *mut SIERRAADPCMWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PSONARCWAVEFORMAT = *mut SONARCWAVEFORMAT;
#[cfg(feature = "mmeapi")]
pub type PTRUESPEECHWAVEFORMAT = *mut TRUESPEECHWAVEFORMAT;
pub type PVOLUMEWAVEFILTER = *mut VOLUMEWAVEFILTER;
pub type PWAVEFILTER = *mut WAVEFILTER;
#[cfg(feature = "mmeapi")]
pub type PWAVEFORMATEXTENSIBLE = *mut WAVEFORMATEXTENSIBLE;
#[cfg(feature = "mmeapi")]
pub type PWAVEFORMATIEEEFLOATEX = *mut WAVEFORMATIEEEFLOATEX;
#[cfg(feature = "mmeapi")]
pub type PWAVEFORMATPCMEX = *mut WAVEFORMATPCMEX;
#[cfg(feature = "mmeapi")]
pub type PYAMAHA_ADPCMWAVEFORMAT = *mut YAMAHA_ADPCMWAVEFORMAT;
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
pub const ROCKWELL_WA1_MIXER: i32 = 103;
pub const ROCKWELL_WA1_MPU401_IN: i32 = 104;
pub const ROCKWELL_WA1_MPU401_OUT: i32 = 105;
pub const ROCKWELL_WA1_SYNTH: i32 = 102;
pub const ROCKWELL_WA1_WAVEIN: i32 = 100;
pub const ROCKWELL_WA1_WAVEOUT: i32 = 101;
pub const ROCKWELL_WA2_MIXER: i32 = 203;
pub const ROCKWELL_WA2_MPU401_IN: i32 = 204;
pub const ROCKWELL_WA2_MPU401_OUT: i32 = 205;
pub const ROCKWELL_WA2_SYNTH: i32 = 202;
pub const ROCKWELL_WA2_WAVEIN: i32 = 200;
pub const ROCKWELL_WA2_WAVEOUT: i32 = 201;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct SIERRAADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wRevision: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct SONARCWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wCompType: u16,
}
pub const SPEAKER_ALL: u32 = 2147483648;
pub const SPEAKER_BACK_CENTER: i32 = 256;
pub const SPEAKER_BACK_LEFT: i32 = 16;
pub const SPEAKER_BACK_RIGHT: i32 = 32;
pub const SPEAKER_FRONT_CENTER: i32 = 4;
pub const SPEAKER_FRONT_LEFT: i32 = 1;
pub const SPEAKER_FRONT_LEFT_OF_CENTER: i32 = 64;
pub const SPEAKER_FRONT_RIGHT: i32 = 2;
pub const SPEAKER_FRONT_RIGHT_OF_CENTER: i32 = 128;
pub const SPEAKER_LOW_FREQUENCY: i32 = 8;
pub const SPEAKER_RESERVED: i32 = 2147221504;
pub const SPEAKER_SIDE_LEFT: i32 = 512;
pub const SPEAKER_SIDE_RIGHT: i32 = 1024;
pub const SPEAKER_TOP_BACK_CENTER: i32 = 65536;
pub const SPEAKER_TOP_BACK_LEFT: i32 = 32768;
pub const SPEAKER_TOP_BACK_RIGHT: i32 = 131072;
pub const SPEAKER_TOP_CENTER: i32 = 2048;
pub const SPEAKER_TOP_FRONT_CENTER: i32 = 8192;
pub const SPEAKER_TOP_FRONT_LEFT: i32 = 4096;
pub const SPEAKER_TOP_FRONT_RIGHT: i32 = 16384;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub struct TRUESPEECHWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wRevision: u16,
    pub nSamplesPerBlock: u16,
    pub abReserved: [u8; 28],
}
#[cfg(feature = "mmeapi")]
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
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: super::WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: windows_core::GUID,
}
#[cfg(feature = "mmeapi")]
impl Default for WAVEFORMATEXTENSIBLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub union WAVEFORMATEXTENSIBLE_0 {
    pub wValidBitsPerSample: u16,
    pub wSamplesPerBlock: u16,
    pub wReserved: u16,
}
#[cfg(feature = "mmeapi")]
impl Default for WAVEFORMATEXTENSIBLE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmeapi")]
pub type WAVEFORMATIEEEFLOATEX = WAVEFORMATEXTENSIBLE;
#[cfg(feature = "mmeapi")]
pub type WAVEFORMATPCMEX = WAVEFORMATEXTENSIBLE;
pub const WAVE_FILTER_DEVELOPMENT: i32 = 65535;
pub const WAVE_FILTER_ECHO: i32 = 2;
pub const WAVE_FILTER_UNKNOWN: i32 = 0;
pub const WAVE_FILTER_VOLUME: i32 = 1;
pub const WAVE_FORMAT_3COM_NBX: i32 = 28672;
pub const WAVE_FORMAT_ADPCM: i32 = 2;
pub const WAVE_FORMAT_ALAC: i32 = 27745;
pub const WAVE_FORMAT_ALAW: i32 = 6;
pub const WAVE_FORMAT_AMR_NB: i32 = 29537;
pub const WAVE_FORMAT_AMR_WB: i32 = 29538;
pub const WAVE_FORMAT_AMR_WP: i32 = 29539;
pub const WAVE_FORMAT_ANTEX_ADPCME: i32 = 51;
pub const WAVE_FORMAT_APTX: i32 = 37;
pub const WAVE_FORMAT_AUDIOFILE_AF10: i32 = 38;
pub const WAVE_FORMAT_AUDIOFILE_AF36: i32 = 36;
pub const WAVE_FORMAT_BTV_DIGITAL: i32 = 1024;
pub const WAVE_FORMAT_CANOPUS_ATRAC: i32 = 99;
pub const WAVE_FORMAT_CIRRUS: i32 = 96;
pub const WAVE_FORMAT_CODIAN: i32 = 41252;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_AVQSBC: i32 = 41217;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_G723_1: i32 = 41216;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_SBC: i32 = 41218;
pub const WAVE_FORMAT_CONGRUENCY: i32 = 141;
pub const WAVE_FORMAT_CONTROL_RES_CR10: i32 = 55;
pub const WAVE_FORMAT_CONTROL_RES_VQLPC: i32 = 52;
pub const WAVE_FORMAT_CONVEDIA_G729: i32 = 140;
pub const WAVE_FORMAT_CREATIVE_ADPCM: i32 = 512;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH10: i32 = 515;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH8: i32 = 514;
pub const WAVE_FORMAT_CS2: i32 = 608;
pub const WAVE_FORMAT_CS_IMAADPCM: i32 = 57;
pub const WAVE_FORMAT_CUSEEME: i32 = 7939;
pub const WAVE_FORMAT_CU_CODEC: i32 = 25;
pub const WAVE_FORMAT_DEVELOPMENT: i32 = 65535;
pub const WAVE_FORMAT_DF_G726: i32 = 133;
pub const WAVE_FORMAT_DF_GSM610: i32 = 134;
pub const WAVE_FORMAT_DIALOGIC_OKI_ADPCM: i32 = 23;
pub const WAVE_FORMAT_DICTAPHONE_CELP54: i32 = 322;
pub const WAVE_FORMAT_DICTAPHONE_CELP68: i32 = 321;
pub const WAVE_FORMAT_DIGIADPCM: i32 = 54;
pub const WAVE_FORMAT_DIGIFIX: i32 = 22;
pub const WAVE_FORMAT_DIGIREAL: i32 = 53;
pub const WAVE_FORMAT_DIGISTD: i32 = 21;
pub const WAVE_FORMAT_DIGITAL_G723: i32 = 291;
pub const WAVE_FORMAT_DIVIO_G726: i32 = 16963;
pub const WAVE_FORMAT_DIVIO_MPEG4_AAC: i32 = 16707;
pub const WAVE_FORMAT_DOLBY_AC2: i32 = 48;
pub const WAVE_FORMAT_DOLBY_AC3_SPDIF: i32 = 146;
pub const WAVE_FORMAT_DOLBY_AC4: i32 = 44096;
pub const WAVE_FORMAT_DRM: i32 = 9;
pub const WAVE_FORMAT_DSAT: i32 = 102;
pub const WAVE_FORMAT_DSAT_DISPLAY: i32 = 103;
pub const WAVE_FORMAT_DSPGROUP_TRUESPEECH: i32 = 34;
pub const WAVE_FORMAT_DTS: i32 = 8;
pub const WAVE_FORMAT_DTS2: i32 = 8193;
pub const WAVE_FORMAT_DTS_DS: i32 = 400;
pub const WAVE_FORMAT_DVI_ADPCM: i32 = 17;
pub const WAVE_FORMAT_DVM: i32 = 8192;
pub const WAVE_FORMAT_ECHOSC1: i32 = 35;
pub const WAVE_FORMAT_ECHOSC3: i32 = 58;
pub const WAVE_FORMAT_ENCORE_G726: i32 = 41223;
pub const WAVE_FORMAT_ESPCM: i32 = 97;
pub const WAVE_FORMAT_ESST_AC3: i32 = 577;
pub const WAVE_FORMAT_EXTENSIBLE: i32 = 65534;
pub const WAVE_FORMAT_FAAD_AAC: i32 = 28781;
pub const WAVE_FORMAT_FLAC: i32 = 61868;
pub const WAVE_FORMAT_FM_TOWNS_SND: i32 = 768;
pub const WAVE_FORMAT_FRACE_TELECOM_G729: i32 = 41251;
pub const WAVE_FORMAT_FRAUNHOFER_IIS_MPEG2_AAC: i32 = 384;
pub const WAVE_FORMAT_G721_ADPCM: i32 = 64;
pub const WAVE_FORMAT_G722_ADPCM: i32 = 101;
pub const WAVE_FORMAT_G723_ADPCM: i32 = 20;
pub const WAVE_FORMAT_G726ADPCM: i32 = 320;
pub const WAVE_FORMAT_G726_ADPCM: i32 = 100;
pub const WAVE_FORMAT_G728_CELP: i32 = 65;
pub const WAVE_FORMAT_G729A: i32 = 131;
pub const WAVE_FORMAT_GENERIC_PASSTHRU: i32 = 585;
pub const WAVE_FORMAT_GLOBAL_IP_ILBC: i32 = 41238;
pub const WAVE_FORMAT_GSM610: i32 = 49;
pub const WAVE_FORMAT_GSM_610: i32 = 41229;
pub const WAVE_FORMAT_GSM_620: i32 = 41230;
pub const WAVE_FORMAT_GSM_660: i32 = 41231;
pub const WAVE_FORMAT_GSM_690: i32 = 41232;
pub const WAVE_FORMAT_GSM_ADAPTIVE_MULTIRATE_WB: i32 = 41233;
pub const WAVE_FORMAT_GSM_AMR_CBR: i32 = 31265;
pub const WAVE_FORMAT_GSM_AMR_VBR_SID: i32 = 31266;
pub const WAVE_FORMAT_HP_DYN_VOICE: i32 = 26;
pub const WAVE_FORMAT_IBM_CVSD: i32 = 5;
pub const WAVE_FORMAT_IEEE_FLOAT: i32 = 3;
pub const WAVE_FORMAT_ILINK_VC: i32 = 560;
pub const WAVE_FORMAT_IMA_ADPCM: i32 = 17;
pub const WAVE_FORMAT_INDEO_AUDIO: i32 = 1026;
pub const WAVE_FORMAT_INFOCOM_ITS_G721_ADPCM: i32 = 139;
pub const WAVE_FORMAT_INGENIENT_G726: i32 = 41221;
pub const WAVE_FORMAT_INNINGS_TELECOM_ADPCM: i32 = 6521;
pub const WAVE_FORMAT_INTEL_G723_1: i32 = 67;
pub const WAVE_FORMAT_INTEL_G729: i32 = 68;
pub const WAVE_FORMAT_INTEL_MUSIC_CODER: i32 = 1025;
pub const WAVE_FORMAT_IPI_HSX: i32 = 592;
pub const WAVE_FORMAT_IPI_RPELP: i32 = 593;
pub const WAVE_FORMAT_IRAT: i32 = 257;
pub const WAVE_FORMAT_ISIAUDIO: i32 = 136;
pub const WAVE_FORMAT_ISIAUDIO_2: i32 = 5121;
pub const WAVE_FORMAT_KNOWLEDGE_ADVENTURE_ADPCM: i32 = 376;
pub const WAVE_FORMAT_LEAD_SPEECH: i32 = 17228;
pub const WAVE_FORMAT_LEAD_VORBIS: i32 = 22092;
pub const WAVE_FORMAT_LH_CODEC: i32 = 4352;
pub const WAVE_FORMAT_LH_CODEC_CELP: i32 = 4353;
pub const WAVE_FORMAT_LH_CODEC_SBC12: i32 = 4355;
pub const WAVE_FORMAT_LH_CODEC_SBC16: i32 = 4356;
pub const WAVE_FORMAT_LH_CODEC_SBC8: i32 = 4354;
pub const WAVE_FORMAT_LIGHTWAVE_LOSSLESS: i32 = 2222;
pub const WAVE_FORMAT_LRC: i32 = 40;
pub const WAVE_FORMAT_LUCENT_G723: i32 = 89;
pub const WAVE_FORMAT_LUCENT_SX5363S: i32 = 7180;
pub const WAVE_FORMAT_LUCENT_SX8300P: i32 = 7175;
pub const WAVE_FORMAT_MAKEAVIS: i32 = 13075;
pub const WAVE_FORMAT_MALDEN_PHONYTALK: i32 = 160;
pub const WAVE_FORMAT_MEDIASONIC_G723: i32 = 147;
pub const WAVE_FORMAT_MEDIASPACE_ADPCM: i32 = 18;
pub const WAVE_FORMAT_MEDIAVISION_ADPCM: i32 = 24;
pub const WAVE_FORMAT_MICRONAS: i32 = 848;
pub const WAVE_FORMAT_MICRONAS_CELP833: i32 = 849;
pub const WAVE_FORMAT_MPEG: i32 = 80;
pub const WAVE_FORMAT_MPEG4_AAC: i32 = 41222;
pub const WAVE_FORMAT_MPEGLAYER3: i32 = 85;
pub const WAVE_FORMAT_MPEG_ADTS_AAC: i32 = 5632;
pub const WAVE_FORMAT_MPEG_HEAAC: i32 = 5648;
pub const WAVE_FORMAT_MPEG_LOAS: i32 = 5634;
pub const WAVE_FORMAT_MPEG_RAW_AAC: i32 = 5633;
pub const WAVE_FORMAT_MSAUDIO1: i32 = 352;
pub const WAVE_FORMAT_MSG723: i32 = 66;
pub const WAVE_FORMAT_MSNAUDIO: i32 = 50;
pub const WAVE_FORMAT_MSRT24: i32 = 130;
pub const WAVE_FORMAT_MULAW: i32 = 7;
pub const WAVE_FORMAT_MULTITUDE_FT_SX20: i32 = 138;
pub const WAVE_FORMAT_MVI_MVI2: i32 = 132;
pub const WAVE_FORMAT_NEC_AAC: i32 = 176;
pub const WAVE_FORMAT_NICE_ACA: i32 = 41240;
pub const WAVE_FORMAT_NICE_ADPCM: i32 = 41241;
pub const WAVE_FORMAT_NICE_G728: i32 = 41250;
pub const WAVE_FORMAT_NMS_VBXADPCM: i32 = 56;
pub const WAVE_FORMAT_NOKIA_ADAPTIVE_MULTIRATE: i32 = 16897;
pub const WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC: i32 = 5640;
pub const WAVE_FORMAT_NOKIA_MPEG_RAW_AAC: i32 = 5641;
pub const WAVE_FORMAT_NORCOM_VOICE_SYSTEMS_ADPCM: i32 = 645;
pub const WAVE_FORMAT_NORRIS: i32 = 5120;
pub const WAVE_FORMAT_NTCSOFT_ALF2CM_ACM: i32 = 8132;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1: i32 = 26447;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1_PLUS: i32 = 26479;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2: i32 = 26448;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2_PLUS: i32 = 26480;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3: i32 = 26449;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3_PLUS: i32 = 26481;
pub const WAVE_FORMAT_OKI_ADPCM: i32 = 16;
pub const WAVE_FORMAT_OLIADPCM: i32 = 4097;
pub const WAVE_FORMAT_OLICELP: i32 = 4098;
pub const WAVE_FORMAT_OLIGSM: i32 = 4096;
pub const WAVE_FORMAT_OLIOPR: i32 = 4100;
pub const WAVE_FORMAT_OLISBC: i32 = 4099;
pub const WAVE_FORMAT_ON2_VP6_AUDIO: i32 = 1281;
pub const WAVE_FORMAT_ON2_VP7_AUDIO: i32 = 1280;
pub const WAVE_FORMAT_ONLIVE: i32 = 137;
pub const WAVE_FORMAT_OPUS: i32 = 28751;
pub const WAVE_FORMAT_PAC: i32 = 83;
pub const WAVE_FORMAT_PACKED: i32 = 153;
pub const WAVE_FORMAT_PHILIPS_CELP: i32 = 288;
pub const WAVE_FORMAT_PHILIPS_GRUNDIG: i32 = 289;
pub const WAVE_FORMAT_PHILIPS_LPCBB: i32 = 152;
pub const WAVE_FORMAT_POLYCOM_G722: i32 = 41234;
pub const WAVE_FORMAT_POLYCOM_G728: i32 = 41235;
pub const WAVE_FORMAT_POLYCOM_G729_A: i32 = 41236;
pub const WAVE_FORMAT_POLYCOM_SIREN: i32 = 41237;
pub const WAVE_FORMAT_PROSODY_1612: i32 = 39;
pub const WAVE_FORMAT_PROSODY_8KBPS: i32 = 148;
pub const WAVE_FORMAT_QDESIGN_MUSIC: i32 = 1104;
pub const WAVE_FORMAT_QUALCOMM_HALFRATE: i32 = 337;
pub const WAVE_FORMAT_QUALCOMM_PUREVOICE: i32 = 336;
pub const WAVE_FORMAT_QUARTERDECK: i32 = 544;
pub const WAVE_FORMAT_RACAL_RECORDER_G720_A: i32 = 162;
pub const WAVE_FORMAT_RACAL_RECORDER_G723_1: i32 = 163;
pub const WAVE_FORMAT_RACAL_RECORDER_GSM: i32 = 161;
pub const WAVE_FORMAT_RACAL_RECORDER_TETRA_ACELP: i32 = 164;
pub const WAVE_FORMAT_RADIOTIME_TIME_SHIFT_RADIO: i32 = 41239;
pub const WAVE_FORMAT_RAW_AAC1: i32 = 255;
pub const WAVE_FORMAT_RAW_SPORT: i32 = 576;
pub const WAVE_FORMAT_RHETOREX_ADPCM: i32 = 256;
pub const WAVE_FORMAT_ROCKWELL_ADPCM: i32 = 59;
pub const WAVE_FORMAT_ROCKWELL_DIGITALK: i32 = 60;
pub const WAVE_FORMAT_RT24: i32 = 82;
pub const WAVE_FORMAT_SANYO_LD_ADPCM: i32 = 293;
pub const WAVE_FORMAT_SBC24: i32 = 145;
pub const WAVE_FORMAT_SHARP_G726: i32 = 69;
pub const WAVE_FORMAT_SIERRA_ADPCM: i32 = 19;
pub const WAVE_FORMAT_SIPROLAB_ACELP4800: i32 = 305;
pub const WAVE_FORMAT_SIPROLAB_ACELP8V3: i32 = 306;
pub const WAVE_FORMAT_SIPROLAB_ACEPLNET: i32 = 304;
pub const WAVE_FORMAT_SIPROLAB_G729: i32 = 307;
pub const WAVE_FORMAT_SIPROLAB_G729A: i32 = 308;
pub const WAVE_FORMAT_SIPROLAB_KELVIN: i32 = 309;
pub const WAVE_FORMAT_SOFTSOUND: i32 = 128;
pub const WAVE_FORMAT_SONARC: i32 = 33;
pub const WAVE_FORMAT_SONICFOUNDRY_LOSSLESS: i32 = 6513;
pub const WAVE_FORMAT_SONY_ATRAC3: i32 = 626;
pub const WAVE_FORMAT_SONY_SCX: i32 = 624;
pub const WAVE_FORMAT_SONY_SCY: i32 = 625;
pub const WAVE_FORMAT_SONY_SPC: i32 = 627;
pub const WAVE_FORMAT_SOUNDSPACE_MUSICOMPRESS: i32 = 5376;
pub const WAVE_FORMAT_SPEEX_VOICE: i32 = 41225;
pub const WAVE_FORMAT_SYCOM_ACM_SYC008: i32 = 372;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP54: i32 = 374;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP68: i32 = 375;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_G726L: i32 = 373;
pub const WAVE_FORMAT_SYMBOL_G729_A: i32 = 41219;
pub const WAVE_FORMAT_TELUM_AUDIO: i32 = 640;
pub const WAVE_FORMAT_TELUM_IA_AUDIO: i32 = 641;
pub const WAVE_FORMAT_TPC: i32 = 1665;
pub const WAVE_FORMAT_TUBGSM: i32 = 341;
pub const WAVE_FORMAT_UHER_ADPCM: i32 = 528;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO: i32 = 533;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO_1: i32 = 534;
pub const WAVE_FORMAT_UNISYS_NAP_16K: i32 = 371;
pub const WAVE_FORMAT_UNISYS_NAP_ADPCM: i32 = 368;
pub const WAVE_FORMAT_UNISYS_NAP_ALAW: i32 = 370;
pub const WAVE_FORMAT_UNISYS_NAP_ULAW: i32 = 369;
pub const WAVE_FORMAT_UNKNOWN: i32 = 0;
pub const WAVE_FORMAT_VIANIX_MASC: i32 = 41226;
pub const WAVE_FORMAT_VIVO_G723: i32 = 273;
pub const WAVE_FORMAT_VIVO_SIREN: i32 = 274;
pub const WAVE_FORMAT_VME_VMPCM: i32 = 1664;
pub const WAVE_FORMAT_VOCORD_G721: i32 = 41242;
pub const WAVE_FORMAT_VOCORD_G722_1: i32 = 41244;
pub const WAVE_FORMAT_VOCORD_G723_1: i32 = 41248;
pub const WAVE_FORMAT_VOCORD_G726: i32 = 41243;
pub const WAVE_FORMAT_VOCORD_G728: i32 = 41245;
pub const WAVE_FORMAT_VOCORD_G729: i32 = 41246;
pub const WAVE_FORMAT_VOCORD_G729_A: i32 = 41247;
pub const WAVE_FORMAT_VOCORD_LBC: i32 = 41249;
pub const WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC: i32 = 5642;
pub const WAVE_FORMAT_VODAFONE_MPEG_RAW_AAC: i32 = 5643;
pub const WAVE_FORMAT_VOICEAGE_AMR: i32 = 310;
pub const WAVE_FORMAT_VOICEAGE_AMR_WB: i32 = 41220;
pub const WAVE_FORMAT_VOXWARE: i32 = 98;
pub const WAVE_FORMAT_VOXWARE_AC10: i32 = 113;
pub const WAVE_FORMAT_VOXWARE_AC16: i32 = 114;
pub const WAVE_FORMAT_VOXWARE_AC20: i32 = 115;
pub const WAVE_FORMAT_VOXWARE_AC8: i32 = 112;
pub const WAVE_FORMAT_VOXWARE_BYTE_ALIGNED: i32 = 105;
pub const WAVE_FORMAT_VOXWARE_RT24: i32 = 116;
pub const WAVE_FORMAT_VOXWARE_RT24_SPEECH: i32 = 6172;
pub const WAVE_FORMAT_VOXWARE_RT29: i32 = 117;
pub const WAVE_FORMAT_VOXWARE_RT29HW: i32 = 118;
pub const WAVE_FORMAT_VOXWARE_SC3: i32 = 122;
pub const WAVE_FORMAT_VOXWARE_SC3_1: i32 = 123;
pub const WAVE_FORMAT_VOXWARE_TQ40: i32 = 121;
pub const WAVE_FORMAT_VOXWARE_TQ60: i32 = 129;
pub const WAVE_FORMAT_VOXWARE_VR12: i32 = 119;
pub const WAVE_FORMAT_VOXWARE_VR18: i32 = 120;
pub const WAVE_FORMAT_VSELP: i32 = 4;
pub const WAVE_FORMAT_WAVPACK_AUDIO: i32 = 22358;
pub const WAVE_FORMAT_WM9_SPECTRUM_ANALYZER: i32 = 41227;
pub const WAVE_FORMAT_WMASPDIF: i32 = 356;
pub const WAVE_FORMAT_WMAUDIO2: i32 = 353;
pub const WAVE_FORMAT_WMAUDIO3: i32 = 354;
pub const WAVE_FORMAT_WMAUDIO_LOSSLESS: i32 = 355;
pub const WAVE_FORMAT_WMAVOICE10: i32 = 11;
pub const WAVE_FORMAT_WMAVOICE9: i32 = 10;
pub const WAVE_FORMAT_WMF_SPECTRUM_ANAYZER: i32 = 41228;
pub const WAVE_FORMAT_XEBEC: i32 = 61;
pub const WAVE_FORMAT_YAMAHA_ADPCM: i32 = 32;
pub const WAVE_FORMAT_ZOLL_ASAO: i32 = 41224;
pub const WAVE_FORMAT_ZYXEL_ADPCM: i32 = 151;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct WMAUDIO2WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub dwSamplesPerBlock: u32,
    pub wEncodeOptions: u16,
    pub dwSuperBlockAlign: u32,
}
pub const WMAUDIO2_BITS_PER_SAMPLE: i32 = 16;
pub const WMAUDIO2_MAX_CHANNELS: i32 = 2;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct WMAUDIO3WAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
    pub wValidBitsPerSample: u16,
    pub dwChannelMask: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub wEncodeOptions: u16,
    pub wReserved3: u16,
}
pub const WMAUDIO_BITS_PER_SAMPLE: i32 = 16;
pub const WMAUDIO_MAX_CHANNELS: i32 = 2;
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct YAMAHA_ADPCMWAVEFORMAT {
    pub wfx: super::WAVEFORMATEX,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct s_RIFFWAVE_inst(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct tag_s_RIFFWAVE_inst {
    pub bUnshiftedNote: u8,
    pub chFineTune: i8,
    pub chGain: i8,
    pub bLowNote: u8,
    pub bHighNote: u8,
    pub bLowVelocity: u8,
    pub bHighVelocity: u8,
}

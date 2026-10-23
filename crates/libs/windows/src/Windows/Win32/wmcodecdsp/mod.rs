pub const AACMFTEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x93af0c51_2275_45d2_a35b_f2ba21caed00);
pub const ACCESSMODE_READ: FILE_ACCESSMODE = 1;
pub const ACCESSMODE_READWRITE: FILE_ACCESSMODE = 3;
pub const ACCESSMODE_WRITE: FILE_ACCESSMODE = 2;
pub const ACCESSMODE_WRITE_EXCLUSIVE: FILE_ACCESSMODE = 4;
pub const ADAPTIVE_ARRAY_AND_AEC: AEC_SYSTEM_MODE = 3;
pub const ADAPTIVE_ARRAY_ONLY: AEC_SYSTEM_MODE = 1;
pub const AEC_CAPTURE_STREAM: AEC_INPUT_STREAM = 0;
pub type AEC_INPUT_STREAM = i32;
pub const AEC_MAX_SYSTEM_MODES: i32 = 6;
pub const AEC_REFERENCE_STREAM: AEC_INPUT_STREAM = 1;
pub type AEC_SYSTEM_MODE = i32;
pub const AEC_VAD_DISABLED: AEC_VAD_MODE = 0;
pub const AEC_VAD_FOR_AGC: AEC_VAD_MODE = 2;
pub const AEC_VAD_FOR_SILENCE_SUPPRESSION: AEC_VAD_MODE = 3;
pub type AEC_VAD_MODE = i32;
pub const AEC_VAD_NORMAL: AEC_VAD_MODE = 1;
pub const ALawCodecWrapper: windows_core::GUID = windows_core::GUID::from_u128(0x36cb6e0c_78c1_42b2_9943_846262f31786);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AecQualityMetrics_Struct {
    pub i64Timestamp: super::LONGLONG,
    pub ConvergenceFlag: super::BYTE,
    pub MicClippedFlag: super::BYTE,
    pub MicSilenceFlag: super::BYTE,
    pub PstvFeadbackFlag: super::BYTE,
    pub SpkClippedFlag: super::BYTE,
    pub SpkMuteFlag: super::BYTE,
    pub GlitchFlag: super::BYTE,
    pub DoubleTalkFlag: super::BYTE,
    pub uGlitchCount: super::ULONG,
    pub uMicClipCount: super::ULONG,
    pub fDuration: f32,
    pub fTSVariance: f32,
    pub fTSDriftRate: f32,
    pub fVoiceLevel: f32,
    pub fNoiseLevel: f32,
    pub fERLE: f32,
    pub fAvgERLE: f32,
    pub dwReserved: super::DWORD,
}
pub const CAC3DecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x03d7c802_ecfa_47d9_b268_5fb3e310dee4);
pub const CClusterDetectorDmo: windows_core::GUID = windows_core::GUID::from_u128(0x36e820c4_165a_4521_863c_619e1160d4d4);
pub const CColorControlDmo: windows_core::GUID = windows_core::GUID::from_u128(0x798059f0_89ca_4160_b325_aeb48efe4f9a);
pub const CColorConvertDMO: windows_core::GUID = windows_core::GUID::from_u128(0x98230571_0087_4204_b020_3282538e57d3);
pub const CColorLegalizerDmo: windows_core::GUID = windows_core::GUID::from_u128(0xfdfaa753_e48e_4e33_9c74_98a27fc6726a);
pub const CDTVAudDecoderDS: windows_core::GUID = windows_core::GUID::from_u128(0x8e269032_fe03_4753_9b17_18253c21722e);
pub const CDTVVidDecoderDS: windows_core::GUID = windows_core::GUID::from_u128(0x64777dc8_4e24_4beb_9d19_60a35be1daaf);
pub const CDVDecoderMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xe54709c5_1e17_4c8d_94e7_478940433584);
pub const CDVEncoderMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xc82ae729_c327_4cce_914d_8171fefebefb);
pub const CDeColorConvMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x49034c05_f43c_400f_84c1_90a683195a3a);
pub const CFrameInterpDMO: windows_core::GUID = windows_core::GUID::from_u128(0x0a7cfe1b_6ab5_4334_9ed8_3f97cb37daa1);
pub const CFrameRateConvertDmo: windows_core::GUID = windows_core::GUID::from_u128(0x01f36ce2_0907_4d8b_979d_f151be91c883);
pub const CInterlaceMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xb5a89c80_4901_407b_9abc_90d9a644bb46);
pub const CLSID_CAsfTocParser: windows_core::GUID = windows_core::GUID::from_u128(0x9b77c0f2_8735_46c5_b90f_5f0b303ef6ab);
pub const CLSID_CAviTocParser: windows_core::GUID = windows_core::GUID::from_u128(0x3adce5cc_13c8_4573_b328_ed438eb694f9);
pub const CLSID_CClusterDetectorEx: windows_core::GUID = windows_core::GUID::from_u128(0x47354492_827e_4b8a_b318_c80eba1381f0);
pub const CLSID_CFileClient: windows_core::GUID = windows_core::GUID::from_u128(0xbfccd195_1244_4840_ab44_480975c4ffe4);
pub const CLSID_CFileIo: windows_core::GUID = windows_core::GUID::from_u128(0x11993195_1244_4840_ab44_480975c4ffe4);
pub const CLSID_CToc: windows_core::GUID = windows_core::GUID::from_u128(0x4fe24495_28ce_4920_a4c4_e556e1f0df2a);
pub const CLSID_CTocCollection: windows_core::GUID = windows_core::GUID::from_u128(0x5058292d_a244_4840_ab44_480975c4ffe4);
pub const CLSID_CTocEntry: windows_core::GUID = windows_core::GUID::from_u128(0xf22f5e05_585c_4def_8523_6555cfbc0cb3);
pub const CLSID_CTocEntryList: windows_core::GUID = windows_core::GUID::from_u128(0x3a8cccbc_0efd_43a3_b838_f38a552ba237);
pub const CLSID_CTocParser: windows_core::GUID = windows_core::GUID::from_u128(0x499eaeea_2737_4849_8bb6_47f107eaf358);
pub const CMP3DecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xbbeea841_0a63_4f52_a7ab_a9b3a84ed38a);
pub const CMPEG2AudDecoderDS: windows_core::GUID = windows_core::GUID::from_u128(0xe1f1a0b8_beee_490d_ba7c_066c40b5e2b9);
pub const CMPEG2AudioEncoderMFT: windows_core::GUID = windows_core::GUID::from_u128(0x46a4dd5c_73f8_4304_94df_308f760974f4);
pub const CMPEG2EncoderAudioDS: windows_core::GUID = windows_core::GUID::from_u128(0xacd453bc_c58a_44d1_bbf5_bfb325be2d78);
pub const CMPEG2EncoderDS: windows_core::GUID = windows_core::GUID::from_u128(0x5f5aff4a_2f7f_4279_88c2_cd88eb39d144);
pub const CMPEG2EncoderVideoDS: windows_core::GUID = windows_core::GUID::from_u128(0x42150cd9_ca9a_4ea5_9939_30ee037f6e74);
pub const CMPEG2VidDecoderDS: windows_core::GUID = windows_core::GUID::from_u128(0x212690fb_83e5_4526_8fd7_74478b7939cd);
pub const CMPEG2VideoEncoderMFT: windows_core::GUID = windows_core::GUID::from_u128(0xe6335f02_80b7_4dc4_adfa_dfe7210d20d5);
pub const CMPEGAACDecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x8dde1772_edad_41c3_b4be_1f30fb4ee0d6);
pub const CMSAACDecMFT: windows_core::GUID = windows_core::GUID::from_u128(0x32d186a7_218f_4c75_8876_dd77273a8999);
pub const CMSAC3Enc: windows_core::GUID = windows_core::GUID::from_u128(0xc6b400e2_20a7_4e58_a2fe_24619682ce6c);
pub const CMSALACDecMFT: windows_core::GUID = windows_core::GUID::from_u128(0xc0cd7d12_31fc_4bbc_b363_7322ee3e1879);
pub const CMSALACEncMFT: windows_core::GUID = windows_core::GUID::from_u128(0x9ab6a28c_748e_4b6a_bfff_cc443b8e8fb4);
pub const CMSDDPlusDecMFT: windows_core::GUID = windows_core::GUID::from_u128(0x177c0afe_900b_48d4_9e4c_57add250b3d4);
pub const CMSDolbyDigitalEncMFT: windows_core::GUID = windows_core::GUID::from_u128(0xac3315c9_f481_45d7_826c_0b406c1f64b8);
pub const CMSFLACDecMFT: windows_core::GUID = windows_core::GUID::from_u128(0x6b0b3e6b_a2c5_4514_8055_afe8a95242d9);
pub const CMSFLACEncMFT: windows_core::GUID = windows_core::GUID::from_u128(0x128509e9_c44e_45dc_95e9_c255b8f466a6);
pub const CMSH263EncoderMFT: windows_core::GUID = windows_core::GUID::from_u128(0xbc47fcfe_98a0_4f27_bb07_698af24f2b38);
pub const CMSH264DecoderMFT: windows_core::GUID = windows_core::GUID::from_u128(0x62ce7e72_4c71_4d20_b15d_452831a87d9d);
pub const CMSH264EncoderMFT: windows_core::GUID = windows_core::GUID::from_u128(0x6ca50344_051a_4ded_9779_a43305165e35);
pub const CMSH264RemuxMFT: windows_core::GUID = windows_core::GUID::from_u128(0x05a47ebb_8bf0_4cbf_ad2f_3b71d75866f5);
pub const CMSH265EncoderMFT: windows_core::GUID = windows_core::GUID::from_u128(0xf2f84074_8bca_40bd_9159_e880f673dd3b);
pub const CMSMPEGAudDecMFT: windows_core::GUID = windows_core::GUID::from_u128(0x70707b39_b2ca_4015_abea_f8447d22d88b);
pub const CMSMPEGDecoderMFT: windows_core::GUID = windows_core::GUID::from_u128(0x2d709e52_123f_49b5_9cbc_9af5cde28fb9);
pub const CMSOpusDecMFT: windows_core::GUID = windows_core::GUID::from_u128(0x63e17c10_2d43_4c42_8fe3_8d8b63e46a6a);
pub const CMSSCDecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x7bafb3b1_d8f4_4279_9253_27da423108de);
pub const CMSSCEncMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x8cb9cc06_d139_4ae6_8bb4_41e612e141d5);
pub const CMSSCEncMediaObject2: windows_core::GUID = windows_core::GUID::from_u128(0xf7ffe0a0_a4f5_44b5_949e_15ed2bc66f9d);
pub const CMSVPXEncoderMFT: windows_core::GUID = windows_core::GUID::from_u128(0xaeb6c755_2546_4881_82cc_e15ae5ebff3d);
pub const CMSVideoDSPMFT: windows_core::GUID = windows_core::GUID::from_u128(0x51571744_7fe4_4ff2_a498_2dc34ff74f1b);
pub const CMpeg2DecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x863d66cd_cdce_4617_b47f_c8929cfc28a6);
pub const CMpeg43DecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xcba9e78b_49a3_49ea_93d4_6bcba8c4de07);
pub const CMpeg4DecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xf371728a_6052_4d47_827c_d039335dfe0a);
pub const CMpeg4EncMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x24f258d8_c651_4042_93e4_ca654abb682c);
pub const CMpeg4sDecMFT: windows_core::GUID = windows_core::GUID::from_u128(0x5686a0d9_fe39_409f_9dff_3fdbc849f9f5);
pub const CMpeg4sDecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x2a11bae2_fe6e_4249_864b_9e9ed6e8dbc2);
pub const CMpeg4sEncMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x6ec5a7be_d81e_4f9e_ada3_cd1bf262b6d8);
pub const CNokiaAACCCDecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xeabf7a6f_ccba_4d60_8620_b152cc977263);
pub const CNokiaAACDecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x3cb2bde4_4e29_4c44_a73e_2d7c2c46d6ec);
pub const CPK_DS_AC3Decoder: windows_core::GUID = windows_core::GUID::from_u128(0x6c9c69d6_0ffc_4481_afdb_cdf1c79c6f3e);
pub const CPK_DS_MPEG2Decoder: windows_core::GUID = windows_core::GUID::from_u128(0x9910c5cd_95c9_4e06_865a_efa1c8016bf4);
pub const CResamplerMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xf447b69e_1884_4a7e_8055_346f74d6edb3);
pub const CResizerDMO: windows_core::GUID = windows_core::GUID::from_u128(0x1ea1ea14_48f4_4054_ad1a_e8aee10ac805);
pub const CResizerMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xd3ec8b8b_7728_4fd8_9fe0_7b67d19f73a3);
pub const CShotDetectorDmo: windows_core::GUID = windows_core::GUID::from_u128(0x56aefacd_110c_4397_9292_b0a0c61b6750);
pub const CSmpteTransformsDmo: windows_core::GUID = windows_core::GUID::from_u128(0xbde6388b_da25_485d_ba7f_fabc28b20318);
pub const CThumbnailGeneratorDmo: windows_core::GUID = windows_core::GUID::from_u128(0x559c6bad_1ea8_4963_a087_8a6810f9218b);
pub const CTocGeneratorDmo: windows_core::GUID = windows_core::GUID::from_u128(0x4dda1941_77a0_4fb1_a518_e2185041d70c);
pub const CVodafoneAACCCDecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x7e76bf7f_c993_4e26_8fab_470a70c0d59c);
pub const CVodafoneAACDecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x7f36f942_dcf3_4d82_9289_5b1820278f7c);
pub const CWMADecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x2eeb4adf_4578_4d10_bca7_bb955f56320a);
pub const CWMAEncMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x70f598e9_f4ab_495a_99e2_a7c4d3d89abf);
pub const CWMATransMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xedcad9cb_3127_40df_b527_0152ccb3f6f5);
pub const CWMAudioAEC: windows_core::GUID = windows_core::GUID::from_u128(0x745057c7_f353_4f2d_a7ee_58434477730e);
pub const CWMAudioCAPXGFXAPO: windows_core::GUID = windows_core::GUID::from_u128(0x13ab3ebd_137e_4903_9d89_60be8277fd17);
pub const CWMAudioCAPXLFXAPO: windows_core::GUID = windows_core::GUID::from_u128(0xc9453e73_8c5c_4463_9984_af8bab2f5447);
pub const CWMAudioGFXAPO: windows_core::GUID = windows_core::GUID::from_u128(0x637c490d_eee3_4c0a_973f_371958802da2);
pub const CWMAudioLFXAPO: windows_core::GUID = windows_core::GUID::from_u128(0x62dc1a93_ae24_464c_a43e_452f824c4250);
pub const CWMAudioSpdTxDMO: windows_core::GUID = windows_core::GUID::from_u128(0x5210f8e4_b0bb_47c3_a8d9_7b2282cc79ed);
pub const CWMSPDecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x874131cb_4ecc_443b_8948_746b89595d20);
pub const CWMSPEncMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x67841b03_c689_4188_ad3f_4c9ebeec710b);
pub const CWMSPEncMediaObject2: windows_core::GUID = windows_core::GUID::from_u128(0x1f1f4e1a_2252_4063_84bb_eee75f8856d5);
pub const CWMTDecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xf9dbc64e_2dd0_45dd_9b52_66642ef94431);
pub const CWMTEncMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x60b67652_e46b_4e44_8609_f74bffdc083c);
pub const CWMV9EncMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xd23b90d0_144f_46bd_841d_59e4eb19dc59);
pub const CWMVDecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x82d353df_90bd_4382_8bc2_3f6192b76e34);
pub const CWMVEncMediaObject2: windows_core::GUID = windows_core::GUID::from_u128(0x96b57cdd_8966_410c_bb1f_c97eea765c04);
pub const CWMVXEncMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x7e320092_596a_41b2_bbeb_175d10504eb6);
pub const CWVC1DecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xc9bfbccf_e60e_4588_a3df_5a03b1fd9585);
pub const CWVC1EncMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0x44653d0d_8cca_41e7_baca_884337b747ac);
pub const CZuneAACCCDecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xa74e98f2_52d6_4b4e_885b_e0a6ca4f187a);
pub const CZuneM4S2DecMediaObject: windows_core::GUID = windows_core::GUID::from_u128(0xc56fc25c_0fc6_404a_9503_b10bf51a8ab9);
pub type ChMtxType = f32;
pub const E_TOCPARSER_INVALIDASFFILE: windows_core::HRESULT = windows_core::HRESULT(0x99000001_u32 as _);
pub const E_TOCPARSER_INVALIDRIFFFILE: windows_core::HRESULT = windows_core::HRESULT(0x99000002_u32 as _);
pub type FILE_ACCESSMODE = i32;
pub type FILE_OPENMODE = i32;
windows_core::imp::define_interface!(IClusterDetector, IClusterDetector_Vtbl, 0x3f07f7b7_c680_41d9_9423_915107ec9ff9);
windows_core::imp::interface_hierarchy!(IClusterDetector, windows_core::IUnknown);
impl IClusterDetector {
    #[cfg(feature = "shared")]
    pub unsafe fn Initialize(&self, wbaseentrylevel: super::WORD, wclusterentrylevel: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), wbaseentrylevel, wclusterentrylevel) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Detect<P3>(&self, dwmaxnumclusters: super::DWORD, fminclusterduration: super::FLOAT, fmaxclusterduration: super::FLOAT, psrctoc: P3) -> windows_core::Result<IToc>
    where
        P3: windows_core::Param<IToc>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Detect)(windows_core::Interface::as_raw(self), dwmaxnumclusters, fminclusterduration, fmaxclusterduration, psrctoc.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClusterDetector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Initialize: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Detect: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::FLOAT, super::FLOAT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Detect: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IClusterDetector_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, wbaseentrylevel: super::WORD, wclusterentrylevel: super::WORD) -> windows_core::Result<()>;
    fn Detect(&self, dwmaxnumclusters: super::DWORD, fminclusterduration: super::FLOAT, fmaxclusterduration: super::FLOAT, psrctoc: windows_core::Ref<IToc>) -> windows_core::Result<IToc>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IClusterDetector_Vtbl {
    pub const fn new<Identity: IClusterDetector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IClusterDetector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wbaseentrylevel: super::WORD, wclusterentrylevel: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClusterDetector_Impl::Initialize(this, core::mem::transmute_copy(&wbaseentrylevel), core::mem::transmute_copy(&wclusterentrylevel)).into()
            }
        }
        unsafe extern "system" fn Detect<Identity: IClusterDetector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxnumclusters: super::DWORD, fminclusterduration: super::FLOAT, fmaxclusterduration: super::FLOAT, psrctoc: *mut core::ffi::c_void, ppdsttoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterDetector_Impl::Detect(this, core::mem::transmute_copy(&dwmaxnumclusters), core::mem::transmute_copy(&fminclusterduration), core::mem::transmute_copy(&fmaxclusterduration), core::mem::transmute_copy(&psrctoc)) {
                    Ok(ok__) => {
                        ppdsttoc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET>, Detect: Detect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClusterDetector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IClusterDetector {}
windows_core::imp::define_interface!(IFileClient, IFileClient_Vtbl, 0xbfccd196_1244_4840_ab44_480975c4ffe4);
windows_core::imp::interface_hierarchy!(IFileClient, windows_core::IUnknown);
impl IFileClient {
    #[cfg(feature = "shared")]
    pub unsafe fn GetObjectDiskSize(&self) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectDiskSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Write<P0>(&self, pfio: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFileIo>,
    {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pfio.param().abi()) }
    }
    pub unsafe fn Read<P0>(&self, pfio: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFileIo>,
    {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pfio.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetObjectDiskSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetObjectDiskSize: usize,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IFileClient_Impl: windows_core::IUnknownImpl {
    fn GetObjectDiskSize(&self) -> windows_core::Result<super::QWORD>;
    fn Write(&self, pfio: windows_core::Ref<IFileIo>) -> windows_core::Result<()>;
    fn Read(&self, pfio: windows_core::Ref<IFileIo>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IFileClient_Vtbl {
    pub const fn new<Identity: IFileClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectDiskSize<Identity: IFileClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqwsize: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileClient_Impl::GetObjectDiskSize(this) {
                    Ok(ok__) => {
                        pqwsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Write<Identity: IFileClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfio: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileClient_Impl::Write(this, core::mem::transmute_copy(&pfio)).into()
            }
        }
        unsafe extern "system" fn Read<Identity: IFileClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfio: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileClient_Impl::Read(this, core::mem::transmute_copy(&pfio)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectDiskSize: GetObjectDiskSize::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileClient as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IFileClient {}
windows_core::imp::define_interface!(IFileIo, IFileIo_Vtbl, 0x11993196_1244_4840_ab44_480975c4ffe4);
windows_core::imp::interface_hierarchy!(IFileIo, windows_core::IUnknown);
impl IFileIo {
    pub unsafe fn Initialize<P2>(&self, eaccessmode: FILE_ACCESSMODE, eopenmode: FILE_OPENMODE, pwszfilename: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), eaccessmode, eopenmode, pwszfilename.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLength(&self) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetLength(&self, qwlength: super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLength)(windows_core::Interface::as_raw(self), qwlength) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCurrentPosition(&self) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetCurrentPosition(&self, qwposition: super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentPosition)(windows_core::Interface::as_raw(self), qwposition) }
    }
    pub unsafe fn IsEndOfStream(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEndOfStream)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Read(&self, pbt: *const super::BYTE, ul: super::ULONG) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pbt, ul, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Write(&self, pbt: *const super::BYTE, ul: super::ULONG) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pbt, ul, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Seek(&self, eseekorigin: SEEK_ORIGIN, qwseekoffset: super::QWORD, dwseekflags: super::DWORD) -> windows_core::Result<super::QWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), eseekorigin, qwseekoffset, dwseekflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileIo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, FILE_ACCESSMODE, FILE_OPENMODE, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLength: usize,
    #[cfg(feature = "shared")]
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetLength: usize,
    #[cfg(feature = "shared")]
    pub GetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCurrentPosition: usize,
    #[cfg(feature = "shared")]
    pub SetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetCurrentPosition: usize,
    pub IsEndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Read: usize,
    #[cfg(feature = "shared")]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Write: usize,
    #[cfg(feature = "shared")]
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, SEEK_ORIGIN, super::QWORD, super::DWORD, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Seek: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IFileIo_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, eaccessmode: FILE_ACCESSMODE, eopenmode: FILE_OPENMODE, pwszfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetLength(&self) -> windows_core::Result<super::QWORD>;
    fn SetLength(&self, qwlength: super::QWORD) -> windows_core::Result<()>;
    fn GetCurrentPosition(&self) -> windows_core::Result<super::QWORD>;
    fn SetCurrentPosition(&self, qwposition: super::QWORD) -> windows_core::Result<()>;
    fn IsEndOfStream(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Read(&self, pbt: *const super::BYTE, ul: super::ULONG) -> windows_core::Result<super::ULONG>;
    fn Write(&self, pbt: *const super::BYTE, ul: super::ULONG) -> windows_core::Result<super::ULONG>;
    fn Seek(&self, eseekorigin: SEEK_ORIGIN, qwseekoffset: super::QWORD, dwseekflags: super::DWORD) -> windows_core::Result<super::QWORD>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IFileIo_Vtbl {
    pub const fn new<Identity: IFileIo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IFileIo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eaccessmode: FILE_ACCESSMODE, eopenmode: FILE_OPENMODE, pwszfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileIo_Impl::Initialize(this, core::mem::transmute_copy(&eaccessmode), core::mem::transmute_copy(&eopenmode), core::mem::transmute(&pwszfilename)).into()
            }
        }
        unsafe extern "system" fn GetLength<Identity: IFileIo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqwlength: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileIo_Impl::GetLength(this) {
                    Ok(ok__) => {
                        pqwlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLength<Identity: IFileIo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, qwlength: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileIo_Impl::SetLength(this, core::mem::transmute_copy(&qwlength)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPosition<Identity: IFileIo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqwposition: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileIo_Impl::GetCurrentPosition(this) {
                    Ok(ok__) => {
                        pqwposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentPosition<Identity: IFileIo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, qwposition: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileIo_Impl::SetCurrentPosition(this, core::mem::transmute_copy(&qwposition)).into()
            }
        }
        unsafe extern "system" fn IsEndOfStream<Identity: IFileIo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbendofstream: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileIo_Impl::IsEndOfStream(this) {
                    Ok(ok__) => {
                        pbendofstream.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Read<Identity: IFileIo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbt: *const super::BYTE, ul: super::ULONG, pulread: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileIo_Impl::Read(this, core::mem::transmute_copy(&pbt), core::mem::transmute_copy(&ul)) {
                    Ok(ok__) => {
                        pulread.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Write<Identity: IFileIo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbt: *const super::BYTE, ul: super::ULONG, pulwritten: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileIo_Impl::Write(this, core::mem::transmute_copy(&pbt), core::mem::transmute_copy(&ul)) {
                    Ok(ok__) => {
                        pulwritten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Seek<Identity: IFileIo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eseekorigin: SEEK_ORIGIN, qwseekoffset: super::QWORD, dwseekflags: super::DWORD, pqwcurrentposition: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileIo_Impl::Seek(this, core::mem::transmute_copy(&eseekorigin), core::mem::transmute_copy(&qwseekoffset), core::mem::transmute_copy(&dwseekflags)) {
                    Ok(ok__) => {
                        pqwcurrentposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IFileIo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileIo_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetLength: GetLength::<Identity, OFFSET>,
            SetLength: SetLength::<Identity, OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Identity, OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Identity, OFFSET>,
            IsEndOfStream: IsEndOfStream::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileIo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IFileIo {}
windows_core::imp::define_interface!(IToc, IToc_Vtbl, 0xd6f05441_a919_423b_91a0_89d5b4a8ab77);
windows_core::imp::interface_hierarchy!(IToc, windows_core::IUnknown);
impl IToc {
    #[cfg(feature = "shared")]
    pub unsafe fn SetDescriptor(&self, pdescriptor: *const TOC_DESCRIPTOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescriptor)(windows_core::Interface::as_raw(self), pdescriptor) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDescriptor(&self, pdescriptor: *mut TOC_DESCRIPTOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDescriptor)(windows_core::Interface::as_raw(self), pdescriptor as _) }
    }
    pub unsafe fn SetDescription<P0>(&self, pwszdescription: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), pwszdescription.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDescription(&self, pwdescriptionsize: *mut super::WORD, pwszdescription: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), pwdescriptionsize as _, pwszdescription.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetContext(&self, dwcontextsize: super::DWORD, pbtcontext: *const super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), dwcontextsize, pbtcontext) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetContext(&self, pdwcontextsize: *mut super::DWORD, pbtcontext: *mut super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), pdwcontextsize as _, pbtcontext as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetEntryListCount(&self) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntryListCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetEntryListByIndex(&self, wentrylistindex: super::WORD) -> windows_core::Result<ITocEntryList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntryListByIndex)(windows_core::Interface::as_raw(self), wentrylistindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddEntryList<P0>(&self, pentrylist: P0) -> windows_core::Result<super::WORD>
    where
        P0: windows_core::Param<ITocEntryList>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddEntryList)(windows_core::Interface::as_raw(self), pentrylist.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddEntryListByIndex<P1>(&self, wentrylistindex: super::WORD, pentrylist: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITocEntryList>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddEntryListByIndex)(windows_core::Interface::as_raw(self), wentrylistindex, pentrylist.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveEntryListByIndex(&self, wentrylistindex: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveEntryListByIndex)(windows_core::Interface::as_raw(self), wentrylistindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *const TOC_DESCRIPTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetDescriptor: usize,
    #[cfg(feature = "shared")]
    pub GetDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TOC_DESCRIPTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDescriptor: usize,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD, windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDescription: usize,
    #[cfg(feature = "shared")]
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetContext: usize,
    #[cfg(feature = "shared")]
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetContext: usize,
    #[cfg(feature = "shared")]
    pub GetEntryListCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEntryListCount: usize,
    #[cfg(feature = "shared")]
    pub GetEntryListByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEntryListByIndex: usize,
    #[cfg(feature = "shared")]
    pub AddEntryList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddEntryList: usize,
    #[cfg(feature = "shared")]
    pub AddEntryListByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddEntryListByIndex: usize,
    #[cfg(feature = "shared")]
    pub RemoveEntryListByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveEntryListByIndex: usize,
}
#[cfg(feature = "shared")]
pub trait IToc_Impl: windows_core::IUnknownImpl {
    fn SetDescriptor(&self, pdescriptor: *const TOC_DESCRIPTOR) -> windows_core::Result<()>;
    fn GetDescriptor(&self, pdescriptor: *mut TOC_DESCRIPTOR) -> windows_core::Result<()>;
    fn SetDescription(&self, pwszdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDescription(&self, pwdescriptionsize: *mut super::WORD, pwszdescription: windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetContext(&self, dwcontextsize: super::DWORD, pbtcontext: *const super::BYTE) -> windows_core::Result<()>;
    fn GetContext(&self, pdwcontextsize: *mut super::DWORD, pbtcontext: *mut super::BYTE) -> windows_core::Result<()>;
    fn GetEntryListCount(&self) -> windows_core::Result<super::WORD>;
    fn GetEntryListByIndex(&self, wentrylistindex: super::WORD) -> windows_core::Result<ITocEntryList>;
    fn AddEntryList(&self, pentrylist: windows_core::Ref<ITocEntryList>) -> windows_core::Result<super::WORD>;
    fn AddEntryListByIndex(&self, wentrylistindex: super::WORD, pentrylist: windows_core::Ref<ITocEntryList>) -> windows_core::Result<()>;
    fn RemoveEntryListByIndex(&self, wentrylistindex: super::WORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IToc_Vtbl {
    pub const fn new<Identity: IToc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDescriptor<Identity: IToc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescriptor: *const TOC_DESCRIPTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToc_Impl::SetDescriptor(this, core::mem::transmute_copy(&pdescriptor)).into()
            }
        }
        unsafe extern "system" fn GetDescriptor<Identity: IToc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescriptor: *mut TOC_DESCRIPTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToc_Impl::GetDescriptor(this, core::mem::transmute_copy(&pdescriptor)).into()
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IToc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToc_Impl::SetDescription(this, core::mem::transmute(&pwszdescription)).into()
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IToc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwdescriptionsize: *mut super::WORD, pwszdescription: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToc_Impl::GetDescription(this, core::mem::transmute_copy(&pwdescriptionsize), core::mem::transmute_copy(&pwszdescription)).into()
            }
        }
        unsafe extern "system" fn SetContext<Identity: IToc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcontextsize: super::DWORD, pbtcontext: *const super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToc_Impl::SetContext(this, core::mem::transmute_copy(&dwcontextsize), core::mem::transmute_copy(&pbtcontext)).into()
            }
        }
        unsafe extern "system" fn GetContext<Identity: IToc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcontextsize: *mut super::DWORD, pbtcontext: *mut super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToc_Impl::GetContext(this, core::mem::transmute_copy(&pdwcontextsize), core::mem::transmute_copy(&pbtcontext)).into()
            }
        }
        unsafe extern "system" fn GetEntryListCount<Identity: IToc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcount: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToc_Impl::GetEntryListCount(this) {
                    Ok(ok__) => {
                        pwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEntryListByIndex<Identity: IToc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wentrylistindex: super::WORD, ppentrylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToc_Impl::GetEntryListByIndex(this, core::mem::transmute_copy(&wentrylistindex)) {
                    Ok(ok__) => {
                        ppentrylist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddEntryList<Identity: IToc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pentrylist: *mut core::ffi::c_void, pwentrylistindex: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToc_Impl::AddEntryList(this, core::mem::transmute_copy(&pentrylist)) {
                    Ok(ok__) => {
                        pwentrylistindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddEntryListByIndex<Identity: IToc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wentrylistindex: super::WORD, pentrylist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToc_Impl::AddEntryListByIndex(this, core::mem::transmute_copy(&wentrylistindex), core::mem::transmute_copy(&pentrylist)).into()
            }
        }
        unsafe extern "system" fn RemoveEntryListByIndex<Identity: IToc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wentrylistindex: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToc_Impl::RemoveEntryListByIndex(this, core::mem::transmute_copy(&wentrylistindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDescriptor: SetDescriptor::<Identity, OFFSET>,
            GetDescriptor: GetDescriptor::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetContext: SetContext::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
            GetEntryListCount: GetEntryListCount::<Identity, OFFSET>,
            GetEntryListByIndex: GetEntryListByIndex::<Identity, OFFSET>,
            AddEntryList: AddEntryList::<Identity, OFFSET>,
            AddEntryListByIndex: AddEntryListByIndex::<Identity, OFFSET>,
            RemoveEntryListByIndex: RemoveEntryListByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IToc as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IToc {}
windows_core::imp::define_interface!(ITocCollection, ITocCollection_Vtbl, 0x23fee831_ae96_42df_b170_25a04847a3ca);
windows_core::imp::interface_hierarchy!(ITocCollection, windows_core::IUnknown);
impl ITocCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn GetEntryCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetEntryByIndex(&self, dwentryindex: super::DWORD) -> windows_core::Result<IToc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntryByIndex)(windows_core::Interface::as_raw(self), dwentryindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddEntry<P0>(&self, ptoc: P0) -> windows_core::Result<super::DWORD>
    where
        P0: windows_core::Param<IToc>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddEntry)(windows_core::Interface::as_raw(self), ptoc.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddEntryByIndex<P1>(&self, dwentryindex: super::DWORD, ptoc: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IToc>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddEntryByIndex)(windows_core::Interface::as_raw(self), dwentryindex, ptoc.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveEntryByIndex(&self, dwentryindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveEntryByIndex)(windows_core::Interface::as_raw(self), dwentryindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITocCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetEntryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEntryCount: usize,
    #[cfg(feature = "shared")]
    pub GetEntryByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEntryByIndex: usize,
    #[cfg(feature = "shared")]
    pub AddEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddEntry: usize,
    #[cfg(feature = "shared")]
    pub AddEntryByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddEntryByIndex: usize,
    #[cfg(feature = "shared")]
    pub RemoveEntryByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveEntryByIndex: usize,
}
#[cfg(feature = "shared")]
pub trait ITocCollection_Impl: windows_core::IUnknownImpl {
    fn GetEntryCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetEntryByIndex(&self, dwentryindex: super::DWORD) -> windows_core::Result<IToc>;
    fn AddEntry(&self, ptoc: windows_core::Ref<IToc>) -> windows_core::Result<super::DWORD>;
    fn AddEntryByIndex(&self, dwentryindex: super::DWORD, ptoc: windows_core::Ref<IToc>) -> windows_core::Result<()>;
    fn RemoveEntryByIndex(&self, dwentryindex: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ITocCollection_Vtbl {
    pub const fn new<Identity: ITocCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEntryCount<Identity: ITocCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwentrycount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITocCollection_Impl::GetEntryCount(this) {
                    Ok(ok__) => {
                        pdwentrycount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEntryByIndex<Identity: ITocCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwentryindex: super::DWORD, pptoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITocCollection_Impl::GetEntryByIndex(this, core::mem::transmute_copy(&dwentryindex)) {
                    Ok(ok__) => {
                        pptoc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddEntry<Identity: ITocCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptoc: *mut core::ffi::c_void, pdwentryindex: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITocCollection_Impl::AddEntry(this, core::mem::transmute_copy(&ptoc)) {
                    Ok(ok__) => {
                        pdwentryindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddEntryByIndex<Identity: ITocCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwentryindex: super::DWORD, ptoc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocCollection_Impl::AddEntryByIndex(this, core::mem::transmute_copy(&dwentryindex), core::mem::transmute_copy(&ptoc)).into()
            }
        }
        unsafe extern "system" fn RemoveEntryByIndex<Identity: ITocCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwentryindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocCollection_Impl::RemoveEntryByIndex(this, core::mem::transmute_copy(&dwentryindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEntryCount: GetEntryCount::<Identity, OFFSET>,
            GetEntryByIndex: GetEntryByIndex::<Identity, OFFSET>,
            AddEntry: AddEntry::<Identity, OFFSET>,
            AddEntryByIndex: AddEntryByIndex::<Identity, OFFSET>,
            RemoveEntryByIndex: RemoveEntryByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITocCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITocCollection {}
windows_core::imp::define_interface!(ITocEntry, ITocEntry_Vtbl, 0xf22f5e06_585c_4def_8523_6555cfbc0cb3);
windows_core::imp::interface_hierarchy!(ITocEntry, windows_core::IUnknown);
impl ITocEntry {
    pub unsafe fn SetTitle<P0>(&self, pwsztitle: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), pwsztitle.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTitle(&self, pwtitlesize: *mut super::WORD, pwsztitle: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTitle)(windows_core::Interface::as_raw(self), pwtitlesize as _, pwsztitle.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetDescriptor(&self, pdescriptor: *const TOC_ENTRY_DESCRIPTOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescriptor)(windows_core::Interface::as_raw(self), pdescriptor) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDescriptor(&self, pdescriptor: *mut TOC_ENTRY_DESCRIPTOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDescriptor)(windows_core::Interface::as_raw(self), pdescriptor as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSubEntries(&self, dwnumsubentries: super::DWORD, pwsubentryindices: *const super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSubEntries)(windows_core::Interface::as_raw(self), dwnumsubentries, pwsubentryindices) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSubEntries(&self, pdwnumsubentries: *mut super::DWORD, pwsubentryindices: *mut super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSubEntries)(windows_core::Interface::as_raw(self), pdwnumsubentries as _, pwsubentryindices as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetDescriptionData(&self, dwdescriptiondatasize: super::DWORD, pbtdescriptiondata: *const super::BYTE, pguidtype: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescriptionData)(windows_core::Interface::as_raw(self), dwdescriptiondatasize, pbtdescriptiondata, pguidtype) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDescriptionData(&self, pdwdescriptiondatasize: *mut super::DWORD, pbtdescriptiondata: *mut super::BYTE, pguidtype: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDescriptionData)(windows_core::Interface::as_raw(self), pdwdescriptiondatasize as _, pbtdescriptiondata as _, pguidtype as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITocEntry_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD, windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTitle: usize,
    #[cfg(feature = "shared")]
    pub SetDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *const TOC_ENTRY_DESCRIPTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetDescriptor: usize,
    #[cfg(feature = "shared")]
    pub GetDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TOC_ENTRY_DESCRIPTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDescriptor: usize,
    #[cfg(feature = "shared")]
    pub SetSubEntries: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSubEntries: usize,
    #[cfg(feature = "shared")]
    pub GetSubEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSubEntries: usize,
    #[cfg(feature = "shared")]
    pub SetDescriptionData: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::BYTE, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetDescriptionData: usize,
    #[cfg(feature = "shared")]
    pub GetDescriptionData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::BYTE, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDescriptionData: usize,
}
#[cfg(feature = "shared")]
pub trait ITocEntry_Impl: windows_core::IUnknownImpl {
    fn SetTitle(&self, pwsztitle: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetTitle(&self, pwtitlesize: *mut super::WORD, pwsztitle: windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetDescriptor(&self, pdescriptor: *const TOC_ENTRY_DESCRIPTOR) -> windows_core::Result<()>;
    fn GetDescriptor(&self, pdescriptor: *mut TOC_ENTRY_DESCRIPTOR) -> windows_core::Result<()>;
    fn SetSubEntries(&self, dwnumsubentries: super::DWORD, pwsubentryindices: *const super::WORD) -> windows_core::Result<()>;
    fn GetSubEntries(&self, pdwnumsubentries: *mut super::DWORD, pwsubentryindices: *mut super::WORD) -> windows_core::Result<()>;
    fn SetDescriptionData(&self, dwdescriptiondatasize: super::DWORD, pbtdescriptiondata: *const super::BYTE, pguidtype: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetDescriptionData(&self, pdwdescriptiondatasize: *mut super::DWORD, pbtdescriptiondata: *mut super::BYTE, pguidtype: *mut windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ITocEntry_Vtbl {
    pub const fn new<Identity: ITocEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTitle<Identity: ITocEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztitle: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocEntry_Impl::SetTitle(this, core::mem::transmute(&pwsztitle)).into()
            }
        }
        unsafe extern "system" fn GetTitle<Identity: ITocEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwtitlesize: *mut super::WORD, pwsztitle: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocEntry_Impl::GetTitle(this, core::mem::transmute_copy(&pwtitlesize), core::mem::transmute_copy(&pwsztitle)).into()
            }
        }
        unsafe extern "system" fn SetDescriptor<Identity: ITocEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescriptor: *const TOC_ENTRY_DESCRIPTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocEntry_Impl::SetDescriptor(this, core::mem::transmute_copy(&pdescriptor)).into()
            }
        }
        unsafe extern "system" fn GetDescriptor<Identity: ITocEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescriptor: *mut TOC_ENTRY_DESCRIPTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocEntry_Impl::GetDescriptor(this, core::mem::transmute_copy(&pdescriptor)).into()
            }
        }
        unsafe extern "system" fn SetSubEntries<Identity: ITocEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnumsubentries: super::DWORD, pwsubentryindices: *const super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocEntry_Impl::SetSubEntries(this, core::mem::transmute_copy(&dwnumsubentries), core::mem::transmute_copy(&pwsubentryindices)).into()
            }
        }
        unsafe extern "system" fn GetSubEntries<Identity: ITocEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwnumsubentries: *mut super::DWORD, pwsubentryindices: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocEntry_Impl::GetSubEntries(this, core::mem::transmute_copy(&pdwnumsubentries), core::mem::transmute_copy(&pwsubentryindices)).into()
            }
        }
        unsafe extern "system" fn SetDescriptionData<Identity: ITocEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdescriptiondatasize: super::DWORD, pbtdescriptiondata: *const super::BYTE, pguidtype: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocEntry_Impl::SetDescriptionData(this, core::mem::transmute_copy(&dwdescriptiondatasize), core::mem::transmute_copy(&pbtdescriptiondata), core::mem::transmute_copy(&pguidtype)).into()
            }
        }
        unsafe extern "system" fn GetDescriptionData<Identity: ITocEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdescriptiondatasize: *mut super::DWORD, pbtdescriptiondata: *mut super::BYTE, pguidtype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocEntry_Impl::GetDescriptionData(this, core::mem::transmute_copy(&pdwdescriptiondatasize), core::mem::transmute_copy(&pbtdescriptiondata), core::mem::transmute_copy(&pguidtype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTitle: SetTitle::<Identity, OFFSET>,
            GetTitle: GetTitle::<Identity, OFFSET>,
            SetDescriptor: SetDescriptor::<Identity, OFFSET>,
            GetDescriptor: GetDescriptor::<Identity, OFFSET>,
            SetSubEntries: SetSubEntries::<Identity, OFFSET>,
            GetSubEntries: GetSubEntries::<Identity, OFFSET>,
            SetDescriptionData: SetDescriptionData::<Identity, OFFSET>,
            GetDescriptionData: GetDescriptionData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITocEntry as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITocEntry {}
windows_core::imp::define_interface!(ITocEntryList, ITocEntryList_Vtbl, 0x3a8cccbd_0efd_43a3_b838_f38a552ba237);
windows_core::imp::interface_hierarchy!(ITocEntryList, windows_core::IUnknown);
impl ITocEntryList {
    #[cfg(feature = "shared")]
    pub unsafe fn GetEntryCount(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetEntryByIndex(&self, dwentryindex: super::DWORD) -> windows_core::Result<ITocEntry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntryByIndex)(windows_core::Interface::as_raw(self), dwentryindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddEntry<P0>(&self, pentry: P0) -> windows_core::Result<super::DWORD>
    where
        P0: windows_core::Param<ITocEntry>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddEntry)(windows_core::Interface::as_raw(self), pentry.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddEntryByIndex<P1>(&self, dwentryindex: super::DWORD, pentry: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ITocEntry>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddEntryByIndex)(windows_core::Interface::as_raw(self), dwentryindex, pentry.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveEntryByIndex(&self, dwentryindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveEntryByIndex)(windows_core::Interface::as_raw(self), dwentryindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITocEntryList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetEntryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEntryCount: usize,
    #[cfg(feature = "shared")]
    pub GetEntryByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEntryByIndex: usize,
    #[cfg(feature = "shared")]
    pub AddEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddEntry: usize,
    #[cfg(feature = "shared")]
    pub AddEntryByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddEntryByIndex: usize,
    #[cfg(feature = "shared")]
    pub RemoveEntryByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveEntryByIndex: usize,
}
#[cfg(feature = "shared")]
pub trait ITocEntryList_Impl: windows_core::IUnknownImpl {
    fn GetEntryCount(&self) -> windows_core::Result<super::DWORD>;
    fn GetEntryByIndex(&self, dwentryindex: super::DWORD) -> windows_core::Result<ITocEntry>;
    fn AddEntry(&self, pentry: windows_core::Ref<ITocEntry>) -> windows_core::Result<super::DWORD>;
    fn AddEntryByIndex(&self, dwentryindex: super::DWORD, pentry: windows_core::Ref<ITocEntry>) -> windows_core::Result<()>;
    fn RemoveEntryByIndex(&self, dwentryindex: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ITocEntryList_Vtbl {
    pub const fn new<Identity: ITocEntryList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEntryCount<Identity: ITocEntryList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwentrycount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITocEntryList_Impl::GetEntryCount(this) {
                    Ok(ok__) => {
                        pdwentrycount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEntryByIndex<Identity: ITocEntryList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwentryindex: super::DWORD, ppentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITocEntryList_Impl::GetEntryByIndex(this, core::mem::transmute_copy(&dwentryindex)) {
                    Ok(ok__) => {
                        ppentry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddEntry<Identity: ITocEntryList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pentry: *mut core::ffi::c_void, pdwentryindex: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITocEntryList_Impl::AddEntry(this, core::mem::transmute_copy(&pentry)) {
                    Ok(ok__) => {
                        pdwentryindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddEntryByIndex<Identity: ITocEntryList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwentryindex: super::DWORD, pentry: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocEntryList_Impl::AddEntryByIndex(this, core::mem::transmute_copy(&dwentryindex), core::mem::transmute_copy(&pentry)).into()
            }
        }
        unsafe extern "system" fn RemoveEntryByIndex<Identity: ITocEntryList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwentryindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocEntryList_Impl::RemoveEntryByIndex(this, core::mem::transmute_copy(&dwentryindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEntryCount: GetEntryCount::<Identity, OFFSET>,
            GetEntryByIndex: GetEntryByIndex::<Identity, OFFSET>,
            AddEntry: AddEntry::<Identity, OFFSET>,
            AddEntryByIndex: AddEntryByIndex::<Identity, OFFSET>,
            RemoveEntryByIndex: RemoveEntryByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITocEntryList as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITocEntryList {}
windows_core::imp::define_interface!(ITocParser, ITocParser_Vtbl, 0xecfb9a55_9298_4f49_887f_0b36206599d2);
windows_core::imp::interface_hierarchy!(ITocParser, windows_core::IUnknown);
impl ITocParser {
    pub unsafe fn Init<P0>(&self, pwszfilename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), pwszfilename.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTocCount(&self, enumtocpostype: TOC_POS_TYPE) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTocCount)(windows_core::Interface::as_raw(self), enumtocpostype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTocByIndex(&self, enumtocpostype: TOC_POS_TYPE, dwtocindex: super::DWORD) -> windows_core::Result<IToc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTocByIndex)(windows_core::Interface::as_raw(self), enumtocpostype, dwtocindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTocByType(&self, enumtocpostype: TOC_POS_TYPE, guidtoctype: windows_core::GUID) -> windows_core::Result<ITocCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTocByType)(windows_core::Interface::as_raw(self), enumtocpostype, guidtoctype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddToc<P1>(&self, enumtocpostype: TOC_POS_TYPE, ptoc: P1) -> windows_core::Result<super::DWORD>
    where
        P1: windows_core::Param<IToc>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddToc)(windows_core::Interface::as_raw(self), enumtocpostype, ptoc.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveTocByIndex(&self, enumtocpostype: TOC_POS_TYPE, dwtocindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveTocByIndex)(windows_core::Interface::as_raw(self), enumtocpostype, dwtocindex) }
    }
    pub unsafe fn RemoveTocByType(&self, enumtocpostype: TOC_POS_TYPE, guidtoctype: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveTocByType)(windows_core::Interface::as_raw(self), enumtocpostype, guidtoctype) }
    }
    pub unsafe fn Commit(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITocParser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetTocCount: unsafe extern "system" fn(*mut core::ffi::c_void, TOC_POS_TYPE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTocCount: usize,
    #[cfg(feature = "shared")]
    pub GetTocByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, TOC_POS_TYPE, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTocByIndex: usize,
    pub GetTocByType: unsafe extern "system" fn(*mut core::ffi::c_void, TOC_POS_TYPE, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub AddToc: unsafe extern "system" fn(*mut core::ffi::c_void, TOC_POS_TYPE, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddToc: usize,
    #[cfg(feature = "shared")]
    pub RemoveTocByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, TOC_POS_TYPE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveTocByIndex: usize,
    pub RemoveTocByType: unsafe extern "system" fn(*mut core::ffi::c_void, TOC_POS_TYPE, windows_core::GUID) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait ITocParser_Impl: windows_core::IUnknownImpl {
    fn Init(&self, pwszfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetTocCount(&self, enumtocpostype: TOC_POS_TYPE) -> windows_core::Result<super::DWORD>;
    fn GetTocByIndex(&self, enumtocpostype: TOC_POS_TYPE, dwtocindex: super::DWORD) -> windows_core::Result<IToc>;
    fn GetTocByType(&self, enumtocpostype: TOC_POS_TYPE, guidtoctype: &windows_core::GUID) -> windows_core::Result<ITocCollection>;
    fn AddToc(&self, enumtocpostype: TOC_POS_TYPE, ptoc: windows_core::Ref<IToc>) -> windows_core::Result<super::DWORD>;
    fn RemoveTocByIndex(&self, enumtocpostype: TOC_POS_TYPE, dwtocindex: super::DWORD) -> windows_core::Result<()>;
    fn RemoveTocByType(&self, enumtocpostype: TOC_POS_TYPE, guidtoctype: &windows_core::GUID) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ITocParser_Vtbl {
    pub const fn new<Identity: ITocParser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: ITocParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocParser_Impl::Init(this, core::mem::transmute(&pwszfilename)).into()
            }
        }
        unsafe extern "system" fn GetTocCount<Identity: ITocParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumtocpostype: TOC_POS_TYPE, pdwtoccount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITocParser_Impl::GetTocCount(this, core::mem::transmute_copy(&enumtocpostype)) {
                    Ok(ok__) => {
                        pdwtoccount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTocByIndex<Identity: ITocParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumtocpostype: TOC_POS_TYPE, dwtocindex: super::DWORD, pptoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITocParser_Impl::GetTocByIndex(this, core::mem::transmute_copy(&enumtocpostype), core::mem::transmute_copy(&dwtocindex)) {
                    Ok(ok__) => {
                        pptoc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTocByType<Identity: ITocParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumtocpostype: TOC_POS_TYPE, guidtoctype: windows_core::GUID, pptocs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITocParser_Impl::GetTocByType(this, core::mem::transmute_copy(&enumtocpostype), core::mem::transmute(&guidtoctype)) {
                    Ok(ok__) => {
                        pptocs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddToc<Identity: ITocParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumtocpostype: TOC_POS_TYPE, ptoc: *mut core::ffi::c_void, pdwtocindex: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITocParser_Impl::AddToc(this, core::mem::transmute_copy(&enumtocpostype), core::mem::transmute_copy(&ptoc)) {
                    Ok(ok__) => {
                        pdwtocindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveTocByIndex<Identity: ITocParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumtocpostype: TOC_POS_TYPE, dwtocindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocParser_Impl::RemoveTocByIndex(this, core::mem::transmute_copy(&enumtocpostype), core::mem::transmute_copy(&dwtocindex)).into()
            }
        }
        unsafe extern "system" fn RemoveTocByType<Identity: ITocParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumtocpostype: TOC_POS_TYPE, guidtoctype: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocParser_Impl::RemoveTocByType(this, core::mem::transmute_copy(&enumtocpostype), core::mem::transmute(&guidtoctype)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: ITocParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITocParser_Impl::Commit(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            GetTocCount: GetTocCount::<Identity, OFFSET>,
            GetTocByIndex: GetTocByIndex::<Identity, OFFSET>,
            GetTocByType: GetTocByType::<Identity, OFFSET>,
            AddToc: AddToc::<Identity, OFFSET>,
            RemoveTocByIndex: RemoveTocByIndex::<Identity, OFFSET>,
            RemoveTocByType: RemoveTocByType::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITocParser as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ITocParser {}
windows_core::imp::define_interface!(IValidateBinding, IValidateBinding_Vtbl, 0x04a578b2_e778_422a_a805_b3ee54d90bd9);
windows_core::imp::interface_hierarchy!(IValidateBinding, windows_core::IUnknown);
impl IValidateBinding {
    #[cfg(feature = "shared")]
    pub unsafe fn GetIdentifier(&self, guidlicensorid: windows_core::GUID, pbephemeron: &[super::BYTE], ppbblobvalidationid: *mut *mut super::BYTE, pcbblobsize: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIdentifier)(windows_core::Interface::as_raw(self), guidlicensorid, pbephemeron.as_ptr(), pbephemeron.len().try_into().unwrap(), ppbblobvalidationid as _, pcbblobsize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IValidateBinding_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const super::BYTE, super::DWORD, *mut *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetIdentifier: usize,
}
#[cfg(feature = "shared")]
pub trait IValidateBinding_Impl: windows_core::IUnknownImpl {
    fn GetIdentifier(&self, guidlicensorid: &windows_core::GUID, pbephemeron: *const super::BYTE, cbephemeron: super::DWORD, ppbblobvalidationid: *mut *mut super::BYTE, pcbblobsize: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IValidateBinding_Vtbl {
    pub const fn new<Identity: IValidateBinding_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIdentifier<Identity: IValidateBinding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidlicensorid: windows_core::GUID, pbephemeron: *const super::BYTE, cbephemeron: super::DWORD, ppbblobvalidationid: *mut *mut super::BYTE, pcbblobsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValidateBinding_Impl::GetIdentifier(this, core::mem::transmute(&guidlicensorid), core::mem::transmute_copy(&pbephemeron), core::mem::transmute_copy(&cbephemeron), core::mem::transmute_copy(&ppbblobvalidationid), core::mem::transmute_copy(&pcbblobsize)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIdentifier: GetIdentifier::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValidateBinding as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IValidateBinding {}
windows_core::imp::define_interface!(IWMCodecLeakyBucket, IWMCodecLeakyBucket_Vtbl, 0xa81ba647_6227_43b7_b231_c7b15135dd7d);
windows_core::imp::interface_hierarchy!(IWMCodecLeakyBucket, windows_core::IUnknown);
impl IWMCodecLeakyBucket {
    #[cfg(feature = "shared")]
    pub unsafe fn SetBufferSizeBits(&self, ulbuffersize: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBufferSizeBits)(windows_core::Interface::as_raw(self), ulbuffersize) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBufferSizeBits(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBufferSizeBits)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetBufferFullnessBits(&self, ulbufferfullness: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBufferFullnessBits)(windows_core::Interface::as_raw(self), ulbufferfullness) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBufferFullnessBits(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBufferFullnessBits)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecLeakyBucket_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetBufferSizeBits: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetBufferSizeBits: usize,
    #[cfg(feature = "shared")]
    pub GetBufferSizeBits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBufferSizeBits: usize,
    #[cfg(feature = "shared")]
    pub SetBufferFullnessBits: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetBufferFullnessBits: usize,
    #[cfg(feature = "shared")]
    pub GetBufferFullnessBits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBufferFullnessBits: usize,
}
#[cfg(feature = "shared")]
pub trait IWMCodecLeakyBucket_Impl: windows_core::IUnknownImpl {
    fn SetBufferSizeBits(&self, ulbuffersize: super::ULONG) -> windows_core::Result<()>;
    fn GetBufferSizeBits(&self) -> windows_core::Result<super::ULONG>;
    fn SetBufferFullnessBits(&self, ulbufferfullness: super::ULONG) -> windows_core::Result<()>;
    fn GetBufferFullnessBits(&self) -> windows_core::Result<super::ULONG>;
}
#[cfg(feature = "shared")]
impl IWMCodecLeakyBucket_Vtbl {
    pub const fn new<Identity: IWMCodecLeakyBucket_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetBufferSizeBits<Identity: IWMCodecLeakyBucket_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulbuffersize: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecLeakyBucket_Impl::SetBufferSizeBits(this, core::mem::transmute_copy(&ulbuffersize)).into()
            }
        }
        unsafe extern "system" fn GetBufferSizeBits<Identity: IWMCodecLeakyBucket_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulbuffersize: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMCodecLeakyBucket_Impl::GetBufferSizeBits(this) {
                    Ok(ok__) => {
                        pulbuffersize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferFullnessBits<Identity: IWMCodecLeakyBucket_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulbufferfullness: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecLeakyBucket_Impl::SetBufferFullnessBits(this, core::mem::transmute_copy(&ulbufferfullness)).into()
            }
        }
        unsafe extern "system" fn GetBufferFullnessBits<Identity: IWMCodecLeakyBucket_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulbufferfullness: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMCodecLeakyBucket_Impl::GetBufferFullnessBits(this) {
                    Ok(ok__) => {
                        pulbufferfullness.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetBufferSizeBits: SetBufferSizeBits::<Identity, OFFSET>,
            GetBufferSizeBits: GetBufferSizeBits::<Identity, OFFSET>,
            SetBufferFullnessBits: SetBufferFullnessBits::<Identity, OFFSET>,
            GetBufferFullnessBits: GetBufferFullnessBits::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCodecLeakyBucket as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMCodecLeakyBucket {}
windows_core::imp::define_interface!(IWMCodecOutputTimestamp, IWMCodecOutputTimestamp_Vtbl, 0xb72adf95_7adc_4a72_bc05_577d8ea6bf68);
windows_core::imp::interface_hierarchy!(IWMCodecOutputTimestamp, windows_core::IUnknown);
impl IWMCodecOutputTimestamp {
    #[cfg(feature = "shared")]
    pub unsafe fn GetNextOutputTime(&self) -> windows_core::Result<super::REFERENCE_TIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextOutputTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecOutputTimestamp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetNextOutputTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::REFERENCE_TIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetNextOutputTime: usize,
}
#[cfg(feature = "shared")]
pub trait IWMCodecOutputTimestamp_Impl: windows_core::IUnknownImpl {
    fn GetNextOutputTime(&self) -> windows_core::Result<super::REFERENCE_TIME>;
}
#[cfg(feature = "shared")]
impl IWMCodecOutputTimestamp_Vtbl {
    pub const fn new<Identity: IWMCodecOutputTimestamp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNextOutputTime<Identity: IWMCodecOutputTimestamp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prttime: *mut super::REFERENCE_TIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMCodecOutputTimestamp_Impl::GetNextOutputTime(this) {
                    Ok(ok__) => {
                        prttime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNextOutputTime: GetNextOutputTime::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCodecOutputTimestamp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMCodecOutputTimestamp {}
windows_core::imp::define_interface!(IWMCodecPrivateData, IWMCodecPrivateData_Vtbl, 0x73f0be8e_57f7_4f01_aa66_9f57340cfe0e);
windows_core::imp::interface_hierarchy!(IWMCodecPrivateData, windows_core::IUnknown);
impl IWMCodecPrivateData {
    #[cfg(all(feature = "mediaobj", feature = "shared"))]
    pub unsafe fn SetPartialOutputType(&self, pmt: *const super::DMO_MEDIA_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPartialOutputType)(windows_core::Interface::as_raw(self), pmt) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPrivateData(&self, pbdata: *mut super::BYTE, pcbdata: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), pbdata as _, pcbdata as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecPrivateData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mediaobj", feature = "shared"))]
    pub SetPartialOutputType: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DMO_MEDIA_TYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mediaobj", feature = "shared")))]
    SetPartialOutputType: usize,
    #[cfg(feature = "shared")]
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPrivateData: usize,
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
pub trait IWMCodecPrivateData_Impl: windows_core::IUnknownImpl {
    fn SetPartialOutputType(&self, pmt: *const super::DMO_MEDIA_TYPE) -> windows_core::Result<()>;
    fn GetPrivateData(&self, pbdata: *mut super::BYTE, pcbdata: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
impl IWMCodecPrivateData_Vtbl {
    pub const fn new<Identity: IWMCodecPrivateData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPartialOutputType<Identity: IWMCodecPrivateData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmt: *const super::DMO_MEDIA_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecPrivateData_Impl::SetPartialOutputType(this, core::mem::transmute_copy(&pmt)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IWMCodecPrivateData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *mut super::BYTE, pcbdata: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecPrivateData_Impl::GetPrivateData(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&pcbdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPartialOutputType: SetPartialOutputType::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCodecPrivateData as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
impl windows_core::RuntimeName for IWMCodecPrivateData {}
windows_core::imp::define_interface!(IWMCodecProps, IWMCodecProps_Vtbl, 0x2573e11a_f01a_4fdd_a98d_63b8e0ba9589);
windows_core::imp::interface_hierarchy!(IWMCodecProps, windows_core::IUnknown);
impl IWMCodecProps {
    #[cfg(all(feature = "mediaobj", feature = "shared"))]
    pub unsafe fn GetFormatProp<P1>(&self, pmt: *const super::DMO_MEDIA_TYPE, pszname: P1, ptype: *mut WMT_PROP_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFormatProp)(windows_core::Interface::as_raw(self), pmt, pszname.param().abi(), ptype as _, pvalue as _, pdwsize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCodecProp<P1>(&self, dwformat: super::DWORD, pszname: P1, ptype: *mut WMT_PROP_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetCodecProp)(windows_core::Interface::as_raw(self), dwformat, pszname.param().abi(), ptype as _, pvalue as _, pdwsize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mediaobj", feature = "shared"))]
    pub GetFormatProp: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DMO_MEDIA_TYPE, windows_core::PCWSTR, *mut WMT_PROP_DATATYPE, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mediaobj", feature = "shared")))]
    GetFormatProp: usize,
    #[cfg(feature = "shared")]
    pub GetCodecProp: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PCWSTR, *mut WMT_PROP_DATATYPE, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCodecProp: usize,
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
pub trait IWMCodecProps_Impl: windows_core::IUnknownImpl {
    fn GetFormatProp(&self, pmt: *const super::DMO_MEDIA_TYPE, pszname: &windows_core::PCWSTR, ptype: *mut WMT_PROP_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetCodecProp(&self, dwformat: super::DWORD, pszname: &windows_core::PCWSTR, ptype: *mut WMT_PROP_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
impl IWMCodecProps_Vtbl {
    pub const fn new<Identity: IWMCodecProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFormatProp<Identity: IWMCodecProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmt: *const super::DMO_MEDIA_TYPE, pszname: windows_core::PCWSTR, ptype: *mut WMT_PROP_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecProps_Impl::GetFormatProp(this, core::mem::transmute_copy(&pmt), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
            }
        }
        unsafe extern "system" fn GetCodecProp<Identity: IWMCodecProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwformat: super::DWORD, pszname: windows_core::PCWSTR, ptype: *mut WMT_PROP_DATATYPE, pvalue: *mut super::BYTE, pdwsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecProps_Impl::GetCodecProp(this, core::mem::transmute_copy(&dwformat), core::mem::transmute(&pszname), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pdwsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFormatProp: GetFormatProp::<Identity, OFFSET>,
            GetCodecProp: GetCodecProp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCodecProps as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
impl windows_core::RuntimeName for IWMCodecProps {}
windows_core::imp::define_interface!(IWMCodecStrings, IWMCodecStrings_Vtbl, 0xa7b2504b_e58a_47fb_958b_cac7165a057d);
windows_core::imp::interface_hierarchy!(IWMCodecStrings, windows_core::IUnknown);
impl IWMCodecStrings {
    #[cfg(all(feature = "mediaobj", feature = "shared"))]
    pub unsafe fn GetName(&self, pmt: *const super::DMO_MEDIA_TYPE, szname: Option<&mut [u16]>, pcchlength: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), pmt, szname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(szname.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), pcchlength as _) }
    }
    #[cfg(all(feature = "mediaobj", feature = "shared"))]
    pub unsafe fn GetDescription(&self, pmt: *const super::DMO_MEDIA_TYPE, szdescription: Option<&mut [u16]>, pcchlength: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), pmt, szdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(szdescription.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), pcchlength as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecStrings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mediaobj", feature = "shared"))]
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DMO_MEDIA_TYPE, super::ULONG, windows_core::PWSTR, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mediaobj", feature = "shared")))]
    GetName: usize,
    #[cfg(all(feature = "mediaobj", feature = "shared"))]
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DMO_MEDIA_TYPE, super::ULONG, windows_core::PWSTR, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mediaobj", feature = "shared")))]
    GetDescription: usize,
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
pub trait IWMCodecStrings_Impl: windows_core::IUnknownImpl {
    fn GetName(&self, pmt: *const super::DMO_MEDIA_TYPE, cchlength: super::ULONG, szname: windows_core::PWSTR, pcchlength: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetDescription(&self, pmt: *const super::DMO_MEDIA_TYPE, cchlength: super::ULONG, szdescription: windows_core::PWSTR, pcchlength: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
impl IWMCodecStrings_Vtbl {
    pub const fn new<Identity: IWMCodecStrings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IWMCodecStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmt: *const super::DMO_MEDIA_TYPE, cchlength: super::ULONG, szname: windows_core::PWSTR, pcchlength: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecStrings_Impl::GetName(this, core::mem::transmute_copy(&pmt), core::mem::transmute_copy(&cchlength), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&pcchlength)).into()
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IWMCodecStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmt: *const super::DMO_MEDIA_TYPE, cchlength: super::ULONG, szdescription: windows_core::PWSTR, pcchlength: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMCodecStrings_Impl::GetDescription(this, core::mem::transmute_copy(&pmt), core::mem::transmute_copy(&cchlength), core::mem::transmute_copy(&szdescription), core::mem::transmute_copy(&pcchlength)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMCodecStrings as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
impl windows_core::RuntimeName for IWMCodecStrings {}
windows_core::imp::define_interface!(IWMColorConvProps, IWMColorConvProps_Vtbl, 0xe6a49e22_c099_421d_aad3_c061fb4ae85b);
windows_core::imp::interface_hierarchy!(IWMColorConvProps, windows_core::IUnknown);
impl IWMColorConvProps {
    #[cfg(feature = "shared")]
    pub unsafe fn SetMode(&self, lmode: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), lmode) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetFullCroppingParam(&self, lsrccropleft: super::LONG, lsrccroptop: super::LONG, ldstcropleft: super::LONG, ldstcroptop: super::LONG, lcropwidth: super::LONG, lcropheight: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFullCroppingParam)(windows_core::Interface::as_raw(self), lsrccropleft, lsrccroptop, ldstcropleft, ldstcroptop, lcropwidth, lcropheight) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMColorConvProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMode: usize,
    #[cfg(feature = "shared")]
    pub SetFullCroppingParam: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::LONG, super::LONG, super::LONG, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetFullCroppingParam: usize,
}
#[cfg(feature = "shared")]
pub trait IWMColorConvProps_Impl: windows_core::IUnknownImpl {
    fn SetMode(&self, lmode: super::LONG) -> windows_core::Result<()>;
    fn SetFullCroppingParam(&self, lsrccropleft: super::LONG, lsrccroptop: super::LONG, ldstcropleft: super::LONG, ldstcroptop: super::LONG, lcropwidth: super::LONG, lcropheight: super::LONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMColorConvProps_Vtbl {
    pub const fn new<Identity: IWMColorConvProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMode<Identity: IWMColorConvProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmode: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMColorConvProps_Impl::SetMode(this, core::mem::transmute_copy(&lmode)).into()
            }
        }
        unsafe extern "system" fn SetFullCroppingParam<Identity: IWMColorConvProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrccropleft: super::LONG, lsrccroptop: super::LONG, ldstcropleft: super::LONG, ldstcroptop: super::LONG, lcropwidth: super::LONG, lcropheight: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMColorConvProps_Impl::SetFullCroppingParam(this, core::mem::transmute_copy(&lsrccropleft), core::mem::transmute_copy(&lsrccroptop), core::mem::transmute_copy(&ldstcropleft), core::mem::transmute_copy(&ldstcroptop), core::mem::transmute_copy(&lcropwidth), core::mem::transmute_copy(&lcropheight)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMode: SetMode::<Identity, OFFSET>,
            SetFullCroppingParam: SetFullCroppingParam::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMColorConvProps as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMColorConvProps {}
windows_core::imp::define_interface!(IWMColorLegalizerProps, IWMColorLegalizerProps_Vtbl, 0x776c93b3_b72d_4508_b6d0_208785f553e7);
windows_core::imp::interface_hierarchy!(IWMColorLegalizerProps, windows_core::IUnknown);
impl IWMColorLegalizerProps {
    #[cfg(feature = "shared")]
    pub unsafe fn SetColorLegalizerQuality(&self, lquality: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColorLegalizerQuality)(windows_core::Interface::as_raw(self), lquality) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMColorLegalizerProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetColorLegalizerQuality: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetColorLegalizerQuality: usize,
}
#[cfg(feature = "shared")]
pub trait IWMColorLegalizerProps_Impl: windows_core::IUnknownImpl {
    fn SetColorLegalizerQuality(&self, lquality: super::LONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMColorLegalizerProps_Vtbl {
    pub const fn new<Identity: IWMColorLegalizerProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetColorLegalizerQuality<Identity: IWMColorLegalizerProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquality: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMColorLegalizerProps_Impl::SetColorLegalizerQuality(this, core::mem::transmute_copy(&lquality)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetColorLegalizerQuality: SetColorLegalizerQuality::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMColorLegalizerProps as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMColorLegalizerProps {}
windows_core::imp::define_interface!(IWMFrameInterpProps, IWMFrameInterpProps_Vtbl, 0x4c06bb9b_626c_4614_8329_cc6a21b93fa0);
windows_core::imp::interface_hierarchy!(IWMFrameInterpProps, windows_core::IUnknown);
impl IWMFrameInterpProps {
    #[cfg(feature = "shared")]
    pub unsafe fn SetFrameRateIn(&self, lframerate: super::LONG, lscale: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFrameRateIn)(windows_core::Interface::as_raw(self), lframerate, lscale) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetFrameRateOut(&self, lframerate: super::LONG, lscale: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFrameRateOut)(windows_core::Interface::as_raw(self), lframerate, lscale) }
    }
    pub unsafe fn SetFrameInterpEnabled(&self, bfienabled: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFrameInterpEnabled)(windows_core::Interface::as_raw(self), bfienabled.into()) }
    }
    pub unsafe fn SetComplexityLevel(&self, icomplexity: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetComplexityLevel)(windows_core::Interface::as_raw(self), icomplexity) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMFrameInterpProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetFrameRateIn: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetFrameRateIn: usize,
    #[cfg(feature = "shared")]
    pub SetFrameRateOut: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetFrameRateOut: usize,
    pub SetFrameInterpEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetComplexityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWMFrameInterpProps_Impl: windows_core::IUnknownImpl {
    fn SetFrameRateIn(&self, lframerate: super::LONG, lscale: super::LONG) -> windows_core::Result<()>;
    fn SetFrameRateOut(&self, lframerate: super::LONG, lscale: super::LONG) -> windows_core::Result<()>;
    fn SetFrameInterpEnabled(&self, bfienabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetComplexityLevel(&self, icomplexity: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMFrameInterpProps_Vtbl {
    pub const fn new<Identity: IWMFrameInterpProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFrameRateIn<Identity: IWMFrameInterpProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lframerate: super::LONG, lscale: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMFrameInterpProps_Impl::SetFrameRateIn(this, core::mem::transmute_copy(&lframerate), core::mem::transmute_copy(&lscale)).into()
            }
        }
        unsafe extern "system" fn SetFrameRateOut<Identity: IWMFrameInterpProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lframerate: super::LONG, lscale: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMFrameInterpProps_Impl::SetFrameRateOut(this, core::mem::transmute_copy(&lframerate), core::mem::transmute_copy(&lscale)).into()
            }
        }
        unsafe extern "system" fn SetFrameInterpEnabled<Identity: IWMFrameInterpProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfienabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMFrameInterpProps_Impl::SetFrameInterpEnabled(this, core::mem::transmute_copy(&bfienabled)).into()
            }
        }
        unsafe extern "system" fn SetComplexityLevel<Identity: IWMFrameInterpProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icomplexity: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMFrameInterpProps_Impl::SetComplexityLevel(this, core::mem::transmute_copy(&icomplexity)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFrameRateIn: SetFrameRateIn::<Identity, OFFSET>,
            SetFrameRateOut: SetFrameRateOut::<Identity, OFFSET>,
            SetFrameInterpEnabled: SetFrameInterpEnabled::<Identity, OFFSET>,
            SetComplexityLevel: SetComplexityLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMFrameInterpProps as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMFrameInterpProps {}
windows_core::imp::define_interface!(IWMInterlaceProps, IWMInterlaceProps_Vtbl, 0x7b12e5d1_bd22_48ea_bc06_98e893221c89);
windows_core::imp::interface_hierarchy!(IWMInterlaceProps, windows_core::IUnknown);
impl IWMInterlaceProps {
    pub unsafe fn SetProcessType(&self, iprocesstype: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProcessType)(windows_core::Interface::as_raw(self), iprocesstype) }
    }
    pub unsafe fn SetInitInverseTeleCinePattern(&self, iinitpattern: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInitInverseTeleCinePattern)(windows_core::Interface::as_raw(self), iinitpattern) }
    }
    pub unsafe fn SetLastFrame(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLastFrame)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMInterlaceProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetProcessType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetInitInverseTeleCinePattern: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetLastFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMInterlaceProps_Impl: windows_core::IUnknownImpl {
    fn SetProcessType(&self, iprocesstype: i32) -> windows_core::Result<()>;
    fn SetInitInverseTeleCinePattern(&self, iinitpattern: i32) -> windows_core::Result<()>;
    fn SetLastFrame(&self) -> windows_core::Result<()>;
}
impl IWMInterlaceProps_Vtbl {
    pub const fn new<Identity: IWMInterlaceProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProcessType<Identity: IWMInterlaceProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprocesstype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMInterlaceProps_Impl::SetProcessType(this, core::mem::transmute_copy(&iprocesstype)).into()
            }
        }
        unsafe extern "system" fn SetInitInverseTeleCinePattern<Identity: IWMInterlaceProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iinitpattern: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMInterlaceProps_Impl::SetInitInverseTeleCinePattern(this, core::mem::transmute_copy(&iinitpattern)).into()
            }
        }
        unsafe extern "system" fn SetLastFrame<Identity: IWMInterlaceProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMInterlaceProps_Impl::SetLastFrame(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetProcessType: SetProcessType::<Identity, OFFSET>,
            SetInitInverseTeleCinePattern: SetInitInverseTeleCinePattern::<Identity, OFFSET>,
            SetLastFrame: SetLastFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMInterlaceProps as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMInterlaceProps {}
windows_core::imp::define_interface!(IWMResamplerProps, IWMResamplerProps_Vtbl, 0xe7e9984f_f09f_4da4_903f_6e2e0efe56b5);
windows_core::imp::interface_hierarchy!(IWMResamplerProps, windows_core::IUnknown);
impl IWMResamplerProps {
    #[cfg(feature = "shared")]
    pub unsafe fn SetHalfFilterLength(&self, lhalffilterlen: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHalfFilterLength)(windows_core::Interface::as_raw(self), lhalffilterlen) }
    }
    pub unsafe fn SetUserChannelMtx(&self, userchannelmtx: *const ChMtxType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUserChannelMtx)(windows_core::Interface::as_raw(self), userchannelmtx) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMResamplerProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetHalfFilterLength: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetHalfFilterLength: usize,
    pub SetUserChannelMtx: unsafe extern "system" fn(*mut core::ffi::c_void, *const ChMtxType) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWMResamplerProps_Impl: windows_core::IUnknownImpl {
    fn SetHalfFilterLength(&self, lhalffilterlen: super::LONG) -> windows_core::Result<()>;
    fn SetUserChannelMtx(&self, userchannelmtx: *const ChMtxType) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMResamplerProps_Vtbl {
    pub const fn new<Identity: IWMResamplerProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHalfFilterLength<Identity: IWMResamplerProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhalffilterlen: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMResamplerProps_Impl::SetHalfFilterLength(this, core::mem::transmute_copy(&lhalffilterlen)).into()
            }
        }
        unsafe extern "system" fn SetUserChannelMtx<Identity: IWMResamplerProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, userchannelmtx: *const ChMtxType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMResamplerProps_Impl::SetUserChannelMtx(this, core::mem::transmute_copy(&userchannelmtx)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHalfFilterLength: SetHalfFilterLength::<Identity, OFFSET>,
            SetUserChannelMtx: SetUserChannelMtx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMResamplerProps as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMResamplerProps {}
windows_core::imp::define_interface!(IWMResizerProps, IWMResizerProps_Vtbl, 0x57665d4c_0414_4faa_905b_10e546f81c33);
windows_core::imp::interface_hierarchy!(IWMResizerProps, windows_core::IUnknown);
impl IWMResizerProps {
    #[cfg(feature = "shared")]
    pub unsafe fn SetResizerQuality(&self, lquality: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetResizerQuality)(windows_core::Interface::as_raw(self), lquality) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetInterlaceMode(&self, lmode: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInterlaceMode)(windows_core::Interface::as_raw(self), lmode) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetClipRegion(&self, lcliporixsrc: super::LONG, lcliporiysrc: super::LONG, lclipwidthsrc: super::LONG, lclipheightsrc: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClipRegion)(windows_core::Interface::as_raw(self), lcliporixsrc, lcliporiysrc, lclipwidthsrc, lclipheightsrc) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetFullCropRegion(&self, lcliporixsrc: super::LONG, lcliporiysrc: super::LONG, lclipwidthsrc: super::LONG, lclipheightsrc: super::LONG, lcliporixdst: super::LONG, lcliporiydst: super::LONG, lclipwidthdst: super::LONG, lclipheightdst: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFullCropRegion)(windows_core::Interface::as_raw(self), lcliporixsrc, lcliporiysrc, lclipwidthsrc, lclipheightsrc, lcliporixdst, lcliporiydst, lclipwidthdst, lclipheightdst) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFullCropRegion(&self, lcliporixsrc: *mut super::LONG, lcliporiysrc: *mut super::LONG, lclipwidthsrc: *mut super::LONG, lclipheightsrc: *mut super::LONG, lcliporixdst: *mut super::LONG, lcliporiydst: *mut super::LONG, lclipwidthdst: *mut super::LONG, lclipheightdst: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFullCropRegion)(windows_core::Interface::as_raw(self), lcliporixsrc as _, lcliporiysrc as _, lclipwidthsrc as _, lclipheightsrc as _, lcliporixdst as _, lcliporiydst as _, lclipwidthdst as _, lclipheightdst as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMResizerProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetResizerQuality: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetResizerQuality: usize,
    #[cfg(feature = "shared")]
    pub SetInterlaceMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetInterlaceMode: usize,
    #[cfg(feature = "shared")]
    pub SetClipRegion: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::LONG, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetClipRegion: usize,
    #[cfg(feature = "shared")]
    pub SetFullCropRegion: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::LONG, super::LONG, super::LONG, super::LONG, super::LONG, super::LONG, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetFullCropRegion: usize,
    #[cfg(feature = "shared")]
    pub GetFullCropRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG, *mut super::LONG, *mut super::LONG, *mut super::LONG, *mut super::LONG, *mut super::LONG, *mut super::LONG, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFullCropRegion: usize,
}
#[cfg(feature = "shared")]
pub trait IWMResizerProps_Impl: windows_core::IUnknownImpl {
    fn SetResizerQuality(&self, lquality: super::LONG) -> windows_core::Result<()>;
    fn SetInterlaceMode(&self, lmode: super::LONG) -> windows_core::Result<()>;
    fn SetClipRegion(&self, lcliporixsrc: super::LONG, lcliporiysrc: super::LONG, lclipwidthsrc: super::LONG, lclipheightsrc: super::LONG) -> windows_core::Result<()>;
    fn SetFullCropRegion(&self, lcliporixsrc: super::LONG, lcliporiysrc: super::LONG, lclipwidthsrc: super::LONG, lclipheightsrc: super::LONG, lcliporixdst: super::LONG, lcliporiydst: super::LONG, lclipwidthdst: super::LONG, lclipheightdst: super::LONG) -> windows_core::Result<()>;
    fn GetFullCropRegion(&self, lcliporixsrc: *mut super::LONG, lcliporiysrc: *mut super::LONG, lclipwidthsrc: *mut super::LONG, lclipheightsrc: *mut super::LONG, lcliporixdst: *mut super::LONG, lcliporiydst: *mut super::LONG, lclipwidthdst: *mut super::LONG, lclipheightdst: *mut super::LONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWMResizerProps_Vtbl {
    pub const fn new<Identity: IWMResizerProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetResizerQuality<Identity: IWMResizerProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquality: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMResizerProps_Impl::SetResizerQuality(this, core::mem::transmute_copy(&lquality)).into()
            }
        }
        unsafe extern "system" fn SetInterlaceMode<Identity: IWMResizerProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmode: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMResizerProps_Impl::SetInterlaceMode(this, core::mem::transmute_copy(&lmode)).into()
            }
        }
        unsafe extern "system" fn SetClipRegion<Identity: IWMResizerProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcliporixsrc: super::LONG, lcliporiysrc: super::LONG, lclipwidthsrc: super::LONG, lclipheightsrc: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMResizerProps_Impl::SetClipRegion(this, core::mem::transmute_copy(&lcliporixsrc), core::mem::transmute_copy(&lcliporiysrc), core::mem::transmute_copy(&lclipwidthsrc), core::mem::transmute_copy(&lclipheightsrc)).into()
            }
        }
        unsafe extern "system" fn SetFullCropRegion<Identity: IWMResizerProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcliporixsrc: super::LONG, lcliporiysrc: super::LONG, lclipwidthsrc: super::LONG, lclipheightsrc: super::LONG, lcliporixdst: super::LONG, lcliporiydst: super::LONG, lclipwidthdst: super::LONG, lclipheightdst: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMResizerProps_Impl::SetFullCropRegion(this, core::mem::transmute_copy(&lcliporixsrc), core::mem::transmute_copy(&lcliporiysrc), core::mem::transmute_copy(&lclipwidthsrc), core::mem::transmute_copy(&lclipheightsrc), core::mem::transmute_copy(&lcliporixdst), core::mem::transmute_copy(&lcliporiydst), core::mem::transmute_copy(&lclipwidthdst), core::mem::transmute_copy(&lclipheightdst)).into()
            }
        }
        unsafe extern "system" fn GetFullCropRegion<Identity: IWMResizerProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcliporixsrc: *mut super::LONG, lcliporiysrc: *mut super::LONG, lclipwidthsrc: *mut super::LONG, lclipheightsrc: *mut super::LONG, lcliporixdst: *mut super::LONG, lcliporiydst: *mut super::LONG, lclipwidthdst: *mut super::LONG, lclipheightdst: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMResizerProps_Impl::GetFullCropRegion(this, core::mem::transmute_copy(&lcliporixsrc), core::mem::transmute_copy(&lcliporiysrc), core::mem::transmute_copy(&lclipwidthsrc), core::mem::transmute_copy(&lclipheightsrc), core::mem::transmute_copy(&lcliporixdst), core::mem::transmute_copy(&lcliporiydst), core::mem::transmute_copy(&lclipwidthdst), core::mem::transmute_copy(&lclipheightdst)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetResizerQuality: SetResizerQuality::<Identity, OFFSET>,
            SetInterlaceMode: SetInterlaceMode::<Identity, OFFSET>,
            SetClipRegion: SetClipRegion::<Identity, OFFSET>,
            SetFullCropRegion: SetFullCropRegion::<Identity, OFFSET>,
            GetFullCropRegion: GetFullCropRegion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMResizerProps as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMResizerProps {}
windows_core::imp::define_interface!(IWMSampleExtensionSupport, IWMSampleExtensionSupport_Vtbl, 0x9bca9884_0604_4c2a_87da_793ff4d586c3);
windows_core::imp::interface_hierarchy!(IWMSampleExtensionSupport, windows_core::IUnknown);
impl IWMSampleExtensionSupport {
    pub unsafe fn SetUseSampleExtensions(&self, fuseextensions: windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUseSampleExtensions)(windows_core::Interface::as_raw(self), fuseextensions) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSampleExtensionSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetUseSampleExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWMSampleExtensionSupport_Impl: windows_core::IUnknownImpl {
    fn SetUseSampleExtensions(&self, fuseextensions: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IWMSampleExtensionSupport_Vtbl {
    pub const fn new<Identity: IWMSampleExtensionSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetUseSampleExtensions<Identity: IWMSampleExtensionSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuseextensions: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMSampleExtensionSupport_Impl::SetUseSampleExtensions(this, core::mem::transmute_copy(&fuseextensions)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetUseSampleExtensions: SetUseSampleExtensions::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMSampleExtensionSupport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMSampleExtensionSupport {}
windows_core::imp::define_interface!(IWMValidate, IWMValidate_Vtbl, 0xcee3def2_3808_414d_be66_fafd472210bc);
windows_core::imp::interface_hierarchy!(IWMValidate, windows_core::IUnknown);
impl IWMValidate {
    pub unsafe fn SetIdentifier(&self, guidvalidationid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIdentifier)(windows_core::Interface::as_raw(self), guidvalidationid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMValidate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IWMValidate_Impl: windows_core::IUnknownImpl {
    fn SetIdentifier(&self, guidvalidationid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl IWMValidate_Vtbl {
    pub const fn new<Identity: IWMValidate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIdentifier<Identity: IWMValidate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidvalidationid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMValidate_Impl::SetIdentifier(this, core::mem::transmute(&guidvalidationid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetIdentifier: SetIdentifier::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMValidate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMValidate {}
windows_core::imp::define_interface!(IWMVideoDecoderHurryup, IWMVideoDecoderHurryup_Vtbl, 0x352bb3bd_2d4d_4323_9e71_dcdcfbd53ca6);
windows_core::imp::interface_hierarchy!(IWMVideoDecoderHurryup, windows_core::IUnknown);
impl IWMVideoDecoderHurryup {
    #[cfg(feature = "shared")]
    pub unsafe fn SetHurryup(&self, lhurryup: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHurryup)(windows_core::Interface::as_raw(self), lhurryup) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetHurryup(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHurryup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMVideoDecoderHurryup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetHurryup: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetHurryup: usize,
    #[cfg(feature = "shared")]
    pub GetHurryup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetHurryup: usize,
}
#[cfg(feature = "shared")]
pub trait IWMVideoDecoderHurryup_Impl: windows_core::IUnknownImpl {
    fn SetHurryup(&self, lhurryup: super::LONG) -> windows_core::Result<()>;
    fn GetHurryup(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(feature = "shared")]
impl IWMVideoDecoderHurryup_Vtbl {
    pub const fn new<Identity: IWMVideoDecoderHurryup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHurryup<Identity: IWMVideoDecoderHurryup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhurryup: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMVideoDecoderHurryup_Impl::SetHurryup(this, core::mem::transmute_copy(&lhurryup)).into()
            }
        }
        unsafe extern "system" fn GetHurryup<Identity: IWMVideoDecoderHurryup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhurryup: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMVideoDecoderHurryup_Impl::GetHurryup(this) {
                    Ok(ok__) => {
                        plhurryup.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHurryup: SetHurryup::<Identity, OFFSET>,
            GetHurryup: GetHurryup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMVideoDecoderHurryup as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWMVideoDecoderHurryup {}
windows_core::imp::define_interface!(IWMVideoDecoderReconBuffer, IWMVideoDecoderReconBuffer_Vtbl, 0x45bda2ac_88e2_4923_98ba_3949080711a3);
windows_core::imp::interface_hierarchy!(IWMVideoDecoderReconBuffer, windows_core::IUnknown);
impl IWMVideoDecoderReconBuffer {
    #[cfg(feature = "shared")]
    pub unsafe fn GetReconstructedVideoFrameSize(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReconstructedVideoFrameSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mediaobj")]
    pub unsafe fn GetReconstructedVideoFrame<P0>(&self, pbuf: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMediaBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetReconstructedVideoFrame)(windows_core::Interface::as_raw(self), pbuf.param().abi()) }
    }
    #[cfg(feature = "mediaobj")]
    pub unsafe fn SetReconstructedVideoFrame<P0>(&self, pbuf: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMediaBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetReconstructedVideoFrame)(windows_core::Interface::as_raw(self), pbuf.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMVideoDecoderReconBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetReconstructedVideoFrameSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetReconstructedVideoFrameSize: usize,
    #[cfg(feature = "mediaobj")]
    pub GetReconstructedVideoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mediaobj"))]
    GetReconstructedVideoFrame: usize,
    #[cfg(feature = "mediaobj")]
    pub SetReconstructedVideoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mediaobj"))]
    SetReconstructedVideoFrame: usize,
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
pub trait IWMVideoDecoderReconBuffer_Impl: windows_core::IUnknownImpl {
    fn GetReconstructedVideoFrameSize(&self) -> windows_core::Result<super::DWORD>;
    fn GetReconstructedVideoFrame(&self, pbuf: windows_core::Ref<super::IMediaBuffer>) -> windows_core::Result<()>;
    fn SetReconstructedVideoFrame(&self, pbuf: windows_core::Ref<super::IMediaBuffer>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
impl IWMVideoDecoderReconBuffer_Vtbl {
    pub const fn new<Identity: IWMVideoDecoderReconBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetReconstructedVideoFrameSize<Identity: IWMVideoDecoderReconBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMVideoDecoderReconBuffer_Impl::GetReconstructedVideoFrameSize(this) {
                    Ok(ok__) => {
                        pdwsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReconstructedVideoFrame<Identity: IWMVideoDecoderReconBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuf: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMVideoDecoderReconBuffer_Impl::GetReconstructedVideoFrame(this, core::mem::transmute_copy(&pbuf)).into()
            }
        }
        unsafe extern "system" fn SetReconstructedVideoFrame<Identity: IWMVideoDecoderReconBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuf: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMVideoDecoderReconBuffer_Impl::SetReconstructedVideoFrame(this, core::mem::transmute_copy(&pbuf)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetReconstructedVideoFrameSize: GetReconstructedVideoFrameSize::<Identity, OFFSET>,
            GetReconstructedVideoFrame: GetReconstructedVideoFrame::<Identity, OFFSET>,
            SetReconstructedVideoFrame: SetReconstructedVideoFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMVideoDecoderReconBuffer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mediaobj", feature = "shared"))]
impl windows_core::RuntimeName for IWMVideoDecoderReconBuffer {}
windows_core::imp::define_interface!(IWMVideoForceKeyFrame, IWMVideoForceKeyFrame_Vtbl, 0x9f8496be_5b9a_41b9_a9e8_f21cd80596c2);
windows_core::imp::interface_hierarchy!(IWMVideoForceKeyFrame, windows_core::IUnknown);
impl IWMVideoForceKeyFrame {
    pub unsafe fn SetKeyFrame(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetKeyFrame)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMVideoForceKeyFrame_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetKeyFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMVideoForceKeyFrame_Impl: windows_core::IUnknownImpl {
    fn SetKeyFrame(&self) -> windows_core::Result<()>;
}
impl IWMVideoForceKeyFrame_Vtbl {
    pub const fn new<Identity: IWMVideoForceKeyFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetKeyFrame<Identity: IWMVideoForceKeyFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMVideoForceKeyFrame_Impl::SetKeyFrame(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetKeyFrame: SetKeyFrame::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMVideoForceKeyFrame as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMVideoForceKeyFrame {}
pub const MEDIASUBTYPE_AVC1: windows_core::GUID = windows_core::GUID::from_u128(0x31435641_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_DOLBY_DDPLUS: windows_core::GUID = windows_core::GUID::from_u128(0xa7fb87af_2d02_42fb_a4d4_05cd93843bdd);
pub const MEDIASUBTYPE_DOLBY_TRUEHD: windows_core::GUID = windows_core::GUID::from_u128(0xeb27cec4_163e_4ca3_8b74_8e25f91b517e);
pub const MEDIASUBTYPE_DTS2: windows_core::GUID = windows_core::GUID::from_u128(0x00002001_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_DTS_HD: windows_core::GUID = windows_core::GUID::from_u128(0xa2e58eb7_0fa9_48bb_a40c_fa0e156d0645);
pub const MEDIASUBTYPE_DTS_HD_HRA: windows_core::GUID = windows_core::GUID::from_u128(0xa61ac364_ad0e_4744_89ff_213ce0df8804);
pub const MEDIASUBTYPE_DVM: windows_core::GUID = windows_core::GUID::from_u128(0x00002000_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_I420: windows_core::GUID = windows_core::GUID::from_u128(0x30323449_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_M4S2: windows_core::GUID = windows_core::GUID::from_u128(0x3253344d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MP42: windows_core::GUID = windows_core::GUID::from_u128(0x3234504d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MP43: windows_core::GUID = windows_core::GUID::from_u128(0x3334504d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MP4S: windows_core::GUID = windows_core::GUID::from_u128(0x5334504d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MPEG_ADTS_AAC: windows_core::GUID = windows_core::GUID::from_u128(0x00001600_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MPEG_HEAAC: windows_core::GUID = windows_core::GUID::from_u128(0x00001610_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MPEG_LOAS: windows_core::GUID = windows_core::GUID::from_u128(0x00001602_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MPEG_RAW_AAC: windows_core::GUID = windows_core::GUID::from_u128(0x00001601_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MPG4: windows_core::GUID = windows_core::GUID::from_u128(0x3447504d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MSAUDIO1: windows_core::GUID = windows_core::GUID::from_u128(0x00000160_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MSS1: windows_core::GUID = windows_core::GUID::from_u128(0x3153534d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_MSS2: windows_core::GUID = windows_core::GUID::from_u128(0x3253534d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_NOKIA_MPEG_ADTS_AAC: windows_core::GUID = windows_core::GUID::from_u128(0x00001608_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_NOKIA_MPEG_RAW_AAC: windows_core::GUID = windows_core::GUID::from_u128(0x00001609_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_RAW_AAC1: windows_core::GUID = windows_core::GUID::from_u128(0x000000ff_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_V216: windows_core::GUID = windows_core::GUID::from_u128(0x36313256_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_V410: windows_core::GUID = windows_core::GUID::from_u128(0x30313456_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_VODAFONE_MPEG_ADTS_AAC: windows_core::GUID = windows_core::GUID::from_u128(0x0000160a_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_VODAFONE_MPEG_RAW_AAC: windows_core::GUID = windows_core::GUID::from_u128(0x0000160b_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMASPDIF: windows_core::GUID = windows_core::GUID::from_u128(0x00000164_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMAUDIO2: windows_core::GUID = windows_core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMAUDIO3: windows_core::GUID = windows_core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMAUDIO4: windows_core::GUID = windows_core::GUID::from_u128(0x00000168_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMAUDIO_LOSSLESS: windows_core::GUID = windows_core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMV1: windows_core::GUID = windows_core::GUID::from_u128(0x31564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMV2: windows_core::GUID = windows_core::GUID::from_u128(0x32564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMV3: windows_core::GUID = windows_core::GUID::from_u128(0x33564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMVA: windows_core::GUID = windows_core::GUID::from_u128(0x41564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMVB: windows_core::GUID = windows_core::GUID::from_u128(0x42564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMVP: windows_core::GUID = windows_core::GUID::from_u128(0x50564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WMVR: windows_core::GUID = windows_core::GUID::from_u128(0x52564d57_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WVC1: windows_core::GUID = windows_core::GUID::from_u128(0x31435657_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_WVP2: windows_core::GUID = windows_core::GUID::from_u128(0x32505657_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_X264: windows_core::GUID = windows_core::GUID::from_u128(0x34363258_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_Y41T: windows_core::GUID = windows_core::GUID::from_u128(0x54313459_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_Y42T: windows_core::GUID = windows_core::GUID::from_u128(0x54323459_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_h264: windows_core::GUID = windows_core::GUID::from_u128(0x34363268_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_m4s2: windows_core::GUID = windows_core::GUID::from_u128(0x3273346d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_mp42: windows_core::GUID = windows_core::GUID::from_u128(0x3234706d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_mp43: windows_core::GUID = windows_core::GUID::from_u128(0x3334706d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_mp4s: windows_core::GUID = windows_core::GUID::from_u128(0x7334706d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_mpg4: windows_core::GUID = windows_core::GUID::from_u128(0x3467706d_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_v210: windows_core::GUID = windows_core::GUID::from_u128(0x30313276_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmv1: windows_core::GUID = windows_core::GUID::from_u128(0x31766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmv2: windows_core::GUID = windows_core::GUID::from_u128(0x32766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmv3: windows_core::GUID = windows_core::GUID::from_u128(0x33766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmva: windows_core::GUID = windows_core::GUID::from_u128(0x61766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmvb: windows_core::GUID = windows_core::GUID::from_u128(0x62766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmvp: windows_core::GUID = windows_core::GUID::from_u128(0x70766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wmvr: windows_core::GUID = windows_core::GUID::from_u128(0x72766d77_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wvc1: windows_core::GUID = windows_core::GUID::from_u128(0x31637677_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_wvp2: windows_core::GUID = windows_core::GUID::from_u128(0x32707677_0000_0010_8000_00aa00389b71);
pub const MEDIASUBTYPE_x264: windows_core::GUID = windows_core::GUID::from_u128(0x34363278_0000_0010_8000_00aa00389b71);
pub const MFAMRNBByteStreamHandler: windows_core::GUID = windows_core::GUID::from_u128(0xefe6208a_0a2c_49fa_8a01_3768b559b6da);
pub const MFAMRNBSinkClassFactory: windows_core::GUID = windows_core::GUID::from_u128(0xb0271158_70d2_4c5b_9f94_76f549d90fdf);
pub const MFFLACBytestreamHandler: windows_core::GUID = windows_core::GUID::from_u128(0x0e41cfb8_0506_40f4_a516_77cc23642d91);
pub const MFFLACSinkClassFactory: windows_core::GUID = windows_core::GUID::from_u128(0x7d39c56f_6075_47c9_9bae_8cf9e531b5f5);
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_ADAPTIVERESOLUTION: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 86 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_ASFOVERHEADPERFRAME: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0eac7502_1957_4beb_914d_885f85e75436), pid: 18 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_AUVRHP_DOHRTFREVERB: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x30bbfebf_24b4_4198_89ba_ad11a2acd601), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_AUVRHP_DOPOSTREVERB: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x30bbfebf_24b4_4198_89ba_ad11a2acd601), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_AUVRHP_LFWEIGHT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x30bbfebf_24b4_4198_89ba_ad11a2acd601), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_AUVRHP_POSTREVERB_LEN: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x30bbfebf_24b4_4198_89ba_ad11a2acd601), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_AUVRHP_POSTREVERB_START: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x30bbfebf_24b4_4198_89ba_ad11a2acd601), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_AUVRHP_ROOMMODEL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x73ae880e_8258_4e57_b85f_7daa6b7d5ef0), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_AUVRHP_SKIPHRTFREVERB: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x30bbfebf_24b4_4198_89ba_ad11a2acd601), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_AUVRHP_SKIPPOSTREVERB: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x30bbfebf_24b4_4198_89ba_ad11a2acd601), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_AVDecVideoSWPowerLevel: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfb5d2347_4dd8_4509_aed0_db5fa9aa93f4), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_AVGCONSTRAINED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_AVGFRAMERATE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x41d700d6_95b1_4e3f_b759_2d66d8c7ada2), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_BASSMGMT_BIGROOM: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xc816a1a7_a119_48a5_9ad2_85451f4b5a2e), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_BASSMGMT_CROSSOVER_FREQ: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x61e8acb9_f04f_4f40_a65f_8f49fab3ba10), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_BASSMGMT_INVERT_SUB: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb1103003_c191_4275_9fa0_8c282c724bce), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_BASSMGMT_NO_SUB: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5c3fd32e_0d40_4e2d_99fb_c91e96420be7), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_BASSMGMT_SPKRBASSCONFIG: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7bfd170d_4770_4dc5_924d_0b7b252ee918), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_BASS_BOOST_AMOUNT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xae7f0b2a_96fc_493a_9247_a019f1f701e1), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_BAVG: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x10174e76_e0ca_4a39_948c_8510c2323276), pid: 23 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_BDELTAQP: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 50 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_BMAX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xff365211_21b6_4134_ab7c_52393a8f80f6), pid: 25 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_BUFFERFULLNESSINFIRSTBYTE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb69dc3c5_64c4_4757_99cb_5d580fd5659e), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CHECKDATACONSISTENCY2P: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CLIP_HEIGHT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x716fe5c8_755c_482f_8df3_b31d5359f0d6), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CLIP_WIDTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x716fe5c8_755c_482f_8df3_b31d5359f0d6), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CLIP_XORIG: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x716fe5c8_755c_482f_8df3_b31d5359f0d6), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CLIP_YORIG: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x716fe5c8_755c_482f_8df3_b31d5359f0d6), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CLOSEDENTRYPOINT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 63 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CLUSTERDETECTOR_MAXCLUSTERDURATION: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb79a666d_8a9d_463c_9d97_e1b10045c13a), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CLUSTERDETECTOR_MAXCLUSTERS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb79a666d_8a9d_463c_9d97_e1b10045c13a), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CLUSTERDETECTOR_MINCLUSTERDURATION: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb79a666d_8a9d_463c_9d97_e1b10045c13a), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CODEDFRAMES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x62872b55_fe0e_4930_a6d2_ccaa37e8f535), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CODEDNONZEROFRAMES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7adf5b69_1e3f_42d3_991d_f81e0cebe93e), pid: 30 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORCONV_DSTLEFT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdc9100be_1228_416c_9948_6f38f479654f), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORCONV_DSTTOP: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdc9100be_1228_416c_9948_6f38f479654f), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORCONV_HEIGHT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdc9100be_1228_416c_9948_6f38f479654f), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORCONV_MODE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdc9100be_1228_416c_9948_6f38f479654f), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORCONV_SRCLEFT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdc9100be_1228_416c_9948_6f38f479654f), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORCONV_SRCTOP: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdc9100be_1228_416c_9948_6f38f479654f), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORCONV_WIDTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xdc9100be_1228_416c_9948_6f38f479654f), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORLEGALIZER_COMPLEXITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xadd0e6c1_cc30_475d_9e5a_f1b19f587be0), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORLEGALIZER_COMPLEXITYEX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xadd0e6c1_cc30_475d_9e5a_f1b19f587be0), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORLEGALIZER_COMPLEXITYLIVE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xadd0e6c1_cc30_475d_9e5a_f1b19f587be0), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORLEGALIZER_COMPLEXITYMAX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xadd0e6c1_cc30_475d_9e5a_f1b19f587be0), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORLEGALIZER_COMPLEXITYOFFLINE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xadd0e6c1_cc30_475d_9e5a_f1b19f587be0), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLORLEGALIZER_bSVideo: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xadd0e6c1_cc30_475d_9e5a_f1b19f587be0), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLOR_BRIGHTNESS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x174fb0ec_2695_476c_88aa_d2b41ce75e67), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLOR_CONTRAST: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x174fb0ec_2695_476c_88aa_d2b41ce75e67), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLOR_HUE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x174fb0ec_2695_476c_88aa_d2b41ce75e67), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COLOR_SATURATION: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x174fb0ec_2695_476c_88aa_d2b41ce75e67), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COMPLEXITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x44fa08c7_92f5_45dc_8376_8d1d324c652a), pid: 16 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COMPLEXITYEX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd6e48f93_fd47_47a3_9262_8aefb5530332), pid: 17 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_COMPRESSIONOPTIMIZATIONTYPE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 67 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CONSTRAINDECLATENCY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CONSTRAINENCCOMPLEXITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CONSTRAINENCLATENCY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CONSTRAIN_ENUMERATED_VBRQUALITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 26 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CONV_INPUTFRAMERATE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x52f8d29b_2e76_43f7_a4f6_1717904e35df), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CONV_OUTPUTFRAMERATE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x52f8d29b_2e76_43f7_a4f6_1717904e35df), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CONV_REVERSEPLAYBACK: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x52f8d29b_2e76_43f7_a4f6_1717904e35df), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CORR_BASS_MANAGEMENT_MODE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1864a4e0_efc1_45e6_a675_5786cbf3b9f0), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CORR_HEADPHONE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x445f3559_b43f_4b67_b0f8_32b67cf94b48), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CORR_LOUDNESS_EQUALIZATION_ON: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xfc52a749_4be9_4510_896e_966ba6525980), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CORR_MULTICHANNEL_MODE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1b5c2483_0839_4523_ba87_95f89d27bd8c), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CORR_NORMALIZATION_GAIN: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd61b266c_5aee_456b_8424_7225477dae77), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CORR_ROOM_CORRECTION_ON: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x01fb17e3_796c_4451_8163_68cdc1321a60), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CORR_SPKRMASK: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd328d8fb_d49f_4aa9_b721_e171e93ad563), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_CRISP: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x2985f772_3af2_4d15_8cfa_8a962ff32040), pid: 11 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DATARATE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe0db0807_8003_4880_ac11_61b73f33dc60), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DECODERCOMPLEXITYPROFILE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xc0d912d6_14da_4d31_8d83_d108915e8d77), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DECODERCOMPLEXITYREQUESTED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb0d7d4a7_422b_44c3_97b4_b9760cceeea9), pid: 28 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DECODER_DEINTERLACING: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00c6281f_4be6_4e44_9ed8_9ec5422316e4), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DEINTERLACE_DETELECINE_FLAG: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6141f4c3_d3a2_48a9_96ac_2f0014176c26), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DEINTERLACE_EDGETHRESHOLD: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6141f4c3_d3a2_48a9_96ac_2f0014176c26), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DEINTERLACE_LASTFRAME: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6141f4c3_d3a2_48a9_96ac_2f0014176c26), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DEINTERLACE_PROCESSTYPE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6141f4c3_d3a2_48a9_96ac_2f0014176c26), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DEINTERLACE_SMOOTHLEVEL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6141f4c3_d3a2_48a9_96ac_2f0014176c26), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DEINTERLACE_TELECINEPATTERN: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6141f4c3_d3a2_48a9_96ac_2f0014176c26), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DELTAMVRANGEINDEX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 47 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DENOISEOPTION: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 38 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DENOISE_CACHEFRAMES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7213c6ef_cdd4_4d09_a89e_f3ebebe5f565), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DENOISE_CAUSAL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7213c6ef_cdd4_4d09_a89e_f3ebebe5f565), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DENOISE_FILTER: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7213c6ef_cdd4_4d09_a89e_f3ebebe5f565), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DENOISE_ITERATIVE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7213c6ef_cdd4_4d09_a89e_f3ebebe5f565), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DENOISE_PARA1: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7213c6ef_cdd4_4d09_a89e_f3ebebe5f565), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DENOISE_PARA2: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7213c6ef_cdd4_4d09_a89e_f3ebebe5f565), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DENOISE_PARA3: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7213c6ef_cdd4_4d09_a89e_f3ebebe5f565), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DENOISE_PARA4: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7213c6ef_cdd4_4d09_a89e_f3ebebe5f565), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DENOISE_PARA5: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7213c6ef_cdd4_4d09_a89e_f3ebebe5f565), pid: 16 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DENOISE_PROCFRAMES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7213c6ef_cdd4_4d09_a89e_f3ebebe5f565), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DESIRED_VBRQUALITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 25 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DQUANTOPTION: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 78 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DQUANTSTRENGTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 79 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DROPPEDFRAMES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xbdb5afd3_4027_4882_806c_4162e4a7a3f0), pid: 29 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DVDEC_SIZE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x75f2421a_e73a_45b3_aef0_913c66846461), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DVENC_FORMAT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf449a927_7b22_46ef_b2e7_4d2b728b699c), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DXVA_ENABLED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x58e28605_1d51_48ed_a3eb_0f9baf785fbd), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYNCOMPLEXLEVEL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 82 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYNENCMODE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 84 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYN_ALLOW_NOISESUB: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 19 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYN_ALLOW_PCMRANGELIMITING: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 27 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYN_BANDTRUNCATION: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 10 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYN_BANDTRUNC_BWCEIL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 14 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYN_BANDTRUNC_BWFLOOR: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 13 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYN_BANDTRUNC_QCEIL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 12 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYN_BANDTRUNC_QFLOOR: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 11 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYN_SIMPLEMASK: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 15 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYN_STEREO_PREPROC: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 16 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYN_VBR_BAVG: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 18 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_DYN_VBR_RAVG: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_Decoder_MaxNumPCMSamplesWithPaddedSilence: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xc678ba85_1212_43da_5ac3_e748b92449ec), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_ENCCOMPLEXITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_ENCODERCOMPLEXITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 71 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_ENDOFPASS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb2030f2a_8bbc_46f8_a64b_a98ff7fcf02a), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_ENHANCED_WMA: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 20 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FACEDETECTOR_SKIPFRAMES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe1a124a6_4fa8_4ba5_a2d8_dc34536f7426), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FI_ALLOWED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x305bca55_1e5b_428e_a94c_65b94d0264ed), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FI_COMPLEXITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x305bca55_1e5b_428e_a94c_65b94d0264ed), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FI_ENABLED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe020c4ca_3bdd_40ec_bcf4_4039b1450eb8), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FI_FRAMERATE_SCAL_DST: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x305bca55_1e5b_428e_a94c_65b94d0264ed), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FI_FRAMERATE_SCAL_SRC: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x305bca55_1e5b_428e_a94c_65b94d0264ed), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FI_FRAMERATE_VALU_DST: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x305bca55_1e5b_428e_a94c_65b94d0264ed), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FI_FRAMERATE_VALU_SRC: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x305bca55_1e5b_428e_a94c_65b94d0264ed), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FI_SUPPORTED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5ce18788_b992_49a7_a4f1_60ea21caa55f), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FORCEFRAMEHEIGHT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 52 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FORCEFRAMEWIDTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 51 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FORCEMEDIANSETTING: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 57 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FORCEOVERLAP: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 80 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FOURCC: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x593e3f2e_f84d_4e85_b68d_f669400edabc), pid: 12 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FRAMECOUNT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x75028eb4_4853_44d3_88a3_e499f89d227f), pid: 14 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_FULLFRAMERATE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 45 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_INTERLACEDCODINGENABLED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x56976073_06c3_4b3b_ad41_b7417fce8474), pid: 26 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_KEYDIST: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x18d6f8c5_2416_4d7b_90d7_9f3f21e752b4), pid: 10 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_LETTERBOXPRESENT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 73 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_LIVEENCODE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x3ffa1e60_5514_4634_86e6_1f3b7c545143), pid: 15 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_LOOKAHEAD: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 54 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_LOOKAHEADRC: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 77 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_LOOPFILTER: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 59 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_LOUDNESS_EQUALIZATION_RELEASE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x9c00eeed_edce_4cd8_ae08_cb05e8ef57a0), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MACROBLOCKMODECOSTMETHOD: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 66 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MAXDECLATENCYMS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MAXENCLATENCYMS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MOST_RECENTLY_ENUMERATED_VBRQUALITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 24 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MOTIONMATCHMETHOD: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 65 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MOTIONSEARCHLEVEL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 64 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MOTIONSEARCHRANGE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 46 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MOTIONVECTORCOSTMETHOD: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 81 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_NOISEEDGEREMOVAL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 60 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_NUMBFRAMES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 48 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_NUMTHREADS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 58 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_NUMTHREADSDEC: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00c6281f_4be6_4e44_9ed8_9ec5422316e4), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_PASSESRECOMMENDED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x38bdceea_393e_4f9a_8dc3_802cc405838f), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_PASSESUSED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xb1653ac1_cb7d_43ee_8454_3f9d811b0331), pid: 19 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_PEAKCONSTRAINED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_PERCEPTUALOPTLEVEL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 56 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_PERIODICALSPDISTANCE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 70 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_PHANTOMING_ON: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x12b53cb2_e12e_4579_8ac3_d02f94f1e89e), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_PIXELASPECTRATIO: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 23 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_POSTPROCESSMODE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x00c6281f_4be6_4e44_9ed8_9ec5422316e4), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_PREFERRED_FRAMESIZE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 22 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_PRODUCEDUMMYFRAMES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x61714bc6_08a1_49d1_b827_a33aada95526), pid: 27 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_PROGRESS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5cefee10_e210_45c6_9e28_f5a8731c96c7), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_QPPERFRAME: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 31 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RANGEREDUX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 53 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RAVG: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x14b2aae6_2987_460a_8b22_9c077c55d05e), pid: 22 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RDSUBPIXELSEARCH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 49 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_REENCDURATION: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 34 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_REENCENDBUFFERSIZE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 36 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_REENCQPREF: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 37 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_REENCSTARTBUFFERSIZE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 35 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_REQUESTING_A_FRAMESIZE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 21 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_DST_HEIGHT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_DST_LEFT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_DST_TOP: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_DST_WIDTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_GEOMAPHEIGHT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 18 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_GEOMAPWIDTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 17 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_GEOMAPX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 15 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_GEOMAPY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 16 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_INTERLACE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 10 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_MINAPHEIGHT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 22 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_MINAPWIDTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 21 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_MINAPX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 19 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_MINAPY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 20 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_PANSCANAPHEIGHT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 14 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_PANSCANAPWIDTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 13 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_PANSCANAPX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 11 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_PANSCANAPY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 12 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_QUALITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_SRC_HEIGHT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_SRC_LEFT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_SRC_TOP: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RESIZE_SRC_WIDTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6612a6bc_e57d_407d_a958_285df0d9b400), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_RMAX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7d8dd246_aaf4_4a24_8166_19396b06ef69), pid: 24 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_ROOMCORR_PROFILE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf311cdc7_f45f_4eb7_a864_9dc1aeeb7e6d), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SCENECHANGE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 74 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SCENECHANGEI: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 76 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SETDYNVIDRES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 87 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SHOTDETECTOR_ADAPTIVETHRESHOLD: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7bcc7b0f_dedf_4a68_96a2_fce019ed956f), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SHOTDETECTOR_ADAPTIVEWINDOWSIZE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7bcc7b0f_dedf_4a68_96a2_fce019ed956f), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SHOTDETECTOR_CLASSIFICATIONMETHOD: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7bcc7b0f_dedf_4a68_96a2_fce019ed956f), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SHOTDETECTOR_GLOBALTHRESHOLD: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7bcc7b0f_dedf_4a68_96a2_fce019ed956f), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SHOTDETECTOR_MAXCLIPDURATION: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7bcc7b0f_dedf_4a68_96a2_fce019ed956f), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SHOTDETECTOR_MAXCLIPSPERHOUR: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7bcc7b0f_dedf_4a68_96a2_fce019ed956f), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SHOTDETECTOR_MINCLIPDURATION: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7bcc7b0f_dedf_4a68_96a2_fce019ed956f), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SHOTDETECTOR_MINCLIPWEIGHT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7bcc7b0f_dedf_4a68_96a2_fce019ed956f), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SHOTDETECTOR_PRESERVEMONOCLIPS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7bcc7b0f_dedf_4a68_96a2_fce019ed956f), pid: 10 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SHOTDETECTOR_TYPE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7bcc7b0f_dedf_4a68_96a2_fce019ed956f), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SMPTE_BORDERCOLOR: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x427ce859_d55c_4f8e_b00e_9cdf761548a6), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SMPTE_BORDERSOFTNESS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x427ce859_d55c_4f8e_b00e_9cdf761548a6), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SMPTE_BORDERWIDTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x427ce859_d55c_4f8e_b00e_9cdf761548a6), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SMPTE_MASKNUM: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x427ce859_d55c_4f8e_b00e_9cdf761548a6), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SMPTE_OFFSETX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x427ce859_d55c_4f8e_b00e_9cdf761548a6), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SMPTE_OFFSETY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x427ce859_d55c_4f8e_b00e_9cdf761548a6), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SMPTE_REPLICATEX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x427ce859_d55c_4f8e_b00e_9cdf761548a6), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SMPTE_REPLICATEY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x427ce859_d55c_4f8e_b00e_9cdf761548a6), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SMPTE_REVERSE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x427ce859_d55c_4f8e_b00e_9cdf761548a6), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_STARTTIME: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5cefee10_e210_45c6_9e28_f5a8731c96c7), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_STAT_BAVG: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x036f6b60_ad43_485c_86c6_21a6db2c1ba3), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_STAT_BMAX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcd95e5b7_9143_47fb_a9d2_9db75f2e74be), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_STAT_RAVG: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x23a0e3b5_fc62_4ab8_b77c_6e0c28ab3016), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_STAT_RMAX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x82ff7c67_6554_4749_a32b_3690dd1ae8de), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_STOPTIME: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x5cefee10_e210_45c6_9e28_f5a8731c96c7), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TARGETENCDELTA: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 85 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TARGETENCRATE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 83 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_THUMBNAILGENERATOR_MAXMOTIONACTIVITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9d7473f_7d68_4226_985c_31852c4c9e74), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_THUMBNAILGENERATOR_MINCOLORENTROPY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9d7473f_7d68_4226_985c_31852c4c9e74), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_THUMBNAILGENERATOR_SEARCHWINDOWSIZE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9d7473f_7d68_4226_985c_31852c4c9e74), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_THUMBNAILGENERATOR_THUMBNAILFILENAME: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9d7473f_7d68_4226_985c_31852c4c9e74), pid: 255 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_THUMBNAILGENERATOR_THUMBNAILHEIGHT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9d7473f_7d68_4226_985c_31852c4c9e74), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_THUMBNAILGENERATOR_THUMBNAILREADY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9d7473f_7d68_4226_985c_31852c4c9e74), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_THUMBNAILGENERATOR_THUMBNAILWIDTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9d7473f_7d68_4226_985c_31852c4c9e74), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_CLUSTERDETECTOR_ON: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_CURRENTTIME: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 253 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_ENDSIGNAL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 251 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_ENDTIME: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 252 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_FACEDETECTOR_ON: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_FEATUREEXTRACTOR_ON: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_INDEXFILENAME: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 255 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_PROCESSEDFRAMES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 254 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_SHOTDETECTOR_ON: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_THUMBNAILGENERATOR_ON: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_TOCOBJECT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_TOCREADY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOCGENERATOR_USEENDSIGNAL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7c109759_3c27_42ef_a6a3_cedef75a35e9), pid: 250 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_TOTALFRAMES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xce5f1e3c_d3d4_4c3f_88c8_01e989d998d2), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_USERDATASIZE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 72 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_VARIABLEGOP: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 75 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_VBRENABLED: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe48d9459_6abe_4eb5_9211_60080c1ab984), pid: 20 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_VBRQUALITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf97b3f3a_9eff_4ac9_8247_35b30eb925f4), pid: 21 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_VIDEOSCALING: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 55 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_VIDEOWINDOW: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xc1c96060_76f0_47d4_a875_5bdda90df5e9), pid: 13 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_VOLHEADERFORREENCODE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 33 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_VTYPE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x4e91bf89_665a_49da_bb94_88c550cfcd28), pid: 61 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_DEVICEPAIR_GUID: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 17 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_DEVICE_INDEXES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_DMO_SOURCE_MODE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_FEATR_AES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 10 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_FEATR_AGC: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_FEATR_CENTER_CLIP: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 12 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_FEATR_ECHO_LENGTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_FEATR_FRAME_SIZE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_FEATR_MICARR_BEAM: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 19 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_FEATR_MICARR_MODE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 18 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_FEATR_MICARR_PREPROC: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 20 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_FEATR_NOISE_FILL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 13 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_FEATR_NS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_FEATR_VAD: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 11 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_FEATURE_MODE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_MICARRAY_DESCPTR: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 16 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_MIC_GAIN_BOUNDER: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 21 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_QUALITY_METRICS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 15 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_RETRIEVE_TS_STATS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 14 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAAECMA_SYSTEM_MODE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6f52c567_0360_4bd2_9617_ccbf1421c939), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMADEC_DRCMODE: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7b613713_3d38_4cda_aa61_0478b1bcfc42), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMADEC_FOLDDOWN_MATRIX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x51647e9b_6a7f_4739_9e0b_294b278969eb), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMADEC_HIRESOUTPUT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8d3fe592_eecc_4f4e_9aff_5af1679d38d2), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMADEC_LTRTOUTPUT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7b613713_3d38_4cda_aa61_0478b1bcfc42), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMADEC_SPKRCFG: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8fff67be_977f_41dc_8faf_23bac9a6df73), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMADRC_AVGREF: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7b613713_3d38_4cda_aa61_0478b1bcfc42), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMADRC_AVGTARGET: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7b613713_3d38_4cda_aa61_0478b1bcfc42), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMADRC_PEAKREF: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7b613713_3d38_4cda_aa61_0478b1bcfc42), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMADRC_PEAKTARGET: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x7b613713_3d38_4cda_aa61_0478b1bcfc42), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAENC_AVGBYTESPERSEC: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x11caf780_921b_42ef_b755_f3a053ea1a41), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAENC_BUFFERLESSCBR: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 29 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAENC_GENERATE_DRC_PARAMS: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 28 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAENC_ORIGWAVEFORMAT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf5c760a2_3635_48e1_8fbd_0e498124e0a2), pid: 0 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAENC_RTSPDIF: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 30 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAVOICE_DEC_RT_JitterControl: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0165f69b_80a1_4ef8_a4a9_ada3b8a689dd), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAVOICE_DEC_RT_JitterMode: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0165f69b_80a1_4ef8_a4a9_ada3b8a689dd), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAVOICE_DEC_RT_PacketLossMode: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x0165f69b_80a1_4ef8_a4a9_ada3b8a689dd), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAVOICE_ENC_BufferWindow: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9c8f5fe_8682_4347_8507_60a3f51f33f1), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAVOICE_ENC_DecoderDelay: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9c8f5fe_8682_4347_8507_60a3f51f33f1), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAVOICE_ENC_EDL: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9c8f5fe_8682_4347_8507_60a3f51f33f1), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAVOICE_ENC_MusicSpeechClassMode: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9c8f5fe_8682_4347_8507_60a3f51f33f1), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAVOICE_ENC_RT_BandWidth: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9c8f5fe_8682_4347_8507_60a3f51f33f1), pid: 7 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAVOICE_ENC_RT_MinBufferSize: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9c8f5fe_8682_4347_8507_60a3f51f33f1), pid: 9 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAVOICE_ENC_RT_PacketLossMode: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9c8f5fe_8682_4347_8507_60a3f51f33f1), pid: 8 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMAVOICE_ENC_RT_VariableRate: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd9c8f5fe_8682_4347_8507_60a3f51f33f1), pid: 6 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMA_ELEMENTARY_STREAM: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6dbdf03b_b05c_4a03_8ec1_bbe63db10cb4), pid: 23 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMRESAMP_CHANNELMTX: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xaf1adc73_a210_4b05_966e_5491cff48b1d), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMRESAMP_FILTERQUALITY: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xaf1adc73_a210_4b05_966e_5491cff48b1d), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_WMRESAMP_LOWPASS_BANDWIDTH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xaf1adc73_a210_4b05_966e_5491cff48b1d), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_ZEROBYTEFRAMES: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x8f04aba4_313d_40fb_8031_31517813d9ef), pid: 7 };
pub const MFSampleExtension_VideoDSPMode: windows_core::GUID = windows_core::GUID::from_u128(0xc12d55cb_d7d9_476d_81f3_69117f163ea0);
pub type MFVideoDSPMode = i32;
pub const MFVideoDSPMode_Passthrough: MFVideoDSPMode = 1;
pub const MFVideoDSPMode_Stabilization: MFVideoDSPMode = 4;
pub type MF_AUVRHP_ROOMMODEL = i32;
pub const MF_VIDEODSP_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x16d720f0_768c_11de_8a39_0800200c9a66);
pub const MICARRAY_EXTERN_BEAM: MIC_ARRAY_MODE = 2048;
pub const MICARRAY_FIXED_BEAM: MIC_ARRAY_MODE = 1024;
pub const MICARRAY_SIMPLE_SUM: MIC_ARRAY_MODE = 256;
pub const MICARRAY_SINGLE_BEAM: MIC_ARRAY_MODE = 512;
pub const MICARRAY_SINGLE_CHAN: MIC_ARRAY_MODE = 0;
pub type MIC_ARRAY_MODE = i32;
pub const MODE_NOT_SET: AEC_SYSTEM_MODE = 6;
pub const MP3ACMCodecWrapper: windows_core::GUID = windows_core::GUID::from_u128(0x11103421_354c_4cca_a7a3_1aff9a5b6701);
pub const MSAMRNBDecoder: windows_core::GUID = windows_core::GUID::from_u128(0x265011ae_5481_4f77_a295_abb6ffe8d63e);
pub const MSAMRNBEncoder: windows_core::GUID = windows_core::GUID::from_u128(0x2fae8afe_04a3_423a_a814_85db454712b0);
pub const MULawCodecWrapper: windows_core::GUID = windows_core::GUID::from_u128(0x92b66080_5e2d_449e_90c4_c41f268e5514);
pub const OPENMODE_APPEND_IF_EXIST: FILE_OPENMODE = 3;
pub const OPENMODE_DELETE_IF_EXIST: FILE_OPENMODE = 4;
pub const OPENMODE_FAIL_IF_EXIST: FILE_OPENMODE = 1;
pub const OPENMODE_FAIL_IF_NOT_EXIST: FILE_OPENMODE = 0;
pub const OPENMODE_RESET_IF_EXIST: FILE_OPENMODE = 2;
pub const OPTIBEAM_ARRAY_AND_AEC: AEC_SYSTEM_MODE = 4;
pub const OPTIBEAM_ARRAY_ONLY: AEC_SYSTEM_MODE = 2;
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const PKEY_SYSFXUI_HIDE_MASK: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xcb9c6bce_7a25_47aa_b2be_6ad84431edde), pid: 2 };
pub type SEEK_ORIGIN = i32;
pub const SINGLE_CHANNEL_AEC: AEC_SYSTEM_MODE = 0;
pub const SINGLE_CHANNEL_NSAGC: AEC_SYSTEM_MODE = 5;
pub const SYSFXUI_DONOTSHOW_BASSBOOST: i32 = 8;
pub const SYSFXUI_DONOTSHOW_BASSMANAGEMENT: i32 = 4;
pub const SYSFXUI_DONOTSHOW_CHANNELPHANTOMING: i32 = 128;
pub const SYSFXUI_DONOTSHOW_HEADPHONEVIRTUALIZATION: i32 = 16;
pub const SYSFXUI_DONOTSHOW_LOUDNESSEQUALIZATION: i32 = 1;
pub const SYSFXUI_DONOTSHOW_ROOMCORRECTION: i32 = 2;
pub const SYSFXUI_DONOTSHOW_SPEAKERFILLING: i32 = 64;
pub const SYSFXUI_DONOTSHOW_VIRTUALSURROUND: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOC_DESCRIPTOR {
    pub guidID: windows_core::GUID,
    pub wStreamNumber: super::WORD,
    pub guidType: windows_core::GUID,
    pub wLanguageIndex: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOC_ENTRY_DESCRIPTOR {
    pub qwStartTime: super::QWORD,
    pub qwEndTime: super::QWORD,
    pub qwStartPacketOffset: super::QWORD,
    pub qwEndPacketOffset: super::QWORD,
    pub qwRepresentativeFrameTime: super::QWORD,
}
pub const TOC_ENTRY_MAX_TITLE_SIZE: i32 = 65535;
pub const TOC_MAX_DESCRIPTION_SIZE: i32 = 65535;
pub const TOC_POS_INHEADER: TOC_POS_TYPE = 0;
pub const TOC_POS_TOPLEVELOBJECT: TOC_POS_TYPE = 1;
pub type TOC_POS_TYPE = i32;
pub const VRHP_BIGROOM: MF_AUVRHP_ROOMMODEL = 2;
pub const VRHP_CUSTUMIZEDROOM: MF_AUVRHP_ROOMMODEL = 3;
pub const VRHP_MEDIUMROOM: MF_AUVRHP_ROOMMODEL = 1;
pub const VRHP_SMALLROOM: MF_AUVRHP_ROOMMODEL = 0;
pub const VorbisDecoderMFT: windows_core::GUID = windows_core::GUID::from_u128(0x1a198ef2_60e5_4ea8_90d8_da1f2832c288);
pub const WMAAECMA_E_NO_ACTIVE_RENDER_STREAM: u32 = 2278293514;
pub type WMT_PROP_DATATYPE = i32;
pub const WMT_PROP_TYPE_BINARY: WMT_PROP_DATATYPE = 2;
pub const WMT_PROP_TYPE_BOOL: WMT_PROP_DATATYPE = 3;
pub const WMT_PROP_TYPE_DWORD: WMT_PROP_DATATYPE = 0;
pub const WMT_PROP_TYPE_GUID: WMT_PROP_DATATYPE = 6;
pub const WMT_PROP_TYPE_QWORD: WMT_PROP_DATATYPE = 4;
pub const WMT_PROP_TYPE_STRING: WMT_PROP_DATATYPE = 1;
pub const WMT_PROP_TYPE_WORD: WMT_PROP_DATATYPE = 5;
pub const WMV_DYNAMIC_BITRATE: WMV_DYNAMIC_FLAGS = 1;
pub const WMV_DYNAMIC_COMPLEXITY: WMV_DYNAMIC_FLAGS = 4;
pub type WMV_DYNAMIC_FLAGS = i32;
pub const WMV_DYNAMIC_RESOLUTION: WMV_DYNAMIC_FLAGS = 2;
pub const WM_CODEC_ONEPASS_CBR: i32 = 1;
pub const WM_CODEC_ONEPASS_VBR: i32 = 2;
pub const WM_CODEC_TWOPASS_CBR: i32 = 4;
pub const WM_CODEC_TWOPASS_VBR_PEAKCONSTRAINED: i32 = 16;
pub const WM_CODEC_TWOPASS_VBR_UNCONSTRAINED: i32 = 8;
pub const _msoBegin: SEEK_ORIGIN = 0;
pub const _msoCurrent: SEEK_ORIGIN = 1;
pub const g_wszWMACAvgBytesPerSec: windows_core::PCWSTR = windows_core::w!("AvgBytesPerSec");
pub const g_wszWMACAvgPCMValue: windows_core::PCWSTR = windows_core::w!("AverageLevel");
pub const g_wszWMACDRCSetting: windows_core::PCWSTR = windows_core::w!("DynamicRangeControl");
pub const g_wszWMACHiResOutput: windows_core::PCWSTR = windows_core::w!("_HIRESOUTPUT");
pub const g_wszWMACIncludeNumPasses: windows_core::PCWSTR = windows_core::w!("_INCLUDENUMPASSES");
pub const g_wszWMACInputFormatName: windows_core::PCWSTR = windows_core::w!("_INPUTFORMATNAME");
pub const g_wszWMACMixTable: windows_core::PCWSTR = windows_core::w!("MixTable");
pub const g_wszWMACMusicSpeechClassMode: windows_core::PCWSTR = windows_core::w!("MusicSpeechClassMode");
pub const g_wszWMACOriginalWaveFormat: windows_core::PCWSTR = windows_core::w!("_ORIGINALWAVEFORMAT");
pub const g_wszWMACPeakPCMValue: windows_core::PCWSTR = windows_core::w!("PeakValue");
pub const g_wszWMACSourceFormatTag: windows_core::PCWSTR = windows_core::w!("_SOURCEFORMATTAG");
pub const g_wszWMACSpeakerConfig: windows_core::PCWSTR = windows_core::w!("SpeakerConfig");
pub const g_wszWMACVoiceBuffer: windows_core::PCWSTR = windows_core::w!("BufferWindow");
pub const g_wszWMACVoiceEDL: windows_core::PCWSTR = windows_core::w!("_EDL");
pub const g_wszWMADRCAverageReference: windows_core::PCWSTR = windows_core::w!("WMADRCAverageReference");
pub const g_wszWMADRCAverageTarget: windows_core::PCWSTR = windows_core::w!("WMADRCAverageTarget");
pub const g_wszWMADRCPeakReference: windows_core::PCWSTR = windows_core::w!("WMADRCPeakReference");
pub const g_wszWMADRCPeakTarget: windows_core::PCWSTR = windows_core::w!("WMADRCPeakTarget");
pub const g_wszWMVAspectHorizSize: windows_core::PCWSTR = windows_core::w!("_AspectHorizSize");
pub const g_wszWMVAspectVertSize: windows_core::PCWSTR = windows_core::w!("_AspectVertSize");
pub const g_wszWMVCAdaptiveResolution: windows_core::PCWSTR = windows_core::w!("_ADAPTIVERESOLUTION");
pub const g_wszWMVCAvgBitrate: windows_core::PCWSTR = windows_core::w!("_RAVG");
pub const g_wszWMVCAvgFrameRate: windows_core::PCWSTR = windows_core::w!("_AVGFRAMERATE");
pub const g_wszWMVCBAvg: windows_core::PCWSTR = windows_core::w!("_BAVG");
pub const g_wszWMVCBDeltaQP: windows_core::PCWSTR = windows_core::w!("_BDELTAQP");
pub const g_wszWMVCBMax: windows_core::PCWSTR = windows_core::w!("_BMAX");
pub const g_wszWMVCBufferFullnessInFirstByte: windows_core::PCWSTR = windows_core::w!("_BUFFERFULLNESSINFIRSTBYTE");
pub const g_wszWMVCChangeBitRate: windows_core::PCWSTR = windows_core::w!("_CHANGEBITRATE");
pub const g_wszWMVCChangeFrameRate: windows_core::PCWSTR = windows_core::w!("_CHANGEFRAMERATE");
pub const g_wszWMVCChangeMaxBitRate: windows_core::PCWSTR = windows_core::w!("_CHANGEMAXBITRATE");
pub const g_wszWMVCClosedEntryPoint: windows_core::PCWSTR = windows_core::w!("_CLOSEDENTRYPOINT");
pub const g_wszWMVCCodedFrames: windows_core::PCWSTR = windows_core::w!("_CODEDFRAMES");
pub const g_wszWMVCComplexityEx: windows_core::PCWSTR = windows_core::w!("_COMPLEXITYEX");
pub const g_wszWMVCComplexityLive: windows_core::PCWSTR = windows_core::w!("_COMPLEXITYEXLIVE");
pub const g_wszWMVCComplexityMax: windows_core::PCWSTR = windows_core::w!("_COMPLEXITYEXMAX");
pub const g_wszWMVCComplexityMode: windows_core::PCWSTR = windows_core::w!("_COMPLEXITY");
pub const g_wszWMVCComplexityOffline: windows_core::PCWSTR = windows_core::w!("_COMPLEXITYEXOFFLINE");
pub const g_wszWMVCCompressionOptimizationType: windows_core::PCWSTR = windows_core::w!("_COMPRESSIONOPTIMIZATIONTYPE");
pub const g_wszWMVCCrisp: windows_core::PCWSTR = windows_core::w!("_CRISP");
pub const g_wszWMVCDatarate: windows_core::PCWSTR = windows_core::w!("_DATARATE");
pub const g_wszWMVCDecoderComplexityProfile: windows_core::PCWSTR = windows_core::w!("_DECODERCOMPLEXITYPROFILE");
pub const g_wszWMVCDecoderComplexityRequested: windows_core::PCWSTR = windows_core::w!("_DECODERCOMPLEXITYREQUESTED");
pub const g_wszWMVCDecoderDeinterlacing: windows_core::PCWSTR = windows_core::w!("_DECODERDEINTERLACING");
pub const g_wszWMVCDecoderForceNoResizeOutput: windows_core::PCWSTR = windows_core::w!("_FORCENORESIZE");
pub const g_wszWMVCDefaultCrisp: windows_core::PCWSTR = windows_core::w!("_DEFAULTCRISP");
pub const g_wszWMVCDeltaMVRangeIndex: windows_core::PCWSTR = windows_core::w!("_DELTAMVRANGEINDEX");
pub const g_wszWMVCDenoiseOption: windows_core::PCWSTR = windows_core::w!("_DENOISEOPTION");
pub const g_wszWMVCDquantOption: windows_core::PCWSTR = windows_core::w!("_DQUANTOPTION");
pub const g_wszWMVCDquantStrength: windows_core::PCWSTR = windows_core::w!("_DQUANTSTRENGTH");
pub const g_wszWMVCDynComplexityLevel: windows_core::PCWSTR = windows_core::w!("_DYNCOMPLEXLEVEL");
pub const g_wszWMVCDynamicEncoderMode: windows_core::PCWSTR = windows_core::w!("_DYNENCMODE");
pub const g_wszWMVCDynamicVideoResolution: windows_core::PCWSTR = windows_core::w!("_SETDYNVIDRES");
pub const g_wszWMVCEncodedHeight_Dec: windows_core::PCWSTR = windows_core::w!("_ENCODEDHEIGHT_DEC");
pub const g_wszWMVCEncodedWidth_Dec: windows_core::PCWSTR = windows_core::w!("_ENCODEDWIDTH_DEC");
pub const g_wszWMVCEncodercomplexity: windows_core::PCWSTR = windows_core::w!("_ENCODERCOMPLEXITY");
pub const g_wszWMVCEncodingHeight: windows_core::PCWSTR = windows_core::w!("_ENCODINGHEIGHT");
pub const g_wszWMVCEncodingWidth: windows_core::PCWSTR = windows_core::w!("_ENCODINGWIDTH");
pub const g_wszWMVCEndOfPass: windows_core::PCWSTR = windows_core::w!("_ENDOFPASS");
pub const g_wszWMVCFOURCC: windows_core::PCWSTR = windows_core::w!("_FOURCC");
pub const g_wszWMVCForceFrameHeight: windows_core::PCWSTR = windows_core::w!("_FORCEFRAMEHEIGHT");
pub const g_wszWMVCForceFrameWidth: windows_core::PCWSTR = windows_core::w!("_FORCEFRAMEWIDTH");
pub const g_wszWMVCForceMedianSetting: windows_core::PCWSTR = windows_core::w!("_FORCEMEDIANSETTING");
pub const g_wszWMVCForceOverlap: windows_core::PCWSTR = windows_core::w!("_FORCEOVERLAP");
pub const g_wszWMVCForcePostProcessMode: windows_core::PCWSTR = windows_core::w!("_POSTPROCESSMODE");
pub const g_wszWMVCFrameCount: windows_core::PCWSTR = windows_core::w!("_FRAMECOUNT");
pub const g_wszWMVCFrameInterpolationEnabled: windows_core::PCWSTR = windows_core::w!("_FRAMEINTERPOLATIONENABLED");
pub const g_wszWMVCFrameInterpolationSupported: windows_core::PCWSTR = windows_core::w!("_FRAMEINTERPOLATIONSUPPORTED");
pub const g_wszWMVCFullFrameRate: windows_core::PCWSTR = windows_core::w!("_FULLFRAMERATE");
pub const g_wszWMVCHonorKeyFrameSettings: windows_core::PCWSTR = windows_core::w!("_HONORKEYSETTINGS");
pub const g_wszWMVCHonorTSFrameQP: windows_core::PCWSTR = windows_core::w!("_HONORTSFRAMEQP");
pub const g_wszWMVCInterlacedCodingEnabled: windows_core::PCWSTR = windows_core::w!("_INTERLACEDCODINGENABLED");
pub const g_wszWMVCInverseTelecinedInput: windows_core::PCWSTR = windows_core::w!("_INVERSETELECINEDINPUT");
pub const g_wszWMVCKeyframeDistance: windows_core::PCWSTR = windows_core::w!("_KEYDIST");
pub const g_wszWMVCLegacy411InterlacedFormat: windows_core::PCWSTR = windows_core::w!("_LEGACY411INTERLACEDFORMAT");
pub const g_wszWMVCLetterboxpresent: windows_core::PCWSTR = windows_core::w!("_LETTERBOXPRESENT");
pub const g_wszWMVCLiveEncode: windows_core::PCWSTR = windows_core::w!("_LIVEENCODE");
pub const g_wszWMVCLookAhead: windows_core::PCWSTR = windows_core::w!("_LOOKAHEAD");
pub const g_wszWMVCLookaheadRC: windows_core::PCWSTR = windows_core::w!("_LOOKAHEADRC");
pub const g_wszWMVCLoopFilter: windows_core::PCWSTR = windows_core::w!("_LOOPFILTER");
pub const g_wszWMVCMacroblockModeCostMethod: windows_core::PCWSTR = windows_core::w!("_MACROBLOCKMODECOSTMETHOD");
pub const g_wszWMVCMaxBitrate: windows_core::PCWSTR = windows_core::w!("_RMAX");
pub const g_wszWMVCMirrorDisplayOn: windows_core::PCWSTR = windows_core::w!("_MIRRORDISPLAYON");
pub const g_wszWMVCMotionMatchMethod: windows_core::PCWSTR = windows_core::w!("_MOTIONMATCHMETHOD");
pub const g_wszWMVCMotionSearchLevel: windows_core::PCWSTR = windows_core::w!("_MOTIONSEARCHLEVEL");
pub const g_wszWMVCMotionSearchRange: windows_core::PCWSTR = windows_core::w!("_MOTIONSEARCHRANGE");
pub const g_wszWMVCMotionVectorCostMethod: windows_core::PCWSTR = windows_core::w!("_MOTIONVECTORCOSTMETHOD");
pub const g_wszWMVCNeedsDrain: windows_core::PCWSTR = windows_core::w!("_DECODERNEEDSDRAIN");
pub const g_wszWMVCNoiseEdgeRemoval: windows_core::PCWSTR = windows_core::w!("_NOISEEDGEREMOVAL");
pub const g_wszWMVCNumBFrames: windows_core::PCWSTR = windows_core::w!("_NUMBFRAMES");
pub const g_wszWMVCNumThreads: windows_core::PCWSTR = windows_core::w!("_NUMTHREADS");
pub const g_wszWMVCNumThreadsDec: windows_core::PCWSTR = windows_core::w!("_NUMTHREADSDEC");
pub const g_wszWMVCPacketOverhead: windows_core::PCWSTR = windows_core::w!("_ASFOVERHEADPERFRAME");
pub const g_wszWMVCPassesRecommended: windows_core::PCWSTR = windows_core::w!("_PASSESRECOMMENDED");
pub const g_wszWMVCPassesUsed: windows_core::PCWSTR = windows_core::w!("_PASSESUSED");
pub const g_wszWMVCPerceptualOptLevel: windows_core::PCWSTR = windows_core::w!("_PERCEPTUALOPTLEVEL");
pub const g_wszWMVCPeriodicalSPDistance: windows_core::PCWSTR = windows_core::w!("_PERIODICALSPDISTANCE");
pub const g_wszWMVCProduceDummyFrames: windows_core::PCWSTR = windows_core::w!("_PRODUCEDUMMYFRAMES");
pub const g_wszWMVCQPPerFrame: windows_core::PCWSTR = windows_core::w!("_QPPERFRAME");
pub const g_wszWMVCQueryTimeStampTag: windows_core::PCWSTR = windows_core::w!("_QUERYTIMESTAMPTAG");
pub const g_wszWMVCQueryTimeStampTagDec: windows_core::PCWSTR = windows_core::w!("_QUERYTIMESTAMPTAGDEC");
pub const g_wszWMVCRDSubpixelSearch: windows_core::PCWSTR = windows_core::w!("_RDSUBPIXELSEARCH");
pub const g_wszWMVCRangeRedux: windows_core::PCWSTR = windows_core::w!("_RANGEREDUX");
pub const g_wszWMVCReencDuration: windows_core::PCWSTR = windows_core::w!("_REENCDURATION");
pub const g_wszWMVCReencEndBufferSize: windows_core::PCWSTR = windows_core::w!("_REENCENDBUFFERSIZE");
pub const g_wszWMVCReencQPRef: windows_core::PCWSTR = windows_core::w!("_REENCQPREF");
pub const g_wszWMVCReencStartBufferSize: windows_core::PCWSTR = windows_core::w!("_REENCSTARTBUFFERSIZE");
pub const g_wszWMVCSceneChangeI: windows_core::PCWSTR = windows_core::w!("_SCENECHANGEI");
pub const g_wszWMVCScenechange: windows_core::PCWSTR = windows_core::w!("_SCENECHANGE");
pub const g_wszWMVCSupportOneInMultiOut: windows_core::PCWSTR = windows_core::w!("_SUPPORTONEINMULTIOUT");
pub const g_wszWMVCSupportOneInMultiOut_Dec: windows_core::PCWSTR = windows_core::w!("_SUPPORTONEINMULTIOUT_DEC");
pub const g_wszWMVCTargetEncodeDelta: windows_core::PCWSTR = windows_core::w!("_TARGETENCDELTA");
pub const g_wszWMVCTargetEncodeRate: windows_core::PCWSTR = windows_core::w!("_TARGETENCRATE");
pub const g_wszWMVCThreadAffinityMask: windows_core::PCWSTR = windows_core::w!("_THREADAFFINITYMASK");
pub const g_wszWMVCTotalFrames: windows_core::PCWSTR = windows_core::w!("_TOTALFRAMES");
pub const g_wszWMVCTotalWindow: windows_core::PCWSTR = windows_core::w!("_TOTALWINDOW");
pub const g_wszWMVCUserdatasize: windows_core::PCWSTR = windows_core::w!("_USERDATASIZE");
pub const g_wszWMVCVBREnabled: windows_core::PCWSTR = windows_core::w!("_VBRENABLED");
pub const g_wszWMVCVBRQuality: windows_core::PCWSTR = windows_core::w!("_VBRQUALITY");
pub const g_wszWMVCVType: windows_core::PCWSTR = windows_core::w!("_VTYPE");
pub const g_wszWMVCVariableGOP: windows_core::PCWSTR = windows_core::w!("_VARIABLEGOP");
pub const g_wszWMVCVideoScaling: windows_core::PCWSTR = windows_core::w!("_VIDEOSCALING");
pub const g_wszWMVCVideoWIndow: windows_core::PCWSTR = windows_core::w!("_VIDEOWINDOW");
pub const g_wszWMVCWatermarkConfig: windows_core::PCWSTR = windows_core::w!("WatermarkConfig");
pub const g_wszWMVCWatermarkDelay: windows_core::PCWSTR = windows_core::w!("WatermarkDelay");
pub const g_wszWMVDisplayHeight: windows_core::PCWSTR = windows_core::w!("_DisplayHeight");
pub const g_wszWMVDisplayWidth: windows_core::PCWSTR = windows_core::w!("_DisplayWidth");
pub const g_wszWMVEncodeHeight: windows_core::PCWSTR = windows_core::w!("_EncodeHeight");
pub const g_wszWMVEncodeWidth: windows_core::PCWSTR = windows_core::w!("_EncodeWidth");
pub const g_wszWMVForceStartCode: windows_core::PCWSTR = windows_core::w!("_FORCESTARTCODE");
pub const g_wszWMVTimeStampFixed: windows_core::PCWSTR = windows_core::w!("TSFixed");
pub const g_wszWMVTranscodeMode: windows_core::PCWSTR = windows_core::w!("_TranscodeMode");

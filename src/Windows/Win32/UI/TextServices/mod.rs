#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ANCHOR_CHANGE_HISTORY_FLAGS(pub u32);
pub const TS_CH_PRECEDING_DEL: ANCHOR_CHANGE_HISTORY_FLAGS = ANCHOR_CHANGE_HISTORY_FLAGS(1u32);
pub const TS_CH_FOLLOWING_DEL: ANCHOR_CHANGE_HISTORY_FLAGS = ANCHOR_CHANGE_HISTORY_FLAGS(2u32);
impl ::core::convert::From<u32> for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const AccClientDocMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfc48cc30_4f3e_4fa1_803b_ad0e196a83b1);
pub const AccDictionary: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6572ee16_5fe5_4331_bb6d_76a49c56e423);
pub const AccServerDocMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6089a37e_eb8a_482d_bd6f_f9f46904d16d);
pub const AccStore: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5440837f_4bff_4ae5_a1b1_7722ecc6332a);
pub const CLSID_TF_CategoryMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa4b544a1_438d_4b41_9325_869523e2d6c7);
pub const CLSID_TF_ClassicLangBar: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3318360c_1afc_4d09_a86b_9f9cb6dceb9c);
pub const CLSID_TF_DisplayAttributeMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3ce74de4_53d3_4d74_8b83_431b3828ba53);
pub const CLSID_TF_InputProcessorProfiles: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x33c53a50_f456_4884_b049_85fd643ecfed);
pub const CLSID_TF_LangBarItemMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb9931692_a2b3_4fab_bf33_9ec6f9fb96ac);
pub const CLSID_TF_LangBarMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xebb08c45_6c4a_4fdc_ae53_4eb8c4c7db8e);
pub const CLSID_TF_ThreadMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x529a9e6b_6587_4f23_ab9e_9c7d683e3c50);
pub const CLSID_TF_TransitoryExtensionUIEntry: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xae6be008_07fb_400d_8beb_337a64f7051f);
pub const CLSID_TsfServices: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x39aedc00_6b60_46db_8d31_3642be0e4373);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const DCM_FLAGS_CTFMON: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const DCM_FLAGS_LOCALTHREADTSF: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const DCM_FLAGS_TASKENG: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DoMsCtfMonitor<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(dwflags: u32, heventforservicestop: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DoMsCtfMonitor(dwflags: u32, heventforservicestop: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DoMsCtfMonitor(::core::mem::transmute(dwflags), heventforservicestop.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DocWrap: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbf426f7e_7a5e_44d6_830c_a390ea9462a3);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GET_TEXT_AND_PROPERTY_UPDATES_FLAGS(pub u32);
pub const TF_GTP_NONE: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS = GET_TEXT_AND_PROPERTY_UPDATES_FLAGS(0u32);
pub const TF_GTP_INCL_TEXT: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS = GET_TEXT_AND_PROPERTY_UPDATES_FLAGS(1u32);
impl ::core::convert::From<u32> for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GUID_APP_FUNCTIONPROVIDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4caef01e_12af_4b0e_9db1_a6ec5b881208);
pub const GUID_COMPARTMENT_CONVERSIONMODEBIAS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5497f516_ee91_436e_b946_aa2c05f1ac5b);
pub const GUID_COMPARTMENT_EMPTYCONTEXT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd7487dbf_804e_41c5_894d_ad96fd4eea13);
pub const GUID_COMPARTMENT_ENABLED_PROFILES_UPDATED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x92c1fd48_a9ae_4a7c_be08_4329e4723817);
pub const GUID_COMPARTMENT_HANDWRITING_OPENCLOSE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf9ae2c6b_1866_4361_af72_7aa30948890e);
pub const GUID_COMPARTMENT_KEYBOARD_DISABLED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x71a5b253_1951_466b_9fbc_9c8808fa84f2);
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb6592511_bcee_4122_a7c4_09f4b3fa4396);
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE_CONVERSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xccf05dd8_4a87_11d7_a6e2_00065b84435c);
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE_SENTENCE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xccf05dd9_4a87_11d7_a6e2_00065b84435c);
pub const GUID_COMPARTMENT_KEYBOARD_OPENCLOSE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x58273aad_01bb_4164_95c6_755ba0b5162d);
pub const GUID_COMPARTMENT_SAPI_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x51af2086_cc6b_457d_b5aa_8b19dc290ab4);
pub const GUID_COMPARTMENT_SPEECH_CFGMENU: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfb6c5c2d_4e83_4bb6_91a2_e019bff6762d);
pub const GUID_COMPARTMENT_SPEECH_DISABLED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x56c5c607_0703_4e59_8e52_cbc84e8bbe35);
pub const GUID_COMPARTMENT_SPEECH_GLOBALSTATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2a54fe8e_0d08_460c_a75d_87035ff436c5);
pub const GUID_COMPARTMENT_SPEECH_OPENCLOSE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x544d6a63_e2e8_4752_bbd1_000960bca083);
pub const GUID_COMPARTMENT_SPEECH_UI_STATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd92016f0_9367_4fe7_9abf_bc59dacbe0e3);
pub const GUID_COMPARTMENT_TIPUISTATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x148ca3ec_0366_401c_8d75_ed978d85fbc9);
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8be347f5_c7a0_11d7_b408_00065b84435c);
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION_DOCUMENTMANAGER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8be347f7_c7a0_11d7_b408_00065b84435c);
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION_PARENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8be347f8_c7a0_11d7_b408_00065b84435c);
pub const GUID_INTEGRATIONSTYLE_SEARCHBOX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe6d1bd11_82f7_4903_ae21_1a6397cde2eb);
pub const GUID_LBI_INPUTMODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2c77a81e_41cc_4178_a3a7_5f8a987568e6);
pub const GUID_LBI_SAPILAYR_CFGMENUBUTTON: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd02f24a1_942d_422e_8d99_b4f2addee999);
pub const GUID_MODEBIAS_CHINESE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7add26de_4328_489b_83ae_6493750cad5c);
pub const GUID_MODEBIAS_CONVERSATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0f4ec104_1790_443b_95f1_e10f939d6546);
pub const GUID_MODEBIAS_DATETIME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf2bdb372_7f61_4039_92ef_1c35599f0222);
pub const GUID_MODEBIAS_FILENAME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd7f707fe_44c6_4fca_8e76_86ab50c7931b);
pub const GUID_MODEBIAS_FULLWIDTHALPHANUMERIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x81489fb8_b36a_473d_8146_e4a2258b24ae);
pub const GUID_MODEBIAS_FULLWIDTHHANGUL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc01ae6c9_45b5_4fd0_9cb1_9f4cebc39fea);
pub const GUID_MODEBIAS_HALFWIDTHKATAKANA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x005f6b63_78d4_41cc_8859_485ca821a795);
pub const GUID_MODEBIAS_HANGUL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x76ef0541_23b3_4d77_a074_691801ccea17);
pub const GUID_MODEBIAS_HIRAGANA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd73d316e_9b91_46f1_a280_31597f52c694);
pub const GUID_MODEBIAS_KATAKANA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2e0eeddd_3a1a_499e_8543_3c7ee7949811);
pub const GUID_MODEBIAS_NAME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfddc10f0_d239_49bf_b8fc_5410caaa427e);
pub const GUID_MODEBIAS_NONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const GUID_MODEBIAS_NUMERIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4021766c_e872_48fd_9cee_4ec5c75e16c3);
pub const GUID_MODEBIAS_READING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe31643a3_6466_4cbf_8d8b_0bd4d8545461);
pub const GUID_MODEBIAS_URLHISTORY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8b0e54d9_63f2_4c68_84d4_79aee7a59f09);
pub const GUID_PROP_ATTRIBUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x34b45670_7526_11d2_a147_00105a2799b5);
pub const GUID_PROP_COMPOSING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe12ac060_af15_11d2_afc5_00105a2799b5);
pub const GUID_PROP_INPUTSCOPE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1713dd5a_68e7_4a5b_9af6_592a595c778d);
pub const GUID_PROP_LANGID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3280ce20_8032_11d2_b603_00105a2799b5);
pub const GUID_PROP_MODEBIAS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x372e0716_974f_40ac_a088_08cdc92ebfbc);
pub const GUID_PROP_READING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5463f7c0_8e31_11d2_bf46_00105a2799b5);
pub const GUID_PROP_TEXTOWNER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf1e2d520_0969_11d3_8df0_00105a2799b5);
pub const GUID_PROP_TKB_ALTERNATES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x70b2a803_968d_462e_b93b_2164c91517f7);
pub const GUID_SYSTEM_FUNCTIONPROVIDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9a698bb0_0f21_11d3_8df1_00105a2799b5);
pub const GUID_TFCAT_CATEGORY_OF_TIP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x534c48c1_0607_4098_a521_4fc899c73e90);
pub const GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb95f181b_ea4c_4af1_8056_7c321abbb091);
pub const GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x046b8c80_1647_40f7_9b21_b93b81aabc1b);
pub const GUID_TFCAT_PROPSTYLE_STATIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x565fb8d8_6bd4_4ca1_b223_0f2ccb8f4f96);
pub const GUID_TFCAT_PROP_AUDIODATA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9b7be3a9_e8ab_4d47_a8fe_254fa423436d);
pub const GUID_TFCAT_PROP_INKDATA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7c6a82ae_b0d7_4f14_a745_14f28b009d61);
pub const GUID_TFCAT_TIPCAP_COMLESS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x364215d9_75bc_11d7_a6ef_00065b84435c);
pub const GUID_TFCAT_TIPCAP_DUALMODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3af314a2_d79f_4b1b_9992_15086d339b05);
pub const GUID_TFCAT_TIPCAP_IMMERSIVEONLY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3a4259ac_640d_4ad4_89f7_1eb67e7c4ee8);
pub const GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x13a016df_560b_46cd_947a_4c3af1e0e35d);
pub const GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xccf05dd7_4a87_11d7_a6e2_00065b84435c);
pub const GUID_TFCAT_TIPCAP_LOCALSERVER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x74769ee9_4a66_4f9d_90d6_bf8b7c3eb461);
pub const GUID_TFCAT_TIPCAP_SECUREMODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x49d2f9ce_1f5e_11d7_a6d3_00065b84435c);
pub const GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x25504fb4_7bab_4bc1_9c69_cf81890f0ef5);
pub const GUID_TFCAT_TIPCAP_TSF3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x07dcb4af_98de_4548_bef7_25bd45979a1f);
pub const GUID_TFCAT_TIPCAP_UIELEMENTENABLED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x49d2f9cf_1f5e_11d7_a6d3_00065b84435c);
pub const GUID_TFCAT_TIPCAP_WOW16: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x364215da_75bc_11d7_a6ef_00065b84435c);
pub const GUID_TFCAT_TIP_HANDWRITING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x246ecb87_c2f2_4abe_905b_c8b38add2c43);
pub const GUID_TFCAT_TIP_KEYBOARD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x34745c63_b2f0_4784_8b67_5e12c8701a31);
pub const GUID_TFCAT_TIP_SPEECH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb5a73cd1_8355_426b_a161_259808f26b14);
pub const GUID_TFCAT_TRANSITORYEXTENSIONUI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6302de22_a5cf_4b02_bfe8_4d72b2bed3c6);
pub const GUID_TS_SERVICE_ACCESSIBLE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf9786200_a5bf_4a0f_8c24_fb16f5d1aabb);
pub const GUID_TS_SERVICE_ACTIVEX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea937a50_c9a6_4b7d_894a_49d99b784834);
pub const GUID_TS_SERVICE_DATAOBJECT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6086fbb5_e225_46ce_a770_c1bbd3e05d7b);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const GXFPF_NEAREST: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const GXFPF_ROUND_NEAREST: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HKL(pub isize);
impl ::core::default::Default for HKL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HKL {}
unsafe impl ::windows::runtime::Abi for HKL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAccClientDocMgr(pub ::windows::runtime::IUnknown);
impl IAccClientDocMgr {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn GetDocuments(&self) -> ::windows::runtime::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__: <super::super::System::Com::IEnumUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IEnumUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn LookupByHWND<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn LookupByPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINT>>(&self, pt: Param0, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pt.into_param().abi(), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetFocused(&self, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAccClientDocMgr {
    type Vtable = IAccClientDocMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4c896039_7b6d_49e6_a8c1_45116a98292b);
}
impl ::core::convert::From<IAccClientDocMgr> for ::windows::runtime::IUnknown {
    fn from(value: IAccClientDocMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAccClientDocMgr> for ::windows::runtime::IUnknown {
    fn from(value: &IAccClientDocMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAccClientDocMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAccClientDocMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccClientDocMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumunknown: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pt: super::super::Foundation::POINT, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAccDictionary(pub ::windows::runtime::IUnknown);
impl IAccDictionary {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetLocalizedString(&self, term: *const ::windows::runtime::GUID, lcid: u32, presult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(term), ::core::mem::transmute(lcid), ::core::mem::transmute(presult), ::core::mem::transmute(plcid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetParentTerm(&self, term: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(term), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetMnemonicString(&self, term: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(term), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn LookupMnemonicTerm<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmnemonic: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrmnemonic.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn ConvertValueToString<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, term: *const ::windows::runtime::GUID, lcid: u32, varvalue: Param2, pbstrresult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(term), ::core::mem::transmute(lcid), varvalue.into_param().abi(), ::core::mem::transmute(pbstrresult), ::core::mem::transmute(plcid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAccDictionary {
    type Vtable = IAccDictionary_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1dc4cb5f_d737_474d_ade9_5ccfc9bc1cc9);
}
impl ::core::convert::From<IAccDictionary> for ::windows::runtime::IUnknown {
    fn from(value: IAccDictionary) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAccDictionary> for ::windows::runtime::IUnknown {
    fn from(value: &IAccDictionary) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAccDictionary {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAccDictionary {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccDictionary_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, term: *const ::windows::runtime::GUID, lcid: u32, presult: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plcid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, term: *const ::windows::runtime::GUID, pparentterm: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, term: *const ::windows::runtime::GUID, presult: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrmnemonic: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pterm: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, term: *const ::windows::runtime::GUID, lcid: u32, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrresult: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plcid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAccServerDocMgr(pub ::windows::runtime::IUnknown);
impl IAccServerDocMgr {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn NewDocument<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punk: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RevokeDocument<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnDocumentFocus<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAccServerDocMgr {
    type Vtable = IAccServerDocMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xad7c73cf_6dd5_4855_abc2_b04bad5b9153);
}
impl ::core::convert::From<IAccServerDocMgr> for ::windows::runtime::IUnknown {
    fn from(value: IAccServerDocMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAccServerDocMgr> for ::windows::runtime::IUnknown {
    fn from(value: &IAccServerDocMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAccServerDocMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAccServerDocMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccServerDocMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAccStore(pub ::windows::runtime::IUnknown);
impl IAccStore {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Register<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punk: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Unregister<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn GetDocuments(&self) -> ::windows::runtime::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__: <super::super::System::Com::IEnumUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IEnumUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn LookupByHWND<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn LookupByPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINT>>(&self, pt: Param0, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pt.into_param().abi(), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnDocumentFocus<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetFocused(&self, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAccStore {
    type Vtable = IAccStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe2cd4a63_2b72_4d48_b739_95e4765195ba);
}
impl ::core::convert::From<IAccStore> for ::windows::runtime::IUnknown {
    fn from(value: IAccStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAccStore> for ::windows::runtime::IUnknown {
    fn from(value: &IAccStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAccStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAccStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumunknown: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pt: super::super::Foundation::POINT, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAnchor(pub ::windows::runtime::IUnknown);
impl IAnchor {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetGravity(&self, gravity: TsGravity) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(gravity)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGravity(&self) -> ::windows::runtime::Result<TsGravity> {
        let mut result__: <TsGravity as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TsGravity>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, pawith: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pawith.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Compare<'a, Param0: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, pawith: Param0) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pawith.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Shift<'a, Param3: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(cchreq), ::core::mem::transmute(pcch), pahaltanchor.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShiftTo<'a, Param0: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, pasite: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pasite.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn ShiftRegion(&self, dwflags: u32, dir: TsShiftDir) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dir), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetChangeHistoryMask(&self, dwmask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetChangeHistory(&self) -> ::windows::runtime::Result<ANCHOR_CHANGE_HISTORY_FLAGS> {
        let mut result__: <ANCHOR_CHANGE_HISTORY_FLAGS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ANCHOR_CHANGE_HISTORY_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ClearChangeHistory(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IAnchor> {
        let mut result__: <IAnchor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAnchor>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAnchor {
    type Vtable = IAnchor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0feb7e34_5a60_4356_8ef7_abdec2ff7cf8);
}
impl ::core::convert::From<IAnchor> for ::windows::runtime::IUnknown {
    fn from(value: IAnchor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAnchor> for ::windows::runtime::IUnknown {
    fn from(value: &IAnchor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAnchor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAnchor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnchor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gravity: TsGravity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgravity: *mut TsGravity) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pawith: ::windows::runtime::RawPtr, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pawith: ::windows::runtime::RawPtr, plresult: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pasite: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, dir: TsShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmask: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwhistory: *mut ANCHOR_CHANGE_HISTORY_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaclone: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IClonableWrapper(pub ::windows::runtime::IUnknown);
impl IClonableWrapper {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn CloneNewWrapper<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IClonableWrapper {
    type Vtable = IClonableWrapper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb33e75ff_e84c_4dca_a25c_33b8dc003374);
}
impl ::core::convert::From<IClonableWrapper> for ::windows::runtime::IUnknown {
    fn from(value: IClonableWrapper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IClonableWrapper> for ::windows::runtime::IUnknown {
    fn from(value: &IClonableWrapper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IClonableWrapper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IClonableWrapper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClonableWrapper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICoCreateLocally(pub ::windows::runtime::IUnknown);
impl ICoCreateLocally {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn CoCreateLocally<'a, Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param6: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, rclsid: *const ::windows::runtime::GUID, dwclscontext: u32, riid: *const ::windows::runtime::GUID, punk: *mut ::core::option::Option<::windows::runtime::IUnknown>, riidparam: *const ::windows::runtime::GUID, punkparam: Param5, varparam: Param6) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(riid), ::core::mem::transmute(punk), ::core::mem::transmute(riidparam), punkparam.into_param().abi(), varparam.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICoCreateLocally {
    type Vtable = ICoCreateLocally_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x03de00aa_f272_41e3_99cb_03c5e8114ea0);
}
impl ::core::convert::From<ICoCreateLocally> for ::windows::runtime::IUnknown {
    fn from(value: ICoCreateLocally) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICoCreateLocally> for ::windows::runtime::IUnknown {
    fn from(value: &ICoCreateLocally) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICoCreateLocally {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICoCreateLocally {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoCreateLocally_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, dwclscontext: u32, riid: *const ::windows::runtime::GUID, punk: *mut ::windows::runtime::RawPtr, riidparam: *const ::windows::runtime::GUID, punkparam: ::windows::runtime::RawPtr, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICoCreatedLocally(pub ::windows::runtime::IUnknown);
impl ICoCreatedLocally {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn LocalInit<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, punklocalobject: Param0, riidparam: *const ::windows::runtime::GUID, punkparam: Param2, varparam: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punklocalobject.into_param().abi(), ::core::mem::transmute(riidparam), punkparam.into_param().abi(), varparam.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICoCreatedLocally {
    type Vtable = ICoCreatedLocally_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0a53eb6c_1908_4742_8cff_2cee2e93f94c);
}
impl ::core::convert::From<ICoCreatedLocally> for ::windows::runtime::IUnknown {
    fn from(value: ICoCreatedLocally) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICoCreatedLocally> for ::windows::runtime::IUnknown {
    fn from(value: &ICoCreatedLocally) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICoCreatedLocally {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICoCreatedLocally {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoCreatedLocally_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punklocalobject: ::windows::runtime::RawPtr, riidparam: *const ::windows::runtime::GUID, punkparam: ::windows::runtime::RawPtr, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDocWrap(pub ::windows::runtime::IUnknown);
impl IDocWrap {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetDoc<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punk: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetWrappedDoc(&self, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDocWrap {
    type Vtable = IDocWrap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdcd285fe_0be0_43bd_99c9_aaaec513c555);
}
impl ::core::convert::From<IDocWrap> for ::windows::runtime::IUnknown {
    fn from(value: IDocWrap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDocWrap> for ::windows::runtime::IUnknown {
    fn from(value: &IDocWrap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDocWrap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDocWrap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDocWrap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumITfCompositionView(pub ::windows::runtime::IUnknown);
impl IEnumITfCompositionView {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumITfCompositionView> {
        let mut result__: <IEnumITfCompositionView as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumITfCompositionView>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, rgcompositionview: *mut ::core::option::Option<ITfCompositionView>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgcompositionview), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumITfCompositionView {
    type Vtable = IEnumITfCompositionView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5efd22ba_7838_46cb_88e2_cadb14124f8f);
}
impl ::core::convert::From<IEnumITfCompositionView> for ::windows::runtime::IUnknown {
    fn from(value: IEnumITfCompositionView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumITfCompositionView> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumITfCompositionView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumITfCompositionView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumITfCompositionView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumITfCompositionView_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, rgcompositionview: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumSpeechCommands(pub ::windows::runtime::IUnknown);
impl IEnumSpeechCommands {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSpeechCommands> {
        let mut result__: <IEnumSpeechCommands as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSpeechCommands>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pspcmds), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSpeechCommands {
    type Vtable = IEnumSpeechCommands_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8c5dac4f_083c_4b85_a4c9_71746048adca);
}
impl ::core::convert::From<IEnumSpeechCommands> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSpeechCommands) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumSpeechCommands> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSpeechCommands) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSpeechCommands {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumSpeechCommands {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSpeechCommands_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfCandidates(pub ::windows::runtime::IUnknown);
impl IEnumTfCandidates {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfCandidates> {
        let mut result__: <IEnumTfCandidates as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfCandidates>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, ppcand: *mut ::core::option::Option<ITfCandidateString>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppcand), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfCandidates {
    type Vtable = IEnumTfCandidates_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdefb1926_6c80_4ce8_87d4_d6b72b812bde);
}
impl ::core::convert::From<IEnumTfCandidates> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfCandidates) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfCandidates> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfCandidates) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfCandidates {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfCandidates {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfCandidates_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, ppcand: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfContextViews(pub ::windows::runtime::IUnknown);
impl IEnumTfContextViews {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfContextViews> {
        let mut result__: <IEnumTfContextViews as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfContextViews>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, rgviews: *mut ::core::option::Option<ITfContextView>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgviews), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfContextViews {
    type Vtable = IEnumTfContextViews_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf0c0f8dd_cf38_44e1_bb0f_68cf0d551c78);
}
impl ::core::convert::From<IEnumTfContextViews> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfContextViews) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfContextViews> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfContextViews) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfContextViews {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfContextViews {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfContextViews_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, rgviews: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfContexts(pub ::windows::runtime::IUnknown);
impl IEnumTfContexts {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfContexts> {
        let mut result__: <IEnumTfContexts as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfContexts>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, rgcontext: *mut ::core::option::Option<ITfContext>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgcontext), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfContexts {
    type Vtable = IEnumTfContexts_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8f1a7ea6_1654_4502_a86e_b2902344d507);
}
impl ::core::convert::From<IEnumTfContexts> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfContexts) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfContexts> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfContexts) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfContexts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfContexts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfContexts_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, rgcontext: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfDisplayAttributeInfo(pub ::windows::runtime::IUnknown);
impl IEnumTfDisplayAttributeInfo {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfDisplayAttributeInfo> {
        let mut result__: <IEnumTfDisplayAttributeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfDisplayAttributeInfo>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, rginfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rginfo), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfDisplayAttributeInfo {
    type Vtable = IEnumTfDisplayAttributeInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7cef04d7_cb75_4e80_a7ab_5f5bc7d332de);
}
impl ::core::convert::From<IEnumTfDisplayAttributeInfo> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfDisplayAttributeInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfDisplayAttributeInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfDisplayAttributeInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfDisplayAttributeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfDisplayAttributeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfDisplayAttributeInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, rginfo: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfDocumentMgrs(pub ::windows::runtime::IUnknown);
impl IEnumTfDocumentMgrs {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfDocumentMgrs> {
        let mut result__: <IEnumTfDocumentMgrs as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfDocumentMgrs>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, rgdocumentmgr: *mut ::core::option::Option<ITfDocumentMgr>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgdocumentmgr), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfDocumentMgrs {
    type Vtable = IEnumTfDocumentMgrs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e808_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<IEnumTfDocumentMgrs> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfDocumentMgrs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfDocumentMgrs> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfDocumentMgrs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfDocumentMgrs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfDocumentMgrs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfDocumentMgrs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, rgdocumentmgr: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfFunctionProviders(pub ::windows::runtime::IUnknown);
impl IEnumTfFunctionProviders {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfFunctionProviders> {
        let mut result__: <IEnumTfFunctionProviders as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfFunctionProviders>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, ppcmdobj: *mut ::core::option::Option<ITfFunctionProvider>, pcfetch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppcmdobj), ::core::mem::transmute(pcfetch)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfFunctionProviders {
    type Vtable = IEnumTfFunctionProviders_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe4b24db0_0990_11d3_8df0_00105a2799b5);
}
impl ::core::convert::From<IEnumTfFunctionProviders> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfFunctionProviders) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfFunctionProviders> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfFunctionProviders) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfFunctionProviders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfFunctionProviders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfFunctionProviders_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, ppcmdobj: *mut ::windows::runtime::RawPtr, pcfetch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfInputProcessorProfiles(pub ::windows::runtime::IUnknown);
impl IEnumTfInputProcessorProfiles {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfInputProcessorProfiles> {
        let mut result__: <IEnumTfInputProcessorProfiles as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfInputProcessorProfiles>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pprofile), ::core::mem::transmute(pcfetch)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfInputProcessorProfiles {
    type Vtable = IEnumTfInputProcessorProfiles_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x71c6e74d_0f28_11d8_a82a_00065b84435c);
}
impl ::core::convert::From<IEnumTfInputProcessorProfiles> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfInputProcessorProfiles) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfInputProcessorProfiles> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfInputProcessorProfiles) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfInputProcessorProfiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfInputProcessorProfiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfInputProcessorProfiles_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfLangBarItems(pub ::windows::runtime::IUnknown);
impl IEnumTfLangBarItems {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfLangBarItems> {
        let mut result__: <IEnumTfLangBarItems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfLangBarItems>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, ppitem: *mut ::core::option::Option<ITfLangBarItem>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppitem), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfLangBarItems {
    type Vtable = IEnumTfLangBarItems_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x583f34d0_de25_11d2_afdd_00105a2799b5);
}
impl ::core::convert::From<IEnumTfLangBarItems> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfLangBarItems) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfLangBarItems> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfLangBarItems) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfLangBarItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfLangBarItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfLangBarItems_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, ppitem: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfLanguageProfiles(pub ::windows::runtime::IUnknown);
impl IEnumTfLanguageProfiles {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfLanguageProfiles> {
        let mut result__: <IEnumTfLanguageProfiles as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfLanguageProfiles>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Next(&self, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pprofile), ::core::mem::transmute(pcfetch)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfLanguageProfiles {
    type Vtable = IEnumTfLanguageProfiles_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3d61bf11_ac5f_42c8_a4cb_931bcc28c744);
}
impl ::core::convert::From<IEnumTfLanguageProfiles> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfLanguageProfiles) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfLanguageProfiles> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfLanguageProfiles) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfLanguageProfiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfLanguageProfiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfLanguageProfiles_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfLatticeElements(pub ::windows::runtime::IUnknown);
impl IEnumTfLatticeElements {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfLatticeElements> {
        let mut result__: <IEnumTfLatticeElements as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfLatticeElements>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Next(&self, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgselements), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfLatticeElements {
    type Vtable = IEnumTfLatticeElements_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x56988052_47da_4a05_911a_e3d941f17145);
}
impl ::core::convert::From<IEnumTfLatticeElements> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfLatticeElements) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfLatticeElements> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfLatticeElements) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfLatticeElements {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfLatticeElements {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfLatticeElements_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, rgselements: *mut ::core::mem::ManuallyDrop<TF_LMLATTELEMENT>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfProperties(pub ::windows::runtime::IUnknown);
impl IEnumTfProperties {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfProperties> {
        let mut result__: <IEnumTfProperties as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfProperties>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, ppprop: *mut ::core::option::Option<ITfProperty>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppprop), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfProperties {
    type Vtable = IEnumTfProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x19188cb0_aca9_11d2_afc5_00105a2799b5);
}
impl ::core::convert::From<IEnumTfProperties> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, ppprop: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfPropertyValue(pub ::windows::runtime::IUnknown);
impl IEnumTfPropertyValue {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfPropertyValue> {
        let mut result__: <IEnumTfPropertyValue as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfPropertyValue>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Next(&self, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgvalues), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfPropertyValue {
    type Vtable = IEnumTfPropertyValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8ed8981b_7c10_4d7d_9fb3_ab72e9c75f72);
}
impl ::core::convert::From<IEnumTfPropertyValue> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfPropertyValue) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfPropertyValue> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfPropertyValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfPropertyValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfPropertyValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfPropertyValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, rgvalues: *mut ::core::mem::ManuallyDrop<TF_PROPERTYVAL>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfRanges(pub ::windows::runtime::IUnknown);
impl IEnumTfRanges {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfRanges> {
        let mut result__: <IEnumTfRanges as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfRanges>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, pprange: *mut ::core::option::Option<ITfRange>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pprange), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfRanges {
    type Vtable = IEnumTfRanges_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf99d3f40_8e32_11d2_bf46_00105a2799b5);
}
impl ::core::convert::From<IEnumTfRanges> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfRanges) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfRanges> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfRanges) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfRanges {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfRanges {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfRanges_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, pprange: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumTfUIElements(pub ::windows::runtime::IUnknown);
impl IEnumTfUIElements {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumTfUIElements> {
        let mut result__: <IEnumTfUIElements as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfUIElements>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Next(&self, ulcount: u32, ppelement: *mut ::core::option::Option<ITfUIElement>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppelement), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumTfUIElements {
    type Vtable = IEnumTfUIElements_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x887aa91e_acba_4931_84da_3c5208cf543f);
}
impl ::core::convert::From<IEnumTfUIElements> for ::windows::runtime::IUnknown {
    fn from(value: IEnumTfUIElements) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumTfUIElements> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumTfUIElements) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumTfUIElements {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumTfUIElements {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfUIElements_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, ppelement: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternalDocWrap(pub ::windows::runtime::IUnknown);
impl IInternalDocWrap {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn NotifyRevoke(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInternalDocWrap {
    type Vtable = IInternalDocWrap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe1aa6466_9db4_40ba_be03_77c38e8e60b2);
}
impl ::core::convert::From<IInternalDocWrap> for ::windows::runtime::IUnknown {
    fn from(value: IInternalDocWrap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternalDocWrap> for ::windows::runtime::IUnknown {
    fn from(value: &IInternalDocWrap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInternalDocWrap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInternalDocWrap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalDocWrap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const ILMCM_CHECKLAYOUTANDTIPENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const ILMCM_LANGUAGEBAROFF: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INSERT_TEXT_AT_SELECTION_FLAGS(pub u32);
pub const TF_IAS_NOQUERY: INSERT_TEXT_AT_SELECTION_FLAGS = INSERT_TEXT_AT_SELECTION_FLAGS(1u32);
pub const TF_IAS_QUERYONLY: INSERT_TEXT_AT_SELECTION_FLAGS = INSERT_TEXT_AT_SELECTION_FLAGS(2u32);
pub const TF_IAS_NO_DEFAULT_COMPOSITION: INSERT_TEXT_AT_SELECTION_FLAGS = INSERT_TEXT_AT_SELECTION_FLAGS(2147483648u32);
impl ::core::convert::From<u32> for INSERT_TEXT_AT_SELECTION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INSERT_TEXT_AT_SELECTION_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for INSERT_TEXT_AT_SELECTION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for INSERT_TEXT_AT_SELECTION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for INSERT_TEXT_AT_SELECTION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for INSERT_TEXT_AT_SELECTION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for INSERT_TEXT_AT_SELECTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpeechCommandProvider(pub ::windows::runtime::IUnknown);
impl ISpeechCommandProvider {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumSpeechCommands(&self, langid: u16) -> ::windows::runtime::Result<IEnumSpeechCommands> {
        let mut result__: <IEnumSpeechCommands as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), &mut result__).from_abi::<IEnumSpeechCommands>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn ProcessCommand<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszcommand: Param0, cch: u32, langid: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszcommand.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(langid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISpeechCommandProvider {
    type Vtable = ISpeechCommandProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x38e09d4c_586d_435a_b592_c8a86691dec6);
}
impl ::core::convert::From<ISpeechCommandProvider> for ::windows::runtime::IUnknown {
    fn from(value: ISpeechCommandProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpeechCommandProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ISpeechCommandProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpeechCommandProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpeechCommandProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechCommandProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcommand: super::super::Foundation::PWSTR, cch: u32, langid: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStoreACP(pub ::windows::runtime::IUnknown);
impl ITextStoreACP {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn AdviseSink<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punk: Param1, dwmask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwmask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnadviseSink<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let mut result__: <::windows::runtime::HRESULT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<TS_STATUS> {
        let mut result__: <TS_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TS_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpteststart), ::core::mem::transmute(acptestend), ::core::mem::transmute(cch), ::core::mem::transmute(pacpresultstart), ::core::mem::transmute(pacpresultend)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection), ::core::mem::transmute(pcfetched)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetText(&self, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(acpstart),
            ::core::mem::transmute(acpend),
            ::core::mem::transmute(pchplain),
            ::core::mem::transmute(cchplainreq),
            ::core::mem::transmute(pcchplainret),
            ::core::mem::transmute(prgruninfo),
            ::core::mem::transmute(cruninforeq),
            ::core::mem::transmute(pcruninforet),
            ::core::mem::transmute(pacpnext),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetText<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: Param3, cch: u32) -> ::windows::runtime::Result<TS_TEXTCHANGE> {
        let mut result__: <TS_TEXTCHANGE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), pchtext.into_param().abi(), ::core::mem::transmute(cch), &mut result__).from_abi::<TS_TEXTCHANGE>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> ::windows::runtime::Result<super::super::System::Com::IDataObject> {
        let mut result__: <super::super::System::Com::IDataObject as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), &mut result__).from_abi::<super::super::System::Com::IDataObject>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetEmbedded(&self, acppos: i32, rguidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(acppos), ::core::mem::transmute(rguidservice), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const ::windows::runtime::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidservice), ::core::mem::transmute(pformatetc), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn InsertEmbedded<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: Param3) -> ::windows::runtime::Result<TS_TEXTCHANGE> {
        let mut result__: <TS_TEXTCHANGE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), pdataobject.into_param().abi(), &mut result__).from_abi::<TS_TEXTCHANGE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn InsertTextAtSelection<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, pchtext: Param1, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pchtext.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(pacpstart), ::core::mem::transmute(pacpend), ::core::mem::transmute(pchange)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn InsertEmbeddedAtSelection<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dwflags: u32, pdataobject: Param1, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pdataobject.into_param().abi(), ::core::mem::transmute(pacpstart), ::core::mem::transmute(pacpend), ::core::mem::transmute(pchange)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(acppos), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(acppos), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(acpstart),
            ::core::mem::transmute(acphalt),
            ::core::mem::transmute(cfilterattrs),
            ::core::mem::transmute(pafilterattrs),
            ::core::mem::transmute(dwflags),
            ::core::mem::transmute(pacpnext),
            ::core::mem::transmute(pffound),
            ::core::mem::transmute(plfoundoffset),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(paattrvals), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetEndACP(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetActiveView(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(ptscreen), ::core::mem::transmute(dwflags), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(prc), ::core::mem::transmute(pfclipped)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetScreenExt(&self, vcview: u32) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetWnd(&self, vcview: u32) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITextStoreACP {
    type Vtable = ITextStoreACP_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x28888fe3_c2a0_483a_a3ea_8cb1ce51ff3d);
}
impl ::core::convert::From<ITextStoreACP> for ::windows::runtime::IUnknown {
    fn from(value: ITextStoreACP) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStoreACP> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStoreACP) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStoreACP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStoreACP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACP_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr, dwmask: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlockflags: u32, phrsession: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdcs: *mut TS_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, acpstart: i32, acpend: i32, pchtext: super::super::Foundation::PWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpstart: i32, acpend: i32, ppdataobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acppos: i32, rguidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidservice: *const ::windows::runtime::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::windows::runtime::RawPtr, pchange: *mut TS_TEXTCHANGE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pdataobject: ::windows::runtime::RawPtr, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, paattrvals: *mut ::core::mem::ManuallyDrop<TS_ATTRVAL>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pacp: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvcview: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStoreACP2(pub ::windows::runtime::IUnknown);
impl ITextStoreACP2 {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn AdviseSink<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punk: Param1, dwmask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwmask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnadviseSink<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let mut result__: <::windows::runtime::HRESULT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<TS_STATUS> {
        let mut result__: <TS_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TS_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpteststart), ::core::mem::transmute(acptestend), ::core::mem::transmute(cch), ::core::mem::transmute(pacpresultstart), ::core::mem::transmute(pacpresultend)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection), ::core::mem::transmute(pcfetched)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetText(&self, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(acpstart),
            ::core::mem::transmute(acpend),
            ::core::mem::transmute(pchplain),
            ::core::mem::transmute(cchplainreq),
            ::core::mem::transmute(pcchplainret),
            ::core::mem::transmute(prgruninfo),
            ::core::mem::transmute(cruninforeq),
            ::core::mem::transmute(pcruninforet),
            ::core::mem::transmute(pacpnext),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetText<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: Param3, cch: u32) -> ::windows::runtime::Result<TS_TEXTCHANGE> {
        let mut result__: <TS_TEXTCHANGE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), pchtext.into_param().abi(), ::core::mem::transmute(cch), &mut result__).from_abi::<TS_TEXTCHANGE>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> ::windows::runtime::Result<super::super::System::Com::IDataObject> {
        let mut result__: <super::super::System::Com::IDataObject as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), &mut result__).from_abi::<super::super::System::Com::IDataObject>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetEmbedded(&self, acppos: i32, rguidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(acppos), ::core::mem::transmute(rguidservice), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const ::windows::runtime::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidservice), ::core::mem::transmute(pformatetc), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn InsertEmbedded<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: Param3) -> ::windows::runtime::Result<TS_TEXTCHANGE> {
        let mut result__: <TS_TEXTCHANGE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), pdataobject.into_param().abi(), &mut result__).from_abi::<TS_TEXTCHANGE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn InsertTextAtSelection<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, pchtext: Param1, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pchtext.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(pacpstart), ::core::mem::transmute(pacpend), ::core::mem::transmute(pchange)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn InsertEmbeddedAtSelection<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dwflags: u32, pdataobject: Param1, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pdataobject.into_param().abi(), ::core::mem::transmute(pacpstart), ::core::mem::transmute(pacpend), ::core::mem::transmute(pchange)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(acppos), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(acppos), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(acpstart),
            ::core::mem::transmute(acphalt),
            ::core::mem::transmute(cfilterattrs),
            ::core::mem::transmute(pafilterattrs),
            ::core::mem::transmute(dwflags),
            ::core::mem::transmute(pacpnext),
            ::core::mem::transmute(pffound),
            ::core::mem::transmute(plfoundoffset),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(paattrvals), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetEndACP(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetActiveView(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(ptscreen), ::core::mem::transmute(dwflags), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(prc), ::core::mem::transmute(pfclipped)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetScreenExt(&self, vcview: u32) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITextStoreACP2 {
    type Vtable = ITextStoreACP2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf86ad89f_5fe4_4b8d_bb9f_ef3797a84f1f);
}
impl ::core::convert::From<ITextStoreACP2> for ::windows::runtime::IUnknown {
    fn from(value: ITextStoreACP2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStoreACP2> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStoreACP2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStoreACP2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStoreACP2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACP2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr, dwmask: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlockflags: u32, phrsession: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdcs: *mut TS_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, acpstart: i32, acpend: i32, pchtext: super::super::Foundation::PWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpstart: i32, acpend: i32, ppdataobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acppos: i32, rguidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidservice: *const ::windows::runtime::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::windows::runtime::RawPtr, pchange: *mut TS_TEXTCHANGE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pdataobject: ::windows::runtime::RawPtr, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, paattrvals: *mut ::core::mem::ManuallyDrop<TS_ATTRVAL>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pacp: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvcview: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStoreACPEx(pub ::windows::runtime::IUnknown);
impl ITextStoreACPEx {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn ScrollToRect<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>>(&self, acpstart: i32, acpend: i32, rc: Param2, dwposition: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), rc.into_param().abi(), ::core::mem::transmute(dwposition)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextStoreACPEx {
    type Vtable = ITextStoreACPEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa2de3bc2_3d8e_11d3_81a9_f753fbe61a00);
}
impl ::core::convert::From<ITextStoreACPEx> for ::windows::runtime::IUnknown {
    fn from(value: ITextStoreACPEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStoreACPEx> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStoreACPEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStoreACPEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStoreACPEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStoreACPServices(pub ::windows::runtime::IUnknown);
impl ITextStoreACPServices {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn Serialize<'a, Param0: ::windows::runtime::IntoParam<'a, ITfProperty>, Param1: ::windows::runtime::IntoParam<'a, ITfRange>, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pprop: Param0, prange: Param1, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pprop.into_param().abi(), prange.into_param().abi(), ::core::mem::transmute(phdr), pstream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn Unserialize<'a, Param0: ::windows::runtime::IntoParam<'a, ITfProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param3: ::windows::runtime::IntoParam<'a, ITfPersistentPropertyLoaderACP>>(&self, pprop: Param0, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Param2, ploader: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pprop.into_param().abi(), ::core::mem::transmute(phdr), pstream.into_param().abi(), ploader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ForceLoadProperty<'a, Param0: ::windows::runtime::IntoParam<'a, ITfProperty>>(&self, pprop: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn CreateRange(&self, acpstart: i32, acpend: i32) -> ::windows::runtime::Result<ITfRangeACP> {
        let mut result__: <ITfRangeACP as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), &mut result__).from_abi::<ITfRangeACP>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITextStoreACPServices {
    type Vtable = ITextStoreACPServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e901_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<ITextStoreACPServices> for ::windows::runtime::IUnknown {
    fn from(value: ITextStoreACPServices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStoreACPServices> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStoreACPServices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStoreACPServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStoreACPServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprop: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprop: ::windows::runtime::RawPtr, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::runtime::RawPtr, ploader: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprop: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpstart: i32, acpend: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStoreACPSink(pub ::windows::runtime::IUnknown);
impl ITextStoreACPSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnTextChange(&self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(pchange)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnSelectionChange(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcode), ::core::mem::transmute(vcview)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnAttrsChange(&self, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(cattrs), ::core::mem::transmute(paattrs)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextStoreACPSink {
    type Vtable = ITextStoreACPSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x22d44c94_a419_4542_a272_ae26093ececf);
}
impl ::core::convert::From<ITextStoreACPSink> for ::windows::runtime::IUnknown {
    fn from(value: ITextStoreACPSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStoreACPSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStoreACPSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStoreACPSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStoreACPSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcode: TsLayoutCode, vcview: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStoreACPSinkEx(pub ::windows::runtime::IUnknown);
impl ITextStoreACPSinkEx {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnTextChange(&self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(pchange)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnSelectionChange(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcode), ::core::mem::transmute(vcview)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnAttrsChange(&self, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(cattrs), ::core::mem::transmute(paattrs)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnDisconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextStoreACPSinkEx {
    type Vtable = ITextStoreACPSinkEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2bdf9464_41e2_43e3_950c_a6865ba25cd4);
}
impl ::core::convert::From<ITextStoreACPSinkEx> for ::windows::runtime::IUnknown {
    fn from(value: ITextStoreACPSinkEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStoreACPSinkEx> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStoreACPSinkEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStoreACPSinkEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStoreACPSinkEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITextStoreACPSinkEx> for ITextStoreACPSink {
    fn from(value: ITextStoreACPSinkEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoreACPSinkEx> for ITextStoreACPSink {
    fn from(value: &ITextStoreACPSinkEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextStoreACPSink> for ITextStoreACPSinkEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextStoreACPSink> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextStoreACPSink> for &ITextStoreACPSinkEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextStoreACPSink> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPSinkEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcode: TsLayoutCode, vcview: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStoreAnchor(pub ::windows::runtime::IUnknown);
impl ITextStoreAnchor {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn AdviseSink<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punk: Param1, dwmask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwmask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnadviseSink<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let mut result__: <::windows::runtime::HRESULT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<TS_STATUS> {
        let mut result__: <TS_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TS_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn QueryInsert<'a, Param0: ::windows::runtime::IntoParam<'a, IAnchor>, Param1: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, pateststart: Param0, patestend: Param1, cch: u32, pparesultstart: *mut ::core::option::Option<IAnchor>, pparesultend: *mut ::core::option::Option<IAnchor>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pateststart.into_param().abi(), patestend.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(pparesultstart), ::core::mem::transmute(pparesultend)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection), ::core::mem::transmute(pcfetched)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetText<'a, Param1: ::windows::runtime::IntoParam<'a, IAnchor>, Param2: ::windows::runtime::IntoParam<'a, IAnchor>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwflags: u32, pastart: Param1, paend: Param2, pchtext: super::super::Foundation::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: Param6) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pastart.into_param().abi(), paend.into_param().abi(), ::core::mem::transmute(pchtext), ::core::mem::transmute(cchreq), ::core::mem::transmute(pcch), fupdateanchor.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetText<'a, Param1: ::windows::runtime::IntoParam<'a, IAnchor>, Param2: ::windows::runtime::IntoParam<'a, IAnchor>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, pastart: Param1, paend: Param2, pchtext: Param3, cch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pastart.into_param().abi(), paend.into_param().abi(), pchtext.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn GetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, IAnchor>, Param1: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, pastart: Param0, paend: Param1) -> ::windows::runtime::Result<super::super::System::Com::IDataObject> {
        let mut result__: <super::super::System::Com::IDataObject as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pastart.into_param().abi(), paend.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::IDataObject>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetEmbedded<'a, Param1: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, dwflags: u32, papos: Param1, rguidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), papos.into_param().abi(), ::core::mem::transmute(rguidservice), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn InsertEmbedded<'a, Param1: ::windows::runtime::IntoParam<'a, IAnchor>, Param2: ::windows::runtime::IntoParam<'a, IAnchor>, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dwflags: u32, pastart: Param1, paend: Param2, pdataobject: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pastart.into_param().abi(), paend.into_param().abi(), pdataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestAttrsAtPosition<'a, Param0: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, papos: Param0, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), papos.into_param().abi(), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestAttrsTransitioningAtPosition<'a, Param0: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, papos: Param0, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), papos.into_param().abi(), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn FindNextAttrTransition<'a, Param0: ::windows::runtime::IntoParam<'a, IAnchor>, Param1: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, pastart: Param0, pahalt: Param1, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pastart.into_param().abi(), pahalt.into_param().abi(), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags), ::core::mem::transmute(pffound), ::core::mem::transmute(plfoundoffset)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(paattrvals), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStart(&self) -> ::windows::runtime::Result<IAnchor> {
        let mut result__: <IAnchor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAnchor>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetEnd(&self) -> ::windows::runtime::Result<IAnchor> {
        let mut result__: <IAnchor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAnchor>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetActiveView(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetAnchorFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::runtime::Result<IAnchor> {
        let mut result__: <IAnchor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(ptscreen), ::core::mem::transmute(dwflags), &mut result__).from_abi::<IAnchor>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTextExt<'a, Param1: ::windows::runtime::IntoParam<'a, IAnchor>, Param2: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, vcview: u32, pastart: Param1, paend: Param2, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), pastart.into_param().abi(), paend.into_param().abi(), ::core::mem::transmute(prc), ::core::mem::transmute(pfclipped)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetScreenExt(&self, vcview: u32) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetWnd(&self, vcview: u32) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const ::windows::runtime::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidservice), ::core::mem::transmute(pformatetc), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn InsertTextAtSelection<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, pchtext: Param1, cch: u32, ppastart: *mut ::core::option::Option<IAnchor>, ppaend: *mut ::core::option::Option<IAnchor>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pchtext.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(ppastart), ::core::mem::transmute(ppaend)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn InsertEmbeddedAtSelection<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dwflags: u32, pdataobject: Param1, ppastart: *mut ::core::option::Option<IAnchor>, ppaend: *mut ::core::option::Option<IAnchor>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pdataobject.into_param().abi(), ::core::mem::transmute(ppastart), ::core::mem::transmute(ppaend)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextStoreAnchor {
    type Vtable = ITextStoreAnchor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9b2077b0_5f18_4dec_bee9_3cc722f5dfe0);
}
impl ::core::convert::From<ITextStoreAnchor> for ::windows::runtime::IUnknown {
    fn from(value: ITextStoreAnchor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStoreAnchor> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStoreAnchor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStoreAnchor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStoreAnchor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreAnchor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr, dwmask: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlockflags: u32, phrsession: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdcs: *mut TS_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pateststart: ::windows::runtime::RawPtr, patestend: ::windows::runtime::RawPtr, cch: u32, pparesultstart: *mut ::windows::runtime::RawPtr, pparesultend: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulindex: u32, ulcount: u32, pselection: *mut ::core::mem::ManuallyDrop<TS_SELECTION_ANCHOR>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, pselection: *const ::core::mem::ManuallyDrop<TS_SELECTION_ANCHOR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pastart: ::windows::runtime::RawPtr, paend: ::windows::runtime::RawPtr, pchtext: super::super::Foundation::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pastart: ::windows::runtime::RawPtr, paend: ::windows::runtime::RawPtr, pchtext: super::super::Foundation::PWSTR, cch: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pastart: ::windows::runtime::RawPtr, paend: ::windows::runtime::RawPtr, ppdataobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, papos: ::windows::runtime::RawPtr, rguidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pastart: ::windows::runtime::RawPtr, paend: ::windows::runtime::RawPtr, pdataobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, papos: ::windows::runtime::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, papos: ::windows::runtime::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pastart: ::windows::runtime::RawPtr, pahalt: ::windows::runtime::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::runtime::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, paattrvals: *mut ::core::mem::ManuallyDrop<TS_ATTRVAL>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppastart: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaend: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvcview: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, ppasite: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcview: u32, pastart: ::windows::runtime::RawPtr, paend: ::windows::runtime::RawPtr, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidservice: *const ::windows::runtime::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, ppastart: *mut ::windows::runtime::RawPtr, ppaend: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pdataobject: ::windows::runtime::RawPtr, ppastart: *mut ::windows::runtime::RawPtr, ppaend: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStoreAnchorEx(pub ::windows::runtime::IUnknown);
impl ITextStoreAnchorEx {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn ScrollToRect<'a, Param0: ::windows::runtime::IntoParam<'a, IAnchor>, Param1: ::windows::runtime::IntoParam<'a, IAnchor>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>>(&self, pstart: Param0, pend: Param1, rc: Param2, dwposition: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pstart.into_param().abi(), pend.into_param().abi(), rc.into_param().abi(), ::core::mem::transmute(dwposition)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextStoreAnchorEx {
    type Vtable = ITextStoreAnchorEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa2de3bc1_3d8e_11d3_81a9_f753fbe61a00);
}
impl ::core::convert::From<ITextStoreAnchorEx> for ::windows::runtime::IUnknown {
    fn from(value: ITextStoreAnchorEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStoreAnchorEx> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStoreAnchorEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStoreAnchorEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStoreAnchorEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreAnchorEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstart: ::windows::runtime::RawPtr, pend: ::windows::runtime::RawPtr, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStoreAnchorSink(pub ::windows::runtime::IUnknown);
impl ITextStoreAnchorSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnTextChange<'a, Param1: ::windows::runtime::IntoParam<'a, IAnchor>, Param2: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: Param1, paend: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pastart.into_param().abi(), paend.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnSelectionChange(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcode), ::core::mem::transmute(vcview)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnAttrsChange<'a, Param0: ::windows::runtime::IntoParam<'a, IAnchor>, Param1: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, pastart: Param0, paend: Param1, cattrs: u32, paattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pastart.into_param().abi(), paend.into_param().abi(), ::core::mem::transmute(cattrs), ::core::mem::transmute(paattrs)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextStoreAnchorSink {
    type Vtable = ITextStoreAnchorSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e905_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<ITextStoreAnchorSink> for ::windows::runtime::IUnknown {
    fn from(value: ITextStoreAnchorSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStoreAnchorSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStoreAnchorSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStoreAnchorSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStoreAnchorSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreAnchorSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: ::windows::runtime::RawPtr, paend: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcode: TsLayoutCode, vcview: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pastart: ::windows::runtime::RawPtr, paend: ::windows::runtime::RawPtr, cattrs: u32, paattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITextStoreSinkAnchorEx(pub ::windows::runtime::IUnknown);
impl ITextStoreSinkAnchorEx {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnTextChange<'a, Param1: ::windows::runtime::IntoParam<'a, IAnchor>, Param2: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: Param1, paend: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pastart.into_param().abi(), paend.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnSelectionChange(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcode), ::core::mem::transmute(vcview)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnAttrsChange<'a, Param0: ::windows::runtime::IntoParam<'a, IAnchor>, Param1: ::windows::runtime::IntoParam<'a, IAnchor>>(&self, pastart: Param0, paend: Param1, cattrs: u32, paattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pastart.into_param().abi(), paend.into_param().abi(), ::core::mem::transmute(cattrs), ::core::mem::transmute(paattrs)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnDisconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextStoreSinkAnchorEx {
    type Vtable = ITextStoreSinkAnchorEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x25642426_028d_4474_977b_111bb114fe3e);
}
impl ::core::convert::From<ITextStoreSinkAnchorEx> for ::windows::runtime::IUnknown {
    fn from(value: ITextStoreSinkAnchorEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITextStoreSinkAnchorEx> for ::windows::runtime::IUnknown {
    fn from(value: &ITextStoreSinkAnchorEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextStoreSinkAnchorEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextStoreSinkAnchorEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITextStoreSinkAnchorEx> for ITextStoreAnchorSink {
    fn from(value: ITextStoreSinkAnchorEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoreSinkAnchorEx> for ITextStoreAnchorSink {
    fn from(value: &ITextStoreSinkAnchorEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextStoreAnchorSink> for ITextStoreSinkAnchorEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextStoreAnchorSink> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextStoreAnchorSink> for &ITextStoreSinkAnchorEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextStoreAnchorSink> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreSinkAnchorEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: ::windows::runtime::RawPtr, paend: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcode: TsLayoutCode, vcview: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pastart: ::windows::runtime::RawPtr, paend: ::windows::runtime::RawPtr, cattrs: u32, paattrs: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfActiveLanguageProfileNotifySink(pub ::windows::runtime::IUnknown);
impl ITfActiveLanguageProfileNotifySink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnActivated<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, clsid: *const ::windows::runtime::GUID, guidprofile: *const ::windows::runtime::GUID, factivated: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), ::core::mem::transmute(guidprofile), factivated.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfActiveLanguageProfileNotifySink {
    type Vtable = ITfActiveLanguageProfileNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb246cb75_a93e_4652_bf8c_b3fe0cfd7e57);
}
impl ::core::convert::From<ITfActiveLanguageProfileNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: ITfActiveLanguageProfileNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfActiveLanguageProfileNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfActiveLanguageProfileNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfActiveLanguageProfileNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfActiveLanguageProfileNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfActiveLanguageProfileNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, guidprofile: *const ::windows::runtime::GUID, factivated: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCandidateList(pub ::windows::runtime::IUnknown);
impl ITfCandidateList {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumCandidates(&self) -> ::windows::runtime::Result<IEnumTfCandidates> {
        let mut result__: <IEnumTfCandidates as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfCandidates>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetCandidate(&self, nindex: u32) -> ::windows::runtime::Result<ITfCandidateString> {
        let mut result__: <ITfCandidateString as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), &mut result__).from_abi::<ITfCandidateString>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetCandidateNum(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetResult(&self, nindex: u32, imcr: TfCandidateResult) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(imcr)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfCandidateList {
    type Vtable = ITfCandidateList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa3ad50fb_9bdb_49e3_a843_6c76520fbf5d);
}
impl ::core::convert::From<ITfCandidateList> for ::windows::runtime::IUnknown {
    fn from(value: ITfCandidateList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCandidateList> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCandidateList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCandidateList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCandidateList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nindex: u32, ppcand: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pncnt: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nindex: u32, imcr: TfCandidateResult) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCandidateListUIElement(pub ::windows::runtime::IUnknown);
impl ITfCandidateListUIElement {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGUID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsShown(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetUpdatedFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::runtime::Result<ITfDocumentMgr> {
        let mut result__: <ITfDocumentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfDocumentMgr>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetSelection(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetString(&self, uindex: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetPageIndex(&self, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pindex), ::core::mem::transmute(usize), ::core::mem::transmute(pupagecnt)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetPageIndex(&self, pindex: *const u32, upagecnt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pindex), ::core::mem::transmute(upagecnt)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetCurrentPage(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfCandidateListUIElement {
    type Vtable = ITfCandidateListUIElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea1ea138_19df_11d7_a6d2_00065b84435c);
}
impl ::core::convert::From<ITfCandidateListUIElement> for ::windows::runtime::IUnknown {
    fn from(value: ITfCandidateListUIElement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCandidateListUIElement> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCandidateListUIElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCandidateListUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCandidateListUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfCandidateListUIElement> for ITfUIElement {
    fn from(value: ITfCandidateListUIElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCandidateListUIElement> for ITfUIElement {
    fn from(value: &ITfCandidateListUIElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfUIElement> for ITfCandidateListUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfUIElement> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfUIElement> for &ITfCandidateListUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfUIElement> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateListUIElement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdim: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pucount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puindex: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uindex: u32, pstr: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pindex: *const u32, upagecnt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pupage: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCandidateListUIElementBehavior(pub ::windows::runtime::IUnknown);
impl ITfCandidateListUIElementBehavior {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGUID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsShown(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetUpdatedFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::runtime::Result<ITfDocumentMgr> {
        let mut result__: <ITfDocumentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfDocumentMgr>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetSelection(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetString(&self, uindex: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetPageIndex(&self, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pindex), ::core::mem::transmute(usize), ::core::mem::transmute(pupagecnt)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetPageIndex(&self, pindex: *const u32, upagecnt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pindex), ::core::mem::transmute(upagecnt)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetCurrentPage(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetSelection(&self, nindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Finalize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Abort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfCandidateListUIElementBehavior {
    type Vtable = ITfCandidateListUIElementBehavior_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x85fad185_58ce_497a_9460_355366b64b9a);
}
impl ::core::convert::From<ITfCandidateListUIElementBehavior> for ::windows::runtime::IUnknown {
    fn from(value: ITfCandidateListUIElementBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCandidateListUIElementBehavior> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCandidateListUIElementBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCandidateListUIElementBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCandidateListUIElementBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfCandidateListUIElementBehavior> for ITfCandidateListUIElement {
    fn from(value: ITfCandidateListUIElementBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCandidateListUIElementBehavior> for ITfCandidateListUIElement {
    fn from(value: &ITfCandidateListUIElementBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfCandidateListUIElement> for ITfCandidateListUIElementBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfCandidateListUIElement> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfCandidateListUIElement> for &ITfCandidateListUIElementBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfCandidateListUIElement> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfCandidateListUIElementBehavior> for ITfUIElement {
    fn from(value: ITfCandidateListUIElementBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCandidateListUIElementBehavior> for ITfUIElement {
    fn from(value: &ITfCandidateListUIElementBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfUIElement> for ITfCandidateListUIElementBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfUIElement> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfUIElement> for &ITfCandidateListUIElementBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfUIElement> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateListUIElementBehavior_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdim: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pucount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puindex: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uindex: u32, pstr: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pindex: *const u32, upagecnt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pupage: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCandidateString(pub ::windows::runtime::IUnknown);
impl ITfCandidateString {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetString(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetIndex(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfCandidateString {
    type Vtable = ITfCandidateString_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x581f317e_fd9d_443f_b972_ed00467c5d40);
}
impl ::core::convert::From<ITfCandidateString> for ::windows::runtime::IUnknown {
    fn from(value: ITfCandidateString) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCandidateString> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCandidateString) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCandidateString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCandidateString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateString_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstr: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnindex: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCategoryMgr(pub ::windows::runtime::IUnknown);
impl ITfCategoryMgr {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RegisterCategory(&self, rclsid: *const ::windows::runtime::GUID, rcatid: *const ::windows::runtime::GUID, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rcatid), ::core::mem::transmute(rguid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnregisterCategory(&self, rclsid: *const ::windows::runtime::GUID, rcatid: *const ::windows::runtime::GUID, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rcatid), ::core::mem::transmute(rguid)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn EnumCategoriesInItem(&self, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::System::Com::IEnumGUID> {
        let mut result__: <super::super::System::Com::IEnumGUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), &mut result__).from_abi::<super::super::System::Com::IEnumGUID>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn EnumItemsInCategory(&self, rcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::System::Com::IEnumGUID> {
        let mut result__: <super::super::System::Com::IEnumGUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rcatid), &mut result__).from_abi::<super::super::System::Com::IEnumGUID>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn FindClosestCategory(&self, rguid: *const ::windows::runtime::GUID, pcatid: *mut ::windows::runtime::GUID, ppcatidlist: *const *const ::windows::runtime::GUID, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(pcatid), ::core::mem::transmute(ppcatidlist), ::core::mem::transmute(ulcount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn RegisterGUIDDescription<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rclsid: *const ::windows::runtime::GUID, rguid: *const ::windows::runtime::GUID, pchdesc: Param2, cch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rguid), pchdesc.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnregisterGUIDDescription(&self, rclsid: *const ::windows::runtime::GUID, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rguid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetGUIDDescription(&self, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RegisterGUIDDWORD(&self, rclsid: *const ::windows::runtime::GUID, rguid: *const ::windows::runtime::GUID, dw: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rguid), ::core::mem::transmute(dw)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnregisterGUIDDWORD(&self, rclsid: *const ::windows::runtime::GUID, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rguid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGUIDDWORD(&self, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RegisterGUID(&self, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGUID(&self, guidatom: u32) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidatom), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsEqualTfGuidAtom(&self, guidatom: u32, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidatom), ::core::mem::transmute(rguid), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfCategoryMgr {
    type Vtable = ITfCategoryMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc3acefb5_f69d_4905_938f_fcadcf4be830);
}
impl ::core::convert::From<ITfCategoryMgr> for ::windows::runtime::IUnknown {
    fn from(value: ITfCategoryMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCategoryMgr> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCategoryMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCategoryMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCategoryMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCategoryMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, rcatid: *const ::windows::runtime::GUID, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, rcatid: *const ::windows::runtime::GUID, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rcatid: *const ::windows::runtime::GUID, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, pcatid: *mut ::windows::runtime::GUID, ppcatidlist: *const *const ::windows::runtime::GUID, ulcount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, rguid: *const ::windows::runtime::GUID, pchdesc: super::super::Foundation::PWSTR, cch: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, pbstrdesc: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, rguid: *const ::windows::runtime::GUID, dw: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, pdw: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, pguidatom: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidatom: u32, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidatom: u32, rguid: *const ::windows::runtime::GUID, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCleanupContextDurationSink(pub ::windows::runtime::IUnknown);
impl ITfCleanupContextDurationSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnStartCleanupContext(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnEndCleanupContext(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfCleanupContextDurationSink {
    type Vtable = ITfCleanupContextDurationSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x45c35144_154e_4797_bed8_d33ae7bf8794);
}
impl ::core::convert::From<ITfCleanupContextDurationSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfCleanupContextDurationSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCleanupContextDurationSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCleanupContextDurationSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCleanupContextDurationSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCleanupContextDurationSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCleanupContextDurationSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCleanupContextSink(pub ::windows::runtime::IUnknown);
impl ITfCleanupContextSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnCleanupContext<'a, Param1: ::windows::runtime::IntoParam<'a, ITfContext>>(&self, ecwrite: u32, pic: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pic.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfCleanupContextSink {
    type Vtable = ITfCleanupContextSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x01689689_7acb_4e9b_ab7c_7ea46b12b522);
}
impl ::core::convert::From<ITfCleanupContextSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfCleanupContextSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCleanupContextSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCleanupContextSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCleanupContextSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCleanupContextSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCleanupContextSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ecwrite: u32, pic: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfClientId(pub ::windows::runtime::IUnknown);
impl ITfClientId {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetClientId(&self, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfClientId {
    type Vtable = ITfClientId_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd60a7b49_1b9f_4be2_b702_47e9dc05dec3);
}
impl ::core::convert::From<ITfClientId> for ::windows::runtime::IUnknown {
    fn from(value: ITfClientId) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfClientId> for ::windows::runtime::IUnknown {
    fn from(value: &ITfClientId) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfClientId {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfClientId {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfClientId_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, ptid: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCompartment(pub ::windows::runtime::IUnknown);
impl ITfCompartment {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetValue(&self, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetValue(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfCompartment {
    type Vtable = ITfCompartment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbb08f7a9_607a_4384_8623_056892b64371);
}
impl ::core::convert::From<ITfCompartment> for ::windows::runtime::IUnknown {
    fn from(value: ITfCompartment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCompartment> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCompartment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCompartment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCompartment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompartment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tid: u32, pvarvalue: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCompartmentEventSink(pub ::windows::runtime::IUnknown);
impl ITfCompartmentEventSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnChange(&self, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfCompartmentEventSink {
    type Vtable = ITfCompartmentEventSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x743abd5f_f26d_48df_8cc5_238492419b64);
}
impl ::core::convert::From<ITfCompartmentEventSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfCompartmentEventSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCompartmentEventSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCompartmentEventSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCompartmentEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCompartmentEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompartmentEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCompartmentMgr(pub ::windows::runtime::IUnknown);
impl ITfCompartmentMgr {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetCompartment(&self, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ITfCompartment> {
        let mut result__: <ITfCompartment as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), &mut result__).from_abi::<ITfCompartment>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ClearCompartment(&self, tid: u32, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), ::core::mem::transmute(rguid)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn EnumCompartments(&self) -> ::windows::runtime::Result<super::super::System::Com::IEnumGUID> {
        let mut result__: <super::super::System::Com::IEnumGUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IEnumGUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfCompartmentMgr {
    type Vtable = ITfCompartmentMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7dcf57ac_18ad_438b_824d_979bffb74b7c);
}
impl ::core::convert::From<ITfCompartmentMgr> for ::windows::runtime::IUnknown {
    fn from(value: ITfCompartmentMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCompartmentMgr> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCompartmentMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCompartmentMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCompartmentMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompartmentMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, ppcomp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tid: u32, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfComposition(pub ::windows::runtime::IUnknown);
impl ITfComposition {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetRange(&self) -> ::windows::runtime::Result<ITfRange> {
        let mut result__: <ITfRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShiftStart<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ecwrite: u32, pnewstart: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pnewstart.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShiftEnd<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ecwrite: u32, pnewend: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pnewend.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EndComposition(&self, ecwrite: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfComposition {
    type Vtable = ITfComposition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x20168d64_5a8f_4a5a_b7bd_cfa29f4d0fd9);
}
impl ::core::convert::From<ITfComposition> for ::windows::runtime::IUnknown {
    fn from(value: ITfComposition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfComposition> for ::windows::runtime::IUnknown {
    fn from(value: &ITfComposition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfComposition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfComposition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfComposition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ecwrite: u32, pnewstart: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ecwrite: u32, pnewend: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ecwrite: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCompositionSink(pub ::windows::runtime::IUnknown);
impl ITfCompositionSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnCompositionTerminated<'a, Param1: ::windows::runtime::IntoParam<'a, ITfComposition>>(&self, ecwrite: u32, pcomposition: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pcomposition.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfCompositionSink {
    type Vtable = ITfCompositionSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa781718c_579a_4b15_a280_32b8577acc5e);
}
impl ::core::convert::From<ITfCompositionSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfCompositionSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCompositionSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCompositionSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCompositionSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCompositionSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompositionSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ecwrite: u32, pcomposition: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCompositionView(pub ::windows::runtime::IUnknown);
impl ITfCompositionView {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetOwnerClsid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetRange(&self) -> ::windows::runtime::Result<ITfRange> {
        let mut result__: <ITfRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfRange>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfCompositionView {
    type Vtable = ITfCompositionView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd7540241_f9a1_4364_befc_dbcd2c4395b7);
}
impl ::core::convert::From<ITfCompositionView> for ::windows::runtime::IUnknown {
    fn from(value: ITfCompositionView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCompositionView> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCompositionView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCompositionView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCompositionView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompositionView_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfConfigureSystemKeystrokeFeed(pub ::windows::runtime::IUnknown);
impl ITfConfigureSystemKeystrokeFeed {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn DisableSystemKeystrokeFeed(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnableSystemKeystrokeFeed(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfConfigureSystemKeystrokeFeed {
    type Vtable = ITfConfigureSystemKeystrokeFeed_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0d2c969a_bc9c_437c_84ee_951c49b1a764);
}
impl ::core::convert::From<ITfConfigureSystemKeystrokeFeed> for ::windows::runtime::IUnknown {
    fn from(value: ITfConfigureSystemKeystrokeFeed) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfConfigureSystemKeystrokeFeed> for ::windows::runtime::IUnknown {
    fn from(value: &ITfConfigureSystemKeystrokeFeed) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfConfigureSystemKeystrokeFeed {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfConfigureSystemKeystrokeFeed {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfConfigureSystemKeystrokeFeed_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfContext(pub ::windows::runtime::IUnknown);
impl ITfContext {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RequestEditSession<'a, Param1: ::windows::runtime::IntoParam<'a, ITfEditSession>>(&self, tid: u32, pes: Param1, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let mut result__: <::windows::runtime::HRESULT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), pes.into_param().abi(), ::core::mem::transmute(dwflags), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn InWriteSession(&self, tid: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetSelection(&self, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(ulindex), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection), ::core::mem::transmute(pcfetched)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetSelection(&self, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStart(&self, ec: u32) -> ::windows::runtime::Result<ITfRange> {
        let mut result__: <ITfRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), &mut result__).from_abi::<ITfRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetEnd(&self, ec: u32) -> ::windows::runtime::Result<ITfRange> {
        let mut result__: <ITfRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), &mut result__).from_abi::<ITfRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetActiveView(&self) -> ::windows::runtime::Result<ITfContextView> {
        let mut result__: <ITfContextView as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfContextView>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumViews(&self) -> ::windows::runtime::Result<IEnumTfContextViews> {
        let mut result__: <IEnumTfContextViews as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfContextViews>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<TS_STATUS> {
        let mut result__: <TS_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TS_STATUS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetProperty(&self, guidprop: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ITfProperty> {
        let mut result__: <ITfProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprop), &mut result__).from_abi::<ITfProperty>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetAppProperty(&self, guidprop: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ITfReadOnlyProperty> {
        let mut result__: <ITfReadOnlyProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprop), &mut result__).from_abi::<ITfReadOnlyProperty>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn TrackProperties(&self, prgprop: *const *const ::windows::runtime::GUID, cprop: u32, prgappprop: *const *const ::windows::runtime::GUID, cappprop: u32) -> ::windows::runtime::Result<ITfReadOnlyProperty> {
        let mut result__: <ITfReadOnlyProperty as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(prgprop), ::core::mem::transmute(cprop), ::core::mem::transmute(prgappprop), ::core::mem::transmute(cappprop), &mut result__).from_abi::<ITfReadOnlyProperty>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumProperties(&self) -> ::windows::runtime::Result<IEnumTfProperties> {
        let mut result__: <IEnumTfProperties as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfProperties>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::runtime::Result<ITfDocumentMgr> {
        let mut result__: <ITfDocumentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfDocumentMgr>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn CreateRangeBackup<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1) -> ::windows::runtime::Result<ITfRangeBackup> {
        let mut result__: <ITfRangeBackup as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), &mut result__).from_abi::<ITfRangeBackup>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfContext {
    type Vtable = ITfContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e7fd_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<ITfContext> for ::windows::runtime::IUnknown {
    fn from(value: ITfContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfContext> for ::windows::runtime::IUnknown {
    fn from(value: &ITfContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tid: u32, pes: ::windows::runtime::RawPtr, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS, phrsession: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tid: u32, pfwritesession: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut ::core::mem::ManuallyDrop<TF_SELECTION>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, ulcount: u32, pselection: *const ::core::mem::ManuallyDrop<TF_SELECTION>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, ppstart: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, ppend: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppview: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdcs: *mut TS_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidprop: *const ::windows::runtime::GUID, ppprop: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidprop: *const ::windows::runtime::GUID, ppprop: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prgprop: *const *const ::windows::runtime::GUID, cprop: u32, prgappprop: *const *const ::windows::runtime::GUID, cappprop: u32, ppproperty: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, ppbackup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfContextComposition(pub ::windows::runtime::IUnknown);
impl ITfContextComposition {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn StartComposition<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>, Param2: ::windows::runtime::IntoParam<'a, ITfCompositionSink>>(&self, ecwrite: u32, pcompositionrange: Param1, psink: Param2) -> ::windows::runtime::Result<ITfComposition> {
        let mut result__: <ITfComposition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pcompositionrange.into_param().abi(), psink.into_param().abi(), &mut result__).from_abi::<ITfComposition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumCompositions(&self) -> ::windows::runtime::Result<IEnumITfCompositionView> {
        let mut result__: <IEnumITfCompositionView as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumITfCompositionView>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn FindComposition<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ecread: u32, ptestrange: Param1) -> ::windows::runtime::Result<IEnumITfCompositionView> {
        let mut result__: <IEnumITfCompositionView as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecread), ptestrange.into_param().abi(), &mut result__).from_abi::<IEnumITfCompositionView>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn TakeOwnership<'a, Param1: ::windows::runtime::IntoParam<'a, ITfCompositionView>, Param2: ::windows::runtime::IntoParam<'a, ITfCompositionSink>>(&self, ecwrite: u32, pcomposition: Param1, psink: Param2) -> ::windows::runtime::Result<ITfComposition> {
        let mut result__: <ITfComposition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pcomposition.into_param().abi(), psink.into_param().abi(), &mut result__).from_abi::<ITfComposition>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfContextComposition {
    type Vtable = ITfContextComposition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd40c8aae_ac92_4fc7_9a11_0ee0e23aa39b);
}
impl ::core::convert::From<ITfContextComposition> for ::windows::runtime::IUnknown {
    fn from(value: ITfContextComposition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfContextComposition> for ::windows::runtime::IUnknown {
    fn from(value: &ITfContextComposition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfContextComposition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfContextComposition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextComposition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ecwrite: u32, pcompositionrange: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr, ppcomposition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ecread: u32, ptestrange: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ecwrite: u32, pcomposition: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr, ppcomposition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfContextKeyEventSink(pub ::windows::runtime::IUnknown);
impl ITfContextKeyEventSink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnKeyDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnKeyUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTestKeyDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTestKeyUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfContextKeyEventSink {
    type Vtable = ITfContextKeyEventSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0552ba5d_c835_4934_bf50_846aaa67432f);
}
impl ::core::convert::From<ITfContextKeyEventSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfContextKeyEventSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfContextKeyEventSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfContextKeyEventSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfContextKeyEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfContextKeyEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextKeyEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfContextOwner(pub ::windows::runtime::IUnknown);
impl ITfContextOwner {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetACPFromPoint(&self, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptscreen), ::core::mem::transmute(dwflags), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTextExt(&self, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(prc), ::core::mem::transmute(pfclipped)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetScreenExt(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<TS_STATUS> {
        let mut result__: <TS_STATUS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TS_STATUS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetWnd(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetAttribute(&self, rguidattribute: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidattribute), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfContextOwner {
    type Vtable = ITfContextOwner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e80c_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<ITfContextOwner> for ::windows::runtime::IUnknown {
    fn from(value: ITfContextOwner) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfContextOwner> for ::windows::runtime::IUnknown {
    fn from(value: &ITfContextOwner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfContextOwner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfContextOwner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwner_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdcs: *mut TS_STATUS) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidattribute: *const ::windows::runtime::GUID, pvarvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfContextOwnerCompositionServices(pub ::windows::runtime::IUnknown);
impl ITfContextOwnerCompositionServices {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn StartComposition<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>, Param2: ::windows::runtime::IntoParam<'a, ITfCompositionSink>>(&self, ecwrite: u32, pcompositionrange: Param1, psink: Param2) -> ::windows::runtime::Result<ITfComposition> {
        let mut result__: <ITfComposition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pcompositionrange.into_param().abi(), psink.into_param().abi(), &mut result__).from_abi::<ITfComposition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumCompositions(&self) -> ::windows::runtime::Result<IEnumITfCompositionView> {
        let mut result__: <IEnumITfCompositionView as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumITfCompositionView>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn FindComposition<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ecread: u32, ptestrange: Param1) -> ::windows::runtime::Result<IEnumITfCompositionView> {
        let mut result__: <IEnumITfCompositionView as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecread), ptestrange.into_param().abi(), &mut result__).from_abi::<IEnumITfCompositionView>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn TakeOwnership<'a, Param1: ::windows::runtime::IntoParam<'a, ITfCompositionView>, Param2: ::windows::runtime::IntoParam<'a, ITfCompositionSink>>(&self, ecwrite: u32, pcomposition: Param1, psink: Param2) -> ::windows::runtime::Result<ITfComposition> {
        let mut result__: <ITfComposition as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pcomposition.into_param().abi(), psink.into_param().abi(), &mut result__).from_abi::<ITfComposition>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn TerminateComposition<'a, Param0: ::windows::runtime::IntoParam<'a, ITfCompositionView>>(&self, pcomposition: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pcomposition.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfContextOwnerCompositionServices {
    type Vtable = ITfContextOwnerCompositionServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x86462810_593b_4916_9764_19c08e9ce110);
}
impl ::core::convert::From<ITfContextOwnerCompositionServices> for ::windows::runtime::IUnknown {
    fn from(value: ITfContextOwnerCompositionServices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfContextOwnerCompositionServices> for ::windows::runtime::IUnknown {
    fn from(value: &ITfContextOwnerCompositionServices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfContextOwnerCompositionServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfContextOwnerCompositionServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfContextOwnerCompositionServices> for ITfContextComposition {
    fn from(value: ITfContextOwnerCompositionServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfContextOwnerCompositionServices> for ITfContextComposition {
    fn from(value: &ITfContextOwnerCompositionServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfContextComposition> for ITfContextOwnerCompositionServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfContextComposition> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfContextComposition> for &ITfContextOwnerCompositionServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfContextComposition> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerCompositionServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ecwrite: u32, pcompositionrange: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr, ppcomposition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ecread: u32, ptestrange: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ecwrite: u32, pcomposition: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr, ppcomposition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcomposition: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfContextOwnerCompositionSink(pub ::windows::runtime::IUnknown);
impl ITfContextOwnerCompositionSink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnStartComposition<'a, Param0: ::windows::runtime::IntoParam<'a, ITfCompositionView>>(&self, pcomposition: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcomposition.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnUpdateComposition<'a, Param0: ::windows::runtime::IntoParam<'a, ITfCompositionView>, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, pcomposition: Param0, prangenew: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcomposition.into_param().abi(), prangenew.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnEndComposition<'a, Param0: ::windows::runtime::IntoParam<'a, ITfCompositionView>>(&self, pcomposition: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pcomposition.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfContextOwnerCompositionSink {
    type Vtable = ITfContextOwnerCompositionSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f20aa40_b57a_4f34_96ab_3576f377cc79);
}
impl ::core::convert::From<ITfContextOwnerCompositionSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfContextOwnerCompositionSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfContextOwnerCompositionSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfContextOwnerCompositionSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfContextOwnerCompositionSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfContextOwnerCompositionSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerCompositionSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcomposition: ::windows::runtime::RawPtr, pfok: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcomposition: ::windows::runtime::RawPtr, prangenew: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcomposition: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfContextOwnerServices(pub ::windows::runtime::IUnknown);
impl ITfContextOwnerServices {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnLayoutChange(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnAttributeChange(&self, rguidattribute: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidattribute)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn Serialize<'a, Param0: ::windows::runtime::IntoParam<'a, ITfProperty>, Param1: ::windows::runtime::IntoParam<'a, ITfRange>, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pprop: Param0, prange: Param1, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pprop.into_param().abi(), prange.into_param().abi(), ::core::mem::transmute(phdr), pstream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn Unserialize<'a, Param0: ::windows::runtime::IntoParam<'a, ITfProperty>, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param3: ::windows::runtime::IntoParam<'a, ITfPersistentPropertyLoaderACP>>(&self, pprop: Param0, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Param2, ploader: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pprop.into_param().abi(), ::core::mem::transmute(phdr), pstream.into_param().abi(), ploader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ForceLoadProperty<'a, Param0: ::windows::runtime::IntoParam<'a, ITfProperty>>(&self, pprop: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pprop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn CreateRange(&self, acpstart: i32, acpend: i32) -> ::windows::runtime::Result<ITfRangeACP> {
        let mut result__: <ITfRangeACP as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), &mut result__).from_abi::<ITfRangeACP>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfContextOwnerServices {
    type Vtable = ITfContextOwnerServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb23eb630_3e1c_11d3_a745_0050040ab407);
}
impl ::core::convert::From<ITfContextOwnerServices> for ::windows::runtime::IUnknown {
    fn from(value: ITfContextOwnerServices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfContextOwnerServices> for ::windows::runtime::IUnknown {
    fn from(value: &ITfContextOwnerServices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfContextOwnerServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfContextOwnerServices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidattribute: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprop: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprop: ::windows::runtime::RawPtr, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::runtime::RawPtr, ploader: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprop: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpstart: i32, acpend: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfContextView(pub ::windows::runtime::IUnknown);
impl ITfContextView {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetRangeFromPoint(&self, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::runtime::Result<ITfRange> {
        let mut result__: <ITfRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(ppt), ::core::mem::transmute(dwflags), &mut result__).from_abi::<ITfRange>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTextExt<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(prc), ::core::mem::transmute(pfclipped)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetScreenExt(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetWnd(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfContextView {
    type Vtable = ITfContextView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2433bf8e_0f9b_435c_ba2c_180611978c30);
}
impl ::core::convert::From<ITfContextView> for ::windows::runtime::IUnknown {
    fn from(value: ITfContextView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfContextView> for ::windows::runtime::IUnknown {
    fn from(value: &ITfContextView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfContextView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfContextView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextView_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfCreatePropertyStore(pub ::windows::runtime::IUnknown);
impl ITfCreatePropertyStore {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsStoreSerializable<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>, Param2: ::windows::runtime::IntoParam<'a, ITfPropertyStore>>(&self, guidprop: *const ::windows::runtime::GUID, prange: Param1, ppropstore: Param2) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprop), prange.into_param().abi(), ppropstore.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn CreatePropertyStore<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, guidprop: *const ::windows::runtime::GUID, prange: Param1, cb: u32, pstream: Param3) -> ::windows::runtime::Result<ITfPropertyStore> {
        let mut result__: <ITfPropertyStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprop), prange.into_param().abi(), ::core::mem::transmute(cb), pstream.into_param().abi(), &mut result__).from_abi::<ITfPropertyStore>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfCreatePropertyStore {
    type Vtable = ITfCreatePropertyStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2463fbf0_b0af_11d2_afc5_00105a2799b5);
}
impl ::core::convert::From<ITfCreatePropertyStore> for ::windows::runtime::IUnknown {
    fn from(value: ITfCreatePropertyStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfCreatePropertyStore> for ::windows::runtime::IUnknown {
    fn from(value: &ITfCreatePropertyStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfCreatePropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfCreatePropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCreatePropertyStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidprop: *const ::windows::runtime::GUID, prange: ::windows::runtime::RawPtr, ppropstore: ::windows::runtime::RawPtr, pfserializable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidprop: *const ::windows::runtime::GUID, prange: ::windows::runtime::RawPtr, cb: u32, pstream: ::windows::runtime::RawPtr, ppstore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfDisplayAttributeInfo(pub ::windows::runtime::IUnknown);
impl ITfDisplayAttributeInfo {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGUID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeInfo(&self) -> ::windows::runtime::Result<TF_DISPLAYATTRIBUTE> {
        let mut result__: <TF_DISPLAYATTRIBUTE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TF_DISPLAYATTRIBUTE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetAttributeInfo(&self, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pda)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfDisplayAttributeInfo {
    type Vtable = ITfDisplayAttributeInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x70528852_2f26_4aea_8c96_215150578932);
}
impl ::core::convert::From<ITfDisplayAttributeInfo> for ::windows::runtime::IUnknown {
    fn from(value: ITfDisplayAttributeInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfDisplayAttributeInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ITfDisplayAttributeInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfDisplayAttributeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfDisplayAttributeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdesc: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pda: *mut TF_DISPLAYATTRIBUTE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfDisplayAttributeMgr(pub ::windows::runtime::IUnknown);
impl ITfDisplayAttributeMgr {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnUpdateInfo(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumDisplayAttributeInfo(&self) -> ::windows::runtime::Result<IEnumTfDisplayAttributeInfo> {
        let mut result__: <IEnumTfDisplayAttributeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfDisplayAttributeInfo>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetDisplayAttributeInfo(&self, guid: *const ::windows::runtime::GUID, ppinfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pclsidowner: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(ppinfo), ::core::mem::transmute(pclsidowner)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfDisplayAttributeMgr {
    type Vtable = ITfDisplayAttributeMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8ded7393_5db1_475c_9e71_a39111b0ff67);
}
impl ::core::convert::From<ITfDisplayAttributeMgr> for ::windows::runtime::IUnknown {
    fn from(value: ITfDisplayAttributeMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfDisplayAttributeMgr> for ::windows::runtime::IUnknown {
    fn from(value: &ITfDisplayAttributeMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfDisplayAttributeMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfDisplayAttributeMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, ppinfo: *mut ::windows::runtime::RawPtr, pclsidowner: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfDisplayAttributeNotifySink(pub ::windows::runtime::IUnknown);
impl ITfDisplayAttributeNotifySink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnUpdateInfo(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfDisplayAttributeNotifySink {
    type Vtable = ITfDisplayAttributeNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xad56f402_e162_4f25_908f_7d577cf9bda9);
}
impl ::core::convert::From<ITfDisplayAttributeNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: ITfDisplayAttributeNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfDisplayAttributeNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfDisplayAttributeNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfDisplayAttributeNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfDisplayAttributeNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfDisplayAttributeProvider(pub ::windows::runtime::IUnknown);
impl ITfDisplayAttributeProvider {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumDisplayAttributeInfo(&self) -> ::windows::runtime::Result<IEnumTfDisplayAttributeInfo> {
        let mut result__: <IEnumTfDisplayAttributeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfDisplayAttributeInfo>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetDisplayAttributeInfo(&self, guid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ITfDisplayAttributeInfo> {
        let mut result__: <ITfDisplayAttributeInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), &mut result__).from_abi::<ITfDisplayAttributeInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfDisplayAttributeProvider {
    type Vtable = ITfDisplayAttributeProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfee47777_163c_4769_996a_6e9c50ad8f54);
}
impl ::core::convert::From<ITfDisplayAttributeProvider> for ::windows::runtime::IUnknown {
    fn from(value: ITfDisplayAttributeProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfDisplayAttributeProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ITfDisplayAttributeProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfDisplayAttributeProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfDisplayAttributeProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *const ::windows::runtime::GUID, ppinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfDocumentMgr(pub ::windows::runtime::IUnknown);
impl ITfDocumentMgr {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn CreateContext<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, tidowner: u32, dwflags: u32, punk: Param2, ppic: *mut ::core::option::Option<ITfContext>, pectextstore: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tidowner), ::core::mem::transmute(dwflags), punk.into_param().abi(), ::core::mem::transmute(ppic), ::core::mem::transmute(pectextstore)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Push<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>>(&self, pic: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pic.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Pop(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetTop(&self) -> ::windows::runtime::Result<ITfContext> {
        let mut result__: <ITfContext as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfContext>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetBase(&self) -> ::windows::runtime::Result<ITfContext> {
        let mut result__: <ITfContext as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfContext>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumContexts(&self) -> ::windows::runtime::Result<IEnumTfContexts> {
        let mut result__: <IEnumTfContexts as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfContexts>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfDocumentMgr {
    type Vtable = ITfDocumentMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e7f4_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<ITfDocumentMgr> for ::windows::runtime::IUnknown {
    fn from(value: ITfDocumentMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfDocumentMgr> for ::windows::runtime::IUnknown {
    fn from(value: &ITfDocumentMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfDocumentMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfDocumentMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDocumentMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tidowner: u32, dwflags: u32, punk: ::windows::runtime::RawPtr, ppic: *mut ::windows::runtime::RawPtr, pectextstore: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppic: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppic: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfEditRecord(pub ::windows::runtime::IUnknown);
impl ITfEditRecord {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetSelectionStatus(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetTextAndPropertyUpdates(&self, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows::runtime::GUID, cproperties: u32) -> ::windows::runtime::Result<IEnumTfRanges> {
        let mut result__: <IEnumTfRanges as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(prgproperties), ::core::mem::transmute(cproperties), &mut result__).from_abi::<IEnumTfRanges>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfEditRecord {
    type Vtable = ITfEditRecord_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x42d4d099_7c1a_4a89_b836_6c6f22160df0);
}
impl ::core::convert::From<ITfEditRecord> for ::windows::runtime::IUnknown {
    fn from(value: ITfEditRecord) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfEditRecord> for ::windows::runtime::IUnknown {
    fn from(value: &ITfEditRecord) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfEditRecord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfEditRecord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfEditRecord_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows::runtime::GUID, cproperties: u32, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfEditSession(pub ::windows::runtime::IUnknown);
impl ITfEditSession {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn DoEditSession(&self, ec: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfEditSession {
    type Vtable = ITfEditSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e803_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<ITfEditSession> for ::windows::runtime::IUnknown {
    fn from(value: ITfEditSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfEditSession> for ::windows::runtime::IUnknown {
    fn from(value: &ITfEditSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfEditSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfEditSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfEditSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfEditTransactionSink(pub ::windows::runtime::IUnknown);
impl ITfEditTransactionSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnStartEditTransaction<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>>(&self, pic: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pic.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnEndEditTransaction<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>>(&self, pic: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pic.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfEditTransactionSink {
    type Vtable = ITfEditTransactionSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x708fbf70_b520_416b_b06c_2c41ab44f8ba);
}
impl ::core::convert::From<ITfEditTransactionSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfEditTransactionSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfEditTransactionSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfEditTransactionSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfEditTransactionSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfEditTransactionSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfEditTransactionSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnAdviseText(pub ::windows::runtime::IUnknown);
impl ITfFnAdviseText {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTextUpdate<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, prange: Param0, pchtext: Param1, cch: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), prange.into_param().abi(), pchtext.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnLatticeUpdate<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>, Param1: ::windows::runtime::IntoParam<'a, ITfLMLattice>>(&self, prange: Param0, plattice: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), prange.into_param().abi(), plattice.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnAdviseText {
    type Vtable = ITfFnAdviseText_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3527268b_7d53_4dd9_92b7_7296ae461249);
}
impl ::core::convert::From<ITfFnAdviseText> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnAdviseText) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnAdviseText> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnAdviseText) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnAdviseText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnAdviseText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnAdviseText> for ITfFunction {
    fn from(value: ITfFnAdviseText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnAdviseText> for ITfFunction {
    fn from(value: &ITfFnAdviseText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnAdviseText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnAdviseText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnAdviseText_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, plattice: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnBalloon(pub ::windows::runtime::IUnknown);
impl ITfFnBalloon {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn UpdateBalloon<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, style: TfLBBalloonStyle, pch: Param1, cch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(style), pch.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnBalloon {
    type Vtable = ITfFnBalloon_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3bab89e4_5fbe_45f4_a5bc_dca36ad225a8);
}
impl ::core::convert::From<ITfFnBalloon> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnBalloon) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnBalloon> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnBalloon) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnBalloon {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnBalloon {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnBalloon_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnConfigure(pub ::windows::runtime::IUnknown);
impl ITfFnConfigure {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, langid: u16, rguidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(langid), ::core::mem::transmute(rguidprofile)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnConfigure {
    type Vtable = ITfFnConfigure_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x88f567c6_1757_49f8_a1b2_89234c1eeff9);
}
impl ::core::convert::From<ITfFnConfigure> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnConfigure) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnConfigure> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnConfigure) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnConfigure {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnConfigure {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnConfigure> for ITfFunction {
    fn from(value: ITfFnConfigure) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnConfigure> for ITfFunction {
    fn from(value: &ITfFnConfigure) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnConfigure {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnConfigure {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnConfigure_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnConfigureRegisterEudc(pub ::windows::runtime::IUnknown);
impl ITfFnConfigureRegisterEudc {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, hwndparent: Param0, langid: u16, rguidprofile: *const ::windows::runtime::GUID, bstrregistered: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(langid), ::core::mem::transmute(rguidprofile), bstrregistered.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnConfigureRegisterEudc {
    type Vtable = ITfFnConfigureRegisterEudc_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb5e26ff5_d7ad_4304_913f_21a2ed95a1b0);
}
impl ::core::convert::From<ITfFnConfigureRegisterEudc> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnConfigureRegisterEudc) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnConfigureRegisterEudc> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnConfigureRegisterEudc) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnConfigureRegisterEudc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnConfigureRegisterEudc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnConfigureRegisterEudc> for ITfFunction {
    fn from(value: ITfFnConfigureRegisterEudc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnConfigureRegisterEudc> for ITfFunction {
    fn from(value: &ITfFnConfigureRegisterEudc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnConfigureRegisterEudc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnConfigureRegisterEudc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnConfigureRegisterEudc_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::runtime::GUID, bstrregistered: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnConfigureRegisterWord(pub ::windows::runtime::IUnknown);
impl ITfFnConfigureRegisterWord {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, hwndparent: Param0, langid: u16, rguidprofile: *const ::windows::runtime::GUID, bstrregistered: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(langid), ::core::mem::transmute(rguidprofile), bstrregistered.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnConfigureRegisterWord {
    type Vtable = ITfFnConfigureRegisterWord_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbb95808a_6d8f_4bca_8400_5390b586aedf);
}
impl ::core::convert::From<ITfFnConfigureRegisterWord> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnConfigureRegisterWord) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnConfigureRegisterWord> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnConfigureRegisterWord) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnConfigureRegisterWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnConfigureRegisterWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnConfigureRegisterWord> for ITfFunction {
    fn from(value: ITfFnConfigureRegisterWord) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnConfigureRegisterWord> for ITfFunction {
    fn from(value: &ITfFnConfigureRegisterWord) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnConfigureRegisterWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnConfigureRegisterWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnConfigureRegisterWord_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::runtime::GUID, bstrregistered: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnCustomSpeechCommand(pub ::windows::runtime::IUnknown);
impl ITfFnCustomSpeechCommand {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetSpeechCommandProvider<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pspcmdprovider: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pspcmdprovider.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnCustomSpeechCommand {
    type Vtable = ITfFnCustomSpeechCommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfca6c349_a12f_43a3_8dd6_5a5a4282577b);
}
impl ::core::convert::From<ITfFnCustomSpeechCommand> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnCustomSpeechCommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnCustomSpeechCommand> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnCustomSpeechCommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnCustomSpeechCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnCustomSpeechCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnCustomSpeechCommand> for ITfFunction {
    fn from(value: ITfFnCustomSpeechCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnCustomSpeechCommand> for ITfFunction {
    fn from(value: &ITfFnCustomSpeechCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnCustomSpeechCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnCustomSpeechCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnCustomSpeechCommand_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pspcmdprovider: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnGetLinguisticAlternates(pub ::windows::runtime::IUnknown);
impl ITfFnGetLinguisticAlternates {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetAlternates<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::runtime::Result<ITfCandidateList> {
        let mut result__: <ITfCandidateList as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<ITfCandidateList>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnGetLinguisticAlternates {
    type Vtable = ITfFnGetLinguisticAlternates_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea163ce2_7a65_4506_82a3_c528215da64e);
}
impl ::core::convert::From<ITfFnGetLinguisticAlternates> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnGetLinguisticAlternates) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnGetLinguisticAlternates> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnGetLinguisticAlternates) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnGetLinguisticAlternates {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnGetLinguisticAlternates {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnGetLinguisticAlternates> for ITfFunction {
    fn from(value: ITfFnGetLinguisticAlternates) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnGetLinguisticAlternates> for ITfFunction {
    fn from(value: &ITfFnGetLinguisticAlternates) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnGetLinguisticAlternates {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnGetLinguisticAlternates {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnGetLinguisticAlternates_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, ppcandidatelist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnGetPreferredTouchKeyboardLayout(pub ::windows::runtime::IUnknown);
impl ITfFnGetPreferredTouchKeyboardLayout {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetLayout(&self, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptkblayouttype), ::core::mem::transmute(pwpreferredlayoutid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnGetPreferredTouchKeyboardLayout {
    type Vtable = ITfFnGetPreferredTouchKeyboardLayout_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f309a41_590a_4acc_a97f_d8efff13fdfc);
}
impl ::core::convert::From<ITfFnGetPreferredTouchKeyboardLayout> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnGetPreferredTouchKeyboardLayout) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnGetPreferredTouchKeyboardLayout> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnGetPreferredTouchKeyboardLayout) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnGetPreferredTouchKeyboardLayout {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnGetPreferredTouchKeyboardLayout {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnGetPreferredTouchKeyboardLayout> for ITfFunction {
    fn from(value: ITfFnGetPreferredTouchKeyboardLayout) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnGetPreferredTouchKeyboardLayout> for ITfFunction {
    fn from(value: &ITfFnGetPreferredTouchKeyboardLayout) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnGetPreferredTouchKeyboardLayout {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnGetPreferredTouchKeyboardLayout {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnGetPreferredTouchKeyboardLayout_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnGetSAPIObject(pub ::windows::runtime::IUnknown);
impl ITfFnGetSAPIObject {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Get(&self, sobj: TfSapiObject) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(sobj), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnGetSAPIObject {
    type Vtable = ITfFnGetSAPIObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5c0ab7ea_167d_4f59_bfb5_4693755e90ca);
}
impl ::core::convert::From<ITfFnGetSAPIObject> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnGetSAPIObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnGetSAPIObject> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnGetSAPIObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnGetSAPIObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnGetSAPIObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnGetSAPIObject> for ITfFunction {
    fn from(value: ITfFnGetSAPIObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnGetSAPIObject> for ITfFunction {
    fn from(value: &ITfFnGetSAPIObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnGetSAPIObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnGetSAPIObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnGetSAPIObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sobj: TfSapiObject, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnLMInternal(pub ::windows::runtime::IUnknown);
impl ITfFnLMInternal {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn QueryRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(ppnewrange), ::core::mem::transmute(pfaccepted)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn QueryLangID(&self, langid: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetReconversion<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::runtime::Result<ITfCandidateList> {
        let mut result__: <ITfCandidateList as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<ITfCandidateList>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reconvert<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn QueryKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, fup: Param0, vkey: Param1, lparamkeydata: Param2) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), fup.into_param().abi(), vkey.into_param().abi(), lparamkeydata.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn InvokeKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, fup: Param0, vkey: Param1, lparamkeydata: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), fup.into_param().abi(), vkey.into_param().abi(), lparamkeydata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn InvokeFunc<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>>(&self, pic: Param0, refguidfunc: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(refguidfunc)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ProcessLattice<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnLMInternal {
    type Vtable = ITfFnLMInternal_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x04b825b1_ac9a_4f7b_b5ad_c7168f1ee445);
}
impl ::core::convert::From<ITfFnLMInternal> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnLMInternal) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnLMInternal> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnLMInternal) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnLMInternal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnLMInternal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnLMInternal> for ITfFnLMProcessor {
    fn from(value: ITfFnLMInternal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnLMInternal> for ITfFnLMProcessor {
    fn from(value: &ITfFnLMInternal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFnLMProcessor> for ITfFnLMInternal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFnLMProcessor> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFnLMProcessor> for &ITfFnLMInternal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFnLMProcessor> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnLMInternal> for ITfFunction {
    fn from(value: ITfFnLMInternal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnLMInternal> for ITfFunction {
    fn from(value: &ITfFnLMInternal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnLMInternal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnLMInternal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnLMInternal_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, ppnewrange: *mut ::windows::runtime::RawPtr, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, ppcandlist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, refguidfunc: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnLMProcessor(pub ::windows::runtime::IUnknown);
impl ITfFnLMProcessor {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn QueryRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(ppnewrange), ::core::mem::transmute(pfaccepted)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn QueryLangID(&self, langid: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetReconversion<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::runtime::Result<ITfCandidateList> {
        let mut result__: <ITfCandidateList as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<ITfCandidateList>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reconvert<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn QueryKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, fup: Param0, vkey: Param1, lparamkeydata: Param2) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), fup.into_param().abi(), vkey.into_param().abi(), lparamkeydata.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn InvokeKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, fup: Param0, vkey: Param1, lparamkeydata: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), fup.into_param().abi(), vkey.into_param().abi(), lparamkeydata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn InvokeFunc<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>>(&self, pic: Param0, refguidfunc: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(refguidfunc)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnLMProcessor {
    type Vtable = ITfFnLMProcessor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7afbf8e7_ac4b_4082_b058_890899d3a010);
}
impl ::core::convert::From<ITfFnLMProcessor> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnLMProcessor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnLMProcessor> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnLMProcessor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnLMProcessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnLMProcessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnLMProcessor> for ITfFunction {
    fn from(value: ITfFnLMProcessor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnLMProcessor> for ITfFunction {
    fn from(value: &ITfFnLMProcessor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnLMProcessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnLMProcessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnLMProcessor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, ppnewrange: *mut ::windows::runtime::RawPtr, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, ppcandlist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, refguidfunc: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnLangProfileUtil(pub ::windows::runtime::IUnknown);
impl ITfFnLangProfileUtil {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RegisterActiveProfiles(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsProfileAvailableForLang(&self, langid: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnLangProfileUtil {
    type Vtable = ITfFnLangProfileUtil_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa87a8574_a6c1_4e15_99f0_3d3965f548eb);
}
impl ::core::convert::From<ITfFnLangProfileUtil> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnLangProfileUtil) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnLangProfileUtil> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnLangProfileUtil) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnLangProfileUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnLangProfileUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnLangProfileUtil> for ITfFunction {
    fn from(value: ITfFnLangProfileUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnLangProfileUtil> for ITfFunction {
    fn from(value: &ITfFnLangProfileUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnLangProfileUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnLangProfileUtil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnLangProfileUtil_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, pfavailable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnPlayBack(pub ::windows::runtime::IUnknown);
impl ITfFnPlayBack {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn QueryRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(ppnewrange), ::core::mem::transmute(pfplayable)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Play<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnPlayBack {
    type Vtable = ITfFnPlayBack_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa3a416a4_0f64_11d3_b5b7_00c04fc324a1);
}
impl ::core::convert::From<ITfFnPlayBack> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnPlayBack) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnPlayBack> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnPlayBack) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnPlayBack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnPlayBack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnPlayBack> for ITfFunction {
    fn from(value: ITfFnPlayBack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnPlayBack> for ITfFunction {
    fn from(value: &ITfFnPlayBack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnPlayBack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnPlayBack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnPlayBack_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, ppnewrange: *mut ::windows::runtime::RawPtr, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnPropertyUIStatus(pub ::windows::runtime::IUnknown);
impl ITfFnPropertyUIStatus {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStatus(&self, refguidprop: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguidprop), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetStatus(&self, refguidprop: *const ::windows::runtime::GUID, dw: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguidprop), ::core::mem::transmute(dw)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnPropertyUIStatus {
    type Vtable = ITfFnPropertyUIStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2338ac6e_2b9d_44c0_a75e_ee64f256b3bd);
}
impl ::core::convert::From<ITfFnPropertyUIStatus> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnPropertyUIStatus) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnPropertyUIStatus> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnPropertyUIStatus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnPropertyUIStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnPropertyUIStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnPropertyUIStatus> for ITfFunction {
    fn from(value: ITfFnPropertyUIStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnPropertyUIStatus> for ITfFunction {
    fn from(value: &ITfFnPropertyUIStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnPropertyUIStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnPropertyUIStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnPropertyUIStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, refguidprop: *const ::windows::runtime::GUID, pdw: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, refguidprop: *const ::windows::runtime::GUID, dw: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnReconversion(pub ::windows::runtime::IUnknown);
impl ITfFnReconversion {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn QueryRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(ppnewrange), ::core::mem::transmute(pfconvertable)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetReconversion<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::runtime::Result<ITfCandidateList> {
        let mut result__: <ITfCandidateList as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), prange.into_param().abi(), &mut result__).from_abi::<ITfCandidateList>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Reconvert<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnReconversion {
    type Vtable = ITfFnReconversion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4cea93c0_0a58_11d3_8df0_00105a2799b5);
}
impl ::core::convert::From<ITfFnReconversion> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnReconversion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnReconversion> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnReconversion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnReconversion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnReconversion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnReconversion> for ITfFunction {
    fn from(value: ITfFnReconversion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnReconversion> for ITfFunction {
    fn from(value: &ITfFnReconversion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnReconversion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnReconversion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnReconversion_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, ppnewrange: *mut ::windows::runtime::RawPtr, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr, ppcandlist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnSearchCandidateProvider(pub ::windows::runtime::IUnknown);
impl ITfFnSearchCandidateProvider {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetSearchCandidates<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrquery: Param0, bstrapplicationid: Param1) -> ::windows::runtime::Result<ITfCandidateList> {
        let mut result__: <ITfCandidateList as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrquery.into_param().abi(), bstrapplicationid.into_param().abi(), &mut result__).from_abi::<ITfCandidateList>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetResult<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrquery: Param0, bstrapplicationid: Param1, bstrresult: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrquery.into_param().abi(), bstrapplicationid.into_param().abi(), bstrresult.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnSearchCandidateProvider {
    type Vtable = ITfFnSearchCandidateProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x87a2ad8f_f27b_4920_8501_67602280175d);
}
impl ::core::convert::From<ITfFnSearchCandidateProvider> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnSearchCandidateProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnSearchCandidateProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnSearchCandidateProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnSearchCandidateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnSearchCandidateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnSearchCandidateProvider> for ITfFunction {
    fn from(value: ITfFnSearchCandidateProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnSearchCandidateProvider> for ITfFunction {
    fn from(value: &ITfFnSearchCandidateProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnSearchCandidateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnSearchCandidateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnSearchCandidateProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresult: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFnShowHelp(pub ::windows::runtime::IUnknown);
impl ITfFnShowHelp {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwndparent.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfFnShowHelp {
    type Vtable = ITfFnShowHelp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5ab1d30c_094d_4c29_8ea5_0bf59be87bf3);
}
impl ::core::convert::From<ITfFnShowHelp> for ::windows::runtime::IUnknown {
    fn from(value: ITfFnShowHelp) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFnShowHelp> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFnShowHelp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFnShowHelp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFnShowHelp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfFnShowHelp> for ITfFunction {
    fn from(value: ITfFnShowHelp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnShowHelp> for ITfFunction {
    fn from(value: &ITfFnShowHelp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for ITfFnShowHelp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfFunction> for &ITfFnShowHelp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfFunction> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnShowHelp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFunction(pub ::windows::runtime::IUnknown);
impl ITfFunction {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfFunction {
    type Vtable = ITfFunction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdb593490_098f_11d3_8df0_00105a2799b5);
}
impl ::core::convert::From<ITfFunction> for ::windows::runtime::IUnknown {
    fn from(value: ITfFunction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFunction> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFunction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFunction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFunction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFunction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfFunctionProvider(pub ::windows::runtime::IUnknown);
impl ITfFunctionProvider {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetFunction(&self, rguid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfFunctionProvider {
    type Vtable = ITfFunctionProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x101d6610_0990_11d3_8df0_00105a2799b5);
}
impl ::core::convert::From<ITfFunctionProvider> for ::windows::runtime::IUnknown {
    fn from(value: ITfFunctionProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfFunctionProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ITfFunctionProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfFunctionProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfFunctionProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFunctionProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdesc: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfInputProcessorProfileActivationSink(pub ::windows::runtime::IUnknown);
impl ITfInputProcessorProfileActivationSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnActivated<'a, Param5: ::windows::runtime::IntoParam<'a, HKL>>(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::runtime::GUID, catid: *const ::windows::runtime::GUID, guidprofile: *const ::windows::runtime::GUID, hkl: Param5, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprofiletype), ::core::mem::transmute(langid), ::core::mem::transmute(clsid), ::core::mem::transmute(catid), ::core::mem::transmute(guidprofile), hkl.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfInputProcessorProfileActivationSink {
    type Vtable = ITfInputProcessorProfileActivationSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x71c6e74e_0f28_11d8_a82a_00065b84435c);
}
impl ::core::convert::From<ITfInputProcessorProfileActivationSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfInputProcessorProfileActivationSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfInputProcessorProfileActivationSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfInputProcessorProfileActivationSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfInputProcessorProfileActivationSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfInputProcessorProfileActivationSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfileActivationSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprofiletype: u32, langid: u16, clsid: *const ::windows::runtime::GUID, catid: *const ::windows::runtime::GUID, guidprofile: *const ::windows::runtime::GUID, hkl: HKL, dwflags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfInputProcessorProfileMgr(pub ::windows::runtime::IUnknown);
impl ITfInputProcessorProfileMgr {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ActivateProfile<'a, Param4: ::windows::runtime::IntoParam<'a, HKL>>(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::runtime::GUID, guidprofile: *const ::windows::runtime::GUID, hkl: Param4, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprofiletype), ::core::mem::transmute(langid), ::core::mem::transmute(clsid), ::core::mem::transmute(guidprofile), hkl.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn DeactivateProfile<'a, Param4: ::windows::runtime::IntoParam<'a, HKL>>(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::runtime::GUID, guidprofile: *const ::windows::runtime::GUID, hkl: Param4, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprofiletype), ::core::mem::transmute(langid), ::core::mem::transmute(clsid), ::core::mem::transmute(guidprofile), hkl.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetProfile<'a, Param4: ::windows::runtime::IntoParam<'a, HKL>>(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::runtime::GUID, guidprofile: *const ::windows::runtime::GUID, hkl: Param4) -> ::windows::runtime::Result<TF_INPUTPROCESSORPROFILE> {
        let mut result__: <TF_INPUTPROCESSORPROFILE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprofiletype), ::core::mem::transmute(langid), ::core::mem::transmute(clsid), ::core::mem::transmute(guidprofile), hkl.into_param().abi(), &mut result__).from_abi::<TF_INPUTPROCESSORPROFILE>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumProfiles(&self, langid: u16) -> ::windows::runtime::Result<IEnumTfInputProcessorProfiles> {
        let mut result__: <IEnumTfInputProcessorProfiles as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), &mut result__).from_abi::<IEnumTfInputProcessorProfiles>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ReleaseInputProcessor(&self, rclsid: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn RegisterProfile<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param8: ::windows::runtime::IntoParam<'a, HKL>, Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
        &self,
        rclsid: *const ::windows::runtime::GUID,
        langid: u16,
        guidprofile: *const ::windows::runtime::GUID,
        pchdesc: Param3,
        cchdesc: u32,
        pchiconfile: Param5,
        cchfile: u32,
        uiconindex: u32,
        hklsubstitute: Param8,
        dwpreferredlayout: u32,
        benabledbydefault: Param10,
        dwflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(rclsid),
            ::core::mem::transmute(langid),
            ::core::mem::transmute(guidprofile),
            pchdesc.into_param().abi(),
            ::core::mem::transmute(cchdesc),
            pchiconfile.into_param().abi(),
            ::core::mem::transmute(cchfile),
            ::core::mem::transmute(uiconindex),
            hklsubstitute.into_param().abi(),
            ::core::mem::transmute(dwpreferredlayout),
            benabledbydefault.into_param().abi(),
            ::core::mem::transmute(dwflags),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnregisterProfile(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetActiveProfile(&self, catid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<TF_INPUTPROCESSORPROFILE> {
        let mut result__: <TF_INPUTPROCESSORPROFILE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(catid), &mut result__).from_abi::<TF_INPUTPROCESSORPROFILE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfInputProcessorProfileMgr {
    type Vtable = ITfInputProcessorProfileMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x71c6e74c_0f28_11d8_a82a_00065b84435c);
}
impl ::core::convert::From<ITfInputProcessorProfileMgr> for ::windows::runtime::IUnknown {
    fn from(value: ITfInputProcessorProfileMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfInputProcessorProfileMgr> for ::windows::runtime::IUnknown {
    fn from(value: &ITfInputProcessorProfileMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfInputProcessorProfileMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfInputProcessorProfileMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfileMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprofiletype: u32, langid: u16, clsid: *const ::windows::runtime::GUID, guidprofile: *const ::windows::runtime::GUID, hkl: HKL, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprofiletype: u32, langid: u16, clsid: *const ::windows::runtime::GUID, guidprofile: *const ::windows::runtime::GUID, hkl: HKL, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprofiletype: u32, langid: u16, clsid: *const ::windows::runtime::GUID, guidprofile: *const ::windows::runtime::GUID, hkl: HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, catid: *const ::windows::runtime::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfInputProcessorProfileSubstituteLayout(pub ::windows::runtime::IUnknown);
impl ITfInputProcessorProfileSubstituteLayout {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetSubstituteKeyboardLayout(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<HKL> {
        let mut result__: <HKL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), &mut result__).from_abi::<HKL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfInputProcessorProfileSubstituteLayout {
    type Vtable = ITfInputProcessorProfileSubstituteLayout_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4fd67194_1002_4513_bff2_c0ddf6258552);
}
impl ::core::convert::From<ITfInputProcessorProfileSubstituteLayout> for ::windows::runtime::IUnknown {
    fn from(value: ITfInputProcessorProfileSubstituteLayout) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfInputProcessorProfileSubstituteLayout> for ::windows::runtime::IUnknown {
    fn from(value: &ITfInputProcessorProfileSubstituteLayout) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfInputProcessorProfileSubstituteLayout {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfInputProcessorProfileSubstituteLayout {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfileSubstituteLayout_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, phkl: *mut HKL) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfInputProcessorProfiles(pub ::windows::runtime::IUnknown);
impl ITfInputProcessorProfiles {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Register(&self, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Unregister(&self, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn AddLanguageProfile<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, pchdesc: Param3, cchdesc: u32, pchiconfile: Param5, cchfile: u32, uiconindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), pchdesc.into_param().abi(), ::core::mem::transmute(cchdesc), pchiconfile.into_param().abi(), ::core::mem::transmute(cchfile), ::core::mem::transmute(uiconindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RemoveLanguageProfile(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn EnumInputProcessorInfo(&self) -> ::windows::runtime::Result<super::super::System::Com::IEnumGUID> {
        let mut result__: <super::super::System::Com::IEnumGUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IEnumGUID>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetDefaultLanguageProfile(&self, langid: u16, catid: *const ::windows::runtime::GUID, pclsid: *mut ::windows::runtime::GUID, pguidprofile: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(catid), ::core::mem::transmute(pclsid), ::core::mem::transmute(pguidprofile)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetDefaultLanguageProfile(&self, langid: u16, rclsid: *const ::windows::runtime::GUID, guidprofiles: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(rclsid), ::core::mem::transmute(guidprofiles)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ActivateLanguageProfile(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofiles: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofiles)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetActiveLanguageProfile(&self, rclsid: *const ::windows::runtime::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(plangid), ::core::mem::transmute(pguidprofile)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguageProfileDescription(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetCurrentLanguage(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ChangeCurrentLanguage(&self, langid: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetLanguageList(&self, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pplangid), ::core::mem::transmute(pulcount)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumLanguageProfiles(&self, langid: u16) -> ::windows::runtime::Result<IEnumTfLanguageProfiles> {
        let mut result__: <IEnumTfLanguageProfiles as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), &mut result__).from_abi::<IEnumTfLanguageProfiles>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn EnableLanguageProfile<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, fenable: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), fenable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsEnabledLanguageProfile(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn EnableLanguageProfileByDefault<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, fenable: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SubstituteKeyboardLayout<'a, Param3: ::windows::runtime::IntoParam<'a, HKL>>(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, hkl: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), hkl.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfInputProcessorProfiles {
    type Vtable = ITfInputProcessorProfiles_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1f02b6c5_7842_4ee6_8a0b_9a24183a95ca);
}
impl ::core::convert::From<ITfInputProcessorProfiles> for ::windows::runtime::IUnknown {
    fn from(value: ITfInputProcessorProfiles) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfInputProcessorProfiles> for ::windows::runtime::IUnknown {
    fn from(value: &ITfInputProcessorProfiles) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfInputProcessorProfiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfInputProcessorProfiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfiles_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, catid: *const ::windows::runtime::GUID, pclsid: *mut ::windows::runtime::GUID, pguidprofile: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, rclsid: *const ::windows::runtime::GUID, guidprofiles: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofiles: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, pbstrprofile: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plangid: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, hkl: HKL) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfInputProcessorProfilesEx(pub ::windows::runtime::IUnknown);
impl ITfInputProcessorProfilesEx {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Register(&self, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Unregister(&self, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn AddLanguageProfile<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, pchdesc: Param3, cchdesc: u32, pchiconfile: Param5, cchfile: u32, uiconindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), pchdesc.into_param().abi(), ::core::mem::transmute(cchdesc), pchiconfile.into_param().abi(), ::core::mem::transmute(cchfile), ::core::mem::transmute(uiconindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RemoveLanguageProfile(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn EnumInputProcessorInfo(&self) -> ::windows::runtime::Result<super::super::System::Com::IEnumGUID> {
        let mut result__: <super::super::System::Com::IEnumGUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IEnumGUID>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetDefaultLanguageProfile(&self, langid: u16, catid: *const ::windows::runtime::GUID, pclsid: *mut ::windows::runtime::GUID, pguidprofile: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(catid), ::core::mem::transmute(pclsid), ::core::mem::transmute(pguidprofile)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetDefaultLanguageProfile(&self, langid: u16, rclsid: *const ::windows::runtime::GUID, guidprofiles: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(rclsid), ::core::mem::transmute(guidprofiles)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ActivateLanguageProfile(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofiles: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofiles)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetActiveLanguageProfile(&self, rclsid: *const ::windows::runtime::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(plangid), ::core::mem::transmute(pguidprofile)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguageProfileDescription(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetCurrentLanguage(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ChangeCurrentLanguage(&self, langid: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetLanguageList(&self, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pplangid), ::core::mem::transmute(pulcount)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumLanguageProfiles(&self, langid: u16) -> ::windows::runtime::Result<IEnumTfLanguageProfiles> {
        let mut result__: <IEnumTfLanguageProfiles as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), &mut result__).from_abi::<IEnumTfLanguageProfiles>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn EnableLanguageProfile<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, fenable: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), fenable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsEnabledLanguageProfile(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn EnableLanguageProfileByDefault<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, fenable: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SubstituteKeyboardLayout<'a, Param3: ::windows::runtime::IntoParam<'a, HKL>>(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, hkl: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), hkl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetLanguageProfileDisplayName<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, pchfile: Param3, cchfile: u32, uresid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), pchfile.into_param().abi(), ::core::mem::transmute(cchfile), ::core::mem::transmute(uresid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfInputProcessorProfilesEx {
    type Vtable = ITfInputProcessorProfilesEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x892f230f_fe00_4a41_a98e_fcd6de0d35ef);
}
impl ::core::convert::From<ITfInputProcessorProfilesEx> for ::windows::runtime::IUnknown {
    fn from(value: ITfInputProcessorProfilesEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfInputProcessorProfilesEx> for ::windows::runtime::IUnknown {
    fn from(value: &ITfInputProcessorProfilesEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfInputProcessorProfilesEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfInputProcessorProfilesEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfInputProcessorProfilesEx> for ITfInputProcessorProfiles {
    fn from(value: ITfInputProcessorProfilesEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfInputProcessorProfilesEx> for ITfInputProcessorProfiles {
    fn from(value: &ITfInputProcessorProfilesEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfInputProcessorProfiles> for ITfInputProcessorProfilesEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfInputProcessorProfiles> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfInputProcessorProfiles> for &ITfInputProcessorProfilesEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfInputProcessorProfiles> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfilesEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, catid: *const ::windows::runtime::GUID, pclsid: *mut ::windows::runtime::GUID, pguidprofile: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, rclsid: *const ::windows::runtime::GUID, guidprofiles: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofiles: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, pbstrprofile: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plangid: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, hkl: HKL) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, langid: u16, guidprofile: *const ::windows::runtime::GUID, pchfile: super::super::Foundation::PWSTR, cchfile: u32, uresid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfInputScope(pub ::windows::runtime::IUnknown);
impl ITfInputScope {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetInputScopes(&self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprginputscopes), ::core::mem::transmute(pccount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetPhrase(&self, ppbstrphrases: *mut *mut super::super::Foundation::BSTR, pccount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbstrphrases), ::core::mem::transmute(pccount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetRegularExpression(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetSRGS(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetXML(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfInputScope {
    type Vtable = ITfInputScope_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfde1eaee_6924_4cdf_91e7_da38cff5559d);
}
impl ::core::convert::From<ITfInputScope> for ::windows::runtime::IUnknown {
    fn from(value: ITfInputScope) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfInputScope> for ::windows::runtime::IUnknown {
    fn from(value: &ITfInputScope) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfInputScope {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfInputScope {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputScope_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppbstrphrases: *mut *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pccount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrregexp: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsrgs: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrxml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfInputScope2(pub ::windows::runtime::IUnknown);
impl ITfInputScope2 {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetInputScopes(&self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprginputscopes), ::core::mem::transmute(pccount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetPhrase(&self, ppbstrphrases: *mut *mut super::super::Foundation::BSTR, pccount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbstrphrases), ::core::mem::transmute(pccount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetRegularExpression(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetSRGS(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetXML(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn EnumWordList(&self) -> ::windows::runtime::Result<super::super::System::Com::IEnumString> {
        let mut result__: <super::super::System::Com::IEnumString as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IEnumString>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfInputScope2 {
    type Vtable = ITfInputScope2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5731eaa0_6bc2_4681_a532_92fbb74d7c41);
}
impl ::core::convert::From<ITfInputScope2> for ::windows::runtime::IUnknown {
    fn from(value: ITfInputScope2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfInputScope2> for ::windows::runtime::IUnknown {
    fn from(value: &ITfInputScope2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfInputScope2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfInputScope2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfInputScope2> for ITfInputScope {
    fn from(value: ITfInputScope2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfInputScope2> for ITfInputScope {
    fn from(value: &ITfInputScope2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfInputScope> for ITfInputScope2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfInputScope> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfInputScope> for &ITfInputScope2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfInputScope> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputScope2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppbstrphrases: *mut *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pccount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrregexp: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsrgs: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrxml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstring: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfInsertAtSelection(pub ::windows::runtime::IUnknown);
impl ITfInsertAtSelection {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn InsertTextAtSelection<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: Param2, cch: i32) -> ::windows::runtime::Result<ITfRange> {
        let mut result__: <ITfRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), pchtext.into_param().abi(), ::core::mem::transmute(cch), &mut result__).from_abi::<ITfRange>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn InsertEmbeddedAtSelection<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, ec: u32, dwflags: u32, pdataobject: Param2) -> ::windows::runtime::Result<ITfRange> {
        let mut result__: <ITfRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), pdataobject.into_param().abi(), &mut result__).from_abi::<ITfRange>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfInsertAtSelection {
    type Vtable = ITfInsertAtSelection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x55ce16ba_3014_41c1_9ceb_fade1446ac6c);
}
impl ::core::convert::From<ITfInsertAtSelection> for ::windows::runtime::IUnknown {
    fn from(value: ITfInsertAtSelection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfInsertAtSelection> for ::windows::runtime::IUnknown {
    fn from(value: &ITfInsertAtSelection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfInsertAtSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfInsertAtSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInsertAtSelection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: super::super::Foundation::PWSTR, cch: i32, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, dwflags: u32, pdataobject: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfIntegratableCandidateListUIElement(pub ::windows::runtime::IUnknown);
impl ITfIntegratableCandidateListUIElement {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetIntegrationStyle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, guidintegrationstyle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), guidintegrationstyle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetSelectionStyle(&self) -> ::windows::runtime::Result<TfIntegratableCandidateListSelectionStyle> {
        let mut result__: <TfIntegratableCandidateListSelectionStyle as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TfIntegratableCandidateListSelectionStyle>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnKeyDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn ShowCandidateNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn FinalizeExactCompositionString(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfIntegratableCandidateListUIElement {
    type Vtable = ITfIntegratableCandidateListUIElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc7a6f54f_b180_416f_b2bf_7bf2e4683d7b);
}
impl ::core::convert::From<ITfIntegratableCandidateListUIElement> for ::windows::runtime::IUnknown {
    fn from(value: ITfIntegratableCandidateListUIElement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfIntegratableCandidateListUIElement> for ::windows::runtime::IUnknown {
    fn from(value: &ITfIntegratableCandidateListUIElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfIntegratableCandidateListUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfIntegratableCandidateListUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfIntegratableCandidateListUIElement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidintegrationstyle: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptfselectionstyle: *mut TfIntegratableCandidateListSelectionStyle) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfshow: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfKeyEventSink(pub ::windows::runtime::IUnknown);
impl ITfKeyEventSink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnSetFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforeground: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fforeground.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTestKeyDown<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, pic: Param0, wparam: Param1, lparam: Param2) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pic.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTestKeyUp<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, pic: Param0, wparam: Param1, lparam: Param2) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pic.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnKeyDown<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, pic: Param0, wparam: Param1, lparam: Param2) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pic.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnKeyUp<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, pic: Param0, wparam: Param1, lparam: Param2) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pic.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnPreservedKey<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>>(&self, pic: Param0, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(rguid), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfKeyEventSink {
    type Vtable = ITfKeyEventSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e7f5_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<ITfKeyEventSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfKeyEventSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfKeyEventSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfKeyEventSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfKeyEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfKeyEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfKeyEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fforeground: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfKeyTraceEventSink(pub ::windows::runtime::IUnknown);
impl ITfKeyTraceEventSink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnKeyTraceDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnKeyTraceUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfKeyTraceEventSink {
    type Vtable = ITfKeyTraceEventSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1cd4c13b_1c36_4191_a70a_7f3e611f367d);
}
impl ::core::convert::From<ITfKeyTraceEventSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfKeyTraceEventSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfKeyTraceEventSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfKeyTraceEventSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfKeyTraceEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfKeyTraceEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfKeyTraceEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfKeystrokeMgr(pub ::windows::runtime::IUnknown);
impl ITfKeystrokeMgr {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn AdviseKeyEventSink<'a, Param1: ::windows::runtime::IntoParam<'a, ITfKeyEventSink>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, tid: u32, psink: Param1, fforeground: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), psink.into_param().abi(), fforeground.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnadviseKeyEventSink(&self, tid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetForeground(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn TestKeyDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn TestKeyUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn KeyDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn KeyUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetPreservedKey<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>>(&self, pic: Param0, pprekey: *const TF_PRESERVEDKEY) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(pprekey), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsPreservedKey(&self, rguid: *const ::windows::runtime::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(pprekey), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn PreserveKey<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, tid: u32, rguid: *const ::windows::runtime::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: Param3, cchdesc: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), ::core::mem::transmute(rguid), ::core::mem::transmute(prekey), pchdesc.into_param().abi(), ::core::mem::transmute(cchdesc)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnpreserveKey(&self, rguid: *const ::windows::runtime::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(pprekey)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetPreservedKeyDescription<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rguid: *const ::windows::runtime::GUID, pchdesc: Param1, cchdesc: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), pchdesc.into_param().abi(), ::core::mem::transmute(cchdesc)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetPreservedKeyDescription(&self, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SimulatePreservedKey<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>>(&self, pic: Param0, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(rguid), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfKeystrokeMgr {
    type Vtable = ITfKeystrokeMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e7f0_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<ITfKeystrokeMgr> for ::windows::runtime::IUnknown {
    fn from(value: ITfKeystrokeMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfKeystrokeMgr> for ::windows::runtime::IUnknown {
    fn from(value: &ITfKeystrokeMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfKeystrokeMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfKeystrokeMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfKeystrokeMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tid: u32, psink: ::windows::runtime::RawPtr, fforeground: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, pprekey: *const TF_PRESERVEDKEY, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, pprekey: *const TF_PRESERVEDKEY, pfregistered: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tid: u32, rguid: *const ::windows::runtime::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, pbstrdesc: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfLMLattice(pub ::windows::runtime::IUnknown);
impl ITfLMLattice {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn QueryType(&self, rguidtype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidtype), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumLatticeElements(&self, dwframestart: u32, rguidtype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IEnumTfLatticeElements> {
        let mut result__: <IEnumTfLatticeElements as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwframestart), ::core::mem::transmute(rguidtype), &mut result__).from_abi::<IEnumTfLatticeElements>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfLMLattice {
    type Vtable = ITfLMLattice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd4236675_a5bf_4570_9d42_5d6d7b02d59b);
}
impl ::core::convert::From<ITfLMLattice> for ::windows::runtime::IUnknown {
    fn from(value: ITfLMLattice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfLMLattice> for ::windows::runtime::IUnknown {
    fn from(value: &ITfLMLattice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfLMLattice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfLMLattice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLMLattice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidtype: *const ::windows::runtime::GUID, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwframestart: u32, rguidtype: *const ::windows::runtime::GUID, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfLangBarEventSink(pub ::windows::runtime::IUnknown);
impl ITfLangBarEventSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnSetFocus(&self, dwthreadid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnThreadTerminate(&self, dwthreadid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnThreadItemChange(&self, dwthreadid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnModalInput<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, dwthreadid: u32, umsg: u32, wparam: Param2, lparam: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(umsg), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShowFloating(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(rguid), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfLangBarEventSink {
    type Vtable = ITfLangBarEventSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x18a4e900_e0ae_11d2_afdd_00105a2799b5);
}
impl ::core::convert::From<ITfLangBarEventSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfLangBarEventSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfLangBarEventSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfLangBarEventSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfLangBarEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfLangBarEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwthreadid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwthreadid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwthreadid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwthreadid: u32, rguid: *const ::windows::runtime::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfLangBarItem(pub ::windows::runtime::IUnknown);
impl ITfLangBarItem {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetInfo(&self) -> ::windows::runtime::Result<TF_LANGBARITEMINFO> {
        let mut result__: <TF_LANGBARITEMINFO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TF_LANGBARITEMINFO>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTooltipString(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfLangBarItem {
    type Vtable = ITfLangBarItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x73540d69_edeb_4ee9_96c9_23aa30b25916);
}
impl ::core::convert::From<ITfLangBarItem> for ::windows::runtime::IUnknown {
    fn from(value: ITfLangBarItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfLangBarItem> for ::windows::runtime::IUnknown {
    fn from(value: &ITfLangBarItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfLangBarItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfLangBarItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtooltip: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfLangBarItemBalloon(pub ::windows::runtime::IUnknown);
impl ITfLangBarItemBalloon {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetInfo(&self) -> ::windows::runtime::Result<TF_LANGBARITEMINFO> {
        let mut result__: <TF_LANGBARITEMINFO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TF_LANGBARITEMINFO>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTooltipString(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnClick<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINT>>(&self, click: TfLBIClick, pt: Param1, prcarea: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(click), pt.into_param().abi(), ::core::mem::transmute(prcarea)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::runtime::Result<super::super::Foundation::SIZE> {
        let mut result__: <super::super::Foundation::SIZE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszdefault), &mut result__).from_abi::<super::super::Foundation::SIZE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetBalloonInfo(&self) -> ::windows::runtime::Result<TF_LBBALLOONINFO> {
        let mut result__: <TF_LBBALLOONINFO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TF_LBBALLOONINFO>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfLangBarItemBalloon {
    type Vtable = ITfLangBarItemBalloon_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x01c2d285_d3c7_4b7b_b5b5_d97411d0c283);
}
impl ::core::convert::From<ITfLangBarItemBalloon> for ::windows::runtime::IUnknown {
    fn from(value: ITfLangBarItemBalloon) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfLangBarItemBalloon> for ::windows::runtime::IUnknown {
    fn from(value: &ITfLangBarItemBalloon) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfLangBarItemBalloon {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfLangBarItemBalloon {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfLangBarItemBalloon> for ITfLangBarItem {
    fn from(value: ITfLangBarItemBalloon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarItemBalloon> for ITfLangBarItem {
    fn from(value: &ITfLangBarItemBalloon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfLangBarItem> for ITfLangBarItemBalloon {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfLangBarItem> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfLangBarItem> for &ITfLangBarItemBalloon {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfLangBarItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemBalloon_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtooltip: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *mut ::core::mem::ManuallyDrop<TF_LBBALLOONINFO>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfLangBarItemBitmap(pub ::windows::runtime::IUnknown);
impl ITfLangBarItemBitmap {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetInfo(&self) -> ::windows::runtime::Result<TF_LANGBARITEMINFO> {
        let mut result__: <TF_LANGBARITEMINFO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TF_LANGBARITEMINFO>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTooltipString(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnClick<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINT>>(&self, click: TfLBIClick, pt: Param1, prcarea: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(click), pt.into_param().abi(), ::core::mem::transmute(prcarea)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::runtime::Result<super::super::Foundation::SIZE> {
        let mut result__: <super::super::Foundation::SIZE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszdefault), &mut result__).from_abi::<super::super::Foundation::SIZE>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(bmwidth), ::core::mem::transmute(bmheight), ::core::mem::transmute(dwflags), ::core::mem::transmute(phbmp), ::core::mem::transmute(phbmpmask)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfLangBarItemBitmap {
    type Vtable = ITfLangBarItemBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x73830352_d722_4179_ada5_f045c98df355);
}
impl ::core::convert::From<ITfLangBarItemBitmap> for ::windows::runtime::IUnknown {
    fn from(value: ITfLangBarItemBitmap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfLangBarItemBitmap> for ::windows::runtime::IUnknown {
    fn from(value: &ITfLangBarItemBitmap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfLangBarItemBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfLangBarItemBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfLangBarItemBitmap> for ITfLangBarItem {
    fn from(value: ITfLangBarItemBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarItemBitmap> for ITfLangBarItem {
    fn from(value: &ITfLangBarItemBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfLangBarItem> for ITfLangBarItemBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfLangBarItem> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfLangBarItem> for &ITfLangBarItemBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfLangBarItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemBitmap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtooltip: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfLangBarItemBitmapButton(pub ::windows::runtime::IUnknown);
impl ITfLangBarItemBitmapButton {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetInfo(&self) -> ::windows::runtime::Result<TF_LANGBARITEMINFO> {
        let mut result__: <TF_LANGBARITEMINFO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TF_LANGBARITEMINFO>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTooltipString(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnClick<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINT>>(&self, click: TfLBIClick, pt: Param1, prcarea: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(click), pt.into_param().abi(), ::core::mem::transmute(prcarea)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn InitMenu<'a, Param0: ::windows::runtime::IntoParam<'a, ITfMenu>>(&self, pmenu: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pmenu.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnMenuSelect(&self, wid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::runtime::Result<super::super::Foundation::SIZE> {
        let mut result__: <super::super::Foundation::SIZE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszdefault), &mut result__).from_abi::<super::super::Foundation::SIZE>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(bmwidth), ::core::mem::transmute(bmheight), ::core::mem::transmute(dwflags), ::core::mem::transmute(phbmp), ::core::mem::transmute(phbmpmask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfLangBarItemBitmapButton {
    type Vtable = ITfLangBarItemBitmapButton_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa26a0525_3fae_4fa0_89ee_88a964f9f1b5);
}
impl ::core::convert::From<ITfLangBarItemBitmapButton> for ::windows::runtime::IUnknown {
    fn from(value: ITfLangBarItemBitmapButton) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfLangBarItemBitmapButton> for ::windows::runtime::IUnknown {
    fn from(value: &ITfLangBarItemBitmapButton) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfLangBarItemBitmapButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfLangBarItemBitmapButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfLangBarItemBitmapButton> for ITfLangBarItem {
    fn from(value: ITfLangBarItemBitmapButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarItemBitmapButton> for ITfLangBarItem {
    fn from(value: &ITfLangBarItemBitmapButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfLangBarItem> for ITfLangBarItemBitmapButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfLangBarItem> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfLangBarItem> for &ITfLangBarItemBitmapButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfLangBarItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemBitmapButton_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtooltip: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmenu: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfLangBarItemButton(pub ::windows::runtime::IUnknown);
impl ITfLangBarItemButton {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetInfo(&self) -> ::windows::runtime::Result<TF_LANGBARITEMINFO> {
        let mut result__: <TF_LANGBARITEMINFO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TF_LANGBARITEMINFO>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetTooltipString(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnClick<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINT>>(&self, click: TfLBIClick, pt: Param1, prcarea: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(click), pt.into_param().abi(), ::core::mem::transmute(prcarea)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn InitMenu<'a, Param0: ::windows::runtime::IntoParam<'a, ITfMenu>>(&self, pmenu: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pmenu.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnMenuSelect(&self, wid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wid)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetIcon(&self) -> ::windows::runtime::Result<super::WindowsAndMessaging::HICON> {
        let mut result__: <super::WindowsAndMessaging::HICON as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfLangBarItemButton {
    type Vtable = ITfLangBarItemButton_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x28c7f1d0_de25_11d2_afdd_00105a2799b5);
}
impl ::core::convert::From<ITfLangBarItemButton> for ::windows::runtime::IUnknown {
    fn from(value: ITfLangBarItemButton) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfLangBarItemButton> for ::windows::runtime::IUnknown {
    fn from(value: &ITfLangBarItemButton) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfLangBarItemButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfLangBarItemButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfLangBarItemButton> for ITfLangBarItem {
    fn from(value: ITfLangBarItemButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarItemButton> for ITfLangBarItem {
    fn from(value: &ITfLangBarItemButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfLangBarItem> for ITfLangBarItemButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfLangBarItem> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfLangBarItem> for &ITfLangBarItemButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfLangBarItem> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemButton_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtooltip: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmenu: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phicon: *mut super::WindowsAndMessaging::HICON) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfLangBarItemMgr(pub ::windows::runtime::IUnknown);
impl ITfLangBarItemMgr {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumItems(&self) -> ::windows::runtime::Result<IEnumTfLangBarItems> {
        let mut result__: <IEnumTfLangBarItems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfLangBarItems>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetItem(&self, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ITfLangBarItem> {
        let mut result__: <ITfLangBarItem as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), &mut result__).from_abi::<ITfLangBarItem>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn AddItem<'a, Param0: ::windows::runtime::IntoParam<'a, ITfLangBarItem>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn RemoveItem<'a, Param0: ::windows::runtime::IntoParam<'a, ITfLangBarItem>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn AdviseItemSink<'a, Param0: ::windows::runtime::IntoParam<'a, ITfLangBarItemSink>>(&self, punk: Param0, pdwcookie: *mut u32, rguiditem: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), punk.into_param().abi(), ::core::mem::transmute(pdwcookie), ::core::mem::transmute(rguiditem)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnadviseItemSink(&self, dwcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(rguid), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetItemsStatus(&self, ulcount: u32, prgguid: *const ::windows::runtime::GUID, pdwstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(prgguid), ::core::mem::transmute(pdwstatus)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetItemNum(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetItems(&self, ulcount: u32, ppitem: *mut ::core::option::Option<ITfLangBarItem>, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppitem), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdwstatus), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn AdviseItemsSink(&self, ulcount: u32, ppunk: *const ::core::option::Option<ITfLangBarItemSink>, pguiditem: *const ::windows::runtime::GUID, pdwcookie: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppunk), ::core::mem::transmute(pguiditem), ::core::mem::transmute(pdwcookie)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnadviseItemsSink(&self, ulcount: u32, pdwcookie: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pdwcookie)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfLangBarItemMgr {
    type Vtable = ITfLangBarItemMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xba468c55_9956_4fb1_a59d_52a7dd7cc6aa);
}
impl ::core::convert::From<ITfLangBarItemMgr> for ::windows::runtime::IUnknown {
    fn from(value: ITfLangBarItemMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfLangBarItemMgr> for ::windows::runtime::IUnknown {
    fn from(value: &ITfLangBarItemMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfLangBarItemMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfLangBarItemMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr, pdwcookie: *mut u32, rguiditem: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwthreadid: u32, rguid: *const ::windows::runtime::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, prgguid: *const ::windows::runtime::GUID, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, ppitem: *mut ::windows::runtime::RawPtr, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, ppunk: *const ::windows::runtime::RawPtr, pguiditem: *const ::windows::runtime::GUID, pdwcookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, pdwcookie: *const u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfLangBarItemSink(pub ::windows::runtime::IUnknown);
impl ITfLangBarItemSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnUpdate(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfLangBarItemSink {
    type Vtable = ITfLangBarItemSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x57dbe1a0_de25_11d2_afdd_00105a2799b5);
}
impl ::core::convert::From<ITfLangBarItemSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfLangBarItemSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfLangBarItemSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfLangBarItemSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfLangBarItemSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfLangBarItemSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfLangBarMgr(pub ::windows::runtime::IUnknown);
impl ITfLangBarMgr {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn AdviseEventSink<'a, Param0: ::windows::runtime::IntoParam<'a, ITfLangBarEventSink>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, psink: Param0, hwnd: Param1, dwflags: u32, pdwcookie: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psink.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pdwcookie)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnadviseEventSink(&self, dwcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetThreadMarshalInterface(&self, dwthreadid: u32, dwtype: u32, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(dwtype), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetThreadLangBarItemMgr(&self, dwthreadid: u32, pplbi: *mut ::core::option::Option<ITfLangBarItemMgr>, pdwthreadid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(pplbi), ::core::mem::transmute(pdwthreadid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetInputProcessorProfiles(&self, dwthreadid: u32, ppaip: *mut ::core::option::Option<ITfInputProcessorProfiles>, pdwthreadid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(ppaip), ::core::mem::transmute(pdwthreadid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn RestoreLastFocus<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pdwthreadid: *mut u32, fprev: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwthreadid), fprev.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetModalInput<'a, Param0: ::windows::runtime::IntoParam<'a, ITfLangBarEventSink>>(&self, psink: Param0, dwthreadid: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), psink.into_param().abi(), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShowFloating(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetShowFloatingStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfLangBarMgr {
    type Vtable = ITfLangBarMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x87955690_e627_11d2_8ddb_00105a2799b5);
}
impl ::core::convert::From<ITfLangBarMgr> for ::windows::runtime::IUnknown {
    fn from(value: ITfLangBarMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfLangBarMgr> for ::windows::runtime::IUnknown {
    fn from(value: &ITfLangBarMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfLangBarMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfLangBarMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwthreadid: u32, dwtype: u32, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwthreadid: u32, pplbi: *mut ::windows::runtime::RawPtr, pdwthreadid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwthreadid: u32, ppaip: *mut ::windows::runtime::RawPtr, pdwthreadid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr, dwthreadid: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfLanguageProfileNotifySink(pub ::windows::runtime::IUnknown);
impl ITfLanguageProfileNotifySink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnLanguageChange(&self, langid: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnLanguageChanged(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfLanguageProfileNotifySink {
    type Vtable = ITfLanguageProfileNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x43c9fe15_f494_4c17_9de2_b8a4ac350aa8);
}
impl ::core::convert::From<ITfLanguageProfileNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: ITfLanguageProfileNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfLanguageProfileNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfLanguageProfileNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfLanguageProfileNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfLanguageProfileNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLanguageProfileNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfMSAAControl(pub ::windows::runtime::IUnknown);
impl ITfMSAAControl {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SystemEnableMSAA(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SystemDisableMSAA(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfMSAAControl {
    type Vtable = ITfMSAAControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb5f8fb3b_393f_4f7c_84cb_504924c2705a);
}
impl ::core::convert::From<ITfMSAAControl> for ::windows::runtime::IUnknown {
    fn from(value: ITfMSAAControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfMSAAControl> for ::windows::runtime::IUnknown {
    fn from(value: &ITfMSAAControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfMSAAControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfMSAAControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMSAAControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfMenu(pub ::windows::runtime::IUnknown);
impl ITfMenu {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn AddMenuItem<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>, Param3: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, uid: u32, dwflags: u32, hbmp: Param2, hbmpmask: Param3, pch: Param4, cch: u32, ppmenu: *mut ::core::option::Option<ITfMenu>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uid), ::core::mem::transmute(dwflags), hbmp.into_param().abi(), hbmpmask.into_param().abi(), pch.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(ppmenu)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfMenu {
    type Vtable = ITfMenu_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6f8a98e4_aaa0_4f15_8c5b_07e0df0a3dd8);
}
impl ::core::convert::From<ITfMenu> for ::windows::runtime::IUnknown {
    fn from(value: ITfMenu) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfMenu> for ::windows::runtime::IUnknown {
    fn from(value: &ITfMenu) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfMenu {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfMenu {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMenu_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: super::super::Foundation::PWSTR, cch: u32, ppmenu: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfMessagePump(pub ::windows::runtime::IUnknown);
impl ITfMessagePump {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn PeekMessageA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: Param1, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), hwnd.into_param().abi(), ::core::mem::transmute(wmsgfiltermin), ::core::mem::transmute(wmsgfiltermax), ::core::mem::transmute(wremovemsg), ::core::mem::transmute(pfresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetMessageA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: Param1, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), hwnd.into_param().abi(), ::core::mem::transmute(wmsgfiltermin), ::core::mem::transmute(wmsgfiltermax), ::core::mem::transmute(pfresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn PeekMessageW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: Param1, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), hwnd.into_param().abi(), ::core::mem::transmute(wmsgfiltermin), ::core::mem::transmute(wmsgfiltermax), ::core::mem::transmute(wremovemsg), ::core::mem::transmute(pfresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetMessageW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: Param1, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), hwnd.into_param().abi(), ::core::mem::transmute(wmsgfiltermin), ::core::mem::transmute(wmsgfiltermax), ::core::mem::transmute(pfresult)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfMessagePump {
    type Vtable = ITfMessagePump_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8f1b8ad8_0b6b_4874_90c5_bd76011e8f7c);
}
impl ::core::convert::From<ITfMessagePump> for ::windows::runtime::IUnknown {
    fn from(value: ITfMessagePump) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfMessagePump> for ::windows::runtime::IUnknown {
    fn from(value: &ITfMessagePump) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfMessagePump {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfMessagePump {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMessagePump_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfMouseSink(pub ::windows::runtime::IUnknown);
impl ITfMouseSink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnMouseEvent(&self, uedge: u32, uquadrant: u32, dwbtnstatus: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uedge), ::core::mem::transmute(uquadrant), ::core::mem::transmute(dwbtnstatus), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfMouseSink {
    type Vtable = ITfMouseSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa1adaaa2_3a24_449d_ac96_5183e7f5c217);
}
impl ::core::convert::From<ITfMouseSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfMouseSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfMouseSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfMouseSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfMouseSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfMouseSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMouseSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uedge: u32, uquadrant: u32, dwbtnstatus: u32, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfMouseTracker(pub ::windows::runtime::IUnknown);
impl ITfMouseTracker {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn AdviseMouseSink<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>, Param1: ::windows::runtime::IntoParam<'a, ITfMouseSink>>(&self, range: Param0, psink: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), range.into_param().abi(), psink.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnadviseMouseSink(&self, dwcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfMouseTracker {
    type Vtable = ITfMouseTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x09d146cd_a544_4132_925b_7afa8ef322d0);
}
impl ::core::convert::From<ITfMouseTracker> for ::windows::runtime::IUnknown {
    fn from(value: ITfMouseTracker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfMouseTracker> for ::windows::runtime::IUnknown {
    fn from(value: &ITfMouseTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfMouseTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfMouseTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMouseTracker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, range: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr, pdwcookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfMouseTrackerACP(pub ::windows::runtime::IUnknown);
impl ITfMouseTrackerACP {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn AdviseMouseSink<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRangeACP>, Param1: ::windows::runtime::IntoParam<'a, ITfMouseSink>>(&self, range: Param0, psink: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), range.into_param().abi(), psink.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnadviseMouseSink(&self, dwcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfMouseTrackerACP {
    type Vtable = ITfMouseTrackerACP_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3bdd78e2_c16e_47fd_b883_ce6facc1a208);
}
impl ::core::convert::From<ITfMouseTrackerACP> for ::windows::runtime::IUnknown {
    fn from(value: ITfMouseTrackerACP) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfMouseTrackerACP> for ::windows::runtime::IUnknown {
    fn from(value: &ITfMouseTrackerACP) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfMouseTrackerACP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfMouseTrackerACP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMouseTrackerACP_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, range: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr, pdwcookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfPersistentPropertyLoaderACP(pub ::windows::runtime::IUnknown);
impl ITfPersistentPropertyLoaderACP {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn LoadProperty(&self, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(phdr), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfPersistentPropertyLoaderACP {
    type Vtable = ITfPersistentPropertyLoaderACP_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4ef89150_0807_11d3_8df0_00105a2799b5);
}
impl ::core::convert::From<ITfPersistentPropertyLoaderACP> for ::windows::runtime::IUnknown {
    fn from(value: ITfPersistentPropertyLoaderACP) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfPersistentPropertyLoaderACP> for ::windows::runtime::IUnknown {
    fn from(value: &ITfPersistentPropertyLoaderACP) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfPersistentPropertyLoaderACP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfPersistentPropertyLoaderACP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPersistentPropertyLoaderACP_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, ppstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfPreservedKeyNotifySink(pub ::windows::runtime::IUnknown);
impl ITfPreservedKeyNotifySink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnUpdated(&self, pprekey: *const TF_PRESERVEDKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprekey)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfPreservedKeyNotifySink {
    type Vtable = ITfPreservedKeyNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6f77c993_d2b1_446e_853e_5912efc8a286);
}
impl ::core::convert::From<ITfPreservedKeyNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: ITfPreservedKeyNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfPreservedKeyNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfPreservedKeyNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfPreservedKeyNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfPreservedKeyNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPreservedKeyNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprekey: *const TF_PRESERVEDKEY) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfProperty(pub ::windows::runtime::IUnknown);
impl ITfProperty {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumRanges<'a, Param2: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, ppenum: *mut ::core::option::Option<IEnumTfRanges>, ptargetrange: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(ppenum), ptargetrange.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetValue<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetContext(&self) -> ::windows::runtime::Result<ITfContext> {
        let mut result__: <ITfContext as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfContext>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn FindRange<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, pprange: *mut ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(pprange), ::core::mem::transmute(apos)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetValueStore<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>, Param2: ::windows::runtime::IntoParam<'a, ITfPropertyStore>>(&self, ec: u32, prange: Param1, ppropstore: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ppropstore.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetValue<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clear<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfProperty {
    type Vtable = ITfProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe2449660_9542_11d2_bf46_00105a2799b5);
}
impl ::core::convert::From<ITfProperty> for ::windows::runtime::IUnknown {
    fn from(value: ITfProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfProperty> for ::windows::runtime::IUnknown {
    fn from(value: &ITfProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfProperty> for ITfReadOnlyProperty {
    fn from(value: ITfProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfProperty> for ITfReadOnlyProperty {
    fn from(value: &ITfProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfReadOnlyProperty> for ITfProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfReadOnlyProperty> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfReadOnlyProperty> for &ITfProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfReadOnlyProperty> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, ppenum: *mut ::windows::runtime::RawPtr, ptargetrange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, pvarvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr, apos: TfAnchor) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, ppropstore: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, pvarvalue: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfPropertyStore(pub ::windows::runtime::IUnknown);
impl ITfPropertyStore {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetDataType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetData(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTextUpdated<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, dwflags: u32, prangenew: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), prangenew.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Shrink<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prangenew: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), prangenew.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Divide<'a, Param0: ::windows::runtime::IntoParam<'a, ITfRange>, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, prangethis: Param0, prangenew: Param1) -> ::windows::runtime::Result<ITfPropertyStore> {
        let mut result__: <ITfPropertyStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), prangethis.into_param().abi(), prangenew.into_param().abi(), &mut result__).from_abi::<ITfPropertyStore>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<ITfPropertyStore> {
        let mut result__: <ITfPropertyStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfPropertyStore>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetPropertyRangeCreator(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn Serialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pstream.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfPropertyStore {
    type Vtable = ITfPropertyStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6834b120_88cb_11d2_bf45_00105a2799b5);
}
impl ::core::convert::From<ITfPropertyStore> for ::windows::runtime::IUnknown {
    fn from(value: ITfPropertyStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfPropertyStore> for ::windows::runtime::IUnknown {
    fn from(value: &ITfPropertyStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPropertyStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwreserved: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, prangenew: ::windows::runtime::RawPtr, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prangenew: ::windows::runtime::RawPtr, pffree: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prangethis: ::windows::runtime::RawPtr, prangenew: ::windows::runtime::RawPtr, pppropstore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropstore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, pcb: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfQueryEmbedded(pub ::windows::runtime::IUnknown);
impl ITfQueryEmbedded {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const ::windows::runtime::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidservice), ::core::mem::transmute(pformatetc), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfQueryEmbedded {
    type Vtable = ITfQueryEmbedded_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0fab9bdb_d250_4169_84e5_6be118fdd7a8);
}
impl ::core::convert::From<ITfQueryEmbedded> for ::windows::runtime::IUnknown {
    fn from(value: ITfQueryEmbedded) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfQueryEmbedded> for ::windows::runtime::IUnknown {
    fn from(value: &ITfQueryEmbedded) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfQueryEmbedded {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfQueryEmbedded {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfQueryEmbedded_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidservice: *const ::windows::runtime::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfRange(pub ::windows::runtime::IUnknown);
impl ITfRange {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetText(&self, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), ::core::mem::transmute(pchtext), ::core::mem::transmute(cchmax), ::core::mem::transmute(pcch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetText<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ec: u32, dwflags: u32, pchtext: Param2, cch: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), pchtext.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn GetFormattedText(&self, ec: u32) -> ::windows::runtime::Result<super::super::System::Com::IDataObject> {
        let mut result__: <super::super::System::Com::IDataObject as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), &mut result__).from_abi::<super::super::System::Com::IDataObject>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetEmbedded(&self, ec: u32, rguidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(rguidservice), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn InsertEmbedded<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, ec: u32, dwflags: u32, pdataobject: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), pdataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShiftStart(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(cchreq), ::core::mem::transmute(pcch), ::core::mem::transmute(phalt)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShiftEnd(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(cchreq), ::core::mem::transmute(pcch), ::core::mem::transmute(phalt)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShiftStartToRange<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, apos: TfAnchor) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(apos)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShiftEndToRange<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, apos: TfAnchor) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(apos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn ShiftStartRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dir), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn ShiftEndRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dir), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsEmpty(&self, ec: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Collapse(&self, ec: u32, apos: TfAnchor) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(apos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsEqualStart<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsEqualEnd<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn CompareStart<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn CompareEnd<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn AdjustForInsert(&self, ec: u32, cchinsert: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(cchinsert), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(pgstart), ::core::mem::transmute(pgend)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetGravity(&self, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(gstart), ::core::mem::transmute(gend)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<ITfRange> {
        let mut result__: <ITfRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetContext(&self) -> ::windows::runtime::Result<ITfContext> {
        let mut result__: <ITfContext as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfContext>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfRange {
    type Vtable = ITfRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e7ff_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<ITfRange> for ::windows::runtime::IUnknown {
    fn from(value: ITfRange) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfRange> for ::windows::runtime::IUnknown {
    fn from(value: &ITfRange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfRange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, ppdataobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, rguidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, dwflags: u32, pdataobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const ::core::mem::ManuallyDrop<TF_HALTCOND>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const ::core::mem::ManuallyDrop<TF_HALTCOND>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, apos: TfAnchor) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, apos: TfAnchor) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, apos: TfAnchor) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, pwith: ::windows::runtime::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, pwith: ::windows::runtime::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, pwith: ::windows::runtime::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, pwith: ::windows::runtime::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclone: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfRangeACP(pub ::windows::runtime::IUnknown);
impl ITfRangeACP {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetText(&self, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), ::core::mem::transmute(pchtext), ::core::mem::transmute(cchmax), ::core::mem::transmute(pcch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetText<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ec: u32, dwflags: u32, pchtext: Param2, cch: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), pchtext.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn GetFormattedText(&self, ec: u32) -> ::windows::runtime::Result<super::super::System::Com::IDataObject> {
        let mut result__: <super::super::System::Com::IDataObject as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), &mut result__).from_abi::<super::super::System::Com::IDataObject>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetEmbedded(&self, ec: u32, rguidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(rguidservice), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_System_Com`*"]
    pub unsafe fn InsertEmbedded<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, ec: u32, dwflags: u32, pdataobject: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), pdataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShiftStart(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(cchreq), ::core::mem::transmute(pcch), ::core::mem::transmute(phalt)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShiftEnd(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(cchreq), ::core::mem::transmute(pcch), ::core::mem::transmute(phalt)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShiftStartToRange<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, apos: TfAnchor) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(apos)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ShiftEndToRange<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, apos: TfAnchor) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(apos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn ShiftStartRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dir), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn ShiftEndRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dir), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsEmpty(&self, ec: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Collapse(&self, ec: u32, apos: TfAnchor) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(apos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsEqualStart<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsEqualEnd<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn CompareStart<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn CompareEnd<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn AdjustForInsert(&self, ec: u32, cchinsert: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(cchinsert), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(pgstart), ::core::mem::transmute(pgend)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetGravity(&self, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(gstart), ::core::mem::transmute(gend)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<ITfRange> {
        let mut result__: <ITfRange as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfRange>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetContext(&self) -> ::windows::runtime::Result<ITfContext> {
        let mut result__: <ITfContext as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfContext>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetExtent(&self, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(pacpanchor), ::core::mem::transmute(pcch)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetExtent(&self, acpanchor: i32, cch: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpanchor), ::core::mem::transmute(cch)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfRangeACP {
    type Vtable = ITfRangeACP_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x057a6296_029b_4154_b79a_0d461d4ea94c);
}
impl ::core::convert::From<ITfRangeACP> for ::windows::runtime::IUnknown {
    fn from(value: ITfRangeACP) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfRangeACP> for ::windows::runtime::IUnknown {
    fn from(value: &ITfRangeACP) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfRangeACP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfRangeACP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfRangeACP> for ITfRange {
    fn from(value: ITfRangeACP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfRangeACP> for ITfRange {
    fn from(value: &ITfRangeACP) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfRange> for ITfRangeACP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfRange> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfRange> for &ITfRangeACP {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfRange> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfRangeACP_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, ppdataobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, rguidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, dwflags: u32, pdataobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const ::core::mem::ManuallyDrop<TF_HALTCOND>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const ::core::mem::ManuallyDrop<TF_HALTCOND>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, apos: TfAnchor) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, apos: TfAnchor) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, apos: TfAnchor) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, pwith: ::windows::runtime::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, pwith: ::windows::runtime::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, pwith: ::windows::runtime::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, pwith: ::windows::runtime::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclone: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, acpanchor: i32, cch: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfRangeBackup(pub ::windows::runtime::IUnknown);
impl ITfRangeBackup {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Restore<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfRangeBackup {
    type Vtable = ITfRangeBackup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x463a506d_6992_49d2_9b88_93d55e70bb16);
}
impl ::core::convert::From<ITfRangeBackup> for ::windows::runtime::IUnknown {
    fn from(value: ITfRangeBackup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfRangeBackup> for ::windows::runtime::IUnknown {
    fn from(value: &ITfRangeBackup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfRangeBackup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfRangeBackup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfRangeBackup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfReadOnlyProperty(pub ::windows::runtime::IUnknown);
impl ITfReadOnlyProperty {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumRanges<'a, Param2: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, ppenum: *mut ::core::option::Option<IEnumTfRanges>, ptargetrange: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(ppenum), ptargetrange.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn GetValue<'a, Param1: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetContext(&self) -> ::windows::runtime::Result<ITfContext> {
        let mut result__: <ITfContext as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfContext>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfReadOnlyProperty {
    type Vtable = ITfReadOnlyProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x17d49a3d_f8b8_4b2f_b254_52319dd64c53);
}
impl ::core::convert::From<ITfReadOnlyProperty> for ::windows::runtime::IUnknown {
    fn from(value: ITfReadOnlyProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfReadOnlyProperty> for ::windows::runtime::IUnknown {
    fn from(value: &ITfReadOnlyProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfReadOnlyProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfReadOnlyProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReadOnlyProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, ppenum: *mut ::windows::runtime::RawPtr, ptargetrange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, pvarvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfReadingInformationUIElement(pub ::windows::runtime::IUnknown);
impl ITfReadingInformationUIElement {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGUID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsShown(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetUpdatedFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetContext(&self) -> ::windows::runtime::Result<ITfContext> {
        let mut result__: <ITfContext as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfContext>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetString(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetMaxReadingStringLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetErrorIndex(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsVerticalOrderPreferred(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfReadingInformationUIElement {
    type Vtable = ITfReadingInformationUIElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea1ea139_19df_11d7_a6d2_00065b84435c);
}
impl ::core::convert::From<ITfReadingInformationUIElement> for ::windows::runtime::IUnknown {
    fn from(value: ITfReadingInformationUIElement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfReadingInformationUIElement> for ::windows::runtime::IUnknown {
    fn from(value: &ITfReadingInformationUIElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfReadingInformationUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfReadingInformationUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfReadingInformationUIElement> for ITfUIElement {
    fn from(value: ITfReadingInformationUIElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfReadingInformationUIElement> for ITfUIElement {
    fn from(value: &ITfReadingInformationUIElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfUIElement> for ITfReadingInformationUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfUIElement> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfUIElement> for &ITfReadingInformationUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfUIElement> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReadingInformationUIElement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppic: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstr: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, perrorindex: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfvertical: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfReverseConversion(pub ::windows::runtime::IUnknown);
impl ITfReverseConversion {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn DoReverseConversion<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lpstr: Param0) -> ::windows::runtime::Result<ITfReverseConversionList> {
        let mut result__: <ITfReverseConversionList as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), lpstr.into_param().abi(), &mut result__).from_abi::<ITfReverseConversionList>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfReverseConversion {
    type Vtable = ITfReverseConversion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa415e162_157d_417d_8a8c_0ab26c7d2781);
}
impl ::core::convert::From<ITfReverseConversion> for ::windows::runtime::IUnknown {
    fn from(value: ITfReverseConversion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfReverseConversion> for ::windows::runtime::IUnknown {
    fn from(value: &ITfReverseConversion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfReverseConversion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfReverseConversion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReverseConversion_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpstr: super::super::Foundation::PWSTR, pplist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfReverseConversionList(pub ::windows::runtime::IUnknown);
impl ITfReverseConversionList {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetString(&self, uindex: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfReverseConversionList {
    type Vtable = ITfReverseConversionList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x151d69f0_86f4_4674_b721_56911e797f47);
}
impl ::core::convert::From<ITfReverseConversionList> for ::windows::runtime::IUnknown {
    fn from(value: ITfReverseConversionList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfReverseConversionList> for ::windows::runtime::IUnknown {
    fn from(value: &ITfReverseConversionList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfReverseConversionList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfReverseConversionList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReverseConversionList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puindex: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uindex: u32, pbstr: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfReverseConversionMgr(pub ::windows::runtime::IUnknown);
impl ITfReverseConversionMgr {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetReverseConversion(&self, langid: u16, guidprofile: *const ::windows::runtime::GUID, dwflag: u32) -> ::windows::runtime::Result<ITfReverseConversion> {
        let mut result__: <ITfReverseConversion as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), ::core::mem::transmute(dwflag), &mut result__).from_abi::<ITfReverseConversion>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfReverseConversionMgr {
    type Vtable = ITfReverseConversionMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb643c236_c493_41b6_abb3_692412775cc4);
}
impl ::core::convert::From<ITfReverseConversionMgr> for ::windows::runtime::IUnknown {
    fn from(value: ITfReverseConversionMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfReverseConversionMgr> for ::windows::runtime::IUnknown {
    fn from(value: &ITfReverseConversionMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfReverseConversionMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfReverseConversionMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReverseConversionMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, langid: u16, guidprofile: *const ::windows::runtime::GUID, dwflag: u32, ppreverseconversion: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfSource(pub ::windows::runtime::IUnknown);
impl ITfSource {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn AdviseSink<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punk: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnadviseSink(&self, dwcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfSource {
    type Vtable = ITfSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4ea48a35_60ae_446f_8fd6_e6a8d82459f7);
}
impl ::core::convert::From<ITfSource> for ::windows::runtime::IUnknown {
    fn from(value: ITfSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfSource> for ::windows::runtime::IUnknown {
    fn from(value: &ITfSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr, pdwcookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfSourceSingle(pub ::windows::runtime::IUnknown);
impl ITfSourceSingle {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn AdviseSingleSink<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, tid: u32, riid: *const ::windows::runtime::GUID, punk: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UnadviseSingleSink(&self, tid: u32, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), ::core::mem::transmute(riid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfSourceSingle {
    type Vtable = ITfSourceSingle_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x73131f9c_56a9_49dd_b0ee_d046633f7528);
}
impl ::core::convert::From<ITfSourceSingle> for ::windows::runtime::IUnknown {
    fn from(value: ITfSourceSingle) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfSourceSingle> for ::windows::runtime::IUnknown {
    fn from(value: &ITfSourceSingle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfSourceSingle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfSourceSingle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSourceSingle_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tid: u32, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tid: u32, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfSpeechUIServer(pub ::windows::runtime::IUnknown);
impl ITfSpeechUIServer {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Initialize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn ShowUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn UpdateBalloon<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, style: TfLBBalloonStyle, pch: Param1, cch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(style), pch.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfSpeechUIServer {
    type Vtable = ITfSpeechUIServer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x90e9a944_9244_489f_a78f_de67afc013a7);
}
impl ::core::convert::From<ITfSpeechUIServer> for ::windows::runtime::IUnknown {
    fn from(value: ITfSpeechUIServer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfSpeechUIServer> for ::windows::runtime::IUnknown {
    fn from(value: &ITfSpeechUIServer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfSpeechUIServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfSpeechUIServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSpeechUIServer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfStatusSink(pub ::windows::runtime::IUnknown);
impl ITfStatusSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnStatusChange<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>>(&self, pic: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfStatusSink {
    type Vtable = ITfStatusSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6b7d8d73_b267_4f69_b32e_1ca321ce4f45);
}
impl ::core::convert::From<ITfStatusSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfStatusSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfStatusSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfStatusSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfStatusSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfStatusSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfStatusSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfSystemDeviceTypeLangBarItem(pub ::windows::runtime::IUnknown);
impl ITfSystemDeviceTypeLangBarItem {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetIconMode(&self, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetIconMode(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfSystemDeviceTypeLangBarItem {
    type Vtable = ITfSystemDeviceTypeLangBarItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x45672eb9_9059_46a2_838d_4530355f6a77);
}
impl ::core::convert::From<ITfSystemDeviceTypeLangBarItem> for ::windows::runtime::IUnknown {
    fn from(value: ITfSystemDeviceTypeLangBarItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfSystemDeviceTypeLangBarItem> for ::windows::runtime::IUnknown {
    fn from(value: &ITfSystemDeviceTypeLangBarItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfSystemDeviceTypeLangBarItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfSystemDeviceTypeLangBarItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemDeviceTypeLangBarItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfSystemLangBarItem(pub ::windows::runtime::IUnknown);
impl ITfSystemLangBarItem {
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn SetIcon<'a, Param0: ::windows::runtime::IntoParam<'a, super::WindowsAndMessaging::HICON>>(&self, hicon: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), hicon.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetTooltipString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pchtooltip: Param0, cch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pchtooltip.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfSystemLangBarItem {
    type Vtable = ITfSystemLangBarItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1e13e9ec_6b33_4d4a_b5eb_8a92f029f356);
}
impl ::core::convert::From<ITfSystemLangBarItem> for ::windows::runtime::IUnknown {
    fn from(value: ITfSystemLangBarItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfSystemLangBarItem> for ::windows::runtime::IUnknown {
    fn from(value: &ITfSystemLangBarItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfSystemLangBarItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfSystemLangBarItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemLangBarItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hicon: super::WindowsAndMessaging::HICON) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchtooltip: super::super::Foundation::PWSTR, cch: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfSystemLangBarItemSink(pub ::windows::runtime::IUnknown);
impl ITfSystemLangBarItemSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn InitMenu<'a, Param0: ::windows::runtime::IntoParam<'a, ITfMenu>>(&self, pmenu: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pmenu.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnMenuSelect(&self, wid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfSystemLangBarItemSink {
    type Vtable = ITfSystemLangBarItemSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1449d9ab_13cf_4687_aa3e_8d8b18574396);
}
impl ::core::convert::From<ITfSystemLangBarItemSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfSystemLangBarItemSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfSystemLangBarItemSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfSystemLangBarItemSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfSystemLangBarItemSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfSystemLangBarItemSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemLangBarItemSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmenu: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wid: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfSystemLangBarItemText(pub ::windows::runtime::IUnknown);
impl ITfSystemLangBarItemText {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn SetItemText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pch: Param0, cch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pch.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetItemText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfSystemLangBarItemText {
    type Vtable = ITfSystemLangBarItemText_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5c4ce0e5_ba49_4b52_ac6b_3b397b4f701f);
}
impl ::core::convert::From<ITfSystemLangBarItemText> for ::windows::runtime::IUnknown {
    fn from(value: ITfSystemLangBarItemText) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfSystemLangBarItemText> for ::windows::runtime::IUnknown {
    fn from(value: &ITfSystemLangBarItemText) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfSystemLangBarItemText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfSystemLangBarItemText {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemLangBarItemText_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfTextEditSink(pub ::windows::runtime::IUnknown);
impl ITfTextEditSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnEndEdit<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>, Param2: ::windows::runtime::IntoParam<'a, ITfEditRecord>>(&self, pic: Param0, ecreadonly: u32, peditrecord: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(ecreadonly), peditrecord.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfTextEditSink {
    type Vtable = ITfTextEditSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8127d409_ccd3_4683_967a_b43d5b482bf7);
}
impl ::core::convert::From<ITfTextEditSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfTextEditSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfTextEditSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfTextEditSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfTextEditSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfTextEditSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextEditSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, ecreadonly: u32, peditrecord: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfTextInputProcessor(pub ::windows::runtime::IUnknown);
impl ITfTextInputProcessor {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, ITfThreadMgr>>(&self, ptim: Param0, tid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptim.into_param().abi(), ::core::mem::transmute(tid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Deactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfTextInputProcessor {
    type Vtable = ITfTextInputProcessor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e7f7_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<ITfTextInputProcessor> for ::windows::runtime::IUnknown {
    fn from(value: ITfTextInputProcessor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfTextInputProcessor> for ::windows::runtime::IUnknown {
    fn from(value: &ITfTextInputProcessor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfTextInputProcessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfTextInputProcessor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextInputProcessor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptim: ::windows::runtime::RawPtr, tid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfTextInputProcessorEx(pub ::windows::runtime::IUnknown);
impl ITfTextInputProcessorEx {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, ITfThreadMgr>>(&self, ptim: Param0, tid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptim.into_param().abi(), ::core::mem::transmute(tid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Deactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ActivateEx<'a, Param0: ::windows::runtime::IntoParam<'a, ITfThreadMgr>>(&self, ptim: Param0, tid: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ptim.into_param().abi(), ::core::mem::transmute(tid), ::core::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfTextInputProcessorEx {
    type Vtable = ITfTextInputProcessorEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6e4e2102_f9cd_433d_b496_303ce03a6507);
}
impl ::core::convert::From<ITfTextInputProcessorEx> for ::windows::runtime::IUnknown {
    fn from(value: ITfTextInputProcessorEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfTextInputProcessorEx> for ::windows::runtime::IUnknown {
    fn from(value: &ITfTextInputProcessorEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfTextInputProcessorEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfTextInputProcessorEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfTextInputProcessorEx> for ITfTextInputProcessor {
    fn from(value: ITfTextInputProcessorEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfTextInputProcessorEx> for ITfTextInputProcessor {
    fn from(value: &ITfTextInputProcessorEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfTextInputProcessor> for ITfTextInputProcessorEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfTextInputProcessor> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfTextInputProcessor> for &ITfTextInputProcessorEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfTextInputProcessor> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextInputProcessorEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptim: ::windows::runtime::RawPtr, tid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptim: ::windows::runtime::RawPtr, tid: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfTextLayoutSink(pub ::windows::runtime::IUnknown);
impl ITfTextLayoutSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnLayoutChange<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>, Param2: ::windows::runtime::IntoParam<'a, ITfContextView>>(&self, pic: Param0, lcode: TfLayoutCode, pview: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(lcode), pview.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfTextLayoutSink {
    type Vtable = ITfTextLayoutSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2af2d06a_dd5b_4927_a0b4_54f19c91fade);
}
impl ::core::convert::From<ITfTextLayoutSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfTextLayoutSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfTextLayoutSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfTextLayoutSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfTextLayoutSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfTextLayoutSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextLayoutSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, lcode: TfLayoutCode, pview: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfThreadFocusSink(pub ::windows::runtime::IUnknown);
impl ITfThreadFocusSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnSetThreadFocus(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnKillThreadFocus(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfThreadFocusSink {
    type Vtable = ITfThreadFocusSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc0f1db0c_3a20_405c_a303_96b6010a885f);
}
impl ::core::convert::From<ITfThreadFocusSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfThreadFocusSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfThreadFocusSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfThreadFocusSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfThreadFocusSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfThreadFocusSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadFocusSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfThreadMgr(pub ::windows::runtime::IUnknown);
impl ITfThreadMgr {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Activate(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Deactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn CreateDocumentMgr(&self) -> ::windows::runtime::Result<ITfDocumentMgr> {
        let mut result__: <ITfDocumentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfDocumentMgr>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumDocumentMgrs(&self) -> ::windows::runtime::Result<IEnumTfDocumentMgrs> {
        let mut result__: <IEnumTfDocumentMgrs as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfDocumentMgrs>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetFocus(&self) -> ::windows::runtime::Result<ITfDocumentMgr> {
        let mut result__: <ITfDocumentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfDocumentMgr>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetFocus<'a, Param0: ::windows::runtime::IntoParam<'a, ITfDocumentMgr>>(&self, pdimfocus: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pdimfocus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn AssociateFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, ITfDocumentMgr>>(&self, hwnd: Param0, pdimnew: Param1) -> ::windows::runtime::Result<ITfDocumentMgr> {
        let mut result__: <ITfDocumentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), pdimnew.into_param().abi(), &mut result__).from_abi::<ITfDocumentMgr>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsThreadFocus(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetFunctionProvider(&self, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ITfFunctionProvider> {
        let mut result__: <ITfFunctionProvider as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), &mut result__).from_abi::<ITfFunctionProvider>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumFunctionProviders(&self) -> ::windows::runtime::Result<IEnumTfFunctionProviders> {
        let mut result__: <IEnumTfFunctionProviders as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfFunctionProviders>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGlobalCompartment(&self) -> ::windows::runtime::Result<ITfCompartmentMgr> {
        let mut result__: <ITfCompartmentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfCompartmentMgr>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfThreadMgr {
    type Vtable = ITfThreadMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e801_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<ITfThreadMgr> for ::windows::runtime::IUnknown {
    fn from(value: ITfThreadMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfThreadMgr> for ::windows::runtime::IUnknown {
    fn from(value: &ITfThreadMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfThreadMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfThreadMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdim: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdimfocus: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdimfocus: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, pdimnew: ::windows::runtime::RawPtr, ppdimprev: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, ppfuncprov: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcompmgr: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfThreadMgr2(pub ::windows::runtime::IUnknown);
impl ITfThreadMgr2 {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Activate(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Deactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn CreateDocumentMgr(&self) -> ::windows::runtime::Result<ITfDocumentMgr> {
        let mut result__: <ITfDocumentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfDocumentMgr>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumDocumentMgrs(&self) -> ::windows::runtime::Result<IEnumTfDocumentMgrs> {
        let mut result__: <IEnumTfDocumentMgrs as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfDocumentMgrs>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetFocus(&self) -> ::windows::runtime::Result<ITfDocumentMgr> {
        let mut result__: <ITfDocumentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfDocumentMgr>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetFocus<'a, Param0: ::windows::runtime::IntoParam<'a, ITfDocumentMgr>>(&self, pdimfocus: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pdimfocus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsThreadFocus(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetFunctionProvider(&self, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ITfFunctionProvider> {
        let mut result__: <ITfFunctionProvider as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), &mut result__).from_abi::<ITfFunctionProvider>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumFunctionProviders(&self) -> ::windows::runtime::Result<IEnumTfFunctionProviders> {
        let mut result__: <IEnumTfFunctionProviders as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfFunctionProviders>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGlobalCompartment(&self) -> ::windows::runtime::Result<ITfCompartmentMgr> {
        let mut result__: <ITfCompartmentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfCompartmentMgr>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptid), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetActiveFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SuspendKeystrokeHandling(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ResumeKeystrokeHandling(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfThreadMgr2 {
    type Vtable = ITfThreadMgr2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0ab198ef_6477_4ee8_8812_6780edb82d5e);
}
impl ::core::convert::From<ITfThreadMgr2> for ::windows::runtime::IUnknown {
    fn from(value: ITfThreadMgr2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfThreadMgr2> for ::windows::runtime::IUnknown {
    fn from(value: &ITfThreadMgr2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfThreadMgr2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfThreadMgr2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgr2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdim: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdimfocus: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdimfocus: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, ppfuncprov: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcompmgr: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptid: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfThreadMgrEventSink(pub ::windows::runtime::IUnknown);
impl ITfThreadMgrEventSink {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnInitDocumentMgr<'a, Param0: ::windows::runtime::IntoParam<'a, ITfDocumentMgr>>(&self, pdim: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pdim.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnUninitDocumentMgr<'a, Param0: ::windows::runtime::IntoParam<'a, ITfDocumentMgr>>(&self, pdim: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pdim.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnSetFocus<'a, Param0: ::windows::runtime::IntoParam<'a, ITfDocumentMgr>, Param1: ::windows::runtime::IntoParam<'a, ITfDocumentMgr>>(&self, pdimfocus: Param0, pdimprevfocus: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pdimfocus.into_param().abi(), pdimprevfocus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnPushContext<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>>(&self, pic: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pic.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnPopContext<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>>(&self, pic: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pic.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfThreadMgrEventSink {
    type Vtable = ITfThreadMgrEventSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa80e80e_2021_11d2_93e0_0060b067b86e);
}
impl ::core::convert::From<ITfThreadMgrEventSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfThreadMgrEventSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfThreadMgrEventSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfThreadMgrEventSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfThreadMgrEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfThreadMgrEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgrEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdim: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdim: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdimfocus: ::windows::runtime::RawPtr, pdimprevfocus: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfThreadMgrEx(pub ::windows::runtime::IUnknown);
impl ITfThreadMgrEx {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Activate(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn Deactivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn CreateDocumentMgr(&self) -> ::windows::runtime::Result<ITfDocumentMgr> {
        let mut result__: <ITfDocumentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfDocumentMgr>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumDocumentMgrs(&self) -> ::windows::runtime::Result<IEnumTfDocumentMgrs> {
        let mut result__: <IEnumTfDocumentMgrs as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfDocumentMgrs>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetFocus(&self) -> ::windows::runtime::Result<ITfDocumentMgr> {
        let mut result__: <ITfDocumentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfDocumentMgr>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn SetFocus<'a, Param0: ::windows::runtime::IntoParam<'a, ITfDocumentMgr>>(&self, pdimfocus: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pdimfocus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn AssociateFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, ITfDocumentMgr>>(&self, hwnd: Param0, pdimnew: Param1) -> ::windows::runtime::Result<ITfDocumentMgr> {
        let mut result__: <ITfDocumentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), pdimnew.into_param().abi(), &mut result__).from_abi::<ITfDocumentMgr>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsThreadFocus(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetFunctionProvider(&self, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ITfFunctionProvider> {
        let mut result__: <ITfFunctionProvider as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), &mut result__).from_abi::<ITfFunctionProvider>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumFunctionProviders(&self) -> ::windows::runtime::Result<IEnumTfFunctionProviders> {
        let mut result__: <IEnumTfFunctionProviders as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfFunctionProviders>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGlobalCompartment(&self) -> ::windows::runtime::Result<ITfCompartmentMgr> {
        let mut result__: <ITfCompartmentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfCompartmentMgr>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptid), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetActiveFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfThreadMgrEx {
    type Vtable = ITfThreadMgrEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3e90ade3_7594_4cb0_bb58_69628f5f458c);
}
impl ::core::convert::From<ITfThreadMgrEx> for ::windows::runtime::IUnknown {
    fn from(value: ITfThreadMgrEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfThreadMgrEx> for ::windows::runtime::IUnknown {
    fn from(value: &ITfThreadMgrEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfThreadMgrEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfThreadMgrEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfThreadMgrEx> for ITfThreadMgr {
    fn from(value: ITfThreadMgrEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfThreadMgrEx> for ITfThreadMgr {
    fn from(value: &ITfThreadMgrEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfThreadMgr> for ITfThreadMgrEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfThreadMgr> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfThreadMgr> for &ITfThreadMgrEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfThreadMgr> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgrEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdim: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdimfocus: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdimfocus: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, pdimnew: ::windows::runtime::RawPtr, ppdimprev: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, ppfuncprov: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcompmgr: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptid: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpdwflags: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfToolTipUIElement(pub ::windows::runtime::IUnknown);
impl ITfToolTipUIElement {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGUID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsShown(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetString(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfToolTipUIElement {
    type Vtable = ITfToolTipUIElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x52b18b5c_555d_46b2_b00a_fa680144fbdb);
}
impl ::core::convert::From<ITfToolTipUIElement> for ::windows::runtime::IUnknown {
    fn from(value: ITfToolTipUIElement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfToolTipUIElement> for ::windows::runtime::IUnknown {
    fn from(value: &ITfToolTipUIElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfToolTipUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfToolTipUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfToolTipUIElement> for ITfUIElement {
    fn from(value: ITfToolTipUIElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfToolTipUIElement> for ITfUIElement {
    fn from(value: &ITfToolTipUIElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfUIElement> for ITfToolTipUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfUIElement> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfUIElement> for &ITfToolTipUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfUIElement> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfToolTipUIElement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstr: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfTransitoryExtensionSink(pub ::windows::runtime::IUnknown);
impl ITfTransitoryExtensionSink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnTransitoryExtensionUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, ITfContext>, Param2: ::windows::runtime::IntoParam<'a, ITfRange>, Param3: ::windows::runtime::IntoParam<'a, ITfRange>>(&self, pic: Param0, ecreadonly: u32, presultrange: Param2, pcompositionrange: Param3) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(ecreadonly), presultrange.into_param().abi(), pcompositionrange.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfTransitoryExtensionSink {
    type Vtable = ITfTransitoryExtensionSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa615096f_1c57_4813_8a15_55ee6e5a839c);
}
impl ::core::convert::From<ITfTransitoryExtensionSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfTransitoryExtensionSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfTransitoryExtensionSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfTransitoryExtensionSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfTransitoryExtensionSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfTransitoryExtensionSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTransitoryExtensionSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pic: ::windows::runtime::RawPtr, ecreadonly: u32, presultrange: ::windows::runtime::RawPtr, pcompositionrange: ::windows::runtime::RawPtr, pfdeleteresultrange: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfTransitoryExtensionUIElement(pub ::windows::runtime::IUnknown);
impl ITfTransitoryExtensionUIElement {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGUID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsShown(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::runtime::Result<ITfDocumentMgr> {
        let mut result__: <ITfDocumentMgr as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITfDocumentMgr>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfTransitoryExtensionUIElement {
    type Vtable = ITfTransitoryExtensionUIElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x858f956a_972f_42a2_a2f2_0321e1abe209);
}
impl ::core::convert::From<ITfTransitoryExtensionUIElement> for ::windows::runtime::IUnknown {
    fn from(value: ITfTransitoryExtensionUIElement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfTransitoryExtensionUIElement> for ::windows::runtime::IUnknown {
    fn from(value: &ITfTransitoryExtensionUIElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfTransitoryExtensionUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfTransitoryExtensionUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITfTransitoryExtensionUIElement> for ITfUIElement {
    fn from(value: ITfTransitoryExtensionUIElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfTransitoryExtensionUIElement> for ITfUIElement {
    fn from(value: &ITfTransitoryExtensionUIElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfUIElement> for ITfTransitoryExtensionUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfUIElement> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITfUIElement> for &ITfTransitoryExtensionUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITfUIElement> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTransitoryExtensionUIElement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdim: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfUIElement(pub ::windows::runtime::IUnknown);
impl ITfUIElement {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetGUID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn IsShown(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfUIElement {
    type Vtable = ITfUIElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea1ea137_19df_11d7_a6d2_00065b84435c);
}
impl ::core::convert::From<ITfUIElement> for ::windows::runtime::IUnknown {
    fn from(value: ITfUIElement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfUIElement> for ::windows::runtime::IUnknown {
    fn from(value: &ITfUIElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfUIElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfUIElement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfUIElementMgr(pub ::windows::runtime::IUnknown);
impl ITfUIElementMgr {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn BeginUIElement<'a, Param0: ::windows::runtime::IntoParam<'a, ITfUIElement>>(&self, pelement: Param0, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pelement.into_param().abi(), ::core::mem::transmute(pbshow), ::core::mem::transmute(pdwuielementid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UpdateUIElement(&self, dwuielementid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwuielementid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EndUIElement(&self, dwuielementid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwuielementid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetUIElement(&self, dwuielementid: u32) -> ::windows::runtime::Result<ITfUIElement> {
        let mut result__: <ITfUIElement as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwuielementid), &mut result__).from_abi::<ITfUIElement>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EnumUIElements(&self) -> ::windows::runtime::Result<IEnumTfUIElements> {
        let mut result__: <IEnumTfUIElements as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumTfUIElements>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfUIElementMgr {
    type Vtable = ITfUIElementMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea1ea135_19df_11d7_a6d2_00065b84435c);
}
impl ::core::convert::From<ITfUIElementMgr> for ::windows::runtime::IUnknown {
    fn from(value: ITfUIElementMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfUIElementMgr> for ::windows::runtime::IUnknown {
    fn from(value: &ITfUIElementMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfUIElementMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfUIElementMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfUIElementMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pelement: ::windows::runtime::RawPtr, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwuielementid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwuielementid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwuielementid: u32, ppelement: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITfUIElementSink(pub ::windows::runtime::IUnknown);
impl ITfUIElementSink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn BeginUIElement(&self, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwuielementid), ::core::mem::transmute(pbshow)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn UpdateUIElement(&self, dwuielementid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwuielementid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn EndUIElement(&self, dwuielementid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwuielementid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITfUIElementSink {
    type Vtable = ITfUIElementSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xea1ea136_19df_11d7_a6d2_00065b84435c);
}
impl ::core::convert::From<ITfUIElementSink> for ::windows::runtime::IUnknown {
    fn from(value: ITfUIElementSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITfUIElementSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITfUIElementSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITfUIElementSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITfUIElementSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfUIElementSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwuielementid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwuielementid: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUIManagerEventSink(pub ::windows::runtime::IUnknown);
impl IUIManagerEventSink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnWindowOpening(&self, prcbounds: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcbounds)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnWindowOpened(&self, prcbounds: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcbounds)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnWindowUpdating(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcupdatedbounds)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn OnWindowUpdated(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcupdatedbounds)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnWindowClosing(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn OnWindowClosed(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUIManagerEventSink {
    type Vtable = IUIManagerEventSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcd91d690_a7e8_4265_9b38_8bb3bbaba7de);
}
impl ::core::convert::From<IUIManagerEventSink> for ::windows::runtime::IUnknown {
    fn from(value: IUIManagerEventSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUIManagerEventSink> for ::windows::runtime::IUnknown {
    fn from(value: &IUIManagerEventSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUIManagerEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUIManagerEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIManagerEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcbounds: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcbounds: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVersionInfo(pub ::windows::runtime::IUnknown);
impl IVersionInfo {
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetSubcomponentCount(&self, ulsub: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsub), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetImplementationID(&self, ulsub: u32) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsub), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub unsafe fn GetBuildVersion(&self, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsub), ::core::mem::transmute(pdwmajor), ::core::mem::transmute(pdwminor)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetComponentDescription(&self, ulsub: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsub), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    pub unsafe fn GetInstanceDescription(&self, ulsub: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsub), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVersionInfo {
    type Vtable = IVersionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x401518ec_db00_4611_9b29_2a0e4b9afa85);
}
impl ::core::convert::From<IVersionInfo> for ::windows::runtime::IUnknown {
    fn from(value: IVersionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVersionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IVersionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVersionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVersionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVersionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulsub: u32, ulcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulsub: u32, implid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulsub: u32, pimplstr: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulsub: u32, pimplstr: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[inline]
pub unsafe fn InitLocalMsCtfMonitor(dwflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitLocalMsCtfMonitor(dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        InitLocalMsCtfMonitor(::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InputScope(pub i32);
pub const IS_DEFAULT: InputScope = InputScope(0i32);
pub const IS_URL: InputScope = InputScope(1i32);
pub const IS_FILE_FULLFILEPATH: InputScope = InputScope(2i32);
pub const IS_FILE_FILENAME: InputScope = InputScope(3i32);
pub const IS_EMAIL_USERNAME: InputScope = InputScope(4i32);
pub const IS_EMAIL_SMTPEMAILADDRESS: InputScope = InputScope(5i32);
pub const IS_LOGINNAME: InputScope = InputScope(6i32);
pub const IS_PERSONALNAME_FULLNAME: InputScope = InputScope(7i32);
pub const IS_PERSONALNAME_PREFIX: InputScope = InputScope(8i32);
pub const IS_PERSONALNAME_GIVENNAME: InputScope = InputScope(9i32);
pub const IS_PERSONALNAME_MIDDLENAME: InputScope = InputScope(10i32);
pub const IS_PERSONALNAME_SURNAME: InputScope = InputScope(11i32);
pub const IS_PERSONALNAME_SUFFIX: InputScope = InputScope(12i32);
pub const IS_ADDRESS_FULLPOSTALADDRESS: InputScope = InputScope(13i32);
pub const IS_ADDRESS_POSTALCODE: InputScope = InputScope(14i32);
pub const IS_ADDRESS_STREET: InputScope = InputScope(15i32);
pub const IS_ADDRESS_STATEORPROVINCE: InputScope = InputScope(16i32);
pub const IS_ADDRESS_CITY: InputScope = InputScope(17i32);
pub const IS_ADDRESS_COUNTRYNAME: InputScope = InputScope(18i32);
pub const IS_ADDRESS_COUNTRYSHORTNAME: InputScope = InputScope(19i32);
pub const IS_CURRENCY_AMOUNTANDSYMBOL: InputScope = InputScope(20i32);
pub const IS_CURRENCY_AMOUNT: InputScope = InputScope(21i32);
pub const IS_DATE_FULLDATE: InputScope = InputScope(22i32);
pub const IS_DATE_MONTH: InputScope = InputScope(23i32);
pub const IS_DATE_DAY: InputScope = InputScope(24i32);
pub const IS_DATE_YEAR: InputScope = InputScope(25i32);
pub const IS_DATE_MONTHNAME: InputScope = InputScope(26i32);
pub const IS_DATE_DAYNAME: InputScope = InputScope(27i32);
pub const IS_DIGITS: InputScope = InputScope(28i32);
pub const IS_NUMBER: InputScope = InputScope(29i32);
pub const IS_ONECHAR: InputScope = InputScope(30i32);
pub const IS_PASSWORD: InputScope = InputScope(31i32);
pub const IS_TELEPHONE_FULLTELEPHONENUMBER: InputScope = InputScope(32i32);
pub const IS_TELEPHONE_COUNTRYCODE: InputScope = InputScope(33i32);
pub const IS_TELEPHONE_AREACODE: InputScope = InputScope(34i32);
pub const IS_TELEPHONE_LOCALNUMBER: InputScope = InputScope(35i32);
pub const IS_TIME_FULLTIME: InputScope = InputScope(36i32);
pub const IS_TIME_HOUR: InputScope = InputScope(37i32);
pub const IS_TIME_MINORSEC: InputScope = InputScope(38i32);
pub const IS_NUMBER_FULLWIDTH: InputScope = InputScope(39i32);
pub const IS_ALPHANUMERIC_HALFWIDTH: InputScope = InputScope(40i32);
pub const IS_ALPHANUMERIC_FULLWIDTH: InputScope = InputScope(41i32);
pub const IS_CURRENCY_CHINESE: InputScope = InputScope(42i32);
pub const IS_BOPOMOFO: InputScope = InputScope(43i32);
pub const IS_HIRAGANA: InputScope = InputScope(44i32);
pub const IS_KATAKANA_HALFWIDTH: InputScope = InputScope(45i32);
pub const IS_KATAKANA_FULLWIDTH: InputScope = InputScope(46i32);
pub const IS_HANJA: InputScope = InputScope(47i32);
pub const IS_HANGUL_HALFWIDTH: InputScope = InputScope(48i32);
pub const IS_HANGUL_FULLWIDTH: InputScope = InputScope(49i32);
pub const IS_SEARCH: InputScope = InputScope(50i32);
pub const IS_FORMULA: InputScope = InputScope(51i32);
pub const IS_SEARCH_INCREMENTAL: InputScope = InputScope(52i32);
pub const IS_CHINESE_HALFWIDTH: InputScope = InputScope(53i32);
pub const IS_CHINESE_FULLWIDTH: InputScope = InputScope(54i32);
pub const IS_NATIVE_SCRIPT: InputScope = InputScope(55i32);
pub const IS_YOMI: InputScope = InputScope(56i32);
pub const IS_TEXT: InputScope = InputScope(57i32);
pub const IS_CHAT: InputScope = InputScope(58i32);
pub const IS_NAME_OR_PHONENUMBER: InputScope = InputScope(59i32);
pub const IS_EMAILNAME_OR_ADDRESS: InputScope = InputScope(60i32);
pub const IS_PRIVATE: InputScope = InputScope(61i32);
pub const IS_MAPS: InputScope = InputScope(62i32);
pub const IS_NUMERIC_PASSWORD: InputScope = InputScope(63i32);
pub const IS_NUMERIC_PIN: InputScope = InputScope(64i32);
pub const IS_ALPHANUMERIC_PIN: InputScope = InputScope(65i32);
pub const IS_ALPHANUMERIC_PIN_SET: InputScope = InputScope(66i32);
pub const IS_FORMULA_NUMBER: InputScope = InputScope(67i32);
pub const IS_CHAT_WITHOUT_EMOJI: InputScope = InputScope(68i32);
pub const IS_PHRASELIST: InputScope = InputScope(-1i32);
pub const IS_REGULAREXPRESSION: InputScope = InputScope(-2i32);
pub const IS_SRGS: InputScope = InputScope(-3i32);
pub const IS_XML: InputScope = InputScope(-4i32);
pub const IS_ENUMSTRING: InputScope = InputScope(-5i32);
impl ::core::convert::From<i32> for InputScope {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InputScope {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LANG_BAR_ITEM_ICON_MODE_FLAGS(pub u32);
pub const TF_DTLBI_NONE: LANG_BAR_ITEM_ICON_MODE_FLAGS = LANG_BAR_ITEM_ICON_MODE_FLAGS(0u32);
pub const TF_DTLBI_USEPROFILEICON: LANG_BAR_ITEM_ICON_MODE_FLAGS = LANG_BAR_ITEM_ICON_MODE_FLAGS(1u32);
impl ::core::convert::From<u32> for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LIBID_MSAATEXTLib: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x150e2d7a_dac1_4582_947d_2a8fd78b82cd);
pub const MSAAControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x08cd963f_7a3e_4f5c_9bd8_d692bb043c5b);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TEXT_STORE_CHANGE_FLAGS(pub u32);
pub const TS_TC_NONE: TEXT_STORE_CHANGE_FLAGS = TEXT_STORE_CHANGE_FLAGS(0u32);
pub const TS_TC_CORRECTION: TEXT_STORE_CHANGE_FLAGS = TEXT_STORE_CHANGE_FLAGS(1u32);
impl ::core::convert::From<u32> for TEXT_STORE_CHANGE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TEXT_STORE_CHANGE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for TEXT_STORE_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for TEXT_STORE_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for TEXT_STORE_CHANGE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for TEXT_STORE_CHANGE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for TEXT_STORE_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TEXT_STORE_LOCK_FLAGS(pub u32);
pub const TS_LF_READ: TEXT_STORE_LOCK_FLAGS = TEXT_STORE_LOCK_FLAGS(2u32);
pub const TS_LF_READWRITE: TEXT_STORE_LOCK_FLAGS = TEXT_STORE_LOCK_FLAGS(6u32);
impl ::core::convert::From<u32> for TEXT_STORE_LOCK_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TEXT_STORE_LOCK_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for TEXT_STORE_LOCK_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for TEXT_STORE_LOCK_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for TEXT_STORE_LOCK_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for TEXT_STORE_LOCK_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for TEXT_STORE_LOCK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TEXT_STORE_TEXT_CHANGE_FLAGS(pub u32);
pub const TS_ST_NONE: TEXT_STORE_TEXT_CHANGE_FLAGS = TEXT_STORE_TEXT_CHANGE_FLAGS(0u32);
pub const TS_ST_CORRECTION: TEXT_STORE_TEXT_CHANGE_FLAGS = TEXT_STORE_TEXT_CHANGE_FLAGS(1u32);
impl ::core::convert::From<u32> for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CHAR_EMBEDDED: u32 = 65532u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CLUIE_COUNT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CLUIE_CURRENTPAGE: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CLUIE_DOCUMENTMGR: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CLUIE_PAGEINDEX: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CLUIE_SELECTION: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CLUIE_STRING: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_COMMANDING_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_COMMANDING_ON: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TF_CONTEXT_EDIT_CONTEXT_FLAGS(pub u32);
pub const TF_ES_ASYNCDONTCARE: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(0u32);
pub const TF_ES_SYNC: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(1u32);
pub const TF_ES_READ: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(2u32);
pub const TF_ES_READWRITE: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(6u32);
pub const TF_ES_ASYNC: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(8u32);
impl ::core::convert::From<u32> for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CONVERSIONMODE_ALPHANUMERIC: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CONVERSIONMODE_CHARCODE: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CONVERSIONMODE_EUDC: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CONVERSIONMODE_FIXED: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CONVERSIONMODE_FULLSHAPE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CONVERSIONMODE_KATAKANA: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CONVERSIONMODE_NATIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CONVERSIONMODE_NOCONVERSION: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CONVERSIONMODE_ROMAN: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CONVERSIONMODE_SOFTKEYBOARD: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_CONVERSIONMODE_SYMBOL: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TF_DA_ATTR_INFO(pub i32);
pub const TF_ATTR_INPUT: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(0i32);
pub const TF_ATTR_TARGET_CONVERTED: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(1i32);
pub const TF_ATTR_CONVERTED: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(2i32);
pub const TF_ATTR_TARGET_NOTCONVERTED: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(3i32);
pub const TF_ATTR_INPUT_ERROR: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(4i32);
pub const TF_ATTR_FIXEDCONVERTED: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(5i32);
pub const TF_ATTR_OTHER: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(-1i32);
impl ::core::convert::From<i32> for TF_DA_ATTR_INFO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TF_DA_ATTR_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub struct TF_DA_COLOR {
    pub r#type: TF_DA_COLORTYPE,
    pub Anonymous: TF_DA_COLOR_0,
}
impl TF_DA_COLOR {}
impl ::core::default::Default for TF_DA_COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TF_DA_COLOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for TF_DA_COLOR {}
unsafe impl ::windows::runtime::Abi for TF_DA_COLOR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub union TF_DA_COLOR_0 {
    pub nIndex: i32,
    pub cr: u32,
}
impl TF_DA_COLOR_0 {}
impl ::core::default::Default for TF_DA_COLOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TF_DA_COLOR_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for TF_DA_COLOR_0 {}
unsafe impl ::windows::runtime::Abi for TF_DA_COLOR_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TF_DA_COLORTYPE(pub i32);
pub const TF_CT_NONE: TF_DA_COLORTYPE = TF_DA_COLORTYPE(0i32);
pub const TF_CT_SYSCOLOR: TF_DA_COLORTYPE = TF_DA_COLORTYPE(1i32);
pub const TF_CT_COLORREF: TF_DA_COLORTYPE = TF_DA_COLORTYPE(2i32);
impl ::core::convert::From<i32> for TF_DA_COLORTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TF_DA_COLORTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TF_DA_LINESTYLE(pub i32);
pub const TF_LS_NONE: TF_DA_LINESTYLE = TF_DA_LINESTYLE(0i32);
pub const TF_LS_SOLID: TF_DA_LINESTYLE = TF_DA_LINESTYLE(1i32);
pub const TF_LS_DOT: TF_DA_LINESTYLE = TF_DA_LINESTYLE(2i32);
pub const TF_LS_DASH: TF_DA_LINESTYLE = TF_DA_LINESTYLE(3i32);
pub const TF_LS_SQUIGGLE: TF_DA_LINESTYLE = TF_DA_LINESTYLE(4i32);
impl ::core::convert::From<i32> for TF_DA_LINESTYLE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TF_DA_LINESTYLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_DICTATION_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_DICTATION_ON: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_DISABLE_BALLOON: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_DISABLE_COMMANDING: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_DISABLE_DICTATION: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_DISABLE_SPEECH: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
pub struct TF_DISPLAYATTRIBUTE {
    pub crText: TF_DA_COLOR,
    pub crBk: TF_DA_COLOR,
    pub lsStyle: TF_DA_LINESTYLE,
    pub fBoldLine: super::super::Foundation::BOOL,
    pub crLine: TF_DA_COLOR,
    pub bAttr: TF_DA_ATTR_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl TF_DISPLAYATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_DISPLAYATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_DISPLAYATTRIBUTE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_DISPLAYATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TF_DISPLAYATTRIBUTE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_ALREADY_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220218i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_COMPOSITION_REJECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220216i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_DISCONNECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220220i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_EMPTYCONTEXT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220215i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220982i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_INVALIDPOINT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220985i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_INVALIDPOS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220992i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_INVALIDVIEW: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220219i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_LOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220224i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_NOCONVERSION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147219968i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_NOINTERFACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220988i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_NOLAYOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220986i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_NOLOCK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220991i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_NOOBJECT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220990i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_NOPROVIDER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220221i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_NOSELECTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220987i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_NOSERVICE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220989i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_NOTOWNEDRANGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220222i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_RANGE_NOT_COVERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220217i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_READONLY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220983i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_STACKFULL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220223i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_E_SYNCHRONOUS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220984i32 as _);
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub struct TF_HALTCOND {
    pub pHaltRange: ::core::option::Option<ITfRange>,
    pub aHaltPos: TfAnchor,
    pub dwFlags: u32,
}
impl TF_HALTCOND {}
impl ::core::default::Default for TF_HALTCOND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TF_HALTCOND {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TF_HALTCOND").field("pHaltRange", &self.pHaltRange).field("aHaltPos", &self.aHaltPos).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::cmp::PartialEq for TF_HALTCOND {
    fn eq(&self, other: &Self) -> bool {
        self.pHaltRange == other.pHaltRange && self.aHaltPos == other.aHaltPos && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for TF_HALTCOND {}
unsafe impl ::windows::runtime::Abi for TF_HALTCOND {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_HF_OBJECT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IE_CORRECTION: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub struct TF_INPUTPROCESSORPROFILE {
    pub dwProfileType: u32,
    pub langid: u16,
    pub clsid: ::windows::runtime::GUID,
    pub guidProfile: ::windows::runtime::GUID,
    pub catid: ::windows::runtime::GUID,
    pub hklSubstitute: HKL,
    pub dwCaps: u32,
    pub hkl: HKL,
    pub dwFlags: u32,
}
impl TF_INPUTPROCESSORPROFILE {}
impl ::core::default::Default for TF_INPUTPROCESSORPROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TF_INPUTPROCESSORPROFILE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TF_INPUTPROCESSORPROFILE")
            .field("dwProfileType", &self.dwProfileType)
            .field("langid", &self.langid)
            .field("clsid", &self.clsid)
            .field("guidProfile", &self.guidProfile)
            .field("catid", &self.catid)
            .field("hklSubstitute", &self.hklSubstitute)
            .field("dwCaps", &self.dwCaps)
            .field("hkl", &self.hkl)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TF_INPUTPROCESSORPROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.dwProfileType == other.dwProfileType && self.langid == other.langid && self.clsid == other.clsid && self.guidProfile == other.guidProfile && self.catid == other.catid && self.hklSubstitute == other.hklSubstitute && self.dwCaps == other.dwCaps && self.hkl == other.hkl && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for TF_INPUTPROCESSORPROFILE {}
unsafe impl ::windows::runtime::Abi for TF_INPUTPROCESSORPROFILE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_INVALID_COOKIE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_INVALID_EDIT_COOKIE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPPMF_DISABLEPROFILE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPPMF_ENABLEPROFILE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPPMF_FORPROCESS: u32 = 268435456u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPPMF_FORSESSION: u32 = 536870912u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPPMF_FORSYSTEMALL: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPP_CAPS_COMLESSSUPPORT: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPP_CAPS_DISABLEONTRANSITORY: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPP_CAPS_IMMERSIVESUPPORT: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPP_CAPS_SECUREMODESUPPORT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPP_CAPS_SYSTRAYSUPPORT: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPP_CAPS_UIELEMENTENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPP_CAPS_WOW16SUPPORT: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPP_FLAG_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPP_FLAG_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPP_FLAG_SUBSTITUTEDBYINPUTPROCESSOR: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_IPSINK_FLAG_ACTIVE: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub struct TF_LANGBARITEMINFO {
    pub clsidService: ::windows::runtime::GUID,
    pub guidItem: ::windows::runtime::GUID,
    pub dwStyle: u32,
    pub ulSort: u32,
    pub szDescription: [u16; 32],
}
impl TF_LANGBARITEMINFO {}
impl ::core::default::Default for TF_LANGBARITEMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TF_LANGBARITEMINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TF_LANGBARITEMINFO").field("clsidService", &self.clsidService).field("guidItem", &self.guidItem).field("dwStyle", &self.dwStyle).field("ulSort", &self.ulSort).field("szDescription", &self.szDescription).finish()
    }
}
impl ::core::cmp::PartialEq for TF_LANGBARITEMINFO {
    fn eq(&self, other: &Self) -> bool {
        self.clsidService == other.clsidService && self.guidItem == other.guidItem && self.dwStyle == other.dwStyle && self.ulSort == other.ulSort && self.szDescription == other.szDescription
    }
}
impl ::core::cmp::Eq for TF_LANGBARITEMINFO {}
unsafe impl ::windows::runtime::Abi for TF_LANGBARITEMINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
pub struct TF_LANGUAGEPROFILE {
    pub clsid: ::windows::runtime::GUID,
    pub langid: u16,
    pub catid: ::windows::runtime::GUID,
    pub fActive: super::super::Foundation::BOOL,
    pub guidProfile: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl TF_LANGUAGEPROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_LANGUAGEPROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TF_LANGUAGEPROFILE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TF_LANGUAGEPROFILE").field("clsid", &self.clsid).field("langid", &self.langid).field("catid", &self.catid).field("fActive", &self.fActive).field("guidProfile", &self.guidProfile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_LANGUAGEPROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.clsid == other.clsid && self.langid == other.langid && self.catid == other.catid && self.fActive == other.fActive && self.guidProfile == other.guidProfile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_LANGUAGEPROFILE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TF_LANGUAGEPROFILE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
pub struct TF_LBBALLOONINFO {
    pub style: TfLBBalloonStyle,
    pub bstrText: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl TF_LBBALLOONINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_LBBALLOONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TF_LBBALLOONINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TF_LBBALLOONINFO").field("style", &self.style).field("bstrText", &self.bstrText).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_LBBALLOONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.style == other.style && self.bstrText == other.bstrText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_LBBALLOONINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TF_LBBALLOONINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_BALLOON: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_BITMAP: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_BMPF_VERTICAL: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_CUSTOMUI: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_DESC_MAXLEN: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_ICON: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STATUS: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STATUS_BTN_TOGGLED: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STATUS_DISABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STATUS_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STYLE_BTN_BUTTON: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STYLE_BTN_MENU: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STYLE_BTN_TOGGLE: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STYLE_HIDDENBYDEFAULT: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STYLE_HIDDENSTATUSCONTROL: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STYLE_HIDEONNOOTHERITEMS: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STYLE_SHOWNINTRAY: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STYLE_SHOWNINTRAYONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_STYLE_TEXTCOLORICON: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_TEXT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBI_TOOLTIP: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBMENUF_CHECKED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBMENUF_GRAYED: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBMENUF_RADIOCHECKED: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBMENUF_SEPARATOR: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_LBMENUF_SUBMENU: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_LMLATTELEMENT {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
pub struct TF_LMLATTELEMENT {
    pub dwFrameStart: u32,
    pub dwFrameLen: u32,
    pub dwFlags: u32,
    pub Anonymous: TF_LMLATTELEMENT_0,
    pub bstrText: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl TF_LMLATTELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_LMLATTELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_LMLATTELEMENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_LMLATTELEMENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TF_LMLATTELEMENT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union TF_LMLATTELEMENT_0 {
    pub iCost: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl TF_LMLATTELEMENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_LMLATTELEMENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_LMLATTELEMENT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_LMLATTELEMENT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TF_LMLATTELEMENT_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_MENUREADY: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_MOD_ALT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_MOD_CONTROL: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_MOD_IGNORE_ALL_MODIFIER: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_MOD_LALT: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_MOD_LCONTROL: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_MOD_LSHIFT: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_MOD_ON_KEYUP: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_MOD_RALT: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_MOD_RCONTROL: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_MOD_RSHIFT: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_MOD_SHIFT: u32 = 4u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub struct TF_PERSISTENT_PROPERTY_HEADER_ACP {
    pub guidType: ::windows::runtime::GUID,
    pub ichStart: i32,
    pub cch: i32,
    pub cb: u32,
    pub dwPrivate: u32,
    pub clsidTIP: ::windows::runtime::GUID,
}
impl TF_PERSISTENT_PROPERTY_HEADER_ACP {}
impl ::core::default::Default for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TF_PERSISTENT_PROPERTY_HEADER_ACP").field("guidType", &self.guidType).field("ichStart", &self.ichStart).field("cch", &self.cch).field("cb", &self.cb).field("dwPrivate", &self.dwPrivate).field("clsidTIP", &self.clsidTIP).finish()
    }
}
impl ::core::cmp::PartialEq for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn eq(&self, other: &Self) -> bool {
        self.guidType == other.guidType && self.ichStart == other.ichStart && self.cch == other.cch && self.cb == other.cb && self.dwPrivate == other.dwPrivate && self.clsidTIP == other.clsidTIP
    }
}
impl ::core::cmp::Eq for TF_PERSISTENT_PROPERTY_HEADER_ACP {}
unsafe impl ::windows::runtime::Abi for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_POPF_ALL: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub struct TF_PRESERVEDKEY {
    pub uVKey: u32,
    pub uModifiers: u32,
}
impl TF_PRESERVEDKEY {}
impl ::core::default::Default for TF_PRESERVEDKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TF_PRESERVEDKEY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TF_PRESERVEDKEY").field("uVKey", &self.uVKey).field("uModifiers", &self.uModifiers).finish()
    }
}
impl ::core::cmp::PartialEq for TF_PRESERVEDKEY {
    fn eq(&self, other: &Self) -> bool {
        self.uVKey == other.uVKey && self.uModifiers == other.uModifiers
    }
}
impl ::core::cmp::Eq for TF_PRESERVEDKEY {}
unsafe impl ::windows::runtime::Abi for TF_PRESERVEDKEY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_PROFILETYPE_INPUTPROCESSOR: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_PROFILETYPE_KEYBOARDLAYOUT: u32 = 2u32;
pub const TF_PROFILE_ARRAY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd38eff65_aa46_4fd5_91a7_67845fb02f5b);
pub const TF_PROFILE_CANTONESE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0aec109c_7e96_11d4_b2ef_0080c882687e);
pub const TF_PROFILE_CHANGJIE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4bdf9f03_c7d3_11d4_b2ab_0080c882687e);
pub const TF_PROFILE_DAYI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x037b2c25_480c_4d7f_b027_d6ca6b69788a);
pub const TF_PROFILE_NEWCHANGJIE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf3ba907a_6c7e_11d4_97fa_0080c882687e);
pub const TF_PROFILE_NEWPHONETIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb2f9c502_1742_11d4_9790_0080c882687e);
pub const TF_PROFILE_NEWQUICK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0b883ba0_c1c7_11d4_87f9_0080c882687e);
pub const TF_PROFILE_PHONETIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x761309de_317a_11d4_9b5d_0080c882687e);
pub const TF_PROFILE_PINYIN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf3ba9077_6c7e_11d4_97fa_0080c882687e);
pub const TF_PROFILE_QUICK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6024b45f_5c54_11d4_b921_0080c882687e);
pub const TF_PROFILE_SIMPLEFAST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfa550b04_5ad7_411f_a5ac_ca038ec515d7);
pub const TF_PROFILE_TIGRINYA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3cab88b7_cc3e_46a6_9765_b772ad7761ff);
pub const TF_PROFILE_WUBI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x82590c13_f4dd_44f4_ba1d_8667246fdf8e);
pub const TF_PROFILE_YI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x409c8376_007b_4357_ae8e_26316ee3fb0d);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for TF_PROPERTYVAL {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
pub struct TF_PROPERTYVAL {
    pub guidId: ::windows::runtime::GUID,
    pub varValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl TF_PROPERTYVAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for TF_PROPERTYVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for TF_PROPERTYVAL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for TF_PROPERTYVAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::runtime::Abi for TF_PROPERTYVAL {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_PROPUI_STATUS_SAVETOFILE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RCM_COMLESS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RCM_HINT_COLLISION: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RCM_HINT_READING_LENGTH: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RCM_VKEY: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RIP_FLAG_FREEUNUSEDLIBRARIES: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RIUIE_CONTEXT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RIUIE_ERRORINDEX: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RIUIE_MAXREADINGSTRINGLENGTH: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RIUIE_STRING: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RIUIE_VERTICALORDER: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RP_HIDDENINSETTINGUI: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RP_LOCALPROCESS: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RP_LOCALTHREAD: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_RP_SUBITEMINSETTINGUI: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SD_LOADING: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SD_READONLY: u32 = 1u32;
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
pub struct TF_SELECTION {
    pub range: ::core::option::Option<ITfRange>,
    pub style: TF_SELECTIONSTYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl TF_SELECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_SELECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TF_SELECTION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TF_SELECTION").field("range", &self.range).field("style", &self.style).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_SELECTION {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range && self.style == other.style
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_SELECTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TF_SELECTION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
pub struct TF_SELECTIONSTYLE {
    pub ase: TfActiveSelEnd,
    pub fInterimChar: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl TF_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_SELECTIONSTYLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TF_SELECTIONSTYLE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TF_SELECTIONSTYLE").field("ase", &self.ase).field("fInterimChar", &self.fInterimChar).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_SELECTIONSTYLE {
    fn eq(&self, other: &Self) -> bool {
        self.ase == other.ase && self.fInterimChar == other.fInterimChar
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TF_SELECTIONSTYLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SENTENCEMODE_AUTOMATIC: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SENTENCEMODE_CONVERSATION: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SENTENCEMODE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SENTENCEMODE_PHRASEPREDICT: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SENTENCEMODE_PLAURALCLAUSE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SENTENCEMODE_SINGLECONVERT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SFT_DESKBAND: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SFT_DOCK: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SFT_EXTRAICONSONMINIMIZED: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SFT_HIDDEN: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SFT_HIGHTRANSPARENCY: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SFT_LABELS: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SFT_LOWTRANSPARENCY: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SFT_MINIMIZED: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SFT_NOEXTRAICONSONMINIMIZED: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SFT_NOLABELS: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SFT_NOTRANSPARENCY: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SFT_SHOWNORMAL: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SHOW_BALLOON: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SPEECHUI_SHOWN: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SS_DISJOINTSEL: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SS_REGIONS: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SS_TKBAUTOCORRECTENABLE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SS_TKBPREDICTIONENABLE: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_SS_TRANSITORY: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_ST_CORRECTION: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_S_ASYNC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(262912i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TF_IGNOREEND: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TF_MOVESTART: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMAE_COMLESS: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMAE_CONSOLE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMAE_NOACTIVATEKEYBOARDLAYOUT: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMAE_NOACTIVATETIP: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMAE_SECUREMODE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMAE_UIELEMENTENABLEDONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMAE_WOW16: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMF_ACTIVATED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMF_COMLESS: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMF_CONSOLE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMF_IMMERSIVEMODE: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMF_NOACTIVATETIP: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMF_SECUREMODE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMF_UIELEMENTENABLEDONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TMF_WOW16: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TRANSITORYEXTENSION_ATSELECTION: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TRANSITORYEXTENSION_FLOATING: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TRANSITORYEXTENSION_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_TU_CORRECTION: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_URP_ALLPROFILES: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_URP_LOCALPROCESS: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_URP_LOCALTHREAD: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TF_US_HIDETIPUI: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_CHANGJIE: u32 = 61506u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_DAYI: u32 = 61507u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_PHONETIC: u32 = 1028u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TKBL_OPT_JAPANESE_ABC: u32 = 1041u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TKBL_OPT_KOREAN_HANGUL_2_BULSIK: u32 = 1042u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TKBL_OPT_SIMPLIFIED_CHINESE_PINYIN: u32 = 2052u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TKBL_OPT_TRADITIONAL_CHINESE_PHONETIC: u32 = 1028u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TKBL_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TKBLayoutType(pub i32);
pub const TKBLT_UNDEFINED: TKBLayoutType = TKBLayoutType(0i32);
pub const TKBLT_CLASSIC: TKBLayoutType = TKBLayoutType(1i32);
pub const TKBLT_OPTIMIZED: TKBLayoutType = TKBLayoutType(2i32);
impl ::core::convert::From<i32> for TKBLayoutType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TKBLayoutType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TKB_ALTERNATES_AUTOCORRECTION_APPLIED: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TKB_ALTERNATES_FOR_AUTOCORRECTION: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TKB_ALTERNATES_FOR_PREDICTION: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TKB_ALTERNATES_STANDARD: u32 = 1u32;
pub const TSATTRID_App: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa80f77df_4237_40e5_849c_b5fa51c13ac7);
pub const TSATTRID_App_IncorrectGrammar: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbd54e398_ad03_4b74_b6b3_5edb19996388);
pub const TSATTRID_App_IncorrectSpelling: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf42de43c_ef12_430d_944c_9a08970a25d2);
pub const TSATTRID_Font: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x573ea825_749b_4f8a_9cfd_21c3605ca828);
pub const TSATTRID_Font_FaceName: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb536aeb6_053b_4eb8_b65a_50da1e81e72e);
pub const TSATTRID_Font_SizePts: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc8493302_a5e9_456d_af04_8005e4130f03);
pub const TSATTRID_Font_Style: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x68b2a77f_6b0e_4f28_8177_571c2f3a42b1);
pub const TSATTRID_Font_Style_Animation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdcf73d22_e029_47b7_bb36_f263a3d004cc);
pub const TSATTRID_Font_Style_Animation_BlinkingBackground: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x86e5b104_0104_4b10_b585_00f2527522b5);
pub const TSATTRID_Font_Style_Animation_LasVegasLights: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf40423d5_0f87_4f8f_bada_e6d60c25e152);
pub const TSATTRID_Font_Style_Animation_MarchingBlackAnts: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7644e067_f186_4902_bfc6_ec815aa20e9d);
pub const TSATTRID_Font_Style_Animation_MarchingRedAnts: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x78368dad_50fb_4c6f_840b_d486bb6cf781);
pub const TSATTRID_Font_Style_Animation_Shimmer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2ce31b58_5293_4c36_8809_bf8bb51a27b3);
pub const TSATTRID_Font_Style_Animation_SparkleText: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x533aad20_962c_4e9f_8c09_b42ea4749711);
pub const TSATTRID_Font_Style_Animation_WipeDown: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5872e874_367b_4803_b160_c90ff62569d0);
pub const TSATTRID_Font_Style_Animation_WipeRight: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb855cbe3_3d2c_4600_b1e9_e1c9ce02f842);
pub const TSATTRID_Font_Style_BackgroundColor: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb50eaa4e_3091_4468_81db_d79ea190c7c7);
pub const TSATTRID_Font_Style_Blink: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbfb2c036_7acf_4532_b720_b416dd7765a8);
pub const TSATTRID_Font_Style_Bold: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x48813a43_8a20_4940_8e58_97823f7b268a);
pub const TSATTRID_Font_Style_Capitalize: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7d85a3ba_b4fd_43b3_befc_6b985c843141);
pub const TSATTRID_Font_Style_Color: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x857a7a37_b8af_4e9a_81b4_acf700c8411b);
pub const TSATTRID_Font_Style_Emboss: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbd8ed742_349e_4e37_82fb_437979cb53a7);
pub const TSATTRID_Font_Style_Engrave: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9c3371de_8332_4897_be5d_89233223179a);
pub const TSATTRID_Font_Style_Height: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7e937477_12e6_458b_926a_1fa44ee8f391);
pub const TSATTRID_Font_Style_Hidden: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb1e28770_881c_475f_863f_887a647b1090);
pub const TSATTRID_Font_Style_Italic: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8740682a_a765_48e1_acfc_d22222b2f810);
pub const TSATTRID_Font_Style_Kerning: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcc26e1b4_2f9a_47c8_8bff_bf1eb7cce0dd);
pub const TSATTRID_Font_Style_Lowercase: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x76d8ccb5_ca7b_4498_8ee9_d5c4f6f74c60);
pub const TSATTRID_Font_Style_Outlined: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x10e6db31_db0d_4ac6_a7f5_9c9cff6f2ab4);
pub const TSATTRID_Font_Style_Overline: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe3989f4a_992b_4301_8ce1_a5b7c6d1f3c8);
pub const TSATTRID_Font_Style_Overline_Double: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdc46063a_e115_46e3_bcd8_ca6772aa95b4);
pub const TSATTRID_Font_Style_Overline_Single: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8440d94c_51ce_47b2_8d4c_15751e5f721b);
pub const TSATTRID_Font_Style_Position: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x15cd26ab_f2fb_4062_b5a6_9a49e1a5cc0b);
pub const TSATTRID_Font_Style_Protected: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1c557cb2_14cf_4554_a574_ecb2f7e7efd4);
pub const TSATTRID_Font_Style_Shadow: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f686d2f_c6cd_4c56_8a1a_994a4b9766be);
pub const TSATTRID_Font_Style_SmallCaps: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfacb6bc6_9100_4cc6_b969_11eea45a86b4);
pub const TSATTRID_Font_Style_Spacing: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x98c1200d_8f06_409a_8e49_6a554bf7c153);
pub const TSATTRID_Font_Style_Strikethrough: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0c562193_2d08_4668_9601_ced41309d7af);
pub const TSATTRID_Font_Style_Strikethrough_Double: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x62489b31_a3e7_4f94_ac43_ebaf8fcc7a9f);
pub const TSATTRID_Font_Style_Strikethrough_Single: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x75d736b6_3c8f_4b97_ab78_1877cb990d31);
pub const TSATTRID_Font_Style_Subscript: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5774fb84_389b_43bc_a74b_1568347cf0f4);
pub const TSATTRID_Font_Style_Superscript: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2ea4993c_563c_49aa_9372_0bef09a9255b);
pub const TSATTRID_Font_Style_Underline: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc3c9c9f3_7902_444b_9a7b_48e70f4b50f7);
pub const TSATTRID_Font_Style_Underline_Double: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x74d24aa6_1db3_4c69_a176_31120e7586d5);
pub const TSATTRID_Font_Style_Underline_Single: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1b6720e5_0f73_4951_a6b3_6f19e43c9461);
pub const TSATTRID_Font_Style_Uppercase: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x33a300e8_e340_4937_b697_8f234045cd9a);
pub const TSATTRID_Font_Style_Weight: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x12f3189c_8bb0_461b_b1fa_eaf907047fe0);
pub const TSATTRID_List: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x436d673b_26f1_4aee_9e65_8f83a4ed4884);
pub const TSATTRID_List_LevelIndel: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7f7cc899_311f_487b_ad5d_e2a459e12d42);
pub const TSATTRID_List_Type: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xae3e665e_4bce_49e3_a0fe_2db47d3a17ae);
pub const TSATTRID_List_Type_Arabic: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1338c5d6_98a3_4fa3_9bd1_7a60eef8e9e0);
pub const TSATTRID_List_Type_Bullet: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbccd77c5_4c4d_4ce2_b102_559f3b2bfcea);
pub const TSATTRID_List_Type_LowerLetter: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x96372285_f3cf_491e_a925_3832347fd237);
pub const TSATTRID_List_Type_LowerRoman: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x90466262_3980_4b8e_9368_918bd1218a41);
pub const TSATTRID_List_Type_UpperLetter: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7987b7cd_ce52_428b_9b95_a357f6f10c45);
pub const TSATTRID_List_Type_UpperRoman: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0f6ab552_4a80_467f_b2f1_127e2aa3ba9e);
pub const TSATTRID_OTHERS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb3c32af9_57d0_46a9_bca8_dac238a13057);
pub const TSATTRID_Text: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7edb8e68_81f9_449d_a15a_87a8388faac0);
pub const TSATTRID_Text_Alignment: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x139941e6_1767_456d_938e_35ba568b5cd4);
pub const TSATTRID_Text_Alignment_Center: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa4a95c16_53bf_4d55_8b87_4bdd8d4275fc);
pub const TSATTRID_Text_Alignment_Justify: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xed350740_a0f7_42d3_8ea8_f81b6488faf0);
pub const TSATTRID_Text_Alignment_Left: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x16ae95d3_6361_43a2_8495_d00f397f1693);
pub const TSATTRID_Text_Alignment_Right: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb36f0f98_1b9e_4360_8616_03fb08a78456);
pub const TSATTRID_Text_EmbeddedObject: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7edb8e68_81f9_449d_a15a_87a8388faac0);
pub const TSATTRID_Text_Hyphenation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdadf4525_618e_49eb_b1a8_3b68bd7648e3);
pub const TSATTRID_Text_Language: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd8c04ef1_5753_4c25_8887_85443fe5f819);
pub const TSATTRID_Text_Link: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x47cd9051_3722_4cd8_b7c8_4e17ca1759f5);
pub const TSATTRID_Text_Orientation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6bab707f_8785_4c39_8b52_96f878303ffb);
pub const TSATTRID_Text_Para: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5edc5822_99dc_4dd6_aec3_b62baa5b2e7c);
pub const TSATTRID_Text_Para_FirstLineIndent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x07c97a13_7472_4dd8_90a9_91e3d7e4f29c);
pub const TSATTRID_Text_Para_LeftIndent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfb2848e9_7471_41c9_b6b3_8a1450e01897);
pub const TSATTRID_Text_Para_LineSpacing: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x699b380d_7f8c_46d6_a73b_dfe3d1538df3);
pub const TSATTRID_Text_Para_LineSpacing_AtLeast: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xadfedf31_2d44_4434_a5ff_7f4c4990a905);
pub const TSATTRID_Text_Para_LineSpacing_Double: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x82fb1805_a6c4_4231_ac12_6260af2aba28);
pub const TSATTRID_Text_Para_LineSpacing_Exactly: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3d45ad40_23de_48d7_a6b3_765420c620cc);
pub const TSATTRID_Text_Para_LineSpacing_Multiple: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x910f1e3c_d6d0_4f65_8a3c_42b4b31868c5);
pub const TSATTRID_Text_Para_LineSpacing_OnePtFive: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0428a021_0397_4b57_9a17_0795994cd3c5);
pub const TSATTRID_Text_Para_LineSpacing_Single: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xed350740_a0f7_42d3_8ea8_f81b6488faf0);
pub const TSATTRID_Text_Para_RightIndent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2c7f26f9_a5e2_48da_b98a_520cb16513bf);
pub const TSATTRID_Text_Para_SpaceAfter: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7b0a3f55_22dc_425f_a411_93da1d8f9baa);
pub const TSATTRID_Text_Para_SpaceBefore: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8df98589_194a_4601_b251_9865a3e906dd);
pub const TSATTRID_Text_ReadOnly: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x85836617_de32_4afd_a50f_a2db110e6e4d);
pub const TSATTRID_Text_RightToLeft: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xca666e71_1b08_453d_bfdd_28e08c8aaf7a);
pub const TSATTRID_Text_VerticalWriting: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6bba8195_046f_4ea9_b311_97fd66c4274b);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_AS_ATTR_CHANGE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_AS_LAYOUT_CHANGE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_AS_SEL_CHANGE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_AS_STATUS_CHANGE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_AS_TEXT_CHANGE: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for TS_ATTRVAL {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
pub struct TS_ATTRVAL {
    pub idAttr: ::windows::runtime::GUID,
    pub dwOverlapId: u32,
    pub varValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl TS_ATTRVAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for TS_ATTRVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for TS_ATTRVAL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for TS_ATTRVAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::runtime::Abi for TS_ATTRVAL {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_ATTR_FIND_BACKWARDS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_ATTR_FIND_HIDDEN: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_ATTR_FIND_UPDATESTART: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_ATTR_FIND_WANT_END: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_ATTR_FIND_WANT_OFFSET: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_ATTR_FIND_WANT_VALUE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_CHAR_EMBEDDED: u32 = 65532u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_CHAR_REGION: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_CHAR_REPLACEMENT: u32 = 65533u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_E_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220982i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_E_INVALIDPOINT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220985i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_E_INVALIDPOS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220992i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_E_NOINTERFACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220988i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_E_NOLAYOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220986i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_E_NOLOCK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220991i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_E_NOOBJECT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220990i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_E_NOSELECTION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220987i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_E_NOSERVICE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220989i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_E_READONLY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220983i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_E_SYNCHRONOUS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220984i32 as _);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_GEA_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_GTA_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_IAS_NOQUERY: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_IAS_QUERYONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_IE_COMPOSITION: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_IE_CORRECTION: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_LF_SYNC: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub struct TS_RUNINFO {
    pub uCount: u32,
    pub r#type: TsRunType,
}
impl TS_RUNINFO {}
impl ::core::default::Default for TS_RUNINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TS_RUNINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TS_RUNINFO").field("uCount", &self.uCount).field("r#type", &self.r#type).finish()
    }
}
impl ::core::cmp::PartialEq for TS_RUNINFO {
    fn eq(&self, other: &Self) -> bool {
        self.uCount == other.uCount && self.r#type == other.r#type
    }
}
impl ::core::cmp::Eq for TS_RUNINFO {}
unsafe impl ::windows::runtime::Abi for TS_RUNINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_ENABLED: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_VISIBLE: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SD_INPUTPANEMANUALDISPLAYENABLE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SD_LOADING: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SD_READONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SD_RESERVED: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SD_TKBAUTOCORRECTENABLE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SD_TKBPREDICTIONENABLE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SD_UIINTEGRATIONENABLE: u32 = 32u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
pub struct TS_SELECTIONSTYLE {
    pub ase: TsActiveSelEnd,
    pub fInterimChar: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl TS_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TS_SELECTIONSTYLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TS_SELECTIONSTYLE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TS_SELECTIONSTYLE").field("ase", &self.ase).field("fInterimChar", &self.fInterimChar).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TS_SELECTIONSTYLE {
    fn eq(&self, other: &Self) -> bool {
        self.ase == other.ase && self.fInterimChar == other.fInterimChar
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TS_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TS_SELECTIONSTYLE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
pub struct TS_SELECTION_ACP {
    pub acpStart: i32,
    pub acpEnd: i32,
    pub style: TS_SELECTIONSTYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl TS_SELECTION_ACP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TS_SELECTION_ACP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TS_SELECTION_ACP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TS_SELECTION_ACP").field("acpStart", &self.acpStart).field("acpEnd", &self.acpEnd).field("style", &self.style).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TS_SELECTION_ACP {
    fn eq(&self, other: &Self) -> bool {
        self.acpStart == other.acpStart && self.acpEnd == other.acpEnd && self.style == other.style
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TS_SELECTION_ACP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TS_SELECTION_ACP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
pub struct TS_SELECTION_ANCHOR {
    pub paStart: ::core::option::Option<IAnchor>,
    pub paEnd: ::core::option::Option<IAnchor>,
    pub style: TS_SELECTIONSTYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl TS_SELECTION_ANCHOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TS_SELECTION_ANCHOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TS_SELECTION_ANCHOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TS_SELECTION_ANCHOR").field("paStart", &self.paStart).field("paEnd", &self.paEnd).field("style", &self.style).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TS_SELECTION_ANCHOR {
    fn eq(&self, other: &Self) -> bool {
        self.paStart == other.paStart && self.paEnd == other.paEnd && self.style == other.style
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TS_SELECTION_ANCHOR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TS_SELECTION_ANCHOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SHIFT_COUNT_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SHIFT_COUNT_ONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SHIFT_HALT_HIDDEN: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SHIFT_HALT_VISIBLE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SS_DISJOINTSEL: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SS_NOHIDDENTEXT: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SS_REGIONS: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SS_TKBAUTOCORRECTENABLE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SS_TKBPREDICTIONENABLE: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SS_TRANSITORY: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_SS_UWPCONTROL: u32 = 64u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub struct TS_STATUS {
    pub dwDynamicFlags: u32,
    pub dwStaticFlags: u32,
}
impl TS_STATUS {}
impl ::core::default::Default for TS_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TS_STATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TS_STATUS").field("dwDynamicFlags", &self.dwDynamicFlags).field("dwStaticFlags", &self.dwStaticFlags).finish()
    }
}
impl ::core::cmp::PartialEq for TS_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwDynamicFlags == other.dwDynamicFlags && self.dwStaticFlags == other.dwStaticFlags
    }
}
impl ::core::cmp::Eq for TS_STATUS {}
unsafe impl ::windows::runtime::Abi for TS_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_STRF_END: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_STRF_MID: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_STRF_START: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_S_ASYNC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(262912i32 as _);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub struct TS_TEXTCHANGE {
    pub acpStart: i32,
    pub acpOldEnd: i32,
    pub acpNewEnd: i32,
}
impl TS_TEXTCHANGE {}
impl ::core::default::Default for TS_TEXTCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TS_TEXTCHANGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TS_TEXTCHANGE").field("acpStart", &self.acpStart).field("acpOldEnd", &self.acpOldEnd).field("acpNewEnd", &self.acpNewEnd).finish()
    }
}
impl ::core::cmp::PartialEq for TS_TEXTCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.acpStart == other.acpStart && self.acpOldEnd == other.acpOldEnd && self.acpNewEnd == other.acpNewEnd
    }
}
impl ::core::cmp::Eq for TS_TEXTCHANGE {}
unsafe impl ::windows::runtime::Abi for TS_TEXTCHANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
pub const TS_VCOOKIE_NUL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TfActiveSelEnd(pub i32);
pub const TF_AE_NONE: TfActiveSelEnd = TfActiveSelEnd(0i32);
pub const TF_AE_START: TfActiveSelEnd = TfActiveSelEnd(1i32);
pub const TF_AE_END: TfActiveSelEnd = TfActiveSelEnd(2i32);
impl ::core::convert::From<i32> for TfActiveSelEnd {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TfActiveSelEnd {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TfAnchor(pub i32);
pub const TF_ANCHOR_START: TfAnchor = TfAnchor(0i32);
pub const TF_ANCHOR_END: TfAnchor = TfAnchor(1i32);
impl ::core::convert::From<i32> for TfAnchor {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TfAnchor {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TfCandidateResult(pub i32);
pub const CAND_FINALIZED: TfCandidateResult = TfCandidateResult(0i32);
pub const CAND_SELECTED: TfCandidateResult = TfCandidateResult(1i32);
pub const CAND_CANCELED: TfCandidateResult = TfCandidateResult(2i32);
impl ::core::convert::From<i32> for TfCandidateResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TfCandidateResult {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TfGravity(pub i32);
pub const TF_GRAVITY_BACKWARD: TfGravity = TfGravity(0i32);
pub const TF_GRAVITY_FORWARD: TfGravity = TfGravity(1i32);
impl ::core::convert::From<i32> for TfGravity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TfGravity {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TfIntegratableCandidateListSelectionStyle(pub i32);
pub const STYLE_ACTIVE_SELECTION: TfIntegratableCandidateListSelectionStyle = TfIntegratableCandidateListSelectionStyle(0i32);
pub const STYLE_IMPLIED_SELECTION: TfIntegratableCandidateListSelectionStyle = TfIntegratableCandidateListSelectionStyle(1i32);
impl ::core::convert::From<i32> for TfIntegratableCandidateListSelectionStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TfIntegratableCandidateListSelectionStyle {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TfLBBalloonStyle(pub i32);
pub const TF_LB_BALLOON_RECO: TfLBBalloonStyle = TfLBBalloonStyle(0i32);
pub const TF_LB_BALLOON_SHOW: TfLBBalloonStyle = TfLBBalloonStyle(1i32);
pub const TF_LB_BALLOON_MISS: TfLBBalloonStyle = TfLBBalloonStyle(2i32);
impl ::core::convert::From<i32> for TfLBBalloonStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TfLBBalloonStyle {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TfLBIClick(pub i32);
pub const TF_LBI_CLK_RIGHT: TfLBIClick = TfLBIClick(1i32);
pub const TF_LBI_CLK_LEFT: TfLBIClick = TfLBIClick(2i32);
impl ::core::convert::From<i32> for TfLBIClick {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TfLBIClick {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TfLayoutCode(pub i32);
pub const TF_LC_CREATE: TfLayoutCode = TfLayoutCode(0i32);
pub const TF_LC_CHANGE: TfLayoutCode = TfLayoutCode(1i32);
pub const TF_LC_DESTROY: TfLayoutCode = TfLayoutCode(2i32);
impl ::core::convert::From<i32> for TfLayoutCode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TfLayoutCode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TfSapiObject(pub i32);
pub const GETIF_RESMGR: TfSapiObject = TfSapiObject(0i32);
pub const GETIF_RECOCONTEXT: TfSapiObject = TfSapiObject(1i32);
pub const GETIF_RECOGNIZER: TfSapiObject = TfSapiObject(2i32);
pub const GETIF_VOICE: TfSapiObject = TfSapiObject(3i32);
pub const GETIF_DICTGRAM: TfSapiObject = TfSapiObject(4i32);
pub const GETIF_RECOGNIZERNOINIT: TfSapiObject = TfSapiObject(5i32);
impl ::core::convert::From<i32> for TfSapiObject {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TfSapiObject {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TfShiftDir(pub i32);
pub const TF_SD_BACKWARD: TfShiftDir = TfShiftDir(0i32);
pub const TF_SD_FORWARD: TfShiftDir = TfShiftDir(1i32);
impl ::core::convert::From<i32> for TfShiftDir {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TfShiftDir {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TsActiveSelEnd(pub i32);
pub const TS_AE_NONE: TsActiveSelEnd = TsActiveSelEnd(0i32);
pub const TS_AE_START: TsActiveSelEnd = TsActiveSelEnd(1i32);
pub const TS_AE_END: TsActiveSelEnd = TsActiveSelEnd(2i32);
impl ::core::convert::From<i32> for TsActiveSelEnd {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TsActiveSelEnd {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TsGravity(pub i32);
pub const TS_GR_BACKWARD: TsGravity = TsGravity(0i32);
pub const TS_GR_FORWARD: TsGravity = TsGravity(1i32);
impl ::core::convert::From<i32> for TsGravity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TsGravity {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TsLayoutCode(pub i32);
pub const TS_LC_CREATE: TsLayoutCode = TsLayoutCode(0i32);
pub const TS_LC_CHANGE: TsLayoutCode = TsLayoutCode(1i32);
pub const TS_LC_DESTROY: TsLayoutCode = TsLayoutCode(2i32);
impl ::core::convert::From<i32> for TsLayoutCode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TsLayoutCode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TsRunType(pub i32);
pub const TS_RT_PLAIN: TsRunType = TsRunType(0i32);
pub const TS_RT_HIDDEN: TsRunType = TsRunType(1i32);
pub const TS_RT_OPAQUE: TsRunType = TsRunType(2i32);
impl ::core::convert::From<i32> for TsRunType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TsRunType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TsShiftDir(pub i32);
pub const TS_SD_BACKWARD: TsShiftDir = TsShiftDir(0i32);
pub const TS_SD_FORWARD: TsShiftDir = TsShiftDir(1i32);
impl ::core::convert::From<i32> for TsShiftDir {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TsShiftDir {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[inline]
pub unsafe fn UninitLocalMsCtfMonitor() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UninitLocalMsCtfMonitor() -> ::windows::runtime::HRESULT;
        }
        UninitLocalMsCtfMonitor().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}

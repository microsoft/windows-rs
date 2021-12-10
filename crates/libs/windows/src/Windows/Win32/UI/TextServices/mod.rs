#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type ANCHOR_CHANGE_HISTORY_FLAGS = u32;
pub const TS_CH_PRECEDING_DEL: ANCHOR_CHANGE_HISTORY_FLAGS = 1u32;
pub const TS_CH_FOLLOWING_DEL: ANCHOR_CHANGE_HISTORY_FLAGS = 2u32;
pub const AccClientDocMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc48cc30_4f3e_4fa1_803b_ad0e196a83b1);
pub const AccDictionary: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6572ee16_5fe5_4331_bb6d_76a49c56e423);
pub const AccServerDocMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6089a37e_eb8a_482d_bd6f_f9f46904d16d);
pub const AccStore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5440837f_4bff_4ae5_a1b1_7722ecc6332a);
pub const CLSID_TF_CategoryMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4b544a1_438d_4b41_9325_869523e2d6c7);
pub const CLSID_TF_ClassicLangBar: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3318360c_1afc_4d09_a86b_9f9cb6dceb9c);
pub const CLSID_TF_DisplayAttributeMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ce74de4_53d3_4d74_8b83_431b3828ba53);
pub const CLSID_TF_InputProcessorProfiles: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33c53a50_f456_4884_b049_85fd643ecfed);
pub const CLSID_TF_LangBarItemMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9931692_a2b3_4fab_bf33_9ec6f9fb96ac);
pub const CLSID_TF_LangBarMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebb08c45_6c4a_4fdc_ae53_4eb8c4c7db8e);
pub const CLSID_TF_ThreadMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x529a9e6b_6587_4f23_ab9e_9c7d683e3c50);
pub const CLSID_TF_TransitoryExtensionUIEntry: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae6be008_07fb_400d_8beb_337a64f7051f);
pub const CLSID_TsfServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39aedc00_6b60_46db_8d31_3642be0e4373);
pub const DCM_FLAGS_CTFMON: u32 = 2u32;
pub const DCM_FLAGS_LOCALTHREADTSF: u32 = 4u32;
pub const DCM_FLAGS_TASKENG: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DoMsCtfMonitor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(dwflags: u32, heventforservicestop: Param1) -> super::super::Foundation::BOOL {
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
pub const DocWrap: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf426f7e_7a5e_44d6_830c_a390ea9462a3);
pub type GET_TEXT_AND_PROPERTY_UPDATES_FLAGS = u32;
pub const TF_GTP_NONE: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS = 0u32;
pub const TF_GTP_INCL_TEXT: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS = 1u32;
pub const GUID_APP_FUNCTIONPROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4caef01e_12af_4b0e_9db1_a6ec5b881208);
pub const GUID_COMPARTMENT_CONVERSIONMODEBIAS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5497f516_ee91_436e_b946_aa2c05f1ac5b);
pub const GUID_COMPARTMENT_EMPTYCONTEXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7487dbf_804e_41c5_894d_ad96fd4eea13);
pub const GUID_COMPARTMENT_ENABLED_PROFILES_UPDATED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92c1fd48_a9ae_4a7c_be08_4329e4723817);
pub const GUID_COMPARTMENT_HANDWRITING_OPENCLOSE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9ae2c6b_1866_4361_af72_7aa30948890e);
pub const GUID_COMPARTMENT_KEYBOARD_DISABLED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71a5b253_1951_466b_9fbc_9c8808fa84f2);
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6592511_bcee_4122_a7c4_09f4b3fa4396);
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE_CONVERSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccf05dd8_4a87_11d7_a6e2_00065b84435c);
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE_SENTENCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccf05dd9_4a87_11d7_a6e2_00065b84435c);
pub const GUID_COMPARTMENT_KEYBOARD_OPENCLOSE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58273aad_01bb_4164_95c6_755ba0b5162d);
pub const GUID_COMPARTMENT_SAPI_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51af2086_cc6b_457d_b5aa_8b19dc290ab4);
pub const GUID_COMPARTMENT_SPEECH_CFGMENU: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c5c2d_4e83_4bb6_91a2_e019bff6762d);
pub const GUID_COMPARTMENT_SPEECH_DISABLED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56c5c607_0703_4e59_8e52_cbc84e8bbe35);
pub const GUID_COMPARTMENT_SPEECH_GLOBALSTATE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a54fe8e_0d08_460c_a75d_87035ff436c5);
pub const GUID_COMPARTMENT_SPEECH_OPENCLOSE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x544d6a63_e2e8_4752_bbd1_000960bca083);
pub const GUID_COMPARTMENT_SPEECH_UI_STATUS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd92016f0_9367_4fe7_9abf_bc59dacbe0e3);
pub const GUID_COMPARTMENT_TIPUISTATUS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x148ca3ec_0366_401c_8d75_ed978d85fbc9);
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8be347f5_c7a0_11d7_b408_00065b84435c);
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION_DOCUMENTMANAGER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8be347f7_c7a0_11d7_b408_00065b84435c);
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION_PARENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8be347f8_c7a0_11d7_b408_00065b84435c);
pub const GUID_INTEGRATIONSTYLE_SEARCHBOX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6d1bd11_82f7_4903_ae21_1a6397cde2eb);
pub const GUID_LBI_INPUTMODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c77a81e_41cc_4178_a3a7_5f8a987568e6);
pub const GUID_LBI_SAPILAYR_CFGMENUBUTTON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd02f24a1_942d_422e_8d99_b4f2addee999);
pub const GUID_MODEBIAS_CHINESE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7add26de_4328_489b_83ae_6493750cad5c);
pub const GUID_MODEBIAS_CONVERSATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f4ec104_1790_443b_95f1_e10f939d6546);
pub const GUID_MODEBIAS_DATETIME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2bdb372_7f61_4039_92ef_1c35599f0222);
pub const GUID_MODEBIAS_FILENAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7f707fe_44c6_4fca_8e76_86ab50c7931b);
pub const GUID_MODEBIAS_FULLWIDTHALPHANUMERIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81489fb8_b36a_473d_8146_e4a2258b24ae);
pub const GUID_MODEBIAS_FULLWIDTHHANGUL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc01ae6c9_45b5_4fd0_9cb1_9f4cebc39fea);
pub const GUID_MODEBIAS_HALFWIDTHKATAKANA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x005f6b63_78d4_41cc_8859_485ca821a795);
pub const GUID_MODEBIAS_HANGUL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76ef0541_23b3_4d77_a074_691801ccea17);
pub const GUID_MODEBIAS_HIRAGANA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd73d316e_9b91_46f1_a280_31597f52c694);
pub const GUID_MODEBIAS_KATAKANA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e0eeddd_3a1a_499e_8543_3c7ee7949811);
pub const GUID_MODEBIAS_NAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfddc10f0_d239_49bf_b8fc_5410caaa427e);
pub const GUID_MODEBIAS_NONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const GUID_MODEBIAS_NUMERIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4021766c_e872_48fd_9cee_4ec5c75e16c3);
pub const GUID_MODEBIAS_READING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe31643a3_6466_4cbf_8d8b_0bd4d8545461);
pub const GUID_MODEBIAS_URLHISTORY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b0e54d9_63f2_4c68_84d4_79aee7a59f09);
pub const GUID_PROP_ATTRIBUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34b45670_7526_11d2_a147_00105a2799b5);
pub const GUID_PROP_COMPOSING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe12ac060_af15_11d2_afc5_00105a2799b5);
pub const GUID_PROP_INPUTSCOPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1713dd5a_68e7_4a5b_9af6_592a595c778d);
pub const GUID_PROP_LANGID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3280ce20_8032_11d2_b603_00105a2799b5);
pub const GUID_PROP_MODEBIAS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x372e0716_974f_40ac_a088_08cdc92ebfbc);
pub const GUID_PROP_READING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5463f7c0_8e31_11d2_bf46_00105a2799b5);
pub const GUID_PROP_TEXTOWNER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1e2d520_0969_11d3_8df0_00105a2799b5);
pub const GUID_PROP_TKB_ALTERNATES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70b2a803_968d_462e_b93b_2164c91517f7);
pub const GUID_SYSTEM_FUNCTIONPROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a698bb0_0f21_11d3_8df1_00105a2799b5);
pub const GUID_TFCAT_CATEGORY_OF_TIP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x534c48c1_0607_4098_a521_4fc899c73e90);
pub const GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb95f181b_ea4c_4af1_8056_7c321abbb091);
pub const GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x046b8c80_1647_40f7_9b21_b93b81aabc1b);
pub const GUID_TFCAT_PROPSTYLE_STATIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x565fb8d8_6bd4_4ca1_b223_0f2ccb8f4f96);
pub const GUID_TFCAT_PROP_AUDIODATA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7be3a9_e8ab_4d47_a8fe_254fa423436d);
pub const GUID_TFCAT_PROP_INKDATA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c6a82ae_b0d7_4f14_a745_14f28b009d61);
pub const GUID_TFCAT_TIPCAP_COMLESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x364215d9_75bc_11d7_a6ef_00065b84435c);
pub const GUID_TFCAT_TIPCAP_DUALMODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3af314a2_d79f_4b1b_9992_15086d339b05);
pub const GUID_TFCAT_TIPCAP_IMMERSIVEONLY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a4259ac_640d_4ad4_89f7_1eb67e7c4ee8);
pub const GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13a016df_560b_46cd_947a_4c3af1e0e35d);
pub const GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccf05dd7_4a87_11d7_a6e2_00065b84435c);
pub const GUID_TFCAT_TIPCAP_LOCALSERVER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74769ee9_4a66_4f9d_90d6_bf8b7c3eb461);
pub const GUID_TFCAT_TIPCAP_SECUREMODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d2f9ce_1f5e_11d7_a6d3_00065b84435c);
pub const GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25504fb4_7bab_4bc1_9c69_cf81890f0ef5);
pub const GUID_TFCAT_TIPCAP_TSF3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07dcb4af_98de_4548_bef7_25bd45979a1f);
pub const GUID_TFCAT_TIPCAP_UIELEMENTENABLED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d2f9cf_1f5e_11d7_a6d3_00065b84435c);
pub const GUID_TFCAT_TIPCAP_WOW16: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x364215da_75bc_11d7_a6ef_00065b84435c);
pub const GUID_TFCAT_TIP_HANDWRITING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x246ecb87_c2f2_4abe_905b_c8b38add2c43);
pub const GUID_TFCAT_TIP_KEYBOARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34745c63_b2f0_4784_8b67_5e12c8701a31);
pub const GUID_TFCAT_TIP_SPEECH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5a73cd1_8355_426b_a161_259808f26b14);
pub const GUID_TFCAT_TRANSITORYEXTENSIONUI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6302de22_a5cf_4b02_bfe8_4d72b2bed3c6);
pub const GUID_TS_SERVICE_ACCESSIBLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9786200_a5bf_4a0f_8c24_fb16f5d1aabb);
pub const GUID_TS_SERVICE_ACTIVEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea937a50_c9a6_4b7d_894a_49d99b784834);
pub const GUID_TS_SERVICE_DATAOBJECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6086fbb5_e225_46ce_a770_c1bbd3e05d7b);
pub const GXFPF_NEAREST: u32 = 2u32;
pub const GXFPF_ROUND_NEAREST: u32 = 1u32;
pub type HKL = isize;
#[repr(transparent)]
pub struct IAccClientDocMgr(::windows::core::IUnknown);
impl IAccClientDocMgr {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocuments(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LookupByHWND<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LookupByPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::POINT>>(&self, pt: Param0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pt.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetFocused(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IAccClientDocMgr> for ::windows::core::IUnknown {
    fn from(value: IAccClientDocMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccClientDocMgr> for ::windows::core::IUnknown {
    fn from(value: &IAccClientDocMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAccClientDocMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAccClientDocMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAccClientDocMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAccClientDocMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccClientDocMgr {}
unsafe impl ::windows::core::Interface for IAccClientDocMgr {
    type Vtable = IAccClientDocMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c896039_7b6d_49e6_a8c1_45116a98292b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccClientDocMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IAccDictionary(::windows::core::IUnknown);
impl IAccDictionary {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalizedString(&self, term: *const ::windows::core::GUID, lcid: u32, presult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(term), ::core::mem::transmute(lcid), ::core::mem::transmute(presult), ::core::mem::transmute(plcid)).ok()
    }
    pub unsafe fn GetParentTerm(&self, term: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(term), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMnemonicString(&self, term: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(term), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LookupMnemonicTerm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmnemonic: Param0) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrmnemonic.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ConvertValueToString<'a, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, term: *const ::windows::core::GUID, lcid: u32, varvalue: Param2, pbstrresult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(term), ::core::mem::transmute(lcid), varvalue.into_param().abi(), ::core::mem::transmute(pbstrresult), ::core::mem::transmute(plcid)).ok()
    }
}
impl ::core::convert::From<IAccDictionary> for ::windows::core::IUnknown {
    fn from(value: IAccDictionary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccDictionary> for ::windows::core::IUnknown {
    fn from(value: &IAccDictionary) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAccDictionary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAccDictionary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAccDictionary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAccDictionary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccDictionary {}
unsafe impl ::windows::core::Interface for IAccDictionary {
    type Vtable = IAccDictionaryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dc4cb5f_d737_474d_ade9_5ccfc9bc1cc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccDictionaryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, lcid: u32, presult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, pparentterm: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, presult: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmnemonic: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pterm: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, lcid: u32, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrresult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct IAccServerDocMgr(::windows::core::IUnknown);
impl IAccServerDocMgr {
    pub unsafe fn NewDocument<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    pub unsafe fn RevokeDocument<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn OnDocumentFocus<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAccServerDocMgr> for ::windows::core::IUnknown {
    fn from(value: IAccServerDocMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccServerDocMgr> for ::windows::core::IUnknown {
    fn from(value: &IAccServerDocMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAccServerDocMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAccServerDocMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAccServerDocMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAccServerDocMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccServerDocMgr {}
unsafe impl ::windows::core::Interface for IAccServerDocMgr {
    type Vtable = IAccServerDocMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad7c73cf_6dd5_4855_abc2_b04bad5b9153);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccServerDocMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IAccStore(::windows::core::IUnknown);
impl IAccStore {
    pub unsafe fn Register<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    pub unsafe fn Unregister<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocuments(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LookupByHWND<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LookupByPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::POINT>>(&self, pt: Param0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pt.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn OnDocumentFocus<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn GetFocused(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IAccStore> for ::windows::core::IUnknown {
    fn from(value: IAccStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccStore> for ::windows::core::IUnknown {
    fn from(value: &IAccStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAccStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAccStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAccStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAccStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccStore {}
unsafe impl ::windows::core::Interface for IAccStore {
    type Vtable = IAccStoreVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2cd4a63_2b72_4d48_b739_95e4765195ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccStoreVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IAnchor(::windows::core::IUnknown);
impl IAnchor {
    pub unsafe fn SetGravity(&self, gravity: TsGravity) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(gravity)).ok()
    }
    pub unsafe fn GetGravity(&self) -> ::windows::core::Result<TsGravity> {
        let mut result__: TsGravity = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TsGravity>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IAnchor>>(&self, pawith: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pawith.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Compare<'a, Param0: ::windows::core::IntoParam<'a, IAnchor>>(&self, pawith: Param0) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pawith.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn Shift<'a, Param3: ::windows::core::IntoParam<'a, IAnchor>>(&self, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(cchreq), ::core::mem::transmute(pcch), pahaltanchor.into_param().abi()).ok()
    }
    pub unsafe fn ShiftTo<'a, Param0: ::windows::core::IntoParam<'a, IAnchor>>(&self, pasite: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pasite.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShiftRegion(&self, dwflags: u32, dir: TsShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dir), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetChangeHistoryMask(&self, dwmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmask)).ok()
    }
    pub unsafe fn GetChangeHistory(&self) -> ::windows::core::Result<ANCHOR_CHANGE_HISTORY_FLAGS> {
        let mut result__: ANCHOR_CHANGE_HISTORY_FLAGS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ANCHOR_CHANGE_HISTORY_FLAGS>(result__)
    }
    pub unsafe fn ClearChangeHistory(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IAnchor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IAnchor>(result__)
    }
}
impl ::core::convert::From<IAnchor> for ::windows::core::IUnknown {
    fn from(value: IAnchor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAnchor> for ::windows::core::IUnknown {
    fn from(value: &IAnchor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAnchor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAnchor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAnchor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAnchor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAnchor {}
unsafe impl ::windows::core::Interface for IAnchor {
    type Vtable = IAnchorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0feb7e34_5a60_4356_8ef7_abdec2ff7cf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnchorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gravity: TsGravity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgravity: *mut TsGravity) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pawith: ::windows::core::RawPtr, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pawith: ::windows::core::RawPtr, plresult: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasite: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dir: TsShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhistory: *mut ANCHOR_CHANGE_HISTORY_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaclone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IClonableWrapper(::windows::core::IUnknown);
impl IClonableWrapper {
    pub unsafe fn CloneNewWrapper<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IClonableWrapper> for ::windows::core::IUnknown {
    fn from(value: IClonableWrapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClonableWrapper> for ::windows::core::IUnknown {
    fn from(value: &IClonableWrapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IClonableWrapper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IClonableWrapper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClonableWrapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClonableWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClonableWrapper {}
unsafe impl ::windows::core::Interface for IClonableWrapper {
    type Vtable = IClonableWrapperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb33e75ff_e84c_4dca_a25c_33b8dc003374);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClonableWrapperVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ICoCreateLocally(::windows::core::IUnknown);
impl ICoCreateLocally {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CoCreateLocally<'a, Param5: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param6: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, rclsid: *const ::windows::core::GUID, dwclscontext: u32, riid: *const ::windows::core::GUID, punk: *mut ::core::option::Option<::windows::core::IUnknown>, riidparam: *const ::windows::core::GUID, punkparam: Param5, varparam: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(riid), ::core::mem::transmute(punk), ::core::mem::transmute(riidparam), punkparam.into_param().abi(), varparam.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ICoCreateLocally> for ::windows::core::IUnknown {
    fn from(value: ICoCreateLocally) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoCreateLocally> for ::windows::core::IUnknown {
    fn from(value: &ICoCreateLocally) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICoCreateLocally {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICoCreateLocally {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoCreateLocally {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoCreateLocally {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoCreateLocally {}
unsafe impl ::windows::core::Interface for ICoCreateLocally {
    type Vtable = ICoCreateLocallyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03de00aa_f272_41e3_99cb_03c5e8114ea0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoCreateLocallyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwclscontext: u32, riid: *const ::windows::core::GUID, punk: *mut *mut ::core::ffi::c_void, riidparam: *const ::windows::core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct ICoCreatedLocally(::windows::core::IUnknown);
impl ICoCreatedLocally {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LocalInit<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, punklocalobject: Param0, riidparam: *const ::windows::core::GUID, punkparam: Param2, varparam: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punklocalobject.into_param().abi(), ::core::mem::transmute(riidparam), punkparam.into_param().abi(), varparam.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ICoCreatedLocally> for ::windows::core::IUnknown {
    fn from(value: ICoCreatedLocally) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoCreatedLocally> for ::windows::core::IUnknown {
    fn from(value: &ICoCreatedLocally) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICoCreatedLocally {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICoCreatedLocally {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoCreatedLocally {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoCreatedLocally {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoCreatedLocally {}
unsafe impl ::windows::core::Interface for ICoCreatedLocally {
    type Vtable = ICoCreatedLocallyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a53eb6c_1908_4742_8cff_2cee2e93f94c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoCreatedLocallyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punklocalobject: *mut ::core::ffi::c_void, riidparam: *const ::windows::core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct IDocWrap(::windows::core::IUnknown);
impl IDocWrap {
    pub unsafe fn SetDoc<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    pub unsafe fn GetWrappedDoc(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IDocWrap> for ::windows::core::IUnknown {
    fn from(value: IDocWrap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDocWrap> for ::windows::core::IUnknown {
    fn from(value: &IDocWrap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDocWrap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDocWrap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDocWrap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDocWrap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDocWrap {}
unsafe impl ::windows::core::Interface for IDocWrap {
    type Vtable = IDocWrapVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcd285fe_0be0_43bd_99c9_aaaec513c555);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDocWrapVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumITfCompositionView(::windows::core::IUnknown);
impl IEnumITfCompositionView {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumITfCompositionView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumITfCompositionView>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, rgcompositionview: *mut ::core::option::Option<ITfCompositionView>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgcompositionview), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumITfCompositionView> for ::windows::core::IUnknown {
    fn from(value: IEnumITfCompositionView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumITfCompositionView> for ::windows::core::IUnknown {
    fn from(value: &IEnumITfCompositionView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumITfCompositionView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumITfCompositionView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumITfCompositionView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumITfCompositionView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumITfCompositionView {}
unsafe impl ::windows::core::Interface for IEnumITfCompositionView {
    type Vtable = IEnumITfCompositionViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5efd22ba_7838_46cb_88e2_cadb14124f8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumITfCompositionViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgcompositionview: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumSpeechCommands(::windows::core::IUnknown);
impl IEnumSpeechCommands {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSpeechCommands> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSpeechCommands>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pspcmds), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumSpeechCommands> for ::windows::core::IUnknown {
    fn from(value: IEnumSpeechCommands) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSpeechCommands> for ::windows::core::IUnknown {
    fn from(value: &IEnumSpeechCommands) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumSpeechCommands {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumSpeechCommands {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumSpeechCommands {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSpeechCommands {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSpeechCommands {}
unsafe impl ::windows::core::Interface for IEnumSpeechCommands {
    type Vtable = IEnumSpeechCommandsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c5dac4f_083c_4b85_a4c9_71746048adca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSpeechCommandsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfCandidates(::windows::core::IUnknown);
impl IEnumTfCandidates {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfCandidates> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfCandidates>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, ppcand: *mut ::core::option::Option<ITfCandidateString>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppcand), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfCandidates> for ::windows::core::IUnknown {
    fn from(value: IEnumTfCandidates) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfCandidates> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfCandidates) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfCandidates {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfCandidates {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfCandidates {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfCandidates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfCandidates {}
unsafe impl ::windows::core::Interface for IEnumTfCandidates {
    type Vtable = IEnumTfCandidatesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdefb1926_6c80_4ce8_87d4_d6b72b812bde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfCandidatesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppcand: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfContextViews(::windows::core::IUnknown);
impl IEnumTfContextViews {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfContextViews> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfContextViews>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, rgviews: *mut ::core::option::Option<ITfContextView>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgviews), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfContextViews> for ::windows::core::IUnknown {
    fn from(value: IEnumTfContextViews) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfContextViews> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfContextViews) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfContextViews {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfContextViews {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfContextViews {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfContextViews {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfContextViews {}
unsafe impl ::windows::core::Interface for IEnumTfContextViews {
    type Vtable = IEnumTfContextViewsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0c0f8dd_cf38_44e1_bb0f_68cf0d551c78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfContextViewsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgviews: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfContexts(::windows::core::IUnknown);
impl IEnumTfContexts {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfContexts> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfContexts>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, rgcontext: *mut ::core::option::Option<ITfContext>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgcontext), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfContexts> for ::windows::core::IUnknown {
    fn from(value: IEnumTfContexts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfContexts> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfContexts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfContexts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfContexts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfContexts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfContexts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfContexts {}
unsafe impl ::windows::core::Interface for IEnumTfContexts {
    type Vtable = IEnumTfContextsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1a7ea6_1654_4502_a86e_b2902344d507);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfContextsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgcontext: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfDisplayAttributeInfo(::windows::core::IUnknown);
impl IEnumTfDisplayAttributeInfo {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfDisplayAttributeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfDisplayAttributeInfo>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, rginfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rginfo), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfDisplayAttributeInfo> for ::windows::core::IUnknown {
    fn from(value: IEnumTfDisplayAttributeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfDisplayAttributeInfo> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfDisplayAttributeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfDisplayAttributeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfDisplayAttributeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfDisplayAttributeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfDisplayAttributeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfDisplayAttributeInfo {}
unsafe impl ::windows::core::Interface for IEnumTfDisplayAttributeInfo {
    type Vtable = IEnumTfDisplayAttributeInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cef04d7_cb75_4e80_a7ab_5f5bc7d332de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfDisplayAttributeInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rginfo: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfDocumentMgrs(::windows::core::IUnknown);
impl IEnumTfDocumentMgrs {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfDocumentMgrs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfDocumentMgrs>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, rgdocumentmgr: *mut ::core::option::Option<ITfDocumentMgr>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgdocumentmgr), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfDocumentMgrs> for ::windows::core::IUnknown {
    fn from(value: IEnumTfDocumentMgrs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfDocumentMgrs> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfDocumentMgrs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfDocumentMgrs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfDocumentMgrs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfDocumentMgrs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfDocumentMgrs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfDocumentMgrs {}
unsafe impl ::windows::core::Interface for IEnumTfDocumentMgrs {
    type Vtable = IEnumTfDocumentMgrsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e808_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfDocumentMgrsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgdocumentmgr: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfFunctionProviders(::windows::core::IUnknown);
impl IEnumTfFunctionProviders {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfFunctionProviders> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfFunctionProviders>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, ppcmdobj: *mut ::core::option::Option<ITfFunctionProvider>, pcfetch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppcmdobj), ::core::mem::transmute(pcfetch)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfFunctionProviders> for ::windows::core::IUnknown {
    fn from(value: IEnumTfFunctionProviders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfFunctionProviders> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfFunctionProviders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfFunctionProviders {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfFunctionProviders {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfFunctionProviders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfFunctionProviders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfFunctionProviders {}
unsafe impl ::windows::core::Interface for IEnumTfFunctionProviders {
    type Vtable = IEnumTfFunctionProvidersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4b24db0_0990_11d3_8df0_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfFunctionProvidersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppcmdobj: *mut ::windows::core::RawPtr, pcfetch: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfInputProcessorProfiles(::windows::core::IUnknown);
impl IEnumTfInputProcessorProfiles {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfInputProcessorProfiles> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfInputProcessorProfiles>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pprofile), ::core::mem::transmute(pcfetch)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfInputProcessorProfiles> for ::windows::core::IUnknown {
    fn from(value: IEnumTfInputProcessorProfiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfInputProcessorProfiles> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfInputProcessorProfiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfInputProcessorProfiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfInputProcessorProfiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfInputProcessorProfiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfInputProcessorProfiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfInputProcessorProfiles {}
unsafe impl ::windows::core::Interface for IEnumTfInputProcessorProfiles {
    type Vtable = IEnumTfInputProcessorProfilesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71c6e74d_0f28_11d8_a82a_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfInputProcessorProfilesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfLangBarItems(::windows::core::IUnknown);
impl IEnumTfLangBarItems {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfLangBarItems> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfLangBarItems>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, ppitem: *mut ::core::option::Option<ITfLangBarItem>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppitem), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfLangBarItems> for ::windows::core::IUnknown {
    fn from(value: IEnumTfLangBarItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfLangBarItems> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfLangBarItems) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfLangBarItems {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfLangBarItems {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfLangBarItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfLangBarItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfLangBarItems {}
unsafe impl ::windows::core::Interface for IEnumTfLangBarItems {
    type Vtable = IEnumTfLangBarItemsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x583f34d0_de25_11d2_afdd_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfLangBarItemsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfLanguageProfiles(::windows::core::IUnknown);
impl IEnumTfLanguageProfiles {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfLanguageProfiles> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfLanguageProfiles>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pprofile), ::core::mem::transmute(pcfetch)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfLanguageProfiles> for ::windows::core::IUnknown {
    fn from(value: IEnumTfLanguageProfiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfLanguageProfiles> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfLanguageProfiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfLanguageProfiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfLanguageProfiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfLanguageProfiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfLanguageProfiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfLanguageProfiles {}
unsafe impl ::windows::core::Interface for IEnumTfLanguageProfiles {
    type Vtable = IEnumTfLanguageProfilesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d61bf11_ac5f_42c8_a4cb_931bcc28c744);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfLanguageProfilesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfLatticeElements(::windows::core::IUnknown);
impl IEnumTfLatticeElements {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfLatticeElements> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfLatticeElements>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgselements), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfLatticeElements> for ::windows::core::IUnknown {
    fn from(value: IEnumTfLatticeElements) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfLatticeElements> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfLatticeElements) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfLatticeElements {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfLatticeElements {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfLatticeElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfLatticeElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfLatticeElements {}
unsafe impl ::windows::core::Interface for IEnumTfLatticeElements {
    type Vtable = IEnumTfLatticeElementsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56988052_47da_4a05_911a_e3d941f17145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfLatticeElementsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfProperties(::windows::core::IUnknown);
impl IEnumTfProperties {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfProperties>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, ppprop: *mut ::core::option::Option<ITfProperty>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppprop), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfProperties> for ::windows::core::IUnknown {
    fn from(value: IEnumTfProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfProperties> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfProperties {}
unsafe impl ::windows::core::Interface for IEnumTfProperties {
    type Vtable = IEnumTfPropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19188cb0_aca9_11d2_afc5_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfPropertiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppprop: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfPropertyValue(::windows::core::IUnknown);
impl IEnumTfPropertyValue {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfPropertyValue> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfPropertyValue>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgvalues), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfPropertyValue> for ::windows::core::IUnknown {
    fn from(value: IEnumTfPropertyValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfPropertyValue> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfPropertyValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfPropertyValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfPropertyValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfPropertyValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfPropertyValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfPropertyValue {}
unsafe impl ::windows::core::Interface for IEnumTfPropertyValue {
    type Vtable = IEnumTfPropertyValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ed8981b_7c10_4d7d_9fb3_ab72e9c75f72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfPropertyValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfRanges(::windows::core::IUnknown);
impl IEnumTfRanges {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfRanges> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfRanges>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, pprange: *mut ::core::option::Option<ITfRange>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pprange), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfRanges> for ::windows::core::IUnknown {
    fn from(value: IEnumTfRanges) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfRanges> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfRanges) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfRanges {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfRanges {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfRanges {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfRanges {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfRanges {}
unsafe impl ::windows::core::Interface for IEnumTfRanges {
    type Vtable = IEnumTfRangesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf99d3f40_8e32_11d2_bf46_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfRangesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pprange: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumTfUIElements(::windows::core::IUnknown);
impl IEnumTfUIElements {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfUIElements> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfUIElements>(result__)
    }
    pub unsafe fn Next(&self, ulcount: u32, ppelement: *mut ::core::option::Option<ITfUIElement>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppelement), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumTfUIElements> for ::windows::core::IUnknown {
    fn from(value: IEnumTfUIElements) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumTfUIElements> for ::windows::core::IUnknown {
    fn from(value: &IEnumTfUIElements) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumTfUIElements {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumTfUIElements {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumTfUIElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumTfUIElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfUIElements {}
unsafe impl ::windows::core::Interface for IEnumTfUIElements {
    type Vtable = IEnumTfUIElementsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x887aa91e_acba_4931_84da_3c5208cf543f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfUIElementsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppelement: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IInternalDocWrap(::windows::core::IUnknown);
impl IInternalDocWrap {
    pub unsafe fn NotifyRevoke(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IInternalDocWrap> for ::windows::core::IUnknown {
    fn from(value: IInternalDocWrap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternalDocWrap> for ::windows::core::IUnknown {
    fn from(value: &IInternalDocWrap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternalDocWrap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInternalDocWrap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInternalDocWrap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInternalDocWrap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternalDocWrap {}
unsafe impl ::windows::core::Interface for IInternalDocWrap {
    type Vtable = IInternalDocWrapVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1aa6466_9db4_40ba_be03_77c38e8e60b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalDocWrapVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
pub const ILMCM_CHECKLAYOUTANDTIPENABLED: u32 = 1u32;
pub const ILMCM_LANGUAGEBAROFF: u32 = 2u32;
pub type INSERT_TEXT_AT_SELECTION_FLAGS = u32;
pub const TF_IAS_NOQUERY: INSERT_TEXT_AT_SELECTION_FLAGS = 1u32;
pub const TF_IAS_QUERYONLY: INSERT_TEXT_AT_SELECTION_FLAGS = 2u32;
pub const TF_IAS_NO_DEFAULT_COMPOSITION: INSERT_TEXT_AT_SELECTION_FLAGS = 2147483648u32;
#[repr(transparent)]
pub struct ISpeechCommandProvider(::windows::core::IUnknown);
impl ISpeechCommandProvider {
    pub unsafe fn EnumSpeechCommands(&self, langid: u16) -> ::windows::core::Result<IEnumSpeechCommands> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSpeechCommands>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessCommand<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszcommand: Param0, cch: u32, langid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszcommand.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(langid)).ok()
    }
}
impl ::core::convert::From<ISpeechCommandProvider> for ::windows::core::IUnknown {
    fn from(value: ISpeechCommandProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpeechCommandProvider> for ::windows::core::IUnknown {
    fn from(value: &ISpeechCommandProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpeechCommandProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISpeechCommandProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpeechCommandProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpeechCommandProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpeechCommandProvider {}
unsafe impl ::windows::core::Interface for ISpeechCommandProvider {
    type Vtable = ISpeechCommandProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38e09d4c_586d_435a_b592_c8a86691dec6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechCommandProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcommand: super::super::Foundation::PWSTR, cch: u32, langid: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITextStoreACP(::windows::core::IUnknown);
impl ITextStoreACP {
    pub unsafe fn AdviseSink<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1, dwmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwmask)).ok()
    }
    pub unsafe fn UnadviseSink<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<TS_STATUS> {
        let mut result__: TS_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TS_STATUS>(result__)
    }
    pub unsafe fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpteststart), ::core::mem::transmute(acptestend), ::core::mem::transmute(cch), ::core::mem::transmute(pacpresultstart), ::core::mem::transmute(pacpresultend)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection), ::core::mem::transmute(pcfetched)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(pchplain), ::core::mem::transmute(cchplainreq), ::core::mem::transmute(pcchplainret), ::core::mem::transmute(prgruninfo), ::core::mem::transmute(cruninforeq), ::core::mem::transmute(pcruninforet), ::core::mem::transmute(pacpnext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: Param3, cch: u32) -> ::windows::core::Result<TS_TEXTCHANGE> {
        let mut result__: TS_TEXTCHANGE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), pchtext.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(&mut result__)).from_abi::<TS_TEXTCHANGE>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> ::windows::core::Result<super::super::System::Com::IDataObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IDataObject>(result__)
    }
    pub unsafe fn GetEmbedded(&self, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(acppos), ::core::mem::transmute(rguidservice), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidservice), ::core::mem::transmute(pformatetc), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<'a, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: Param3) -> ::windows::core::Result<TS_TEXTCHANGE> {
        let mut result__: TS_TEXTCHANGE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), pdataobject.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<TS_TEXTCHANGE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertTextAtSelection<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, pchtext: Param1, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pchtext.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(pacpstart), ::core::mem::transmute(pacpend), ::core::mem::transmute(pchange)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbeddedAtSelection<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dwflags: u32, pdataobject: Param1, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pdataobject.into_param().abi(), ::core::mem::transmute(pacpstart), ::core::mem::transmute(pacpend), ::core::mem::transmute(pchange)).ok()
    }
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs)).ok()
    }
    pub unsafe fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(acppos), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(acppos), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acphalt), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags), ::core::mem::transmute(pacpnext), ::core::mem::transmute(pffound), ::core::mem::transmute(plfoundoffset)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(paattrvals), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn GetEndACP(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn GetActiveView(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(ptscreen), ::core::mem::transmute(dwflags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(prc), ::core::mem::transmute(pfclipped)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScreenExt(&self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWnd(&self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
}
impl ::core::convert::From<ITextStoreACP> for ::windows::core::IUnknown {
    fn from(value: ITextStoreACP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoreACP> for ::windows::core::IUnknown {
    fn from(value: &ITextStoreACP) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStoreACP {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStoreACP {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStoreACP {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStoreACP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACP {}
unsafe impl ::windows::core::Interface for ITextStoreACP {
    type Vtable = ITextStoreACPVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28888fe3_c2a0_483a_a3ea_8cb1ce51ff3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: super::super::Foundation::PWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::windows::core::RawPtr, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: ::windows::core::RawPtr, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITextStoreACP2(::windows::core::IUnknown);
impl ITextStoreACP2 {
    pub unsafe fn AdviseSink<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1, dwmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwmask)).ok()
    }
    pub unsafe fn UnadviseSink<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<TS_STATUS> {
        let mut result__: TS_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TS_STATUS>(result__)
    }
    pub unsafe fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpteststart), ::core::mem::transmute(acptestend), ::core::mem::transmute(cch), ::core::mem::transmute(pacpresultstart), ::core::mem::transmute(pacpresultend)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection), ::core::mem::transmute(pcfetched)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(pchplain), ::core::mem::transmute(cchplainreq), ::core::mem::transmute(pcchplainret), ::core::mem::transmute(prgruninfo), ::core::mem::transmute(cruninforeq), ::core::mem::transmute(pcruninforet), ::core::mem::transmute(pacpnext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: Param3, cch: u32) -> ::windows::core::Result<TS_TEXTCHANGE> {
        let mut result__: TS_TEXTCHANGE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), pchtext.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(&mut result__)).from_abi::<TS_TEXTCHANGE>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> ::windows::core::Result<super::super::System::Com::IDataObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IDataObject>(result__)
    }
    pub unsafe fn GetEmbedded(&self, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(acppos), ::core::mem::transmute(rguidservice), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidservice), ::core::mem::transmute(pformatetc), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<'a, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: Param3) -> ::windows::core::Result<TS_TEXTCHANGE> {
        let mut result__: TS_TEXTCHANGE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), pdataobject.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<TS_TEXTCHANGE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertTextAtSelection<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, pchtext: Param1, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pchtext.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(pacpstart), ::core::mem::transmute(pacpend), ::core::mem::transmute(pchange)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbeddedAtSelection<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dwflags: u32, pdataobject: Param1, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pdataobject.into_param().abi(), ::core::mem::transmute(pacpstart), ::core::mem::transmute(pacpend), ::core::mem::transmute(pchange)).ok()
    }
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs)).ok()
    }
    pub unsafe fn RequestAttrsAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(acppos), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(acppos), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acphalt), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags), ::core::mem::transmute(pacpnext), ::core::mem::transmute(pffound), ::core::mem::transmute(plfoundoffset)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(paattrvals), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn GetEndACP(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn GetActiveView(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(ptscreen), ::core::mem::transmute(dwflags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(prc), ::core::mem::transmute(pfclipped)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScreenExt(&self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
}
impl ::core::convert::From<ITextStoreACP2> for ::windows::core::IUnknown {
    fn from(value: ITextStoreACP2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoreACP2> for ::windows::core::IUnknown {
    fn from(value: &ITextStoreACP2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStoreACP2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStoreACP2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStoreACP2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStoreACP2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACP2 {}
unsafe impl ::windows::core::Interface for ITextStoreACP2 {
    type Vtable = ITextStoreACP2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf86ad89f_5fe4_4b8d_bb9f_ef3797a84f1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACP2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: super::super::Foundation::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: super::super::Foundation::PWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::windows::core::RawPtr, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: ::windows::core::RawPtr, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITextStoreACPEx(::windows::core::IUnknown);
impl ITextStoreACPEx {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScrollToRect<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::RECT>>(&self, acpstart: i32, acpend: i32, rc: Param2, dwposition: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), rc.into_param().abi(), ::core::mem::transmute(dwposition)).ok()
    }
}
impl ::core::convert::From<ITextStoreACPEx> for ::windows::core::IUnknown {
    fn from(value: ITextStoreACPEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoreACPEx> for ::windows::core::IUnknown {
    fn from(value: &ITextStoreACPEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStoreACPEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStoreACPEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStoreACPEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStoreACPEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACPEx {}
unsafe impl ::windows::core::Interface for ITextStoreACPEx {
    type Vtable = ITextStoreACPExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2de3bc2_3d8e_11d3_81a9_f753fbe61a00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITextStoreACPServices(::windows::core::IUnknown);
impl ITextStoreACPServices {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<'a, Param0: ::windows::core::IntoParam<'a, ITfProperty>, Param1: ::windows::core::IntoParam<'a, ITfRange>, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pprop: Param0, prange: Param1, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pprop.into_param().abi(), prange.into_param().abi(), ::core::mem::transmute(phdr), pstream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Unserialize<'a, Param0: ::windows::core::IntoParam<'a, ITfProperty>, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param3: ::windows::core::IntoParam<'a, ITfPersistentPropertyLoaderACP>>(&self, pprop: Param0, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Param2, ploader: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pprop.into_param().abi(), ::core::mem::transmute(phdr), pstream.into_param().abi(), ploader.into_param().abi()).ok()
    }
    pub unsafe fn ForceLoadProperty<'a, Param0: ::windows::core::IntoParam<'a, ITfProperty>>(&self, pprop: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pprop.into_param().abi()).ok()
    }
    pub unsafe fn CreateRange(&self, acpstart: i32, acpend: i32) -> ::windows::core::Result<ITfRangeACP> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(&mut result__)).from_abi::<ITfRangeACP>(result__)
    }
}
impl ::core::convert::From<ITextStoreACPServices> for ::windows::core::IUnknown {
    fn from(value: ITextStoreACPServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoreACPServices> for ::windows::core::IUnknown {
    fn from(value: &ITextStoreACPServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStoreACPServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStoreACPServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStoreACPServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStoreACPServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACPServices {}
unsafe impl ::windows::core::Interface for ITextStoreACPServices {
    type Vtable = ITextStoreACPServicesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e901_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPServicesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, prange: ::windows::core::RawPtr, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr, ploader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITextStoreACPSink(::windows::core::IUnknown);
impl ITextStoreACPSink {
    pub unsafe fn OnTextChange(&self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(pchange)).ok()
    }
    pub unsafe fn OnSelectionChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcode), ::core::mem::transmute(vcview)).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn OnAttrsChange(&self, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(cattrs), ::core::mem::transmute(paattrs)).ok()
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags)).ok()
    }
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITextStoreACPSink> for ::windows::core::IUnknown {
    fn from(value: ITextStoreACPSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoreACPSink> for ::windows::core::IUnknown {
    fn from(value: &ITextStoreACPSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStoreACPSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStoreACPSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStoreACPSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStoreACPSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACPSink {}
unsafe impl ::windows::core::Interface for ITextStoreACPSink {
    type Vtable = ITextStoreACPSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22d44c94_a419_4542_a272_ae26093ececf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITextStoreACPSinkEx(::windows::core::IUnknown);
impl ITextStoreACPSinkEx {
    pub unsafe fn OnTextChange(&self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(pchange)).ok()
    }
    pub unsafe fn OnSelectionChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcode), ::core::mem::transmute(vcview)).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn OnAttrsChange(&self, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(cattrs), ::core::mem::transmute(paattrs)).ok()
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags)).ok()
    }
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnDisconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITextStoreACPSink> for ITextStoreACPSinkEx {
    fn into_param(self) -> ::windows::core::Param<'a, ITextStoreACPSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextStoreACPSink> for &ITextStoreACPSinkEx {
    fn into_param(self) -> ::windows::core::Param<'a, ITextStoreACPSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextStoreACPSinkEx> for ::windows::core::IUnknown {
    fn from(value: ITextStoreACPSinkEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoreACPSinkEx> for ::windows::core::IUnknown {
    fn from(value: &ITextStoreACPSinkEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStoreACPSinkEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStoreACPSinkEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStoreACPSinkEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStoreACPSinkEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACPSinkEx {}
unsafe impl ::windows::core::Interface for ITextStoreACPSinkEx {
    type Vtable = ITextStoreACPSinkExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bdf9464_41e2_43e3_950c_a6865ba25cd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPSinkExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITextStoreAnchor(::windows::core::IUnknown);
impl ITextStoreAnchor {
    pub unsafe fn AdviseSink<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1, dwmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwmask)).ok()
    }
    pub unsafe fn UnadviseSink<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<TS_STATUS> {
        let mut result__: TS_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TS_STATUS>(result__)
    }
    pub unsafe fn QueryInsert<'a, Param0: ::windows::core::IntoParam<'a, IAnchor>, Param1: ::windows::core::IntoParam<'a, IAnchor>>(&self, pateststart: Param0, patestend: Param1, cch: u32, pparesultstart: *mut ::core::option::Option<IAnchor>, pparesultend: *mut ::core::option::Option<IAnchor>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pateststart.into_param().abi(), patestend.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(pparesultstart), ::core::mem::transmute(pparesultend)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSelection(&self, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection), ::core::mem::transmute(pcfetched)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSelection(&self, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText<'a, Param1: ::windows::core::IntoParam<'a, IAnchor>, Param2: ::windows::core::IntoParam<'a, IAnchor>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwflags: u32, pastart: Param1, paend: Param2, pchtext: super::super::Foundation::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pastart.into_param().abi(), paend.into_param().abi(), ::core::mem::transmute(pchtext), ::core::mem::transmute(cchreq), ::core::mem::transmute(pcch), fupdateanchor.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText<'a, Param1: ::windows::core::IntoParam<'a, IAnchor>, Param2: ::windows::core::IntoParam<'a, IAnchor>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, pastart: Param1, paend: Param2, pchtext: Param3, cch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pastart.into_param().abi(), paend.into_param().abi(), pchtext.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText<'a, Param0: ::windows::core::IntoParam<'a, IAnchor>, Param1: ::windows::core::IntoParam<'a, IAnchor>>(&self, pastart: Param0, paend: Param1) -> ::windows::core::Result<super::super::System::Com::IDataObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pastart.into_param().abi(), paend.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IDataObject>(result__)
    }
    pub unsafe fn GetEmbedded<'a, Param1: ::windows::core::IntoParam<'a, IAnchor>>(&self, dwflags: u32, papos: Param1, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), papos.into_param().abi(), ::core::mem::transmute(rguidservice), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<'a, Param1: ::windows::core::IntoParam<'a, IAnchor>, Param2: ::windows::core::IntoParam<'a, IAnchor>, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dwflags: u32, pastart: Param1, paend: Param2, pdataobject: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pastart.into_param().abi(), paend.into_param().abi(), pdataobject.into_param().abi()).ok()
    }
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs)).ok()
    }
    pub unsafe fn RequestAttrsAtPosition<'a, Param0: ::windows::core::IntoParam<'a, IAnchor>>(&self, papos: Param0, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), papos.into_param().abi(), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn RequestAttrsTransitioningAtPosition<'a, Param0: ::windows::core::IntoParam<'a, IAnchor>>(&self, papos: Param0, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), papos.into_param().abi(), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindNextAttrTransition<'a, Param0: ::windows::core::IntoParam<'a, IAnchor>, Param1: ::windows::core::IntoParam<'a, IAnchor>>(&self, pastart: Param0, pahalt: Param1, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pastart.into_param().abi(), pahalt.into_param().abi(), ::core::mem::transmute(cfilterattrs), ::core::mem::transmute(pafilterattrs), ::core::mem::transmute(dwflags), ::core::mem::transmute(pffound), ::core::mem::transmute(plfoundoffset)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(paattrvals), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn GetStart(&self) -> ::windows::core::Result<IAnchor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IAnchor>(result__)
    }
    pub unsafe fn GetEnd(&self) -> ::windows::core::Result<IAnchor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IAnchor>(result__)
    }
    pub unsafe fn GetActiveView(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnchorFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<IAnchor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(ptscreen), ::core::mem::transmute(dwflags), ::core::mem::transmute(&mut result__)).from_abi::<IAnchor>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextExt<'a, Param1: ::windows::core::IntoParam<'a, IAnchor>, Param2: ::windows::core::IntoParam<'a, IAnchor>>(&self, vcview: u32, pastart: Param1, paend: Param2, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), pastart.into_param().abi(), paend.into_param().abi(), ::core::mem::transmute(prc), ::core::mem::transmute(pfclipped)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScreenExt(&self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWnd(&self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(vcview), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidservice), ::core::mem::transmute(pformatetc), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertTextAtSelection<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, pchtext: Param1, cch: u32, ppastart: *mut ::core::option::Option<IAnchor>, ppaend: *mut ::core::option::Option<IAnchor>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pchtext.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(ppastart), ::core::mem::transmute(ppaend)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbeddedAtSelection<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dwflags: u32, pdataobject: Param1, ppastart: *mut ::core::option::Option<IAnchor>, ppaend: *mut ::core::option::Option<IAnchor>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pdataobject.into_param().abi(), ::core::mem::transmute(ppastart), ::core::mem::transmute(ppaend)).ok()
    }
}
impl ::core::convert::From<ITextStoreAnchor> for ::windows::core::IUnknown {
    fn from(value: ITextStoreAnchor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoreAnchor> for ::windows::core::IUnknown {
    fn from(value: &ITextStoreAnchor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStoreAnchor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStoreAnchor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStoreAnchor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStoreAnchor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreAnchor {}
unsafe impl ::windows::core::Interface for ITextStoreAnchor {
    type Vtable = ITextStoreAnchorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b2077b0_5f18_4dec_bee9_3cc722f5dfe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreAnchorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pateststart: ::windows::core::RawPtr, patestend: ::windows::core::RawPtr, cch: u32, pparesultstart: *mut ::windows::core::RawPtr, pparesultend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, pchtext: super::super::Foundation::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, pchtext: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, papos: ::windows::core::RawPtr, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papos: ::windows::core::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papos: ::windows::core::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, pahalt: ::windows::core::RawPtr, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppastart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, ppasite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: u32, ppastart: *mut ::windows::core::RawPtr, ppaend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: ::windows::core::RawPtr, ppastart: *mut ::windows::core::RawPtr, ppaend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct ITextStoreAnchorEx(::windows::core::IUnknown);
impl ITextStoreAnchorEx {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScrollToRect<'a, Param0: ::windows::core::IntoParam<'a, IAnchor>, Param1: ::windows::core::IntoParam<'a, IAnchor>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::RECT>>(&self, pstart: Param0, pend: Param1, rc: Param2, dwposition: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pstart.into_param().abi(), pend.into_param().abi(), rc.into_param().abi(), ::core::mem::transmute(dwposition)).ok()
    }
}
impl ::core::convert::From<ITextStoreAnchorEx> for ::windows::core::IUnknown {
    fn from(value: ITextStoreAnchorEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoreAnchorEx> for ::windows::core::IUnknown {
    fn from(value: &ITextStoreAnchorEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStoreAnchorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStoreAnchorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStoreAnchorEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStoreAnchorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreAnchorEx {}
unsafe impl ::windows::core::Interface for ITextStoreAnchorEx {
    type Vtable = ITextStoreAnchorExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2de3bc1_3d8e_11d3_81a9_f753fbe61a00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreAnchorExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstart: ::windows::core::RawPtr, pend: ::windows::core::RawPtr, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITextStoreAnchorSink(::windows::core::IUnknown);
impl ITextStoreAnchorSink {
    pub unsafe fn OnTextChange<'a, Param1: ::windows::core::IntoParam<'a, IAnchor>, Param2: ::windows::core::IntoParam<'a, IAnchor>>(&self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: Param1, paend: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pastart.into_param().abi(), paend.into_param().abi()).ok()
    }
    pub unsafe fn OnSelectionChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcode), ::core::mem::transmute(vcview)).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn OnAttrsChange<'a, Param0: ::windows::core::IntoParam<'a, IAnchor>, Param1: ::windows::core::IntoParam<'a, IAnchor>>(&self, pastart: Param0, paend: Param1, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pastart.into_param().abi(), paend.into_param().abi(), ::core::mem::transmute(cattrs), ::core::mem::transmute(paattrs)).ok()
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags)).ok()
    }
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITextStoreAnchorSink> for ::windows::core::IUnknown {
    fn from(value: ITextStoreAnchorSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoreAnchorSink> for ::windows::core::IUnknown {
    fn from(value: &ITextStoreAnchorSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStoreAnchorSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStoreAnchorSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStoreAnchorSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStoreAnchorSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreAnchorSink {}
unsafe impl ::windows::core::Interface for ITextStoreAnchorSink {
    type Vtable = ITextStoreAnchorSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e905_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreAnchorSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITextStoreSinkAnchorEx(::windows::core::IUnknown);
impl ITextStoreSinkAnchorEx {
    pub unsafe fn OnTextChange<'a, Param1: ::windows::core::IntoParam<'a, IAnchor>, Param2: ::windows::core::IntoParam<'a, IAnchor>>(&self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: Param1, paend: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pastart.into_param().abi(), paend.into_param().abi()).ok()
    }
    pub unsafe fn OnSelectionChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcode), ::core::mem::transmute(vcview)).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn OnAttrsChange<'a, Param0: ::windows::core::IntoParam<'a, IAnchor>, Param1: ::windows::core::IntoParam<'a, IAnchor>>(&self, pastart: Param0, paend: Param1, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pastart.into_param().abi(), paend.into_param().abi(), ::core::mem::transmute(cattrs), ::core::mem::transmute(paattrs)).ok()
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlockflags)).ok()
    }
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnDisconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITextStoreAnchorSink> for ITextStoreSinkAnchorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ITextStoreAnchorSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextStoreAnchorSink> for &ITextStoreSinkAnchorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ITextStoreAnchorSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextStoreSinkAnchorEx> for ::windows::core::IUnknown {
    fn from(value: ITextStoreSinkAnchorEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextStoreSinkAnchorEx> for ::windows::core::IUnknown {
    fn from(value: &ITextStoreSinkAnchorEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextStoreSinkAnchorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITextStoreSinkAnchorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextStoreSinkAnchorEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextStoreSinkAnchorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreSinkAnchorEx {}
unsafe impl ::windows::core::Interface for ITextStoreSinkAnchorEx {
    type Vtable = ITextStoreSinkAnchorExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25642426_028d_4474_977b_111bb114fe3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreSinkAnchorExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pastart: ::windows::core::RawPtr, paend: ::windows::core::RawPtr, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfActiveLanguageProfileNotifySink(::windows::core::IUnknown);
impl ITfActiveLanguageProfileNotifySink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnActivated<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, factivated: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), ::core::mem::transmute(guidprofile), factivated.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITfActiveLanguageProfileNotifySink> for ::windows::core::IUnknown {
    fn from(value: ITfActiveLanguageProfileNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfActiveLanguageProfileNotifySink> for ::windows::core::IUnknown {
    fn from(value: &ITfActiveLanguageProfileNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfActiveLanguageProfileNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfActiveLanguageProfileNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfActiveLanguageProfileNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfActiveLanguageProfileNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfActiveLanguageProfileNotifySink {}
unsafe impl ::windows::core::Interface for ITfActiveLanguageProfileNotifySink {
    type Vtable = ITfActiveLanguageProfileNotifySinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb246cb75_a93e_4652_bf8c_b3fe0cfd7e57);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfActiveLanguageProfileNotifySinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, factivated: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfCandidateList(::windows::core::IUnknown);
impl ITfCandidateList {
    pub unsafe fn EnumCandidates(&self) -> ::windows::core::Result<IEnumTfCandidates> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfCandidates>(result__)
    }
    pub unsafe fn GetCandidate(&self, nindex: u32) -> ::windows::core::Result<ITfCandidateString> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(&mut result__)).from_abi::<ITfCandidateString>(result__)
    }
    pub unsafe fn GetCandidateNum(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetResult(&self, nindex: u32, imcr: TfCandidateResult) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(imcr)).ok()
    }
}
impl ::core::convert::From<ITfCandidateList> for ::windows::core::IUnknown {
    fn from(value: ITfCandidateList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCandidateList> for ::windows::core::IUnknown {
    fn from(value: &ITfCandidateList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCandidateList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCandidateList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCandidateList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCandidateList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCandidateList {}
unsafe impl ::windows::core::Interface for ITfCandidateList {
    type Vtable = ITfCandidateListVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3ad50fb_9bdb_49e3_a843_6c76520fbf5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateListVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppcand: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pncnt: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, imcr: TfCandidateResult) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfCandidateListUIElement(::windows::core::IUnknown);
impl ITfCandidateListUIElement {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetUpdatedFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfDocumentMgr>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetString(&self, uindex: u32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetPageIndex(&self, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pindex), ::core::mem::transmute(usize), ::core::mem::transmute(pupagecnt)).ok()
    }
    pub unsafe fn SetPageIndex(&self, pindex: *const u32, upagecnt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pindex), ::core::mem::transmute(upagecnt)).ok()
    }
    pub unsafe fn GetCurrentPage(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITfUIElement> for ITfCandidateListUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ITfUIElement> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfUIElement> for &ITfCandidateListUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ITfUIElement> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfCandidateListUIElement> for ::windows::core::IUnknown {
    fn from(value: ITfCandidateListUIElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCandidateListUIElement> for ::windows::core::IUnknown {
    fn from(value: &ITfCandidateListUIElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCandidateListUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCandidateListUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCandidateListUIElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCandidateListUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCandidateListUIElement {}
unsafe impl ::windows::core::Interface for ITfCandidateListUIElement {
    type Vtable = ITfCandidateListUIElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea1ea138_19df_11d7_a6d2_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateListUIElementVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindex: *const u32, upagecnt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pupage: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfCandidateListUIElementBehavior(::windows::core::IUnknown);
impl ITfCandidateListUIElementBehavior {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetUpdatedFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfDocumentMgr>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetString(&self, uindex: u32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetPageIndex(&self, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pindex), ::core::mem::transmute(usize), ::core::mem::transmute(pupagecnt)).ok()
    }
    pub unsafe fn SetPageIndex(&self, pindex: *const u32, upagecnt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pindex), ::core::mem::transmute(upagecnt)).ok()
    }
    pub unsafe fn GetCurrentPage(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSelection(&self, nindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex)).ok()
    }
    pub unsafe fn Finalize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfCandidateListUIElement> for ITfCandidateListUIElementBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ITfCandidateListUIElement> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfCandidateListUIElement> for &ITfCandidateListUIElementBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ITfCandidateListUIElement> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ITfUIElement> for ITfCandidateListUIElementBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ITfUIElement> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfUIElement> for &ITfCandidateListUIElementBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ITfUIElement> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfCandidateListUIElementBehavior> for ::windows::core::IUnknown {
    fn from(value: ITfCandidateListUIElementBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCandidateListUIElementBehavior> for ::windows::core::IUnknown {
    fn from(value: &ITfCandidateListUIElementBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCandidateListUIElementBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCandidateListUIElementBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCandidateListUIElementBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCandidateListUIElementBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCandidateListUIElementBehavior {}
unsafe impl ::windows::core::Interface for ITfCandidateListUIElementBehavior {
    type Vtable = ITfCandidateListUIElementBehaviorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85fad185_58ce_497a_9460_355366b64b9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateListUIElementBehaviorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindex: *const u32, upagecnt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pupage: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfCandidateString(::windows::core::IUnknown);
impl ITfCandidateString {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ITfCandidateString> for ::windows::core::IUnknown {
    fn from(value: ITfCandidateString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCandidateString> for ::windows::core::IUnknown {
    fn from(value: &ITfCandidateString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCandidateString {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCandidateString {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCandidateString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCandidateString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCandidateString {}
unsafe impl ::windows::core::Interface for ITfCandidateString {
    type Vtable = ITfCandidateStringVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x581f317e_fd9d_443f_b972_ed00467c5d40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateStringVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnindex: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfCategoryMgr(::windows::core::IUnknown);
impl ITfCategoryMgr {
    pub unsafe fn RegisterCategory(&self, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rcatid), ::core::mem::transmute(rguid)).ok()
    }
    pub unsafe fn UnregisterCategory(&self, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rcatid), ::core::mem::transmute(rguid)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCategoriesInItem(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumGUID>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumItemsInCategory(&self, rcatid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rcatid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumGUID>(result__)
    }
    pub unsafe fn FindClosestCategory(&self, rguid: *const ::windows::core::GUID, pcatid: *mut ::windows::core::GUID, ppcatidlist: *const *const ::windows::core::GUID, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(pcatid), ::core::mem::transmute(ppcatidlist), ::core::mem::transmute(ulcount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterGUIDDescription<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, pchdesc: Param2, cch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rguid), pchdesc.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
    pub unsafe fn UnregisterGUIDDescription(&self, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rguid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGUIDDescription(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RegisterGUIDDWORD(&self, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, dw: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rguid), ::core::mem::transmute(dw)).ok()
    }
    pub unsafe fn UnregisterGUIDDWORD(&self, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(rguid)).ok()
    }
    pub unsafe fn GetGUIDDWORD(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn RegisterGUID(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetGUID(&self, guidatom: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidatom), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqualTfGuidAtom(&self, guidatom: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidatom), ::core::mem::transmute(rguid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ITfCategoryMgr> for ::windows::core::IUnknown {
    fn from(value: ITfCategoryMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCategoryMgr> for ::windows::core::IUnknown {
    fn from(value: &ITfCategoryMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCategoryMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCategoryMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCategoryMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCategoryMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCategoryMgr {}
unsafe impl ::windows::core::Interface for ITfCategoryMgr {
    type Vtable = ITfCategoryMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3acefb5_f69d_4905_938f_fcadcf4be830);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCategoryMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rcatid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pcatid: *mut ::windows::core::GUID, ppcatidlist: *const *const ::windows::core::GUID, ulcount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, dw: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pdw: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pguidatom: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidatom: u32, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidatom: u32, rguid: *const ::windows::core::GUID, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfCleanupContextDurationSink(::windows::core::IUnknown);
impl ITfCleanupContextDurationSink {
    pub unsafe fn OnStartCleanupContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnEndCleanupContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITfCleanupContextDurationSink> for ::windows::core::IUnknown {
    fn from(value: ITfCleanupContextDurationSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCleanupContextDurationSink> for ::windows::core::IUnknown {
    fn from(value: &ITfCleanupContextDurationSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCleanupContextDurationSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCleanupContextDurationSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCleanupContextDurationSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCleanupContextDurationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCleanupContextDurationSink {}
unsafe impl ::windows::core::Interface for ITfCleanupContextDurationSink {
    type Vtable = ITfCleanupContextDurationSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45c35144_154e_4797_bed8_d33ae7bf8794);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCleanupContextDurationSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfCleanupContextSink(::windows::core::IUnknown);
impl ITfCleanupContextSink {
    pub unsafe fn OnCleanupContext<'a, Param1: ::windows::core::IntoParam<'a, ITfContext>>(&self, ecwrite: u32, pic: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pic.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITfCleanupContextSink> for ::windows::core::IUnknown {
    fn from(value: ITfCleanupContextSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCleanupContextSink> for ::windows::core::IUnknown {
    fn from(value: &ITfCleanupContextSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCleanupContextSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCleanupContextSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCleanupContextSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCleanupContextSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCleanupContextSink {}
unsafe impl ::windows::core::Interface for ITfCleanupContextSink {
    type Vtable = ITfCleanupContextSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01689689_7acb_4e9b_ab7c_7ea46b12b522);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCleanupContextSinkVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ITfClientId(::windows::core::IUnknown);
impl ITfClientId {
    pub unsafe fn GetClientId(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ITfClientId> for ::windows::core::IUnknown {
    fn from(value: ITfClientId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfClientId> for ::windows::core::IUnknown {
    fn from(value: &ITfClientId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfClientId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfClientId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfClientId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfClientId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfClientId {}
unsafe impl ::windows::core::Interface for ITfClientId {
    type Vtable = ITfClientIdVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd60a7b49_1b9f_4be2_b702_47e9dc05dec3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfClientIdVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ptid: *mut u32) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ITfCompartment(::windows::core::IUnknown);
impl ITfCompartment {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ITfCompartment> for ::windows::core::IUnknown {
    fn from(value: ITfCompartment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCompartment> for ::windows::core::IUnknown {
    fn from(value: &ITfCompartment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCompartment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCompartment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCompartment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCompartment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompartment {}
unsafe impl ::windows::core::Interface for ITfCompartment {
    type Vtable = ITfCompartmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb08f7a9_607a_4384_8623_056892b64371);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompartmentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct ITfCompartmentEventSink(::windows::core::IUnknown);
impl ITfCompartmentEventSink {
    pub unsafe fn OnChange(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid)).ok()
    }
}
impl ::core::convert::From<ITfCompartmentEventSink> for ::windows::core::IUnknown {
    fn from(value: ITfCompartmentEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCompartmentEventSink> for ::windows::core::IUnknown {
    fn from(value: &ITfCompartmentEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCompartmentEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCompartmentEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCompartmentEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCompartmentEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompartmentEventSink {}
unsafe impl ::windows::core::Interface for ITfCompartmentEventSink {
    type Vtable = ITfCompartmentEventSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x743abd5f_f26d_48df_8cc5_238492419b64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompartmentEventSinkVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ITfCompartmentMgr(::windows::core::IUnknown);
impl ITfCompartmentMgr {
    pub unsafe fn GetCompartment(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfCompartment> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(&mut result__)).from_abi::<ITfCompartment>(result__)
    }
    pub unsafe fn ClearCompartment(&self, tid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), ::core::mem::transmute(rguid)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCompartments(&self) -> ::windows::core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumGUID>(result__)
    }
}
impl ::core::convert::From<ITfCompartmentMgr> for ::windows::core::IUnknown {
    fn from(value: ITfCompartmentMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCompartmentMgr> for ::windows::core::IUnknown {
    fn from(value: &ITfCompartmentMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCompartmentMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCompartmentMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCompartmentMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCompartmentMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompartmentMgr {}
unsafe impl ::windows::core::Interface for ITfCompartmentMgr {
    type Vtable = ITfCompartmentMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7dcf57ac_18ad_438b_824d_979bffb74b7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompartmentMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct ITfComposition(::windows::core::IUnknown);
impl ITfComposition {
    pub unsafe fn GetRange(&self) -> ::windows::core::Result<ITfRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfRange>(result__)
    }
    pub unsafe fn ShiftStart<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ecwrite: u32, pnewstart: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pnewstart.into_param().abi()).ok()
    }
    pub unsafe fn ShiftEnd<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ecwrite: u32, pnewend: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pnewend.into_param().abi()).ok()
    }
    pub unsafe fn EndComposition(&self, ecwrite: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite)).ok()
    }
}
impl ::core::convert::From<ITfComposition> for ::windows::core::IUnknown {
    fn from(value: ITfComposition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfComposition> for ::windows::core::IUnknown {
    fn from(value: &ITfComposition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfComposition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfComposition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfComposition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfComposition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfComposition {}
unsafe impl ::windows::core::Interface for ITfComposition {
    type Vtable = ITfCompositionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20168d64_5a8f_4a5a_b7bd_cfa29f4d0fd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompositionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewstart: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewend: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfCompositionSink(::windows::core::IUnknown);
impl ITfCompositionSink {
    pub unsafe fn OnCompositionTerminated<'a, Param1: ::windows::core::IntoParam<'a, ITfComposition>>(&self, ecwrite: u32, pcomposition: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pcomposition.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITfCompositionSink> for ::windows::core::IUnknown {
    fn from(value: ITfCompositionSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCompositionSink> for ::windows::core::IUnknown {
    fn from(value: &ITfCompositionSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCompositionSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCompositionSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCompositionSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCompositionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompositionSink {}
unsafe impl ::windows::core::Interface for ITfCompositionSink {
    type Vtable = ITfCompositionSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa781718c_579a_4b15_a280_32b8577acc5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompositionSinkVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ITfCompositionView(::windows::core::IUnknown);
impl ITfCompositionView {
    pub unsafe fn GetOwnerClsid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetRange(&self) -> ::windows::core::Result<ITfRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfRange>(result__)
    }
}
impl ::core::convert::From<ITfCompositionView> for ::windows::core::IUnknown {
    fn from(value: ITfCompositionView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCompositionView> for ::windows::core::IUnknown {
    fn from(value: &ITfCompositionView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCompositionView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCompositionView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCompositionView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCompositionView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompositionView {}
unsafe impl ::windows::core::Interface for ITfCompositionView {
    type Vtable = ITfCompositionViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7540241_f9a1_4364_befc_dbcd2c4395b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompositionViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfConfigureSystemKeystrokeFeed(::windows::core::IUnknown);
impl ITfConfigureSystemKeystrokeFeed {
    pub unsafe fn DisableSystemKeystrokeFeed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EnableSystemKeystrokeFeed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITfConfigureSystemKeystrokeFeed> for ::windows::core::IUnknown {
    fn from(value: ITfConfigureSystemKeystrokeFeed) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfConfigureSystemKeystrokeFeed> for ::windows::core::IUnknown {
    fn from(value: &ITfConfigureSystemKeystrokeFeed) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfConfigureSystemKeystrokeFeed {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfConfigureSystemKeystrokeFeed {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfConfigureSystemKeystrokeFeed {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfConfigureSystemKeystrokeFeed {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfConfigureSystemKeystrokeFeed {}
unsafe impl ::windows::core::Interface for ITfConfigureSystemKeystrokeFeed {
    type Vtable = ITfConfigureSystemKeystrokeFeedVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d2c969a_bc9c_437c_84ee_951c49b1a764);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfConfigureSystemKeystrokeFeedVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfContext(::windows::core::IUnknown);
impl ITfContext {
    pub unsafe fn RequestEditSession<'a, Param1: ::windows::core::IntoParam<'a, ITfEditSession>>(&self, tid: u32, pes: Param1, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), pes.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::HRESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InWriteSession(&self, tid: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSelection(&self, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(ulindex), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection), ::core::mem::transmute(pcfetched)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSelection(&self, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(ulcount), ::core::mem::transmute(pselection)).ok()
    }
    pub unsafe fn GetStart(&self, ec: u32) -> ::windows::core::Result<ITfRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(&mut result__)).from_abi::<ITfRange>(result__)
    }
    pub unsafe fn GetEnd(&self, ec: u32) -> ::windows::core::Result<ITfRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(&mut result__)).from_abi::<ITfRange>(result__)
    }
    pub unsafe fn GetActiveView(&self) -> ::windows::core::Result<ITfContextView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfContextView>(result__)
    }
    pub unsafe fn EnumViews(&self) -> ::windows::core::Result<IEnumTfContextViews> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfContextViews>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<TS_STATUS> {
        let mut result__: TS_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TS_STATUS>(result__)
    }
    pub unsafe fn GetProperty(&self, guidprop: *const ::windows::core::GUID) -> ::windows::core::Result<ITfProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprop), ::core::mem::transmute(&mut result__)).from_abi::<ITfProperty>(result__)
    }
    pub unsafe fn GetAppProperty(&self, guidprop: *const ::windows::core::GUID) -> ::windows::core::Result<ITfReadOnlyProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprop), ::core::mem::transmute(&mut result__)).from_abi::<ITfReadOnlyProperty>(result__)
    }
    pub unsafe fn TrackProperties(&self, prgprop: *const *const ::windows::core::GUID, cprop: u32, prgappprop: *const *const ::windows::core::GUID, cappprop: u32) -> ::windows::core::Result<ITfReadOnlyProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(prgprop), ::core::mem::transmute(cprop), ::core::mem::transmute(prgappprop), ::core::mem::transmute(cappprop), ::core::mem::transmute(&mut result__)).from_abi::<ITfReadOnlyProperty>(result__)
    }
    pub unsafe fn EnumProperties(&self) -> ::windows::core::Result<IEnumTfProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfProperties>(result__)
    }
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfDocumentMgr>(result__)
    }
    pub unsafe fn CreateRangeBackup<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1) -> ::windows::core::Result<ITfRangeBackup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfRangeBackup>(result__)
    }
}
impl ::core::convert::From<ITfContext> for ::windows::core::IUnknown {
    fn from(value: ITfContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfContext> for ::windows::core::IUnknown {
    fn from(value: &ITfContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContext {}
unsafe impl ::windows::core::Interface for ITfContext {
    type Vtable = ITfContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e7fd_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, pes: ::windows::core::RawPtr, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, pfwritesession: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ppstart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ppend: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prgprop: *const *const ::windows::core::GUID, cprop: u32, prgappprop: *const *const ::windows::core::GUID, cappprop: u32, ppproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, ppbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfContextComposition(::windows::core::IUnknown);
impl ITfContextComposition {
    pub unsafe fn StartComposition<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>, Param2: ::windows::core::IntoParam<'a, ITfCompositionSink>>(&self, ecwrite: u32, pcompositionrange: Param1, psink: Param2) -> ::windows::core::Result<ITfComposition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pcompositionrange.into_param().abi(), psink.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfComposition>(result__)
    }
    pub unsafe fn EnumCompositions(&self) -> ::windows::core::Result<IEnumITfCompositionView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumITfCompositionView>(result__)
    }
    pub unsafe fn FindComposition<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ecread: u32, ptestrange: Param1) -> ::windows::core::Result<IEnumITfCompositionView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecread), ptestrange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IEnumITfCompositionView>(result__)
    }
    pub unsafe fn TakeOwnership<'a, Param1: ::windows::core::IntoParam<'a, ITfCompositionView>, Param2: ::windows::core::IntoParam<'a, ITfCompositionSink>>(&self, ecwrite: u32, pcomposition: Param1, psink: Param2) -> ::windows::core::Result<ITfComposition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pcomposition.into_param().abi(), psink.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfComposition>(result__)
    }
}
impl ::core::convert::From<ITfContextComposition> for ::windows::core::IUnknown {
    fn from(value: ITfContextComposition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfContextComposition> for ::windows::core::IUnknown {
    fn from(value: &ITfContextComposition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfContextComposition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfContextComposition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfContextComposition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfContextComposition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextComposition {}
unsafe impl ::windows::core::Interface for ITfContextComposition {
    type Vtable = ITfContextCompositionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd40c8aae_ac92_4fc7_9a11_0ee0e23aa39b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextCompositionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pcompositionrange: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, ppcomposition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecread: u32, ptestrange: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, ppcomposition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfContextKeyEventSink(::windows::core::IUnknown);
impl ITfContextKeyEventSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTestKeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTestKeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ITfContextKeyEventSink> for ::windows::core::IUnknown {
    fn from(value: ITfContextKeyEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfContextKeyEventSink> for ::windows::core::IUnknown {
    fn from(value: &ITfContextKeyEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfContextKeyEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfContextKeyEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfContextKeyEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfContextKeyEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextKeyEventSink {}
unsafe impl ::windows::core::Interface for ITfContextKeyEventSink {
    type Vtable = ITfContextKeyEventSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0552ba5d_c835_4934_bf50_846aaa67432f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextKeyEventSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfContextOwner(::windows::core::IUnknown);
impl ITfContextOwner {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetACPFromPoint(&self, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptscreen), ::core::mem::transmute(dwflags), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextExt(&self, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(prc), ::core::mem::transmute(pfclipped)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScreenExt(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<TS_STATUS> {
        let mut result__: TS_STATUS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TS_STATUS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttribute(&self, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidattribute), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ITfContextOwner> for ::windows::core::IUnknown {
    fn from(value: ITfContextOwner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfContextOwner> for ::windows::core::IUnknown {
    fn from(value: &ITfContextOwner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfContextOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfContextOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfContextOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfContextOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextOwner {}
unsafe impl ::windows::core::Interface for ITfContextOwner {
    type Vtable = ITfContextOwnerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e80c_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct ITfContextOwnerCompositionServices(::windows::core::IUnknown);
impl ITfContextOwnerCompositionServices {
    pub unsafe fn StartComposition<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>, Param2: ::windows::core::IntoParam<'a, ITfCompositionSink>>(&self, ecwrite: u32, pcompositionrange: Param1, psink: Param2) -> ::windows::core::Result<ITfComposition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pcompositionrange.into_param().abi(), psink.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfComposition>(result__)
    }
    pub unsafe fn EnumCompositions(&self) -> ::windows::core::Result<IEnumITfCompositionView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumITfCompositionView>(result__)
    }
    pub unsafe fn FindComposition<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ecread: u32, ptestrange: Param1) -> ::windows::core::Result<IEnumITfCompositionView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecread), ptestrange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IEnumITfCompositionView>(result__)
    }
    pub unsafe fn TakeOwnership<'a, Param1: ::windows::core::IntoParam<'a, ITfCompositionView>, Param2: ::windows::core::IntoParam<'a, ITfCompositionSink>>(&self, ecwrite: u32, pcomposition: Param1, psink: Param2) -> ::windows::core::Result<ITfComposition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ecwrite), pcomposition.into_param().abi(), psink.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfComposition>(result__)
    }
    pub unsafe fn TerminateComposition<'a, Param0: ::windows::core::IntoParam<'a, ITfCompositionView>>(&self, pcomposition: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pcomposition.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfContextComposition> for ITfContextOwnerCompositionServices {
    fn into_param(self) -> ::windows::core::Param<'a, ITfContextComposition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfContextComposition> for &ITfContextOwnerCompositionServices {
    fn into_param(self) -> ::windows::core::Param<'a, ITfContextComposition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfContextOwnerCompositionServices> for ::windows::core::IUnknown {
    fn from(value: ITfContextOwnerCompositionServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfContextOwnerCompositionServices> for ::windows::core::IUnknown {
    fn from(value: &ITfContextOwnerCompositionServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfContextOwnerCompositionServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfContextOwnerCompositionServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfContextOwnerCompositionServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfContextOwnerCompositionServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextOwnerCompositionServices {}
unsafe impl ::windows::core::Interface for ITfContextOwnerCompositionServices {
    type Vtable = ITfContextOwnerCompositionServicesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86462810_593b_4916_9764_19c08e9ce110);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerCompositionServicesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pcompositionrange: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, ppcomposition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecread: u32, ptestrange: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, ppcomposition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfContextOwnerCompositionSink(::windows::core::IUnknown);
impl ITfContextOwnerCompositionSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnStartComposition<'a, Param0: ::windows::core::IntoParam<'a, ITfCompositionView>>(&self, pcomposition: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcomposition.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn OnUpdateComposition<'a, Param0: ::windows::core::IntoParam<'a, ITfCompositionView>, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, pcomposition: Param0, prangenew: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcomposition.into_param().abi(), prangenew.into_param().abi()).ok()
    }
    pub unsafe fn OnEndComposition<'a, Param0: ::windows::core::IntoParam<'a, ITfCompositionView>>(&self, pcomposition: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pcomposition.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITfContextOwnerCompositionSink> for ::windows::core::IUnknown {
    fn from(value: ITfContextOwnerCompositionSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfContextOwnerCompositionSink> for ::windows::core::IUnknown {
    fn from(value: &ITfContextOwnerCompositionSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfContextOwnerCompositionSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfContextOwnerCompositionSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfContextOwnerCompositionSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfContextOwnerCompositionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextOwnerCompositionSink {}
unsafe impl ::windows::core::Interface for ITfContextOwnerCompositionSink {
    type Vtable = ITfContextOwnerCompositionSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f20aa40_b57a_4f34_96ab_3576f377cc79);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerCompositionSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr, pfok: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr, prangenew: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomposition: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfContextOwnerServices(::windows::core::IUnknown);
impl ITfContextOwnerServices {
    pub unsafe fn OnLayoutChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn OnAttributeChange(&self, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidattribute)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<'a, Param0: ::windows::core::IntoParam<'a, ITfProperty>, Param1: ::windows::core::IntoParam<'a, ITfRange>, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pprop: Param0, prange: Param1, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pprop.into_param().abi(), prange.into_param().abi(), ::core::mem::transmute(phdr), pstream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Unserialize<'a, Param0: ::windows::core::IntoParam<'a, ITfProperty>, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param3: ::windows::core::IntoParam<'a, ITfPersistentPropertyLoaderACP>>(&self, pprop: Param0, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: Param2, ploader: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pprop.into_param().abi(), ::core::mem::transmute(phdr), pstream.into_param().abi(), ploader.into_param().abi()).ok()
    }
    pub unsafe fn ForceLoadProperty<'a, Param0: ::windows::core::IntoParam<'a, ITfProperty>>(&self, pprop: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pprop.into_param().abi()).ok()
    }
    pub unsafe fn CreateRange(&self, acpstart: i32, acpend: i32) -> ::windows::core::Result<ITfRangeACP> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpstart), ::core::mem::transmute(acpend), ::core::mem::transmute(&mut result__)).from_abi::<ITfRangeACP>(result__)
    }
}
impl ::core::convert::From<ITfContextOwnerServices> for ::windows::core::IUnknown {
    fn from(value: ITfContextOwnerServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfContextOwnerServices> for ::windows::core::IUnknown {
    fn from(value: &ITfContextOwnerServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfContextOwnerServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfContextOwnerServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfContextOwnerServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfContextOwnerServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextOwnerServices {}
unsafe impl ::windows::core::Interface for ITfContextOwnerServices {
    type Vtable = ITfContextOwnerServicesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb23eb630_3e1c_11d3_a745_0050040ab407);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerServicesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, prange: ::windows::core::RawPtr, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::windows::core::RawPtr, ploader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprop: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfContextView(::windows::core::IUnknown);
impl ITfContextView {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRangeFromPoint(&self, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<ITfRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(ppt), ::core::mem::transmute(dwflags), ::core::mem::transmute(&mut result__)).from_abi::<ITfRange>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextExt<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(prc), ::core::mem::transmute(pfclipped)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScreenExt(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
}
impl ::core::convert::From<ITfContextView> for ::windows::core::IUnknown {
    fn from(value: ITfContextView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfContextView> for ::windows::core::IUnknown {
    fn from(value: &ITfContextView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfContextView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfContextView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfContextView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfContextView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextView {}
unsafe impl ::windows::core::Interface for ITfContextView {
    type Vtable = ITfContextViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2433bf8e_0f9b_435c_ba2c_180611978c30);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfCreatePropertyStore(::windows::core::IUnknown);
impl ITfCreatePropertyStore {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStoreSerializable<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>, Param2: ::windows::core::IntoParam<'a, ITfPropertyStore>>(&self, guidprop: *const ::windows::core::GUID, prange: Param1, ppropstore: Param2) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprop), prange.into_param().abi(), ppropstore.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyStore<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, guidprop: *const ::windows::core::GUID, prange: Param1, cb: u32, pstream: Param3) -> ::windows::core::Result<ITfPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprop), prange.into_param().abi(), ::core::mem::transmute(cb), pstream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfPropertyStore>(result__)
    }
}
impl ::core::convert::From<ITfCreatePropertyStore> for ::windows::core::IUnknown {
    fn from(value: ITfCreatePropertyStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfCreatePropertyStore> for ::windows::core::IUnknown {
    fn from(value: &ITfCreatePropertyStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfCreatePropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfCreatePropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfCreatePropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfCreatePropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCreatePropertyStore {}
unsafe impl ::windows::core::Interface for ITfCreatePropertyStore {
    type Vtable = ITfCreatePropertyStoreVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2463fbf0_b0af_11d2_afc5_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCreatePropertyStoreVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, prange: ::windows::core::RawPtr, ppropstore: ::windows::core::RawPtr, pfserializable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, prange: ::windows::core::RawPtr, cb: u32, pstream: ::windows::core::RawPtr, ppstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct ITfDisplayAttributeInfo(::windows::core::IUnknown);
impl ITfDisplayAttributeInfo {
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeInfo(&self) -> ::windows::core::Result<TF_DISPLAYATTRIBUTE> {
        let mut result__: TF_DISPLAYATTRIBUTE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TF_DISPLAYATTRIBUTE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttributeInfo(&self, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pda)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITfDisplayAttributeInfo> for ::windows::core::IUnknown {
    fn from(value: ITfDisplayAttributeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfDisplayAttributeInfo> for ::windows::core::IUnknown {
    fn from(value: &ITfDisplayAttributeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfDisplayAttributeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfDisplayAttributeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfDisplayAttributeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfDisplayAttributeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDisplayAttributeInfo {}
unsafe impl ::windows::core::Interface for ITfDisplayAttributeInfo {
    type Vtable = ITfDisplayAttributeInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70528852_2f26_4aea_8c96_215150578932);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *mut TF_DISPLAYATTRIBUTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfDisplayAttributeMgr(::windows::core::IUnknown);
impl ITfDisplayAttributeMgr {
    pub unsafe fn OnUpdateInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EnumDisplayAttributeInfo(&self) -> ::windows::core::Result<IEnumTfDisplayAttributeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfDisplayAttributeInfo>(result__)
    }
    pub unsafe fn GetDisplayAttributeInfo(&self, guid: *const ::windows::core::GUID, ppinfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pclsidowner: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(ppinfo), ::core::mem::transmute(pclsidowner)).ok()
    }
}
impl ::core::convert::From<ITfDisplayAttributeMgr> for ::windows::core::IUnknown {
    fn from(value: ITfDisplayAttributeMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfDisplayAttributeMgr> for ::windows::core::IUnknown {
    fn from(value: &ITfDisplayAttributeMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfDisplayAttributeMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfDisplayAttributeMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfDisplayAttributeMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfDisplayAttributeMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDisplayAttributeMgr {}
unsafe impl ::windows::core::Interface for ITfDisplayAttributeMgr {
    type Vtable = ITfDisplayAttributeMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ded7393_5db1_475c_9e71_a39111b0ff67);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, ppinfo: *mut ::windows::core::RawPtr, pclsidowner: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfDisplayAttributeNotifySink(::windows::core::IUnknown);
impl ITfDisplayAttributeNotifySink {
    pub unsafe fn OnUpdateInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITfDisplayAttributeNotifySink> for ::windows::core::IUnknown {
    fn from(value: ITfDisplayAttributeNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfDisplayAttributeNotifySink> for ::windows::core::IUnknown {
    fn from(value: &ITfDisplayAttributeNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfDisplayAttributeNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfDisplayAttributeNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfDisplayAttributeNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfDisplayAttributeNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDisplayAttributeNotifySink {}
unsafe impl ::windows::core::Interface for ITfDisplayAttributeNotifySink {
    type Vtable = ITfDisplayAttributeNotifySinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad56f402_e162_4f25_908f_7d577cf9bda9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeNotifySinkVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ITfDisplayAttributeProvider(::windows::core::IUnknown);
impl ITfDisplayAttributeProvider {
    pub unsafe fn EnumDisplayAttributeInfo(&self) -> ::windows::core::Result<IEnumTfDisplayAttributeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfDisplayAttributeInfo>(result__)
    }
    pub unsafe fn GetDisplayAttributeInfo(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfDisplayAttributeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<ITfDisplayAttributeInfo>(result__)
    }
}
impl ::core::convert::From<ITfDisplayAttributeProvider> for ::windows::core::IUnknown {
    fn from(value: ITfDisplayAttributeProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfDisplayAttributeProvider> for ::windows::core::IUnknown {
    fn from(value: &ITfDisplayAttributeProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfDisplayAttributeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfDisplayAttributeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfDisplayAttributeProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfDisplayAttributeProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDisplayAttributeProvider {}
unsafe impl ::windows::core::Interface for ITfDisplayAttributeProvider {
    type Vtable = ITfDisplayAttributeProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfee47777_163c_4769_996a_6e9c50ad8f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfDocumentMgr(::windows::core::IUnknown);
impl ITfDocumentMgr {
    pub unsafe fn CreateContext<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, tidowner: u32, dwflags: u32, punk: Param2, ppic: *mut ::core::option::Option<ITfContext>, pectextstore: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tidowner), ::core::mem::transmute(dwflags), punk.into_param().abi(), ::core::mem::transmute(ppic), ::core::mem::transmute(pectextstore)).ok()
    }
    pub unsafe fn Push<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>>(&self, pic: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pic.into_param().abi()).ok()
    }
    pub unsafe fn Pop(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetTop(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfContext>(result__)
    }
    pub unsafe fn GetBase(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfContext>(result__)
    }
    pub unsafe fn EnumContexts(&self) -> ::windows::core::Result<IEnumTfContexts> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfContexts>(result__)
    }
}
impl ::core::convert::From<ITfDocumentMgr> for ::windows::core::IUnknown {
    fn from(value: ITfDocumentMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfDocumentMgr> for ::windows::core::IUnknown {
    fn from(value: &ITfDocumentMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfDocumentMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfDocumentMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfDocumentMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfDocumentMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDocumentMgr {}
unsafe impl ::windows::core::Interface for ITfDocumentMgr {
    type Vtable = ITfDocumentMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e7f4_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDocumentMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tidowner: u32, dwflags: u32, punk: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr, pectextstore: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfEditRecord(::windows::core::IUnknown);
impl ITfEditRecord {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSelectionStatus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetTextAndPropertyUpdates(&self, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows::core::GUID, cproperties: u32) -> ::windows::core::Result<IEnumTfRanges> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(prgproperties), ::core::mem::transmute(cproperties), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfRanges>(result__)
    }
}
impl ::core::convert::From<ITfEditRecord> for ::windows::core::IUnknown {
    fn from(value: ITfEditRecord) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfEditRecord> for ::windows::core::IUnknown {
    fn from(value: &ITfEditRecord) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfEditRecord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfEditRecord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfEditRecord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfEditRecord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfEditRecord {}
unsafe impl ::windows::core::Interface for ITfEditRecord {
    type Vtable = ITfEditRecordVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42d4d099_7c1a_4a89_b836_6c6f22160df0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfEditRecordVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows::core::GUID, cproperties: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfEditSession(::windows::core::IUnknown);
impl ITfEditSession {
    pub unsafe fn DoEditSession(&self, ec: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec)).ok()
    }
}
impl ::core::convert::From<ITfEditSession> for ::windows::core::IUnknown {
    fn from(value: ITfEditSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfEditSession> for ::windows::core::IUnknown {
    fn from(value: &ITfEditSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfEditSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfEditSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfEditSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfEditSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfEditSession {}
unsafe impl ::windows::core::Interface for ITfEditSession {
    type Vtable = ITfEditSessionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e803_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfEditSessionVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ITfEditTransactionSink(::windows::core::IUnknown);
impl ITfEditTransactionSink {
    pub unsafe fn OnStartEditTransaction<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>>(&self, pic: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pic.into_param().abi()).ok()
    }
    pub unsafe fn OnEndEditTransaction<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>>(&self, pic: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pic.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITfEditTransactionSink> for ::windows::core::IUnknown {
    fn from(value: ITfEditTransactionSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfEditTransactionSink> for ::windows::core::IUnknown {
    fn from(value: &ITfEditTransactionSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfEditTransactionSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfEditTransactionSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfEditTransactionSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfEditTransactionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfEditTransactionSink {}
unsafe impl ::windows::core::Interface for ITfEditTransactionSink {
    type Vtable = ITfEditTransactionSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x708fbf70_b520_416b_b06c_2c41ab44f8ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfEditTransactionSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfFnAdviseText(::windows::core::IUnknown);
impl ITfFnAdviseText {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTextUpdate<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, prange: Param0, pchtext: Param1, cch: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), prange.into_param().abi(), pchtext.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
    pub unsafe fn OnLatticeUpdate<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>, Param1: ::windows::core::IntoParam<'a, ITfLMLattice>>(&self, prange: Param0, plattice: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), prange.into_param().abi(), plattice.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnAdviseText {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnAdviseText {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnAdviseText> for ::windows::core::IUnknown {
    fn from(value: ITfFnAdviseText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnAdviseText> for ::windows::core::IUnknown {
    fn from(value: &ITfFnAdviseText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnAdviseText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnAdviseText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnAdviseText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnAdviseText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnAdviseText {}
unsafe impl ::windows::core::Interface for ITfFnAdviseText {
    type Vtable = ITfFnAdviseTextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3527268b_7d53_4dd9_92b7_7296ae461249);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnAdviseTextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, plattice: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfFnBalloon(::windows::core::IUnknown);
impl ITfFnBalloon {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateBalloon<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, style: TfLBBalloonStyle, pch: Param1, cch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(style), pch.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
}
impl ::core::convert::From<ITfFnBalloon> for ::windows::core::IUnknown {
    fn from(value: ITfFnBalloon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnBalloon> for ::windows::core::IUnknown {
    fn from(value: &ITfFnBalloon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnBalloon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnBalloon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnBalloon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnBalloon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnBalloon {}
unsafe impl ::windows::core::Interface for ITfFnBalloon {
    type Vtable = ITfFnBalloonVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bab89e4_5fbe_45f4_a5bc_dca36ad225a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnBalloonVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfFnConfigure(::windows::core::IUnknown);
impl ITfFnConfigure {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, langid: u16, rguidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(langid), ::core::mem::transmute(rguidprofile)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnConfigure {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnConfigure {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnConfigure> for ::windows::core::IUnknown {
    fn from(value: ITfFnConfigure) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnConfigure> for ::windows::core::IUnknown {
    fn from(value: &ITfFnConfigure) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnConfigure {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnConfigure {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnConfigure {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnConfigure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnConfigure {}
unsafe impl ::windows::core::Interface for ITfFnConfigure {
    type Vtable = ITfFnConfigureVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88f567c6_1757_49f8_a1b2_89234c1eeff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnConfigureVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfFnConfigureRegisterEudc(::windows::core::IUnknown);
impl ITfFnConfigureRegisterEudc {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, hwndparent: Param0, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(langid), ::core::mem::transmute(rguidprofile), bstrregistered.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnConfigureRegisterEudc {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnConfigureRegisterEudc {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnConfigureRegisterEudc> for ::windows::core::IUnknown {
    fn from(value: ITfFnConfigureRegisterEudc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnConfigureRegisterEudc> for ::windows::core::IUnknown {
    fn from(value: &ITfFnConfigureRegisterEudc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnConfigureRegisterEudc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnConfigureRegisterEudc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnConfigureRegisterEudc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnConfigureRegisterEudc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnConfigureRegisterEudc {}
unsafe impl ::windows::core::Interface for ITfFnConfigureRegisterEudc {
    type Vtable = ITfFnConfigureRegisterEudcVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5e26ff5_d7ad_4304_913f_21a2ed95a1b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnConfigureRegisterEudcVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfFnConfigureRegisterWord(::windows::core::IUnknown);
impl ITfFnConfigureRegisterWord {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, hwndparent: Param0, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(langid), ::core::mem::transmute(rguidprofile), bstrregistered.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnConfigureRegisterWord {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnConfigureRegisterWord {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnConfigureRegisterWord> for ::windows::core::IUnknown {
    fn from(value: ITfFnConfigureRegisterWord) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnConfigureRegisterWord> for ::windows::core::IUnknown {
    fn from(value: &ITfFnConfigureRegisterWord) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnConfigureRegisterWord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnConfigureRegisterWord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnConfigureRegisterWord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnConfigureRegisterWord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnConfigureRegisterWord {}
unsafe impl ::windows::core::Interface for ITfFnConfigureRegisterWord {
    type Vtable = ITfFnConfigureRegisterWordVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb95808a_6d8f_4bca_8400_5390b586aedf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnConfigureRegisterWordVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfFnCustomSpeechCommand(::windows::core::IUnknown);
impl ITfFnCustomSpeechCommand {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetSpeechCommandProvider<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pspcmdprovider: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pspcmdprovider.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnCustomSpeechCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnCustomSpeechCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnCustomSpeechCommand> for ::windows::core::IUnknown {
    fn from(value: ITfFnCustomSpeechCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnCustomSpeechCommand> for ::windows::core::IUnknown {
    fn from(value: &ITfFnCustomSpeechCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnCustomSpeechCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnCustomSpeechCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnCustomSpeechCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnCustomSpeechCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnCustomSpeechCommand {}
unsafe impl ::windows::core::Interface for ITfFnCustomSpeechCommand {
    type Vtable = ITfFnCustomSpeechCommandVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfca6c349_a12f_43a3_8dd6_5a5a4282577b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnCustomSpeechCommandVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pspcmdprovider: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfFnGetLinguisticAlternates(::windows::core::IUnknown);
impl ITfFnGetLinguisticAlternates {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetAlternates<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::core::Result<ITfCandidateList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfCandidateList>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnGetLinguisticAlternates {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnGetLinguisticAlternates {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnGetLinguisticAlternates> for ::windows::core::IUnknown {
    fn from(value: ITfFnGetLinguisticAlternates) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnGetLinguisticAlternates> for ::windows::core::IUnknown {
    fn from(value: &ITfFnGetLinguisticAlternates) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnGetLinguisticAlternates {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnGetLinguisticAlternates {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnGetLinguisticAlternates {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnGetLinguisticAlternates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnGetLinguisticAlternates {}
unsafe impl ::windows::core::Interface for ITfFnGetLinguisticAlternates {
    type Vtable = ITfFnGetLinguisticAlternatesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea163ce2_7a65_4506_82a3_c528215da64e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnGetLinguisticAlternatesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandidatelist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfFnGetPreferredTouchKeyboardLayout(::windows::core::IUnknown);
impl ITfFnGetPreferredTouchKeyboardLayout {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetLayout(&self, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptkblayouttype), ::core::mem::transmute(pwpreferredlayoutid)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnGetPreferredTouchKeyboardLayout {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnGetPreferredTouchKeyboardLayout {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnGetPreferredTouchKeyboardLayout> for ::windows::core::IUnknown {
    fn from(value: ITfFnGetPreferredTouchKeyboardLayout) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnGetPreferredTouchKeyboardLayout> for ::windows::core::IUnknown {
    fn from(value: &ITfFnGetPreferredTouchKeyboardLayout) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnGetPreferredTouchKeyboardLayout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnGetPreferredTouchKeyboardLayout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnGetPreferredTouchKeyboardLayout {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnGetPreferredTouchKeyboardLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnGetPreferredTouchKeyboardLayout {}
unsafe impl ::windows::core::Interface for ITfFnGetPreferredTouchKeyboardLayout {
    type Vtable = ITfFnGetPreferredTouchKeyboardLayoutVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f309a41_590a_4acc_a97f_d8efff13fdfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnGetPreferredTouchKeyboardLayoutVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfFnGetSAPIObject(::windows::core::IUnknown);
impl ITfFnGetSAPIObject {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Get(&self, sobj: TfSapiObject) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(sobj), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnGetSAPIObject {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnGetSAPIObject {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnGetSAPIObject> for ::windows::core::IUnknown {
    fn from(value: ITfFnGetSAPIObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnGetSAPIObject> for ::windows::core::IUnknown {
    fn from(value: &ITfFnGetSAPIObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnGetSAPIObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnGetSAPIObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnGetSAPIObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnGetSAPIObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnGetSAPIObject {}
unsafe impl ::windows::core::Interface for ITfFnGetSAPIObject {
    type Vtable = ITfFnGetSAPIObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c0ab7ea_167d_4f59_bfb5_4693755e90ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnGetSAPIObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sobj: TfSapiObject, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfFnLMInternal(::windows::core::IUnknown);
impl ITfFnLMInternal {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryRange<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(ppnewrange), ::core::mem::transmute(pfaccepted)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryLangID(&self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetReconversion<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::core::Result<ITfCandidateList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfCandidateList>(result__)
    }
    pub unsafe fn Reconvert<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, fup: Param0, vkey: Param1, lparamkeydata: Param2) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), fup.into_param().abi(), vkey.into_param().abi(), lparamkeydata.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, fup: Param0, vkey: Param1, lparamkeydata: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), fup.into_param().abi(), vkey.into_param().abi(), lparamkeydata.into_param().abi()).ok()
    }
    pub unsafe fn InvokeFunc<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>>(&self, pic: Param0, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(refguidfunc)).ok()
    }
    pub unsafe fn ProcessLattice<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFnLMProcessor> for ITfFnLMInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFnLMProcessor> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFnLMProcessor> for &ITfFnLMInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFnLMProcessor> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnLMInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnLMInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnLMInternal> for ::windows::core::IUnknown {
    fn from(value: ITfFnLMInternal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnLMInternal> for ::windows::core::IUnknown {
    fn from(value: &ITfFnLMInternal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnLMInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnLMInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnLMInternal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnLMInternal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnLMInternal {}
unsafe impl ::windows::core::Interface for ITfFnLMInternal {
    type Vtable = ITfFnLMInternalVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b825b1_ac9a_4f7b_b5ad_c7168f1ee445);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnLMInternalVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfFnLMProcessor(::windows::core::IUnknown);
impl ITfFnLMProcessor {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryRange<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(ppnewrange), ::core::mem::transmute(pfaccepted)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryLangID(&self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetReconversion<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::core::Result<ITfCandidateList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfCandidateList>(result__)
    }
    pub unsafe fn Reconvert<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, fup: Param0, vkey: Param1, lparamkeydata: Param2) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), fup.into_param().abi(), vkey.into_param().abi(), lparamkeydata.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, fup: Param0, vkey: Param1, lparamkeydata: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), fup.into_param().abi(), vkey.into_param().abi(), lparamkeydata.into_param().abi()).ok()
    }
    pub unsafe fn InvokeFunc<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>>(&self, pic: Param0, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(refguidfunc)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnLMProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnLMProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnLMProcessor> for ::windows::core::IUnknown {
    fn from(value: ITfFnLMProcessor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnLMProcessor> for ::windows::core::IUnknown {
    fn from(value: &ITfFnLMProcessor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnLMProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnLMProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnLMProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnLMProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnLMProcessor {}
unsafe impl ::windows::core::Interface for ITfFnLMProcessor {
    type Vtable = ITfFnLMProcessorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7afbf8e7_ac4b_4082_b058_890899d3a010);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnLMProcessorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfFnLangProfileUtil(::windows::core::IUnknown);
impl ITfFnLangProfileUtil {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RegisterActiveProfiles(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsProfileAvailableForLang(&self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnLangProfileUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnLangProfileUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnLangProfileUtil> for ::windows::core::IUnknown {
    fn from(value: ITfFnLangProfileUtil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnLangProfileUtil> for ::windows::core::IUnknown {
    fn from(value: &ITfFnLangProfileUtil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnLangProfileUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnLangProfileUtil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnLangProfileUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnLangProfileUtil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnLangProfileUtil {}
unsafe impl ::windows::core::Interface for ITfFnLangProfileUtil {
    type Vtable = ITfFnLangProfileUtilVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa87a8574_a6c1_4e15_99f0_3d3965f548eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnLangProfileUtilVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, pfavailable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfFnPlayBack(::windows::core::IUnknown);
impl ITfFnPlayBack {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryRange<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(ppnewrange), ::core::mem::transmute(pfplayable)).ok()
    }
    pub unsafe fn Play<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnPlayBack {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnPlayBack {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnPlayBack> for ::windows::core::IUnknown {
    fn from(value: ITfFnPlayBack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnPlayBack> for ::windows::core::IUnknown {
    fn from(value: &ITfFnPlayBack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnPlayBack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnPlayBack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnPlayBack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnPlayBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnPlayBack {}
unsafe impl ::windows::core::Interface for ITfFnPlayBack {
    type Vtable = ITfFnPlayBackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3a416a4_0f64_11d3_b5b7_00c04fc324a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnPlayBackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfFnPropertyUIStatus(::windows::core::IUnknown);
impl ITfFnPropertyUIStatus {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetStatus(&self, refguidprop: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguidprop), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetStatus(&self, refguidprop: *const ::windows::core::GUID, dw: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(refguidprop), ::core::mem::transmute(dw)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnPropertyUIStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnPropertyUIStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnPropertyUIStatus> for ::windows::core::IUnknown {
    fn from(value: ITfFnPropertyUIStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnPropertyUIStatus> for ::windows::core::IUnknown {
    fn from(value: &ITfFnPropertyUIStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnPropertyUIStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnPropertyUIStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnPropertyUIStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnPropertyUIStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnPropertyUIStatus {}
unsafe impl ::windows::core::Interface for ITfFnPropertyUIStatus {
    type Vtable = ITfFnPropertyUIStatusVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2338ac6e_2b9d_44c0_a75e_ee64f256b3bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnPropertyUIStatusVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows::core::GUID, pdw: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows::core::GUID, dw: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfFnReconversion(::windows::core::IUnknown);
impl ITfFnReconversion {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryRange<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(ppnewrange), ::core::mem::transmute(pfconvertable)).ok()
    }
    pub unsafe fn GetReconversion<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::core::Result<ITfCandidateList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfCandidateList>(result__)
    }
    pub unsafe fn Reconvert<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prange: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), prange.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnReconversion {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnReconversion {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnReconversion> for ::windows::core::IUnknown {
    fn from(value: ITfFnReconversion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnReconversion> for ::windows::core::IUnknown {
    fn from(value: &ITfFnReconversion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnReconversion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnReconversion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnReconversion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnReconversion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnReconversion {}
unsafe impl ::windows::core::Interface for ITfFnReconversion {
    type Vtable = ITfFnReconversionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cea93c0_0a58_11d3_8df0_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnReconversionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppnewrange: *mut ::windows::core::RawPtr, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, ppcandlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfFnSearchCandidateProvider(::windows::core::IUnknown);
impl ITfFnSearchCandidateProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSearchCandidates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrquery: Param0, bstrapplicationid: Param1) -> ::windows::core::Result<ITfCandidateList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrquery.into_param().abi(), bstrapplicationid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfCandidateList>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetResult<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrquery: Param0, bstrapplicationid: Param1, bstrresult: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrquery.into_param().abi(), bstrapplicationid.into_param().abi(), bstrresult.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnSearchCandidateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnSearchCandidateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnSearchCandidateProvider> for ::windows::core::IUnknown {
    fn from(value: ITfFnSearchCandidateProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnSearchCandidateProvider> for ::windows::core::IUnknown {
    fn from(value: &ITfFnSearchCandidateProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnSearchCandidateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnSearchCandidateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnSearchCandidateProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnSearchCandidateProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnSearchCandidateProvider {}
unsafe impl ::windows::core::Interface for ITfFnSearchCandidateProvider {
    type Vtable = ITfFnSearchCandidateProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87a2ad8f_f27b_4920_8501_67602280175d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnSearchCandidateProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrresult: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfFnShowHelp(::windows::core::IUnknown);
impl ITfFnShowHelp {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwndparent.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for ITfFnShowHelp {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfFunction> for &ITfFnShowHelp {
    fn into_param(self) -> ::windows::core::Param<'a, ITfFunction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfFnShowHelp> for ::windows::core::IUnknown {
    fn from(value: ITfFnShowHelp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFnShowHelp> for ::windows::core::IUnknown {
    fn from(value: &ITfFnShowHelp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFnShowHelp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFnShowHelp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFnShowHelp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFnShowHelp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnShowHelp {}
unsafe impl ::windows::core::Interface for ITfFnShowHelp {
    type Vtable = ITfFnShowHelpVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ab1d30c_094d_4c29_8ea5_0bf59be87bf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnShowHelpVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfFunction(::windows::core::IUnknown);
impl ITfFunction {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITfFunction> for ::windows::core::IUnknown {
    fn from(value: ITfFunction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFunction> for ::windows::core::IUnknown {
    fn from(value: &ITfFunction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFunction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFunction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFunction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFunction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFunction {}
unsafe impl ::windows::core::Interface for ITfFunction {
    type Vtable = ITfFunctionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb593490_098f_11d3_8df0_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFunctionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfFunctionProvider(::windows::core::IUnknown);
impl ITfFunctionProvider {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetFunction(&self, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ITfFunctionProvider> for ::windows::core::IUnknown {
    fn from(value: ITfFunctionProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfFunctionProvider> for ::windows::core::IUnknown {
    fn from(value: &ITfFunctionProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfFunctionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfFunctionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfFunctionProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfFunctionProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFunctionProvider {}
unsafe impl ::windows::core::Interface for ITfFunctionProvider {
    type Vtable = ITfFunctionProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x101d6610_0990_11d3_8df0_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFunctionProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfInputProcessorProfileActivationSink(::windows::core::IUnknown);
impl ITfInputProcessorProfileActivationSink {
    pub unsafe fn OnActivated<'a, Param5: ::windows::core::IntoParam<'a, HKL>>(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, catid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: Param5, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprofiletype), ::core::mem::transmute(langid), ::core::mem::transmute(clsid), ::core::mem::transmute(catid), ::core::mem::transmute(guidprofile), hkl.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<ITfInputProcessorProfileActivationSink> for ::windows::core::IUnknown {
    fn from(value: ITfInputProcessorProfileActivationSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfInputProcessorProfileActivationSink> for ::windows::core::IUnknown {
    fn from(value: &ITfInputProcessorProfileActivationSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfInputProcessorProfileActivationSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfInputProcessorProfileActivationSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfInputProcessorProfileActivationSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfInputProcessorProfileActivationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfileActivationSink {}
unsafe impl ::windows::core::Interface for ITfInputProcessorProfileActivationSink {
    type Vtable = ITfInputProcessorProfileActivationSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71c6e74e_0f28_11d8_a82a_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfileActivationSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, catid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfInputProcessorProfileMgr(::windows::core::IUnknown);
impl ITfInputProcessorProfileMgr {
    pub unsafe fn ActivateProfile<'a, Param4: ::windows::core::IntoParam<'a, HKL>>(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: Param4, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprofiletype), ::core::mem::transmute(langid), ::core::mem::transmute(clsid), ::core::mem::transmute(guidprofile), hkl.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn DeactivateProfile<'a, Param4: ::windows::core::IntoParam<'a, HKL>>(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: Param4, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprofiletype), ::core::mem::transmute(langid), ::core::mem::transmute(clsid), ::core::mem::transmute(guidprofile), hkl.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetProfile<'a, Param4: ::windows::core::IntoParam<'a, HKL>>(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: Param4) -> ::windows::core::Result<TF_INPUTPROCESSORPROFILE> {
        let mut result__: TF_INPUTPROCESSORPROFILE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprofiletype), ::core::mem::transmute(langid), ::core::mem::transmute(clsid), ::core::mem::transmute(guidprofile), hkl.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<TF_INPUTPROCESSORPROFILE>(result__)
    }
    pub unsafe fn EnumProfiles(&self, langid: u16) -> ::windows::core::Result<IEnumTfInputProcessorProfiles> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfInputProcessorProfiles>(result__)
    }
    pub unsafe fn ReleaseInputProcessor(&self, rclsid: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterProfile<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param8: ::windows::core::IntoParam<'a, HKL>, Param10: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: Param3, cchdesc: u32, pchiconfile: Param5, cchfile: u32, uiconindex: u32, hklsubstitute: Param8, dwpreferredlayout: u32, benabledbydefault: Param10, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), pchdesc.into_param().abi(), ::core::mem::transmute(cchdesc), pchiconfile.into_param().abi(), ::core::mem::transmute(cchfile), ::core::mem::transmute(uiconindex), hklsubstitute.into_param().abi(), ::core::mem::transmute(dwpreferredlayout), benabledbydefault.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn UnregisterProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetActiveProfile(&self, catid: *const ::windows::core::GUID) -> ::windows::core::Result<TF_INPUTPROCESSORPROFILE> {
        let mut result__: TF_INPUTPROCESSORPROFILE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(catid), ::core::mem::transmute(&mut result__)).from_abi::<TF_INPUTPROCESSORPROFILE>(result__)
    }
}
impl ::core::convert::From<ITfInputProcessorProfileMgr> for ::windows::core::IUnknown {
    fn from(value: ITfInputProcessorProfileMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfInputProcessorProfileMgr> for ::windows::core::IUnknown {
    fn from(value: &ITfInputProcessorProfileMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfInputProcessorProfileMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfInputProcessorProfileMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfInputProcessorProfileMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfInputProcessorProfileMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfileMgr {}
unsafe impl ::windows::core::Interface for ITfInputProcessorProfileMgr {
    type Vtable = ITfInputProcessorProfileMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71c6e74c_0f28_11d8_a82a_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfileMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, catid: *const ::windows::core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfInputProcessorProfileSubstituteLayout(::windows::core::IUnknown);
impl ITfInputProcessorProfileSubstituteLayout {
    pub unsafe fn GetSubstituteKeyboardLayout(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<HKL> {
        let mut result__: HKL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), ::core::mem::transmute(&mut result__)).from_abi::<HKL>(result__)
    }
}
impl ::core::convert::From<ITfInputProcessorProfileSubstituteLayout> for ::windows::core::IUnknown {
    fn from(value: ITfInputProcessorProfileSubstituteLayout) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfInputProcessorProfileSubstituteLayout> for ::windows::core::IUnknown {
    fn from(value: &ITfInputProcessorProfileSubstituteLayout) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfInputProcessorProfileSubstituteLayout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfInputProcessorProfileSubstituteLayout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfInputProcessorProfileSubstituteLayout {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfInputProcessorProfileSubstituteLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfileSubstituteLayout {}
unsafe impl ::windows::core::Interface for ITfInputProcessorProfileSubstituteLayout {
    type Vtable = ITfInputProcessorProfileSubstituteLayoutVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fd67194_1002_4513_bff2_c0ddf6258552);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfileSubstituteLayoutVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, phkl: *mut HKL) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfInputProcessorProfiles(::windows::core::IUnknown);
impl ITfInputProcessorProfiles {
    pub unsafe fn Register(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid)).ok()
    }
    pub unsafe fn Unregister(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddLanguageProfile<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: Param3, cchdesc: u32, pchiconfile: Param5, cchfile: u32, uiconindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), pchdesc.into_param().abi(), ::core::mem::transmute(cchdesc), pchiconfile.into_param().abi(), ::core::mem::transmute(cchfile), ::core::mem::transmute(uiconindex)).ok()
    }
    pub unsafe fn RemoveLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumInputProcessorInfo(&self) -> ::windows::core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumGUID>(result__)
    }
    pub unsafe fn GetDefaultLanguageProfile(&self, langid: u16, catid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(catid), ::core::mem::transmute(pclsid), ::core::mem::transmute(pguidprofile)).ok()
    }
    pub unsafe fn SetDefaultLanguageProfile(&self, langid: u16, rclsid: *const ::windows::core::GUID, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(rclsid), ::core::mem::transmute(guidprofiles)).ok()
    }
    pub unsafe fn ActivateLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofiles)).ok()
    }
    pub unsafe fn GetActiveLanguageProfile(&self, rclsid: *const ::windows::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(plangid), ::core::mem::transmute(pguidprofile)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguageProfileDescription(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetCurrentLanguage(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    pub unsafe fn ChangeCurrentLanguage(&self, langid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid)).ok()
    }
    pub unsafe fn GetLanguageList(&self, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pplangid), ::core::mem::transmute(pulcount)).ok()
    }
    pub unsafe fn EnumLanguageProfiles(&self, langid: u16) -> ::windows::core::Result<IEnumTfLanguageProfiles> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfLanguageProfiles>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableLanguageProfile<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), fenable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabledLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableLanguageProfileByDefault<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), fenable.into_param().abi()).ok()
    }
    pub unsafe fn SubstituteKeyboardLayout<'a, Param3: ::windows::core::IntoParam<'a, HKL>>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, hkl: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), hkl.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITfInputProcessorProfiles> for ::windows::core::IUnknown {
    fn from(value: ITfInputProcessorProfiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfInputProcessorProfiles> for ::windows::core::IUnknown {
    fn from(value: &ITfInputProcessorProfiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfInputProcessorProfiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfInputProcessorProfiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfInputProcessorProfiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfInputProcessorProfiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfiles {}
unsafe impl ::windows::core::Interface for ITfInputProcessorProfiles {
    type Vtable = ITfInputProcessorProfilesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f02b6c5_7842_4ee6_8a0b_9a24183a95ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfilesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, catid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, rclsid: *const ::windows::core::GUID, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pbstrprofile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, hkl: HKL) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfInputProcessorProfilesEx(::windows::core::IUnknown);
impl ITfInputProcessorProfilesEx {
    pub unsafe fn Register(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid)).ok()
    }
    pub unsafe fn Unregister(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddLanguageProfile<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: Param3, cchdesc: u32, pchiconfile: Param5, cchfile: u32, uiconindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), pchdesc.into_param().abi(), ::core::mem::transmute(cchdesc), pchiconfile.into_param().abi(), ::core::mem::transmute(cchfile), ::core::mem::transmute(uiconindex)).ok()
    }
    pub unsafe fn RemoveLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumInputProcessorInfo(&self) -> ::windows::core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumGUID>(result__)
    }
    pub unsafe fn GetDefaultLanguageProfile(&self, langid: u16, catid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(catid), ::core::mem::transmute(pclsid), ::core::mem::transmute(pguidprofile)).ok()
    }
    pub unsafe fn SetDefaultLanguageProfile(&self, langid: u16, rclsid: *const ::windows::core::GUID, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(rclsid), ::core::mem::transmute(guidprofiles)).ok()
    }
    pub unsafe fn ActivateLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofiles)).ok()
    }
    pub unsafe fn GetActiveLanguageProfile(&self, rclsid: *const ::windows::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(plangid), ::core::mem::transmute(pguidprofile)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguageProfileDescription(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetCurrentLanguage(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    pub unsafe fn ChangeCurrentLanguage(&self, langid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid)).ok()
    }
    pub unsafe fn GetLanguageList(&self, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pplangid), ::core::mem::transmute(pulcount)).ok()
    }
    pub unsafe fn EnumLanguageProfiles(&self, langid: u16) -> ::windows::core::Result<IEnumTfLanguageProfiles> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfLanguageProfiles>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableLanguageProfile<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), fenable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabledLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableLanguageProfileByDefault<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), fenable.into_param().abi()).ok()
    }
    pub unsafe fn SubstituteKeyboardLayout<'a, Param3: ::windows::core::IntoParam<'a, HKL>>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, hkl: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), hkl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLanguageProfileDisplayName<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchfile: Param3, cchfile: u32, uresid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), pchfile.into_param().abi(), ::core::mem::transmute(cchfile), ::core::mem::transmute(uresid)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfInputProcessorProfiles> for ITfInputProcessorProfilesEx {
    fn into_param(self) -> ::windows::core::Param<'a, ITfInputProcessorProfiles> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfInputProcessorProfiles> for &ITfInputProcessorProfilesEx {
    fn into_param(self) -> ::windows::core::Param<'a, ITfInputProcessorProfiles> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfInputProcessorProfilesEx> for ::windows::core::IUnknown {
    fn from(value: ITfInputProcessorProfilesEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfInputProcessorProfilesEx> for ::windows::core::IUnknown {
    fn from(value: &ITfInputProcessorProfilesEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfInputProcessorProfilesEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfInputProcessorProfilesEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfInputProcessorProfilesEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfInputProcessorProfilesEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfilesEx {}
unsafe impl ::windows::core::Interface for ITfInputProcessorProfilesEx {
    type Vtable = ITfInputProcessorProfilesExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x892f230f_fe00_4a41_a98e_fcd6de0d35ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfilesExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32, pchiconfile: super::super::Foundation::PWSTR, cchfile: u32, uiconindex: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, catid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, rclsid: *const ::windows::core::GUID, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pbstrprofile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, hkl: HKL) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchfile: super::super::Foundation::PWSTR, cchfile: u32, uresid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfInputScope(::windows::core::IUnknown);
impl ITfInputScope {
    pub unsafe fn GetInputScopes(&self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprginputscopes), ::core::mem::transmute(pccount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPhrase(&self, ppbstrphrases: *mut *mut super::super::Foundation::BSTR, pccount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbstrphrases), ::core::mem::transmute(pccount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRegularExpression(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSRGS(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXML(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITfInputScope> for ::windows::core::IUnknown {
    fn from(value: ITfInputScope) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfInputScope> for ::windows::core::IUnknown {
    fn from(value: &ITfInputScope) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfInputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfInputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfInputScope {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfInputScope {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputScope {}
unsafe impl ::windows::core::Interface for ITfInputScope {
    type Vtable = ITfInputScopeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfde1eaee_6924_4cdf_91e7_da38cff5559d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputScopeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbstrphrases: *mut *mut super::super::Foundation::BSTR, pccount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrregexp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsrgs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfInputScope2(::windows::core::IUnknown);
impl ITfInputScope2 {
    pub unsafe fn GetInputScopes(&self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprginputscopes), ::core::mem::transmute(pccount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPhrase(&self, ppbstrphrases: *mut *mut super::super::Foundation::BSTR, pccount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbstrphrases), ::core::mem::transmute(pccount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRegularExpression(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSRGS(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXML(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumWordList(&self) -> ::windows::core::Result<super::super::System::Com::IEnumString> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IEnumString>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITfInputScope> for ITfInputScope2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITfInputScope> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfInputScope> for &ITfInputScope2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITfInputScope> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfInputScope2> for ::windows::core::IUnknown {
    fn from(value: ITfInputScope2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfInputScope2> for ::windows::core::IUnknown {
    fn from(value: &ITfInputScope2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfInputScope2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfInputScope2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfInputScope2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfInputScope2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputScope2 {}
unsafe impl ::windows::core::Interface for ITfInputScope2 {
    type Vtable = ITfInputScope2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5731eaa0_6bc2_4681_a532_92fbb74d7c41);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputScope2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbstrphrases: *mut *mut super::super::Foundation::BSTR, pccount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrregexp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsrgs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstring: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct ITfInsertAtSelection(::windows::core::IUnknown);
impl ITfInsertAtSelection {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertTextAtSelection<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: Param2, cch: i32) -> ::windows::core::Result<ITfRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), pchtext.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(&mut result__)).from_abi::<ITfRange>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbeddedAtSelection<'a, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, ec: u32, dwflags: u32, pdataobject: Param2) -> ::windows::core::Result<ITfRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), pdataobject.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfRange>(result__)
    }
}
impl ::core::convert::From<ITfInsertAtSelection> for ::windows::core::IUnknown {
    fn from(value: ITfInsertAtSelection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfInsertAtSelection> for ::windows::core::IUnknown {
    fn from(value: &ITfInsertAtSelection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfInsertAtSelection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfInsertAtSelection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfInsertAtSelection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfInsertAtSelection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInsertAtSelection {}
unsafe impl ::windows::core::Interface for ITfInsertAtSelection {
    type Vtable = ITfInsertAtSelectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55ce16ba_3014_41c1_9ceb_fade1446ac6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInsertAtSelectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: super::super::Foundation::PWSTR, cch: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: ::windows::core::RawPtr, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct ITfIntegratableCandidateListUIElement(::windows::core::IUnknown);
impl ITfIntegratableCandidateListUIElement {
    pub unsafe fn SetIntegrationStyle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidintegrationstyle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), guidintegrationstyle.into_param().abi()).ok()
    }
    pub unsafe fn GetSelectionStyle(&self) -> ::windows::core::Result<TfIntegratableCandidateListSelectionStyle> {
        let mut result__: TfIntegratableCandidateListSelectionStyle = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TfIntegratableCandidateListSelectionStyle>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowCandidateNumbers(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn FinalizeExactCompositionString(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITfIntegratableCandidateListUIElement> for ::windows::core::IUnknown {
    fn from(value: ITfIntegratableCandidateListUIElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfIntegratableCandidateListUIElement> for ::windows::core::IUnknown {
    fn from(value: &ITfIntegratableCandidateListUIElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfIntegratableCandidateListUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfIntegratableCandidateListUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfIntegratableCandidateListUIElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfIntegratableCandidateListUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfIntegratableCandidateListUIElement {}
unsafe impl ::windows::core::Interface for ITfIntegratableCandidateListUIElement {
    type Vtable = ITfIntegratableCandidateListUIElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7a6f54f_b180_416f_b2bf_7bf2e4683d7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfIntegratableCandidateListUIElementVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidintegrationstyle: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptfselectionstyle: *mut TfIntegratableCandidateListSelectionStyle) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfKeyEventSink(::windows::core::IUnknown);
impl ITfKeyEventSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnSetFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforeground: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fforeground.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTestKeyDown<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, pic: Param0, wparam: Param1, lparam: Param2) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pic.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTestKeyUp<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, pic: Param0, wparam: Param1, lparam: Param2) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pic.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyDown<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, pic: Param0, wparam: Param1, lparam: Param2) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pic.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyUp<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, pic: Param0, wparam: Param1, lparam: Param2) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pic.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnPreservedKey<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>>(&self, pic: Param0, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(rguid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ITfKeyEventSink> for ::windows::core::IUnknown {
    fn from(value: ITfKeyEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfKeyEventSink> for ::windows::core::IUnknown {
    fn from(value: &ITfKeyEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfKeyEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfKeyEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfKeyEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfKeyEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfKeyEventSink {}
unsafe impl ::windows::core::Interface for ITfKeyEventSink {
    type Vtable = ITfKeyEventSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e7f5_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfKeyEventSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, rguid: *const ::windows::core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfKeyTraceEventSink(::windows::core::IUnknown);
impl ITfKeyTraceEventSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyTraceDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyTraceUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITfKeyTraceEventSink> for ::windows::core::IUnknown {
    fn from(value: ITfKeyTraceEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfKeyTraceEventSink> for ::windows::core::IUnknown {
    fn from(value: &ITfKeyTraceEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfKeyTraceEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfKeyTraceEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfKeyTraceEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfKeyTraceEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfKeyTraceEventSink {}
unsafe impl ::windows::core::Interface for ITfKeyTraceEventSink {
    type Vtable = ITfKeyTraceEventSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cd4c13b_1c36_4191_a70a_7f3e611f367d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfKeyTraceEventSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfKeystrokeMgr(::windows::core::IUnknown);
impl ITfKeystrokeMgr {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdviseKeyEventSink<'a, Param1: ::windows::core::IntoParam<'a, ITfKeyEventSink>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, tid: u32, psink: Param1, fforeground: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), psink.into_param().abi(), fforeground.into_param().abi()).ok()
    }
    pub unsafe fn UnadviseKeyEventSink(&self, tid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid)).ok()
    }
    pub unsafe fn GetForeground(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TestKeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TestKeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn KeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn KeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, wparam: Param0, lparam: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetPreservedKey<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>>(&self, pic: Param0, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(pprekey), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPreservedKey(&self, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(pprekey), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreserveKey<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, tid: u32, rguid: *const ::windows::core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: Param3, cchdesc: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), ::core::mem::transmute(rguid), ::core::mem::transmute(prekey), pchdesc.into_param().abi(), ::core::mem::transmute(cchdesc)).ok()
    }
    pub unsafe fn UnpreserveKey(&self, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(pprekey)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreservedKeyDescription<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, rguid: *const ::windows::core::GUID, pchdesc: Param1, cchdesc: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), pchdesc.into_param().abi(), ::core::mem::transmute(cchdesc)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreservedKeyDescription(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SimulatePreservedKey<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>>(&self, pic: Param0, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(rguid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ITfKeystrokeMgr> for ::windows::core::IUnknown {
    fn from(value: ITfKeystrokeMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfKeystrokeMgr> for ::windows::core::IUnknown {
    fn from(value: &ITfKeystrokeMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfKeystrokeMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfKeystrokeMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfKeystrokeMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfKeystrokeMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfKeystrokeMgr {}
unsafe impl ::windows::core::Interface for ITfKeystrokeMgr {
    type Vtable = ITfKeystrokeMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e7f0_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfKeystrokeMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, psink: ::windows::core::RawPtr, fforeground: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, pprekey: *const TF_PRESERVEDKEY, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY, pfregistered: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows::core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pchdesc: super::super::Foundation::PWSTR, cchdesc: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pbstrdesc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, rguid: *const ::windows::core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfLMLattice(::windows::core::IUnknown);
impl ITfLMLattice {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryType(&self, rguidtype: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidtype), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn EnumLatticeElements(&self, dwframestart: u32, rguidtype: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumTfLatticeElements> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwframestart), ::core::mem::transmute(rguidtype), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfLatticeElements>(result__)
    }
}
impl ::core::convert::From<ITfLMLattice> for ::windows::core::IUnknown {
    fn from(value: ITfLMLattice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLMLattice> for ::windows::core::IUnknown {
    fn from(value: &ITfLMLattice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfLMLattice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfLMLattice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfLMLattice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfLMLattice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLMLattice {}
unsafe impl ::windows::core::Interface for ITfLMLattice {
    type Vtable = ITfLMLatticeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4236675_a5bf_4570_9d42_5d6d7b02d59b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLMLatticeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidtype: *const ::windows::core::GUID, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwframestart: u32, rguidtype: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfLangBarEventSink(::windows::core::IUnknown);
impl ITfLangBarEventSink {
    pub unsafe fn OnSetFocus(&self, dwthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid)).ok()
    }
    pub unsafe fn OnThreadTerminate(&self, dwthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid)).ok()
    }
    pub unsafe fn OnThreadItemChange(&self, dwthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnModalInput<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, dwthreadid: u32, umsg: u32, wparam: Param2, lparam: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(umsg), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    pub unsafe fn ShowFloating(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(rguid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
}
impl ::core::convert::From<ITfLangBarEventSink> for ::windows::core::IUnknown {
    fn from(value: ITfLangBarEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarEventSink> for ::windows::core::IUnknown {
    fn from(value: &ITfLangBarEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfLangBarEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfLangBarEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfLangBarEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfLangBarEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarEventSink {}
unsafe impl ::windows::core::Interface for ITfLangBarEventSink {
    type Vtable = ITfLangBarEventSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18a4e900_e0ae_11d2_afdd_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarEventSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows::core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfLangBarItem(::windows::core::IUnknown);
impl ITfLangBarItem {
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<TF_LANGBARITEMINFO> {
        let mut result__: TF_LANGBARITEMINFO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TF_LANGBARITEMINFO>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITfLangBarItem> for ::windows::core::IUnknown {
    fn from(value: ITfLangBarItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarItem> for ::windows::core::IUnknown {
    fn from(value: &ITfLangBarItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfLangBarItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfLangBarItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfLangBarItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItem {}
unsafe impl ::windows::core::Interface for ITfLangBarItem {
    type Vtable = ITfLangBarItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73540d69_edeb_4ee9_96c9_23aa30b25916);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtooltip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfLangBarItemBalloon(::windows::core::IUnknown);
impl ITfLangBarItemBalloon {
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<TF_LANGBARITEMINFO> {
        let mut result__: TF_LANGBARITEMINFO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TF_LANGBARITEMINFO>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnClick<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::POINT>>(&self, click: TfLBIClick, pt: Param1, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(click), pt.into_param().abi(), ::core::mem::transmute(prcarea)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__: super::super::Foundation::SIZE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszdefault), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SIZE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBalloonInfo(&self) -> ::windows::core::Result<TF_LBBALLOONINFO> {
        let mut result__: ::core::mem::ManuallyDrop<TF_LBBALLOONINFO> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TF_LBBALLOONINFO>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITfLangBarItem> for ITfLangBarItemBalloon {
    fn into_param(self) -> ::windows::core::Param<'a, ITfLangBarItem> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfLangBarItem> for &ITfLangBarItemBalloon {
    fn into_param(self) -> ::windows::core::Param<'a, ITfLangBarItem> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfLangBarItemBalloon> for ::windows::core::IUnknown {
    fn from(value: ITfLangBarItemBalloon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarItemBalloon> for ::windows::core::IUnknown {
    fn from(value: &ITfLangBarItemBalloon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfLangBarItemBalloon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfLangBarItemBalloon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfLangBarItemBalloon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItemBalloon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemBalloon {}
unsafe impl ::windows::core::Interface for ITfLangBarItemBalloon {
    type Vtable = ITfLangBarItemBalloonVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01c2d285_d3c7_4b7b_b5b5_d97411d0c283);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemBalloonVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtooltip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LBBALLOONINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfLangBarItemBitmap(::windows::core::IUnknown);
impl ITfLangBarItemBitmap {
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<TF_LANGBARITEMINFO> {
        let mut result__: TF_LANGBARITEMINFO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TF_LANGBARITEMINFO>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnClick<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::POINT>>(&self, click: TfLBIClick, pt: Param1, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(click), pt.into_param().abi(), ::core::mem::transmute(prcarea)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__: super::super::Foundation::SIZE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszdefault), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SIZE>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(bmwidth), ::core::mem::transmute(bmheight), ::core::mem::transmute(dwflags), ::core::mem::transmute(phbmp), ::core::mem::transmute(phbmpmask)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfLangBarItem> for ITfLangBarItemBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ITfLangBarItem> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfLangBarItem> for &ITfLangBarItemBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ITfLangBarItem> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfLangBarItemBitmap> for ::windows::core::IUnknown {
    fn from(value: ITfLangBarItemBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarItemBitmap> for ::windows::core::IUnknown {
    fn from(value: &ITfLangBarItemBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfLangBarItemBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfLangBarItemBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfLangBarItemBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItemBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemBitmap {}
unsafe impl ::windows::core::Interface for ITfLangBarItemBitmap {
    type Vtable = ITfLangBarItemBitmapVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73830352_d722_4179_ada5_f045c98df355);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemBitmapVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtooltip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[repr(transparent)]
pub struct ITfLangBarItemBitmapButton(::windows::core::IUnknown);
impl ITfLangBarItemBitmapButton {
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<TF_LANGBARITEMINFO> {
        let mut result__: TF_LANGBARITEMINFO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TF_LANGBARITEMINFO>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnClick<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::POINT>>(&self, click: TfLBIClick, pt: Param1, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(click), pt.into_param().abi(), ::core::mem::transmute(prcarea)).ok()
    }
    pub unsafe fn InitMenu<'a, Param0: ::windows::core::IntoParam<'a, ITfMenu>>(&self, pmenu: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pmenu.into_param().abi()).ok()
    }
    pub unsafe fn OnMenuSelect(&self, wid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__: super::super::Foundation::SIZE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszdefault), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SIZE>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(bmwidth), ::core::mem::transmute(bmheight), ::core::mem::transmute(dwflags), ::core::mem::transmute(phbmp), ::core::mem::transmute(phbmpmask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITfLangBarItem> for ITfLangBarItemBitmapButton {
    fn into_param(self) -> ::windows::core::Param<'a, ITfLangBarItem> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfLangBarItem> for &ITfLangBarItemBitmapButton {
    fn into_param(self) -> ::windows::core::Param<'a, ITfLangBarItem> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfLangBarItemBitmapButton> for ::windows::core::IUnknown {
    fn from(value: ITfLangBarItemBitmapButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarItemBitmapButton> for ::windows::core::IUnknown {
    fn from(value: &ITfLangBarItemBitmapButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfLangBarItemBitmapButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfLangBarItemBitmapButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfLangBarItemBitmapButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItemBitmapButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemBitmapButton {}
unsafe impl ::windows::core::Interface for ITfLangBarItemBitmapButton {
    type Vtable = ITfLangBarItemBitmapButtonVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa26a0525_3fae_4fa0_89ee_88a964f9f1b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemBitmapButtonVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtooltip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmenu: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfLangBarItemButton(::windows::core::IUnknown);
impl ITfLangBarItemButton {
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<TF_LANGBARITEMINFO> {
        let mut result__: TF_LANGBARITEMINFO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TF_LANGBARITEMINFO>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnClick<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::POINT>>(&self, click: TfLBIClick, pt: Param1, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(click), pt.into_param().abi(), ::core::mem::transmute(prcarea)).ok()
    }
    pub unsafe fn InitMenu<'a, Param0: ::windows::core::IntoParam<'a, ITfMenu>>(&self, pmenu: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pmenu.into_param().abi()).ok()
    }
    pub unsafe fn OnMenuSelect(&self, wid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wid)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon(&self) -> ::windows::core::Result<super::WindowsAndMessaging::HICON> {
        let mut result__: super::WindowsAndMessaging::HICON = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITfLangBarItem> for ITfLangBarItemButton {
    fn into_param(self) -> ::windows::core::Param<'a, ITfLangBarItem> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfLangBarItem> for &ITfLangBarItemButton {
    fn into_param(self) -> ::windows::core::Param<'a, ITfLangBarItem> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfLangBarItemButton> for ::windows::core::IUnknown {
    fn from(value: ITfLangBarItemButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarItemButton> for ::windows::core::IUnknown {
    fn from(value: &ITfLangBarItemButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfLangBarItemButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfLangBarItemButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfLangBarItemButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItemButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemButton {}
unsafe impl ::windows::core::Interface for ITfLangBarItemButton {
    type Vtable = ITfLangBarItemButtonVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28c7f1d0_de25_11d2_afdd_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemButtonVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtooltip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmenu: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phicon: *mut super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfLangBarItemMgr(::windows::core::IUnknown);
impl ITfLangBarItemMgr {
    pub unsafe fn EnumItems(&self) -> ::windows::core::Result<IEnumTfLangBarItems> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfLangBarItems>(result__)
    }
    pub unsafe fn GetItem(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfLangBarItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid), ::core::mem::transmute(&mut result__)).from_abi::<ITfLangBarItem>(result__)
    }
    pub unsafe fn AddItem<'a, Param0: ::windows::core::IntoParam<'a, ITfLangBarItem>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn RemoveItem<'a, Param0: ::windows::core::IntoParam<'a, ITfLangBarItem>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn AdviseItemSink<'a, Param0: ::windows::core::IntoParam<'a, ITfLangBarItemSink>>(&self, punk: Param0, pdwcookie: *mut u32, rguiditem: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), punk.into_param().abi(), ::core::mem::transmute(pdwcookie), ::core::mem::transmute(rguiditem)).ok()
    }
    pub unsafe fn UnadviseItemSink(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__: super::super::Foundation::RECT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(rguid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::RECT>(result__)
    }
    pub unsafe fn GetItemsStatus(&self, ulcount: u32, prgguid: *const ::windows::core::GUID, pdwstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(prgguid), ::core::mem::transmute(pdwstatus)).ok()
    }
    pub unsafe fn GetItemNum(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetItems(&self, ulcount: u32, ppitem: *mut ::core::option::Option<ITfLangBarItem>, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppitem), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdwstatus), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn AdviseItemsSink(&self, ulcount: u32, ppunk: *const ::core::option::Option<ITfLangBarItemSink>, pguiditem: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(ppunk), ::core::mem::transmute(pguiditem), ::core::mem::transmute(pdwcookie)).ok()
    }
    pub unsafe fn UnadviseItemsSink(&self, ulcount: u32, pdwcookie: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(pdwcookie)).ok()
    }
}
impl ::core::convert::From<ITfLangBarItemMgr> for ::windows::core::IUnknown {
    fn from(value: ITfLangBarItemMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarItemMgr> for ::windows::core::IUnknown {
    fn from(value: &ITfLangBarItemMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfLangBarItemMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfLangBarItemMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfLangBarItemMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItemMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemMgr {}
unsafe impl ::windows::core::Interface for ITfLangBarItemMgr {
    type Vtable = ITfLangBarItemMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba468c55_9956_4fb1_a59d_52a7dd7cc6aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr, pdwcookie: *mut u32, rguiditem: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows::core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, prgguid: *const ::windows::core::GUID, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut ::windows::core::RawPtr, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppunk: *const ::windows::core::RawPtr, pguiditem: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pdwcookie: *const u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfLangBarItemSink(::windows::core::IUnknown);
impl ITfLangBarItemSink {
    pub unsafe fn OnUpdate(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<ITfLangBarItemSink> for ::windows::core::IUnknown {
    fn from(value: ITfLangBarItemSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarItemSink> for ::windows::core::IUnknown {
    fn from(value: &ITfLangBarItemSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfLangBarItemSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfLangBarItemSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfLangBarItemSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfLangBarItemSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemSink {}
unsafe impl ::windows::core::Interface for ITfLangBarItemSink {
    type Vtable = ITfLangBarItemSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57dbe1a0_de25_11d2_afdd_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemSinkVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ITfLangBarMgr(::windows::core::IUnknown);
impl ITfLangBarMgr {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdviseEventSink<'a, Param0: ::windows::core::IntoParam<'a, ITfLangBarEventSink>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, psink: Param0, hwnd: Param1, dwflags: u32, pdwcookie: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psink.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pdwcookie)).ok()
    }
    pub unsafe fn UnadviseEventSink(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    pub unsafe fn GetThreadMarshalInterface(&self, dwthreadid: u32, dwtype: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(dwtype), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetThreadLangBarItemMgr(&self, dwthreadid: u32, pplbi: *mut ::core::option::Option<ITfLangBarItemMgr>, pdwthreadid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(pplbi), ::core::mem::transmute(pdwthreadid)).ok()
    }
    pub unsafe fn GetInputProcessorProfiles(&self, dwthreadid: u32, ppaip: *mut ::core::option::Option<ITfInputProcessorProfiles>, pdwthreadid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(ppaip), ::core::mem::transmute(pdwthreadid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreLastFocus<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pdwthreadid: *mut u32, fprev: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwthreadid), fprev.into_param().abi()).ok()
    }
    pub unsafe fn SetModalInput<'a, Param0: ::windows::core::IntoParam<'a, ITfLangBarEventSink>>(&self, psink: Param0, dwthreadid: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), psink.into_param().abi(), ::core::mem::transmute(dwthreadid), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn ShowFloating(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetShowFloatingStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ITfLangBarMgr> for ::windows::core::IUnknown {
    fn from(value: ITfLangBarMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLangBarMgr> for ::windows::core::IUnknown {
    fn from(value: &ITfLangBarMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfLangBarMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfLangBarMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfLangBarMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfLangBarMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarMgr {}
unsafe impl ::windows::core::Interface for ITfLangBarMgr {
    type Vtable = ITfLangBarMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87955690_e627_11d2_8ddb_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32, dwtype: u32, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32, pplbi: *mut ::windows::core::RawPtr, pdwthreadid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32, ppaip: *mut ::windows::core::RawPtr, pdwthreadid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, dwthreadid: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfLanguageProfileNotifySink(::windows::core::IUnknown);
impl ITfLanguageProfileNotifySink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnLanguageChange(&self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn OnLanguageChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITfLanguageProfileNotifySink> for ::windows::core::IUnknown {
    fn from(value: ITfLanguageProfileNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfLanguageProfileNotifySink> for ::windows::core::IUnknown {
    fn from(value: &ITfLanguageProfileNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfLanguageProfileNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfLanguageProfileNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfLanguageProfileNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfLanguageProfileNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLanguageProfileNotifySink {}
unsafe impl ::windows::core::Interface for ITfLanguageProfileNotifySink {
    type Vtable = ITfLanguageProfileNotifySinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43c9fe15_f494_4c17_9de2_b8a4ac350aa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLanguageProfileNotifySinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfMSAAControl(::windows::core::IUnknown);
impl ITfMSAAControl {
    pub unsafe fn SystemEnableMSAA(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SystemDisableMSAA(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITfMSAAControl> for ::windows::core::IUnknown {
    fn from(value: ITfMSAAControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfMSAAControl> for ::windows::core::IUnknown {
    fn from(value: &ITfMSAAControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfMSAAControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfMSAAControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfMSAAControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfMSAAControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMSAAControl {}
unsafe impl ::windows::core::Interface for ITfMSAAControl {
    type Vtable = ITfMSAAControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5f8fb3b_393f_4f7c_84cb_504924c2705a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMSAAControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfMenu(::windows::core::IUnknown);
impl ITfMenu {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn AddMenuItem<'a, Param2: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>, Param3: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, uid: u32, dwflags: u32, hbmp: Param2, hbmpmask: Param3, pch: Param4, cch: u32, ppmenu: *mut ::core::option::Option<ITfMenu>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uid), ::core::mem::transmute(dwflags), hbmp.into_param().abi(), hbmpmask.into_param().abi(), pch.into_param().abi(), ::core::mem::transmute(cch), ::core::mem::transmute(ppmenu)).ok()
    }
}
impl ::core::convert::From<ITfMenu> for ::windows::core::IUnknown {
    fn from(value: ITfMenu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfMenu> for ::windows::core::IUnknown {
    fn from(value: &ITfMenu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMenu {}
unsafe impl ::windows::core::Interface for ITfMenu {
    type Vtable = ITfMenuVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f8a98e4_aaa0_4f15_8c5b_07e0df0a3dd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMenuVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: super::super::Foundation::PWSTR, cch: u32, ppmenu: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
);
#[repr(transparent)]
pub struct ITfMessagePump(::windows::core::IUnknown);
impl ITfMessagePump {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn PeekMessageA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: Param1, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), hwnd.into_param().abi(), ::core::mem::transmute(wmsgfiltermin), ::core::mem::transmute(wmsgfiltermax), ::core::mem::transmute(wremovemsg), ::core::mem::transmute(pfresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetMessageA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: Param1, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), hwnd.into_param().abi(), ::core::mem::transmute(wmsgfiltermin), ::core::mem::transmute(wmsgfiltermax), ::core::mem::transmute(pfresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn PeekMessageW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: Param1, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), hwnd.into_param().abi(), ::core::mem::transmute(wmsgfiltermin), ::core::mem::transmute(wmsgfiltermax), ::core::mem::transmute(wremovemsg), ::core::mem::transmute(pfresult)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetMessageW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: Param1, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), hwnd.into_param().abi(), ::core::mem::transmute(wmsgfiltermin), ::core::mem::transmute(wmsgfiltermax), ::core::mem::transmute(pfresult)).ok()
    }
}
impl ::core::convert::From<ITfMessagePump> for ::windows::core::IUnknown {
    fn from(value: ITfMessagePump) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfMessagePump> for ::windows::core::IUnknown {
    fn from(value: &ITfMessagePump) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfMessagePump {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfMessagePump {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfMessagePump {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfMessagePump {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMessagePump {}
unsafe impl ::windows::core::Interface for ITfMessagePump {
    type Vtable = ITfMessagePumpVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1b8ad8_0b6b_4874_90c5_bd76011e8f7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMessagePumpVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[repr(transparent)]
pub struct ITfMouseSink(::windows::core::IUnknown);
impl ITfMouseSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnMouseEvent(&self, uedge: u32, uquadrant: u32, dwbtnstatus: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uedge), ::core::mem::transmute(uquadrant), ::core::mem::transmute(dwbtnstatus), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ITfMouseSink> for ::windows::core::IUnknown {
    fn from(value: ITfMouseSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfMouseSink> for ::windows::core::IUnknown {
    fn from(value: &ITfMouseSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfMouseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfMouseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfMouseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfMouseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMouseSink {}
unsafe impl ::windows::core::Interface for ITfMouseSink {
    type Vtable = ITfMouseSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1adaaa2_3a24_449d_ac96_5183e7f5c217);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMouseSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uedge: u32, uquadrant: u32, dwbtnstatus: u32, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfMouseTracker(::windows::core::IUnknown);
impl ITfMouseTracker {
    pub unsafe fn AdviseMouseSink<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>, Param1: ::windows::core::IntoParam<'a, ITfMouseSink>>(&self, range: Param0, psink: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), range.into_param().abi(), psink.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn UnadviseMouseSink(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
}
impl ::core::convert::From<ITfMouseTracker> for ::windows::core::IUnknown {
    fn from(value: ITfMouseTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfMouseTracker> for ::windows::core::IUnknown {
    fn from(value: &ITfMouseTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfMouseTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfMouseTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfMouseTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfMouseTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMouseTracker {}
unsafe impl ::windows::core::Interface for ITfMouseTracker {
    type Vtable = ITfMouseTrackerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09d146cd_a544_4132_925b_7afa8ef322d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMouseTrackerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfMouseTrackerACP(::windows::core::IUnknown);
impl ITfMouseTrackerACP {
    pub unsafe fn AdviseMouseSink<'a, Param0: ::windows::core::IntoParam<'a, ITfRangeACP>, Param1: ::windows::core::IntoParam<'a, ITfMouseSink>>(&self, range: Param0, psink: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), range.into_param().abi(), psink.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn UnadviseMouseSink(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
}
impl ::core::convert::From<ITfMouseTrackerACP> for ::windows::core::IUnknown {
    fn from(value: ITfMouseTrackerACP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfMouseTrackerACP> for ::windows::core::IUnknown {
    fn from(value: &ITfMouseTrackerACP) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfMouseTrackerACP {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfMouseTrackerACP {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfMouseTrackerACP {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfMouseTrackerACP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMouseTrackerACP {}
unsafe impl ::windows::core::Interface for ITfMouseTrackerACP {
    type Vtable = ITfMouseTrackerACPVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bdd78e2_c16e_47fd_b883_ce6facc1a208);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMouseTrackerACPVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, psink: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfPersistentPropertyLoaderACP(::windows::core::IUnknown);
impl ITfPersistentPropertyLoaderACP {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadProperty(&self, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(phdr), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
}
impl ::core::convert::From<ITfPersistentPropertyLoaderACP> for ::windows::core::IUnknown {
    fn from(value: ITfPersistentPropertyLoaderACP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfPersistentPropertyLoaderACP> for ::windows::core::IUnknown {
    fn from(value: &ITfPersistentPropertyLoaderACP) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfPersistentPropertyLoaderACP {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfPersistentPropertyLoaderACP {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfPersistentPropertyLoaderACP {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfPersistentPropertyLoaderACP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfPersistentPropertyLoaderACP {}
unsafe impl ::windows::core::Interface for ITfPersistentPropertyLoaderACP {
    type Vtable = ITfPersistentPropertyLoaderACPVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ef89150_0807_11d3_8df0_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPersistentPropertyLoaderACPVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct ITfPreservedKeyNotifySink(::windows::core::IUnknown);
impl ITfPreservedKeyNotifySink {
    pub unsafe fn OnUpdated(&self, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprekey)).ok()
    }
}
impl ::core::convert::From<ITfPreservedKeyNotifySink> for ::windows::core::IUnknown {
    fn from(value: ITfPreservedKeyNotifySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfPreservedKeyNotifySink> for ::windows::core::IUnknown {
    fn from(value: &ITfPreservedKeyNotifySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfPreservedKeyNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfPreservedKeyNotifySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfPreservedKeyNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfPreservedKeyNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfPreservedKeyNotifySink {}
unsafe impl ::windows::core::Interface for ITfPreservedKeyNotifySink {
    type Vtable = ITfPreservedKeyNotifySinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f77c993_d2b1_446e_853e_5912efc8a286);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPreservedKeyNotifySinkVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ITfProperty(::windows::core::IUnknown);
impl ITfProperty {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn EnumRanges<'a, Param2: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, ppenum: *mut ::core::option::Option<IEnumTfRanges>, ptargetrange: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(ppenum), ptargetrange.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfContext>(result__)
    }
    pub unsafe fn FindRange<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, pprange: *mut ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(pprange), ::core::mem::transmute(apos)).ok()
    }
    pub unsafe fn SetValueStore<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>, Param2: ::windows::core::IntoParam<'a, ITfPropertyStore>>(&self, ec: u32, prange: Param1, ppropstore: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ppropstore.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn Clear<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfReadOnlyProperty> for ITfProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ITfReadOnlyProperty> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfReadOnlyProperty> for &ITfProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ITfReadOnlyProperty> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfProperty> for ::windows::core::IUnknown {
    fn from(value: ITfProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfProperty> for ::windows::core::IUnknown {
    fn from(value: &ITfProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfProperty {}
unsafe impl ::windows::core::Interface for ITfProperty {
    type Vtable = ITfPropertyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2449660_9542_11d2_bf46_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPropertyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ppenum: *mut ::windows::core::RawPtr, ptargetrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pprange: *mut ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, ppropstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfPropertyStore(::windows::core::IUnknown);
impl ITfPropertyStore {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetDataType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetData(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTextUpdated<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, dwflags: u32, prangenew: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), prangenew.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Shrink<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>>(&self, prangenew: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), prangenew.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Divide<'a, Param0: ::windows::core::IntoParam<'a, ITfRange>, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, prangethis: Param0, prangenew: Param1) -> ::windows::core::Result<ITfPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), prangethis.into_param().abi(), prangenew.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfPropertyStore>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ITfPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfPropertyStore>(result__)
    }
    pub unsafe fn GetPropertyRangeCreator(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pstream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ITfPropertyStore> for ::windows::core::IUnknown {
    fn from(value: ITfPropertyStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfPropertyStore> for ::windows::core::IUnknown {
    fn from(value: &ITfPropertyStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfPropertyStore {}
unsafe impl ::windows::core::Interface for ITfPropertyStore {
    type Vtable = ITfPropertyStoreVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6834b120_88cb_11d2_bf45_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPropertyStoreVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, prangenew: ::windows::core::RawPtr, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangenew: ::windows::core::RawPtr, pffree: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangethis: ::windows::core::RawPtr, prangenew: ::windows::core::RawPtr, pppropstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcb: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct ITfQueryEmbedded(::windows::core::IUnknown);
impl ITfQueryEmbedded {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidservice), ::core::mem::transmute(pformatetc), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ITfQueryEmbedded> for ::windows::core::IUnknown {
    fn from(value: ITfQueryEmbedded) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfQueryEmbedded> for ::windows::core::IUnknown {
    fn from(value: &ITfQueryEmbedded) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfQueryEmbedded {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfQueryEmbedded {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfQueryEmbedded {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfQueryEmbedded {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfQueryEmbedded {}
unsafe impl ::windows::core::Interface for ITfQueryEmbedded {
    type Vtable = ITfQueryEmbeddedVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fab9bdb_d250_4169_84e5_6be118fdd7a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfQueryEmbeddedVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct ITfRange(::windows::core::IUnknown);
impl ITfRange {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), ::core::mem::transmute(pchtext), ::core::mem::transmute(cchmax), ::core::mem::transmute(pcch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ec: u32, dwflags: u32, pchtext: Param2, cch: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), pchtext.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self, ec: u32) -> ::windows::core::Result<super::super::System::Com::IDataObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IDataObject>(result__)
    }
    pub unsafe fn GetEmbedded(&self, ec: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(rguidservice), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<'a, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, ec: u32, dwflags: u32, pdataobject: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), pdataobject.into_param().abi()).ok()
    }
    pub unsafe fn ShiftStart(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(cchreq), ::core::mem::transmute(pcch), ::core::mem::transmute(phalt)).ok()
    }
    pub unsafe fn ShiftEnd(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(cchreq), ::core::mem::transmute(pcch), ::core::mem::transmute(phalt)).ok()
    }
    pub unsafe fn ShiftStartToRange<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, apos: TfAnchor) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(apos)).ok()
    }
    pub unsafe fn ShiftEndToRange<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, apos: TfAnchor) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(apos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShiftStartRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dir), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShiftEndRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dir), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEmpty(&self, ec: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Collapse(&self, ec: u32, apos: TfAnchor) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(apos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqualStart<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqualEnd<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn CompareStart<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn CompareEnd<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdjustForInsert(&self, ec: u32, cchinsert: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(cchinsert), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(pgstart), ::core::mem::transmute(pgend)).ok()
    }
    pub unsafe fn SetGravity(&self, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(gstart), ::core::mem::transmute(gend)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ITfRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfRange>(result__)
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfContext>(result__)
    }
}
impl ::core::convert::From<ITfRange> for ::windows::core::IUnknown {
    fn from(value: ITfRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfRange> for ::windows::core::IUnknown {
    fn from(value: &ITfRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfRange {}
unsafe impl ::windows::core::Interface for ITfRange {
    type Vtable = ITfRangeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e7ff_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfRangeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, apos: TfAnchor) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfRangeACP(::windows::core::IUnknown);
impl ITfRangeACP {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), ::core::mem::transmute(pchtext), ::core::mem::transmute(cchmax), ::core::mem::transmute(pcch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ec: u32, dwflags: u32, pchtext: Param2, cch: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), pchtext.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self, ec: u32) -> ::windows::core::Result<super::super::System::Com::IDataObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IDataObject>(result__)
    }
    pub unsafe fn GetEmbedded(&self, ec: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(rguidservice), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<'a, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, ec: u32, dwflags: u32, pdataobject: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dwflags), pdataobject.into_param().abi()).ok()
    }
    pub unsafe fn ShiftStart(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(cchreq), ::core::mem::transmute(pcch), ::core::mem::transmute(phalt)).ok()
    }
    pub unsafe fn ShiftEnd(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(cchreq), ::core::mem::transmute(pcch), ::core::mem::transmute(phalt)).ok()
    }
    pub unsafe fn ShiftStartToRange<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, apos: TfAnchor) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(apos)).ok()
    }
    pub unsafe fn ShiftEndToRange<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1, apos: TfAnchor) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(apos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShiftStartRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dir), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShiftEndRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(dir), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEmpty(&self, ec: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Collapse(&self, ec: u32, apos: TfAnchor) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(apos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqualStart<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqualEnd<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn CompareStart<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn CompareEnd<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, pwith: Param1, apos: TfAnchor) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), pwith.into_param().abi(), ::core::mem::transmute(apos), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdjustForInsert(&self, ec: u32, cchinsert: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(cchinsert), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(pgstart), ::core::mem::transmute(pgend)).ok()
    }
    pub unsafe fn SetGravity(&self, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(gstart), ::core::mem::transmute(gend)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ITfRange> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfRange>(result__)
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfContext>(result__)
    }
    pub unsafe fn GetExtent(&self, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(pacpanchor), ::core::mem::transmute(pcch)).ok()
    }
    pub unsafe fn SetExtent(&self, acpanchor: i32, cch: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(acpanchor), ::core::mem::transmute(cch)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfRange> for ITfRangeACP {
    fn into_param(self) -> ::windows::core::Param<'a, ITfRange> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfRange> for &ITfRangeACP {
    fn into_param(self) -> ::windows::core::Param<'a, ITfRange> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfRangeACP> for ::windows::core::IUnknown {
    fn from(value: ITfRangeACP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfRangeACP> for ::windows::core::IUnknown {
    fn from(value: &ITfRangeACP) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfRangeACP {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfRangeACP {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfRangeACP {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfRangeACP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfRangeACP {}
unsafe impl ::windows::core::Interface for ITfRangeACP {
    type Vtable = ITfRangeACPVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x057a6296_029b_4154_b79a_0d461d4ea94c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfRangeACPVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: super::super::Foundation::PWSTR, cch: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ppdataobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, apos: TfAnchor) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, apos: TfAnchor) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pwith: ::windows::core::RawPtr, apos: TfAnchor, plresult: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpanchor: i32, cch: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfRangeBackup(::windows::core::IUnknown);
impl ITfRangeBackup {
    pub unsafe fn Restore<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITfRangeBackup> for ::windows::core::IUnknown {
    fn from(value: ITfRangeBackup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfRangeBackup> for ::windows::core::IUnknown {
    fn from(value: &ITfRangeBackup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfRangeBackup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfRangeBackup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfRangeBackup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfRangeBackup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfRangeBackup {}
unsafe impl ::windows::core::Interface for ITfRangeBackup {
    type Vtable = ITfRangeBackupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x463a506d_6992_49d2_9b88_93d55e70bb16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfRangeBackupVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ITfReadOnlyProperty(::windows::core::IUnknown);
impl ITfReadOnlyProperty {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn EnumRanges<'a, Param2: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, ppenum: *mut ::core::option::Option<IEnumTfRanges>, ptargetrange: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), ::core::mem::transmute(ppenum), ptargetrange.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue<'a, Param1: ::windows::core::IntoParam<'a, ITfRange>>(&self, ec: u32, prange: Param1) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ec), prange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfContext>(result__)
    }
}
impl ::core::convert::From<ITfReadOnlyProperty> for ::windows::core::IUnknown {
    fn from(value: ITfReadOnlyProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfReadOnlyProperty> for ::windows::core::IUnknown {
    fn from(value: &ITfReadOnlyProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfReadOnlyProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfReadOnlyProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfReadOnlyProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfReadOnlyProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReadOnlyProperty {}
unsafe impl ::windows::core::Interface for ITfReadOnlyProperty {
    type Vtable = ITfReadOnlyPropertyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17d49a3d_f8b8_4b2f_b254_52319dd64c53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReadOnlyPropertyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ppenum: *mut ::windows::core::RawPtr, ptargetrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: ::windows::core::RawPtr, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfReadingInformationUIElement(::windows::core::IUnknown);
impl ITfReadingInformationUIElement {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetUpdatedFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfContext>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetMaxReadingStringLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetErrorIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVerticalOrderPreferred(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITfUIElement> for ITfReadingInformationUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ITfUIElement> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfUIElement> for &ITfReadingInformationUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ITfUIElement> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfReadingInformationUIElement> for ::windows::core::IUnknown {
    fn from(value: ITfReadingInformationUIElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfReadingInformationUIElement> for ::windows::core::IUnknown {
    fn from(value: &ITfReadingInformationUIElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfReadingInformationUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfReadingInformationUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfReadingInformationUIElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfReadingInformationUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReadingInformationUIElement {}
unsafe impl ::windows::core::Interface for ITfReadingInformationUIElement {
    type Vtable = ITfReadingInformationUIElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea1ea139_19df_11d7_a6d2_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReadingInformationUIElementVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppic: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrorindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvertical: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfReverseConversion(::windows::core::IUnknown);
impl ITfReverseConversion {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoReverseConversion<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lpstr: Param0) -> ::windows::core::Result<ITfReverseConversionList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), lpstr.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfReverseConversionList>(result__)
    }
}
impl ::core::convert::From<ITfReverseConversion> for ::windows::core::IUnknown {
    fn from(value: ITfReverseConversion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfReverseConversion> for ::windows::core::IUnknown {
    fn from(value: &ITfReverseConversion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfReverseConversion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfReverseConversion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfReverseConversion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfReverseConversion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReverseConversion {}
unsafe impl ::windows::core::Interface for ITfReverseConversion {
    type Vtable = ITfReverseConversionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa415e162_157d_417d_8a8c_0ab26c7d2781);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReverseConversionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpstr: super::super::Foundation::PWSTR, pplist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfReverseConversionList(::windows::core::IUnknown);
impl ITfReverseConversionList {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetString(&self, uindex: u32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITfReverseConversionList> for ::windows::core::IUnknown {
    fn from(value: ITfReverseConversionList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfReverseConversionList> for ::windows::core::IUnknown {
    fn from(value: &ITfReverseConversionList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfReverseConversionList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfReverseConversionList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfReverseConversionList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfReverseConversionList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReverseConversionList {}
unsafe impl ::windows::core::Interface for ITfReverseConversionList {
    type Vtable = ITfReverseConversionListVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x151d69f0_86f4_4674_b721_56911e797f47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReverseConversionListVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pbstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfReverseConversionMgr(::windows::core::IUnknown);
impl ITfReverseConversionMgr {
    pub unsafe fn GetReverseConversion(&self, langid: u16, guidprofile: *const ::windows::core::GUID, dwflag: u32) -> ::windows::core::Result<ITfReverseConversion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(langid), ::core::mem::transmute(guidprofile), ::core::mem::transmute(dwflag), ::core::mem::transmute(&mut result__)).from_abi::<ITfReverseConversion>(result__)
    }
}
impl ::core::convert::From<ITfReverseConversionMgr> for ::windows::core::IUnknown {
    fn from(value: ITfReverseConversionMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfReverseConversionMgr> for ::windows::core::IUnknown {
    fn from(value: &ITfReverseConversionMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfReverseConversionMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfReverseConversionMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfReverseConversionMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfReverseConversionMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReverseConversionMgr {}
unsafe impl ::windows::core::Interface for ITfReverseConversionMgr {
    type Vtable = ITfReverseConversionMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb643c236_c493_41b6_abb3_692412775cc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReverseConversionMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, guidprofile: *const ::windows::core::GUID, dwflag: u32, ppreverseconversion: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfSource(::windows::core::IUnknown);
impl ITfSource {
    pub unsafe fn AdviseSink<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn UnadviseSink(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
}
impl ::core::convert::From<ITfSource> for ::windows::core::IUnknown {
    fn from(value: ITfSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfSource> for ::windows::core::IUnknown {
    fn from(value: &ITfSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSource {}
unsafe impl ::windows::core::Interface for ITfSource {
    type Vtable = ITfSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ea48a35_60ae_446f_8fd6_e6a8d82459f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfSourceSingle(::windows::core::IUnknown);
impl ITfSourceSingle {
    pub unsafe fn AdviseSingleSink<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, tid: u32, riid: *const ::windows::core::GUID, punk: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    pub unsafe fn UnadviseSingleSink(&self, tid: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), ::core::mem::transmute(riid)).ok()
    }
}
impl ::core::convert::From<ITfSourceSingle> for ::windows::core::IUnknown {
    fn from(value: ITfSourceSingle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfSourceSingle> for ::windows::core::IUnknown {
    fn from(value: &ITfSourceSingle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfSourceSingle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfSourceSingle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfSourceSingle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfSourceSingle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSourceSingle {}
unsafe impl ::windows::core::Interface for ITfSourceSingle {
    type Vtable = ITfSourceSingleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73131f9c_56a9_49dd_b0ee_d046633f7528);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSourceSingleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfSpeechUIServer(::windows::core::IUnknown);
impl ITfSpeechUIServer {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowUI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateBalloon<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, style: TfLBBalloonStyle, pch: Param1, cch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(style), pch.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
}
impl ::core::convert::From<ITfSpeechUIServer> for ::windows::core::IUnknown {
    fn from(value: ITfSpeechUIServer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfSpeechUIServer> for ::windows::core::IUnknown {
    fn from(value: &ITfSpeechUIServer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfSpeechUIServer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfSpeechUIServer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfSpeechUIServer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfSpeechUIServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSpeechUIServer {}
unsafe impl ::windows::core::Interface for ITfSpeechUIServer {
    type Vtable = ITfSpeechUIServerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90e9a944_9244_489f_a78f_de67afc013a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSpeechUIServerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfStatusSink(::windows::core::IUnknown);
impl ITfStatusSink {
    pub unsafe fn OnStatusChange<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>>(&self, pic: Param0, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<ITfStatusSink> for ::windows::core::IUnknown {
    fn from(value: ITfStatusSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfStatusSink> for ::windows::core::IUnknown {
    fn from(value: &ITfStatusSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfStatusSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfStatusSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfStatusSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfStatusSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfStatusSink {}
unsafe impl ::windows::core::Interface for ITfStatusSink {
    type Vtable = ITfStatusSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b7d8d73_b267_4f69_b32e_1ca321ce4f45);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfStatusSinkVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ITfSystemDeviceTypeLangBarItem(::windows::core::IUnknown);
impl ITfSystemDeviceTypeLangBarItem {
    pub unsafe fn SetIconMode(&self, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetIconMode(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ITfSystemDeviceTypeLangBarItem> for ::windows::core::IUnknown {
    fn from(value: ITfSystemDeviceTypeLangBarItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfSystemDeviceTypeLangBarItem> for ::windows::core::IUnknown {
    fn from(value: &ITfSystemDeviceTypeLangBarItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfSystemDeviceTypeLangBarItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfSystemDeviceTypeLangBarItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfSystemDeviceTypeLangBarItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfSystemDeviceTypeLangBarItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSystemDeviceTypeLangBarItem {}
unsafe impl ::windows::core::Interface for ITfSystemDeviceTypeLangBarItem {
    type Vtable = ITfSystemDeviceTypeLangBarItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45672eb9_9059_46a2_838d_4530355f6a77);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemDeviceTypeLangBarItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfSystemLangBarItem(::windows::core::IUnknown);
impl ITfSystemLangBarItem {
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetIcon<'a, Param0: ::windows::core::IntoParam<'a, super::WindowsAndMessaging::HICON>>(&self, hicon: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), hicon.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTooltipString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pchtooltip: Param0, cch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pchtooltip.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
}
impl ::core::convert::From<ITfSystemLangBarItem> for ::windows::core::IUnknown {
    fn from(value: ITfSystemLangBarItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfSystemLangBarItem> for ::windows::core::IUnknown {
    fn from(value: &ITfSystemLangBarItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfSystemLangBarItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfSystemLangBarItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfSystemLangBarItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfSystemLangBarItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSystemLangBarItem {}
unsafe impl ::windows::core::Interface for ITfSystemLangBarItem {
    type Vtable = ITfSystemLangBarItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e13e9ec_6b33_4d4a_b5eb_8a92f029f356);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemLangBarItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hicon: super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchtooltip: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfSystemLangBarItemSink(::windows::core::IUnknown);
impl ITfSystemLangBarItemSink {
    pub unsafe fn InitMenu<'a, Param0: ::windows::core::IntoParam<'a, ITfMenu>>(&self, pmenu: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pmenu.into_param().abi()).ok()
    }
    pub unsafe fn OnMenuSelect(&self, wid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wid)).ok()
    }
}
impl ::core::convert::From<ITfSystemLangBarItemSink> for ::windows::core::IUnknown {
    fn from(value: ITfSystemLangBarItemSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfSystemLangBarItemSink> for ::windows::core::IUnknown {
    fn from(value: &ITfSystemLangBarItemSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfSystemLangBarItemSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfSystemLangBarItemSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfSystemLangBarItemSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfSystemLangBarItemSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSystemLangBarItemSink {}
unsafe impl ::windows::core::Interface for ITfSystemLangBarItemSink {
    type Vtable = ITfSystemLangBarItemSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1449d9ab_13cf_4687_aa3e_8d8b18574396);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemLangBarItemSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmenu: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfSystemLangBarItemText(::windows::core::IUnknown);
impl ITfSystemLangBarItemText {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItemText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pch: Param0, cch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pch.into_param().abi(), ::core::mem::transmute(cch)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItemText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITfSystemLangBarItemText> for ::windows::core::IUnknown {
    fn from(value: ITfSystemLangBarItemText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfSystemLangBarItemText> for ::windows::core::IUnknown {
    fn from(value: &ITfSystemLangBarItemText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfSystemLangBarItemText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfSystemLangBarItemText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfSystemLangBarItemText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfSystemLangBarItemText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSystemLangBarItemText {}
unsafe impl ::windows::core::Interface for ITfSystemLangBarItemText {
    type Vtable = ITfSystemLangBarItemTextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c4ce0e5_ba49_4b52_ac6b_3b397b4f701f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemLangBarItemTextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pch: super::super::Foundation::PWSTR, cch: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfTextEditSink(::windows::core::IUnknown);
impl ITfTextEditSink {
    pub unsafe fn OnEndEdit<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>, Param2: ::windows::core::IntoParam<'a, ITfEditRecord>>(&self, pic: Param0, ecreadonly: u32, peditrecord: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(ecreadonly), peditrecord.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITfTextEditSink> for ::windows::core::IUnknown {
    fn from(value: ITfTextEditSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfTextEditSink> for ::windows::core::IUnknown {
    fn from(value: &ITfTextEditSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfTextEditSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfTextEditSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfTextEditSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfTextEditSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTextEditSink {}
unsafe impl ::windows::core::Interface for ITfTextEditSink {
    type Vtable = ITfTextEditSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8127d409_ccd3_4683_967a_b43d5b482bf7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextEditSinkVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, ecreadonly: u32, peditrecord: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ITfTextInputProcessor(::windows::core::IUnknown);
impl ITfTextInputProcessor {
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, ITfThreadMgr>>(&self, ptim: Param0, tid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptim.into_param().abi(), ::core::mem::transmute(tid)).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITfTextInputProcessor> for ::windows::core::IUnknown {
    fn from(value: ITfTextInputProcessor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfTextInputProcessor> for ::windows::core::IUnknown {
    fn from(value: &ITfTextInputProcessor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfTextInputProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfTextInputProcessor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfTextInputProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfTextInputProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTextInputProcessor {}
unsafe impl ::windows::core::Interface for ITfTextInputProcessor {
    type Vtable = ITfTextInputProcessorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e7f7_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextInputProcessorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptim: ::windows::core::RawPtr, tid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfTextInputProcessorEx(::windows::core::IUnknown);
impl ITfTextInputProcessorEx {
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, ITfThreadMgr>>(&self, ptim: Param0, tid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ptim.into_param().abi(), ::core::mem::transmute(tid)).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ActivateEx<'a, Param0: ::windows::core::IntoParam<'a, ITfThreadMgr>>(&self, ptim: Param0, tid: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ptim.into_param().abi(), ::core::mem::transmute(tid), ::core::mem::transmute(dwflags)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ITfTextInputProcessor> for ITfTextInputProcessorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ITfTextInputProcessor> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfTextInputProcessor> for &ITfTextInputProcessorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ITfTextInputProcessor> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfTextInputProcessorEx> for ::windows::core::IUnknown {
    fn from(value: ITfTextInputProcessorEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfTextInputProcessorEx> for ::windows::core::IUnknown {
    fn from(value: &ITfTextInputProcessorEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfTextInputProcessorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfTextInputProcessorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfTextInputProcessorEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfTextInputProcessorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTextInputProcessorEx {}
unsafe impl ::windows::core::Interface for ITfTextInputProcessorEx {
    type Vtable = ITfTextInputProcessorExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e4e2102_f9cd_433d_b496_303ce03a6507);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextInputProcessorExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptim: ::windows::core::RawPtr, tid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptim: ::windows::core::RawPtr, tid: u32, dwflags: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfTextLayoutSink(::windows::core::IUnknown);
impl ITfTextLayoutSink {
    pub unsafe fn OnLayoutChange<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>, Param2: ::windows::core::IntoParam<'a, ITfContextView>>(&self, pic: Param0, lcode: TfLayoutCode, pview: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(lcode), pview.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITfTextLayoutSink> for ::windows::core::IUnknown {
    fn from(value: ITfTextLayoutSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfTextLayoutSink> for ::windows::core::IUnknown {
    fn from(value: &ITfTextLayoutSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfTextLayoutSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfTextLayoutSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfTextLayoutSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfTextLayoutSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTextLayoutSink {}
unsafe impl ::windows::core::Interface for ITfTextLayoutSink {
    type Vtable = ITfTextLayoutSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2af2d06a_dd5b_4927_a0b4_54f19c91fade);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextLayoutSinkVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, lcode: TfLayoutCode, pview: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct ITfThreadFocusSink(::windows::core::IUnknown);
impl ITfThreadFocusSink {
    pub unsafe fn OnSetThreadFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnKillThreadFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITfThreadFocusSink> for ::windows::core::IUnknown {
    fn from(value: ITfThreadFocusSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfThreadFocusSink> for ::windows::core::IUnknown {
    fn from(value: &ITfThreadFocusSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfThreadFocusSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfThreadFocusSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfThreadFocusSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfThreadFocusSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadFocusSink {}
unsafe impl ::windows::core::Interface for ITfThreadFocusSink {
    type Vtable = ITfThreadFocusSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0f1db0c_3a20_405c_a303_96b6010a885f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadFocusSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfThreadMgr(::windows::core::IUnknown);
impl ITfThreadMgr {
    pub unsafe fn Activate(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CreateDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfDocumentMgr>(result__)
    }
    pub unsafe fn EnumDocumentMgrs(&self) -> ::windows::core::Result<IEnumTfDocumentMgrs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfDocumentMgrs>(result__)
    }
    pub unsafe fn GetFocus(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfDocumentMgr>(result__)
    }
    pub unsafe fn SetFocus<'a, Param0: ::windows::core::IntoParam<'a, ITfDocumentMgr>>(&self, pdimfocus: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pdimfocus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssociateFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ITfDocumentMgr>>(&self, hwnd: Param0, pdimnew: Param1) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), pdimnew.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfDocumentMgr>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThreadFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetFunctionProvider(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfFunctionProvider> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), ::core::mem::transmute(&mut result__)).from_abi::<ITfFunctionProvider>(result__)
    }
    pub unsafe fn EnumFunctionProviders(&self) -> ::windows::core::Result<IEnumTfFunctionProviders> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfFunctionProviders>(result__)
    }
    pub unsafe fn GetGlobalCompartment(&self) -> ::windows::core::Result<ITfCompartmentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfCompartmentMgr>(result__)
    }
}
impl ::core::convert::From<ITfThreadMgr> for ::windows::core::IUnknown {
    fn from(value: ITfThreadMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfThreadMgr> for ::windows::core::IUnknown {
    fn from(value: &ITfThreadMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfThreadMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfThreadMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfThreadMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfThreadMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadMgr {}
unsafe impl ::windows::core::Interface for ITfThreadMgr {
    type Vtable = ITfThreadMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e801_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdimfocus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdimnew: ::windows::core::RawPtr, ppdimprev: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, ppfuncprov: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcompmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfThreadMgr2(::windows::core::IUnknown);
impl ITfThreadMgr2 {
    pub unsafe fn Activate(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CreateDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfDocumentMgr>(result__)
    }
    pub unsafe fn EnumDocumentMgrs(&self) -> ::windows::core::Result<IEnumTfDocumentMgrs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfDocumentMgrs>(result__)
    }
    pub unsafe fn GetFocus(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfDocumentMgr>(result__)
    }
    pub unsafe fn SetFocus<'a, Param0: ::windows::core::IntoParam<'a, ITfDocumentMgr>>(&self, pdimfocus: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pdimfocus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThreadFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetFunctionProvider(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfFunctionProvider> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), ::core::mem::transmute(&mut result__)).from_abi::<ITfFunctionProvider>(result__)
    }
    pub unsafe fn EnumFunctionProviders(&self) -> ::windows::core::Result<IEnumTfFunctionProviders> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfFunctionProviders>(result__)
    }
    pub unsafe fn GetGlobalCompartment(&self) -> ::windows::core::Result<ITfCompartmentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfCompartmentMgr>(result__)
    }
    pub unsafe fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptid), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetActiveFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SuspendKeystrokeHandling(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResumeKeystrokeHandling(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ITfThreadMgr2> for ::windows::core::IUnknown {
    fn from(value: ITfThreadMgr2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfThreadMgr2> for ::windows::core::IUnknown {
    fn from(value: &ITfThreadMgr2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfThreadMgr2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfThreadMgr2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfThreadMgr2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfThreadMgr2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadMgr2 {}
unsafe impl ::windows::core::Interface for ITfThreadMgr2 {
    type Vtable = ITfThreadMgr2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ab198ef_6477_4ee8_8812_6780edb82d5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgr2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdimfocus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, ppfuncprov: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcompmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfThreadMgrEventSink(::windows::core::IUnknown);
impl ITfThreadMgrEventSink {
    pub unsafe fn OnInitDocumentMgr<'a, Param0: ::windows::core::IntoParam<'a, ITfDocumentMgr>>(&self, pdim: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pdim.into_param().abi()).ok()
    }
    pub unsafe fn OnUninitDocumentMgr<'a, Param0: ::windows::core::IntoParam<'a, ITfDocumentMgr>>(&self, pdim: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pdim.into_param().abi()).ok()
    }
    pub unsafe fn OnSetFocus<'a, Param0: ::windows::core::IntoParam<'a, ITfDocumentMgr>, Param1: ::windows::core::IntoParam<'a, ITfDocumentMgr>>(&self, pdimfocus: Param0, pdimprevfocus: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pdimfocus.into_param().abi(), pdimprevfocus.into_param().abi()).ok()
    }
    pub unsafe fn OnPushContext<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>>(&self, pic: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pic.into_param().abi()).ok()
    }
    pub unsafe fn OnPopContext<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>>(&self, pic: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pic.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ITfThreadMgrEventSink> for ::windows::core::IUnknown {
    fn from(value: ITfThreadMgrEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfThreadMgrEventSink> for ::windows::core::IUnknown {
    fn from(value: &ITfThreadMgrEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfThreadMgrEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfThreadMgrEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfThreadMgrEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfThreadMgrEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadMgrEventSink {}
unsafe impl ::windows::core::Interface for ITfThreadMgrEventSink {
    type Vtable = ITfThreadMgrEventSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e80e_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgrEventSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdim: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdim: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr, pdimprevfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfThreadMgrEx(::windows::core::IUnknown);
impl ITfThreadMgrEx {
    pub unsafe fn Activate(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CreateDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfDocumentMgr>(result__)
    }
    pub unsafe fn EnumDocumentMgrs(&self) -> ::windows::core::Result<IEnumTfDocumentMgrs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfDocumentMgrs>(result__)
    }
    pub unsafe fn GetFocus(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfDocumentMgr>(result__)
    }
    pub unsafe fn SetFocus<'a, Param0: ::windows::core::IntoParam<'a, ITfDocumentMgr>>(&self, pdimfocus: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pdimfocus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssociateFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ITfDocumentMgr>>(&self, hwnd: Param0, pdimnew: Param1) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), pdimnew.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ITfDocumentMgr>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThreadFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetFunctionProvider(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfFunctionProvider> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), ::core::mem::transmute(&mut result__)).from_abi::<ITfFunctionProvider>(result__)
    }
    pub unsafe fn EnumFunctionProviders(&self) -> ::windows::core::Result<IEnumTfFunctionProviders> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfFunctionProviders>(result__)
    }
    pub unsafe fn GetGlobalCompartment(&self) -> ::windows::core::Result<ITfCompartmentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfCompartmentMgr>(result__)
    }
    pub unsafe fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptid), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetActiveFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITfThreadMgr> for ITfThreadMgrEx {
    fn into_param(self) -> ::windows::core::Param<'a, ITfThreadMgr> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfThreadMgr> for &ITfThreadMgrEx {
    fn into_param(self) -> ::windows::core::Param<'a, ITfThreadMgr> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfThreadMgrEx> for ::windows::core::IUnknown {
    fn from(value: ITfThreadMgrEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfThreadMgrEx> for ::windows::core::IUnknown {
    fn from(value: &ITfThreadMgrEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfThreadMgrEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfThreadMgrEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfThreadMgrEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfThreadMgrEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadMgrEx {}
unsafe impl ::windows::core::Interface for ITfThreadMgrEx {
    type Vtable = ITfThreadMgrExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e90ade3_7594_4cb0_bb58_69628f5f458c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgrExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdimfocus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdimfocus: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdimnew: ::windows::core::RawPtr, ppdimprev: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, ppfuncprov: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcompmgr: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfToolTipUIElement(::windows::core::IUnknown);
impl ITfToolTipUIElement {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITfUIElement> for ITfToolTipUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ITfUIElement> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfUIElement> for &ITfToolTipUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ITfUIElement> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfToolTipUIElement> for ::windows::core::IUnknown {
    fn from(value: ITfToolTipUIElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfToolTipUIElement> for ::windows::core::IUnknown {
    fn from(value: &ITfToolTipUIElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfToolTipUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfToolTipUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfToolTipUIElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfToolTipUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfToolTipUIElement {}
unsafe impl ::windows::core::Interface for ITfToolTipUIElement {
    type Vtable = ITfToolTipUIElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52b18b5c_555d_46b2_b00a_fa680144fbdb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfToolTipUIElementVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfTransitoryExtensionSink(::windows::core::IUnknown);
impl ITfTransitoryExtensionSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTransitoryExtensionUpdated<'a, Param0: ::windows::core::IntoParam<'a, ITfContext>, Param2: ::windows::core::IntoParam<'a, ITfRange>, Param3: ::windows::core::IntoParam<'a, ITfRange>>(&self, pic: Param0, ecreadonly: u32, presultrange: Param2, pcompositionrange: Param3) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pic.into_param().abi(), ::core::mem::transmute(ecreadonly), presultrange.into_param().abi(), pcompositionrange.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ITfTransitoryExtensionSink> for ::windows::core::IUnknown {
    fn from(value: ITfTransitoryExtensionSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfTransitoryExtensionSink> for ::windows::core::IUnknown {
    fn from(value: &ITfTransitoryExtensionSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfTransitoryExtensionSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfTransitoryExtensionSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfTransitoryExtensionSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfTransitoryExtensionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTransitoryExtensionSink {}
unsafe impl ::windows::core::Interface for ITfTransitoryExtensionSink {
    type Vtable = ITfTransitoryExtensionSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa615096f_1c57_4813_8a15_55ee6e5a839c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTransitoryExtensionSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: ::windows::core::RawPtr, ecreadonly: u32, presultrange: ::windows::core::RawPtr, pcompositionrange: ::windows::core::RawPtr, pfdeleteresultrange: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfTransitoryExtensionUIElement(::windows::core::IUnknown);
impl ITfTransitoryExtensionUIElement {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITfDocumentMgr>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ITfUIElement> for ITfTransitoryExtensionUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ITfUIElement> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITfUIElement> for &ITfTransitoryExtensionUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ITfUIElement> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITfTransitoryExtensionUIElement> for ::windows::core::IUnknown {
    fn from(value: ITfTransitoryExtensionUIElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfTransitoryExtensionUIElement> for ::windows::core::IUnknown {
    fn from(value: &ITfTransitoryExtensionUIElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfTransitoryExtensionUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfTransitoryExtensionUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfTransitoryExtensionUIElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfTransitoryExtensionUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTransitoryExtensionUIElement {}
unsafe impl ::windows::core::Interface for ITfTransitoryExtensionUIElement {
    type Vtable = ITfTransitoryExtensionUIElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x858f956a_972f_42a2_a2f2_0321e1abe209);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTransitoryExtensionUIElementVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdim: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfUIElement(::windows::core::IUnknown);
impl ITfUIElement {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bshow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ITfUIElement> for ::windows::core::IUnknown {
    fn from(value: ITfUIElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfUIElement> for ::windows::core::IUnknown {
    fn from(value: &ITfUIElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfUIElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfUIElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfUIElement {}
unsafe impl ::windows::core::Interface for ITfUIElement {
    type Vtable = ITfUIElementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea1ea137_19df_11d7_a6d2_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfUIElementVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ITfUIElementMgr(::windows::core::IUnknown);
impl ITfUIElementMgr {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginUIElement<'a, Param0: ::windows::core::IntoParam<'a, ITfUIElement>>(&self, pelement: Param0, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pelement.into_param().abi(), ::core::mem::transmute(pbshow), ::core::mem::transmute(pdwuielementid)).ok()
    }
    pub unsafe fn UpdateUIElement(&self, dwuielementid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwuielementid)).ok()
    }
    pub unsafe fn EndUIElement(&self, dwuielementid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwuielementid)).ok()
    }
    pub unsafe fn GetUIElement(&self, dwuielementid: u32) -> ::windows::core::Result<ITfUIElement> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwuielementid), ::core::mem::transmute(&mut result__)).from_abi::<ITfUIElement>(result__)
    }
    pub unsafe fn EnumUIElements(&self) -> ::windows::core::Result<IEnumTfUIElements> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumTfUIElements>(result__)
    }
}
impl ::core::convert::From<ITfUIElementMgr> for ::windows::core::IUnknown {
    fn from(value: ITfUIElementMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfUIElementMgr> for ::windows::core::IUnknown {
    fn from(value: &ITfUIElementMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfUIElementMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfUIElementMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfUIElementMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfUIElementMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfUIElementMgr {}
unsafe impl ::windows::core::Interface for ITfUIElementMgr {
    type Vtable = ITfUIElementMgrVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea1ea135_19df_11d7_a6d2_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfUIElementMgrVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pelement: ::windows::core::RawPtr, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwuielementid: u32, ppelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ITfUIElementSink(::windows::core::IUnknown);
impl ITfUIElementSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginUIElement(&self, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwuielementid), ::core::mem::transmute(pbshow)).ok()
    }
    pub unsafe fn UpdateUIElement(&self, dwuielementid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwuielementid)).ok()
    }
    pub unsafe fn EndUIElement(&self, dwuielementid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwuielementid)).ok()
    }
}
impl ::core::convert::From<ITfUIElementSink> for ::windows::core::IUnknown {
    fn from(value: ITfUIElementSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITfUIElementSink> for ::windows::core::IUnknown {
    fn from(value: &ITfUIElementSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITfUIElementSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ITfUIElementSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITfUIElementSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITfUIElementSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfUIElementSink {}
unsafe impl ::windows::core::Interface for ITfUIElementSink {
    type Vtable = ITfUIElementSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea1ea136_19df_11d7_a6d2_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfUIElementSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IUIManagerEventSink(::windows::core::IUnknown);
impl IUIManagerEventSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowOpening(&self, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcbounds)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowOpened(&self, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcbounds)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowUpdating(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcupdatedbounds)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowUpdated(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(prcupdatedbounds)).ok()
    }
    pub unsafe fn OnWindowClosing(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnWindowClosed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IUIManagerEventSink> for ::windows::core::IUnknown {
    fn from(value: IUIManagerEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUIManagerEventSink> for ::windows::core::IUnknown {
    fn from(value: &IUIManagerEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUIManagerEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUIManagerEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUIManagerEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUIManagerEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIManagerEventSink {}
unsafe impl ::windows::core::Interface for IUIManagerEventSink {
    type Vtable = IUIManagerEventSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd91d690_a7e8_4265_9b38_8bb3bbaba7de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIManagerEventSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IVersionInfo(::windows::core::IUnknown);
impl IVersionInfo {
    pub unsafe fn GetSubcomponentCount(&self, ulsub: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsub), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetImplementationID(&self, ulsub: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsub), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetBuildVersion(&self, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsub), ::core::mem::transmute(pdwmajor), ::core::mem::transmute(pdwminor)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentDescription(&self, ulsub: u32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsub), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInstanceDescription(&self, ulsub: u32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsub), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IVersionInfo> for ::windows::core::IUnknown {
    fn from(value: IVersionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVersionInfo> for ::windows::core::IUnknown {
    fn from(value: &IVersionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVersionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVersionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVersionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVersionInfo {}
unsafe impl ::windows::core::Interface for IVersionInfo {
    type Vtable = IVersionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x401518ec_db00_4611_9b29_2a0e4b9afa85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVersionInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsub: u32, ulcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsub: u32, implid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[inline]
pub unsafe fn InitLocalMsCtfMonitor(dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitLocalMsCtfMonitor(dwflags: u32) -> ::windows::core::HRESULT;
        }
        InitLocalMsCtfMonitor(::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type InputScope = i32;
pub const IS_DEFAULT: InputScope = 0i32;
pub const IS_URL: InputScope = 1i32;
pub const IS_FILE_FULLFILEPATH: InputScope = 2i32;
pub const IS_FILE_FILENAME: InputScope = 3i32;
pub const IS_EMAIL_USERNAME: InputScope = 4i32;
pub const IS_EMAIL_SMTPEMAILADDRESS: InputScope = 5i32;
pub const IS_LOGINNAME: InputScope = 6i32;
pub const IS_PERSONALNAME_FULLNAME: InputScope = 7i32;
pub const IS_PERSONALNAME_PREFIX: InputScope = 8i32;
pub const IS_PERSONALNAME_GIVENNAME: InputScope = 9i32;
pub const IS_PERSONALNAME_MIDDLENAME: InputScope = 10i32;
pub const IS_PERSONALNAME_SURNAME: InputScope = 11i32;
pub const IS_PERSONALNAME_SUFFIX: InputScope = 12i32;
pub const IS_ADDRESS_FULLPOSTALADDRESS: InputScope = 13i32;
pub const IS_ADDRESS_POSTALCODE: InputScope = 14i32;
pub const IS_ADDRESS_STREET: InputScope = 15i32;
pub const IS_ADDRESS_STATEORPROVINCE: InputScope = 16i32;
pub const IS_ADDRESS_CITY: InputScope = 17i32;
pub const IS_ADDRESS_COUNTRYNAME: InputScope = 18i32;
pub const IS_ADDRESS_COUNTRYSHORTNAME: InputScope = 19i32;
pub const IS_CURRENCY_AMOUNTANDSYMBOL: InputScope = 20i32;
pub const IS_CURRENCY_AMOUNT: InputScope = 21i32;
pub const IS_DATE_FULLDATE: InputScope = 22i32;
pub const IS_DATE_MONTH: InputScope = 23i32;
pub const IS_DATE_DAY: InputScope = 24i32;
pub const IS_DATE_YEAR: InputScope = 25i32;
pub const IS_DATE_MONTHNAME: InputScope = 26i32;
pub const IS_DATE_DAYNAME: InputScope = 27i32;
pub const IS_DIGITS: InputScope = 28i32;
pub const IS_NUMBER: InputScope = 29i32;
pub const IS_ONECHAR: InputScope = 30i32;
pub const IS_PASSWORD: InputScope = 31i32;
pub const IS_TELEPHONE_FULLTELEPHONENUMBER: InputScope = 32i32;
pub const IS_TELEPHONE_COUNTRYCODE: InputScope = 33i32;
pub const IS_TELEPHONE_AREACODE: InputScope = 34i32;
pub const IS_TELEPHONE_LOCALNUMBER: InputScope = 35i32;
pub const IS_TIME_FULLTIME: InputScope = 36i32;
pub const IS_TIME_HOUR: InputScope = 37i32;
pub const IS_TIME_MINORSEC: InputScope = 38i32;
pub const IS_NUMBER_FULLWIDTH: InputScope = 39i32;
pub const IS_ALPHANUMERIC_HALFWIDTH: InputScope = 40i32;
pub const IS_ALPHANUMERIC_FULLWIDTH: InputScope = 41i32;
pub const IS_CURRENCY_CHINESE: InputScope = 42i32;
pub const IS_BOPOMOFO: InputScope = 43i32;
pub const IS_HIRAGANA: InputScope = 44i32;
pub const IS_KATAKANA_HALFWIDTH: InputScope = 45i32;
pub const IS_KATAKANA_FULLWIDTH: InputScope = 46i32;
pub const IS_HANJA: InputScope = 47i32;
pub const IS_HANGUL_HALFWIDTH: InputScope = 48i32;
pub const IS_HANGUL_FULLWIDTH: InputScope = 49i32;
pub const IS_SEARCH: InputScope = 50i32;
pub const IS_FORMULA: InputScope = 51i32;
pub const IS_SEARCH_INCREMENTAL: InputScope = 52i32;
pub const IS_CHINESE_HALFWIDTH: InputScope = 53i32;
pub const IS_CHINESE_FULLWIDTH: InputScope = 54i32;
pub const IS_NATIVE_SCRIPT: InputScope = 55i32;
pub const IS_YOMI: InputScope = 56i32;
pub const IS_TEXT: InputScope = 57i32;
pub const IS_CHAT: InputScope = 58i32;
pub const IS_NAME_OR_PHONENUMBER: InputScope = 59i32;
pub const IS_EMAILNAME_OR_ADDRESS: InputScope = 60i32;
pub const IS_PRIVATE: InputScope = 61i32;
pub const IS_MAPS: InputScope = 62i32;
pub const IS_NUMERIC_PASSWORD: InputScope = 63i32;
pub const IS_NUMERIC_PIN: InputScope = 64i32;
pub const IS_ALPHANUMERIC_PIN: InputScope = 65i32;
pub const IS_ALPHANUMERIC_PIN_SET: InputScope = 66i32;
pub const IS_FORMULA_NUMBER: InputScope = 67i32;
pub const IS_CHAT_WITHOUT_EMOJI: InputScope = 68i32;
pub const IS_PHRASELIST: InputScope = -1i32;
pub const IS_REGULAREXPRESSION: InputScope = -2i32;
pub const IS_SRGS: InputScope = -3i32;
pub const IS_XML: InputScope = -4i32;
pub const IS_ENUMSTRING: InputScope = -5i32;
pub type LANG_BAR_ITEM_ICON_MODE_FLAGS = u32;
pub const TF_DTLBI_NONE: LANG_BAR_ITEM_ICON_MODE_FLAGS = 0u32;
pub const TF_DTLBI_USEPROFILEICON: LANG_BAR_ITEM_ICON_MODE_FLAGS = 1u32;
pub const LIBID_MSAATEXTLib: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x150e2d7a_dac1_4582_947d_2a8fd78b82cd);
pub const MSAAControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08cd963f_7a3e_4f5c_9bd8_d692bb043c5b);
pub type TEXT_STORE_CHANGE_FLAGS = u32;
pub const TS_TC_NONE: TEXT_STORE_CHANGE_FLAGS = 0u32;
pub const TS_TC_CORRECTION: TEXT_STORE_CHANGE_FLAGS = 1u32;
pub type TEXT_STORE_LOCK_FLAGS = u32;
pub const TS_LF_READ: TEXT_STORE_LOCK_FLAGS = 2u32;
pub const TS_LF_READWRITE: TEXT_STORE_LOCK_FLAGS = 6u32;
pub type TEXT_STORE_TEXT_CHANGE_FLAGS = u32;
pub const TS_ST_NONE: TEXT_STORE_TEXT_CHANGE_FLAGS = 0u32;
pub const TS_ST_CORRECTION: TEXT_STORE_TEXT_CHANGE_FLAGS = 1u32;
pub const TF_CHAR_EMBEDDED: u32 = 65532u32;
pub const TF_CLUIE_COUNT: u32 = 2u32;
pub const TF_CLUIE_CURRENTPAGE: u32 = 32u32;
pub const TF_CLUIE_DOCUMENTMGR: u32 = 1u32;
pub const TF_CLUIE_PAGEINDEX: u32 = 16u32;
pub const TF_CLUIE_SELECTION: u32 = 4u32;
pub const TF_CLUIE_STRING: u32 = 8u32;
pub const TF_COMMANDING_ENABLED: u32 = 4u32;
pub const TF_COMMANDING_ON: u32 = 8u32;
pub type TF_CONTEXT_EDIT_CONTEXT_FLAGS = u32;
pub const TF_ES_ASYNCDONTCARE: TF_CONTEXT_EDIT_CONTEXT_FLAGS = 0u32;
pub const TF_ES_SYNC: TF_CONTEXT_EDIT_CONTEXT_FLAGS = 1u32;
pub const TF_ES_READ: TF_CONTEXT_EDIT_CONTEXT_FLAGS = 2u32;
pub const TF_ES_READWRITE: TF_CONTEXT_EDIT_CONTEXT_FLAGS = 6u32;
pub const TF_ES_ASYNC: TF_CONTEXT_EDIT_CONTEXT_FLAGS = 8u32;
pub const TF_CONVERSIONMODE_ALPHANUMERIC: u32 = 0u32;
pub const TF_CONVERSIONMODE_CHARCODE: u32 = 32u32;
pub const TF_CONVERSIONMODE_EUDC: u32 = 512u32;
pub const TF_CONVERSIONMODE_FIXED: u32 = 2048u32;
pub const TF_CONVERSIONMODE_FULLSHAPE: u32 = 8u32;
pub const TF_CONVERSIONMODE_KATAKANA: u32 = 2u32;
pub const TF_CONVERSIONMODE_NATIVE: u32 = 1u32;
pub const TF_CONVERSIONMODE_NOCONVERSION: u32 = 256u32;
pub const TF_CONVERSIONMODE_ROMAN: u32 = 16u32;
pub const TF_CONVERSIONMODE_SOFTKEYBOARD: u32 = 128u32;
pub const TF_CONVERSIONMODE_SYMBOL: u32 = 1024u32;
pub type TF_DA_ATTR_INFO = i32;
pub const TF_ATTR_INPUT: TF_DA_ATTR_INFO = 0i32;
pub const TF_ATTR_TARGET_CONVERTED: TF_DA_ATTR_INFO = 1i32;
pub const TF_ATTR_CONVERTED: TF_DA_ATTR_INFO = 2i32;
pub const TF_ATTR_TARGET_NOTCONVERTED: TF_DA_ATTR_INFO = 3i32;
pub const TF_ATTR_INPUT_ERROR: TF_DA_ATTR_INFO = 4i32;
pub const TF_ATTR_FIXEDCONVERTED: TF_DA_ATTR_INFO = 5i32;
pub const TF_ATTR_OTHER: TF_DA_ATTR_INFO = -1i32;
#[repr(C)]
pub struct TF_DA_COLOR {
    pub r#type: TF_DA_COLORTYPE,
    pub Anonymous: TF_DA_COLOR_0,
}
impl ::core::marker::Copy for TF_DA_COLOR {}
impl ::core::clone::Clone for TF_DA_COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TF_DA_COLOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TF_DA_COLOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TF_DA_COLOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for TF_DA_COLOR {}
impl ::core::default::Default for TF_DA_COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union TF_DA_COLOR_0 {
    pub nIndex: i32,
    pub cr: u32,
}
impl ::core::marker::Copy for TF_DA_COLOR_0 {}
impl ::core::clone::Clone for TF_DA_COLOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TF_DA_COLOR_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TF_DA_COLOR_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TF_DA_COLOR_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TF_DA_COLOR_0 {}
impl ::core::default::Default for TF_DA_COLOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type TF_DA_COLORTYPE = i32;
pub const TF_CT_NONE: TF_DA_COLORTYPE = 0i32;
pub const TF_CT_SYSCOLOR: TF_DA_COLORTYPE = 1i32;
pub const TF_CT_COLORREF: TF_DA_COLORTYPE = 2i32;
pub type TF_DA_LINESTYLE = i32;
pub const TF_LS_NONE: TF_DA_LINESTYLE = 0i32;
pub const TF_LS_SOLID: TF_DA_LINESTYLE = 1i32;
pub const TF_LS_DOT: TF_DA_LINESTYLE = 2i32;
pub const TF_LS_DASH: TF_DA_LINESTYLE = 3i32;
pub const TF_LS_SQUIGGLE: TF_DA_LINESTYLE = 4i32;
pub const TF_DICTATION_ENABLED: u32 = 2u32;
pub const TF_DICTATION_ON: u32 = 1u32;
pub const TF_DISABLE_BALLOON: u32 = 2u32;
pub const TF_DISABLE_COMMANDING: u32 = 4u32;
pub const TF_DISABLE_DICTATION: u32 = 2u32;
pub const TF_DISABLE_SPEECH: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_DISPLAYATTRIBUTE {
    pub crText: TF_DA_COLOR,
    pub crBk: TF_DA_COLOR,
    pub lsStyle: TF_DA_LINESTYLE,
    pub fBoldLine: super::super::Foundation::BOOL,
    pub crLine: TF_DA_COLOR,
    pub bAttr: TF_DA_ATTR_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_DISPLAYATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_DISPLAYATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TF_DISPLAYATTRIBUTE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_DISPLAYATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TF_DISPLAYATTRIBUTE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_DISPLAYATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_DISPLAYATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TF_E_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220218i32);
pub const TF_E_COMPOSITION_REJECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220216i32);
pub const TF_E_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220220i32);
pub const TF_E_EMPTYCONTEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220215i32);
pub const TF_E_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220982i32);
pub const TF_E_INVALIDPOINT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220985i32);
pub const TF_E_INVALIDPOS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220992i32);
pub const TF_E_INVALIDVIEW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220219i32);
pub const TF_E_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220224i32);
pub const TF_E_NOCONVERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219968i32);
pub const TF_E_NOINTERFACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220988i32);
pub const TF_E_NOLAYOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220986i32);
pub const TF_E_NOLOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220991i32);
pub const TF_E_NOOBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220990i32);
pub const TF_E_NOPROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220221i32);
pub const TF_E_NOSELECTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220987i32);
pub const TF_E_NOSERVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220989i32);
pub const TF_E_NOTOWNEDRANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220222i32);
pub const TF_E_RANGE_NOT_COVERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220217i32);
pub const TF_E_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220983i32);
pub const TF_E_STACKFULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220223i32);
pub const TF_E_SYNCHRONOUS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220984i32);
#[repr(C)]
pub struct TF_HALTCOND {
    pub pHaltRange: ::core::option::Option<ITfRange>,
    pub aHaltPos: TfAnchor,
    pub dwFlags: u32,
}
impl ::core::clone::Clone for TF_HALTCOND {
    fn clone(&self) -> Self {
        Self { pHaltRange: self.pHaltRange.clone(), aHaltPos: self.aHaltPos, dwFlags: self.dwFlags }
    }
}
unsafe impl ::windows::core::Abi for TF_HALTCOND {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for TF_HALTCOND {
    fn eq(&self, other: &Self) -> bool {
        self.pHaltRange == other.pHaltRange && self.aHaltPos == other.aHaltPos && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for TF_HALTCOND {}
impl ::core::default::Default for TF_HALTCOND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TF_HF_OBJECT: u32 = 1u32;
pub const TF_IE_CORRECTION: u32 = 1u32;
#[repr(C)]
pub struct TF_INPUTPROCESSORPROFILE {
    pub dwProfileType: u32,
    pub langid: u16,
    pub clsid: ::windows::core::GUID,
    pub guidProfile: ::windows::core::GUID,
    pub catid: ::windows::core::GUID,
    pub hklSubstitute: HKL,
    pub dwCaps: u32,
    pub hkl: HKL,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for TF_INPUTPROCESSORPROFILE {}
impl ::core::clone::Clone for TF_INPUTPROCESSORPROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TF_INPUTPROCESSORPROFILE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TF_INPUTPROCESSORPROFILE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TF_INPUTPROCESSORPROFILE>()) == 0 }
    }
}
impl ::core::cmp::Eq for TF_INPUTPROCESSORPROFILE {}
impl ::core::default::Default for TF_INPUTPROCESSORPROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TF_INVALID_COOKIE: u32 = 4294967295u32;
pub const TF_INVALID_EDIT_COOKIE: u32 = 0u32;
pub const TF_IPPMF_DISABLEPROFILE: u32 = 2u32;
pub const TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE: u32 = 4u32;
pub const TF_IPPMF_ENABLEPROFILE: u32 = 1u32;
pub const TF_IPPMF_FORPROCESS: u32 = 268435456u32;
pub const TF_IPPMF_FORSESSION: u32 = 536870912u32;
pub const TF_IPPMF_FORSYSTEMALL: u32 = 1073741824u32;
pub const TF_IPP_CAPS_COMLESSSUPPORT: u32 = 8u32;
pub const TF_IPP_CAPS_DISABLEONTRANSITORY: u32 = 1u32;
pub const TF_IPP_CAPS_IMMERSIVESUPPORT: u32 = 65536u32;
pub const TF_IPP_CAPS_SECUREMODESUPPORT: u32 = 2u32;
pub const TF_IPP_CAPS_SYSTRAYSUPPORT: u32 = 131072u32;
pub const TF_IPP_CAPS_UIELEMENTENABLED: u32 = 4u32;
pub const TF_IPP_CAPS_WOW16SUPPORT: u32 = 16u32;
pub const TF_IPP_FLAG_ACTIVE: u32 = 1u32;
pub const TF_IPP_FLAG_ENABLED: u32 = 2u32;
pub const TF_IPP_FLAG_SUBSTITUTEDBYINPUTPROCESSOR: u32 = 4u32;
pub const TF_IPSINK_FLAG_ACTIVE: u32 = 1u32;
#[repr(C)]
pub struct TF_LANGBARITEMINFO {
    pub clsidService: ::windows::core::GUID,
    pub guidItem: ::windows::core::GUID,
    pub dwStyle: u32,
    pub ulSort: u32,
    pub szDescription: [u16; 32],
}
impl ::core::marker::Copy for TF_LANGBARITEMINFO {}
impl ::core::clone::Clone for TF_LANGBARITEMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TF_LANGBARITEMINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TF_LANGBARITEMINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TF_LANGBARITEMINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TF_LANGBARITEMINFO {}
impl ::core::default::Default for TF_LANGBARITEMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_LANGUAGEPROFILE {
    pub clsid: ::windows::core::GUID,
    pub langid: u16,
    pub catid: ::windows::core::GUID,
    pub fActive: super::super::Foundation::BOOL,
    pub guidProfile: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_LANGUAGEPROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_LANGUAGEPROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TF_LANGUAGEPROFILE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_LANGUAGEPROFILE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TF_LANGUAGEPROFILE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_LANGUAGEPROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_LANGUAGEPROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_LBBALLOONINFO {
    pub style: TfLBBalloonStyle,
    pub bstrText: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_LBBALLOONINFO {
    fn clone(&self) -> Self {
        Self { style: self.style, bstrText: self.bstrText.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TF_LBBALLOONINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for TF_LBBALLOONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TF_LBI_BALLOON: u32 = 16u32;
pub const TF_LBI_BITMAP: u32 = 8u32;
pub const TF_LBI_BMPF_VERTICAL: u32 = 1u32;
pub const TF_LBI_CUSTOMUI: u32 = 32u32;
pub const TF_LBI_DESC_MAXLEN: u32 = 32u32;
pub const TF_LBI_ICON: u32 = 1u32;
pub const TF_LBI_STATUS: u32 = 65536u32;
pub const TF_LBI_STATUS_BTN_TOGGLED: u32 = 65536u32;
pub const TF_LBI_STATUS_DISABLED: u32 = 2u32;
pub const TF_LBI_STATUS_HIDDEN: u32 = 1u32;
pub const TF_LBI_STYLE_BTN_BUTTON: u32 = 65536u32;
pub const TF_LBI_STYLE_BTN_MENU: u32 = 131072u32;
pub const TF_LBI_STYLE_BTN_TOGGLE: u32 = 262144u32;
pub const TF_LBI_STYLE_HIDDENBYDEFAULT: u32 = 16u32;
pub const TF_LBI_STYLE_HIDDENSTATUSCONTROL: u32 = 1u32;
pub const TF_LBI_STYLE_HIDEONNOOTHERITEMS: u32 = 4u32;
pub const TF_LBI_STYLE_SHOWNINTRAY: u32 = 2u32;
pub const TF_LBI_STYLE_SHOWNINTRAYONLY: u32 = 8u32;
pub const TF_LBI_STYLE_TEXTCOLORICON: u32 = 32u32;
pub const TF_LBI_TEXT: u32 = 2u32;
pub const TF_LBI_TOOLTIP: u32 = 4u32;
pub const TF_LBMENUF_CHECKED: u32 = 1u32;
pub const TF_LBMENUF_GRAYED: u32 = 16u32;
pub const TF_LBMENUF_RADIOCHECKED: u32 = 8u32;
pub const TF_LBMENUF_SEPARATOR: u32 = 4u32;
pub const TF_LBMENUF_SUBMENU: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_LMLATTELEMENT {
    pub dwFrameStart: u32,
    pub dwFrameLen: u32,
    pub dwFlags: u32,
    pub Anonymous: TF_LMLATTELEMENT_0,
    pub bstrText: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_LMLATTELEMENT {
    fn clone(&self) -> Self {
        Self { dwFrameStart: self.dwFrameStart, dwFrameLen: self.dwFrameLen, dwFlags: self.dwFlags, Anonymous: self.Anonymous, bstrText: self.bstrText.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TF_LMLATTELEMENT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_LMLATTELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.dwFrameStart == other.dwFrameStart && self.dwFrameLen == other.dwFrameLen && self.dwFlags == other.dwFlags && self.Anonymous == other.Anonymous && self.bstrText == other.bstrText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_LMLATTELEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_LMLATTELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union TF_LMLATTELEMENT_0 {
    pub iCost: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_LMLATTELEMENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_LMLATTELEMENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TF_LMLATTELEMENT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_LMLATTELEMENT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TF_LMLATTELEMENT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_LMLATTELEMENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_LMLATTELEMENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TF_MENUREADY: u32 = 1u32;
pub const TF_MOD_ALT: u32 = 1u32;
pub const TF_MOD_CONTROL: u32 = 2u32;
pub const TF_MOD_IGNORE_ALL_MODIFIER: u32 = 1024u32;
pub const TF_MOD_LALT: u32 = 64u32;
pub const TF_MOD_LCONTROL: u32 = 128u32;
pub const TF_MOD_LSHIFT: u32 = 256u32;
pub const TF_MOD_ON_KEYUP: u32 = 512u32;
pub const TF_MOD_RALT: u32 = 8u32;
pub const TF_MOD_RCONTROL: u32 = 16u32;
pub const TF_MOD_RSHIFT: u32 = 32u32;
pub const TF_MOD_SHIFT: u32 = 4u32;
#[repr(C)]
pub struct TF_PERSISTENT_PROPERTY_HEADER_ACP {
    pub guidType: ::windows::core::GUID,
    pub ichStart: i32,
    pub cch: i32,
    pub cb: u32,
    pub dwPrivate: u32,
    pub clsidTIP: ::windows::core::GUID,
}
impl ::core::marker::Copy for TF_PERSISTENT_PROPERTY_HEADER_ACP {}
impl ::core::clone::Clone for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TF_PERSISTENT_PROPERTY_HEADER_ACP>()) == 0 }
    }
}
impl ::core::cmp::Eq for TF_PERSISTENT_PROPERTY_HEADER_ACP {}
impl ::core::default::Default for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TF_POPF_ALL: u32 = 1u32;
#[repr(C)]
pub struct TF_PRESERVEDKEY {
    pub uVKey: u32,
    pub uModifiers: u32,
}
impl ::core::marker::Copy for TF_PRESERVEDKEY {}
impl ::core::clone::Clone for TF_PRESERVEDKEY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TF_PRESERVEDKEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TF_PRESERVEDKEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TF_PRESERVEDKEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for TF_PRESERVEDKEY {}
impl ::core::default::Default for TF_PRESERVEDKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TF_PROFILETYPE_INPUTPROCESSOR: u32 = 1u32;
pub const TF_PROFILETYPE_KEYBOARDLAYOUT: u32 = 2u32;
pub const TF_PROFILE_ARRAY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd38eff65_aa46_4fd5_91a7_67845fb02f5b);
pub const TF_PROFILE_CANTONESE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aec109c_7e96_11d4_b2ef_0080c882687e);
pub const TF_PROFILE_CHANGJIE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bdf9f03_c7d3_11d4_b2ab_0080c882687e);
pub const TF_PROFILE_DAYI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x037b2c25_480c_4d7f_b027_d6ca6b69788a);
pub const TF_PROFILE_NEWCHANGJIE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3ba907a_6c7e_11d4_97fa_0080c882687e);
pub const TF_PROFILE_NEWPHONETIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2f9c502_1742_11d4_9790_0080c882687e);
pub const TF_PROFILE_NEWQUICK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b883ba0_c1c7_11d4_87f9_0080c882687e);
pub const TF_PROFILE_PHONETIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x761309de_317a_11d4_9b5d_0080c882687e);
pub const TF_PROFILE_PINYIN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3ba9077_6c7e_11d4_97fa_0080c882687e);
pub const TF_PROFILE_QUICK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6024b45f_5c54_11d4_b921_0080c882687e);
pub const TF_PROFILE_SIMPLEFAST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa550b04_5ad7_411f_a5ac_ca038ec515d7);
pub const TF_PROFILE_TIGRINYA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cab88b7_cc3e_46a6_9765_b772ad7761ff);
pub const TF_PROFILE_WUBI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82590c13_f4dd_44f4_ba1d_8667246fdf8e);
pub const TF_PROFILE_YI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x409c8376_007b_4357_ae8e_26316ee3fb0d);
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct TF_PROPERTYVAL {
    pub guidId: ::windows::core::GUID,
    pub varValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for TF_PROPERTYVAL {
    fn clone(&self) -> Self {
        Self { guidId: self.guidId, varValue: self.varValue.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for TF_PROPERTYVAL {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for TF_PROPERTYVAL {
    fn eq(&self, other: &Self) -> bool {
        self.guidId == other.guidId && self.varValue == other.varValue
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for TF_PROPERTYVAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for TF_PROPERTYVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TF_PROPUI_STATUS_SAVETOFILE: u32 = 1u32;
pub const TF_RCM_COMLESS: u32 = 1u32;
pub const TF_RCM_HINT_COLLISION: u32 = 8u32;
pub const TF_RCM_HINT_READING_LENGTH: u32 = 4u32;
pub const TF_RCM_VKEY: u32 = 2u32;
pub const TF_RIP_FLAG_FREEUNUSEDLIBRARIES: u32 = 1u32;
pub const TF_RIUIE_CONTEXT: u32 = 1u32;
pub const TF_RIUIE_ERRORINDEX: u32 = 8u32;
pub const TF_RIUIE_MAXREADINGSTRINGLENGTH: u32 = 4u32;
pub const TF_RIUIE_STRING: u32 = 2u32;
pub const TF_RIUIE_VERTICALORDER: u32 = 16u32;
pub const TF_RP_HIDDENINSETTINGUI: u32 = 2u32;
pub const TF_RP_LOCALPROCESS: u32 = 4u32;
pub const TF_RP_LOCALTHREAD: u32 = 8u32;
pub const TF_RP_SUBITEMINSETTINGUI: u32 = 16u32;
pub const TF_SD_LOADING: u32 = 2u32;
pub const TF_SD_READONLY: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_SELECTION {
    pub range: ::core::option::Option<ITfRange>,
    pub style: TF_SELECTIONSTYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_SELECTION {
    fn clone(&self) -> Self {
        Self { range: self.range.clone(), style: self.style }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TF_SELECTION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for TF_SELECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_SELECTIONSTYLE {
    pub ase: TfActiveSelEnd,
    pub fInterimChar: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_SELECTIONSTYLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TF_SELECTIONSTYLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_SELECTIONSTYLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TF_SELECTIONSTYLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_SELECTIONSTYLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TF_SENTENCEMODE_AUTOMATIC: u32 = 4u32;
pub const TF_SENTENCEMODE_CONVERSATION: u32 = 16u32;
pub const TF_SENTENCEMODE_NONE: u32 = 0u32;
pub const TF_SENTENCEMODE_PHRASEPREDICT: u32 = 8u32;
pub const TF_SENTENCEMODE_PLAURALCLAUSE: u32 = 1u32;
pub const TF_SENTENCEMODE_SINGLECONVERT: u32 = 2u32;
pub const TF_SFT_DESKBAND: u32 = 2048u32;
pub const TF_SFT_DOCK: u32 = 2u32;
pub const TF_SFT_EXTRAICONSONMINIMIZED: u32 = 512u32;
pub const TF_SFT_HIDDEN: u32 = 8u32;
pub const TF_SFT_HIGHTRANSPARENCY: u32 = 64u32;
pub const TF_SFT_LABELS: u32 = 128u32;
pub const TF_SFT_LOWTRANSPARENCY: u32 = 32u32;
pub const TF_SFT_MINIMIZED: u32 = 4u32;
pub const TF_SFT_NOEXTRAICONSONMINIMIZED: u32 = 1024u32;
pub const TF_SFT_NOLABELS: u32 = 256u32;
pub const TF_SFT_NOTRANSPARENCY: u32 = 16u32;
pub const TF_SFT_SHOWNORMAL: u32 = 1u32;
pub const TF_SHOW_BALLOON: u32 = 1u32;
pub const TF_SPEECHUI_SHOWN: u32 = 16u32;
pub const TF_SS_DISJOINTSEL: u32 = 1u32;
pub const TF_SS_REGIONS: u32 = 2u32;
pub const TF_SS_TKBAUTOCORRECTENABLE: u32 = 16u32;
pub const TF_SS_TKBPREDICTIONENABLE: u32 = 32u32;
pub const TF_SS_TRANSITORY: u32 = 4u32;
pub const TF_ST_CORRECTION: u32 = 1u32;
pub const TF_S_ASYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(262912i32);
pub const TF_TF_IGNOREEND: u32 = 2u32;
pub const TF_TF_MOVESTART: u32 = 1u32;
pub const TF_TMAE_COMLESS: u32 = 8u32;
pub const TF_TMAE_CONSOLE: u32 = 64u32;
pub const TF_TMAE_NOACTIVATEKEYBOARDLAYOUT: u32 = 32u32;
pub const TF_TMAE_NOACTIVATETIP: u32 = 1u32;
pub const TF_TMAE_SECUREMODE: u32 = 2u32;
pub const TF_TMAE_UIELEMENTENABLEDONLY: u32 = 4u32;
pub const TF_TMAE_WOW16: u32 = 16u32;
pub const TF_TMF_ACTIVATED: u32 = 2147483648u32;
pub const TF_TMF_COMLESS: u32 = 8u32;
pub const TF_TMF_CONSOLE: u32 = 64u32;
pub const TF_TMF_IMMERSIVEMODE: u32 = 1073741824u32;
pub const TF_TMF_NOACTIVATETIP: u32 = 1u32;
pub const TF_TMF_SECUREMODE: u32 = 2u32;
pub const TF_TMF_UIELEMENTENABLEDONLY: u32 = 4u32;
pub const TF_TMF_WOW16: u32 = 16u32;
pub const TF_TRANSITORYEXTENSION_ATSELECTION: u32 = 2u32;
pub const TF_TRANSITORYEXTENSION_FLOATING: u32 = 1u32;
pub const TF_TRANSITORYEXTENSION_NONE: u32 = 0u32;
pub const TF_TU_CORRECTION: u32 = 1u32;
pub const TF_URP_ALLPROFILES: u32 = 2u32;
pub const TF_URP_LOCALPROCESS: u32 = 4u32;
pub const TF_URP_LOCALTHREAD: u32 = 8u32;
pub const TF_US_HIDETIPUI: u32 = 1u32;
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_CHANGJIE: u32 = 61506u32;
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_DAYI: u32 = 61507u32;
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_PHONETIC: u32 = 1028u32;
pub const TKBL_OPT_JAPANESE_ABC: u32 = 1041u32;
pub const TKBL_OPT_KOREAN_HANGUL_2_BULSIK: u32 = 1042u32;
pub const TKBL_OPT_SIMPLIFIED_CHINESE_PINYIN: u32 = 2052u32;
pub const TKBL_OPT_TRADITIONAL_CHINESE_PHONETIC: u32 = 1028u32;
pub const TKBL_UNDEFINED: u32 = 0u32;
pub type TKBLayoutType = i32;
pub const TKBLT_UNDEFINED: TKBLayoutType = 0i32;
pub const TKBLT_CLASSIC: TKBLayoutType = 1i32;
pub const TKBLT_OPTIMIZED: TKBLayoutType = 2i32;
pub const TKB_ALTERNATES_AUTOCORRECTION_APPLIED: u32 = 4u32;
pub const TKB_ALTERNATES_FOR_AUTOCORRECTION: u32 = 2u32;
pub const TKB_ALTERNATES_FOR_PREDICTION: u32 = 3u32;
pub const TKB_ALTERNATES_STANDARD: u32 = 1u32;
pub const TSATTRID_App: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa80f77df_4237_40e5_849c_b5fa51c13ac7);
pub const TSATTRID_App_IncorrectGrammar: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd54e398_ad03_4b74_b6b3_5edb19996388);
pub const TSATTRID_App_IncorrectSpelling: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf42de43c_ef12_430d_944c_9a08970a25d2);
pub const TSATTRID_Font: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x573ea825_749b_4f8a_9cfd_21c3605ca828);
pub const TSATTRID_Font_FaceName: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb536aeb6_053b_4eb8_b65a_50da1e81e72e);
pub const TSATTRID_Font_SizePts: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8493302_a5e9_456d_af04_8005e4130f03);
pub const TSATTRID_Font_Style: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68b2a77f_6b0e_4f28_8177_571c2f3a42b1);
pub const TSATTRID_Font_Style_Animation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcf73d22_e029_47b7_bb36_f263a3d004cc);
pub const TSATTRID_Font_Style_Animation_BlinkingBackground: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86e5b104_0104_4b10_b585_00f2527522b5);
pub const TSATTRID_Font_Style_Animation_LasVegasLights: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf40423d5_0f87_4f8f_bada_e6d60c25e152);
pub const TSATTRID_Font_Style_Animation_MarchingBlackAnts: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7644e067_f186_4902_bfc6_ec815aa20e9d);
pub const TSATTRID_Font_Style_Animation_MarchingRedAnts: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78368dad_50fb_4c6f_840b_d486bb6cf781);
pub const TSATTRID_Font_Style_Animation_Shimmer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ce31b58_5293_4c36_8809_bf8bb51a27b3);
pub const TSATTRID_Font_Style_Animation_SparkleText: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x533aad20_962c_4e9f_8c09_b42ea4749711);
pub const TSATTRID_Font_Style_Animation_WipeDown: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5872e874_367b_4803_b160_c90ff62569d0);
pub const TSATTRID_Font_Style_Animation_WipeRight: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb855cbe3_3d2c_4600_b1e9_e1c9ce02f842);
pub const TSATTRID_Font_Style_BackgroundColor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb50eaa4e_3091_4468_81db_d79ea190c7c7);
pub const TSATTRID_Font_Style_Blink: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfb2c036_7acf_4532_b720_b416dd7765a8);
pub const TSATTRID_Font_Style_Bold: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48813a43_8a20_4940_8e58_97823f7b268a);
pub const TSATTRID_Font_Style_Capitalize: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d85a3ba_b4fd_43b3_befc_6b985c843141);
pub const TSATTRID_Font_Style_Color: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x857a7a37_b8af_4e9a_81b4_acf700c8411b);
pub const TSATTRID_Font_Style_Emboss: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd8ed742_349e_4e37_82fb_437979cb53a7);
pub const TSATTRID_Font_Style_Engrave: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c3371de_8332_4897_be5d_89233223179a);
pub const TSATTRID_Font_Style_Height: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e937477_12e6_458b_926a_1fa44ee8f391);
pub const TSATTRID_Font_Style_Hidden: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1e28770_881c_475f_863f_887a647b1090);
pub const TSATTRID_Font_Style_Italic: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8740682a_a765_48e1_acfc_d22222b2f810);
pub const TSATTRID_Font_Style_Kerning: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc26e1b4_2f9a_47c8_8bff_bf1eb7cce0dd);
pub const TSATTRID_Font_Style_Lowercase: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76d8ccb5_ca7b_4498_8ee9_d5c4f6f74c60);
pub const TSATTRID_Font_Style_Outlined: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10e6db31_db0d_4ac6_a7f5_9c9cff6f2ab4);
pub const TSATTRID_Font_Style_Overline: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3989f4a_992b_4301_8ce1_a5b7c6d1f3c8);
pub const TSATTRID_Font_Style_Overline_Double: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc46063a_e115_46e3_bcd8_ca6772aa95b4);
pub const TSATTRID_Font_Style_Overline_Single: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8440d94c_51ce_47b2_8d4c_15751e5f721b);
pub const TSATTRID_Font_Style_Position: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15cd26ab_f2fb_4062_b5a6_9a49e1a5cc0b);
pub const TSATTRID_Font_Style_Protected: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c557cb2_14cf_4554_a574_ecb2f7e7efd4);
pub const TSATTRID_Font_Style_Shadow: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f686d2f_c6cd_4c56_8a1a_994a4b9766be);
pub const TSATTRID_Font_Style_SmallCaps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfacb6bc6_9100_4cc6_b969_11eea45a86b4);
pub const TSATTRID_Font_Style_Spacing: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98c1200d_8f06_409a_8e49_6a554bf7c153);
pub const TSATTRID_Font_Style_Strikethrough: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c562193_2d08_4668_9601_ced41309d7af);
pub const TSATTRID_Font_Style_Strikethrough_Double: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62489b31_a3e7_4f94_ac43_ebaf8fcc7a9f);
pub const TSATTRID_Font_Style_Strikethrough_Single: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d736b6_3c8f_4b97_ab78_1877cb990d31);
pub const TSATTRID_Font_Style_Subscript: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5774fb84_389b_43bc_a74b_1568347cf0f4);
pub const TSATTRID_Font_Style_Superscript: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ea4993c_563c_49aa_9372_0bef09a9255b);
pub const TSATTRID_Font_Style_Underline: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3c9c9f3_7902_444b_9a7b_48e70f4b50f7);
pub const TSATTRID_Font_Style_Underline_Double: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74d24aa6_1db3_4c69_a176_31120e7586d5);
pub const TSATTRID_Font_Style_Underline_Single: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b6720e5_0f73_4951_a6b3_6f19e43c9461);
pub const TSATTRID_Font_Style_Uppercase: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33a300e8_e340_4937_b697_8f234045cd9a);
pub const TSATTRID_Font_Style_Weight: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12f3189c_8bb0_461b_b1fa_eaf907047fe0);
pub const TSATTRID_List: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x436d673b_26f1_4aee_9e65_8f83a4ed4884);
pub const TSATTRID_List_LevelIndel: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f7cc899_311f_487b_ad5d_e2a459e12d42);
pub const TSATTRID_List_Type: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae3e665e_4bce_49e3_a0fe_2db47d3a17ae);
pub const TSATTRID_List_Type_Arabic: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1338c5d6_98a3_4fa3_9bd1_7a60eef8e9e0);
pub const TSATTRID_List_Type_Bullet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbccd77c5_4c4d_4ce2_b102_559f3b2bfcea);
pub const TSATTRID_List_Type_LowerLetter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96372285_f3cf_491e_a925_3832347fd237);
pub const TSATTRID_List_Type_LowerRoman: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90466262_3980_4b8e_9368_918bd1218a41);
pub const TSATTRID_List_Type_UpperLetter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7987b7cd_ce52_428b_9b95_a357f6f10c45);
pub const TSATTRID_List_Type_UpperRoman: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f6ab552_4a80_467f_b2f1_127e2aa3ba9e);
pub const TSATTRID_OTHERS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3c32af9_57d0_46a9_bca8_dac238a13057);
pub const TSATTRID_Text: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7edb8e68_81f9_449d_a15a_87a8388faac0);
pub const TSATTRID_Text_Alignment: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x139941e6_1767_456d_938e_35ba568b5cd4);
pub const TSATTRID_Text_Alignment_Center: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4a95c16_53bf_4d55_8b87_4bdd8d4275fc);
pub const TSATTRID_Text_Alignment_Justify: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed350740_a0f7_42d3_8ea8_f81b6488faf0);
pub const TSATTRID_Text_Alignment_Left: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16ae95d3_6361_43a2_8495_d00f397f1693);
pub const TSATTRID_Text_Alignment_Right: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb36f0f98_1b9e_4360_8616_03fb08a78456);
pub const TSATTRID_Text_EmbeddedObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7edb8e68_81f9_449d_a15a_87a8388faac0);
pub const TSATTRID_Text_Hyphenation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdadf4525_618e_49eb_b1a8_3b68bd7648e3);
pub const TSATTRID_Text_Language: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8c04ef1_5753_4c25_8887_85443fe5f819);
pub const TSATTRID_Text_Link: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47cd9051_3722_4cd8_b7c8_4e17ca1759f5);
pub const TSATTRID_Text_Orientation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bab707f_8785_4c39_8b52_96f878303ffb);
pub const TSATTRID_Text_Para: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5edc5822_99dc_4dd6_aec3_b62baa5b2e7c);
pub const TSATTRID_Text_Para_FirstLineIndent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07c97a13_7472_4dd8_90a9_91e3d7e4f29c);
pub const TSATTRID_Text_Para_LeftIndent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb2848e9_7471_41c9_b6b3_8a1450e01897);
pub const TSATTRID_Text_Para_LineSpacing: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x699b380d_7f8c_46d6_a73b_dfe3d1538df3);
pub const TSATTRID_Text_Para_LineSpacing_AtLeast: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadfedf31_2d44_4434_a5ff_7f4c4990a905);
pub const TSATTRID_Text_Para_LineSpacing_Double: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82fb1805_a6c4_4231_ac12_6260af2aba28);
pub const TSATTRID_Text_Para_LineSpacing_Exactly: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d45ad40_23de_48d7_a6b3_765420c620cc);
pub const TSATTRID_Text_Para_LineSpacing_Multiple: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x910f1e3c_d6d0_4f65_8a3c_42b4b31868c5);
pub const TSATTRID_Text_Para_LineSpacing_OnePtFive: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0428a021_0397_4b57_9a17_0795994cd3c5);
pub const TSATTRID_Text_Para_LineSpacing_Single: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed350740_a0f7_42d3_8ea8_f81b6488faf0);
pub const TSATTRID_Text_Para_RightIndent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c7f26f9_a5e2_48da_b98a_520cb16513bf);
pub const TSATTRID_Text_Para_SpaceAfter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b0a3f55_22dc_425f_a411_93da1d8f9baa);
pub const TSATTRID_Text_Para_SpaceBefore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8df98589_194a_4601_b251_9865a3e906dd);
pub const TSATTRID_Text_ReadOnly: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85836617_de32_4afd_a50f_a2db110e6e4d);
pub const TSATTRID_Text_RightToLeft: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca666e71_1b08_453d_bfdd_28e08c8aaf7a);
pub const TSATTRID_Text_VerticalWriting: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bba8195_046f_4ea9_b311_97fd66c4274b);
pub const TS_AS_ATTR_CHANGE: u32 = 8u32;
pub const TS_AS_LAYOUT_CHANGE: u32 = 4u32;
pub const TS_AS_SEL_CHANGE: u32 = 2u32;
pub const TS_AS_STATUS_CHANGE: u32 = 16u32;
pub const TS_AS_TEXT_CHANGE: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct TS_ATTRVAL {
    pub idAttr: ::windows::core::GUID,
    pub dwOverlapId: u32,
    pub varValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for TS_ATTRVAL {
    fn clone(&self) -> Self {
        Self { idAttr: self.idAttr, dwOverlapId: self.dwOverlapId, varValue: self.varValue.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for TS_ATTRVAL {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for TS_ATTRVAL {
    fn eq(&self, other: &Self) -> bool {
        self.idAttr == other.idAttr && self.dwOverlapId == other.dwOverlapId && self.varValue == other.varValue
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for TS_ATTRVAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for TS_ATTRVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TS_ATTR_FIND_BACKWARDS: u32 = 1u32;
pub const TS_ATTR_FIND_HIDDEN: u32 = 32u32;
pub const TS_ATTR_FIND_UPDATESTART: u32 = 4u32;
pub const TS_ATTR_FIND_WANT_END: u32 = 16u32;
pub const TS_ATTR_FIND_WANT_OFFSET: u32 = 2u32;
pub const TS_ATTR_FIND_WANT_VALUE: u32 = 8u32;
pub const TS_CHAR_EMBEDDED: u32 = 65532u32;
pub const TS_CHAR_REGION: u32 = 0u32;
pub const TS_CHAR_REPLACEMENT: u32 = 65533u32;
pub const TS_E_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220982i32);
pub const TS_E_INVALIDPOINT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220985i32);
pub const TS_E_INVALIDPOS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220992i32);
pub const TS_E_NOINTERFACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220988i32);
pub const TS_E_NOLAYOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220986i32);
pub const TS_E_NOLOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220991i32);
pub const TS_E_NOOBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220990i32);
pub const TS_E_NOSELECTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220987i32);
pub const TS_E_NOSERVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220989i32);
pub const TS_E_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220983i32);
pub const TS_E_SYNCHRONOUS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220984i32);
pub const TS_GEA_HIDDEN: u32 = 1u32;
pub const TS_GTA_HIDDEN: u32 = 1u32;
pub const TS_IAS_NOQUERY: u32 = 1u32;
pub const TS_IAS_QUERYONLY: u32 = 2u32;
pub const TS_IE_COMPOSITION: u32 = 2u32;
pub const TS_IE_CORRECTION: u32 = 1u32;
pub const TS_LF_SYNC: u32 = 1u32;
#[repr(C)]
pub struct TS_RUNINFO {
    pub uCount: u32,
    pub r#type: TsRunType,
}
impl ::core::marker::Copy for TS_RUNINFO {}
impl ::core::clone::Clone for TS_RUNINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TS_RUNINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TS_RUNINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TS_RUNINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TS_RUNINFO {}
impl ::core::default::Default for TS_RUNINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_ENABLED: u32 = 128u32;
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_VISIBLE: u32 = 256u32;
pub const TS_SD_INPUTPANEMANUALDISPLAYENABLE: u32 = 64u32;
pub const TS_SD_LOADING: u32 = 2u32;
pub const TS_SD_READONLY: u32 = 1u32;
pub const TS_SD_RESERVED: u32 = 4u32;
pub const TS_SD_TKBAUTOCORRECTENABLE: u32 = 8u32;
pub const TS_SD_TKBPREDICTIONENABLE: u32 = 16u32;
pub const TS_SD_UIINTEGRATIONENABLE: u32 = 32u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TS_SELECTIONSTYLE {
    pub ase: TsActiveSelEnd,
    pub fInterimChar: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TS_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TS_SELECTIONSTYLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TS_SELECTIONSTYLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TS_SELECTIONSTYLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TS_SELECTIONSTYLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TS_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TS_SELECTIONSTYLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TS_SELECTION_ACP {
    pub acpStart: i32,
    pub acpEnd: i32,
    pub style: TS_SELECTIONSTYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TS_SELECTION_ACP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TS_SELECTION_ACP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TS_SELECTION_ACP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TS_SELECTION_ACP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TS_SELECTION_ACP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TS_SELECTION_ACP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TS_SELECTION_ACP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TS_SELECTION_ANCHOR {
    pub paStart: ::core::option::Option<IAnchor>,
    pub paEnd: ::core::option::Option<IAnchor>,
    pub style: TS_SELECTIONSTYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TS_SELECTION_ANCHOR {
    fn clone(&self) -> Self {
        Self { paStart: self.paStart.clone(), paEnd: self.paEnd.clone(), style: self.style }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TS_SELECTION_ANCHOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for TS_SELECTION_ANCHOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TS_SHIFT_COUNT_HIDDEN: u32 = 1u32;
pub const TS_SHIFT_COUNT_ONLY: u32 = 8u32;
pub const TS_SHIFT_HALT_HIDDEN: u32 = 2u32;
pub const TS_SHIFT_HALT_VISIBLE: u32 = 4u32;
pub const TS_SS_DISJOINTSEL: u32 = 1u32;
pub const TS_SS_NOHIDDENTEXT: u32 = 8u32;
pub const TS_SS_REGIONS: u32 = 2u32;
pub const TS_SS_TKBAUTOCORRECTENABLE: u32 = 16u32;
pub const TS_SS_TKBPREDICTIONENABLE: u32 = 32u32;
pub const TS_SS_TRANSITORY: u32 = 4u32;
pub const TS_SS_UWPCONTROL: u32 = 64u32;
#[repr(C)]
pub struct TS_STATUS {
    pub dwDynamicFlags: u32,
    pub dwStaticFlags: u32,
}
impl ::core::marker::Copy for TS_STATUS {}
impl ::core::clone::Clone for TS_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TS_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TS_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TS_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for TS_STATUS {}
impl ::core::default::Default for TS_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TS_STRF_END: u32 = 2u32;
pub const TS_STRF_MID: u32 = 1u32;
pub const TS_STRF_START: u32 = 0u32;
pub const TS_S_ASYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(262912i32);
#[repr(C)]
pub struct TS_TEXTCHANGE {
    pub acpStart: i32,
    pub acpOldEnd: i32,
    pub acpNewEnd: i32,
}
impl ::core::marker::Copy for TS_TEXTCHANGE {}
impl ::core::clone::Clone for TS_TEXTCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TS_TEXTCHANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TS_TEXTCHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TS_TEXTCHANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for TS_TEXTCHANGE {}
impl ::core::default::Default for TS_TEXTCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TS_VCOOKIE_NUL: u32 = 4294967295u32;
pub type TfActiveSelEnd = i32;
pub const TF_AE_NONE: TfActiveSelEnd = 0i32;
pub const TF_AE_START: TfActiveSelEnd = 1i32;
pub const TF_AE_END: TfActiveSelEnd = 2i32;
pub type TfAnchor = i32;
pub const TF_ANCHOR_START: TfAnchor = 0i32;
pub const TF_ANCHOR_END: TfAnchor = 1i32;
pub type TfCandidateResult = i32;
pub const CAND_FINALIZED: TfCandidateResult = 0i32;
pub const CAND_SELECTED: TfCandidateResult = 1i32;
pub const CAND_CANCELED: TfCandidateResult = 2i32;
pub type TfGravity = i32;
pub const TF_GRAVITY_BACKWARD: TfGravity = 0i32;
pub const TF_GRAVITY_FORWARD: TfGravity = 1i32;
pub type TfIntegratableCandidateListSelectionStyle = i32;
pub const STYLE_ACTIVE_SELECTION: TfIntegratableCandidateListSelectionStyle = 0i32;
pub const STYLE_IMPLIED_SELECTION: TfIntegratableCandidateListSelectionStyle = 1i32;
pub type TfLBBalloonStyle = i32;
pub const TF_LB_BALLOON_RECO: TfLBBalloonStyle = 0i32;
pub const TF_LB_BALLOON_SHOW: TfLBBalloonStyle = 1i32;
pub const TF_LB_BALLOON_MISS: TfLBBalloonStyle = 2i32;
pub type TfLBIClick = i32;
pub const TF_LBI_CLK_RIGHT: TfLBIClick = 1i32;
pub const TF_LBI_CLK_LEFT: TfLBIClick = 2i32;
pub type TfLayoutCode = i32;
pub const TF_LC_CREATE: TfLayoutCode = 0i32;
pub const TF_LC_CHANGE: TfLayoutCode = 1i32;
pub const TF_LC_DESTROY: TfLayoutCode = 2i32;
pub type TfSapiObject = i32;
pub const GETIF_RESMGR: TfSapiObject = 0i32;
pub const GETIF_RECOCONTEXT: TfSapiObject = 1i32;
pub const GETIF_RECOGNIZER: TfSapiObject = 2i32;
pub const GETIF_VOICE: TfSapiObject = 3i32;
pub const GETIF_DICTGRAM: TfSapiObject = 4i32;
pub const GETIF_RECOGNIZERNOINIT: TfSapiObject = 5i32;
pub type TfShiftDir = i32;
pub const TF_SD_BACKWARD: TfShiftDir = 0i32;
pub const TF_SD_FORWARD: TfShiftDir = 1i32;
pub type TsActiveSelEnd = i32;
pub const TS_AE_NONE: TsActiveSelEnd = 0i32;
pub const TS_AE_START: TsActiveSelEnd = 1i32;
pub const TS_AE_END: TsActiveSelEnd = 2i32;
pub type TsGravity = i32;
pub const TS_GR_BACKWARD: TsGravity = 0i32;
pub const TS_GR_FORWARD: TsGravity = 1i32;
pub type TsLayoutCode = i32;
pub const TS_LC_CREATE: TsLayoutCode = 0i32;
pub const TS_LC_CHANGE: TsLayoutCode = 1i32;
pub const TS_LC_DESTROY: TsLayoutCode = 2i32;
pub type TsRunType = i32;
pub const TS_RT_PLAIN: TsRunType = 0i32;
pub const TS_RT_HIDDEN: TsRunType = 1i32;
pub const TS_RT_OPAQUE: TsRunType = 2i32;
pub type TsShiftDir = i32;
pub const TS_SD_BACKWARD: TsShiftDir = 0i32;
pub const TS_SD_FORWARD: TsShiftDir = 1i32;
#[inline]
pub unsafe fn UninitLocalMsCtfMonitor() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UninitLocalMsCtfMonitor() -> ::windows::core::HRESULT;
        }
        UninitLocalMsCtfMonitor().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}

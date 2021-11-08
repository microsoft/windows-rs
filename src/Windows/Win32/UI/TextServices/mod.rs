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
pub const AccClientDocMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4232629296, 20286, 20385, [128, 59, 173, 14, 25, 106, 131, 177]);
pub const AccDictionary: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1702030870, 24549, 17201, [187, 109, 118, 164, 156, 86, 228, 35]);
pub const AccServerDocMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1619633022, 60298, 18477, [189, 111, 249, 244, 105, 4, 209, 109]);
pub const AccStore: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1413514111, 19455, 19173, [161, 177, 119, 34, 236, 198, 51, 42]);
pub const CLSID_TF_CategoryMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2763343009, 17293, 19265, [147, 37, 134, 149, 35, 226, 214, 199]);
pub const CLSID_TF_ClassicLangBar: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(857224716, 6908, 19721, [168, 107, 159, 156, 182, 220, 235, 156]);
pub const CLSID_TF_DisplayAttributeMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1021791716, 21459, 19828, [139, 131, 67, 27, 56, 40, 186, 83]);
pub const CLSID_TF_InputProcessorProfiles: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(868563536, 62550, 18564, [176, 73, 133, 253, 100, 62, 207, 237]);
pub const CLSID_TF_LangBarItemMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3113424530, 41651, 20395, [191, 51, 158, 198, 249, 251, 150, 172]);
pub const CLSID_TF_LangBarMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3954216005, 27722, 20444, [174, 83, 78, 184, 196, 199, 219, 142]);
pub const CLSID_TF_ThreadMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1385864811, 25991, 20259, [171, 158, 156, 125, 104, 62, 60, 80]);
pub const CLSID_TF_TransitoryExtensionUIEntry: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2926305288, 2043, 16397, [139, 235, 51, 122, 100, 247, 5, 31]);
pub const CLSID_TsfServices: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(967760896, 27488, 18139, [141, 49, 54, 66, 190, 14, 67, 115]);
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
pub const DocWrap: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3208802174, 31326, 17622, [131, 12, 163, 144, 234, 148, 98, 163]);
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
pub const GUID_APP_FUNCTIONPROVIDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1286533150, 4783, 19214, [157, 177, 166, 236, 91, 136, 18, 8]);
pub const GUID_COMPARTMENT_CONVERSIONMODEBIAS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1419244822, 61073, 17262, [185, 70, 170, 44, 5, 241, 172, 91]);
pub const GUID_COMPARTMENT_EMPTYCONTEXT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3611852223, 32846, 16837, [137, 77, 173, 150, 253, 78, 234, 19]);
pub const GUID_COMPARTMENT_ENABLED_PROFILES_UPDATED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2462186824, 43438, 19068, [190, 8, 67, 41, 228, 114, 56, 23]);
pub const GUID_COMPARTMENT_HANDWRITING_OPENCLOSE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4188941419, 6246, 17249, [175, 114, 122, 163, 9, 72, 137, 14]);
pub const GUID_COMPARTMENT_KEYBOARD_DISABLED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1906684499, 6481, 18027, [159, 188, 156, 136, 8, 250, 132, 242]);
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3059295505, 48366, 16674, [167, 196, 9, 244, 179, 250, 67, 150]);
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE_CONVERSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3438304728, 19079, 4567, [166, 226, 0, 6, 91, 132, 67, 92]);
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE_SENTENCE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3438304729, 19079, 4567, [166, 226, 0, 6, 91, 132, 67, 92]);
pub const GUID_COMPARTMENT_KEYBOARD_OPENCLOSE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1478965933, 443, 16740, [149, 198, 117, 91, 160, 181, 22, 45]);
pub const GUID_COMPARTMENT_SAPI_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1370431622, 52331, 17789, [181, 170, 139, 25, 220, 41, 10, 180]);
pub const GUID_COMPARTMENT_SPEECH_CFGMENU: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218182701, 20099, 19382, [145, 162, 224, 25, 191, 246, 118, 45]);
pub const GUID_COMPARTMENT_SPEECH_DISABLED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1455801863, 1795, 20057, [142, 82, 203, 200, 78, 139, 190, 53]);
pub const GUID_COMPARTMENT_SPEECH_GLOBALSTATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(710213262, 3336, 17932, [167, 93, 135, 3, 95, 244, 54, 197]);
pub const GUID_COMPARTMENT_SPEECH_OPENCLOSE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1414359651, 58088, 18258, [187, 209, 0, 9, 96, 188, 160, 131]);
pub const GUID_COMPARTMENT_SPEECH_UI_STATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3642758896, 37735, 20455, [154, 191, 188, 89, 218, 203, 224, 227]);
pub const GUID_COMPARTMENT_TIPUISTATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(344761324, 870, 16412, [141, 117, 237, 151, 141, 133, 251, 201]);
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2346928117, 51104, 4567, [180, 8, 0, 6, 91, 132, 67, 92]);
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION_DOCUMENTMANAGER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2346928119, 51104, 4567, [180, 8, 0, 6, 91, 132, 67, 92]);
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION_PARENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2346928120, 51104, 4567, [180, 8, 0, 6, 91, 132, 67, 92]);
pub const GUID_INTEGRATIONSTYLE_SEARCHBOX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3872505105, 33527, 18691, [174, 33, 26, 99, 151, 205, 226, 235]);
pub const GUID_LBI_INPUTMODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(746039326, 16844, 16760, [163, 167, 95, 138, 152, 117, 104, 230]);
pub const GUID_LBI_SAPILAYR_CFGMENUBUTTON: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3492750497, 37933, 16942, [141, 153, 180, 242, 173, 222, 233, 153]);
pub const GUID_MODEBIAS_CHINESE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2061313758, 17192, 18587, [131, 174, 100, 147, 117, 12, 173, 92]);
pub const GUID_MODEBIAS_CONVERSATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(256819460, 6032, 17467, [149, 241, 225, 15, 147, 157, 101, 70]);
pub const GUID_MODEBIAS_DATETIME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4072518514, 32609, 16441, [146, 239, 28, 53, 89, 159, 2, 34]);
pub const GUID_MODEBIAS_FILENAME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3623290878, 17606, 20426, [142, 118, 134, 171, 80, 199, 147, 27]);
pub const GUID_MODEBIAS_FULLWIDTHALPHANUMERIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2169020344, 45930, 18237, [129, 70, 228, 162, 37, 139, 36, 174]);
pub const GUID_MODEBIAS_FULLWIDTHHANGUL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3222988489, 17845, 20432, [156, 177, 159, 76, 235, 195, 159, 234]);
pub const GUID_MODEBIAS_HALFWIDTHKATAKANA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(6253411, 30932, 16844, [136, 89, 72, 92, 168, 33, 167, 149]);
pub const GUID_MODEBIAS_HANGUL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1995375937, 9139, 19831, [160, 116, 105, 24, 1, 204, 234, 23]);
pub const GUID_MODEBIAS_HIRAGANA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3611111790, 39825, 18161, [162, 128, 49, 89, 127, 82, 198, 148]);
pub const GUID_MODEBIAS_KATAKANA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(772730333, 14874, 18846, [133, 67, 60, 126, 231, 148, 152, 17]);
pub const GUID_MODEBIAS_NAME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4259057904, 53817, 18879, [184, 252, 84, 16, 202, 170, 66, 126]);
pub const GUID_MODEBIAS_NONE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(0, 0, 0, [0, 0, 0, 0, 0, 0, 0, 0]);
pub const GUID_MODEBIAS_NUMERIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1075934828, 59506, 18685, [156, 238, 78, 197, 199, 94, 22, 195]);
pub const GUID_MODEBIAS_READING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3809887139, 25702, 19647, [141, 139, 11, 212, 216, 84, 84, 97]);
pub const GUID_MODEBIAS_URLHISTORY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2332972249, 25586, 19560, [132, 212, 121, 174, 231, 165, 159, 9]);
pub const GUID_PROP_ATTRIBUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(884233840, 29990, 4562, [161, 71, 0, 16, 90, 39, 153, 181]);
pub const GUID_PROP_COMPOSING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3777675360, 44821, 4562, [175, 197, 0, 16, 90, 39, 153, 181]);
pub const GUID_PROP_INPUTSCOPE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(387177818, 26855, 19035, [154, 246, 89, 42, 89, 92, 119, 141]);
pub const GUID_PROP_LANGID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(847302176, 32818, 4562, [182, 3, 0, 16, 90, 39, 153, 181]);
pub const GUID_PROP_MODEBIAS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(925763350, 38735, 16556, [160, 136, 8, 205, 201, 46, 191, 188]);
pub const GUID_PROP_READING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1415837632, 36401, 4562, [191, 70, 0, 16, 90, 39, 153, 181]);
pub const GUID_PROP_TEXTOWNER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4058174752, 2409, 4563, [141, 240, 0, 16, 90, 39, 153, 181]);
pub const GUID_PROP_TKB_ALTERNATES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1890756611, 38541, 17966, [185, 59, 33, 100, 201, 21, 23, 247]);
pub const GUID_SYSTEM_FUNCTIONPROVIDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2590608304, 3873, 4563, [141, 241, 0, 16, 90, 39, 153, 181]);
pub const GUID_TFCAT_CATEGORY_OF_TIP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1397508289, 1543, 16536, [165, 33, 79, 200, 153, 199, 62, 144]);
pub const GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3110017051, 59980, 19185, [128, 86, 124, 50, 26, 187, 176, 145]);
pub const GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(74157184, 5703, 16631, [155, 33, 185, 59, 129, 170, 188, 27]);
pub const GUID_TFCAT_PROPSTYLE_STATIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1449113816, 27604, 19617, [178, 35, 15, 44, 203, 143, 79, 150]);
pub const GUID_TFCAT_PROP_AUDIODATA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2608587689, 59563, 19783, [168, 254, 37, 79, 164, 35, 67, 109]);
pub const GUID_TFCAT_PROP_INKDATA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2087355054, 45271, 20244, [167, 69, 20, 242, 139, 0, 157, 97]);
pub const GUID_TFCAT_TIPCAP_COMLESS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(910300633, 30140, 4567, [166, 239, 0, 6, 91, 132, 67, 92]);
pub const GUID_TFCAT_TIPCAP_DUALMODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(989009058, 55199, 19227, [153, 146, 21, 8, 109, 51, 155, 5]);
pub const GUID_TFCAT_TIPCAP_IMMERSIVEONLY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(977426860, 25613, 19156, [137, 247, 30, 182, 126, 124, 78, 232]);
pub const GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(329258719, 22027, 18125, [148, 122, 76, 58, 241, 224, 227, 93]);
pub const GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3438304727, 19079, 4567, [166, 226, 0, 6, 91, 132, 67, 92]);
pub const GUID_TFCAT_TIPCAP_LOCALSERVER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1953930985, 19046, 20381, [144, 214, 191, 139, 124, 62, 180, 97]);
pub const GUID_TFCAT_TIPCAP_SECUREMODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1238563278, 8030, 4567, [166, 211, 0, 6, 91, 132, 67, 92]);
pub const GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(626020276, 31659, 19393, [156, 105, 207, 129, 137, 15, 14, 245]);
pub const GUID_TFCAT_TIPCAP_TSF3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(131904687, 39134, 17736, [190, 247, 37, 189, 69, 151, 154, 31]);
pub const GUID_TFCAT_TIPCAP_UIELEMENTENABLED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1238563279, 8030, 4567, [166, 211, 0, 6, 91, 132, 67, 92]);
pub const GUID_TFCAT_TIPCAP_WOW16: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(910300634, 30140, 4567, [166, 239, 0, 6, 91, 132, 67, 92]);
pub const GUID_TFCAT_TIP_HANDWRITING: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(611240839, 49906, 19134, [144, 91, 200, 179, 138, 221, 44, 67]);
pub const GUID_TFCAT_TIP_KEYBOARD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(880041059, 45808, 18308, [139, 103, 94, 18, 200, 112, 26, 49]);
pub const GUID_TFCAT_TIP_SPEECH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3047636177, 33621, 17003, [161, 97, 37, 152, 8, 242, 107, 20]);
pub const GUID_TFCAT_TRANSITORYEXTENSIONUI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1661132322, 42447, 19202, [191, 232, 77, 114, 178, 190, 211, 198]);
pub const GUID_TS_SERVICE_ACCESSIBLE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4185416192, 42431, 18959, [140, 36, 251, 22, 245, 209, 170, 187]);
pub const GUID_TS_SERVICE_ACTIVEX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3935533648, 51622, 19325, [137, 74, 73, 217, 155, 120, 72, 52]);
pub const GUID_TS_SERVICE_DATAOBJECT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1619458997, 57893, 18126, [167, 112, 193, 187, 211, 224, 93, 123]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1284071481, 31597, 18918, [168, 193, 69, 17, 106, 152, 41, 43]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn ConvertValueToString<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, term: *const ::windows::runtime::GUID, lcid: u32, varvalue: Param2, pbstrresult: *mut super::super::Foundation::BSTR, plcid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(term), ::core::mem::transmute(lcid), varvalue.into_param().abi(), ::core::mem::transmute(pbstrresult), ::core::mem::transmute(plcid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAccDictionary {
    type Vtable = IAccDictionary_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499436383, 55095, 18253, [173, 233, 92, 207, 201, 188, 28, 201]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, term: *const ::windows::runtime::GUID, lcid: u32, varvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pbstrresult: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plcid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2910614479, 28117, 18517, [171, 194, 176, 75, 173, 91, 145, 83]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3805104739, 11122, 19784, [183, 57, 149, 228, 118, 81, 149, 186]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(267091508, 23136, 17238, [142, 247, 171, 222, 194, 255, 124, 248]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3007215103, 59468, 19914, [162, 92, 51, 184, 220, 0, 51, 116]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn CoCreateLocally<'a, Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param6: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, rclsid: *const ::windows::runtime::GUID, dwclscontext: u32, riid: *const ::windows::runtime::GUID, punk: *mut ::core::option::Option<::windows::runtime::IUnknown>, riidparam: *const ::windows::runtime::GUID, punkparam: Param5, varparam: Param6) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(riid), ::core::mem::transmute(punk), ::core::mem::transmute(riidparam), punkparam.into_param().abi(), varparam.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICoCreateLocally {
    type Vtable = ICoCreateLocally_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(64880810, 62066, 16867, [153, 203, 3, 197, 232, 17, 78, 160]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, dwclscontext: u32, riid: *const ::windows::runtime::GUID, punk: *mut ::windows::runtime::RawPtr, riidparam: *const ::windows::runtime::GUID, punkparam: ::windows::runtime::RawPtr, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TextServices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICoCreatedLocally(pub ::windows::runtime::IUnknown);
impl ICoCreatedLocally {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn LocalInit<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, punklocalobject: Param0, riidparam: *const ::windows::runtime::GUID, punkparam: Param2, varparam: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punklocalobject.into_param().abi(), ::core::mem::transmute(riidparam), punkparam.into_param().abi(), varparam.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICoCreatedLocally {
    type Vtable = ICoCreatedLocally_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(173271916, 6408, 18242, [140, 255, 44, 238, 46, 147, 249, 76]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punklocalobject: ::windows::runtime::RawPtr, riidparam: *const ::windows::runtime::GUID, punkparam: ::windows::runtime::RawPtr, varparam: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3704784382, 3040, 17341, [153, 201, 170, 174, 197, 19, 197, 85]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1593647802, 30776, 18123, [136, 226, 202, 219, 20, 18, 79, 143]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2354949199, 2108, 19333, [164, 201, 113, 116, 96, 72, 173, 202]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3740997926, 27776, 19688, [135, 212, 214, 183, 43, 129, 43, 222]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4039178461, 53048, 17633, [187, 15, 104, 207, 13, 85, 28, 120]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2400878246, 5716, 17666, [168, 110, 178, 144, 35, 68, 213, 7]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2096039127, 52085, 20096, [167, 171, 95, 91, 199, 211, 50, 222]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574728, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3836890544, 2448, 4563, [141, 240, 0, 16, 90, 39, 153, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1908860749, 3880, 4568, [168, 42, 0, 6, 91, 132, 67, 92]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1480537296, 56869, 4562, [175, 221, 0, 16, 90, 39, 153, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029816081, 44127, 17096, [164, 203, 147, 27, 204, 40, 199, 68]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1452834898, 18394, 18949, [145, 26, 227, 217, 65, 241, 113, 69]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(421039280, 44201, 4562, [175, 197, 0, 16, 90, 39, 153, 181]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2396559387, 31760, 19837, [159, 179, 171, 114, 233, 199, 95, 114]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, rgvalues: *mut ::core::mem::ManuallyDrop<TF_PROPERTYVAL>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4187832128, 36402, 4562, [191, 70, 0, 16, 90, 39, 153, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2289740062, 44218, 18737, [132, 218, 60, 82, 8, 207, 84, 63]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3786040422, 40372, 16570, [190, 3, 119, 195, 142, 142, 96, 178]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(954244428, 22637, 17242, [181, 146, 200, 168, 102, 145, 222, 198]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(680038371, 49824, 18490, [163, 234, 140, 177, 206, 81, 255, 61]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, paattrvals: *mut ::core::mem::ManuallyDrop<TS_ATTRVAL>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4167751839, 24548, 19341, [187, 159, 239, 55, 151, 168, 79, 31]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, paattrvals: *mut ::core::mem::ManuallyDrop<TS_ATTRVAL>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2732473282, 15758, 4563, [129, 169, 247, 83, 251, 230, 26, 0]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574977, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(584338580, 42009, 17730, [162, 114, 174, 38, 9, 62, 206, 207]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(736072804, 16866, 17379, [149, 12, 166, 134, 91, 162, 92, 212]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2602596272, 24344, 19948, [190, 233, 60, 199, 34, 245, 223, 224]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulcount: u32, paattrvals: *mut ::core::mem::ManuallyDrop<TS_ATTRVAL>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2732473281, 15758, 4563, [129, 169, 247, 83, 251, 230, 26, 0]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574981, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(627319846, 653, 17524, [151, 123, 17, 27, 177, 20, 254, 62]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2990984053, 43326, 18002, [191, 140, 179, 254, 12, 253, 126, 87]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2746044667, 39899, 18915, [168, 67, 108, 118, 82, 15, 191, 93]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3927875896, 6623, 4567, [166, 210, 0, 6, 91, 132, 67, 92]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2247807365, 22734, 18810, [148, 96, 53, 83, 102, 182, 75, 154]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1478439294, 64925, 17471, [185, 114, 237, 0, 70, 124, 93, 64]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3282890677, 63133, 18693, [147, 143, 252, 173, 207, 75, 232, 48]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1170428228, 5454, 18327, [190, 216, 211, 58, 231, 191, 135, 148]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(23631497, 31435, 20123, [171, 124, 126, 164, 107, 18, 181, 34]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3591011145, 7071, 19426, [183, 2, 71, 233, 220, 5, 222, 195]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetValue(&self, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid), ::core::mem::transmute(pvarvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetValue(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfCompartment {
    type Vtable = ITfCompartment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3137927081, 24698, 17284, [134, 35, 5, 104, 146, 182, 67, 113]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tid: u32, pvarvalue: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1950006623, 62061, 18655, [140, 197, 35, 132, 146, 65, 155, 100]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2110740396, 6317, 17291, [130, 77, 151, 155, 255, 183, 75, 124]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(538348900, 23183, 19034, [183, 189, 207, 162, 159, 77, 15, 217]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2810278284, 22426, 19221, [162, 128, 50, 184, 87, 122, 204, 94]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3612607041, 63905, 17252, [190, 252, 219, 205, 44, 67, 149, 183]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(221025946, 48284, 17276, [132, 238, 149, 28, 73, 177, 167, 100]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574717, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3557591726, 44178, 20423, [154, 17, 14, 224, 226, 58, 163, 155]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(89307741, 51253, 18740, [191, 80, 132, 106, 170, 103, 67, 47]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetAttribute(&self, rguidattribute: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidattribute), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITfContextOwner {
    type Vtable = ITfContextOwner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574732, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidattribute: *const ::windows::runtime::GUID, pvarvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2252744720, 22843, 18710, [151, 100, 25, 192, 142, 156, 225, 16]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1595976256, 46458, 20276, [150, 171, 53, 118, 243, 119, 204, 121]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2990454320, 15900, 4563, [167, 69, 0, 80, 4, 10, 180, 7]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(607371150, 3995, 17244, [186, 44, 24, 6, 17, 151, 140, 48]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(610532336, 45231, 4562, [175, 197, 0, 16, 90, 39, 153, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1884457042, 12070, 19178, [140, 150, 33, 81, 80, 87, 137, 50]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2381149075, 23985, 18268, [158, 113, 163, 145, 17, 176, 255, 103]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2908156930, 57698, 20261, [144, 143, 125, 87, 124, 249, 189, 169]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4276385655, 5692, 18281, [153, 106, 110, 156, 80, 173, 143, 84]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574708, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1121243289, 31770, 19081, [184, 54, 108, 111, 34, 22, 13, 240]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574723, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1888468848, 46368, 16747, [176, 108, 44, 65, 171, 68, 248, 186]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(891758219, 32083, 19929, [146, 183, 114, 150, 174, 70, 18, 73]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1001097700, 24510, 17908, [165, 188, 220, 163, 106, 210, 37, 168]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2297784262, 5975, 18936, [161, 178, 137, 35, 76, 30, 239, 249]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3051515893, 55213, 17156, [145, 63, 33, 162, 237, 149, 161, 176]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3147137162, 28047, 19402, [132, 0, 83, 144, 181, 134, 174, 223]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4238787401, 41263, 17315, [141, 214, 90, 90, 66, 130, 87, 123]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3927325922, 31333, 17670, [130, 163, 197, 40, 33, 93, 166, 78]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1597020737, 22794, 19148, [169, 127, 216, 239, 255, 19, 253, 252]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1544206314, 5757, 20313, [191, 181, 70, 147, 117, 94, 144, 202]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(79177137, 44186, 20347, [181, 173, 199, 22, 143, 30, 228, 69]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2063333607, 44107, 16514, [176, 88, 137, 8, 153, 211, 160, 16]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2826601844, 42689, 19989, [153, 240, 61, 57, 101, 245, 72, 235]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2745439908, 3940, 4563, [181, 183, 0, 192, 79, 195, 36, 161]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(590916718, 11165, 17600, [167, 94, 238, 100, 242, 86, 179, 189]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1290441664, 2648, 4563, [141, 240, 0, 16, 90, 39, 153, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2275585423, 62075, 18720, [133, 1, 103, 96, 34, 128, 23, 93]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1521603340, 2381, 19497, [142, 165, 11, 245, 155, 232, 123, 243]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3680056464, 2447, 4563, [141, 240, 0, 16, 90, 39, 153, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(270362128, 2448, 4563, [141, 240, 0, 16, 90, 39, 153, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1908860750, 3880, 4568, [168, 42, 0, 6, 91, 132, 67, 92]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1908860748, 3880, 4568, [168, 42, 0, 6, 91, 132, 67, 92]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1339453844, 4098, 17683, [191, 242, 192, 221, 246, 37, 133, 82]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(520271557, 30786, 20198, [138, 11, 154, 36, 24, 58, 149, 202]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2301567759, 65024, 19009, [169, 142, 252, 214, 222, 13, 53, 239]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4259441390, 26916, 19679, [145, 231, 218, 56, 207, 245, 85, 157]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1462889120, 27586, 18049, [165, 50, 146, 251, 183, 77, 124, 65]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1439569594, 12308, 16833, [156, 235, 250, 222, 20, 70, 172, 108]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3349607759, 45440, 16751, [178, 191, 123, 242, 228, 104, 61, 123]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574709, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(483705147, 7222, 16785, [167, 10, 127, 62, 97, 31, 54, 125]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574704, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3559089781, 42431, 17776, [157, 66, 93, 109, 123, 2, 213, 155]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(413460736, 57518, 4562, [175, 221, 0, 16, 90, 39, 153, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1934888297, 60907, 20201, [150, 201, 35, 170, 48, 178, 89, 22]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(29545093, 54215, 19323, [181, 181, 217, 116, 17, 208, 194, 131]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1937965906, 55074, 16761, [173, 165, 240, 69, 201, 141, 243, 85]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2724857125, 16302, 20384, [137, 238, 136, 169, 100, 249, 241, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(684192208, 56869, 4562, [175, 221, 0, 16, 90, 39, 153, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3125185621, 39254, 20401, [165, 157, 82, 167, 221, 124, 198, 170]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1474027936, 56869, 4562, [175, 221, 0, 16, 90, 39, 153, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2274711184, 58919, 4562, [141, 219, 0, 16, 90, 39, 153, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1137311253, 62612, 19479, [157, 226, 184, 164, 172, 53, 10, 168]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3052993339, 14655, 20348, [132, 203, 80, 73, 36, 194, 112, 90]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1871354084, 43680, 20245, [140, 91, 7, 224, 223, 10, 61, 216]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2400946904, 2923, 18548, [144, 197, 189, 118, 1, 30, 143, 124]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2712513186, 14884, 17565, [172, 150, 81, 131, 231, 245, 194, 23]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(164710093, 42308, 16690, [146, 91, 122, 250, 142, 243, 34, 208]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1004370146, 49518, 18429, [184, 131, 206, 111, 172, 193, 162, 8]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1324912976, 2055, 4563, [141, 240, 0, 16, 90, 39, 153, 181]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1870121363, 53937, 17518, [133, 62, 89, 18, 239, 200, 162, 134]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3796145760, 38210, 4562, [191, 70, 0, 16, 90, 39, 153, 181]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, pvarvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, pprange: *mut ::windows::runtime::RawPtr, apos: TfAnchor) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, ppropstore: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, pvarvalue: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1748283680, 35019, 4562, [191, 69, 0, 16, 90, 39, 153, 181]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(262904795, 53840, 16745, [132, 229, 107, 225, 24, 253, 215, 168]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574719, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(91906710, 667, 16724, [183, 154, 13, 70, 29, 78, 169, 76]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1178226797, 27026, 18898, [155, 136, 147, 213, 94, 112, 187, 22]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(399809085, 63672, 19247, [178, 84, 82, 49, 157, 214, 76, 83]);
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ec: u32, prange: ::windows::runtime::RawPtr, pvarvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3927875897, 6623, 4567, [166, 210, 0, 6, 91, 132, 67, 92]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2752897378, 5501, 16765, [138, 140, 10, 178, 108, 125, 39, 129]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(354249200, 34548, 18036, [183, 33, 86, 145, 30, 121, 127, 71]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3057893942, 50323, 16822, [171, 179, 105, 36, 18, 119, 92, 196]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1319406133, 24750, 17519, [143, 214, 230, 168, 216, 36, 89, 247]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1930633116, 22185, 18909, [176, 238, 208, 70, 99, 63, 117, 40]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2431232324, 37444, 18591, [167, 143, 222, 103, 175, 192, 19, 167]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1803390323, 45671, 20329, [179, 46, 28, 163, 33, 206, 79, 69]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1164390073, 36953, 18082, [131, 141, 69, 48, 53, 95, 106, 119]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(504621548, 27443, 19786, [181, 235, 138, 146, 240, 41, 243, 86]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(340384171, 5071, 18055, [170, 62, 141, 139, 24, 87, 67, 150]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1548542181, 47689, 19282, [172, 107, 59, 57, 123, 79, 112, 31]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2166871049, 52435, 18051, [150, 122, 180, 61, 91, 72, 43, 247]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574711, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1850614018, 63949, 17213, [180, 150, 48, 60, 224, 58, 101, 7]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(720556138, 56667, 18727, [160, 180, 84, 241, 156, 145, 250, 222]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3237075724, 14880, 16476, [163, 3, 150, 182, 1, 10, 136, 95]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574721, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(179411183, 25719, 20200, [136, 18, 103, 128, 237, 184, 45, 94]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860574734, 8225, 4562, [147, 224, 0, 96, 176, 103, 184, 110]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1049669091, 30100, 19632, [187, 88, 105, 98, 143, 95, 69, 140]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1387367260, 21853, 18098, [176, 10, 250, 104, 1, 68, 251, 219]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2786396527, 7255, 18451, [138, 21, 85, 238, 110, 90, 131, 156]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2240779626, 38703, 17058, [162, 242, 3, 33, 225, 171, 226, 9]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3927875895, 6623, 4567, [166, 210, 0, 6, 91, 132, 67, 92]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3927875893, 6623, 4567, [166, 210, 0, 6, 91, 132, 67, 92]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3927875894, 6623, 4567, [166, 210, 0, 6, 91, 132, 67, 92]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3448886928, 42984, 16997, [155, 56, 139, 179, 187, 171, 167, 222]);
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1075124460, 56064, 17937, [155, 41, 42, 14, 75, 154, 250, 133]);
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
pub const LIBID_MSAATEXTLib: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(353250682, 56001, 17794, [148, 125, 42, 143, 215, 139, 130, 205]);
pub const MSAAControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(147691071, 31294, 20316, [155, 216, 214, 146, 187, 4, 60, 91]);
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
pub const TF_PROFILE_ARRAY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3549364069, 43590, 20437, [145, 167, 103, 132, 95, 176, 47, 91]);
pub const TF_PROFILE_CANTONESE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(183242908, 32406, 4564, [178, 239, 0, 128, 200, 130, 104, 126]);
pub const TF_PROFILE_CHANGJIE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1272946435, 51155, 4564, [178, 171, 0, 128, 200, 130, 104, 126]);
pub const TF_PROFILE_DAYI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(58403877, 18444, 19839, [176, 39, 214, 202, 107, 105, 120, 138]);
pub const TF_PROFILE_NEWCHANGJIE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4089090170, 27774, 4564, [151, 250, 0, 128, 200, 130, 104, 126]);
pub const TF_PROFILE_NEWPHONETIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3002713346, 5954, 4564, [151, 144, 0, 128, 200, 130, 104, 126]);
pub const TF_PROFILE_NEWQUICK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(193477536, 49607, 4564, [135, 249, 0, 128, 200, 130, 104, 126]);
pub const TF_PROFILE_PHONETIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1980959198, 12666, 4564, [155, 93, 0, 128, 200, 130, 104, 126]);
pub const TF_PROFILE_PINYIN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4089090167, 27774, 4564, [151, 250, 0, 128, 200, 130, 104, 126]);
pub const TF_PROFILE_QUICK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1613018207, 23636, 4564, [185, 33, 0, 128, 200, 130, 104, 126]);
pub const TF_PROFILE_SIMPLEFAST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4199877380, 23255, 16671, [165, 172, 202, 3, 142, 197, 21, 215]);
pub const TF_PROFILE_TIGRINYA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1017874615, 52286, 18086, [151, 101, 183, 114, 173, 119, 97, 255]);
pub const TF_PROFILE_WUBI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2186873875, 62685, 17652, [186, 29, 134, 103, 36, 111, 223, 142]);
pub const TF_PROFILE_YI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1083999094, 123, 17239, [174, 142, 38, 49, 110, 227, 251, 13]);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl ::core::clone::Clone for TF_PROPERTYVAL {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
pub struct TF_PROPERTYVAL {
    pub guidId: ::windows::runtime::GUID,
    pub varValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl TF_PROPERTYVAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl ::core::default::Default for TF_PROPERTYVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::PartialEq for TF_PROPERTYVAL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::Eq for TF_PROPERTYVAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
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
pub const TSATTRID_App: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2819586015, 16951, 16613, [132, 156, 181, 250, 81, 193, 58, 199]);
pub const TSATTRID_App_IncorrectGrammar: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3176457112, 44291, 19316, [182, 179, 94, 219, 25, 153, 99, 136]);
pub const TSATTRID_App_IncorrectSpelling: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4096648252, 61202, 17165, [148, 76, 154, 8, 151, 10, 37, 210]);
pub const TSATTRID_Font: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1463724069, 29851, 20362, [156, 253, 33, 195, 96, 92, 168, 40]);
pub const TSATTRID_Font_FaceName: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3040259766, 1339, 20152, [182, 90, 80, 218, 30, 129, 231, 46]);
pub const TSATTRID_Font_SizePts: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3360240386, 42473, 17773, [175, 4, 128, 5, 228, 19, 15, 3]);
pub const TSATTRID_Font_Style: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1756538751, 27406, 20264, [129, 119, 87, 28, 47, 58, 66, 177]);
pub const TSATTRID_Font_Style_Animation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3707190562, 57385, 18359, [187, 54, 242, 99, 163, 208, 4, 204]);
pub const TSATTRID_Font_Style_Animation_BlinkingBackground: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2263200004, 260, 19216, [181, 133, 0, 242, 82, 117, 34, 181]);
pub const TSATTRID_Font_Style_Animation_LasVegasLights: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4093912021, 3975, 20367, [186, 218, 230, 214, 12, 37, 225, 82]);
pub const TSATTRID_Font_Style_Animation_MarchingBlackAnts: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1984225383, 61830, 18690, [191, 198, 236, 129, 90, 162, 14, 157]);
pub const TSATTRID_Font_Style_Animation_MarchingRedAnts: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2016841133, 20731, 19567, [132, 11, 212, 134, 187, 108, 247, 129]);
pub const TSATTRID_Font_Style_Animation_Shimmer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(753081176, 21139, 19510, [136, 9, 191, 139, 181, 26, 39, 179]);
pub const TSATTRID_Font_Style_Animation_SparkleText: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1396354336, 38444, 20127, [140, 9, 180, 46, 164, 116, 151, 17]);
pub const TSATTRID_Font_Style_Animation_WipeDown: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1483925620, 13947, 18435, [177, 96, 201, 15, 246, 37, 105, 208]);
pub const TSATTRID_Font_Style_Animation_WipeRight: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3092630499, 15660, 17920, [177, 233, 225, 201, 206, 2, 248, 66]);
pub const TSATTRID_Font_Style_BackgroundColor: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3037637198, 12433, 17512, [129, 219, 215, 158, 161, 144, 199, 199]);
pub const TSATTRID_Font_Style_Blink: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3216162870, 31439, 17714, [183, 32, 180, 22, 221, 119, 101, 168]);
pub const TSATTRID_Font_Style_Bold: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1216428611, 35360, 18752, [142, 88, 151, 130, 63, 123, 38, 138]);
pub const TSATTRID_Font_Style_Capitalize: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2105910202, 46333, 17331, [190, 252, 107, 152, 92, 132, 49, 65]);
pub const TSATTRID_Font_Style_Color: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2239396407, 47279, 20122, [129, 180, 172, 247, 0, 200, 65, 27]);
pub const TSATTRID_Font_Style_Emboss: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3180255042, 13470, 20023, [130, 251, 67, 121, 121, 203, 83, 167]);
pub const TSATTRID_Font_Style_Engrave: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2620617182, 33586, 18583, [190, 93, 137, 35, 50, 35, 23, 154]);
pub const TSATTRID_Font_Style_Height: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2123592823, 4838, 17803, [146, 106, 31, 164, 78, 232, 243, 145]);
pub const TSATTRID_Font_Style_Hidden: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2984413040, 34844, 18271, [134, 63, 136, 122, 100, 123, 16, 144]);
pub const TSATTRID_Font_Style_Italic: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2269145130, 42853, 18657, [172, 252, 210, 34, 34, 178, 248, 16]);
pub const TSATTRID_Font_Style_Kerning: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3425100212, 12186, 18376, [139, 255, 191, 30, 183, 204, 224, 221]);
pub const TSATTRID_Font_Style_Lowercase: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1993919669, 51835, 17560, [142, 233, 213, 196, 246, 247, 76, 96]);
pub const TSATTRID_Font_Style_Outlined: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(283564849, 56077, 19142, [167, 245, 156, 156, 255, 111, 42, 180]);
pub const TSATTRID_Font_Style_Overline: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3818430282, 39211, 17153, [140, 225, 165, 183, 198, 209, 243, 200]);
pub const TSATTRID_Font_Style_Overline_Double: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3695576634, 57621, 18147, [188, 216, 202, 103, 114, 170, 149, 180]);
pub const TSATTRID_Font_Style_Overline_Single: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2218842444, 20942, 18354, [141, 76, 21, 117, 30, 95, 114, 27]);
pub const TSATTRID_Font_Style_Position: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(365766315, 62203, 16482, [181, 166, 154, 73, 225, 165, 204, 11]);
pub const TSATTRID_Font_Style_Protected: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(475364530, 5327, 17748, [165, 116, 236, 178, 247, 231, 239, 212]);
pub const TSATTRID_Font_Style_Shadow: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1600679215, 50893, 19542, [138, 26, 153, 74, 75, 151, 102, 190]);
pub const TSATTRID_Font_Style_SmallCaps: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4207635398, 37120, 19654, [185, 105, 17, 238, 164, 90, 134, 180]);
pub const TSATTRID_Font_Style_Spacing: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2562793485, 36614, 16538, [142, 73, 106, 85, 75, 247, 193, 83]);
pub const TSATTRID_Font_Style_Strikethrough: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(206971283, 11528, 18024, [150, 1, 206, 212, 19, 9, 215, 175]);
pub const TSATTRID_Font_Style_Strikethrough_Double: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1648925489, 41959, 20372, [172, 67, 235, 175, 143, 204, 122, 159]);
pub const TSATTRID_Font_Style_Strikethrough_Single: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1977038518, 15503, 19351, [171, 120, 24, 119, 203, 153, 13, 49]);
pub const TSATTRID_Font_Style_Subscript: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1467284356, 14491, 17340, [167, 75, 21, 104, 52, 124, 240, 244]);
pub const TSATTRID_Font_Style_Superscript: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(782539068, 22076, 18858, [147, 114, 11, 239, 9, 169, 37, 91]);
pub const TSATTRID_Font_Style_Underline: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3284781555, 30978, 17483, [154, 123, 72, 231, 15, 75, 80, 247]);
pub const TSATTRID_Font_Style_Underline_Double: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1959938726, 7603, 19561, [161, 118, 49, 18, 14, 117, 134, 213]);
pub const TSATTRID_Font_Style_Underline_Single: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(459743461, 3955, 18769, [166, 179, 111, 25, 228, 60, 148, 97]);
pub const TSATTRID_Font_Style_Uppercase: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(866320616, 58176, 18743, [182, 151, 143, 35, 64, 69, 205, 154]);
pub const TSATTRID_Font_Style_Weight: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(317921436, 35760, 17947, [177, 250, 234, 249, 7, 4, 127, 224]);
pub const TSATTRID_List: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1131243323, 9969, 19182, [158, 101, 143, 131, 164, 237, 72, 132]);
pub const TSATTRID_List_LevelIndel: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2138884249, 12575, 18555, [173, 93, 226, 164, 89, 225, 45, 66]);
pub const TSATTRID_List_Type: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2923325022, 19406, 18915, [160, 254, 45, 180, 125, 58, 23, 174]);
pub const TSATTRID_List_Type_Arabic: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(322487766, 39075, 20387, [155, 209, 122, 96, 238, 248, 233, 224]);
pub const TSATTRID_List_Type_Bullet: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3167582149, 19533, 19682, [177, 2, 85, 159, 59, 43, 252, 234]);
pub const TSATTRID_List_Type_LowerLetter: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520195717, 62415, 18718, [169, 37, 56, 50, 52, 127, 210, 55]);
pub const TSATTRID_List_Type_LowerRoman: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2420531810, 14720, 19342, [147, 104, 145, 139, 209, 33, 138, 65]);
pub const TSATTRID_List_Type_UpperLetter: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2038937549, 52818, 17035, [155, 149, 163, 87, 246, 241, 12, 69]);
pub const TSATTRID_List_Type_UpperRoman: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(258651474, 19072, 18047, [178, 241, 18, 126, 42, 163, 186, 158]);
pub const TSATTRID_OTHERS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3015912185, 22480, 18089, [188, 168, 218, 194, 56, 161, 48, 87]);
pub const TSATTRID_Text: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2128318056, 33273, 17565, [161, 90, 135, 168, 56, 143, 170, 192]);
pub const TSATTRID_Text_Alignment: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(328810982, 5991, 17773, [147, 142, 53, 186, 86, 139, 92, 212]);
pub const TSATTRID_Text_Alignment_Center: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2762562582, 21439, 19797, [139, 135, 75, 221, 141, 66, 117, 252]);
pub const TSATTRID_Text_Alignment_Justify: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3979675456, 41207, 17107, [142, 168, 248, 27, 100, 136, 250, 240]);
pub const TSATTRID_Text_Alignment_Left: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(380540371, 25441, 17314, [132, 149, 208, 15, 57, 127, 22, 147]);
pub const TSATTRID_Text_Alignment_Right: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3010400152, 7070, 17248, [134, 22, 3, 251, 8, 167, 132, 86]);
pub const TSATTRID_Text_EmbeddedObject: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2128318056, 33273, 17565, [161, 90, 135, 168, 56, 143, 170, 192]);
pub const TSATTRID_Text_Hyphenation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3672065317, 24974, 18923, [177, 168, 59, 104, 189, 118, 72, 227]);
pub const TSATTRID_Text_Language: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3636481777, 22355, 19493, [136, 135, 133, 68, 63, 229, 248, 25]);
pub const TSATTRID_Text_Link: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1204654161, 14114, 19672, [183, 200, 78, 23, 202, 23, 89, 245]);
pub const TSATTRID_Text_Orientation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1806397567, 34693, 19513, [139, 82, 150, 248, 120, 48, 63, 251]);
pub const TSATTRID_Text_Para: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1591498786, 39388, 19926, [174, 195, 182, 43, 170, 91, 46, 124]);
pub const TSATTRID_Text_Para_FirstLineIndent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(130644499, 29810, 19928, [144, 169, 145, 227, 215, 228, 242, 156]);
pub const TSATTRID_Text_Para_LeftIndent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4213721321, 29809, 16841, [182, 179, 138, 20, 80, 224, 24, 151]);
pub const TSATTRID_Text_Para_LineSpacing: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1771780109, 32652, 18134, [167, 59, 223, 227, 209, 83, 141, 243]);
pub const TSATTRID_Text_Para_LineSpacing_AtLeast: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2919161649, 11588, 17460, [165, 255, 127, 76, 73, 144, 169, 5]);
pub const TSATTRID_Text_Para_LineSpacing_Double: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2197493765, 42692, 16945, [172, 18, 98, 96, 175, 42, 186, 40]);
pub const TSATTRID_Text_Para_LineSpacing_Exactly: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1027976512, 9182, 18647, [166, 179, 118, 84, 32, 198, 32, 204]);
pub const TSATTRID_Text_Para_LineSpacing_Multiple: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2433687100, 54992, 20325, [138, 60, 66, 180, 179, 24, 104, 197]);
pub const TSATTRID_Text_Para_LineSpacing_OnePtFive: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(69771297, 919, 19287, [154, 23, 7, 149, 153, 76, 211, 197]);
pub const TSATTRID_Text_Para_LineSpacing_Single: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3979675456, 41207, 17107, [142, 168, 248, 27, 100, 136, 250, 240]);
pub const TSATTRID_Text_Para_RightIndent: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(746530553, 42466, 18650, [185, 138, 82, 12, 177, 101, 19, 191]);
pub const TSATTRID_Text_Para_SpaceAfter: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2064269141, 8924, 16991, [164, 17, 147, 218, 29, 143, 155, 170]);
pub const TSATTRID_Text_Para_SpaceBefore: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2381940105, 6474, 17921, [178, 81, 152, 101, 163, 233, 6, 221]);
pub const TSATTRID_Text_ReadOnly: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2239981079, 56882, 19197, [165, 15, 162, 219, 17, 14, 110, 77]);
pub const TSATTRID_Text_RightToLeft: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3395710577, 6920, 17725, [191, 221, 40, 224, 140, 138, 175, 122]);
pub const TSATTRID_Text_VerticalWriting: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1807384981, 1135, 20137, [179, 17, 151, 253, 102, 196, 39, 75]);
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl ::core::clone::Clone for TS_ATTRVAL {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
pub struct TS_ATTRVAL {
    pub idAttr: ::windows::runtime::GUID,
    pub dwOverlapId: u32,
    pub varValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl TS_ATTRVAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl ::core::default::Default for TS_ATTRVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::PartialEq for TS_ATTRVAL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::Eq for TS_ATTRVAL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
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

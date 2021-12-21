#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct APPLETIDLIST {
    pub count: i32,
    pub pIIDList: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for APPLETIDLIST {}
impl ::core::clone::Clone for APPLETIDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APPLETIDLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPLETIDLIST").field("count", &self.count).field("pIIDList", &self.pIIDList).finish()
    }
}
unsafe impl ::windows::core::Abi for APPLETIDLIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APPLETIDLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APPLETIDLIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for APPLETIDLIST {}
impl ::core::default::Default for APPLETIDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct APPLYCANDEXPARAM {
    pub dwSize: u32,
    pub lpwstrDisplay: super::super::super::Foundation::PWSTR,
    pub lpwstrReading: super::super::super::Foundation::PWSTR,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for APPLYCANDEXPARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for APPLYCANDEXPARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for APPLYCANDEXPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPLYCANDEXPARAM").field("dwSize", &self.dwSize).field("lpwstrDisplay", &self.lpwstrDisplay).field("lpwstrReading", &self.lpwstrReading).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for APPLYCANDEXPARAM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for APPLYCANDEXPARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APPLYCANDEXPARAM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for APPLYCANDEXPARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for APPLYCANDEXPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const ATTR_CONVERTED: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const ATTR_FIXEDCONVERTED: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const ATTR_INPUT: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const ATTR_INPUT_ERROR: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const ATTR_TARGET_CONVERTED: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const ATTR_TARGET_NOTCONVERTED: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CANDIDATEFORM {
    pub dwIndex: u32,
    pub dwStyle: u32,
    pub ptCurrentPos: super::super::super::Foundation::POINT,
    pub rcArea: super::super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CANDIDATEFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CANDIDATEFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CANDIDATEFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CANDIDATEFORM").field("dwIndex", &self.dwIndex).field("dwStyle", &self.dwStyle).field("ptCurrentPos", &self.ptCurrentPos).field("rcArea", &self.rcArea).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CANDIDATEFORM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CANDIDATEFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CANDIDATEFORM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CANDIDATEFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CANDIDATEFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct CANDIDATEINFO {
    pub dwSize: u32,
    pub dwCount: u32,
    pub dwOffset: [u32; 32],
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl ::core::marker::Copy for CANDIDATEINFO {}
impl ::core::clone::Clone for CANDIDATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CANDIDATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CANDIDATEINFO").field("dwSize", &self.dwSize).field("dwCount", &self.dwCount).field("dwOffset", &self.dwOffset).field("dwPrivateSize", &self.dwPrivateSize).field("dwPrivateOffset", &self.dwPrivateOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for CANDIDATEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CANDIDATEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CANDIDATEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CANDIDATEINFO {}
impl ::core::default::Default for CANDIDATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct CANDIDATELIST {
    pub dwSize: u32,
    pub dwStyle: u32,
    pub dwCount: u32,
    pub dwSelection: u32,
    pub dwPageStart: u32,
    pub dwPageSize: u32,
    pub dwOffset: [u32; 1],
}
impl ::core::marker::Copy for CANDIDATELIST {}
impl ::core::clone::Clone for CANDIDATELIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CANDIDATELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CANDIDATELIST").field("dwSize", &self.dwSize).field("dwStyle", &self.dwStyle).field("dwCount", &self.dwCount).field("dwSelection", &self.dwSelection).field("dwPageStart", &self.dwPageStart).field("dwPageSize", &self.dwPageSize).field("dwOffset", &self.dwOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for CANDIDATELIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CANDIDATELIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CANDIDATELIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for CANDIDATELIST {}
impl ::core::default::Default for CANDIDATELIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CATID_MSIME_IImePadApplet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7566cad1_4ec9_4478_9fe9_8ed766619edf);
pub const CATID_MSIME_IImePadApplet1000: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe081e1d6_2389_43cb_b66f_609f823d9f9c);
pub const CATID_MSIME_IImePadApplet1200: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa47fb5fc_7d15_4223_a789_b781bf9ae667);
pub const CATID_MSIME_IImePadApplet900: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaae51bf_5e5b_4a1d_8de1_17c1d9e1728d);
pub const CATID_MSIME_IImePadApplet_VER7: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a0f8e31_c3ee_11d1_afef_00805f0c8b6d);
pub const CATID_MSIME_IImePadApplet_VER80: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56f7a792_fef1_11d3_8463_00c04f7a06e5);
pub const CATID_MSIME_IImePadApplet_VER81: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x656520b0_bb88_11d4_84c0_00c04f7a06e5);
pub const CActiveIMM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4955dd33_b159_11d0_8fcf_00aa006bcc59);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CFS_CANDIDATEPOS: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CFS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CFS_EXCLUDE: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CFS_FORCE_POSITION: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CFS_POINT: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CFS_RECT: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CHARINFO_APPLETID_MASK: u32 = 4278190080u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CHARINFO_CHARID_MASK: u32 = 65535u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CHARINFO_FEID_MASK: u32 = 15728640u32;
pub const CLSID_ImePlugInDictDictionaryList_CHS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bf0129b_5bef_4de4_9b0b_5edb66ac2fa6);
pub const CLSID_ImePlugInDictDictionaryList_JPN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fe2776b_b0f9_4396_b5fc_e9d4cf1ec195);
pub const CLSID_VERSION_DEPENDENT_MSIME_JAPANESE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a91029e_aa49_471b_aee7_7d332785660d);
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMPOSITIONFORM {
    pub dwStyle: u32,
    pub ptCurrentPos: super::super::super::Foundation::POINT,
    pub rcArea: super::super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMPOSITIONFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMPOSITIONFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMPOSITIONFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITIONFORM").field("dwStyle", &self.dwStyle).field("ptCurrentPos", &self.ptCurrentPos).field("rcArea", &self.rcArea).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COMPOSITIONFORM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMPOSITIONFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPOSITIONFORM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMPOSITIONFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMPOSITIONFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct COMPOSITIONSTRING {
    pub dwSize: u32,
    pub dwCompReadAttrLen: u32,
    pub dwCompReadAttrOffset: u32,
    pub dwCompReadClauseLen: u32,
    pub dwCompReadClauseOffset: u32,
    pub dwCompReadStrLen: u32,
    pub dwCompReadStrOffset: u32,
    pub dwCompAttrLen: u32,
    pub dwCompAttrOffset: u32,
    pub dwCompClauseLen: u32,
    pub dwCompClauseOffset: u32,
    pub dwCompStrLen: u32,
    pub dwCompStrOffset: u32,
    pub dwCursorPos: u32,
    pub dwDeltaStart: u32,
    pub dwResultReadClauseLen: u32,
    pub dwResultReadClauseOffset: u32,
    pub dwResultReadStrLen: u32,
    pub dwResultReadStrOffset: u32,
    pub dwResultClauseLen: u32,
    pub dwResultClauseOffset: u32,
    pub dwResultStrLen: u32,
    pub dwResultStrOffset: u32,
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl ::core::marker::Copy for COMPOSITIONSTRING {}
impl ::core::clone::Clone for COMPOSITIONSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITIONSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITIONSTRING")
            .field("dwSize", &self.dwSize)
            .field("dwCompReadAttrLen", &self.dwCompReadAttrLen)
            .field("dwCompReadAttrOffset", &self.dwCompReadAttrOffset)
            .field("dwCompReadClauseLen", &self.dwCompReadClauseLen)
            .field("dwCompReadClauseOffset", &self.dwCompReadClauseOffset)
            .field("dwCompReadStrLen", &self.dwCompReadStrLen)
            .field("dwCompReadStrOffset", &self.dwCompReadStrOffset)
            .field("dwCompAttrLen", &self.dwCompAttrLen)
            .field("dwCompAttrOffset", &self.dwCompAttrOffset)
            .field("dwCompClauseLen", &self.dwCompClauseLen)
            .field("dwCompClauseOffset", &self.dwCompClauseOffset)
            .field("dwCompStrLen", &self.dwCompStrLen)
            .field("dwCompStrOffset", &self.dwCompStrOffset)
            .field("dwCursorPos", &self.dwCursorPos)
            .field("dwDeltaStart", &self.dwDeltaStart)
            .field("dwResultReadClauseLen", &self.dwResultReadClauseLen)
            .field("dwResultReadClauseOffset", &self.dwResultReadClauseOffset)
            .field("dwResultReadStrLen", &self.dwResultReadStrLen)
            .field("dwResultReadStrOffset", &self.dwResultReadStrOffset)
            .field("dwResultClauseLen", &self.dwResultClauseLen)
            .field("dwResultClauseOffset", &self.dwResultClauseOffset)
            .field("dwResultStrLen", &self.dwResultStrLen)
            .field("dwResultStrOffset", &self.dwResultStrOffset)
            .field("dwPrivateSize", &self.dwPrivateSize)
            .field("dwPrivateOffset", &self.dwPrivateOffset)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for COMPOSITIONSTRING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPOSITIONSTRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPOSITIONSTRING>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPOSITIONSTRING {}
impl ::core::default::Default for COMPOSITIONSTRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CS_INSERTCHAR: u32 = 8192u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CS_NOMOVECARET: u32 = 16384u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const E_LARGEINPUT: u32 = 51u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const E_NOCAND: u32 = 48u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const E_NOTENOUGH_BUFFER: u32 = 49u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const E_NOTENOUGH_WDD: u32 = 50u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FEID_CHINESE_HONGKONG: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FEID_CHINESE_SIMPLIFIED: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FEID_CHINESE_SINGAPORE: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FEID_CHINESE_TRADITIONAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FEID_JAPANESE: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FEID_KOREAN: u32 = 6u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FEID_KOREAN_JOHAB: u32 = 7u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FEID_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CLMN_FIXD: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CLMN_FIXR: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CLMN_NOPBREAK: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CLMN_NOWBREAK: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CLMN_PBREAK: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CLMN_WBREAK: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_AUTOMATIC: u32 = 134217728u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_BESTFIRST: u32 = 16384u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_BOPOMOFO: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_CONVERSATION: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_FULLWIDTHOUT: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_HALFWIDTHOUT: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_HANGUL: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_HIRAGANAOUT: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_KATAKANAOUT: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_MERGECAND: u32 = 4096u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_MONORUBY: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_NAME: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_NOINVISIBLECHAR: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_NONE: u32 = 16777216u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_NOPRUNING: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_PHRASEPREDICT: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_PINYIN: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_PLAURALCLAUSE: u32 = 33554432u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_PRECONV: u32 = 512u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_RADICAL: u32 = 1024u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_ROMAN: u32 = 8192u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_SINGLECONVERT: u32 = 67108864u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_UNKNOWNREADING: u32 = 2048u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_CMODE_USENOREVWORDS: u32 = 32768u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_INVALD_PO: u32 = 65535u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_REQ_CONV: u32 = 65536u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_REQ_RECONV: u32 = 131072u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FELANG_REQ_REV: u32 = 196608u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_MSIME_KMS_DEL_KEYLIST: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_MSIME_KMS_FUNCDESC: u32 = 9u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_MSIME_KMS_GETMAP: u32 = 6u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_MSIME_KMS_GETMAPFAST: u32 = 11u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_MSIME_KMS_GETMAPSEAMLESS: u32 = 10u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_MSIME_KMS_INIT: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_MSIME_KMS_INVOKE: u32 = 7u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_MSIME_KMS_NOTIFY: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_MSIME_KMS_SETMAP: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_MSIME_KMS_TERM: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_MSIME_KMS_VERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_MSIME_VERSION: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const FID_RECONVERT_VERSION: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCSEX_CANCELRECONVERT: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCS_COMPATTR: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCS_COMPCLAUSE: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCS_COMPREADATTR: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCS_COMPREADCLAUSE: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCS_COMPREADSTR: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCS_COMPSTR: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCS_CURSORPOS: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCS_DELTASTART: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCS_RESULTCLAUSE: u32 = 4096u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCS_RESULTREADCLAUSE: u32 = 1024u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCS_RESULTREADSTR: u32 = 512u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCS_RESULTSTR: u32 = 2048u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type GET_CONVERSION_LIST_FLAG = u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCL_CONVERSION: GET_CONVERSION_LIST_FLAG = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCL_REVERSECONVERSION: GET_CONVERSION_LIST_FLAG = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GCL_REVERSE_LENGTH: GET_CONVERSION_LIST_FLAG = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type GET_GUIDE_LINE_TYPE = u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GGL_LEVEL: GET_GUIDE_LINE_TYPE = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GGL_INDEX: GET_GUIDE_LINE_TYPE = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GGL_STRING: GET_GUIDE_LINE_TYPE = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GGL_PRIVATE: GET_GUIDE_LINE_TYPE = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_CANNOTSAVE: u32 = 17u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_CHOOSECANDIDATE: u32 = 40u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_INPUTCODE: u32 = 38u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_INPUTRADICAL: u32 = 37u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_INPUTREADING: u32 = 36u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_INPUTSYMBOL: u32 = 39u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_NOCONVERT: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_NODICTIONARY: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_NOMODULE: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_PRIVATE_FIRST: u32 = 32768u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_PRIVATE_LAST: u32 = 65535u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_READINGCONFLICT: u32 = 35u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_REVERSECONVERSION: u32 = 41u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_TOOMANYSTROKE: u32 = 34u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_TYPINGERROR: u32 = 33u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_ID_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_LEVEL_ERROR: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_LEVEL_FATAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_LEVEL_INFORMATION: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_LEVEL_NOGUIDELINE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const GL_LEVEL_WARNING: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct GUIDELINE {
    pub dwSize: u32,
    pub dwLevel: u32,
    pub dwIndex: u32,
    pub dwStrLen: u32,
    pub dwStrOffset: u32,
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl ::core::marker::Copy for GUIDELINE {}
impl ::core::clone::Clone for GUIDELINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GUIDELINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GUIDELINE").field("dwSize", &self.dwSize).field("dwLevel", &self.dwLevel).field("dwIndex", &self.dwIndex).field("dwStrLen", &self.dwStrLen).field("dwStrOffset", &self.dwStrOffset).field("dwPrivateSize", &self.dwPrivateSize).field("dwPrivateOffset", &self.dwPrivateOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for GUIDELINE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GUIDELINE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GUIDELINE>()) == 0 }
    }
}
impl ::core::cmp::Eq for GUIDELINE {}
impl ::core::default::Default for GUIDELINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IACE_CHILDREN: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IACE_DEFAULT: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IACE_IGNORENOCONTEXT: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IActiveIME(::windows::core::IUnknown);
impl IActiveIME {
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Inquire(&self, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: super::super::super::Foundation::PWSTR, pdwprivate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsysteminfoflags), ::core::mem::transmute(pimeinfo), ::core::mem::transmute(szwndclass), ::core::mem::transmute(pdwprivate)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn ConversionList<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, himc: Param0, szsource: Param1, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), himc.into_param().abi(), szsource.into_param().abi(), ::core::mem::transmute(uflag), ::core::mem::transmute(ubuflen), ::core::mem::transmute(pdest), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn Configure<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hkl: Param0, hwnd: Param1, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), hkl.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pregisterword)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Destroy(&self, ureserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ureserved)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn Escape<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(uescape), ::core::mem::transmute(pdata), ::core::mem::transmute(plresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetActiveContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, himc: Param0, fflag: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), himc.into_param().abi(), fflag.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ProcessKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(uvirkey), ::core::mem::transmute(lparam), ::core::mem::transmute(pbkeystate)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn Notify<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn Select<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, himc: Param0, fselect: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), himc.into_param().abi(), fselect.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(pread), ::core::mem::transmute(dwreadlen)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ToAsciiEx<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: Param4, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(uvirkey), ::core::mem::transmute(uscancode), ::core::mem::transmute(pbkeystate), ::core::mem::transmute(fustate), himc.into_param().abi(), ::core::mem::transmute(pdwtransbuf), ::core::mem::transmute(pusize)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterWord<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, szreading: Param0, dwstyle: u32, szstring: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szstring.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterWord<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, szreading: Param0, dwstyle: u32, szstring: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szstring.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn GetRegisterWordStyle(&self, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(nitem), ::core::mem::transmute(pstylebuf), ::core::mem::transmute(pubufsize)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumRegisterWord<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, szreading: Param0, dwstyle: u32, szregister: Param2, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<IEnumRegisterWordW> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(&mut result__)).from_abi::<IEnumRegisterWordW>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn GetCodePageA(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn GetLangId(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
}
impl ::core::convert::From<IActiveIME> for ::windows::core::IUnknown {
    fn from(value: IActiveIME) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIME> for ::windows::core::IUnknown {
    fn from(value: &IActiveIME) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActiveIME {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IActiveIME {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveIME {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveIME {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveIME {}
impl ::core::fmt::Debug for IActiveIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveIME").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveIME {
    type Vtable = IActiveIMEVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fe20962_d077_11d0_8fe7_00aa006bcc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMEVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: super::super::super::Foundation::PWSTR, pdwprivate: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, szsource: super::super::super::Foundation::PWSTR, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ureserved: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fflag: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fselect: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: super::super::super::Globalization::HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szstring: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szstring: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR, pdata: *const ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ucodepage: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plid: *mut u16) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IActiveIME2(::windows::core::IUnknown);
impl IActiveIME2 {
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Inquire(&self, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: super::super::super::Foundation::PWSTR, pdwprivate: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsysteminfoflags), ::core::mem::transmute(pimeinfo), ::core::mem::transmute(szwndclass), ::core::mem::transmute(pdwprivate)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn ConversionList<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, himc: Param0, szsource: Param1, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), himc.into_param().abi(), szsource.into_param().abi(), ::core::mem::transmute(uflag), ::core::mem::transmute(ubuflen), ::core::mem::transmute(pdest), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn Configure<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hkl: Param0, hwnd: Param1, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), hkl.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pregisterword)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Destroy(&self, ureserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ureserved)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn Escape<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(uescape), ::core::mem::transmute(pdata), ::core::mem::transmute(plresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetActiveContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, himc: Param0, fflag: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), himc.into_param().abi(), fflag.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ProcessKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(uvirkey), ::core::mem::transmute(lparam), ::core::mem::transmute(pbkeystate)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn Notify<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn Select<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, himc: Param0, fselect: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), himc.into_param().abi(), fselect.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(pread), ::core::mem::transmute(dwreadlen)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ToAsciiEx<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: Param4, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(uvirkey), ::core::mem::transmute(uscancode), ::core::mem::transmute(pbkeystate), ::core::mem::transmute(fustate), himc.into_param().abi(), ::core::mem::transmute(pdwtransbuf), ::core::mem::transmute(pusize)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterWord<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, szreading: Param0, dwstyle: u32, szstring: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szstring.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterWord<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, szreading: Param0, dwstyle: u32, szstring: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szstring.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn GetRegisterWordStyle(&self, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(nitem), ::core::mem::transmute(pstylebuf), ::core::mem::transmute(pubufsize)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumRegisterWord<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, szreading: Param0, dwstyle: u32, szregister: Param2, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<IEnumRegisterWordW> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(&mut result__)).from_abi::<IEnumRegisterWordW>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn GetCodePageA(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn GetLangId(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Sleep(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unsleep<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fdead: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), fdead.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IActiveIME2> for IActiveIME {
    fn from(value: IActiveIME2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIME2> for IActiveIME {
    fn from(value: &IActiveIME2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActiveIME> for IActiveIME2 {
    fn into_param(self) -> ::windows::core::Param<'a, IActiveIME> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActiveIME> for &IActiveIME2 {
    fn into_param(self) -> ::windows::core::Param<'a, IActiveIME> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IActiveIME2> for ::windows::core::IUnknown {
    fn from(value: IActiveIME2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIME2> for ::windows::core::IUnknown {
    fn from(value: &IActiveIME2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActiveIME2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IActiveIME2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveIME2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveIME2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveIME2 {}
impl ::core::fmt::Debug for IActiveIME2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveIME2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveIME2 {
    type Vtable = IActiveIME2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1c4bf0e_2d53_11d2_93e1_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIME2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: super::super::super::Foundation::PWSTR, pdwprivate: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, szsource: super::super::super::Foundation::PWSTR, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ureserved: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fflag: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fselect: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: super::super::super::Globalization::HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szstring: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szstring: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR, pdata: *const ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ucodepage: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plid: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdead: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IActiveIMMApp(::windows::core::IUnknown);
impl IActiveIMMApp {
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn AssociateContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hwnd: Param0, hime: Param1) -> ::windows::core::Result<super::super::super::Globalization::HIMC> {
        let mut result__: super::super::super::Globalization::HIMC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), hime.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Globalization::HIMC>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn ConfigureIMEA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hkl: Param0, hwnd: Param1, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hkl.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn ConfigureIMEW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hkl: Param0, hwnd: Param1, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), hkl.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn CreateContext(&self) -> ::windows::core::Result<super::super::super::Globalization::HIMC> {
        let mut result__: super::super::super::Globalization::HIMC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Globalization::HIMC>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn DestroyContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hime: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), hime.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EnumRegisterWordA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<IEnumRegisterWordA> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(&mut result__)).from_abi::<IEnumRegisterWordA>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EnumRegisterWordW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<IEnumRegisterWordW> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(&mut result__)).from_abi::<IEnumRegisterWordW>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EscapeA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hkl: Param0, himc: Param1, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), hkl.into_param().abi(), himc.into_param().abi(), ::core::mem::transmute(uescape), ::core::mem::transmute(pdata), ::core::mem::transmute(plresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EscapeW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hkl: Param0, himc: Param1, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), hkl.into_param().abi(), himc.into_param().abi(), ::core::mem::transmute(uescape), ::core::mem::transmute(pdata), ::core::mem::transmute(plresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(ubuflen), ::core::mem::transmute(pcandlist), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(ubuflen), ::core::mem::transmute(pcandlist), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListCountA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(pdwlistsize), ::core::mem::transmute(pdwbuflen)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListCountW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(pdwlistsize), ::core::mem::transmute(pdwbuflen)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetCandidateWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32) -> ::windows::core::Result<CANDIDATEFORM> {
        let mut result__: CANDIDATEFORM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(&mut result__)).from_abi::<CANDIDATEFORM>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCompositionFontA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<super::super::super::Graphics::Gdi::LOGFONTA> {
        let mut result__: super::super::super::Graphics::Gdi::LOGFONTA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Graphics::Gdi::LOGFONTA>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCompositionFontW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<super::super::super::Graphics::Gdi::LOGFONTW> {
        let mut result__: super::super::super::Graphics::Gdi::LOGFONTW = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Graphics::Gdi::LOGFONTW>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCompositionStringA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(plcopied), ::core::mem::transmute(pbuf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCompositionStringW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(plcopied), ::core::mem::transmute(pbuf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetCompositionWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<COMPOSITIONFORM> {
        let mut result__: COMPOSITIONFORM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<COMPOSITIONFORM>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<super::super::super::Globalization::HIMC> {
        let mut result__: super::super::super::Globalization::HIMC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Globalization::HIMC>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetConversionListA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, hkl: Param0, himc: Param1, psrc: Param2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), hkl.into_param().abi(), himc.into_param().abi(), psrc.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(uflag), ::core::mem::transmute(pdst), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetConversionListW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, hkl: Param0, himc: Param1, psrc: Param2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), hkl.into_param().abi(), himc.into_param().abi(), psrc.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(uflag), ::core::mem::transmute(pdst), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetConversionStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(pfdwconversion), ::core::mem::transmute(pfdwsentence)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefaultIMEWnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: super::super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetDescriptionA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szdescription: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szdescription), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetDescriptionW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szdescription: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szdescription), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetGuideLineA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PSTR, pdwresult: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(pbuf), ::core::mem::transmute(pdwresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetGuideLineW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PWSTR, pdwresult: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(pbuf), ::core::mem::transmute(pdwresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetIMEFileNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szfilename: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szfilename), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetIMEFileNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szfilename: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szfilename), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetOpenStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), himc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, fdwindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(fdwindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetRegisterWordStyleA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(nitem), ::core::mem::transmute(pstylebuf), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetRegisterWordStyleW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(nitem), ::core::mem::transmute(pstylebuf), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetStatusWindowPos<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<super::super::super::Foundation::POINT> {
        let mut result__: super::super::super::Foundation::POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::POINT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVirtualKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn InstallIMEA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, szimefilename: Param0, szlayouttext: Param1) -> ::windows::core::Result<super::super::TextServices::HKL> {
        let mut result__: super::super::TextServices::HKL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), szimefilename.into_param().abi(), szlayouttext.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::TextServices::HKL>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn InstallIMEW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, szimefilename: Param0, szlayouttext: Param1) -> ::windows::core::Result<super::super::TextServices::HKL> {
        let mut result__: super::super::TextServices::HKL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), szimefilename.into_param().abi(), szlayouttext.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::TextServices::HKL>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn IsIME<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), hkl.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUIMessageA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwndime: Param0, msg: u32, wparam: Param2, lparam: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), hwndime.into_param().abi(), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUIMessageW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwndime: Param0, msg: u32, wparam: Param2, lparam: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), hwndime.into_param().abi(), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn NotifyIME<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn RegisterWordA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn RegisterWordW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn ReleaseContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hwnd: Param0, himc: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), himc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetCandidateWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pcandidate: *const CANDIDATEFORM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(pcandidate)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetCompositionFontA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(plf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetCompositionFontW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(plf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionStringA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(pread), ::core::mem::transmute(dwreadlen)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionStringW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(pread), ::core::mem::transmute(dwreadlen)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetCompositionWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pcompform: *const COMPOSITIONFORM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(pcompform)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetConversionStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, fdwconversion: u32, fdwsentence: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(fdwconversion), ::core::mem::transmute(fdwsentence)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetOpenStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, himc: Param0, fopen: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), himc.into_param().abi(), fopen.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetStatusWindowPos<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pptpos: *const super::super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(pptpos)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SimulateHotKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, dwhotkeyid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(dwhotkeyid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn UnregisterWordA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szunregister: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szunregister.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn UnregisterWordW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szunregister: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szunregister.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, frestorelayout: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), frestorelayout.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDefWindowProc<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwnd: Param0, msg: u32, wparam: Param2, lparam: Param3) -> ::windows::core::Result<super::super::super::Foundation::LRESULT> {
        let mut result__: super::super::super::Foundation::LRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::LRESULT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn FilterClientWindows(&self, aaclasslist: *const u16, usize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(aaclasslist), ::core::mem::transmute(usize)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetCodePageA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetLangId<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn AssociateContextEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hwnd: Param0, himc: Param1, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), himc.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn DisableIME(&self, idthread: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(idthread)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImeMenuItemsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtype), ::core::mem::transmute(pimeparentmenu), ::core::mem::transmute(pimemenu), ::core::mem::transmute(dwsize), ::core::mem::transmute(pdwresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImeMenuItemsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtype), ::core::mem::transmute(pimeparentmenu), ::core::mem::transmute(pimemenu), ::core::mem::transmute(dwsize), ::core::mem::transmute(pdwresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn EnumInputContext(&self, idthread: u32) -> ::windows::core::Result<IEnumInputContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(idthread), ::core::mem::transmute(&mut result__)).from_abi::<IEnumInputContext>(result__)
    }
}
impl ::core::convert::From<IActiveIMMApp> for ::windows::core::IUnknown {
    fn from(value: IActiveIMMApp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIMMApp> for ::windows::core::IUnknown {
    fn from(value: &IActiveIMMApp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActiveIMMApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IActiveIMMApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveIMMApp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveIMMApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveIMMApp {}
impl ::core::fmt::Debug for IActiveIMMApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveIMMApp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveIMMApp {
    type Vtable = IActiveIMMAppVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c0e040_62d1_11d1_9326_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMAppVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szregister: super::super::super::Foundation::PSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: super::super::super::Foundation::PSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: super::super::super::Foundation::PWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PWSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_UI_TextServices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_UI_TextServices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szimefilename: super::super::super::Foundation::PSTR, szlayouttext: super::super::super::Foundation::PSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szimefilename: super::super::super::Foundation::PWSTR, szlayouttext: super::super::super::Foundation::PWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_UI_TextServices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szregister: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szunregister: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szunregister: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frestorelayout: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aaclasslist: *const u16, usize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_TextServices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))] usize,
    #[cfg(feature = "Win32_UI_TextServices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IActiveIMMIME(::windows::core::IUnknown);
impl IActiveIMMIME {
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn AssociateContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hwnd: Param0, hime: Param1) -> ::windows::core::Result<super::super::super::Globalization::HIMC> {
        let mut result__: super::super::super::Globalization::HIMC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), hime.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Globalization::HIMC>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn ConfigureIMEA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hkl: Param0, hwnd: Param1, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hkl.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn ConfigureIMEW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hkl: Param0, hwnd: Param1, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), hkl.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn CreateContext(&self) -> ::windows::core::Result<super::super::super::Globalization::HIMC> {
        let mut result__: super::super::super::Globalization::HIMC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Globalization::HIMC>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn DestroyContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hime: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), hime.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EnumRegisterWordA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<IEnumRegisterWordA> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(&mut result__)).from_abi::<IEnumRegisterWordA>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EnumRegisterWordW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<IEnumRegisterWordW> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(&mut result__)).from_abi::<IEnumRegisterWordW>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EscapeA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hkl: Param0, himc: Param1, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), hkl.into_param().abi(), himc.into_param().abi(), ::core::mem::transmute(uescape), ::core::mem::transmute(pdata), ::core::mem::transmute(plresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn EscapeW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hkl: Param0, himc: Param1, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), hkl.into_param().abi(), himc.into_param().abi(), ::core::mem::transmute(uescape), ::core::mem::transmute(pdata), ::core::mem::transmute(plresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(ubuflen), ::core::mem::transmute(pcandlist), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(ubuflen), ::core::mem::transmute(pcandlist), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListCountA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(pdwlistsize), ::core::mem::transmute(pdwbuflen)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCandidateListCountW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(pdwlistsize), ::core::mem::transmute(pdwbuflen)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetCandidateWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32) -> ::windows::core::Result<CANDIDATEFORM> {
        let mut result__: CANDIDATEFORM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(&mut result__)).from_abi::<CANDIDATEFORM>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCompositionFontA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<super::super::super::Graphics::Gdi::LOGFONTA> {
        let mut result__: super::super::super::Graphics::Gdi::LOGFONTA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Graphics::Gdi::LOGFONTA>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCompositionFontW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<super::super::super::Graphics::Gdi::LOGFONTW> {
        let mut result__: super::super::super::Graphics::Gdi::LOGFONTW = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Graphics::Gdi::LOGFONTW>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCompositionStringA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(plcopied), ::core::mem::transmute(pbuf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetCompositionStringW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(plcopied), ::core::mem::transmute(pbuf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetCompositionWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<COMPOSITIONFORM> {
        let mut result__: COMPOSITIONFORM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<COMPOSITIONFORM>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<super::super::super::Globalization::HIMC> {
        let mut result__: super::super::super::Globalization::HIMC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Globalization::HIMC>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetConversionListA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, hkl: Param0, himc: Param1, psrc: Param2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), hkl.into_param().abi(), himc.into_param().abi(), psrc.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(uflag), ::core::mem::transmute(pdst), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetConversionListW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, hkl: Param0, himc: Param1, psrc: Param2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), hkl.into_param().abi(), himc.into_param().abi(), psrc.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(uflag), ::core::mem::transmute(pdst), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetConversionStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(pfdwconversion), ::core::mem::transmute(pfdwsentence)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefaultIMEWnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: super::super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetDescriptionA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szdescription: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szdescription), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetDescriptionW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szdescription: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szdescription), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetGuideLineA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PSTR, pdwresult: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(pbuf), ::core::mem::transmute(pdwresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetGuideLineW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PWSTR, pdwresult: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(pbuf), ::core::mem::transmute(pdwresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetIMEFileNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szfilename: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szfilename), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetIMEFileNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, ubuflen: u32, szfilename: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(ubuflen), ::core::mem::transmute(szfilename), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetOpenStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), himc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, fdwindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(fdwindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn GetRegisterWordStyleA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(nitem), ::core::mem::transmute(pstylebuf), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetRegisterWordStyleW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(nitem), ::core::mem::transmute(pstylebuf), ::core::mem::transmute(pucopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn GetStatusWindowPos<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<super::super::super::Foundation::POINT> {
        let mut result__: super::super::super::Foundation::POINT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::POINT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVirtualKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn InstallIMEA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, szimefilename: Param0, szlayouttext: Param1) -> ::windows::core::Result<super::super::TextServices::HKL> {
        let mut result__: super::super::TextServices::HKL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), szimefilename.into_param().abi(), szlayouttext.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::TextServices::HKL>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn InstallIMEW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, szimefilename: Param0, szlayouttext: Param1) -> ::windows::core::Result<super::super::TextServices::HKL> {
        let mut result__: super::super::TextServices::HKL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), szimefilename.into_param().abi(), szlayouttext.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::TextServices::HKL>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn IsIME<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), hkl.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUIMessageA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwndime: Param0, msg: u32, wparam: Param2, lparam: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), hwndime.into_param().abi(), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUIMessageW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwndime: Param0, msg: u32, wparam: Param2, lparam: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), hwndime.into_param().abi(), ::core::mem::transmute(msg), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn NotifyIME<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn RegisterWordA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn RegisterWordW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szregister: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szregister.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn ReleaseContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hwnd: Param0, himc: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), himc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetCandidateWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pcandidate: *const CANDIDATEFORM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(pcandidate)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetCompositionFontA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(plf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetCompositionFontW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(plf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionStringA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(pread), ::core::mem::transmute(dwreadlen)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetCompositionStringW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(pread), ::core::mem::transmute(dwreadlen)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetCompositionWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pcompform: *const COMPOSITIONFORM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(pcompform)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn SetConversionStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, fdwconversion: u32, fdwsentence: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(fdwconversion), ::core::mem::transmute(fdwsentence)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetOpenStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, himc: Param0, fopen: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), himc.into_param().abi(), fopen.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn SetStatusWindowPos<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, pptpos: *const super::super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(pptpos)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SimulateHotKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, dwhotkeyid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(dwhotkeyid)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn UnregisterWordA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szunregister: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szunregister.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub unsafe fn UnregisterWordW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, hkl: Param0, szreading: Param1, dwstyle: u32, szunregister: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), hkl.into_param().abi(), szreading.into_param().abi(), ::core::mem::transmute(dwstyle), szunregister.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GenerateMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), himc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn LockIMC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<*mut INPUTCONTEXT> {
        let mut result__: *mut INPUTCONTEXT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut INPUTCONTEXT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn UnlockIMC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), himc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetIMCLockCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn CreateIMCC(&self, dwsize: u32) -> ::windows::core::Result<super::super::super::Globalization::HIMCC> {
        let mut result__: super::super::super::Globalization::HIMCC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsize), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Globalization::HIMCC>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn DestroyIMCC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(&self, himcc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), himcc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn LockIMCC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(&self, himcc: Param0, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), himcc.into_param().abi(), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn UnlockIMCC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(&self, himcc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), himcc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn ReSizeIMCC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(&self, himcc: Param0, dwsize: u32) -> ::windows::core::Result<super::super::super::Globalization::HIMCC> {
        let mut result__: super::super::super::Globalization::HIMCC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), himcc.into_param().abi(), ::core::mem::transmute(dwsize), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Globalization::HIMCC>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetIMCCSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(&self, himcc: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), himcc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetIMCCLockCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(&self, himcc: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), himcc.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetHotKey(&self, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwhotkeyid), ::core::mem::transmute(pumodifiers), ::core::mem::transmute(puvkey), ::core::mem::transmute(phkl)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn SetHotKey<'a, Param3: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwhotkeyid), ::core::mem::transmute(umodifiers), ::core::mem::transmute(uvkey), hkl.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftKeyboard<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, utype: u32, howner: Param1, x: i32, y: i32) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: super::super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), ::core::mem::transmute(utype), howner.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroySoftKeyboard<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hsoftkbdwnd: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), hsoftkbdwnd.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowSoftKeyboard<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hsoftkbdwnd: Param0, ncmdshow: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), hsoftkbdwnd.into_param().abi(), ::core::mem::transmute(ncmdshow)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetCodePageA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub unsafe fn GetLangId<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(&self, hkl: Param0) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), hkl.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn KeybdEvent(&self, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), ::core::mem::transmute(lgidime), ::core::mem::transmute(bvk), ::core::mem::transmute(bscan), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwextrainfo)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn LockModal(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn UnlockModal(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn AssociateContextEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, hwnd: Param0, himc: Param1, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), himc.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn DisableIME(&self, idthread: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).82)(::core::mem::transmute_copy(self), ::core::mem::transmute(idthread)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImeMenuItemsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).83)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtype), ::core::mem::transmute(pimeparentmenu), ::core::mem::transmute(pimemenu), ::core::mem::transmute(dwsize), ::core::mem::transmute(pdwresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetImeMenuItemsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(&self, himc: Param0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).84)(::core::mem::transmute_copy(self), himc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtype), ::core::mem::transmute(pimeparentmenu), ::core::mem::transmute(pimemenu), ::core::mem::transmute(dwsize), ::core::mem::transmute(pdwresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn EnumInputContext(&self, idthread: u32) -> ::windows::core::Result<IEnumInputContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).85)(::core::mem::transmute_copy(self), ::core::mem::transmute(idthread), ::core::mem::transmute(&mut result__)).from_abi::<IEnumInputContext>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn RequestMessageA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, himc: Param0, wparam: Param1, lparam: Param2) -> ::windows::core::Result<super::super::super::Foundation::LRESULT> {
        let mut result__: super::super::super::Foundation::LRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).86)(::core::mem::transmute_copy(self), himc.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::LRESULT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub unsafe fn RequestMessageW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, himc: Param0, wparam: Param1, lparam: Param2) -> ::windows::core::Result<super::super::super::Foundation::LRESULT> {
        let mut result__: super::super::super::Foundation::LRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).87)(::core::mem::transmute_copy(self), himc.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::LRESULT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendIMCA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwnd: Param0, umsg: u32, wparam: Param2, lparam: Param3) -> ::windows::core::Result<super::super::super::Foundation::LRESULT> {
        let mut result__: super::super::super::Foundation::LRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).88)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(umsg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::LRESULT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendIMCW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hwnd: Param0, umsg: u32, wparam: Param2, lparam: Param3) -> ::windows::core::Result<super::super::super::Foundation::LRESULT> {
        let mut result__: super::super::super::Foundation::LRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).89)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(umsg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::LRESULT>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn IsSleeping(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).90)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IActiveIMMIME> for ::windows::core::IUnknown {
    fn from(value: IActiveIMMIME) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIMMIME> for ::windows::core::IUnknown {
    fn from(value: &IActiveIMMIME) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActiveIMMIME {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IActiveIMMIME {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveIMMIME {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveIMMIME {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveIMMIME {}
impl ::core::fmt::Debug for IActiveIMMIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveIMMIME").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveIMMIME {
    type Vtable = IActiveIMMIMEVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c03411_f96b_11d0_a475_00aa006bcc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMIMEVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szregister: super::super::super::Foundation::PSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR, pdata: *const ::core::ffi::c_void, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: super::super::super::Foundation::PSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: super::super::super::Foundation::PWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: super::super::super::Foundation::PWSTR, pdwresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: super::super::super::Foundation::PSTR, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: super::super::super::Foundation::PWSTR, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_UI_TextServices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_UI_TextServices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szimefilename: super::super::super::Foundation::PSTR, szlayouttext: super::super::super::Foundation::PSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szimefilename: super::super::super::Foundation::PWSTR, szlayouttext: super::super::super::Foundation::PWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_UI_TextServices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szregister: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szregister: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PSTR, dwstyle: u32, szunregister: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: super::super::super::Foundation::PWSTR, dwstyle: u32, szunregister: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, ppimc: *mut *mut INPUTCONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlockcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwlockcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    #[cfg(feature = "Win32_UI_TextServices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))] usize,
    #[cfg(feature = "Win32_UI_TextServices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: super::super::TextServices::HKL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, utype: u32, howner: super::super::super::Foundation::HWND, x: i32, y: i32, phsoftkbdwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND, ncmdshow: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_UI_TextServices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))] usize,
    #[cfg(feature = "Win32_UI_TextServices")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_TextServices"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Globalization")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IActiveIMMMessagePumpOwner(::windows::core::IUnknown);
impl IActiveIMMMessagePumpOwner {
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn End(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn OnTranslateMessage(&self, pmsg: *const super::super::WindowsAndMessaging::MSG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Pause(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Resume(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
}
impl ::core::convert::From<IActiveIMMMessagePumpOwner> for ::windows::core::IUnknown {
    fn from(value: IActiveIMMMessagePumpOwner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIMMMessagePumpOwner> for ::windows::core::IUnknown {
    fn from(value: &IActiveIMMMessagePumpOwner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActiveIMMMessagePumpOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IActiveIMMMessagePumpOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveIMMMessagePumpOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveIMMMessagePumpOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveIMMMessagePumpOwner {}
impl ::core::fmt::Debug for IActiveIMMMessagePumpOwner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveIMMMessagePumpOwner").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveIMMMessagePumpOwner {
    type Vtable = IActiveIMMMessagePumpOwnerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5cf2cfa_8aeb_11d1_9364_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMMessagePumpOwnerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const super::super::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IActiveIMMRegistrar(::windows::core::IUnknown);
impl IActiveIMMRegistrar {
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterIME<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, rclsid: *const ::windows::core::GUID, lgid: u16, psziconfile: Param2, pszdesc: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(lgid), psziconfile.into_param().abi(), pszdesc.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn UnregisterIME(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid)).ok()
    }
}
impl ::core::convert::From<IActiveIMMRegistrar> for ::windows::core::IUnknown {
    fn from(value: IActiveIMMRegistrar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActiveIMMRegistrar> for ::windows::core::IUnknown {
    fn from(value: &IActiveIMMRegistrar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActiveIMMRegistrar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IActiveIMMRegistrar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActiveIMMRegistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActiveIMMRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveIMMRegistrar {}
impl ::core::fmt::Debug for IActiveIMMRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveIMMRegistrar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveIMMRegistrar {
    type Vtable = IActiveIMMRegistrarVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3458082_bd00_11d1_939b_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMRegistrarVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, lgid: u16, psziconfile: super::super::super::Foundation::PWSTR, pszdesc: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IEnumInputContext(::windows::core::IUnknown);
impl IEnumInputContext {
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumInputContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumInputContext>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn Next(&self, ulcount: u32, rginputcontext: *mut super::super::super::Globalization::HIMC, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rginputcontext), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumInputContext> for ::windows::core::IUnknown {
    fn from(value: IEnumInputContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumInputContext> for ::windows::core::IUnknown {
    fn from(value: &IEnumInputContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumInputContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumInputContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumInputContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumInputContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumInputContext {}
impl ::core::fmt::Debug for IEnumInputContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumInputContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumInputContext {
    type Vtable = IEnumInputContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09b5eab0_f997_11d1_93d4_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumInputContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rginputcontext: *mut super::super::super::Globalization::HIMC, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IEnumRegisterWordA(::windows::core::IUnknown);
impl IEnumRegisterWordA {
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumRegisterWordA> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumRegisterWordA>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgregisterword), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumRegisterWordA> for ::windows::core::IUnknown {
    fn from(value: IEnumRegisterWordA) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumRegisterWordA> for ::windows::core::IUnknown {
    fn from(value: &IEnumRegisterWordA) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumRegisterWordA {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumRegisterWordA {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumRegisterWordA {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumRegisterWordA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumRegisterWordA {}
impl ::core::fmt::Debug for IEnumRegisterWordA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumRegisterWordA").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumRegisterWordA {
    type Vtable = IEnumRegisterWordAVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c03412_f96b_11d0_a475_00aa006bcc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRegisterWordAVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IEnumRegisterWordW(::windows::core::IUnknown);
impl IEnumRegisterWordW {
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumRegisterWordW> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumRegisterWordW>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount), ::core::mem::transmute(rgregisterword), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulcount)).ok()
    }
}
impl ::core::convert::From<IEnumRegisterWordW> for ::windows::core::IUnknown {
    fn from(value: IEnumRegisterWordW) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumRegisterWordW> for ::windows::core::IUnknown {
    fn from(value: &IEnumRegisterWordW) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumRegisterWordW {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumRegisterWordW {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumRegisterWordW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumRegisterWordW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumRegisterWordW {}
impl ::core::fmt::Debug for IEnumRegisterWordW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumRegisterWordW").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumRegisterWordW {
    type Vtable = IEnumRegisterWordWVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4955dd31_b159_11d0_8fcf_00aa006bcc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRegisterWordWVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFEC_S_ALREADY_DEFAULT: ::windows::core::HRESULT = ::windows::core::HRESULT(291840i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IFEClassFactory(::windows::core::IUnknown);
impl IFEClassFactory {
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, punkouter: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn LockServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, flock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), flock.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFEClassFactory> for super::super::super::System::Com::IClassFactory {
    fn from(value: IFEClassFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFEClassFactory> for super::super::super::System::Com::IClassFactory {
    fn from(value: &IFEClassFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IClassFactory> for IFEClassFactory {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IClassFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::System::Com::IClassFactory> for &IFEClassFactory {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::System::Com::IClassFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFEClassFactory> for ::windows::core::IUnknown {
    fn from(value: IFEClassFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFEClassFactory> for ::windows::core::IUnknown {
    fn from(value: &IFEClassFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFEClassFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFEClassFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFEClassFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFEClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFEClassFactory {}
impl ::core::fmt::Debug for IFEClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFEClassFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFEClassFactory {
    type Vtable = IFEClassFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFEClassFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IFECommon(::windows::core::IUnknown);
impl IFECommon {
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDefaultIME<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, szname: Param0, cszname: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szname.into_param().abi(), ::core::mem::transmute(cszname)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn SetDefaultIME(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeWordRegDialog(&self, pimedlg: *mut IMEDLG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pimedlg)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeDictToolDialog(&self, pimedlg: *mut IMEDLG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pimedlg)).ok()
    }
}
impl ::core::convert::From<IFECommon> for ::windows::core::IUnknown {
    fn from(value: IFECommon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFECommon> for ::windows::core::IUnknown {
    fn from(value: &IFECommon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFECommon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFECommon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFECommon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFECommon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFECommon {}
impl ::core::fmt::Debug for IFECommon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFECommon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFECommon {
    type Vtable = IFECommonVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x019f7151_e6db_11d0_83c3_00c04fddb82e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFECommonVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: super::super::super::Foundation::PSTR, cszname: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_E_INVALID_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147192063i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_E_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147192064i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_E_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147192057i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_E_NOT_USER_DIC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147192058i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_E_NO_ENTRY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147192060i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_E_OPEN_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147192062i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_E_REGISTER_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147192053i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_E_REGISTER_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147192059i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_E_REGISTER_ILLEGAL_POS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147192055i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_E_REGISTER_IMPROPER_WORD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147192054i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_E_USER_COMMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147192056i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_E_WRITE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147192061i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_ADJECTIVE: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_ADJECTIVE_VERB: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_ADNOUN: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_ADVERB: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_AFFIX: u32 = 1536u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_ALL: u32 = 131071u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_AUXILIARY_VERB: u32 = 32768u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_CONJUNCTION: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_DEPENDENT: u32 = 114688u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_IDIOMS: u32 = 4096u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_INDEPENDENT: u32 = 255u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_INFLECTIONALSUFFIX: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_INTERJECTION: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_NOUN: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_PARTICLE: u32 = 16384u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_PREFIX: u32 = 512u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_SUB_VERB: u32 = 65536u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_SUFFIX: u32 = 1024u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_SYMBOLS: u32 = 8192u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_TANKANJI: u32 = 2048u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_POS_VERB: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REG_ALL: u32 = 7u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REG_AUTO: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REG_GRAMMAR: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REG_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REG_USER: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_SELECT_ALL: u32 = 15u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_SELECT_COMMENT: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_SELECT_DISPLAY: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_SELECT_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_SELECT_POS: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_SELECT_READING: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_S_COMMENT_CHANGED: ::windows::core::HRESULT = ::windows::core::HRESULT(291331i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_S_EMPTY_DICTIONARY: ::windows::core::HRESULT = ::windows::core::HRESULT(291329i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_S_MORE_ENTRIES: ::windows::core::HRESULT = ::windows::core::HRESULT(291328i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_S_WORD_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(291330i32);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_TYPE_ALL: u32 = 31u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_TYPE_ENGLISH: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_TYPE_GENERAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_TYPE_NAMEPLACE: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_TYPE_REVERSE: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_TYPE_SPEECH: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IFEDictionary(::windows::core::IUnknown);
impl IFEDictionary {
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, pchdictpath: Param0, pshf: *mut IMESHF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pchdictpath.into_param().abi(), ::core::mem::transmute(pshf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHeader<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, pchdictpath: Param0, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pchdictpath.into_param().abi(), ::core::mem::transmute(pshf), ::core::mem::transmute(pjfmt), ::core::mem::transmute(pultype)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), hwnd.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn GetPosTable(&self, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prgpostbl), ::core::mem::transmute(pcpostbl)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWords<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwchfirst: Param0, pwchlast: Param1, pwchdisplay: Param2, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwchfirst.into_param().abi(), pwchlast.into_param().abi(), pwchdisplay.into_param().abi(), ::core::mem::transmute(ulpos), ::core::mem::transmute(ulselect), ::core::mem::transmute(ulwordsrc), ::core::mem::transmute(pchbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(pcwrd)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn NextWords(&self, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pchbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(pcwrd)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, pchdictpath: Param0, pshf: *mut IMESHF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pchdictpath.into_param().abi(), ::core::mem::transmute(pshf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHeader(&self, pshf: *mut IMESHF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pshf)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistWord(&self, pwrd: *mut IMEWRD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwrd)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExistDependency(&self, pdp: *mut IMEDP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdp)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterWord(&self, reg: IMEREG, pwrd: *mut IMEWRD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(reg), ::core::mem::transmute(pwrd)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterDependency(&self, reg: IMEREG, pdp: *mut IMEDP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(reg), ::core::mem::transmute(pdp)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDependencies<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwchkakarireading: Param0, pwchkakaridisplay: Param1, ulkakaripos: u32, pwchukereading: Param3, pwchukedisplay: Param4, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pwchkakarireading.into_param().abi(), pwchkakaridisplay.into_param().abi(), ::core::mem::transmute(ulkakaripos), pwchukereading.into_param().abi(), pwchukedisplay.into_param().abi(), ::core::mem::transmute(ulukepos), ::core::mem::transmute(jrel), ::core::mem::transmute(ulwordsrc), ::core::mem::transmute(pchbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(pcdp)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn NextDependencies(&self, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pchbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(pcdp)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConvertFromOldMSIME<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(&self, pchdic: Param0, pfnlog: PFNLOG, reg: IMEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pchdic.into_param().abi(), ::core::mem::transmute(pfnlog), ::core::mem::transmute(reg)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn ConvertFromUserToSys(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IFEDictionary> for ::windows::core::IUnknown {
    fn from(value: IFEDictionary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFEDictionary> for ::windows::core::IUnknown {
    fn from(value: &IFEDictionary) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFEDictionary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFEDictionary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFEDictionary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFEDictionary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFEDictionary {}
impl ::core::fmt::Debug for IFEDictionary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFEDictionary").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFEDictionary {
    type Vtable = IFEDictionaryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x019f7153_e6db_11d0_83c3_00c04fddb82e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFEDictionaryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchdictpath: super::super::super::Foundation::PSTR, pshf: *mut IMESHF) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchdictpath: super::super::super::Foundation::PSTR, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchfirst: super::super::super::Foundation::PWSTR, pwchlast: super::super::super::Foundation::PWSTR, pwchdisplay: super::super::super::Foundation::PWSTR, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchdictpath: super::super::super::Foundation::PSTR, pshf: *mut IMESHF) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshf: *mut IMESHF) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwrd: *mut IMEWRD) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdp: *mut IMEDP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reg: IMEREG, pwrd: *mut IMEWRD) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reg: IMEREG, pdp: *mut IMEDP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchkakarireading: super::super::super::Foundation::PWSTR, pwchkakaridisplay: super::super::super::Foundation::PWSTR, ulkakaripos: u32, pwchukereading: super::super::super::Foundation::PWSTR, pwchukedisplay: super::super::super::Foundation::PWSTR, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchdic: super::super::super::Foundation::PSTR, pfnlog: ::windows::core::RawPtr, reg: IMEREG) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IFELanguage(::windows::core::IUnknown);
impl IFELanguage {
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetJMorphResult<'a, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: Param3, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwrequest), ::core::mem::transmute(dwcmode), ::core::mem::transmute(cwchinput), pwchinput.into_param().abi(), ::core::mem::transmute(pfcinfo), ::core::mem::transmute(ppresult)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn GetConversionModeCaps(&self, pdwcaps: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcaps)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPhonetic<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, string: Param0, start: i32, length: i32, phonetic: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), string.into_param().abi(), ::core::mem::transmute(start), ::core::mem::transmute(length), ::core::mem::transmute(phonetic)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConversion<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, string: Param0, start: i32, length: i32, result: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), string.into_param().abi(), ::core::mem::transmute(start), ::core::mem::transmute(length), ::core::mem::transmute(result)).ok()
    }
}
impl ::core::convert::From<IFELanguage> for ::windows::core::IUnknown {
    fn from(value: IFELanguage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFELanguage> for ::windows::core::IUnknown {
    fn from(value: &IFELanguage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFELanguage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFELanguage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFELanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFELanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFELanguage {}
impl ::core::fmt::Debug for IFELanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFELanguage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFELanguage {
    type Vtable = IFELanguageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x019f7152_e6db_11d0_83c3_00c04fddb82e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFELanguageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: super::super::super::Foundation::PWSTR, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcaps: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, start: i32, length: i32, phonetic: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, string: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, start: i32, length: i32, result: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IGIMIF_RIGHTMENU: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IGIMII_CMODE: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IGIMII_CONFIGURE: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IGIMII_HELP: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IGIMII_INPUTTOOLS: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IGIMII_OTHER: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IGIMII_SMODE: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IGIMII_TOOLS: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IImePad(::windows::core::IUnknown);
impl IImePad {
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Request<'a, Param0: ::windows::core::IntoParam<'a, IImePadApplet>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, piimepadapplet: Param0, reqid: IME_PAD_REQUEST_FLAGS, wparam: Param2, lparam: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), piimepadapplet.into_param().abi(), ::core::mem::transmute(reqid), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IImePad> for ::windows::core::IUnknown {
    fn from(value: IImePad) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImePad> for ::windows::core::IUnknown {
    fn from(value: &IImePad) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IImePad {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IImePad {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IImePad {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImePad {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImePad {}
impl ::core::fmt::Debug for IImePad {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImePad").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IImePad {
    type Vtable = IImePadVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d8e643a_c3a9_11d1_afef_00805f0c8b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImePadVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piimepadapplet: ::windows::core::RawPtr, reqid: IME_PAD_REQUEST_FLAGS, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IImePadApplet(::windows::core::IUnknown);
impl IImePadApplet {
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, lpiimepad: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), lpiimepad.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn Terminate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetAppletConfig(&self, lpappletcfg: *mut IMEAPPLETCFG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpappletcfg)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateUI<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwndparent: Param0, lpimeappletui: *mut IMEAPPLETUI) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(lpimeappletui)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Notify<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, lpimepad: Param0, notify: i32, wparam: Param2, lparam: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), lpimepad.into_param().abi(), ::core::mem::transmute(notify), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IImePadApplet> for ::windows::core::IUnknown {
    fn from(value: IImePadApplet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImePadApplet> for ::windows::core::IUnknown {
    fn from(value: &IImePadApplet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IImePadApplet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IImePadApplet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IImePadApplet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImePadApplet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImePadApplet {}
impl ::core::fmt::Debug for IImePadApplet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImePadApplet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IImePadApplet {
    type Vtable = IImePadAppletVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d8e643b_c3a9_11d1_afef_00805f0c8b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImePadAppletVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiimepad: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpappletcfg: *mut IMEAPPLETCFG) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::Foundation::HWND, lpimeappletui: *mut IMEAPPLETUI) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpimepad: *mut ::core::ffi::c_void, notify: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IImePlugInDictDictionaryList(::windows::core::IUnknown);
impl IImePlugInDictDictionaryList {
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDictionariesInUse(&self, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(prgdictionaryguid), ::core::mem::transmute(prgdatecreated), ::core::mem::transmute(prgfencrypted)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteDictionary<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrdictionaryguid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrdictionaryguid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IImePlugInDictDictionaryList> for ::windows::core::IUnknown {
    fn from(value: IImePlugInDictDictionaryList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImePlugInDictDictionaryList> for ::windows::core::IUnknown {
    fn from(value: &IImePlugInDictDictionaryList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IImePlugInDictDictionaryList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IImePlugInDictDictionaryList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IImePlugInDictDictionaryList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImePlugInDictDictionaryList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImePlugInDictDictionaryList {}
impl ::core::fmt::Debug for IImePlugInDictDictionaryList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImePlugInDictDictionaryList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IImePlugInDictDictionaryList {
    type Vtable = IImePlugInDictDictionaryListVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98752974_b0a6_489b_8f6f_bff3769c8eeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImePlugInDictDictionaryListVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdictionaryguid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
#[repr(transparent)]
pub struct IImeSpecifyApplets(::windows::core::IUnknown);
impl IImeSpecifyApplets {
    #[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
    pub unsafe fn GetAppletIIDList(&self, refiid: *const ::windows::core::GUID, lpiidlist: *mut APPLETIDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(refiid), ::core::mem::transmute(lpiidlist)).ok()
    }
}
impl ::core::convert::From<IImeSpecifyApplets> for ::windows::core::IUnknown {
    fn from(value: IImeSpecifyApplets) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImeSpecifyApplets> for ::windows::core::IUnknown {
    fn from(value: &IImeSpecifyApplets) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IImeSpecifyApplets {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IImeSpecifyApplets {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IImeSpecifyApplets {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImeSpecifyApplets {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImeSpecifyApplets {}
impl ::core::fmt::Debug for IImeSpecifyApplets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImeSpecifyApplets").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IImeSpecifyApplets {
    type Vtable = IImeSpecifyAppletsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d8e643c_c3a9_11d1_afef_00805f0c8b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImeSpecifyAppletsVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, lpiidlist: *mut APPLETIDLIST) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
pub type IMCENUMPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_CLOSESTATUSWINDOW: u32 = 33u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_GETCANDIDATEPOS: u32 = 7u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_GETCOMPOSITIONFONT: u32 = 9u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_GETCOMPOSITIONWINDOW: u32 = 11u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_GETSOFTKBDFONT: u32 = 17u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_GETSOFTKBDPOS: u32 = 19u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_GETSOFTKBDSUBTYPE: u32 = 21u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_GETSTATUSWINDOWPOS: u32 = 15u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_OPENSTATUSWINDOW: u32 = 34u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_SETCANDIDATEPOS: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_SETCOMPOSITIONFONT: u32 = 10u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_SETCOMPOSITIONWINDOW: u32 = 12u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_SETCONVERSIONMODE: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_SETOPENSTATUS: u32 = 6u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_SETSENTENCEMODE: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_SETSOFTKBDDATA: u32 = 24u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_SETSOFTKBDFONT: u32 = 18u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_SETSOFTKBDPOS: u32 = 20u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_SETSOFTKBDSUBTYPE: u32 = 22u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMC_SETSTATUSWINDOWPOS: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct IMEAPPLETCFG {
    pub dwConfig: u32,
    pub wchTitle: [u16; 64],
    pub wchTitleFontFace: [u16; 32],
    pub dwCharSet: u32,
    pub iCategory: i32,
    pub hIcon: super::super::WindowsAndMessaging::HICON,
    pub langID: u16,
    pub dummy: u16,
    pub lReserved1: super::super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for IMEAPPLETCFG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for IMEAPPLETCFG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for IMEAPPLETCFG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEAPPLETCFG").field("dwConfig", &self.dwConfig).field("wchTitle", &self.wchTitle).field("wchTitleFontFace", &self.wchTitleFontFace).field("dwCharSet", &self.dwCharSet).field("iCategory", &self.iCategory).field("hIcon", &self.hIcon).field("langID", &self.langID).field("dummy", &self.dummy).field("lReserved1", &self.lReserved1).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for IMEAPPLETCFG {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for IMEAPPLETCFG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEAPPLETCFG>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for IMEAPPLETCFG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for IMEAPPLETCFG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEAPPLETUI {
    pub hwnd: super::super::super::Foundation::HWND,
    pub dwStyle: u32,
    pub width: i32,
    pub height: i32,
    pub minWidth: i32,
    pub minHeight: i32,
    pub maxWidth: i32,
    pub maxHeight: i32,
    pub lReserved1: super::super::super::Foundation::LPARAM,
    pub lReserved2: super::super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEAPPLETUI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEAPPLETUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMEAPPLETUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEAPPLETUI").field("hwnd", &self.hwnd).field("dwStyle", &self.dwStyle).field("width", &self.width).field("height", &self.height).field("minWidth", &self.minWidth).field("minHeight", &self.minHeight).field("maxWidth", &self.maxWidth).field("maxHeight", &self.maxHeight).field("lReserved1", &self.lReserved1).field("lReserved2", &self.lReserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMEAPPLETUI {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEAPPLETUI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEAPPLETUI>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEAPPLETUI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEAPPLETUI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct IMECHARINFO {
    pub wch: u16,
    pub dwCharInfo: u32,
}
impl ::core::marker::Copy for IMECHARINFO {}
impl ::core::clone::Clone for IMECHARINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMECHARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMECHARINFO").field("wch", &self.wch).field("dwCharInfo", &self.dwCharInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for IMECHARINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMECHARINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMECHARINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMECHARINFO {}
impl ::core::default::Default for IMECHARINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMECHARPOSITION {
    pub dwSize: u32,
    pub dwCharPos: u32,
    pub pt: super::super::super::Foundation::POINT,
    pub cLineHeight: u32,
    pub rcDocument: super::super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMECHARPOSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMECHARPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMECHARPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMECHARPOSITION").field("dwSize", &self.dwSize).field("dwCharPos", &self.dwCharPos).field("pt", &self.pt).field("cLineHeight", &self.cLineHeight).field("rcDocument", &self.rcDocument).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMECHARPOSITION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMECHARPOSITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMECHARPOSITION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMECHARPOSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMECHARPOSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct IMECOMPOSITIONSTRINGINFO {
    pub iCompStrLen: i32,
    pub iCaretPos: i32,
    pub iEditStart: i32,
    pub iEditLen: i32,
    pub iTargetStart: i32,
    pub iTargetLen: i32,
}
impl ::core::marker::Copy for IMECOMPOSITIONSTRINGINFO {}
impl ::core::clone::Clone for IMECOMPOSITIONSTRINGINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMECOMPOSITIONSTRINGINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMECOMPOSITIONSTRINGINFO").field("iCompStrLen", &self.iCompStrLen).field("iCaretPos", &self.iCaretPos).field("iEditStart", &self.iEditStart).field("iEditLen", &self.iEditLen).field("iTargetStart", &self.iTargetStart).field("iTargetLen", &self.iTargetLen).finish()
    }
}
unsafe impl ::windows::core::Abi for IMECOMPOSITIONSTRINGINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMECOMPOSITIONSTRINGINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMECOMPOSITIONSTRINGINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMECOMPOSITIONSTRINGINFO {}
impl ::core::default::Default for IMECOMPOSITIONSTRINGINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEDLG {
    pub cbIMEDLG: i32,
    pub hwnd: super::super::super::Foundation::HWND,
    pub lpwstrWord: super::super::super::Foundation::PWSTR,
    pub nTabId: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEDLG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEDLG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMEDLG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEDLG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEDLG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEDLG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEDLG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEDP {
    pub wrdModifier: IMEWRD,
    pub wrdModifiee: IMEWRD,
    pub relID: IMEREL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEDP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEDP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMEDP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEDP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEDP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEDP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEDP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEFAREASTINFO_TYPE_COMMENT: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEFAREASTINFO_TYPE_COSTTIME: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEFAREASTINFO_TYPE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEFAREASTINFO_TYPE_READING: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type IMEFMT = i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_UNKNOWN: IMEFMT = 0i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME2_BIN_SYSTEM: IMEFMT = 1i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME2_BIN_USER: IMEFMT = 2i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME2_TEXT_USER: IMEFMT = 3i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME95_BIN_SYSTEM: IMEFMT = 4i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME95_BIN_USER: IMEFMT = 5i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME95_TEXT_USER: IMEFMT = 6i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME97_BIN_SYSTEM: IMEFMT = 7i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME97_BIN_USER: IMEFMT = 8i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME97_TEXT_USER: IMEFMT = 9i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME98_BIN_SYSTEM: IMEFMT = 10i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME98_BIN_USER: IMEFMT = 11i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME98_TEXT_USER: IMEFMT = 12i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_ACTIVE_DICT: IMEFMT = 13i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_ATOK9: IMEFMT = 14i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_ATOK10: IMEFMT = 15i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_NEC_AI_: IMEFMT = 16i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_WX_II: IMEFMT = 17i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_WX_III: IMEFMT = 18i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_VJE_20: IMEFMT = 19i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME98_SYSTEM_CE: IMEFMT = 20i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME_BIN_SYSTEM: IMEFMT = 21i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME_BIN_USER: IMEFMT = 22i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_MSIME_TEXT_USER: IMEFMT = 23i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_PIME2_BIN_USER: IMEFMT = 24i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_PIME2_BIN_SYSTEM: IMEFMT = 25i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_PIME2_BIN_STANDARD_SYSTEM: IMEFMT = 26i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct IMEINFO {
    pub dwPrivateDataSize: u32,
    pub fdwProperty: u32,
    pub fdwConversionCaps: u32,
    pub fdwSentenceCaps: u32,
    pub fdwUICaps: u32,
    pub fdwSCSCaps: u32,
    pub fdwSelectCaps: u32,
}
impl ::core::marker::Copy for IMEINFO {}
impl ::core::clone::Clone for IMEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEINFO").field("dwPrivateDataSize", &self.dwPrivateDataSize).field("fdwProperty", &self.fdwProperty).field("fdwConversionCaps", &self.fdwConversionCaps).field("fdwSentenceCaps", &self.fdwSentenceCaps).field("fdwUICaps", &self.fdwUICaps).field("fdwSCSCaps", &self.fdwSCSCaps).field("fdwSelectCaps", &self.fdwSelectCaps).finish()
    }
}
unsafe impl ::windows::core::Abi for IMEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEINFO {}
impl ::core::default::Default for IMEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct IMEITEM {
    pub cbSize: i32,
    pub iType: i32,
    pub lpItemData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for IMEITEM {}
impl ::core::clone::Clone for IMEITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMEITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEITEM").field("cbSize", &self.cbSize).field("iType", &self.iType).field("lpItemData", &self.lpItemData).finish()
    }
}
unsafe impl ::windows::core::Abi for IMEITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEITEM {}
impl ::core::default::Default for IMEITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct IMEITEMCANDIDATE {
    pub uCount: u32,
    pub imeItem: [IMEITEM; 1],
}
impl ::core::marker::Copy for IMEITEMCANDIDATE {}
impl ::core::clone::Clone for IMEITEMCANDIDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMEITEMCANDIDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEITEMCANDIDATE").field("uCount", &self.uCount).field("imeItem", &self.imeItem).finish()
    }
}
unsafe impl ::windows::core::Abi for IMEITEMCANDIDATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEITEMCANDIDATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEITEMCANDIDATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEITEMCANDIDATE {}
impl ::core::default::Default for IMEITEMCANDIDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKEYCTRLMASK_ALT: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKEYCTRLMASK_CTRL: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKEYCTRLMASK_SHIFT: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKEYCTRL_DOWN: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKEYCTRL_UP: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
pub struct IMEKMS {
    pub cbSize: i32,
    pub hIMC: super::super::super::Globalization::HIMC,
    pub cKeyList: u32,
    pub pKeyList: *mut IMEKMSKEY,
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::marker::Copy for IMEKMS {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::clone::Clone for IMEKMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Globalization")]
unsafe impl ::windows::core::Abi for IMEKMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::cmp::PartialEq for IMEKMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::cmp::Eq for IMEKMS {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::default::Default for IMEKMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct IMEKMSFUNCDESC {
    pub cbSize: i32,
    pub idLang: u16,
    pub dwControl: u32,
    pub pwszDescription: [u16; 128],
}
impl ::core::marker::Copy for IMEKMSFUNCDESC {}
impl ::core::clone::Clone for IMEKMSFUNCDESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMEKMSFUNCDESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEKMSFUNCDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSFUNCDESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEKMSFUNCDESC {}
impl ::core::default::Default for IMEKMSFUNCDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEKMSINIT {
    pub cbSize: i32,
    pub hWnd: super::super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEKMSINIT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEKMSINIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMEKMSINIT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEKMSINIT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSINIT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEKMSINIT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEKMSINIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
pub struct IMEKMSINVK {
    pub cbSize: i32,
    pub hIMC: super::super::super::Globalization::HIMC,
    pub dwControl: u32,
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::marker::Copy for IMEKMSINVK {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::clone::Clone for IMEKMSINVK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Globalization")]
unsafe impl ::windows::core::Abi for IMEKMSINVK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::cmp::PartialEq for IMEKMSINVK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSINVK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::cmp::Eq for IMEKMSINVK {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::default::Default for IMEKMSINVK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct IMEKMSKEY {
    pub dwStatus: u32,
    pub dwCompStatus: u32,
    pub dwVKEY: u32,
    pub Anonymous1: IMEKMSKEY_0,
    pub Anonymous2: IMEKMSKEY_1,
}
impl ::core::marker::Copy for IMEKMSKEY {}
impl ::core::clone::Clone for IMEKMSKEY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMEKMSKEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEKMSKEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSKEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEKMSKEY {}
impl ::core::default::Default for IMEKMSKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub union IMEKMSKEY_0 {
    pub dwControl: u32,
    pub dwNotUsed: u32,
}
impl ::core::marker::Copy for IMEKMSKEY_0 {}
impl ::core::clone::Clone for IMEKMSKEY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMEKMSKEY_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEKMSKEY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSKEY_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEKMSKEY_0 {}
impl ::core::default::Default for IMEKMSKEY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub union IMEKMSKEY_1 {
    pub pwszDscr: [u16; 31],
    pub pwszNoUse: [u16; 31],
}
impl ::core::marker::Copy for IMEKMSKEY_1 {}
impl ::core::clone::Clone for IMEKMSKEY_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IMEKMSKEY_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMEKMSKEY_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSKEY_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMEKMSKEY_1 {}
impl ::core::default::Default for IMEKMSKEY_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
pub struct IMEKMSKMP {
    pub cbSize: i32,
    pub hIMC: super::super::super::Globalization::HIMC,
    pub idLang: u16,
    pub wVKStart: u16,
    pub wVKEnd: u16,
    pub cKeyList: i32,
    pub pKeyList: *mut IMEKMSKEY,
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::marker::Copy for IMEKMSKMP {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::clone::Clone for IMEKMSKMP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Globalization")]
unsafe impl ::windows::core::Abi for IMEKMSKMP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::cmp::PartialEq for IMEKMSKMP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSKMP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Globalization")]
impl ::core::cmp::Eq for IMEKMSKMP {}
#[cfg(feature = "Win32_Globalization")]
impl ::core::default::Default for IMEKMSKMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
pub struct IMEKMSNTFY {
    pub cbSize: i32,
    pub hIMC: super::super::super::Globalization::HIMC,
    pub fSelect: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::marker::Copy for IMEKMSNTFY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::clone::Clone for IMEKMSNTFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
unsafe impl ::windows::core::Abi for IMEKMSNTFY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::cmp::PartialEq for IMEKMSNTFY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEKMSNTFY>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::cmp::Eq for IMEKMSNTFY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
impl ::core::default::Default for IMEKMSNTFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKMS_2NDLEVEL: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKMS_CANDIDATE: u32 = 6u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKMS_COMPOSITION: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKMS_IMEOFF: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKMS_INPTGL: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKMS_NOCOMPOSITION: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKMS_SELECTION: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEKMS_TYPECAND: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct IMEMENUITEMINFOA {
    pub cbSize: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hbmpChecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: u32,
    pub szString: [super::super::super::Foundation::CHAR; 80],
    pub hbmpItem: super::super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for IMEMENUITEMINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for IMEMENUITEMINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for IMEMENUITEMINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEMENUITEMINFOA").field("cbSize", &self.cbSize).field("fType", &self.fType).field("fState", &self.fState).field("wID", &self.wID).field("hbmpChecked", &self.hbmpChecked).field("hbmpUnchecked", &self.hbmpUnchecked).field("dwItemData", &self.dwItemData).field("szString", &self.szString).field("hbmpItem", &self.hbmpItem).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for IMEMENUITEMINFOA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for IMEMENUITEMINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEMENUITEMINFOA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for IMEMENUITEMINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for IMEMENUITEMINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Graphics_Gdi'*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct IMEMENUITEMINFOW {
    pub cbSize: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hbmpChecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: u32,
    pub szString: [u16; 80],
    pub hbmpItem: super::super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for IMEMENUITEMINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for IMEMENUITEMINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for IMEMENUITEMINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMEMENUITEMINFOW").field("cbSize", &self.cbSize).field("fType", &self.fType).field("fState", &self.fState).field("wID", &self.wID).field("hbmpChecked", &self.hbmpChecked).field("hbmpUnchecked", &self.hbmpUnchecked).field("dwItemData", &self.dwItemData).field("szString", &self.szString).field("hbmpItem", &self.hbmpItem).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for IMEMENUITEMINFOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for IMEMENUITEMINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEMENUITEMINFOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for IMEMENUITEMINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for IMEMENUITEMINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEMENUITEM_STRING_SIZE: u32 = 80u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEMOUSERET_NOTHANDLED: i32 = -1i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEMOUSE_LDOWN: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEMOUSE_MDOWN: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEMOUSE_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEMOUSE_RDOWN: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEMOUSE_VERSION: u32 = 255u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEMOUSE_WDOWN: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEMOUSE_WUP: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_CARETBACKSPACE: u32 = 10u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_CARETBOTTOM: u32 = 9u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_CARETDELETE: u32 = 11u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_CARETLEFT: u32 = 6u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_CARETRIGHT: u32 = 7u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_CARETSET: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_CARETTOP: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_CLEARALL: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_CONVERTALL: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_DETERMINALL: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_DETERMINCHAR: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_INSERTFULLSPACE: u32 = 14u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_INSERTHALFSPACE: u32 = 15u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_INSERTSPACE: u32 = 13u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_OFFIME: u32 = 17u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_OFFPRECONVERSION: u32 = 19u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_ONIME: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_ONPRECONVERSION: u32 = 18u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_PHONETICCANDIDATE: u32 = 20u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADCTRL_PHRASEDELETE: u32 = 12u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_CHANGESTRINGCANDIDATEINFO: u32 = 4111u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_CHANGESTRINGINFO: u32 = 4115u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_FIRST: u32 = 4096u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_GETAPPLETDATA: u32 = 4106u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_GETCOMPOSITIONSTRINGID: u32 = 4109u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_GETCURRENTUILANGID: u32 = 4120u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_GETSELECTEDSTRING: u32 = 4103u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_INSERTITEMCANDIDATE: u32 = 4099u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_INSERTSTRINGCANDIDATE: u32 = 4098u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_INSERTSTRINGCANDIDATEINFO: u32 = 4110u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_INSERTSTRINGINFO: u32 = 4114u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_SENDKEYCONTROL: u32 = 4101u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_SETAPPLETDATA: u32 = 4105u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_SETTITLEFONT: u32 = 4107u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_ACTIVATE: u32 = 257u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_APPLYCAND: u32 = 267u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_APPLYCANDEX: u32 = 268u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_CONFIG: u32 = 264u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_FIRST: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_HELP: u32 = 265u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_HIDE: u32 = 261u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_INACTIVATE: u32 = 258u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_QUERYCAND: u32 = 266u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_SETTINGCHANGED: u32 = 269u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_SHOW: u32 = 260u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_SIZECHANGED: u32 = 263u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_SIZECHANGING: u32 = 262u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPN_USER: u32 = 356u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type IMEREG = i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REG_HEAD: IMEREG = 0i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REG_TAIL: IMEREG = 1i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REG_DEL: IMEREG = 2i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type IMEREL = i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_NONE: IMEREL = 0i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_NO: IMEREL = 1i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_GA: IMEREL = 2i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_WO: IMEREL = 3i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_NI: IMEREL = 4i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_DE: IMEREL = 5i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_YORI: IMEREL = 6i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_KARA: IMEREL = 7i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_MADE: IMEREL = 8i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_HE: IMEREL = 9i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_TO: IMEREL = 10i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_IDEOM: IMEREL = 11i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_FUKU_YOUGEN: IMEREL = 12i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_KEIYOU_YOUGEN: IMEREL = 13i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_KEIDOU1_YOUGEN: IMEREL = 14i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_KEIDOU2_YOUGEN: IMEREL = 15i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_TAIGEN: IMEREL = 16i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_YOUGEN: IMEREL = 17i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_RENTAI_MEI: IMEREL = 18i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_RENSOU: IMEREL = 19i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_KEIYOU_TO_YOUGEN: IMEREL = 20i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_KEIYOU_TARU_YOUGEN: IMEREL = 21i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_UNKNOWN1: IMEREL = 22i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_UNKNOWN2: IMEREL = 23i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_REL_ALL: IMEREL = 24i32;
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMESHF {
    pub cbShf: u16,
    pub verDic: u16,
    pub szTitle: [super::super::super::Foundation::CHAR; 48],
    pub szDescription: [super::super::super::Foundation::CHAR; 256],
    pub szCopyright: [super::super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMESHF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMESHF {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMESHF {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMESHF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMESHF>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMESHF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMESHF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMESTRINGCANDIDATE {
    pub uCount: u32,
    pub lpwstr: [super::super::super::Foundation::PWSTR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMESTRINGCANDIDATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMESTRINGCANDIDATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMESTRINGCANDIDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMESTRINGCANDIDATE").field("uCount", &self.uCount).field("lpwstr", &self.lpwstr).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMESTRINGCANDIDATE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMESTRINGCANDIDATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMESTRINGCANDIDATE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMESTRINGCANDIDATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMESTRINGCANDIDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMESTRINGCANDIDATEINFO {
    pub dwFarEastId: u32,
    pub lpFarEastInfo: *mut tabIMEFAREASTINFO,
    pub fInfoMask: u32,
    pub iSelIndex: i32,
    pub uCount: u32,
    pub lpwstr: [super::super::super::Foundation::PWSTR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMESTRINGCANDIDATEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMESTRINGCANDIDATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMESTRINGCANDIDATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMESTRINGCANDIDATEINFO").field("dwFarEastId", &self.dwFarEastId).field("lpFarEastInfo", &self.lpFarEastInfo).field("fInfoMask", &self.fInfoMask).field("iSelIndex", &self.iSelIndex).field("uCount", &self.uCount).field("lpwstr", &self.lpwstr).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMESTRINGCANDIDATEINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMESTRINGCANDIDATEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMESTRINGCANDIDATEINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMESTRINGCANDIDATEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMESTRINGCANDIDATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type IMEUCT = i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_UCT_NONE: IMEUCT = 0i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_UCT_STRING_SJIS: IMEUCT = 1i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_UCT_STRING_UNICODE: IMEUCT = 2i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_UCT_USER_DEFINED: IMEUCT = 3i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IFED_UCT_MAX: IMEUCT = 4i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEVER_0310: u32 = 196618u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEVER_0400: u32 = 262144u32;
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEWRD {
    pub pwchReading: super::super::super::Foundation::PWSTR,
    pub pwchDisplay: super::super::super::Foundation::PWSTR,
    pub Anonymous: IMEWRD_0,
    pub rgulAttrs: [u32; 2],
    pub cbComment: i32,
    pub uct: IMEUCT,
    pub pvComment: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEWRD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEWRD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMEWRD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEWRD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEWRD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEWRD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEWRD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union IMEWRD_0 {
    pub ulPos: u32,
    pub Anonymous: IMEWRD_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEWRD_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEWRD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMEWRD_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEWRD_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEWRD_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEWRD_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEWRD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEWRD_0_0 {
    pub nPos1: u16,
    pub nPos2: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEWRD_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEWRD_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IMEWRD_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMEWRD_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMEWRD_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMEWRD_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMEWRD_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CAND_CODE: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CAND_MEANING: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CAND_RADICAL: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CAND_READ: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CAND_STROKE: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CAND_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CHOTKEY_IME_NONIME_TOGGLE: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CHOTKEY_SHAPE_TOGGLE: u32 = 17u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CHOTKEY_SYMBOL_TOGGLE: u32 = 18u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CMODE_EUDC: u32 = 512u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CMODE_FIXED: u32 = 2048u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CMODE_NOCONVERSION: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CMODE_RESERVED: u32 = 4026531840u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CMODE_SOFTKBD: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CMODE_SYMBOL: u32 = 1024u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CONFIG_GENERAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CONFIG_REGISTERWORD: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_CONFIG_SELECTDICTIONARY: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_AUTOMATA: u32 = 4105u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_GETHELPFILENAME: u32 = 4107u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_GET_EUDC_DICTIONARY: u32 = 4099u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_HANJA_MODE: u32 = 4104u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_IME_NAME: u32 = 4102u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_MAX_KEY: u32 = 4101u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_PRIVATE_FIRST: u32 = 2048u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_PRIVATE_HOTKEY: u32 = 4106u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_PRIVATE_LAST: u32 = 4095u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_QUERY_SUPPORT: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_RESERVED_FIRST: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_RESERVED_LAST: u32 = 2047u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_SEQUENCE_TO_INTERNAL: u32 = 4097u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_SET_EUDC_DICTIONARY: u32 = 4100u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_STRING_BUFFER_SIZE: u32 = 80u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ESC_SYNC_HOTKEY: u32 = 4103u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_HOTKEY_DSWITCH_FIRST: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_HOTKEY_DSWITCH_LAST: u32 = 287u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_HOTKEY_PRIVATE_FIRST: u32 = 512u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_HOTKEY_PRIVATE_LAST: u32 = 543u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ITHOTKEY_PREVIOUS_COMPOSITION: u32 = 513u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ITHOTKEY_RECONVERTSTRING: u32 = 515u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ITHOTKEY_RESEND_RESULTSTR: u32 = 512u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_ITHOTKEY_UISTYLE_TOGGLE: u32 = 514u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_JHOTKEY_CLOSE_OPEN: u32 = 48u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_KHOTKEY_ENGLISH: u32 = 82u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_KHOTKEY_HANJACONVERT: u32 = 81u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_KHOTKEY_SHAPE_TOGGLE: u32 = 80u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type IME_PAD_REQUEST_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_INSERTSTRING: IME_PAD_REQUEST_FLAGS = 4097u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_SENDCONTROL: IME_PAD_REQUEST_FLAGS = 4100u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_SETAPPLETSIZE: IME_PAD_REQUEST_FLAGS = 4104u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_GETCOMPOSITIONSTRING: IME_PAD_REQUEST_FLAGS = 4102u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_GETCOMPOSITIONSTRINGINFO: IME_PAD_REQUEST_FLAGS = 4108u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_DELETESTRING: IME_PAD_REQUEST_FLAGS = 4112u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_CHANGESTRING: IME_PAD_REQUEST_FLAGS = 4113u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_GETAPPLHWND: IME_PAD_REQUEST_FLAGS = 4116u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_FORCEIMEPADWINDOWSHOW: IME_PAD_REQUEST_FLAGS = 4117u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_POSTMODALNOTIFY: IME_PAD_REQUEST_FLAGS = 4118u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_GETDEFAULTUILANGID: IME_PAD_REQUEST_FLAGS = 4119u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_GETAPPLETUISTYLE: IME_PAD_REQUEST_FLAGS = 4121u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_SETAPPLETUISTYLE: IME_PAD_REQUEST_FLAGS = 4122u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_ISAPPLETACTIVE: IME_PAD_REQUEST_FLAGS = 4123u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_ISIMEPADWINDOWVISIBLE: IME_PAD_REQUEST_FLAGS = 4124u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_SETAPPLETMINMAXSIZE: IME_PAD_REQUEST_FLAGS = 4125u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_GETCONVERSIONSTATUS: IME_PAD_REQUEST_FLAGS = 4126u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_GETVERSION: IME_PAD_REQUEST_FLAGS = 4127u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMEPADREQ_GETCURRENTIMEINFO: IME_PAD_REQUEST_FLAGS = 4128u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_PROP_ACCEPT_WIDE_VKEY: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_PROP_AT_CARET: u32 = 65536u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_PROP_CANDLIST_START_FROM_1: u32 = 262144u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_PROP_COMPLETE_ON_UNSELECT: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_PROP_END_UNLOAD: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_PROP_IGNORE_UPKEYS: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_PROP_KBD_CHAR_FIRST: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_PROP_NEED_ALTKEY: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_PROP_NO_KEYS_ON_CLOSE: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_PROP_SPECIAL_UI: u32 = 131072u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_PROP_UNICODE: u32 = 524288u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_REGWORD_STYLE_EUDC: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_REGWORD_STYLE_USER_FIRST: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_REGWORD_STYLE_USER_LAST: u32 = 4294967295u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_SMODE_AUTOMATIC: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_SMODE_CONVERSATION: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_SMODE_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_SMODE_PHRASEPREDICT: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_SMODE_PLAURALCLAUSE: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_SMODE_RESERVED: u32 = 61440u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_SMODE_SINGLECONVERT: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_SYSINFO_WINLOGON: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_SYSINFO_WOW16: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_THOTKEY_IME_NONIME_TOGGLE: u32 = 112u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_THOTKEY_SHAPE_TOGGLE: u32 = 113u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_THOTKEY_SYMBOL_TOGGLE: u32 = 114u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IME_UI_CLASS_NAME_SIZE: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMFT_RADIOCHECK: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMFT_SEPARATOR: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMFT_SUBMENU: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMMGWLP_IMC: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMMGWL_IMC: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMM_ERROR_GENERAL: i32 = -2i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMM_ERROR_NODATA: i32 = -1i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_CHANGECANDIDATE: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_CLOSECANDIDATE: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_CLOSESTATUSWINDOW: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_GUIDELINE: u32 = 13u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_OPENCANDIDATE: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_OPENSTATUSWINDOW: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_PRIVATE: u32 = 14u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_SETCANDIDATEPOS: u32 = 9u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_SETCOMPOSITIONFONT: u32 = 10u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_SETCOMPOSITIONWINDOW: u32 = 11u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_SETCONVERSIONMODE: u32 = 6u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_SETOPENSTATUS: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_SETSENTENCEMODE: u32 = 7u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_SETSTATUSWINDOWPOS: u32 = 12u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMN_SOFTKBDDESTROYED: u32 = 17u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMR_CANDIDATEWINDOW: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMR_COMPOSITIONFONT: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMR_COMPOSITIONWINDOW: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMR_CONFIRMRECONVERTSTRING: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMR_DOCUMENTFEED: u32 = 7u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMR_QUERYCHARPOSITION: u32 = 6u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IMR_RECONVERTSTRING: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INFOMASK_APPLY_CAND: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INFOMASK_APPLY_CAND_EX: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INFOMASK_BLOCK_CAND: u32 = 262144u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INFOMASK_HIDE_CAND: u32 = 131072u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INFOMASK_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INFOMASK_QUERY_CAND: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INFOMASK_STRING_FIX: u32 = 65536u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INIT_COMPFORM: u32 = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INIT_CONVERSION: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INIT_LOGFONT: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INIT_SENTENCE: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INIT_SOFTKBDPOS: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const INIT_STATUSWNDPOS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
pub struct INPUTCONTEXT {
    pub hWnd: super::super::super::Foundation::HWND,
    pub fOpen: super::super::super::Foundation::BOOL,
    pub ptStatusWndPos: super::super::super::Foundation::POINT,
    pub ptSoftKbdPos: super::super::super::Foundation::POINT,
    pub fdwConversion: u32,
    pub fdwSentence: u32,
    pub lfFont: INPUTCONTEXT_0,
    pub cfCompForm: COMPOSITIONFORM,
    pub cfCandForm: [CANDIDATEFORM; 4],
    pub hCompStr: super::super::super::Globalization::HIMCC,
    pub hCandInfo: super::super::super::Globalization::HIMCC,
    pub hGuideLine: super::super::super::Globalization::HIMCC,
    pub hPrivate: super::super::super::Globalization::HIMCC,
    pub dwNumMsgBuf: u32,
    pub hMsgBuf: super::super::super::Globalization::HIMCC,
    pub fdwInit: u32,
    pub dwReserve: [u32; 3],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for INPUTCONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for INPUTCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for INPUTCONTEXT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for INPUTCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUTCONTEXT>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for INPUTCONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for INPUTCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
pub union INPUTCONTEXT_0 {
    pub A: super::super::super::Graphics::Gdi::LOGFONTA,
    pub W: super::super::super::Graphics::Gdi::LOGFONTW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for INPUTCONTEXT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for INPUTCONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for INPUTCONTEXT_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for INPUTCONTEXT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUTCONTEXT_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for INPUTCONTEXT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for INPUTCONTEXT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACFG_CATEGORY: i32 = 262144i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACFG_HELP: i32 = 2i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACFG_LANG: i32 = 16i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACFG_NONE: i32 = 0i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACFG_PROPERTY: i32 = 1i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACFG_TITLE: i32 = 65536i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACFG_TITLEFONTFACE: i32 = 131072i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACID_CHARLIST: u32 = 9u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACID_EPWING: u32 = 7u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACID_HANDWRITING: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACID_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACID_OCR: u32 = 8u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACID_RADICALSEARCH: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACID_SOFTKEY: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACID_STROKESEARCH: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACID_SYMBOLSEARCH: u32 = 5u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACID_USER: u32 = 256u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPACID_VOICE: u32 = 6u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPAWS_ENABLED: i32 = 1i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPAWS_HORIZONTALFIXED: i32 = 512i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPAWS_MAXHEIGHTFIXED: i32 = 8192i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPAWS_MAXSIZEFIXED: i32 = 12288i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPAWS_MAXWIDTHFIXED: i32 = 4096i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPAWS_MINHEIGHTFIXED: i32 = 131072i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPAWS_MINSIZEFIXED: i32 = 196608i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPAWS_MINWIDTHFIXED: i32 = 65536i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPAWS_SIZEFIXED: i32 = 768i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPAWS_SIZINGNOTIFY: i32 = 4i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const IPAWS_VERTICALFIXED: i32 = 256i32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const ISC_SHOWUIALL: u32 = 3221225487u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const ISC_SHOWUIALLCANDIDATEWINDOW: u32 = 15u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const ISC_SHOWUICANDIDATEWINDOW: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const ISC_SHOWUICOMPOSITIONWINDOW: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const ISC_SHOWUIGUIDELINE: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmAssociateContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: Param1) -> super::super::super::Globalization::HIMC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmAssociateContext(param0: super::super::super::Foundation::HWND, param1: super::super::super::Globalization::HIMC) -> super::super::super::Globalization::HIMC;
        }
        ::core::mem::transmute(ImmAssociateContext(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmAssociateContextEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: Param1, param2: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmAssociateContextEx(param0: super::super::super::Foundation::HWND, param1: super::super::super::Globalization::HIMC, param2: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmAssociateContextEx(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmConfigureIMEA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0, param1: Param1, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmConfigureIMEA(param0: super::super::TextServices::HKL, param1: super::super::super::Foundation::HWND, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmConfigureIMEA(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmConfigureIMEW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0, param1: Param1, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmConfigureIMEW(param0: super::super::TextServices::HKL, param1: super::super::super::Foundation::HWND, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmConfigureIMEW(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmCreateContext() -> super::super::super::Globalization::HIMC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmCreateContext() -> super::super::super::Globalization::HIMC;
        }
        ::core::mem::transmute(ImmCreateContext())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmCreateIMCC(param0: u32) -> super::super::super::Globalization::HIMCC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmCreateIMCC(param0: u32) -> super::super::super::Globalization::HIMCC;
        }
        ::core::mem::transmute(ImmCreateIMCC(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmCreateSoftKeyboard<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: u32, param1: Param1, param2: i32, param3: i32) -> super::super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmCreateSoftKeyboard(param0: u32, param1: super::super::super::Foundation::HWND, param2: i32, param3: i32) -> super::super::super::Foundation::HWND;
        }
        ::core::mem::transmute(ImmCreateSoftKeyboard(::core::mem::transmute(param0), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmDestroyContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmDestroyContext(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmDestroyContext(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmDestroyIMCC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(param0: Param0) -> super::super::super::Globalization::HIMCC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmDestroyIMCC(param0: super::super::super::Globalization::HIMCC) -> super::super::super::Globalization::HIMCC;
        }
        ::core::mem::transmute(ImmDestroyIMCC(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmDestroySoftKeyboard<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmDestroySoftKeyboard(param0: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmDestroySoftKeyboard(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmDisableIME(param0: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmDisableIME(param0: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmDisableIME(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmDisableLegacyIME() -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmDisableLegacyIME() -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmDisableLegacyIME())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmDisableTextFrameService(idthread: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmDisableTextFrameService(idthread: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmDisableTextFrameService(::core::mem::transmute(idthread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmEnumInputContext<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(idthread: u32, lpfn: IMCENUMPROC, lparam: Param2) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmEnumInputContext(idthread: u32, lpfn: ::windows::core::RawPtr, lparam: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmEnumInputContext(::core::mem::transmute(idthread), ::core::mem::transmute(lpfn), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmEnumRegisterWordA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(param0: Param0, param1: REGISTERWORDENUMPROCA, lpszreading: Param2, param3: u32, lpszregister: Param4, param5: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmEnumRegisterWordA(param0: super::super::TextServices::HKL, param1: ::windows::core::RawPtr, lpszreading: super::super::super::Foundation::PSTR, param3: u32, lpszregister: super::super::super::Foundation::PSTR, param5: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(ImmEnumRegisterWordA(param0.into_param().abi(), ::core::mem::transmute(param1), lpszreading.into_param().abi(), ::core::mem::transmute(param3), lpszregister.into_param().abi(), ::core::mem::transmute(param5)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmEnumRegisterWordW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(param0: Param0, param1: REGISTERWORDENUMPROCW, lpszreading: Param2, param3: u32, lpszregister: Param4, param5: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmEnumRegisterWordW(param0: super::super::TextServices::HKL, param1: ::windows::core::RawPtr, lpszreading: super::super::super::Foundation::PWSTR, param3: u32, lpszregister: super::super::super::Foundation::PWSTR, param5: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(ImmEnumRegisterWordW(param0.into_param().abi(), ::core::mem::transmute(param1), lpszreading.into_param().abi(), ::core::mem::transmute(param3), lpszregister.into_param().abi(), ::core::mem::transmute(param5)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmEscapeA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: Param1, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmEscapeA(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(ImmEscapeA(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmEscapeW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: Param1, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmEscapeW(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, param2: u32, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(ImmEscapeW(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGenerateMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGenerateMessage(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGenerateMessage(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCandidateListA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, deindex: u32, lpcandlist: *mut CANDIDATELIST, dwbuflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCandidateListA(param0: super::super::super::Globalization::HIMC, deindex: u32, lpcandlist: *mut CANDIDATELIST, dwbuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetCandidateListA(param0.into_param().abi(), ::core::mem::transmute(deindex), ::core::mem::transmute(lpcandlist), ::core::mem::transmute(dwbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCandidateListCountA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpdwlistcount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCandidateListCountA(param0: super::super::super::Globalization::HIMC, lpdwlistcount: *mut u32) -> u32;
        }
        ::core::mem::transmute(ImmGetCandidateListCountA(param0.into_param().abi(), ::core::mem::transmute(lpdwlistcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCandidateListCountW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpdwlistcount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCandidateListCountW(param0: super::super::super::Globalization::HIMC, lpdwlistcount: *mut u32) -> u32;
        }
        ::core::mem::transmute(ImmGetCandidateListCountW(param0.into_param().abi(), ::core::mem::transmute(lpdwlistcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCandidateListW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, deindex: u32, lpcandlist: *mut CANDIDATELIST, dwbuflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCandidateListW(param0: super::super::super::Globalization::HIMC, deindex: u32, lpcandlist: *mut CANDIDATELIST, dwbuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetCandidateListW(param0.into_param().abi(), ::core::mem::transmute(deindex), ::core::mem::transmute(lpcandlist), ::core::mem::transmute(dwbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetCandidateWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: u32, lpcandidate: *mut CANDIDATEFORM) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCandidateWindow(param0: super::super::super::Globalization::HIMC, param1: u32, lpcandidate: *mut CANDIDATEFORM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetCandidateWindow(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(lpcandidate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmGetCompositionFontA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCompositionFontA(param0: super::super::super::Globalization::HIMC, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetCompositionFontA(param0.into_param().abi(), ::core::mem::transmute(lplf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmGetCompositionFontW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCompositionFontW(param0: super::super::super::Globalization::HIMC, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetCompositionFontW(param0.into_param().abi(), ::core::mem::transmute(lplf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCompositionStringA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: u32, lpbuf: *mut ::core::ffi::c_void, dwbuflen: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCompositionStringA(param0: super::super::super::Globalization::HIMC, param1: u32, lpbuf: *mut ::core::ffi::c_void, dwbuflen: u32) -> i32;
        }
        ::core::mem::transmute(ImmGetCompositionStringA(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(lpbuf), ::core::mem::transmute(dwbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetCompositionStringW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: u32, lpbuf: *mut ::core::ffi::c_void, dwbuflen: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCompositionStringW(param0: super::super::super::Globalization::HIMC, param1: u32, lpbuf: *mut ::core::ffi::c_void, dwbuflen: u32) -> i32;
        }
        ::core::mem::transmute(ImmGetCompositionStringW(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(lpbuf), ::core::mem::transmute(dwbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetCompositionWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpcompform: *mut COMPOSITIONFORM) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetCompositionWindow(param0: super::super::super::Globalization::HIMC, lpcompform: *mut COMPOSITIONFORM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetCompositionWindow(param0.into_param().abi(), ::core::mem::transmute(lpcompform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0) -> super::super::super::Globalization::HIMC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetContext(param0: super::super::super::Foundation::HWND) -> super::super::super::Globalization::HIMC;
        }
        ::core::mem::transmute(ImmGetContext(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetConversionListA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(param0: Param0, param1: Param1, lpsrc: Param2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetConversionListA(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, lpsrc: super::super::super::Foundation::PSTR, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32;
        }
        ::core::mem::transmute(ImmGetConversionListA(param0.into_param().abi(), param1.into_param().abi(), lpsrc.into_param().abi(), ::core::mem::transmute(lpdst), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(uflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetConversionListW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(param0: Param0, param1: Param1, lpsrc: Param2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetConversionListW(param0: super::super::TextServices::HKL, param1: super::super::super::Globalization::HIMC, lpsrc: super::super::super::Foundation::PWSTR, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32;
        }
        ::core::mem::transmute(ImmGetConversionListW(param0.into_param().abi(), param1.into_param().abi(), lpsrc.into_param().abi(), ::core::mem::transmute(lpdst), ::core::mem::transmute(dwbuflen), ::core::mem::transmute(uflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetConversionStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpfdwconversion: *mut u32, lpfdwsentence: *mut u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetConversionStatus(param0: super::super::super::Globalization::HIMC, lpfdwconversion: *mut u32, lpfdwsentence: *mut u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetConversionStatus(param0.into_param().abi(), ::core::mem::transmute(lpfdwconversion), ::core::mem::transmute(lpfdwsentence)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmGetDefaultIMEWnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0) -> super::super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetDefaultIMEWnd(param0: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
        }
        ::core::mem::transmute(ImmGetDefaultIMEWnd(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetDescriptionA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, lpszdescription: super::super::super::Foundation::PSTR, ubuflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetDescriptionA(param0: super::super::TextServices::HKL, lpszdescription: super::super::super::Foundation::PSTR, ubuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetDescriptionA(param0.into_param().abi(), ::core::mem::transmute(lpszdescription), ::core::mem::transmute(ubuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetDescriptionW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, lpszdescription: super::super::super::Foundation::PWSTR, ubuflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetDescriptionW(param0: super::super::TextServices::HKL, lpszdescription: super::super::super::Foundation::PWSTR, ubuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetDescriptionW(param0.into_param().abi(), ::core::mem::transmute(lpszdescription), ::core::mem::transmute(ubuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetGuideLineA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: super::super::super::Foundation::PSTR, dwbuflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetGuideLineA(param0: super::super::super::Globalization::HIMC, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: super::super::super::Foundation::PSTR, dwbuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetGuideLineA(param0.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(lpbuf), ::core::mem::transmute(dwbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetGuideLineW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: super::super::super::Foundation::PWSTR, dwbuflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetGuideLineW(param0: super::super::super::Globalization::HIMC, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: super::super::super::Foundation::PWSTR, dwbuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetGuideLineW(param0.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(lpbuf), ::core::mem::transmute(dwbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmGetHotKey(param0: u32, lpumodifiers: *mut u32, lpuvkey: *mut u32, phkl: *mut isize) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetHotKey(param0: u32, lpumodifiers: *mut u32, lpuvkey: *mut u32, phkl: *mut isize) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetHotKey(::core::mem::transmute(param0), ::core::mem::transmute(lpumodifiers), ::core::mem::transmute(lpuvkey), ::core::mem::transmute(phkl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetIMCCLockCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(param0: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetIMCCLockCount(param0: super::super::super::Globalization::HIMCC) -> u32;
        }
        ::core::mem::transmute(ImmGetIMCCLockCount(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetIMCCSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(param0: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetIMCCSize(param0: super::super::super::Globalization::HIMCC) -> u32;
        }
        ::core::mem::transmute(ImmGetIMCCSize(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmGetIMCLockCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetIMCLockCount(param0: super::super::super::Globalization::HIMC) -> u32;
        }
        ::core::mem::transmute(ImmGetIMCLockCount(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetIMEFileNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, lpszfilename: super::super::super::Foundation::PSTR, ubuflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetIMEFileNameA(param0: super::super::TextServices::HKL, lpszfilename: super::super::super::Foundation::PSTR, ubuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetIMEFileNameA(param0.into_param().abi(), ::core::mem::transmute(lpszfilename), ::core::mem::transmute(ubuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetIMEFileNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, lpszfilename: super::super::super::Foundation::PWSTR, ubuflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetIMEFileNameW(param0: super::super::TextServices::HKL, lpszfilename: super::super::super::Foundation::PWSTR, ubuflen: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetIMEFileNameW(param0.into_param().abi(), ::core::mem::transmute(lpszfilename), ::core::mem::transmute(ubuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmGetImeMenuItemsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: u32, param2: u32, lpimeparentmenu: *mut IMEMENUITEMINFOA, lpimemenu: *mut IMEMENUITEMINFOA, dwsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetImeMenuItemsA(param0: super::super::super::Globalization::HIMC, param1: u32, param2: u32, lpimeparentmenu: *mut IMEMENUITEMINFOA, lpimemenu: *mut IMEMENUITEMINFOA, dwsize: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetImeMenuItemsA(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(lpimeparentmenu), ::core::mem::transmute(lpimemenu), ::core::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmGetImeMenuItemsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: u32, param2: u32, lpimeparentmenu: *mut IMEMENUITEMINFOW, lpimemenu: *mut IMEMENUITEMINFOW, dwsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetImeMenuItemsW(param0: super::super::super::Globalization::HIMC, param1: u32, param2: u32, lpimeparentmenu: *mut IMEMENUITEMINFOW, lpimemenu: *mut IMEMENUITEMINFOW, dwsize: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetImeMenuItemsW(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(lpimeparentmenu), ::core::mem::transmute(lpimemenu), ::core::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetOpenStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetOpenStatus(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetOpenStatus(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, param1: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetProperty(param0: super::super::TextServices::HKL, param1: u32) -> u32;
        }
        ::core::mem::transmute(ImmGetProperty(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, nitem: u32, lpstylebuf: *mut STYLEBUFA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetRegisterWordStyleA(param0: super::super::TextServices::HKL, nitem: u32, lpstylebuf: *mut STYLEBUFA) -> u32;
        }
        ::core::mem::transmute(ImmGetRegisterWordStyleA(param0.into_param().abi(), ::core::mem::transmute(nitem), ::core::mem::transmute(lpstylebuf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_UI_TextServices'*"]
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0, nitem: u32, lpstylebuf: *mut STYLEBUFW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetRegisterWordStyleW(param0: super::super::TextServices::HKL, nitem: u32, lpstylebuf: *mut STYLEBUFW) -> u32;
        }
        ::core::mem::transmute(ImmGetRegisterWordStyleW(param0.into_param().abi(), ::core::mem::transmute(nitem), ::core::mem::transmute(lpstylebuf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmGetStatusWindowPos<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpptpos: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetStatusWindowPos(param0: super::super::super::Globalization::HIMC, lpptpos: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmGetStatusWindowPos(param0.into_param().abi(), ::core::mem::transmute(lpptpos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmGetVirtualKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmGetVirtualKey(param0: super::super::super::Foundation::HWND) -> u32;
        }
        ::core::mem::transmute(ImmGetVirtualKey(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmInstallIMEA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(lpszimefilename: Param0, lpszlayouttext: Param1) -> super::super::TextServices::HKL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmInstallIMEA(lpszimefilename: super::super::super::Foundation::PSTR, lpszlayouttext: super::super::super::Foundation::PSTR) -> super::super::TextServices::HKL;
        }
        ::core::mem::transmute(ImmInstallIMEA(lpszimefilename.into_param().abi(), lpszlayouttext.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmInstallIMEW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(lpszimefilename: Param0, lpszlayouttext: Param1) -> super::super::TextServices::HKL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmInstallIMEW(lpszimefilename: super::super::super::Foundation::PWSTR, lpszlayouttext: super::super::super::Foundation::PWSTR) -> super::super::TextServices::HKL;
        }
        ::core::mem::transmute(ImmInstallIMEW(lpszimefilename.into_param().abi(), lpszlayouttext.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmIsIME<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmIsIME(param0: super::super::TextServices::HKL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmIsIME(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmIsUIMessageA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(param0: Param0, param1: u32, param2: Param2, param3: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmIsUIMessageA(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmIsUIMessageA(param0.into_param().abi(), ::core::mem::transmute(param1), param2.into_param().abi(), param3.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmIsUIMessageW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(param0: Param0, param1: u32, param2: Param2, param3: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmIsUIMessageW(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmIsUIMessageW(param0.into_param().abi(), ::core::mem::transmute(param1), param2.into_param().abi(), param3.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmLockIMC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0) -> *mut INPUTCONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmLockIMC(param0: super::super::super::Globalization::HIMC) -> *mut INPUTCONTEXT;
        }
        ::core::mem::transmute(ImmLockIMC(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmLockIMCC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(param0: Param0) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmLockIMCC(param0: super::super::super::Globalization::HIMCC) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(ImmLockIMCC(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmNotifyIME<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, dwaction: NOTIFY_IME_ACTION, dwindex: NOTIFY_IME_INDEX, dwvalue: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmNotifyIME(param0: super::super::super::Globalization::HIMC, dwaction: NOTIFY_IME_ACTION, dwindex: NOTIFY_IME_INDEX, dwvalue: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmNotifyIME(param0.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(dwindex), ::core::mem::transmute(dwvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Globalization'*"]
#[cfg(feature = "Win32_Globalization")]
#[inline]
pub unsafe fn ImmReSizeIMCC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(param0: Param0, param1: u32) -> super::super::super::Globalization::HIMCC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmReSizeIMCC(param0: super::super::super::Globalization::HIMCC, param1: u32) -> super::super::super::Globalization::HIMCC;
        }
        ::core::mem::transmute(ImmReSizeIMCC(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmRegisterWordA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(param0: Param0, lpszreading: Param1, param2: u32, lpszregister: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmRegisterWordA(param0: super::super::TextServices::HKL, lpszreading: super::super::super::Foundation::PSTR, param2: u32, lpszregister: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmRegisterWordA(param0.into_param().abi(), lpszreading.into_param().abi(), ::core::mem::transmute(param2), lpszregister.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmRegisterWordW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(param0: Param0, lpszreading: Param1, param2: u32, lpszregister: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmRegisterWordW(param0: super::super::TextServices::HKL, lpszreading: super::super::super::Foundation::PWSTR, param2: u32, lpszregister: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmRegisterWordW(param0.into_param().abi(), lpszreading.into_param().abi(), ::core::mem::transmute(param2), lpszregister.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmReleaseContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: Param1) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmReleaseContext(param0: super::super::super::Foundation::HWND, param1: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmReleaseContext(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmRequestMessageA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(param0: Param0, param1: Param1, param2: Param2) -> super::super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmRequestMessageA(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::WPARAM, param2: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(ImmRequestMessageA(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmRequestMessageW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(param0: Param0, param1: Param1, param2: Param2) -> super::super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmRequestMessageW(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::WPARAM, param2: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(ImmRequestMessageW(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetCandidateWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpcandidate: *const CANDIDATEFORM) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetCandidateWindow(param0: super::super::super::Globalization::HIMC, lpcandidate: *const CANDIDATEFORM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetCandidateWindow(param0.into_param().abi(), ::core::mem::transmute(lpcandidate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmSetCompositionFontA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lplf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetCompositionFontA(param0: super::super::super::Globalization::HIMC, lplf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetCompositionFontA(param0.into_param().abi(), ::core::mem::transmute(lplf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImmSetCompositionFontW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lplf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetCompositionFontW(param0: super::super::super::Globalization::HIMC, lplf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetCompositionFontW(param0.into_param().abi(), ::core::mem::transmute(lplf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetCompositionStringA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: *const ::core::ffi::c_void, dwcomplen: u32, lpread: *const ::core::ffi::c_void, dwreadlen: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetCompositionStringA(param0: super::super::super::Globalization::HIMC, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: *const ::core::ffi::c_void, dwcomplen: u32, lpread: *const ::core::ffi::c_void, dwreadlen: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetCompositionStringA(param0.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(lpcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(lpread), ::core::mem::transmute(dwreadlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetCompositionStringW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: *const ::core::ffi::c_void, dwcomplen: u32, lpread: *const ::core::ffi::c_void, dwreadlen: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetCompositionStringW(param0: super::super::super::Globalization::HIMC, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: *const ::core::ffi::c_void, dwcomplen: u32, lpread: *const ::core::ffi::c_void, dwreadlen: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetCompositionStringW(param0.into_param().abi(), ::core::mem::transmute(dwindex), ::core::mem::transmute(lpcomp), ::core::mem::transmute(dwcomplen), ::core::mem::transmute(lpread), ::core::mem::transmute(dwreadlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetCompositionWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpcompform: *const COMPOSITIONFORM) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetCompositionWindow(param0: super::super::super::Globalization::HIMC, lpcompform: *const COMPOSITIONFORM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetCompositionWindow(param0.into_param().abi(), ::core::mem::transmute(lpcompform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetConversionStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, param1: u32, param2: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetConversionStatus(param0: super::super::super::Globalization::HIMC, param1: u32, param2: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetConversionStatus(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmSetHotKey<'a, Param3: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>>(param0: u32, param1: u32, param2: u32, param3: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetHotKey(param0: u32, param1: u32, param2: u32, param3: super::super::TextServices::HKL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetHotKey(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2), param3.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetOpenStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(param0: Param0, param1: Param1) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetOpenStatus(param0: super::super::super::Globalization::HIMC, param1: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetOpenStatus(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmSetStatusWindowPos<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0, lpptpos: *const super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSetStatusWindowPos(param0: super::super::super::Globalization::HIMC, lpptpos: *const super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSetStatusWindowPos(param0.into_param().abi(), ::core::mem::transmute(lpptpos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmShowSoftKeyboard<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0, param1: i32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmShowSoftKeyboard(param0: super::super::super::Foundation::HWND, param1: i32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmShowSoftKeyboard(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImmSimulateHotKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(param0: Param0, param1: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmSimulateHotKey(param0: super::super::super::Foundation::HWND, param1: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmSimulateHotKey(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmUnlockIMC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMC>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmUnlockIMC(param0: super::super::super::Globalization::HIMC) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmUnlockIMC(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_Globalization'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
#[inline]
pub unsafe fn ImmUnlockIMCC<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Globalization::HIMCC>>(param0: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmUnlockIMCC(param0: super::super::super::Globalization::HIMCC) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmUnlockIMCC(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmUnregisterWordA<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(param0: Param0, lpszreading: Param1, param2: u32, lpszunregister: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmUnregisterWordA(param0: super::super::TextServices::HKL, lpszreading: super::super::super::Foundation::PSTR, param2: u32, lpszunregister: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmUnregisterWordA(param0.into_param().abi(), lpszreading.into_param().abi(), ::core::mem::transmute(param2), lpszunregister.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation', 'Win32_UI_TextServices'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ImmUnregisterWordW<'a, Param0: ::windows::core::IntoParam<'a, super::super::TextServices::HKL>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(param0: Param0, lpszreading: Param1, param2: u32, lpszunregister: Param3) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImmUnregisterWordW(param0: super::super::TextServices::HKL, lpszreading: super::super::super::Foundation::PWSTR, param2: u32, lpszunregister: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImmUnregisterWordW(param0.into_param().abi(), lpszreading.into_param().abi(), ::core::mem::transmute(param2), lpszunregister.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_1DAN: u32 = 213u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_4DAN_HA: u32 = 212u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_5DAN_AWA: u32 = 200u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_5DAN_AWAUON: u32 = 209u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_5DAN_BA: u32 = 206u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_5DAN_GA: u32 = 202u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_5DAN_KA: u32 = 201u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_5DAN_KASOKUON: u32 = 210u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_5DAN_MA: u32 = 207u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_5DAN_NA: u32 = 205u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_5DAN_RA: u32 = 208u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_5DAN_RAHEN: u32 = 211u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_5DAN_SA: u32 = 203u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_5DAN_TA: u32 = 204u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_BUPPIN: u32 = 122u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_CHIMEI: u32 = 109u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_CHIMEI_EKI: u32 = 117u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_CHIMEI_GUN: u32 = 112u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_CHIMEI_KEN: u32 = 111u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_CHIMEI_KU: u32 = 113u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_CHIMEI_KUNI: u32 = 110u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_CHIMEI_MACHI: u32 = 115u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_CHIMEI_MURA: u32 = 116u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_CHIMEI_SHI: u32 = 114u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_CLOSEBRACE: u32 = 911u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_DAIMEISHI: u32 = 123u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_DAIMEISHI_NINSHOU: u32 = 124u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_DAIMEISHI_SHIJI: u32 = 125u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_DOKURITSUGO: u32 = 903u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_EIJI: u32 = 906u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_FUKUSHI: u32 = 500u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_FUKUSHI_DA: u32 = 504u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_FUKUSHI_NANO: u32 = 503u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_FUKUSHI_NI: u32 = 502u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_FUKUSHI_SAHEN: u32 = 501u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_FUKUSHI_TO: u32 = 505u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_FUKUSHI_TOSURU: u32 = 506u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_FUTEIGO: u32 = 904u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_HUKUSIMEISHI: u32 = 104u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_JINMEI: u32 = 106u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_JINMEI_MEI: u32 = 108u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_JINMEI_SEI: u32 = 107u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KANDOUSHI: u32 = 670u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KANJI: u32 = 909u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KANYOUKU: u32 = 902u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KAZU: u32 = 126u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KAZU_SURYOU: u32 = 127u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KAZU_SUSHI: u32 = 128u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KEIDOU: u32 = 400u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KEIDOU_GARU: u32 = 403u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KEIDOU_NO: u32 = 401u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KEIDOU_TARU: u32 = 402u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KEIYOU: u32 = 300u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KEIYOU_GARU: u32 = 301u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KEIYOU_GE: u32 = 302u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KEIYOU_ME: u32 = 303u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KEIYOU_U: u32 = 305u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KEIYOU_YUU: u32 = 304u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KENCHIKU: u32 = 121u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KIGOU: u32 = 905u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KURU_KI: u32 = 219u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KURU_KITA: u32 = 220u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KURU_KITARA: u32 = 221u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KURU_KITARI: u32 = 222u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KURU_KITAROU: u32 = 223u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KURU_KITE: u32 = 224u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KURU_KO: u32 = 226u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KURU_KOI: u32 = 227u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KURU_KOYOU: u32 = 228u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KURU_KUREBA: u32 = 225u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_KUTEN: u32 = 907u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_MEISA_KEIDOU: u32 = 105u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_MEISHI_FUTSU: u32 = 100u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_MEISHI_KEIYOUDOUSHI: u32 = 103u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_MEISHI_SAHEN: u32 = 101u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_MEISHI_ZAHEN: u32 = 102u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_OPENBRACE: u32 = 910u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_RENTAISHI: u32 = 600u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_RENTAISHI_SHIJI: u32 = 601u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_RENYOU_SETSUBI: u32 = 826u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI: u32 = 800u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_CHIMEI: u32 = 811u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_CHOU: u32 = 818u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_CHU: u32 = 804u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_DONO: u32 = 835u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_EKI: u32 = 821u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_FU: u32 = 805u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_FUKUSU: u32 = 836u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_GUN: u32 = 814u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_JIKAN: u32 = 829u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_JIKANPLUS: u32 = 830u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_JINMEI: u32 = 810u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_JOSUSHI: u32 = 827u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_JOSUSHIPLUS: u32 = 828u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_KA: u32 = 803u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_KATA: u32 = 808u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_KEN: u32 = 813u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_KENCHIKU: u32 = 825u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_KU: u32 = 815u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_KUN: u32 = 833u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_KUNI: u32 = 812u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_MACHI: u32 = 817u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_MEISHIRENDAKU: u32 = 809u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_MURA: u32 = 819u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_RA: u32 = 838u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_RYU: u32 = 806u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_SAMA: u32 = 834u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_SAN: u32 = 832u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_SEI: u32 = 802u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_SHAMEI: u32 = 823u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_SHI: u32 = 816u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_SON: u32 = 820u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_SONOTA: u32 = 822u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_SOSHIKI: u32 = 824u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_TACHI: u32 = 837u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_TEINEI: u32 = 831u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_TEKI: u32 = 801u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUBI_YOU: u32 = 807u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETSUZOKUSHI: u32 = 650u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU: u32 = 700u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_CHIMEI: u32 = 710u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_CHOUTAN: u32 = 707u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_DAISHOU: u32 = 705u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_FUKU: u32 = 703u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_JINMEI: u32 = 709u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_JOSUSHI: u32 = 712u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_KAKU: u32 = 701u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_KOUTEI: u32 = 706u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_MI: u32 = 704u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_SAI: u32 = 702u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_SHINKYU: u32 = 708u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_SONOTA: u32 = 711u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_TEINEI_GO: u32 = 714u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_TEINEI_O: u32 = 713u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SETTOU_TEINEI_ON: u32 = 715u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SHAMEI: u32 = 119u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SONOTA: u32 = 118u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SOSHIKI: u32 = 120u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SURU_SA: u32 = 229u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SURU_SE: u32 = 238u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SURU_SEYO: u32 = 239u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SURU_SI: u32 = 230u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SURU_SIATRI: u32 = 233u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SURU_SITA: u32 = 231u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SURU_SITARA: u32 = 232u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SURU_SITAROU: u32 = 234u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SURU_SITE: u32 = 235u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SURU_SIYOU: u32 = 236u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_SURU_SUREBA: u32 = 237u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_TANKANJI: u32 = 900u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_TANKANJI_KAO: u32 = 901u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_TANSHUKU: u32 = 913u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_TOKUSHU_KAHEN: u32 = 214u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_TOKUSHU_NAHEN: u32 = 218u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_TOKUSHU_SAHEN: u32 = 216u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_TOKUSHU_SAHENSURU: u32 = 215u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_TOKUSHU_ZAHEN: u32 = 217u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_TOUTEN: u32 = 908u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const JPOS_YOKUSEI: u32 = 912u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MAX_APPLETTITLE: u32 = 64u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MAX_FONTFACE: u32 = 32u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MODEBIASMODE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MODEBIASMODE_DIGIT: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MODEBIASMODE_FILENAME: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MODEBIASMODE_READING: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MODEBIAS_GETVALUE: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MODEBIAS_GETVERSION: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MODEBIAS_SETVALUE: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MOD_IGNORE_ALL_MODIFIER: u32 = 1024u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MOD_LEFT: u32 = 32768u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MOD_ON_KEYUP: u32 = 2048u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const MOD_RIGHT: u32 = 16384u32;
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MORRSLT {
    pub dwSize: u32,
    pub pwchOutput: super::super::super::Foundation::PWSTR,
    pub cchOutput: u16,
    pub Anonymous1: MORRSLT_0,
    pub Anonymous2: MORRSLT_1,
    pub pchInputPos: *mut u16,
    pub pchOutputIdxWDD: *mut u16,
    pub Anonymous3: MORRSLT_2,
    pub paMonoRubyPos: *mut u16,
    pub pWDD: *mut WDD,
    pub cWDD: i32,
    pub pPrivate: *mut ::core::ffi::c_void,
    pub BLKBuff: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MORRSLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MORRSLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MORRSLT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MORRSLT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MORRSLT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MORRSLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MORRSLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union MORRSLT_0 {
    pub pwchRead: super::super::super::Foundation::PWSTR,
    pub pwchComp: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MORRSLT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MORRSLT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MORRSLT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MORRSLT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MORRSLT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MORRSLT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MORRSLT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union MORRSLT_1 {
    pub cchRead: u16,
    pub cchComp: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MORRSLT_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MORRSLT_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MORRSLT_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MORRSLT_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MORRSLT_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MORRSLT_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MORRSLT_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union MORRSLT_2 {
    pub pchReadIdxWDD: *mut u16,
    pub pchCompIdxWDD: *mut u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MORRSLT_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MORRSLT_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MORRSLT_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MORRSLT_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MORRSLT_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MORRSLT_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MORRSLT_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const NI_CONTEXTUPDATED: u32 = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const NI_FINALIZECONVERSIONRESULT: u32 = 20u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type NOTIFY_IME_ACTION = u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const NI_CHANGECANDIDATELIST: NOTIFY_IME_ACTION = 19u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const NI_CLOSECANDIDATE: NOTIFY_IME_ACTION = 17u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const NI_COMPOSITIONSTR: NOTIFY_IME_ACTION = 21u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const NI_IMEMENUSELECTED: NOTIFY_IME_ACTION = 24u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const NI_OPENCANDIDATE: NOTIFY_IME_ACTION = 16u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const NI_SELECTCANDIDATESTR: NOTIFY_IME_ACTION = 18u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const NI_SETCANDIDATE_PAGESIZE: NOTIFY_IME_ACTION = 23u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const NI_SETCANDIDATE_PAGESTART: NOTIFY_IME_ACTION = 22u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type NOTIFY_IME_INDEX = u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CPS_CANCEL: NOTIFY_IME_INDEX = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CPS_COMPLETE: NOTIFY_IME_INDEX = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CPS_CONVERT: NOTIFY_IME_INDEX = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const CPS_REVERT: NOTIFY_IME_INDEX = 3u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNLOG = ::core::option::Option<unsafe extern "system" fn(param0: *mut IMEDP, param1: ::windows::core::HRESULT) -> super::super::super::Foundation::BOOL>;
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct POSTBL {
    pub nPos: u16,
    pub szName: *mut u8,
}
impl ::core::marker::Copy for POSTBL {}
impl ::core::clone::Clone for POSTBL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POSTBL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for POSTBL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POSTBL>()) == 0 }
    }
}
impl ::core::cmp::Eq for POSTBL {}
impl ::core::default::Default for POSTBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const POS_UNDEFINED: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct RECONVERTSTRING {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwStrLen: u32,
    pub dwStrOffset: u32,
    pub dwCompStrLen: u32,
    pub dwCompStrOffset: u32,
    pub dwTargetStrLen: u32,
    pub dwTargetStrOffset: u32,
}
impl ::core::marker::Copy for RECONVERTSTRING {}
impl ::core::clone::Clone for RECONVERTSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RECONVERTSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECONVERTSTRING").field("dwSize", &self.dwSize).field("dwVersion", &self.dwVersion).field("dwStrLen", &self.dwStrLen).field("dwStrOffset", &self.dwStrOffset).field("dwCompStrLen", &self.dwCompStrLen).field("dwCompStrOffset", &self.dwCompStrOffset).field("dwTargetStrLen", &self.dwTargetStrLen).field("dwTargetStrOffset", &self.dwTargetStrOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for RECONVERTSTRING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RECONVERTSTRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RECONVERTSTRING>()) == 0 }
    }
}
impl ::core::cmp::Eq for RECONVERTSTRING {}
impl ::core::default::Default for RECONVERTSTRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const RECONVOPT_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const RECONVOPT_USECANCELNOTIFY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REGISTERWORDA {
    pub lpReading: super::super::super::Foundation::PSTR,
    pub lpWord: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REGISTERWORDA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REGISTERWORDA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REGISTERWORDA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REGISTERWORDA").field("lpReading", &self.lpReading).field("lpWord", &self.lpWord).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REGISTERWORDA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REGISTERWORDA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REGISTERWORDA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REGISTERWORDA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REGISTERWORDA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type REGISTERWORDENUMPROCA = ::core::option::Option<unsafe extern "system" fn(lpszreading: super::super::super::Foundation::PSTR, param1: u32, lpszstring: super::super::super::Foundation::PSTR, param3: *mut ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type REGISTERWORDENUMPROCW = ::core::option::Option<unsafe extern "system" fn(lpszreading: super::super::super::Foundation::PWSTR, param1: u32, lpszstring: super::super::super::Foundation::PWSTR, param3: *mut ::core::ffi::c_void) -> i32>;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REGISTERWORDW {
    pub lpReading: super::super::super::Foundation::PWSTR,
    pub lpWord: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REGISTERWORDW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REGISTERWORDW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REGISTERWORDW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REGISTERWORDW").field("lpReading", &self.lpReading).field("lpWord", &self.lpWord).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REGISTERWORDW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REGISTERWORDW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REGISTERWORDW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REGISTERWORDW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REGISTERWORDW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SCS_CAP_COMPSTR: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SCS_CAP_MAKEREAD: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SCS_CAP_SETRECONVERTSTRING: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SELECT_CAP_CONVERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SELECT_CAP_SENTENCE: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type SET_COMPOSITION_STRING_TYPE = u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SCS_SETSTR: SET_COMPOSITION_STRING_TYPE = 9u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SCS_CHANGEATTR: SET_COMPOSITION_STRING_TYPE = 18u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SCS_CHANGECLAUSE: SET_COMPOSITION_STRING_TYPE = 36u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SCS_SETRECONVERTSTRING: SET_COMPOSITION_STRING_TYPE = 65536u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SCS_QUERYRECONVERTSTRING: SET_COMPOSITION_STRING_TYPE = 131072u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SHOWIMEPAD_CATEGORY: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SHOWIMEPAD_DEFAULT: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SHOWIMEPAD_GUID: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct SOFTKBDDATA {
    pub uCount: u32,
    pub wCode: [u16; 256],
}
impl ::core::marker::Copy for SOFTKBDDATA {}
impl ::core::clone::Clone for SOFTKBDDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOFTKBDDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOFTKBDDATA").field("uCount", &self.uCount).field("wCode", &self.wCode).finish()
    }
}
unsafe impl ::windows::core::Abi for SOFTKBDDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOFTKBDDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOFTKBDDATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for SOFTKBDDATA {}
impl ::core::default::Default for SOFTKBDDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SOFTKEYBOARD_TYPE_C1: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const SOFTKEYBOARD_TYPE_T1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STYLEBUFA {
    pub dwStyle: u32,
    pub szDescription: [super::super::super::Foundation::CHAR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STYLEBUFA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STYLEBUFA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STYLEBUFA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STYLEBUFA").field("dwStyle", &self.dwStyle).field("szDescription", &self.szDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STYLEBUFA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STYLEBUFA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STYLEBUFA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STYLEBUFA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STYLEBUFA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct STYLEBUFW {
    pub dwStyle: u32,
    pub szDescription: [u16; 32],
}
impl ::core::marker::Copy for STYLEBUFW {}
impl ::core::clone::Clone for STYLEBUFW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STYLEBUFW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STYLEBUFW").field("dwStyle", &self.dwStyle).field("szDescription", &self.szDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for STYLEBUFW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STYLEBUFW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STYLEBUFW>()) == 0 }
    }
}
impl ::core::cmp::Eq for STYLEBUFW {}
impl ::core::default::Default for STYLEBUFW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const STYLE_DESCRIPTION_SIZE: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRANSMSG {
    pub message: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRANSMSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRANSMSG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRANSMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSMSG").field("message", &self.message).field("wParam", &self.wParam).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRANSMSG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRANSMSG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRANSMSG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRANSMSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRANSMSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRANSMSGLIST {
    pub uMsgCount: u32,
    pub TransMsg: [TRANSMSG; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRANSMSGLIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRANSMSGLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRANSMSGLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSMSGLIST").field("uMsgCount", &self.uMsgCount).field("TransMsg", &self.TransMsg).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRANSMSGLIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRANSMSGLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRANSMSGLIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRANSMSGLIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRANSMSGLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const UI_CAP_2700: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const UI_CAP_ROT90: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const UI_CAP_ROTANY: u32 = 4u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const UI_CAP_SOFTKBD: u32 = 65536u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const VERSION_DOCUMENTFEED: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const VERSION_ID_CHINESE_SIMPLIFIED: u32 = 134217728u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const VERSION_ID_CHINESE_TRADITIONAL: u32 = 67108864u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const VERSION_ID_JAPANESE: u32 = 16777216u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const VERSION_ID_KOREAN: u32 = 33554432u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const VERSION_MODEBIAS: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const VERSION_MOUSE_OPERATION: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const VERSION_QUERYPOSITION: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub const VERSION_RECONVERSION: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct WDD {
    pub wDispPos: u16,
    pub Anonymous1: WDD_0,
    pub cchDisp: u16,
    pub Anonymous2: WDD_1,
    pub WDD_nReserve1: u32,
    pub nPos: u16,
    pub _bitfield: u16,
    pub pReserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WDD {}
impl ::core::clone::Clone for WDD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WDD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WDD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WDD>()) == 0 }
    }
}
impl ::core::cmp::Eq for WDD {}
impl ::core::default::Default for WDD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub union WDD_0 {
    pub wReadPos: u16,
    pub wCompPos: u16,
}
impl ::core::marker::Copy for WDD_0 {}
impl ::core::clone::Clone for WDD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WDD_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WDD_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WDD_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WDD_0 {}
impl ::core::default::Default for WDD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub union WDD_1 {
    pub cchRead: u16,
    pub cchComp: u16,
}
impl ::core::marker::Copy for WDD_1 {}
impl ::core::clone::Clone for WDD_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WDD_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WDD_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WDD_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for WDD_1 {}
impl ::core::default::Default for WDD_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type fpCreateIFECommonInstanceType = ::core::option::Option<unsafe extern "system" fn(ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type fpCreateIFEDictionaryInstanceType = ::core::option::Option<unsafe extern "system" fn(ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub type fpCreateIFELanguageInstanceType = ::core::option::Option<unsafe extern "system" fn(clsid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime'*"]
pub struct tabIMEFAREASTINFO {
    pub dwSize: u32,
    pub dwType: u32,
    pub dwData: [u32; 1],
}
impl ::core::marker::Copy for tabIMEFAREASTINFO {}
impl ::core::clone::Clone for tabIMEFAREASTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tabIMEFAREASTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tabIMEFAREASTINFO").field("dwSize", &self.dwSize).field("dwType", &self.dwType).field("dwData", &self.dwData).finish()
    }
}
unsafe impl ::windows::core::Abi for tabIMEFAREASTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tabIMEFAREASTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tabIMEFAREASTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for tabIMEFAREASTINFO {}
impl ::core::default::Default for tabIMEFAREASTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Input_Ime', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct tabIMESTRINGINFO {
    pub dwFarEastId: u32,
    pub lpwstr: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for tabIMESTRINGINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for tabIMESTRINGINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for tabIMESTRINGINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tabIMESTRINGINFO").field("dwFarEastId", &self.dwFarEastId).field("lpwstr", &self.lpwstr).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for tabIMESTRINGINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for tabIMESTRINGINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tabIMESTRINGINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for tabIMESTRINGINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for tabIMESTRINGINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}

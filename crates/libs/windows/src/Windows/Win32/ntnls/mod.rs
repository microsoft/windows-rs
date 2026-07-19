#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CPTABLEINFO {
    pub CodePage: u16,
    pub MaximumCharacterSize: u16,
    pub DefaultChar: u16,
    pub UniDefaultChar: u16,
    pub TransDefaultChar: u16,
    pub TransUniDefaultChar: u16,
    pub DBCSCodePage: u16,
    pub LeadByte: [u8; 12],
    pub MultiByteTable: super::PUSHORT,
    pub WideCharTable: *mut core::ffi::c_void,
    pub DBCSRanges: super::PUSHORT,
    pub DBCSOffsets: super::PUSHORT,
}
#[cfg(feature = "minwindef")]
impl Default for CPTABLEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAXIMUM_LEADBYTES: u32 = 12;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLSTABLEINFO {
    pub OemTableInfo: CPTABLEINFO,
    pub AnsiTableInfo: CPTABLEINFO,
    pub UpperCaseTable: super::PUSHORT,
    pub LowerCaseTable: super::PUSHORT,
}
#[cfg(feature = "minwindef")]
pub type PCPTABLEINFO = *mut CPTABLEINFO;
#[cfg(feature = "minwindef")]
pub type PNLSTABLEINFO = *mut NLSTABLEINFO;
#[cfg(feature = "minwindef")]
pub type PRTL_NLS_STATE = *mut RTL_NLS_STATE;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTL_NLS_STATE {
    pub DefaultAcpTableInfo: CPTABLEINFO,
    pub DefaultOemTableInfo: CPTABLEINFO,
    pub ActiveCodePageData: super::PUSHORT,
    pub OemCodePageData: super::PUSHORT,
    pub LeadByteInfo: super::PUSHORT,
    pub OemLeadByteInfo: super::PUSHORT,
    pub CaseMappingData: super::PUSHORT,
    pub UnicodeUpcaseTable844: super::PUSHORT,
    pub UnicodeLowercaseTable844: super::PUSHORT,
}

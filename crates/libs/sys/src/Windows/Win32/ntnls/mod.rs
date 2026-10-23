#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CPTABLEINFO {
    pub CodePage: super::USHORT,
    pub MaximumCharacterSize: super::USHORT,
    pub DefaultChar: super::USHORT,
    pub UniDefaultChar: super::USHORT,
    pub TransDefaultChar: super::USHORT,
    pub TransUniDefaultChar: super::USHORT,
    pub DBCSCodePage: super::USHORT,
    pub LeadByte: [super::UCHAR; 12],
    pub MultiByteTable: super::PUSHORT,
    pub WideCharTable: super::PVOID,
    pub DBCSRanges: super::PUSHORT,
    pub DBCSOffsets: super::PUSHORT,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for CPTABLEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAXIMUM_LEADBYTES: i32 = 12;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct NLSTABLEINFO {
    pub OemTableInfo: CPTABLEINFO,
    pub AnsiTableInfo: CPTABLEINFO,
    pub UpperCaseTable: super::PUSHORT,
    pub LowerCaseTable: super::PUSHORT,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PCPTABLEINFO = *mut CPTABLEINFO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PNLSTABLEINFO = *mut NLSTABLEINFO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PRTL_NLS_STATE = *mut RTL_NLS_STATE;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
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

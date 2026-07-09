#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CPTABLEINFO {
    pub CodePage: u16,
    pub MaximumCharacterSize: u16,
    pub DefaultChar: u16,
    pub UniDefaultChar: u16,
    pub TransDefaultChar: u16,
    pub TransUniDefaultChar: u16,
    pub DBCSCodePage: u16,
    pub LeadByte: [u8; 12],
    pub MultiByteTable: super::super::Win32::minwindef::PUSHORT,
    pub WideCharTable: *mut core::ffi::c_void,
    pub DBCSRanges: super::super::Win32::minwindef::PUSHORT,
    pub DBCSOffsets: super::super::Win32::minwindef::PUSHORT,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for CPTABLEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAXIMUM_LEADBYTES: u32 = 12;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NLSTABLEINFO {
    pub OemTableInfo: CPTABLEINFO,
    pub AnsiTableInfo: CPTABLEINFO,
    pub UpperCaseTable: super::super::Win32::minwindef::PUSHORT,
    pub LowerCaseTable: super::super::Win32::minwindef::PUSHORT,
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCPTABLEINFO(pub *mut CPTABLEINFO);
#[cfg(feature = "Win32_minwindef")]
impl PCPTABLEINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PCPTABLEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNLSTABLEINFO(pub *mut NLSTABLEINFO);
#[cfg(feature = "Win32_minwindef")]
impl PNLSTABLEINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PNLSTABLEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_NLS_STATE(pub *mut RTL_NLS_STATE);
#[cfg(feature = "Win32_minwindef")]
impl PRTL_NLS_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PRTL_NLS_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RTL_NLS_STATE {
    pub DefaultAcpTableInfo: CPTABLEINFO,
    pub DefaultOemTableInfo: CPTABLEINFO,
    pub ActiveCodePageData: super::super::Win32::minwindef::PUSHORT,
    pub OemCodePageData: super::super::Win32::minwindef::PUSHORT,
    pub LeadByteInfo: super::super::Win32::minwindef::PUSHORT,
    pub OemLeadByteInfo: super::super::Win32::minwindef::PUSHORT,
    pub CaseMappingData: super::super::Win32::minwindef::PUSHORT,
    pub UnicodeUpcaseTable844: super::super::Win32::minwindef::PUSHORT,
    pub UnicodeLowercaseTable844: super::super::Win32::minwindef::PUSHORT,
}

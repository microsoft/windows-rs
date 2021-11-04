#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct BSTRBLOB {
    pub cbSize: u32,
    pub pData: *mut u8,
}
impl BSTRBLOB {}
impl ::std::default::Default for BSTRBLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BSTRBLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BSTRBLOB").field("cbSize", &self.cbSize).field("pData", &self.pData).finish()
    }
}
impl ::std::cmp::PartialEq for BSTRBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pData == other.pData
    }
}
impl ::std::cmp::Eq for BSTRBLOB {}
unsafe impl ::windows::runtime::Abi for BSTRBLOB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl CABOOL {}
impl ::std::default::Default for CABOOL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CABOOL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CABOOL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CABOOL {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CABOOL {}
unsafe impl ::windows::runtime::Abi for CABOOL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CABSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CABSTR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CABSTR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CABSTR").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CABSTR {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CABSTR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CABSTR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut BSTRBLOB,
}
impl CABSTRBLOB {}
impl ::std::default::Default for CABSTRBLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CABSTRBLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CABSTRBLOB").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CABSTRBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CABSTRBLOB {}
unsafe impl ::windows::runtime::Abi for CABSTRBLOB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
pub struct CAC {
    pub cElems: u32,
    pub pElems: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CAC {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CAC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CAC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAC").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CAC {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CAC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CAC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut CLIPDATA,
}
impl CACLIPDATA {}
impl ::std::default::Default for CACLIPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CACLIPDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CACLIPDATA").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CACLIPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CACLIPDATA {}
unsafe impl ::windows::runtime::Abi for CACLIPDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut ::windows::runtime::GUID,
}
impl CACLSID {}
impl ::std::default::Default for CACLSID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CACLSID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CACLSID").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CACLSID {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CACLSID {}
unsafe impl ::windows::runtime::Abi for CACLSID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut super::CY,
}
impl CACY {}
impl ::std::default::Default for CACY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CACY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CACY").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CACY {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CACY {}
unsafe impl ::windows::runtime::Abi for CACY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl CADATE {}
impl ::std::default::Default for CADATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CADATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CADATE").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CADATE {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CADATE {}
unsafe impl ::windows::runtime::Abi for CADATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl CADBL {}
impl ::std::default::Default for CADBL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CADBL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CADBL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CADBL {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CADBL {}
unsafe impl ::windows::runtime::Abi for CADBL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl CAFILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CAFILETIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CAFILETIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAFILETIME").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CAFILETIME {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CAFILETIME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CAFILETIME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
impl CAFLT {}
impl ::std::default::Default for CAFLT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CAFLT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAFLT").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CAFLT {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CAFLT {}
unsafe impl ::windows::runtime::Abi for CAFLT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
impl CAH {}
impl ::std::default::Default for CAH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CAH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAH").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CAH {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CAH {}
unsafe impl ::windows::runtime::Abi for CAH {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl CAI {}
impl ::std::default::Default for CAI {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CAI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAI").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CAI {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CAI {}
unsafe impl ::windows::runtime::Abi for CAI {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl CAL {}
impl ::std::default::Default for CAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CAL {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CAL {}
unsafe impl ::windows::runtime::Abi for CAL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CALPSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CALPSTR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CALPSTR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CALPSTR").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CALPSTR {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CALPSTR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CALPSTR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CALPWSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CALPWSTR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CALPWSTR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CALPWSTR").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CALPWSTR {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CALPWSTR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CALPWSTR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl CAPROPVARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for CAPROPVARIANT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::fmt::Debug for CAPROPVARIANT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAPROPVARIANT").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for CAPROPVARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for CAPROPVARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for CAPROPVARIANT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl CASCODE {}
impl ::std::default::Default for CASCODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CASCODE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CASCODE").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CASCODE {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CASCODE {}
unsafe impl ::windows::runtime::Abi for CASCODE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
impl CAUB {}
impl ::std::default::Default for CAUB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CAUB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAUB").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CAUB {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CAUB {}
unsafe impl ::windows::runtime::Abi for CAUB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
impl CAUH {}
impl ::std::default::Default for CAUH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CAUH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAUH").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CAUH {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CAUH {}
unsafe impl ::windows::runtime::Abi for CAUH {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
impl CAUI {}
impl ::std::default::Default for CAUI {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CAUI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAUI").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CAUI {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CAUI {}
unsafe impl ::windows::runtime::Abi for CAUI {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl CAUL {}
impl ::std::default::Default for CAUL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CAUL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAUL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
impl ::std::cmp::PartialEq for CAUL {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::std::cmp::Eq for CAUL {}
unsafe impl ::windows::runtime::Abi for CAUL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const CCH_MAX_PROPSTG_NAME: u32 = 31u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct CLIPDATA {
    pub cbSize: u32,
    pub ulClipFmt: i32,
    pub pClipData: *mut u8,
}
impl CLIPDATA {}
impl ::std::default::Default for CLIPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLIPDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLIPDATA").field("cbSize", &self.cbSize).field("ulClipFmt", &self.ulClipFmt).field("pClipData", &self.pClipData).finish()
    }
}
impl ::std::cmp::PartialEq for CLIPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulClipFmt == other.ulClipFmt && self.pClipData == other.pClipData
    }
}
impl ::std::cmp::Eq for CLIPDATA {}
unsafe impl ::windows::runtime::Abi for CLIPDATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const CWCSTORAGENAME: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoGetInstanceFromFile<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::runtime::GUID, punkouter: Param2, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: Param5, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetInstanceFromFile(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: super::super::super::Foundation::PWSTR, dwcount: u32, presults: *mut ::std::mem::ManuallyDrop<super::MULTI_QI>) -> ::windows::runtime::HRESULT;
        }
        CoGetInstanceFromFile(::std::mem::transmute(pserverinfo), ::std::mem::transmute(pclsid), punkouter.into_param().abi(), ::std::mem::transmute(dwclsctx), ::std::mem::transmute(grfmode), pwszname.into_param().abi(), ::std::mem::transmute(dwcount), ::std::mem::transmute(presults)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoGetInstanceFromIStorage<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param4: ::windows::runtime::IntoParam<'a, IStorage>>(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::runtime::GUID, punkouter: Param2, dwclsctx: super::CLSCTX, pstg: Param4, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetInstanceFromIStorage(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclsctx: super::CLSCTX, pstg: ::windows::runtime::RawPtr, dwcount: u32, presults: *mut ::std::mem::ManuallyDrop<super::MULTI_QI>) -> ::windows::runtime::HRESULT;
        }
        CoGetInstanceFromIStorage(::std::mem::transmute(pserverinfo), ::std::mem::transmute(pclsid), punkouter.into_param().abi(), ::std::mem::transmute(dwclsctx), pstg.into_param().abi(), ::std::mem::transmute(dwcount), ::std::mem::transmute(presults)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn CoGetInterfaceAndReleaseStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>, T: ::windows::runtime::Interface>(pstm: Param0) -> ::windows::runtime::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetInterfaceAndReleaseStream(pstm: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        let mut result__ = ::std::option::Option::None;
        CoGetInterfaceAndReleaseStream(pstm.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateILockBytesOnHGlobal<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(hglobal: isize, fdeleteonrelease: Param1) -> ::windows::runtime::Result<ILockBytes> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateILockBytesOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, pplkbyt: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <ILockBytes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateILockBytesOnHGlobal(::std::mem::transmute(hglobal), fdeleteonrelease.into_param().abi(), &mut result__).from_abi::<ILockBytes>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStreamOnHGlobal<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(hglobal: isize, fdeleteonrelease: Param1) -> ::windows::runtime::Result<super::IStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStreamOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateStreamOnHGlobal(::std::mem::transmute(hglobal), fdeleteonrelease.into_param().abi(), &mut result__).from_abi::<super::IStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FmtIdToPropStgName(pfmtid: *const ::windows::runtime::GUID, oszname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FmtIdToPropStgName(pfmtid: *const ::windows::runtime::GUID, oszname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        FmtIdToPropStgName(::std::mem::transmute(pfmtid), ::std::mem::transmute(oszname)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn FreePropVariantArray(cvariants: u32, rgvars: *mut PROPVARIANT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreePropVariantArray(cvariants: u32, rgvars: *mut ::std::mem::ManuallyDrop<PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        FreePropVariantArray(::std::mem::transmute(cvariants), ::std::mem::transmute(rgvars)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn GetConvertStg<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(pstg: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConvertStg(pstg: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        GetConvertStg(pstg.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn GetHGlobalFromILockBytes<'a, Param0: ::windows::runtime::IntoParam<'a, ILockBytes>>(plkbyt: Param0) -> ::windows::runtime::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetHGlobalFromILockBytes(plkbyt: ::windows::runtime::RawPtr, phglobal: *mut isize) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetHGlobalFromILockBytes(plkbyt.into_param().abi(), &mut result__).from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn GetHGlobalFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(pstm: Param0) -> ::windows::runtime::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetHGlobalFromStream(pstm: ::windows::runtime::RawPtr, phglobal: *mut isize) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetHGlobalFromStream(pstm.into_param().abi(), &mut result__).from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectWriterLock(pub ::windows::runtime::IUnknown);
impl IDirectWriterLock {
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn WaitForWriteAccess(&self, dwtimeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwtimeout)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn ReleaseWriteAccess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn HaveWriteAccess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectWriterLock {
    type Vtable = IDirectWriterLock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(242044306, 26424, 4559, [150, 8, 0, 170, 0, 104, 13, 180]);
}
impl ::std::convert::From<IDirectWriterLock> for ::windows::runtime::IUnknown {
    fn from(value: IDirectWriterLock) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectWriterLock> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectWriterLock) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectWriterLock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectWriterLock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectWriterLock_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwtimeout: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumSTATPROPSETSTG(pub ::windows::runtime::IUnknown);
impl IEnumSTATPROPSETSTG {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSTATPROPSETSTG> {
        let mut result__: <IEnumSTATPROPSETSTG as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATPROPSETSTG>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSTATPROPSETSTG {
    type Vtable = IEnumSTATPROPSETSTG_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(315, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumSTATPROPSETSTG> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSTATPROPSETSTG) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumSTATPROPSETSTG> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSTATPROPSETSTG) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSTATPROPSETSTG {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumSTATPROPSETSTG {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATPROPSETSTG_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumSTATPROPSTG(pub ::windows::runtime::IUnknown);
impl IEnumSTATPROPSTG {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSTATPROPSTG> {
        let mut result__: <IEnumSTATPROPSTG as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATPROPSTG>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSTATPROPSTG {
    type Vtable = IEnumSTATPROPSTG_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(313, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumSTATPROPSTG> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSTATPROPSTG) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumSTATPROPSTG> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSTATPROPSTG) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSTATPROPSTG {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumSTATPROPSTG {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATPROPSTG_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumSTATSTG(pub ::windows::runtime::IUnknown);
impl IEnumSTATSTG {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSTATSTG> {
        let mut result__: <IEnumSTATSTG as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATSTG>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSTATSTG {
    type Vtable = IEnumSTATSTG_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(13, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumSTATSTG> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSTATSTG) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumSTATSTG> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSTATSTG) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSTATSTG {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumSTATSTG {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATSTG_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFillLockBytes(pub ::windows::runtime::IUnknown);
impl IFillLockBytes {
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn FillAppend(&self, pv: *const ::std::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv), ::std::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn FillAt(&self, uloffset: u64, pv: *const ::std::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(uloffset), ::std::mem::transmute(pv), ::std::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetFillSize(&self, ulsize: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn Terminate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, bcanceled: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), bcanceled.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFillLockBytes {
    type Vtable = IFillLockBytes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2580213776, 16734, 4559, [136, 20, 0, 170, 0, 181, 105, 245]);
}
impl ::std::convert::From<IFillLockBytes> for ::windows::runtime::IUnknown {
    fn from(value: IFillLockBytes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFillLockBytes> for ::windows::runtime::IUnknown {
    fn from(value: &IFillLockBytes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFillLockBytes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFillLockBytes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFillLockBytes_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::std::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uloffset: u64, pv: *const ::std::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulsize: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bcanceled: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ILayoutStorage(pub ::windows::runtime::IUnknown);
impl ILayoutStorage {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn LayoutScript(&self, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstoragelayout), ::std::mem::transmute(nentries), ::std::mem::transmute(glfinterleavedflag)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn BeginMonitor(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn EndMonitor(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn ReLayoutDocfile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsnewdfname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pwcsnewdfname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn ReLayoutDocfileOnILockBytes<'a, Param0: ::windows::runtime::IntoParam<'a, ILockBytes>>(&self, pilockbytes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pilockbytes.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILayoutStorage {
    type Vtable = ILayoutStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(242044304, 26424, 4559, [150, 8, 0, 170, 0, 104, 13, 180]);
}
impl ::std::convert::From<ILayoutStorage> for ::windows::runtime::IUnknown {
    fn from(value: ILayoutStorage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ILayoutStorage> for ::windows::runtime::IUnknown {
    fn from(value: &ILayoutStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILayoutStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILayoutStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsnewdfname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pilockbytes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ILockBytes(pub ::windows::runtime::IUnknown);
impl ILockBytes {
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn ReadAt(&self, uloffset: u64, pv: *mut ::std::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(uloffset), ::std::mem::transmute(pv), ::std::mem::transmute(cb), ::std::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn WriteAt(&self, uloffset: u64, pv: *const ::std::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(uloffset), ::std::mem::transmute(pv), ::std::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetSize(&self, cb: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(cb)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(liboffset), ::std::mem::transmute(cb), ::std::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(liboffset), ::std::mem::transmute(cb), ::std::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstatstg), ::std::mem::transmute(grfstatflag)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILockBytes {
    type Vtable = ILockBytes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(10, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ILockBytes> for ::windows::runtime::IUnknown {
    fn from(value: ILockBytes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ILockBytes> for ::windows::runtime::IUnknown {
    fn from(value: &ILockBytes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILockBytes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILockBytes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockBytes_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uloffset: u64, pv: *mut ::std::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uloffset: u64, pv: *const ::std::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cb: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPersistStorage(pub ::windows::runtime::IUnknown);
impl IPersistStorage {
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn InitNew<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(&self, pstg: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pstg.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(&self, pstg: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pstg.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, pstgsave: Param0, fsameasload: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pstgsave.into_param().abi(), fsameasload.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SaveCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(&self, pstgnew: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pstgnew.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn HandsOffStorage(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPersistStorage {
    type Vtable = IPersistStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(266, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IPersistStorage> for ::windows::runtime::IUnknown {
    fn from(value: IPersistStorage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPersistStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IPersistStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersistStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPersistStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IPersistStorage> for super::IPersist {
    fn from(value: IPersistStorage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPersistStorage> for super::IPersist {
    fn from(value: &IPersistStorage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IPersist> for IPersistStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IPersist> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IPersist> for &IPersistStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IPersist> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclassid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstgsave: ::windows::runtime::RawPtr, fsameasload: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstgnew: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertySetStorage(pub ::windows::runtime::IUnknown);
impl IPropertySetStorage {
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Create(&self, rfmtid: *const ::windows::runtime::GUID, pclsid: *const ::windows::runtime::GUID, grfflags: u32, grfmode: u32) -> ::windows::runtime::Result<IPropertyStorage> {
        let mut result__: <IPropertyStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(rfmtid), ::std::mem::transmute(pclsid), ::std::mem::transmute(grfflags), ::std::mem::transmute(grfmode), &mut result__).from_abi::<IPropertyStorage>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Open(&self, rfmtid: *const ::windows::runtime::GUID, grfmode: u32) -> ::windows::runtime::Result<IPropertyStorage> {
        let mut result__: <IPropertyStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(rfmtid), ::std::mem::transmute(grfmode), &mut result__).from_abi::<IPropertyStorage>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Delete(&self, rfmtid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(rfmtid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Enum(&self) -> ::windows::runtime::Result<IEnumSTATPROPSETSTG> {
        let mut result__: <IEnumSTATPROPSETSTG as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATPROPSETSTG>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertySetStorage {
    type Vtable = IPropertySetStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(314, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IPropertySetStorage> for ::windows::runtime::IUnknown {
    fn from(value: IPropertySetStorage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPropertySetStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertySetStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertySetStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPropertySetStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rfmtid: *const ::windows::runtime::GUID, pclsid: *const ::windows::runtime::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rfmtid: *const ::windows::runtime::GUID, grfmode: u32, ppprstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rfmtid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPropertyStorage(pub ::windows::runtime::IUnknown);
impl IPropertyStorage {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn ReadMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cpspec), ::std::mem::transmute(rgpspec), ::std::mem::transmute(rgpropvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn WriteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cpspec), ::std::mem::transmute(rgpspec), ::std::mem::transmute(rgpropvar), ::std::mem::transmute(propidnamefirst)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn DeleteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cpspec), ::std::mem::transmute(rgpspec)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(cpropid), ::std::mem::transmute(rgpropid), ::std::mem::transmute(rglpwstrname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(cpropid), ::std::mem::transmute(rgpropid), ::std::mem::transmute(rglpwstrname)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn DeletePropertyNames(&self, cpropid: u32, rgpropid: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cpropid), ::std::mem::transmute(rgpropid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Enum(&self) -> ::windows::runtime::Result<IEnumSTATPROPSTG> {
        let mut result__: <IEnumSTATPROPSTG as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATPROPSTG>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn SetTimes(&self, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pctime), ::std::mem::transmute(patime), ::std::mem::transmute(pmtime)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetClass(&self, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(clsid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn Stat(&self) -> ::windows::runtime::Result<STATPROPSETSTG> {
        let mut result__: <STATPROPSETSTG as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<STATPROPSETSTG>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyStorage {
    type Vtable = IPropertyStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(312, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IPropertyStorage> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyStorage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPropertyStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPropertyStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut ::std::mem::ManuallyDrop<PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const ::std::mem::ManuallyDrop<PROPVARIANT>, propidnamefirst: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpropid: u32, rgpropid: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfcommitflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatpsstg: *mut STATPROPSETSTG) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRootStorage(pub ::windows::runtime::IUnknown);
impl IRootStorage {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn SwitchToFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pszfile: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszfile.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRootStorage {
    type Vtable = IRootStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(18, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IRootStorage> for ::windows::runtime::IUnknown {
    fn from(value: IRootStorage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRootStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IRootStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRootStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRootStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRootStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfile: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStorage(pub ::windows::runtime::IUnknown);
impl IStorage {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn CreateStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsname: Param0, grfmode: u32, reserved1: u32, reserved2: u32) -> ::windows::runtime::Result<super::IStream> {
        let mut result__: <super::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwcsname.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(reserved1), ::std::mem::transmute(reserved2), &mut result__).from_abi::<super::IStream>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn OpenStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsname: Param0, reserved1: *mut ::std::ffi::c_void, grfmode: u32, reserved2: u32, ppstm: *mut ::std::option::Option<super::IStream>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pwcsname.into_param().abi(), ::std::mem::transmute(reserved1), ::std::mem::transmute(grfmode), ::std::mem::transmute(reserved2), ::std::mem::transmute(ppstm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn CreateStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsname: Param0, grfmode: u32, reserved1: u32, reserved2: u32) -> ::windows::runtime::Result<IStorage> {
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwcsname.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(reserved1), ::std::mem::transmute(reserved2), &mut result__).from_abi::<IStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn OpenStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IStorage>>(&self, pwcsname: Param0, pstgpriority: Param1, grfmode: u32, snbexclude: *const *const u16, reserved: u32) -> ::windows::runtime::Result<IStorage> {
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pwcsname.into_param().abi(), pstgpriority.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(snbexclude), ::std::mem::transmute(reserved), &mut result__).from_abi::<IStorage>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn CopyTo<'a, Param3: ::windows::runtime::IntoParam<'a, IStorage>>(&self, ciidexclude: u32, rgiidexclude: *const ::windows::runtime::GUID, snbexclude: *const *const u16, pstgdest: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(ciidexclude), ::std::mem::transmute(rgiidexclude), ::std::mem::transmute(snbexclude), pstgdest.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn MoveElementTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IStorage>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsname: Param0, pstgdest: Param1, pwcsnewname: Param2, grfflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pwcsname.into_param().abi(), pstgdest.into_param().abi(), pwcsnewname.into_param().abi(), ::std::mem::transmute(grfflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn EnumElements(&self, reserved1: u32, reserved2: *mut ::std::ffi::c_void, reserved3: u32, ppenum: *mut ::std::option::Option<IEnumSTATSTG>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(reserved1), ::std::mem::transmute(reserved2), ::std::mem::transmute(reserved3), ::std::mem::transmute(ppenum)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn DestroyElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pwcsname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn RenameElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsoldname: Param0, pwcsnewname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pwcsoldname.into_param().abi(), pwcsnewname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn SetElementTimes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsname: Param0, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pwcsname.into_param().abi(), ::std::mem::transmute(pctime), ::std::mem::transmute(patime), ::std::mem::transmute(pmtime)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetClass(&self, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(clsid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfstatebits), ::std::mem::transmute(grfmask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstatstg), ::std::mem::transmute(grfstatflag)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStorage {
    type Vtable = IStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(11, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IStorage> for ::windows::runtime::IUnknown {
    fn from(value: IStorage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved1: u32, reserved2: u32, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsname: super::super::super::Foundation::PWSTR, reserved1: *mut ::std::ffi::c_void, grfmode: u32, reserved2: u32, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved1: u32, reserved2: u32, ppstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsname: super::super::super::Foundation::PWSTR, pstgpriority: ::windows::runtime::RawPtr, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ciidexclude: u32, rgiidexclude: *const ::windows::runtime::GUID, snbexclude: *const *const u16, pstgdest: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsname: super::super::super::Foundation::PWSTR, pstgdest: ::windows::runtime::RawPtr, pwcsnewname: super::super::super::Foundation::PWSTR, grfflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfcommitflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved1: u32, reserved2: *mut ::std::ffi::c_void, reserved3: u32, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsoldname: super::super::super::Foundation::PWSTR, pwcsnewname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsname: super::super::super::Foundation::PWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfstatebits: u32, grfmask: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LOCKTYPE(pub i32);
pub const LOCK_WRITE: LOCKTYPE = LOCKTYPE(1i32);
pub const LOCK_EXCLUSIVE: LOCKTYPE = LOCKTYPE(2i32);
pub const LOCK_ONLYONCE: LOCKTYPE = LOCKTYPE(4i32);
impl ::std::convert::From<i32> for LOCKTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LOCKTYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct OLESTREAM {
    pub lpstbl: *mut OLESTREAMVTBL,
}
impl OLESTREAM {}
impl ::std::default::Default for OLESTREAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OLESTREAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLESTREAM").field("lpstbl", &self.lpstbl).finish()
    }
}
impl ::std::cmp::PartialEq for OLESTREAM {
    fn eq(&self, other: &Self) -> bool {
        self.lpstbl == other.lpstbl
    }
}
impl ::std::cmp::Eq for OLESTREAM {}
unsafe impl ::windows::runtime::Abi for OLESTREAM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct OLESTREAMVTBL {
    pub Get: isize,
    pub Put: isize,
}
impl OLESTREAMVTBL {}
impl ::std::default::Default for OLESTREAMVTBL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OLESTREAMVTBL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OLESTREAMVTBL").field("Get", &self.Get).field("Put", &self.Put).finish()
    }
}
impl ::std::cmp::PartialEq for OLESTREAMVTBL {
    fn eq(&self, other: &Self) -> bool {
        self.Get == other.Get && self.Put == other.Put
    }
}
impl ::std::cmp::Eq for OLESTREAMVTBL {}
unsafe impl ::windows::runtime::Abi for OLESTREAMVTBL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAM<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(pstg: Param0, lpolestream: *mut OLESTREAM) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleConvertIStorageToOLESTREAM(pstg: ::windows::runtime::RawPtr, lpolestream: *mut OLESTREAM) -> ::windows::runtime::HRESULT;
        }
        OleConvertIStorageToOLESTREAM(pstg.into_param().abi(), ::std::mem::transmute(lpolestream)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAMEx<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(pstg: Param0, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *mut super::STGMEDIUM, polestm: *mut OLESTREAM) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleConvertIStorageToOLESTREAMEx(pstg: ::windows::runtime::RawPtr, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *mut ::std::mem::ManuallyDrop<super::STGMEDIUM>, polestm: *mut OLESTREAM) -> ::windows::runtime::HRESULT;
        }
        OleConvertIStorageToOLESTREAMEx(pstg.into_param().abi(), ::std::mem::transmute(cfformat), ::std::mem::transmute(lwidth), ::std::mem::transmute(lheight), ::std::mem::transmute(dwsize), ::std::mem::transmute(pmedium), ::std::mem::transmute(polestm)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage<'a, Param1: ::windows::runtime::IntoParam<'a, IStorage>>(lpolestream: *mut OLESTREAM, pstg: Param1, ptd: *const super::DVTARGETDEVICE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleConvertOLESTREAMToIStorage(lpolestream: *mut OLESTREAM, pstg: ::windows::runtime::RawPtr, ptd: *const super::DVTARGETDEVICE) -> ::windows::runtime::HRESULT;
        }
        OleConvertOLESTREAMToIStorage(::std::mem::transmute(lpolestream), pstg.into_param().abi(), ::std::mem::transmute(ptd)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx<'a, Param1: ::windows::runtime::IntoParam<'a, IStorage>>(polestm: *mut OLESTREAM, pstg: Param1, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::STGMEDIUM) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleConvertOLESTREAMToIStorageEx(polestm: *mut OLESTREAM, pstg: ::windows::runtime::RawPtr, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut ::std::mem::ManuallyDrop<super::STGMEDIUM>) -> ::windows::runtime::HRESULT;
        }
        OleConvertOLESTREAMToIStorageEx(::std::mem::transmute(polestm), pstg.into_param().abi(), ::std::mem::transmute(pcfformat), ::std::mem::transmute(plwwidth), ::std::mem::transmute(plheight), ::std::mem::transmute(pdwsize), ::std::mem::transmute(pmedium)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDI_THUMBNAIL: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_BYTECOUNT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_CATEGORY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_COMPANY: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_DOCPARTS: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_HEADINGPAIR: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_HIDDENCOUNT: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_LINECOUNT: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_LINKSDIRTY: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_MANAGER: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_MMCLIPCOUNT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_NOTECOUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_PARCOUNT: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_PRESFORMAT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_SCALE: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDDSI_SLIDECOUNT: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_COPYRIGHT: i32 = 11i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_EDITOR: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_OWNER: i32 = 8i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_PRODUCTION: i32 = 10i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_PROJECT: i32 = 6i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_RATING: i32 = 9i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_SEQUENCE_NO: i32 = 5i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_SOURCE: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_STATUS: i32 = 7i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PIDMSI_STATUS_VALUE(pub i32);
pub const PIDMSI_STATUS_NORMAL: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(0i32);
pub const PIDMSI_STATUS_NEW: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(1i32);
pub const PIDMSI_STATUS_PRELIM: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(2i32);
pub const PIDMSI_STATUS_DRAFT: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(3i32);
pub const PIDMSI_STATUS_INPROGRESS: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(4i32);
pub const PIDMSI_STATUS_EDIT: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(5i32);
pub const PIDMSI_STATUS_REVIEW: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(6i32);
pub const PIDMSI_STATUS_PROOF: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(7i32);
pub const PIDMSI_STATUS_FINAL: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(8i32);
pub const PIDMSI_STATUS_OTHER: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(32767i32);
impl ::std::convert::From<i32> for PIDMSI_STATUS_VALUE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PIDMSI_STATUS_VALUE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDMSI_SUPPLIER: i32 = 3i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_APPNAME: i32 = 18i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_AUTHOR: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_CHARCOUNT: i32 = 16i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_COMMENTS: i32 = 6i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_CREATE_DTM: i32 = 12i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_DOC_SECURITY: i32 = 19i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_EDITTIME: i32 = 10i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_KEYWORDS: i32 = 5i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_LASTAUTHOR: i32 = 8i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_LASTPRINTED: i32 = 11i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_LASTSAVE_DTM: i32 = 13i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_PAGECOUNT: i32 = 14i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_REVNUMBER: i32 = 9i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_SUBJECT: i32 = 3i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_TEMPLATE: i32 = 7i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_THUMBNAIL: i32 = 17i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_TITLE: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PIDSI_WORDCOUNT: i32 = 15i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_BEHAVIOR: u32 = 2147483651u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_CODEPAGE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_DICTIONARY: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_FIRST_NAME_DEFAULT: u32 = 4095u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_FIRST_USABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_ILLEGAL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_LOCALE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_MAX_READONLY: u32 = 3221225471u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_MIN_READONLY: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_MODIFY_TIME: u32 = 2147483649u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PID_SECURITY: u32 = 2147483650u32;
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct PMemoryAllocator(pub u8);
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSETFLAG_ANSI: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSETFLAG_CASE_SENSITIVE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSETFLAG_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSETFLAG_NONSIMPLE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSETFLAG_UNBUFFERED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
pub struct PROPSPEC {
    pub ulKind: PROPSPEC_KIND,
    pub Anonymous: PROPSPEC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl PROPSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROPSPEC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROPSPEC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROPSPEC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROPSPEC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PROPSPEC_0 {
    pub propid: u32,
    pub lpwstr: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PROPSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROPSPEC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROPSPEC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROPSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROPSPEC_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPSPEC_KIND(pub u32);
pub const PRSPEC_LPWSTR: PROPSPEC_KIND = PROPSPEC_KIND(0u32);
pub const PRSPEC_PROPID: PROPSPEC_KIND = PROPSPEC_KIND(1u32);
impl ::std::convert::From<u32> for PROPSPEC_KIND {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPSPEC_KIND {
    type Abi = Self;
}
impl ::std::ops::BitOr for PROPSPEC_KIND {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PROPSPEC_KIND {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PROPSPEC_KIND {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PROPSPEC_KIND {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PROPSPEC_KIND {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::clone::Clone for PROPVARIANT {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl PROPVARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for PROPVARIANT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for PROPVARIANT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for PROPVARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for PROPVARIANT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::clone::Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
pub union PROPVARIANT_0 {
    pub Anonymous: ::std::mem::ManuallyDrop<PROPVARIANT_0_0>,
    pub decVal: super::super::super::Foundation::DECIMAL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl PROPVARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for PROPVARIANT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for PROPVARIANT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for PROPVARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for PROPVARIANT_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::clone::Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
pub struct PROPVARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl PROPVARIANT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for PROPVARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for PROPVARIANT_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for PROPVARIANT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for PROPVARIANT_0_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::clone::Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
pub union PROPVARIANT_0_0_0 {
    pub cVal: super::super::super::Foundation::CHAR,
    pub bVal: u8,
    pub iVal: i16,
    pub uiVal: u16,
    pub lVal: i32,
    pub ulVal: u32,
    pub intVal: i32,
    pub uintVal: u32,
    pub hVal: i64,
    pub uhVal: u64,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: i16,
    pub __OBSOLETE__VARIANT_BOOL: i16,
    pub scode: i32,
    pub cyVal: super::CY,
    pub date: f64,
    pub filetime: super::super::super::Foundation::FILETIME,
    pub puuid: *mut ::windows::runtime::GUID,
    pub pclipdata: *mut CLIPDATA,
    pub bstrVal: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
    pub bstrblobVal: BSTRBLOB,
    pub blob: super::BLOB,
    pub pszVal: super::super::super::Foundation::PSTR,
    pub pwszVal: super::super::super::Foundation::PWSTR,
    pub punkVal: ::windows::runtime::RawPtr,
    pub pdispVal: ::windows::runtime::RawPtr,
    pub pStream: ::windows::runtime::RawPtr,
    pub pStorage: ::windows::runtime::RawPtr,
    pub pVersionedStream: *mut ::std::mem::ManuallyDrop<VERSIONEDSTREAM>,
    pub parray: *mut super::SAFEARRAY,
    pub cac: CAC,
    pub caub: CAUB,
    pub cai: CAI,
    pub caui: CAUI,
    pub cal: CAL,
    pub caul: CAUL,
    pub cah: CAH,
    pub cauh: CAUH,
    pub caflt: CAFLT,
    pub cadbl: CADBL,
    pub cabool: CABOOL,
    pub cascode: CASCODE,
    pub cacy: CACY,
    pub cadate: CADATE,
    pub cafiletime: CAFILETIME,
    pub cauuid: CACLSID,
    pub caclipdata: CACLIPDATA,
    pub cabstr: CABSTR,
    pub cabstrblob: CABSTRBLOB,
    pub calpstr: CALPSTR,
    pub calpwstr: CALPWSTR,
    pub capropvar: CAPROPVARIANT,
    pub pcVal: super::super::super::Foundation::PSTR,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub puiVal: *mut u16,
    pub plVal: *mut i32,
    pub pulVal: *mut u32,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut i16,
    pub pdecVal: *mut super::super::super::Foundation::DECIMAL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
    pub ppunkVal: *mut ::windows::runtime::RawPtr,
    pub ppdispVal: *mut ::windows::runtime::RawPtr,
    pub pparray: *mut *mut super::SAFEARRAY,
    pub pvarVal: *mut ::std::mem::ManuallyDrop<PROPVARIANT>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl PROPVARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for PROPVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for PROPVARIANT_0_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for PROPVARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for PROPVARIANT_0_0_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const PRSPEC_INVALID: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PropStgNameToFmtId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(oszname: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropStgNameToFmtId(oszname: super::super::super::Foundation::PWSTR, pfmtid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PropStgNameToFmtId(oszname.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn PropVariantClear(pvar: *mut PROPVARIANT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantClear(pvar: *mut ::std::mem::ManuallyDrop<PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        PropVariantClear(::std::mem::transmute(pvar)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn PropVariantCopy(pvardest: *mut PROPVARIANT, pvarsrc: *const PROPVARIANT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantCopy(pvardest: *mut ::std::mem::ManuallyDrop<PROPVARIANT>, pvarsrc: *const ::std::mem::ManuallyDrop<PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        PropVariantCopy(::std::mem::transmute(pvardest), ::std::mem::transmute(pvarsrc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn ReadClassStg<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(pstg: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadClassStg(pstg: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        ReadClassStg(pstg.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn ReadClassStm<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(pstm: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadClassStm(pstm: ::windows::runtime::RawPtr, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        ReadClassStm(pstm.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadFmtUserTypeStg<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(pstg: Param0, pcf: *mut u16, lplpszusertype: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadFmtUserTypeStg(pstg: ::windows::runtime::RawPtr, pcf: *mut u16, lplpszusertype: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        ReadFmtUserTypeStg(pstg.into_param().abi(), ::std::mem::transmute(pcf), ::std::mem::transmute(lplpszusertype)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct RemSNB {
    pub ulCntStr: u32,
    pub ulCntChar: u32,
    pub rgString: [u16; 1],
}
impl RemSNB {}
impl ::std::default::Default for RemSNB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RemSNB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RemSNB").field("ulCntStr", &self.ulCntStr).field("ulCntChar", &self.ulCntChar).field("rgString", &self.rgString).finish()
    }
}
impl ::std::cmp::PartialEq for RemSNB {
    fn eq(&self, other: &Self) -> bool {
        self.ulCntStr == other.ulCntStr && self.ulCntChar == other.ulCntChar && self.rgString == other.rgString
    }
}
impl ::std::cmp::Eq for RemSNB {}
unsafe impl ::windows::runtime::Abi for RemSNB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct SERIALIZEDPROPERTYVALUE {
    pub dwType: u32,
    pub rgb: [u8; 1],
}
impl SERIALIZEDPROPERTYVALUE {}
impl ::std::default::Default for SERIALIZEDPROPERTYVALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERIALIZEDPROPERTYVALUE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERIALIZEDPROPERTYVALUE").field("dwType", &self.dwType).field("rgb", &self.rgb).finish()
    }
}
impl ::std::cmp::PartialEq for SERIALIZEDPROPERTYVALUE {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.rgb == other.rgb
    }
}
impl ::std::cmp::Eq for SERIALIZEDPROPERTYVALUE {}
unsafe impl ::windows::runtime::Abi for SERIALIZEDPROPERTYVALUE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct STATFLAG(pub i32);
pub const STATFLAG_DEFAULT: STATFLAG = STATFLAG(0i32);
pub const STATFLAG_NONAME: STATFLAG = STATFLAG(1i32);
pub const STATFLAG_NOOPEN: STATFLAG = STATFLAG(2i32);
impl ::std::convert::From<i32> for STATFLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STATFLAG {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
pub struct STATPROPSETSTG {
    pub fmtid: ::windows::runtime::GUID,
    pub clsid: ::windows::runtime::GUID,
    pub grfFlags: u32,
    pub mtime: super::super::super::Foundation::FILETIME,
    pub ctime: super::super::super::Foundation::FILETIME,
    pub atime: super::super::super::Foundation::FILETIME,
    pub dwOSVersion: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl STATPROPSETSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STATPROPSETSTG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STATPROPSETSTG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STATPROPSETSTG").field("fmtid", &self.fmtid).field("clsid", &self.clsid).field("grfFlags", &self.grfFlags).field("mtime", &self.mtime).field("ctime", &self.ctime).field("atime", &self.atime).field("dwOSVersion", &self.dwOSVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STATPROPSETSTG {
    fn eq(&self, other: &Self) -> bool {
        self.fmtid == other.fmtid && self.clsid == other.clsid && self.grfFlags == other.grfFlags && self.mtime == other.mtime && self.ctime == other.ctime && self.atime == other.atime && self.dwOSVersion == other.dwOSVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STATPROPSETSTG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STATPROPSETSTG {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
pub struct STATPROPSTG {
    pub lpwstrName: super::super::super::Foundation::PWSTR,
    pub propid: u32,
    pub vt: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl STATPROPSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STATPROPSTG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STATPROPSTG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STATPROPSTG").field("lpwstrName", &self.lpwstrName).field("propid", &self.propid).field("vt", &self.vt).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STATPROPSTG {
    fn eq(&self, other: &Self) -> bool {
        self.lpwstrName == other.lpwstrName && self.propid == other.propid && self.vt == other.vt
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STATPROPSTG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STATPROPSTG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct STGC(pub i32);
pub const STGC_DEFAULT: STGC = STGC(0i32);
pub const STGC_OVERWRITE: STGC = STGC(1i32);
pub const STGC_ONLYIFCURRENT: STGC = STGC(2i32);
pub const STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE: STGC = STGC(4i32);
pub const STGC_CONSOLIDATE: STGC = STGC(8i32);
impl ::std::convert::From<i32> for STGC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STGC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGFMT_ANY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGFMT_DOCFILE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGFMT_DOCUMENT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGFMT_FILE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGFMT_NATIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGFMT_STORAGE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct STGMOVE(pub i32);
pub const STGMOVE_MOVE: STGMOVE = STGMOVE(0i32);
pub const STGMOVE_COPY: STGMOVE = STGMOVE(1i32);
pub const STGMOVE_SHALLOWCOPY: STGMOVE = STGMOVE(2i32);
impl ::std::convert::From<i32> for STGMOVE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STGMOVE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_CONVERT: i32 = 131072i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_CREATE: i32 = 4096i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_DELETEONRELEASE: i32 = 67108864i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_DIRECT: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_DIRECT_SWMR: i32 = 4194304i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_FAILIFTHERE: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_NOSCRATCH: i32 = 1048576i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_NOSNAPSHOT: i32 = 2097152i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_PRIORITY: i32 = 262144i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_READ: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_READWRITE: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_SHARE_DENY_NONE: i32 = 64i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_SHARE_DENY_READ: i32 = 48i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_SHARE_DENY_WRITE: i32 = 32i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_SHARE_EXCLUSIVE: i32 = 16i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_SIMPLE: i32 = 134217728i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_TRANSACTED: i32 = 65536i32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGM_WRITE: i32 = 1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
pub struct STGOPTIONS {
    pub usVersion: u16,
    pub reserved: u16,
    pub ulSectorSize: u32,
    pub pwcsTemplateFile: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl STGOPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STGOPTIONS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STGOPTIONS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STGOPTIONS").field("usVersion", &self.usVersion).field("reserved", &self.reserved).field("ulSectorSize", &self.ulSectorSize).field("pwcsTemplateFile", &self.pwcsTemplateFile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STGOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.usVersion == other.usVersion && self.reserved == other.reserved && self.ulSectorSize == other.ulSectorSize && self.pwcsTemplateFile == other.pwcsTemplateFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STGOPTIONS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STGOPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub const STGOPTIONS_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConvertStg<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(pstg: Param0, fconvert: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConvertStg(pstg: ::windows::runtime::RawPtr, fconvert: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        SetConvertStg(pstg.into_param().abi(), fconvert.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn StgConvertPropertyToVariant(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut PROPVARIANT, pma: *const PMemoryAllocator) -> super::super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgConvertPropertyToVariant(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut ::std::mem::ManuallyDrop<PROPVARIANT>, pma: *const PMemoryAllocator) -> super::super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(StgConvertPropertyToVariant(::std::mem::transmute(pprop), ::std::mem::transmute(codepage), ::std::mem::transmute(pvar), ::std::mem::transmute(pma)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn StgConvertVariantToProperty<'a, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOLEAN>>(pvar: *const PROPVARIANT, codepage: u16, pprop: *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32, pid: u32, freserved: Param5, pcindirect: *mut u32) -> *mut SERIALIZEDPROPERTYVALUE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgConvertVariantToProperty(pvar: *const ::std::mem::ManuallyDrop<PROPVARIANT>, codepage: u16, pprop: *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32, pid: u32, freserved: super::super::super::Foundation::BOOLEAN, pcindirect: *mut u32) -> *mut SERIALIZEDPROPERTYVALUE;
        }
        ::std::mem::transmute(StgConvertVariantToProperty(::std::mem::transmute(pvar), ::std::mem::transmute(codepage), ::std::mem::transmute(pprop), ::std::mem::transmute(pcb), ::std::mem::transmute(pid), freserved.into_param().abi(), ::std::mem::transmute(pcindirect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgCreateDocfile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwcsname: Param0, grfmode: u32, reserved: u32) -> ::windows::runtime::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgCreateDocfile(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgCreateDocfile(pwcsname.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(reserved), &mut result__).from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn StgCreateDocfileOnILockBytes<'a, Param0: ::windows::runtime::IntoParam<'a, ILockBytes>>(plkbyt: Param0, grfmode: u32, reserved: u32) -> ::windows::runtime::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgCreateDocfileOnILockBytes(plkbyt: ::windows::runtime::RawPtr, grfmode: u32, reserved: u32, ppstgopen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgCreateDocfileOnILockBytes(plkbyt.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(reserved), &mut result__).from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn StgCreatePropSetStg<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(pstorage: Param0, dwreserved: u32) -> ::windows::runtime::Result<IPropertySetStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgCreatePropSetStg(pstorage: ::windows::runtime::RawPtr, dwreserved: u32, pppropsetstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IPropertySetStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgCreatePropSetStg(pstorage.into_param().abi(), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<IPropertySetStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn StgCreatePropStg<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punk: Param0, fmtid: *const ::windows::runtime::GUID, pclsid: *const ::windows::runtime::GUID, grfflags: u32, dwreserved: u32) -> ::windows::runtime::Result<IPropertyStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgCreatePropStg(punk: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, pclsid: *const ::windows::runtime::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IPropertyStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgCreatePropStg(punk.into_param().abi(), ::std::mem::transmute(fmtid), ::std::mem::transmute(pclsid), ::std::mem::transmute(grfflags), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<IPropertyStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn StgCreateStorageEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows::runtime::GUID, ppobjectopen: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgCreateStorageEx(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows::runtime::GUID, ppobjectopen: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        StgCreateStorageEx(pwcsname.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(stgfmt), ::std::mem::transmute(grfattrs), ::std::mem::transmute(pstgoptions), ::std::mem::transmute(psecuritydescriptor), ::std::mem::transmute(riid), ::std::mem::transmute(ppobjectopen)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32) -> ::windows::runtime::Result<PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32, ppropvar: *mut ::std::mem::ManuallyDrop<PROPVARIANT>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgDeserializePropVariant(::std::mem::transmute(pprop), ::std::mem::transmute(cbmax), &mut result__).from_abi::<PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgGetIFillLockBytesOnFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwcsname: Param0) -> ::windows::runtime::Result<IFillLockBytes> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgGetIFillLockBytesOnFile(pwcsname: super::super::super::Foundation::PWSTR, ppflb: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IFillLockBytes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgGetIFillLockBytesOnFile(pwcsname.into_param().abi(), &mut result__).from_abi::<IFillLockBytes>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn StgGetIFillLockBytesOnILockBytes<'a, Param0: ::windows::runtime::IntoParam<'a, ILockBytes>>(pilb: Param0) -> ::windows::runtime::Result<IFillLockBytes> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgGetIFillLockBytesOnILockBytes(pilb: ::windows::runtime::RawPtr, ppflb: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IFillLockBytes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgGetIFillLockBytesOnILockBytes(pilb.into_param().abi(), &mut result__).from_abi::<IFillLockBytes>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgIsStorageFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwcsname: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgIsStorageFile(pwcsname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        StgIsStorageFile(pwcsname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn StgIsStorageILockBytes<'a, Param0: ::windows::runtime::IntoParam<'a, ILockBytes>>(plkbyt: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgIsStorageILockBytes(plkbyt: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        StgIsStorageILockBytes(plkbyt.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn StgOpenAsyncDocfileOnIFillLockBytes<'a, Param0: ::windows::runtime::IntoParam<'a, IFillLockBytes>>(pflb: Param0, grfmode: u32, asyncflags: u32) -> ::windows::runtime::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgOpenAsyncDocfileOnIFillLockBytes(pflb: ::windows::runtime::RawPtr, grfmode: u32, asyncflags: u32, ppstgopen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgOpenAsyncDocfileOnIFillLockBytes(pflb.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(asyncflags), &mut result__).from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgOpenLayoutDocfile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwcsdfname: Param0, grfmode: u32, reserved: u32) -> ::windows::runtime::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgOpenLayoutDocfile(pwcsdfname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgOpenLayoutDocfile(pwcsdfname.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(reserved), &mut result__).from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn StgOpenPropStg<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punk: Param0, fmtid: *const ::windows::runtime::GUID, grfflags: u32, dwreserved: u32) -> ::windows::runtime::Result<IPropertyStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgOpenPropStg(punk: ::windows::runtime::RawPtr, fmtid: *const ::windows::runtime::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IPropertyStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgOpenPropStg(punk.into_param().abi(), ::std::mem::transmute(fmtid), ::std::mem::transmute(grfflags), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<IPropertyStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgOpenStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IStorage>>(pwcsname: Param0, pstgpriority: Param1, grfmode: u32, snbexclude: *const *const u16, reserved: u32) -> ::windows::runtime::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgOpenStorage(pwcsname: super::super::super::Foundation::PWSTR, pstgpriority: ::windows::runtime::RawPtr, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgOpenStorage(pwcsname.into_param().abi(), pstgpriority.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(snbexclude), ::std::mem::transmute(reserved), &mut result__).from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn StgOpenStorageEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows::runtime::GUID, ppobjectopen: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgOpenStorageEx(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows::runtime::GUID, ppobjectopen: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        StgOpenStorageEx(pwcsname.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(stgfmt), ::std::mem::transmute(grfattrs), ::std::mem::transmute(pstgoptions), ::std::mem::transmute(psecuritydescriptor), ::std::mem::transmute(riid), ::std::mem::transmute(ppobjectopen)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn StgOpenStorageOnILockBytes<'a, Param0: ::windows::runtime::IntoParam<'a, ILockBytes>, Param1: ::windows::runtime::IntoParam<'a, IStorage>>(plkbyt: Param0, pstgpriority: Param1, grfmode: u32, snbexclude: *const *const u16, reserved: u32) -> ::windows::runtime::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgOpenStorageOnILockBytes(plkbyt: ::windows::runtime::RawPtr, pstgpriority: ::windows::runtime::RawPtr, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgOpenStorageOnILockBytes(plkbyt.into_param().abi(), pstgpriority.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(snbexclude), ::std::mem::transmute(reserved), &mut result__).from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32;
        }
        ::std::mem::transmute(StgPropertyLengthAsVariant(::std::mem::transmute(pprop), ::std::mem::transmute(cbprop), ::std::mem::transmute(codepage), ::std::mem::transmute(breserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn StgSerializePropVariant(ppropvar: *const PROPVARIANT, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgSerializePropVariant(ppropvar: *const ::std::mem::ManuallyDrop<PROPVARIANT>, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows::runtime::HRESULT;
        }
        StgSerializePropVariant(::std::mem::transmute(ppropvar), ::std::mem::transmute(ppprop), ::std::mem::transmute(pcb)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgSetTimes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(lpszname: Param0, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgSetTimes(lpszname: super::super::super::Foundation::PWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
        }
        StgSetTimes(lpszname.into_param().abi(), ::std::mem::transmute(pctime), ::std::mem::transmute(patime), ::std::mem::transmute(pmtime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
pub struct VERSIONEDSTREAM {
    pub guidVersion: ::windows::runtime::GUID,
    pub pStream: ::std::option::Option<super::IStream>,
}
impl VERSIONEDSTREAM {}
impl ::std::default::Default for VERSIONEDSTREAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VERSIONEDSTREAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VERSIONEDSTREAM").field("guidVersion", &self.guidVersion).field("pStream", &self.pStream).finish()
    }
}
impl ::std::cmp::PartialEq for VERSIONEDSTREAM {
    fn eq(&self, other: &Self) -> bool {
        self.guidVersion == other.guidVersion && self.pStream == other.pStream
    }
}
impl ::std::cmp::Eq for VERSIONEDSTREAM {}
unsafe impl ::windows::runtime::Abi for VERSIONEDSTREAM {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn WriteClassStg<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(pstg: Param0, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteClassStg(pstg: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        WriteClassStg(pstg.into_param().abi(), ::std::mem::transmute(rclsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn WriteClassStm<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(pstm: Param0, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteClassStm(pstm: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        WriteClassStm(pstm.into_param().abi(), ::std::mem::transmute(rclsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_StructuredStorage`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteFmtUserTypeStg<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pstg: Param0, cf: u16, lpszusertype: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteFmtUserTypeStg(pstg: ::windows::runtime::RawPtr, cf: u16, lpszusertype: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        WriteFmtUserTypeStg(pstg.into_param().abi(), ::std::mem::transmute(cf), lpszusertype.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}

#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CABOOL")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::Foundation::BSTR,
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
        fmt.debug_struct("CABSTR")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut super::super::System::SystemServices::BSTRBLOB,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl CABSTRBLOB {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for CABSTRBLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for CABSTRBLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CABSTRBLOB")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for CABSTRBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for CABSTRBLOB {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for CABSTRBLOB {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CAC {
    pub cElems: u32,
    pub pElems: super::super::Foundation::PSTR,
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
        fmt.debug_struct("CAC")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut super::super::System::SystemServices::CLIPDATA,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl CACLIPDATA {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for CACLIPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for CACLIPDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CACLIPDATA")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for CACLIPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for CACLIPDATA {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for CACLIPDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CACLSID")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut super::super::System::SystemServices::CY,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl CACY {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for CACY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for CACY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CACY")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for CACY {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for CACY {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for CACY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CADATE")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CADBL")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut super::super::Foundation::FILETIME,
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
        fmt.debug_struct("CAFILETIME")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CAFLT")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CAH")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CAI")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CAL")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::Foundation::PSTR,
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
        fmt.debug_struct("CALPSTR")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::Foundation::PWSTR,
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
        fmt.debug_struct("CALPWSTR")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl CAPROPVARIANT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for CAPROPVARIANT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::fmt::Debug for CAPROPVARIANT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAPROPVARIANT")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for CAPROPVARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for CAPROPVARIANT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for CAPROPVARIANT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CASCODE")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CAUB")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CAUH")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CAUI")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CAUL")
            .field("cElems", &self.cElems)
            .field("pElems", &self.pElems)
            .finish()
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
    type DefaultType = Self;
}
pub const CCH_MAX_PROPSTG_NAME: u32 = 31u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONVERT_A {
    pub szOldDll: super::super::Foundation::PSTR,
    pub Anonymous: CONVERT_A_0,
}
#[cfg(feature = "Win32_Foundation")]
impl CONVERT_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CONVERT_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CONVERT_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CONVERT_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CONVERT_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union CONVERT_A_0 {
    pub fFlags: u32,
    pub Anonymous: CONVERT_A_0_0,
}
impl CONVERT_A_0 {}
impl ::std::default::Default for CONVERT_A_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CONVERT_A_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CONVERT_A_0 {}
unsafe impl ::windows::runtime::Abi for CONVERT_A_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CONVERT_A_0_0 {
    pub _bitfield: u32,
}
impl CONVERT_A_0_0 {}
impl ::std::default::Default for CONVERT_A_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CONVERT_A_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CONVERT_A_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for CONVERT_A_0_0 {}
unsafe impl ::windows::runtime::Abi for CONVERT_A_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONVERT_W {
    pub szOldDll: super::super::Foundation::PWSTR,
    pub Anonymous: CONVERT_W_0,
}
#[cfg(feature = "Win32_Foundation")]
impl CONVERT_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CONVERT_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CONVERT_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CONVERT_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CONVERT_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union CONVERT_W_0 {
    pub fFlags: u32,
    pub Anonymous: CONVERT_W_0_0,
}
impl CONVERT_W_0 {}
impl ::std::default::Default for CONVERT_W_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CONVERT_W_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CONVERT_W_0 {}
unsafe impl ::windows::runtime::Abi for CONVERT_W_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CONVERT_W_0_0 {
    pub _bitfield: u32,
}
impl CONVERT_W_0_0 {}
impl ::std::default::Default for CONVERT_W_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CONVERT_W_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CONVERT_W_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for CONVERT_W_0_0 {}
unsafe impl ::windows::runtime::Abi for CONVERT_W_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CWCSTORAGENAME: u32 = 32u32;
pub unsafe fn CoBuildVersion() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn CoBuildVersion() -> u32;
        }
        ::std::mem::transmute(CoBuildVersion())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateILockBytesOnHGlobal<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hglobal: isize,
    fdeleteonrelease: Param1,
) -> ::windows::runtime::Result<ILockBytes> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn CreateILockBytesOnHGlobal(
                hglobal: isize,
                fdeleteonrelease: super::super::Foundation::BOOL,
                pplkbyt: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <ILockBytes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateILockBytesOnHGlobal(
            ::std::mem::transmute(hglobal),
            fdeleteonrelease.into_param().abi(),
            &mut result__,
        )
        .from_abi::<ILockBytes>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub unsafe fn CreateStdProgressIndicator<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IBindStatusCallback>,
>(
    hwndparent: Param0,
    psztitle: Param1,
    pibsccaller: Param2,
) -> ::windows::runtime::Result<super::super::System::Com::IBindStatusCallback> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn CreateStdProgressIndicator(
                hwndparent: super::super::Foundation::HWND,
                psztitle: super::super::Foundation::PWSTR,
                pibsccaller: ::windows::runtime::RawPtr,
                ppibsc: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__ : < super::super::System::Com:: IBindStatusCallback as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        CreateStdProgressIndicator(
            hwndparent.into_param().abi(),
            psztitle.into_param().abi(),
            pibsccaller.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::System::Com::IBindStatusCallback>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateStreamOnHGlobal<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hglobal: isize,
    fdeleteonrelease: Param1,
) -> ::windows::runtime::Result<IStream> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn CreateStreamOnHGlobal(
                hglobal: isize,
                fdeleteonrelease: super::super::Foundation::BOOL,
                ppstm: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateStreamOnHGlobal(
            ::std::mem::transmute(hglobal),
            fdeleteonrelease.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DcomChannelSetHResult(
    pvreserved: *const ::std::ffi::c_void,
    pulreserved: *const u32,
    appshr: ::windows::runtime::HRESULT,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn DcomChannelSetHResult(
                pvreserved: *const ::std::ffi::c_void,
                pulreserved: *const u32,
                appshr: ::windows::runtime::HRESULT,
            ) -> ::windows::runtime::HRESULT;
        }
        DcomChannelSetHResult(
            ::std::mem::transmute(pvreserved),
            ::std::mem::transmute(pulreserved),
            ::std::mem::transmute(appshr),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FmtIdToPropStgName(
    pfmtid: *const ::windows::runtime::GUID,
    oszname: super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn FmtIdToPropStgName(
                pfmtid: *const ::windows::runtime::GUID,
                oszname: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        FmtIdToPropStgName(
            ::std::mem::transmute(pfmtid),
            ::std::mem::transmute(oszname),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn FreePropVariantArray(
    cvariants: u32,
    rgvars: *mut PROPVARIANT,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn FreePropVariantArray(
                cvariants: u32,
                rgvars: *mut ::std::mem::ManuallyDrop<PROPVARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        FreePropVariantArray(
            ::std::mem::transmute(cvariants),
            ::std::mem::transmute(rgvars),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetConvertStg<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(
    pstg: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn GetConvertStg(pstg: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        GetConvertStg(pstg.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetHGlobalFromILockBytes<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ILockBytes>,
>(
    plkbyt: Param0,
) -> ::windows::runtime::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn GetHGlobalFromILockBytes(
                plkbyt: ::windows::runtime::RawPtr,
                phglobal: *mut isize,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetHGlobalFromILockBytes(plkbyt.into_param().abi(), &mut result__)
            .from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetHGlobalFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>>(
    pstm: Param0,
) -> ::windows::runtime::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn GetHGlobalFromStream(
                pstm: ::windows::runtime::RawPtr,
                phglobal: *mut isize,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetHGlobalFromStream(pstm.into_param().abi(), &mut result__).from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDirectWriterLock(::windows::runtime::IUnknown);
impl IDirectWriterLock {
    pub unsafe fn WaitForWriteAccess(&self, dwtimeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwtimeout),
        )
        .ok()
    }
    pub unsafe fn ReleaseWriteAccess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn HaveWriteAccess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectWriterLock {
    type Vtable = IDirectWriterLock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        242044306,
        26424,
        4559,
        [150, 8, 0, 170, 0, 104, 13, 180],
    );
}
impl ::std::convert::From<IDirectWriterLock> for ::windows::runtime::IUnknown {
    fn from(value: IDirectWriterLock) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectWriterLock> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectWriterLock) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectWriterLock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDirectWriterLock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectWriterLock_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwtimeout: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEnumSTATPROPSETSTG(::windows::runtime::IUnknown);
impl IEnumSTATPROPSETSTG {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut STATPROPSETSTG,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
            ::std::mem::transmute(rgelt),
            ::std::mem::transmute(pceltfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSTATPROPSETSTG> {
        let mut result__: <IEnumSTATPROPSETSTG as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumSTATPROPSETSTG>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSTATPROPSETSTG {
    type Vtable = IEnumSTATPROPSETSTG_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(315, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumSTATPROPSETSTG> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSTATPROPSETSTG) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumSTATPROPSETSTG> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSTATPROPSETSTG) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSTATPROPSETSTG {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumSTATPROPSETSTG {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATPROPSETSTG_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
        rgelt: *mut STATPROPSETSTG,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEnumSTATPROPSTG(::windows::runtime::IUnknown);
impl IEnumSTATPROPSTG {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut STATPROPSTG,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
            ::std::mem::transmute(rgelt),
            ::std::mem::transmute(pceltfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSTATPROPSTG> {
        let mut result__: <IEnumSTATPROPSTG as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumSTATPROPSTG>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSTATPROPSTG {
    type Vtable = IEnumSTATPROPSTG_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(313, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumSTATPROPSTG> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSTATPROPSTG) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumSTATPROPSTG> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSTATPROPSTG) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSTATPROPSTG {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumSTATPROPSTG {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATPROPSTG_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
        rgelt: *mut STATPROPSTG,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEnumSTATSTG(::windows::runtime::IUnknown);
impl IEnumSTATSTG {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut STATSTG,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
            ::std::mem::transmute(rgelt),
            ::std::mem::transmute(pceltfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSTATSTG> {
        let mut result__: <IEnumSTATSTG as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumSTATSTG>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSTATSTG {
    type Vtable = IEnumSTATSTG_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(13, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumSTATSTG> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSTATSTG) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumSTATSTG> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSTATSTG) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSTATSTG {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumSTATSTG {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATSTG_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
        rgelt: *mut STATSTG,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IFillLockBytes(::windows::runtime::IUnknown);
impl IFillLockBytes {
    pub unsafe fn FillAppend(
        &self,
        pv: *const ::std::ffi::c_void,
        cb: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn FillAt(
        &self,
        uloffset: u64,
        pv: *const ::std::ffi::c_void,
        cb: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uloffset),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn SetFillSize(&self, ulsize: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ulsize),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Terminate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        bcanceled: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            bcanceled.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFillLockBytes {
    type Vtable = IFillLockBytes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2580213776,
        16734,
        4559,
        [136, 20, 0, 170, 0, 181, 105, 245],
    );
}
impl ::std::convert::From<IFillLockBytes> for ::windows::runtime::IUnknown {
    fn from(value: IFillLockBytes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFillLockBytes> for ::windows::runtime::IUnknown {
    fn from(value: &IFillLockBytes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFillLockBytes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFillLockBytes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFillLockBytes_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcbwritten: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uloffset: u64,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcbwritten: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ulsize: u64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bcanceled: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ILayoutStorage(::windows::runtime::IUnknown);
impl ILayoutStorage {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LayoutScript(
        &self,
        pstoragelayout: *const StorageLayout,
        nentries: u32,
        glfinterleavedflag: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pstoragelayout),
            ::std::mem::transmute(nentries),
            ::std::mem::transmute(glfinterleavedflag),
        )
        .ok()
    }
    pub unsafe fn BeginMonitor(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EndMonitor(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReLayoutDocfile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pwcsnewdfname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            pwcsnewdfname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReLayoutDocfileOnILockBytes<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ILockBytes>,
    >(
        &self,
        pilockbytes: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            pilockbytes.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILayoutStorage {
    type Vtable = ILayoutStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        242044304,
        26424,
        4559,
        [150, 8, 0, 170, 0, 104, 13, 180],
    );
}
impl ::std::convert::From<ILayoutStorage> for ::windows::runtime::IUnknown {
    fn from(value: ILayoutStorage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILayoutStorage> for ::windows::runtime::IUnknown {
    fn from(value: &ILayoutStorage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILayoutStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ILayoutStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutStorage_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstoragelayout: *const StorageLayout,
        nentries: u32,
        glfinterleavedflag: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwcsnewdfname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pilockbytes: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ILockBytes(::windows::runtime::IUnknown);
impl ILockBytes {
    pub unsafe fn ReadAt(
        &self,
        uloffset: u64,
        pv: *mut ::std::ffi::c_void,
        cb: u32,
        pcbread: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uloffset),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(pcbread),
        )
        .ok()
    }
    pub unsafe fn WriteAt(
        &self,
        uloffset: u64,
        pv: *const ::std::ffi::c_void,
        cb: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uloffset),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetSize(&self, cb: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cb),
        )
        .ok()
    }
    pub unsafe fn LockRegion(
        &self,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(liboffset),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(dwlocktype),
        )
        .ok()
    }
    pub unsafe fn UnlockRegion(
        &self,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(liboffset),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(dwlocktype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(
        &self,
        pstatstg: *mut STATSTG,
        grfstatflag: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pstatstg),
            ::std::mem::transmute(grfstatflag),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILockBytes {
    type Vtable = ILockBytes_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(10, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ILockBytes> for ::windows::runtime::IUnknown {
    fn from(value: ILockBytes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILockBytes> for ::windows::runtime::IUnknown {
    fn from(value: &ILockBytes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILockBytes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ILockBytes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockBytes_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uloffset: u64,
        pv: *mut ::std::ffi::c_void,
        cb: u32,
        pcbread: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uloffset: u64,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcbwritten: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cb: u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstatstg: *mut STATSTG,
        grfstatflag: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPropertySetStorage(::windows::runtime::IUnknown);
impl IPropertySetStorage {
    pub unsafe fn Create(
        &self,
        rfmtid: *const ::windows::runtime::GUID,
        pclsid: *const ::windows::runtime::GUID,
        grfflags: u32,
        grfmode: u32,
    ) -> ::windows::runtime::Result<IPropertyStorage> {
        let mut result__: <IPropertyStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rfmtid),
            ::std::mem::transmute(pclsid),
            ::std::mem::transmute(grfflags),
            ::std::mem::transmute(grfmode),
            &mut result__,
        )
        .from_abi::<IPropertyStorage>(result__)
    }
    pub unsafe fn Open(
        &self,
        rfmtid: *const ::windows::runtime::GUID,
        grfmode: u32,
    ) -> ::windows::runtime::Result<IPropertyStorage> {
        let mut result__: <IPropertyStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rfmtid),
            ::std::mem::transmute(grfmode),
            &mut result__,
        )
        .from_abi::<IPropertyStorage>(result__)
    }
    pub unsafe fn Delete(
        &self,
        rfmtid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rfmtid),
        )
        .ok()
    }
    pub unsafe fn Enum(&self) -> ::windows::runtime::Result<IEnumSTATPROPSETSTG> {
        let mut result__: <IEnumSTATPROPSETSTG as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumSTATPROPSETSTG>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertySetStorage {
    type Vtable = IPropertySetStorage_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(314, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IPropertySetStorage> for ::windows::runtime::IUnknown {
    fn from(value: IPropertySetStorage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertySetStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertySetStorage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertySetStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertySetStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetStorage_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rfmtid: *const ::windows::runtime::GUID,
        pclsid: *const ::windows::runtime::GUID,
        grfflags: u32,
        grfmode: u32,
        ppprstg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rfmtid: *const ::windows::runtime::GUID,
        grfmode: u32,
        ppprstg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rfmtid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPropertyStorage(::windows::runtime::IUnknown);
impl IPropertyStorage {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn ReadMultiple(
        &self,
        cpspec: u32,
        rgpspec: *const PROPSPEC,
        rgpropvar: *mut PROPVARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpspec),
            ::std::mem::transmute(rgpspec),
            ::std::mem::transmute(rgpropvar),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn WriteMultiple(
        &self,
        cpspec: u32,
        rgpspec: *const PROPSPEC,
        rgpropvar: *const PROPVARIANT,
        propidnamefirst: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpspec),
            ::std::mem::transmute(rgpspec),
            ::std::mem::transmute(rgpropvar),
            ::std::mem::transmute(propidnamefirst),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteMultiple(
        &self,
        cpspec: u32,
        rgpspec: *const PROPSPEC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpspec),
            ::std::mem::transmute(rgpspec),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadPropertyNames(
        &self,
        cpropid: u32,
        rgpropid: *const u32,
        rglpwstrname: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpropid),
            ::std::mem::transmute(rgpropid),
            ::std::mem::transmute(rglpwstrname),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WritePropertyNames(
        &self,
        cpropid: u32,
        rgpropid: *const u32,
        rglpwstrname: *const super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpropid),
            ::std::mem::transmute(rgpropid),
            ::std::mem::transmute(rglpwstrname),
        )
        .ok()
    }
    pub unsafe fn DeletePropertyNames(
        &self,
        cpropid: u32,
        rgpropid: *const u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpropid),
            ::std::mem::transmute(rgpropid),
        )
        .ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(grfcommitflags),
        )
        .ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Enum(&self) -> ::windows::runtime::Result<IEnumSTATPROPSTG> {
        let mut result__: <IEnumSTATPROPSTG as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumSTATPROPSTG>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTimes(
        &self,
        pctime: *const super::super::Foundation::FILETIME,
        patime: *const super::super::Foundation::FILETIME,
        pmtime: *const super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pctime),
            ::std::mem::transmute(patime),
            ::std::mem::transmute(pmtime),
        )
        .ok()
    }
    pub unsafe fn SetClass(
        &self,
        clsid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(clsid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self) -> ::windows::runtime::Result<STATPROPSETSTG> {
        let mut result__: <STATPROPSETSTG as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<STATPROPSETSTG>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPropertyStorage {
    type Vtable = IPropertyStorage_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(312, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IPropertyStorage> for ::windows::runtime::IUnknown {
    fn from(value: IPropertyStorage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPropertyStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IPropertyStorage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPropertyStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPropertyStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStorage_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpspec: u32,
        rgpspec: *const PROPSPEC,
        rgpropvar: *mut ::std::mem::ManuallyDrop<PROPVARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpspec: u32,
        rgpspec: *const PROPSPEC,
        rgpropvar: *const ::std::mem::ManuallyDrop<PROPVARIANT>,
        propidnamefirst: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpspec: u32,
        rgpspec: *const PROPSPEC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpropid: u32,
        rgpropid: *const u32,
        rglpwstrname: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpropid: u32,
        rgpropid: *const u32,
        rglpwstrname: *const super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpropid: u32,
        rgpropid: *const u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        grfcommitflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctime: *const super::super::Foundation::FILETIME,
        patime: *const super::super::Foundation::FILETIME,
        pmtime: *const super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clsid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstatpsstg: *mut STATPROPSETSTG,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IRootStorage(::windows::runtime::IUnknown);
impl IRootStorage {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SwitchToFile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszfile: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszfile.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRootStorage {
    type Vtable = IRootStorage_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(18, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IRootStorage> for ::windows::runtime::IUnknown {
    fn from(value: IRootStorage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRootStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IRootStorage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRootStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRootStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRootStorage_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszfile: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISequentialStream(::windows::runtime::IUnknown);
impl ISequentialStream {
    pub unsafe fn Read(
        &self,
        pv: *mut ::std::ffi::c_void,
        cb: u32,
        pcbread: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(pcbread),
        )
        .ok()
    }
    pub unsafe fn Write(
        &self,
        pv: *const ::std::ffi::c_void,
        cb: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISequentialStream {
    type Vtable = ISequentialStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        208878128,
        10780,
        4558,
        [173, 229, 0, 170, 0, 68, 119, 61],
    );
}
impl ::std::convert::From<ISequentialStream> for ::windows::runtime::IUnknown {
    fn from(value: ISequentialStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISequentialStream> for ::windows::runtime::IUnknown {
    fn from(value: &ISequentialStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISequentialStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISequentialStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStream_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pv: *mut ::std::ffi::c_void,
        cb: u32,
        pcbread: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcbwritten: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IStorage(::windows::runtime::IUnknown);
impl IStorage {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateStream<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pwcsname: Param0,
        grfmode: u32,
        reserved1: u32,
        reserved2: u32,
    ) -> ::windows::runtime::Result<IStream> {
        let mut result__: <IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pwcsname.into_param().abi(),
            ::std::mem::transmute(grfmode),
            ::std::mem::transmute(reserved1),
            ::std::mem::transmute(reserved2),
            &mut result__,
        )
        .from_abi::<IStream>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenStream<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pwcsname: Param0,
        reserved1: *mut ::std::ffi::c_void,
        grfmode: u32,
        reserved2: u32,
        ppstm: *mut ::std::option::Option<IStream>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pwcsname.into_param().abi(),
            ::std::mem::transmute(reserved1),
            ::std::mem::transmute(grfmode),
            ::std::mem::transmute(reserved2),
            ::std::mem::transmute(ppstm),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateStorage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pwcsname: Param0,
        grfmode: u32,
        reserved1: u32,
        reserved2: u32,
    ) -> ::windows::runtime::Result<IStorage> {
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            pwcsname.into_param().abi(),
            ::std::mem::transmute(grfmode),
            ::std::mem::transmute(reserved1),
            ::std::mem::transmute(reserved2),
            &mut result__,
        )
        .from_abi::<IStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenStorage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, IStorage>,
    >(
        &self,
        pwcsname: Param0,
        pstgpriority: Param1,
        grfmode: u32,
        snbexclude: *const *const u16,
        reserved: u32,
    ) -> ::windows::runtime::Result<IStorage> {
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            pwcsname.into_param().abi(),
            pstgpriority.into_param().abi(),
            ::std::mem::transmute(grfmode),
            ::std::mem::transmute(snbexclude),
            ::std::mem::transmute(reserved),
            &mut result__,
        )
        .from_abi::<IStorage>(result__)
    }
    pub unsafe fn CopyTo<'a, Param3: ::windows::runtime::IntoParam<'a, IStorage>>(
        &self,
        ciidexclude: u32,
        rgiidexclude: *const ::windows::runtime::GUID,
        snbexclude: *const *const u16,
        pstgdest: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ciidexclude),
            ::std::mem::transmute(rgiidexclude),
            ::std::mem::transmute(snbexclude),
            pstgdest.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveElementTo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, IStorage>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pwcsname: Param0,
        pstgdest: Param1,
        pwcsnewname: Param2,
        grfflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            pwcsname.into_param().abi(),
            pstgdest.into_param().abi(),
            pwcsnewname.into_param().abi(),
            ::std::mem::transmute(grfflags),
        )
        .ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(grfcommitflags),
        )
        .ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EnumElements(
        &self,
        reserved1: u32,
        reserved2: *mut ::std::ffi::c_void,
        reserved3: u32,
        ppenum: *mut ::std::option::Option<IEnumSTATSTG>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(reserved1),
            ::std::mem::transmute(reserved2),
            ::std::mem::transmute(reserved3),
            ::std::mem::transmute(ppenum),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyElement<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pwcsname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            pwcsname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenameElement<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pwcsoldname: Param0,
        pwcsnewname: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            pwcsoldname.into_param().abi(),
            pwcsnewname.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetElementTimes<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pwcsname: Param0,
        pctime: *const super::super::Foundation::FILETIME,
        patime: *const super::super::Foundation::FILETIME,
        pmtime: *const super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            pwcsname.into_param().abi(),
            ::std::mem::transmute(pctime),
            ::std::mem::transmute(patime),
            ::std::mem::transmute(pmtime),
        )
        .ok()
    }
    pub unsafe fn SetClass(
        &self,
        clsid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(clsid),
        )
        .ok()
    }
    pub unsafe fn SetStateBits(
        &self,
        grfstatebits: u32,
        grfmask: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(grfstatebits),
            ::std::mem::transmute(grfmask),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(
        &self,
        pstatstg: *mut STATSTG,
        grfstatflag: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pstatstg),
            ::std::mem::transmute(grfstatflag),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStorage {
    type Vtable = IStorage_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(11, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IStorage> for ::windows::runtime::IUnknown {
    fn from(value: IStorage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IStorage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorage_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwcsname: super::super::Foundation::PWSTR,
        grfmode: u32,
        reserved1: u32,
        reserved2: u32,
        ppstm: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwcsname: super::super::Foundation::PWSTR,
        reserved1: *mut ::std::ffi::c_void,
        grfmode: u32,
        reserved2: u32,
        ppstm: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwcsname: super::super::Foundation::PWSTR,
        grfmode: u32,
        reserved1: u32,
        reserved2: u32,
        ppstg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwcsname: super::super::Foundation::PWSTR,
        pstgpriority: ::windows::runtime::RawPtr,
        grfmode: u32,
        snbexclude: *const *const u16,
        reserved: u32,
        ppstg: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ciidexclude: u32,
        rgiidexclude: *const ::windows::runtime::GUID,
        snbexclude: *const *const u16,
        pstgdest: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwcsname: super::super::Foundation::PWSTR,
        pstgdest: ::windows::runtime::RawPtr,
        pwcsnewname: super::super::Foundation::PWSTR,
        grfflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        grfcommitflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reserved1: u32,
        reserved2: *mut ::std::ffi::c_void,
        reserved3: u32,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwcsname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwcsoldname: super::super::Foundation::PWSTR,
        pwcsnewname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwcsname: super::super::Foundation::PWSTR,
        pctime: *const super::super::Foundation::FILETIME,
        patime: *const super::super::Foundation::FILETIME,
        pmtime: *const super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clsid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        grfstatebits: u32,
        grfmask: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstatstg: *mut STATSTG,
        grfstatflag: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IStream(::windows::runtime::IUnknown);
impl IStream {
    pub unsafe fn Read(
        &self,
        pv: *mut ::std::ffi::c_void,
        cb: u32,
        pcbread: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(pcbread),
        )
        .ok()
    }
    pub unsafe fn Write(
        &self,
        pv: *const ::std::ffi::c_void,
        cb: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn Seek(
        &self,
        dlibmove: i64,
        dworigin: STREAM_SEEK,
    ) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dlibmove),
            ::std::mem::transmute(dworigin),
            &mut result__,
        )
        .from_abi::<u64>(result__)
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(libnewsize),
        )
        .ok()
    }
    pub unsafe fn CopyTo<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>>(
        &self,
        pstm: Param0,
        cb: u64,
        pcbread: *mut u64,
        pcbwritten: *mut u64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            pstm.into_param().abi(),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(pcbread),
            ::std::mem::transmute(pcbwritten),
        )
        .ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(grfcommitflags),
        )
        .ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn LockRegion(
        &self,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(liboffset),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(dwlocktype),
        )
        .ok()
    }
    pub unsafe fn UnlockRegion(
        &self,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(liboffset),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(dwlocktype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(
        &self,
        pstatstg: *mut STATSTG,
        grfstatflag: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pstatstg),
            ::std::mem::transmute(grfstatflag),
        )
        .ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IStream> {
        let mut result__: <IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IStream>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IStream {
    type Vtable = IStream_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(12, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IStream> for ::windows::runtime::IUnknown {
    fn from(value: IStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStream> for ::windows::runtime::IUnknown {
    fn from(value: &IStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IStream> for ISequentialStream {
    fn from(value: IStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStream> for ISequentialStream {
    fn from(value: &IStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISequentialStream> for IStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISequentialStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISequentialStream>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISequentialStream> for &IStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISequentialStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISequentialStream>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStream_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pv: *mut ::std::ffi::c_void,
        cb: u32,
        pcbread: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pv: *const ::std::ffi::c_void,
        cb: u32,
        pcbwritten: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dlibmove: i64,
        dworigin: STREAM_SEEK,
        plibnewposition: *mut u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        libnewsize: u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstm: ::windows::runtime::RawPtr,
        cb: u64,
        pcbread: *mut u64,
        pcbwritten: *mut u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        grfcommitflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        liboffset: u64,
        cb: u64,
        dwlocktype: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstatstg: *mut STATSTG,
        grfstatflag: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppstm: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct JET_API_PTR(pub usize);
impl ::std::default::Default for JET_API_PTR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for JET_API_PTR {}
unsafe impl ::windows::runtime::Abi for JET_API_PTR {
    type Abi = Self;
    type DefaultType = Self;
}
pub const JET_BASE_NAME_LENGTH: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_BKINFO {
    pub lgposMark: JET_LGPOS,
    pub Anonymous: JET_BKINFO_0,
    pub genLow: u32,
    pub genHigh: u32,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_BKINFO {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_BKINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_BKINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_BKINFO {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_BKINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub union JET_BKINFO_0 {
    pub logtimeMark: JET_LOGTIME,
    pub bklogtimeMark: JET_BKLOGTIME,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_BKINFO_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_BKINFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_BKINFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_BKINFO_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_BKINFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_BKLOGTIME {
    pub bSeconds: super::super::System::SystemServices::CHAR,
    pub bMinutes: super::super::System::SystemServices::CHAR,
    pub bHours: super::super::System::SystemServices::CHAR,
    pub bDay: super::super::System::SystemServices::CHAR,
    pub bMonth: super::super::System::SystemServices::CHAR,
    pub bYear: super::super::System::SystemServices::CHAR,
    pub Anonymous1: JET_BKLOGTIME_0,
    pub Anonymous2: JET_BKLOGTIME_1,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_BKLOGTIME {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_BKLOGTIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_BKLOGTIME {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_BKLOGTIME {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_BKLOGTIME {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub union JET_BKLOGTIME_0 {
    pub bFiller1: super::super::System::SystemServices::CHAR,
    pub Anonymous: JET_BKLOGTIME_0_0,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_BKLOGTIME_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_BKLOGTIME_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_BKLOGTIME_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_BKLOGTIME_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_BKLOGTIME_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_BKLOGTIME_0_0 {
    pub _bitfield: u8,
}
impl JET_BKLOGTIME_0_0 {}
impl ::std::default::Default for JET_BKLOGTIME_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_BKLOGTIME_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_BKLOGTIME_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for JET_BKLOGTIME_0_0 {}
unsafe impl ::windows::runtime::Abi for JET_BKLOGTIME_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub union JET_BKLOGTIME_1 {
    pub bFiller2: super::super::System::SystemServices::CHAR,
    pub Anonymous: JET_BKLOGTIME_1_0,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_BKLOGTIME_1 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_BKLOGTIME_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_BKLOGTIME_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_BKLOGTIME_1 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_BKLOGTIME_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_BKLOGTIME_1_0 {
    pub _bitfield: u8,
}
impl JET_BKLOGTIME_1_0 {}
impl ::std::default::Default for JET_BKLOGTIME_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_BKLOGTIME_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_BKLOGTIME_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for JET_BKLOGTIME_1_0 {}
unsafe impl ::windows::runtime::Abi for JET_BKLOGTIME_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub type JET_CALLBACK = unsafe extern "system" fn(
    sesid: JET_SESID,
    dbid: u32,
    tableid: JET_TABLEID,
    cbtyp: u32,
    pvarg1: *mut ::std::ffi::c_void,
    pvarg2: *mut ::std::ffi::c_void,
    pvcontext: *const ::std::ffi::c_void,
    ulunused: JET_API_PTR,
) -> i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_COLUMNBASE_A {
    pub cbStruct: u32,
    pub columnid: u32,
    pub coltyp: u32,
    pub wCountry: u16,
    pub langid: u16,
    pub cp: u16,
    pub wFiller: u16,
    pub cbMax: u32,
    pub grbit: u32,
    pub szBaseTableName: [super::super::System::SystemServices::CHAR; 256],
    pub szBaseColumnName: [super::super::System::SystemServices::CHAR; 256],
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_COLUMNBASE_A {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_COLUMNBASE_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for JET_COLUMNBASE_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_COLUMNBASE_A")
            .field("cbStruct", &self.cbStruct)
            .field("columnid", &self.columnid)
            .field("coltyp", &self.coltyp)
            .field("wCountry", &self.wCountry)
            .field("langid", &self.langid)
            .field("cp", &self.cp)
            .field("wFiller", &self.wFiller)
            .field("cbMax", &self.cbMax)
            .field("grbit", &self.grbit)
            .field("szBaseTableName", &self.szBaseTableName)
            .field("szBaseColumnName", &self.szBaseColumnName)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_COLUMNBASE_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.columnid == other.columnid
            && self.coltyp == other.coltyp
            && self.wCountry == other.wCountry
            && self.langid == other.langid
            && self.cp == other.cp
            && self.wFiller == other.wFiller
            && self.cbMax == other.cbMax
            && self.grbit == other.grbit
            && self.szBaseTableName == other.szBaseTableName
            && self.szBaseColumnName == other.szBaseColumnName
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_COLUMNBASE_A {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_COLUMNBASE_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_COLUMNBASE_W {
    pub cbStruct: u32,
    pub columnid: u32,
    pub coltyp: u32,
    pub wCountry: u16,
    pub langid: u16,
    pub cp: u16,
    pub wFiller: u16,
    pub cbMax: u32,
    pub grbit: u32,
    pub szBaseTableName: [u16; 256],
    pub szBaseColumnName: [u16; 256],
}
impl JET_COLUMNBASE_W {}
impl ::std::default::Default for JET_COLUMNBASE_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_COLUMNBASE_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_COLUMNBASE_W")
            .field("cbStruct", &self.cbStruct)
            .field("columnid", &self.columnid)
            .field("coltyp", &self.coltyp)
            .field("wCountry", &self.wCountry)
            .field("langid", &self.langid)
            .field("cp", &self.cp)
            .field("wFiller", &self.wFiller)
            .field("cbMax", &self.cbMax)
            .field("grbit", &self.grbit)
            .field("szBaseTableName", &self.szBaseTableName)
            .field("szBaseColumnName", &self.szBaseColumnName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_COLUMNBASE_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.columnid == other.columnid
            && self.coltyp == other.coltyp
            && self.wCountry == other.wCountry
            && self.langid == other.langid
            && self.cp == other.cp
            && self.wFiller == other.wFiller
            && self.cbMax == other.cbMax
            && self.grbit == other.grbit
            && self.szBaseTableName == other.szBaseTableName
            && self.szBaseColumnName == other.szBaseColumnName
    }
}
impl ::std::cmp::Eq for JET_COLUMNBASE_W {}
unsafe impl ::windows::runtime::Abi for JET_COLUMNBASE_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_COLUMNCREATE_A {
    pub cbStruct: u32,
    pub szColumnName: super::super::Foundation::PSTR,
    pub coltyp: u32,
    pub cbMax: u32,
    pub grbit: u32,
    pub pvDefault: *mut ::std::ffi::c_void,
    pub cbDefault: u32,
    pub cp: u32,
    pub columnid: u32,
    pub err: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_COLUMNCREATE_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_COLUMNCREATE_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_COLUMNCREATE_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_COLUMNCREATE_A")
            .field("cbStruct", &self.cbStruct)
            .field("szColumnName", &self.szColumnName)
            .field("coltyp", &self.coltyp)
            .field("cbMax", &self.cbMax)
            .field("grbit", &self.grbit)
            .field("pvDefault", &self.pvDefault)
            .field("cbDefault", &self.cbDefault)
            .field("cp", &self.cp)
            .field("columnid", &self.columnid)
            .field("err", &self.err)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_COLUMNCREATE_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.szColumnName == other.szColumnName
            && self.coltyp == other.coltyp
            && self.cbMax == other.cbMax
            && self.grbit == other.grbit
            && self.pvDefault == other.pvDefault
            && self.cbDefault == other.cbDefault
            && self.cp == other.cp
            && self.columnid == other.columnid
            && self.err == other.err
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_COLUMNCREATE_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_COLUMNCREATE_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_COLUMNCREATE_W {
    pub cbStruct: u32,
    pub szColumnName: super::super::Foundation::PWSTR,
    pub coltyp: u32,
    pub cbMax: u32,
    pub grbit: u32,
    pub pvDefault: *mut ::std::ffi::c_void,
    pub cbDefault: u32,
    pub cp: u32,
    pub columnid: u32,
    pub err: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_COLUMNCREATE_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_COLUMNCREATE_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_COLUMNCREATE_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_COLUMNCREATE_W")
            .field("cbStruct", &self.cbStruct)
            .field("szColumnName", &self.szColumnName)
            .field("coltyp", &self.coltyp)
            .field("cbMax", &self.cbMax)
            .field("grbit", &self.grbit)
            .field("pvDefault", &self.pvDefault)
            .field("cbDefault", &self.cbDefault)
            .field("cp", &self.cp)
            .field("columnid", &self.columnid)
            .field("err", &self.err)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_COLUMNCREATE_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.szColumnName == other.szColumnName
            && self.coltyp == other.coltyp
            && self.cbMax == other.cbMax
            && self.grbit == other.grbit
            && self.pvDefault == other.pvDefault
            && self.cbDefault == other.cbDefault
            && self.cp == other.cp
            && self.columnid == other.columnid
            && self.err == other.err
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_COLUMNCREATE_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_COLUMNCREATE_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_COLUMNDEF {
    pub cbStruct: u32,
    pub columnid: u32,
    pub coltyp: u32,
    pub wCountry: u16,
    pub langid: u16,
    pub cp: u16,
    pub wCollate: u16,
    pub cbMax: u32,
    pub grbit: u32,
}
impl JET_COLUMNDEF {}
impl ::std::default::Default for JET_COLUMNDEF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_COLUMNDEF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_COLUMNDEF")
            .field("cbStruct", &self.cbStruct)
            .field("columnid", &self.columnid)
            .field("coltyp", &self.coltyp)
            .field("wCountry", &self.wCountry)
            .field("langid", &self.langid)
            .field("cp", &self.cp)
            .field("wCollate", &self.wCollate)
            .field("cbMax", &self.cbMax)
            .field("grbit", &self.grbit)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_COLUMNDEF {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.columnid == other.columnid
            && self.coltyp == other.coltyp
            && self.wCountry == other.wCountry
            && self.langid == other.langid
            && self.cp == other.cp
            && self.wCollate == other.wCollate
            && self.cbMax == other.cbMax
            && self.grbit == other.grbit
    }
}
impl ::std::cmp::Eq for JET_COLUMNDEF {}
unsafe impl ::windows::runtime::Abi for JET_COLUMNDEF {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_COLUMNLIST {
    pub cbStruct: u32,
    pub tableid: JET_TABLEID,
    pub cRecord: u32,
    pub columnidPresentationOrder: u32,
    pub columnidcolumnname: u32,
    pub columnidcolumnid: u32,
    pub columnidcoltyp: u32,
    pub columnidCountry: u32,
    pub columnidLangid: u32,
    pub columnidCp: u32,
    pub columnidCollate: u32,
    pub columnidcbMax: u32,
    pub columnidgrbit: u32,
    pub columnidDefault: u32,
    pub columnidBaseTableName: u32,
    pub columnidBaseColumnName: u32,
    pub columnidDefinitionName: u32,
}
impl JET_COLUMNLIST {}
impl ::std::default::Default for JET_COLUMNLIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_COLUMNLIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_COLUMNLIST")
            .field("cbStruct", &self.cbStruct)
            .field("tableid", &self.tableid)
            .field("cRecord", &self.cRecord)
            .field("columnidPresentationOrder", &self.columnidPresentationOrder)
            .field("columnidcolumnname", &self.columnidcolumnname)
            .field("columnidcolumnid", &self.columnidcolumnid)
            .field("columnidcoltyp", &self.columnidcoltyp)
            .field("columnidCountry", &self.columnidCountry)
            .field("columnidLangid", &self.columnidLangid)
            .field("columnidCp", &self.columnidCp)
            .field("columnidCollate", &self.columnidCollate)
            .field("columnidcbMax", &self.columnidcbMax)
            .field("columnidgrbit", &self.columnidgrbit)
            .field("columnidDefault", &self.columnidDefault)
            .field("columnidBaseTableName", &self.columnidBaseTableName)
            .field("columnidBaseColumnName", &self.columnidBaseColumnName)
            .field("columnidDefinitionName", &self.columnidDefinitionName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_COLUMNLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.tableid == other.tableid
            && self.cRecord == other.cRecord
            && self.columnidPresentationOrder == other.columnidPresentationOrder
            && self.columnidcolumnname == other.columnidcolumnname
            && self.columnidcolumnid == other.columnidcolumnid
            && self.columnidcoltyp == other.columnidcoltyp
            && self.columnidCountry == other.columnidCountry
            && self.columnidLangid == other.columnidLangid
            && self.columnidCp == other.columnidCp
            && self.columnidCollate == other.columnidCollate
            && self.columnidcbMax == other.columnidcbMax
            && self.columnidgrbit == other.columnidgrbit
            && self.columnidDefault == other.columnidDefault
            && self.columnidBaseTableName == other.columnidBaseTableName
            && self.columnidBaseColumnName == other.columnidBaseColumnName
            && self.columnidDefinitionName == other.columnidDefinitionName
    }
}
impl ::std::cmp::Eq for JET_COLUMNLIST {}
unsafe impl ::windows::runtime::Abi for JET_COLUMNLIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_COMMIT_ID {
    pub signLog: JET_SIGNATURE,
    pub reserved: i32,
    pub commitId: i64,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_COMMIT_ID {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_COMMIT_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_COMMIT_ID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_COMMIT_ID {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_COMMIT_ID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_CONDITIONALCOLUMN_A {
    pub cbStruct: u32,
    pub szColumnName: super::super::Foundation::PSTR,
    pub grbit: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_CONDITIONALCOLUMN_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_CONDITIONALCOLUMN_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_CONDITIONALCOLUMN_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_CONDITIONALCOLUMN_A")
            .field("cbStruct", &self.cbStruct)
            .field("szColumnName", &self.szColumnName)
            .field("grbit", &self.grbit)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_CONDITIONALCOLUMN_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.szColumnName == other.szColumnName
            && self.grbit == other.grbit
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_CONDITIONALCOLUMN_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_CONDITIONALCOLUMN_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_CONDITIONALCOLUMN_W {
    pub cbStruct: u32,
    pub szColumnName: super::super::Foundation::PWSTR,
    pub grbit: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_CONDITIONALCOLUMN_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_CONDITIONALCOLUMN_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_CONDITIONALCOLUMN_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_CONDITIONALCOLUMN_W")
            .field("cbStruct", &self.cbStruct)
            .field("szColumnName", &self.szColumnName)
            .field("grbit", &self.grbit)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_CONDITIONALCOLUMN_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.szColumnName == other.szColumnName
            && self.grbit == other.grbit
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_CONDITIONALCOLUMN_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_CONDITIONALCOLUMN_W {
    type Abi = Self;
    type DefaultType = Self;
}
pub const JET_ColInfoGrbitMinimalInfo: u32 = 1073741824u32;
pub const JET_ColInfoGrbitNonDerivedColumnsOnly: u32 = 2147483648u32;
pub const JET_ColInfoGrbitSortByColumnid: u32 = 536870912u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_DBINFOMISC {
    pub ulVersion: u32,
    pub ulUpdate: u32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: u32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: u32,
    pub fUpgradeDb: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub lSPNumber: i32,
    pub cbPageSize: u32,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_DBINFOMISC {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_DBINFOMISC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_DBINFOMISC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_DBINFOMISC {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_DBINFOMISC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_DBINFOMISC2 {
    pub ulVersion: u32,
    pub ulUpdate: u32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: u32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: u32,
    pub fUpgradeDb: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub lSPNumber: i32,
    pub cbPageSize: u32,
    pub genMinRequired: u32,
    pub genMaxRequired: u32,
    pub logtimeGenMaxCreate: JET_LOGTIME,
    pub ulRepairCount: u32,
    pub logtimeRepair: JET_LOGTIME,
    pub ulRepairCountOld: u32,
    pub ulECCFixSuccess: u32,
    pub logtimeECCFixSuccess: JET_LOGTIME,
    pub ulECCFixSuccessOld: u32,
    pub ulECCFixFail: u32,
    pub logtimeECCFixFail: JET_LOGTIME,
    pub ulECCFixFailOld: u32,
    pub ulBadChecksum: u32,
    pub logtimeBadChecksum: JET_LOGTIME,
    pub ulBadChecksumOld: u32,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_DBINFOMISC2 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_DBINFOMISC2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_DBINFOMISC2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_DBINFOMISC2 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_DBINFOMISC2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_DBINFOMISC3 {
    pub ulVersion: u32,
    pub ulUpdate: u32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: u32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: u32,
    pub fUpgradeDb: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub lSPNumber: i32,
    pub cbPageSize: u32,
    pub genMinRequired: u32,
    pub genMaxRequired: u32,
    pub logtimeGenMaxCreate: JET_LOGTIME,
    pub ulRepairCount: u32,
    pub logtimeRepair: JET_LOGTIME,
    pub ulRepairCountOld: u32,
    pub ulECCFixSuccess: u32,
    pub logtimeECCFixSuccess: JET_LOGTIME,
    pub ulECCFixSuccessOld: u32,
    pub ulECCFixFail: u32,
    pub logtimeECCFixFail: JET_LOGTIME,
    pub ulECCFixFailOld: u32,
    pub ulBadChecksum: u32,
    pub logtimeBadChecksum: JET_LOGTIME,
    pub ulBadChecksumOld: u32,
    pub genCommitted: u32,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_DBINFOMISC3 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_DBINFOMISC3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_DBINFOMISC3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_DBINFOMISC3 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_DBINFOMISC3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_DBINFOMISC4 {
    pub ulVersion: u32,
    pub ulUpdate: u32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: u32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: u32,
    pub fUpgradeDb: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub lSPNumber: i32,
    pub cbPageSize: u32,
    pub genMinRequired: u32,
    pub genMaxRequired: u32,
    pub logtimeGenMaxCreate: JET_LOGTIME,
    pub ulRepairCount: u32,
    pub logtimeRepair: JET_LOGTIME,
    pub ulRepairCountOld: u32,
    pub ulECCFixSuccess: u32,
    pub logtimeECCFixSuccess: JET_LOGTIME,
    pub ulECCFixSuccessOld: u32,
    pub ulECCFixFail: u32,
    pub logtimeECCFixFail: JET_LOGTIME,
    pub ulECCFixFailOld: u32,
    pub ulBadChecksum: u32,
    pub logtimeBadChecksum: JET_LOGTIME,
    pub ulBadChecksumOld: u32,
    pub genCommitted: u32,
    pub bkinfoCopyPrev: JET_BKINFO,
    pub bkinfoDiffPrev: JET_BKINFO,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_DBINFOMISC4 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_DBINFOMISC4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_DBINFOMISC4 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_DBINFOMISC4 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_DBINFOMISC4 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_DBINFOUPGRADE {
    pub cbStruct: u32,
    pub cbFilesizeLow: u32,
    pub cbFilesizeHigh: u32,
    pub cbFreeSpaceRequiredLow: u32,
    pub cbFreeSpaceRequiredHigh: u32,
    pub csecToUpgrade: u32,
    pub Anonymous: JET_DBINFOUPGRADE_0,
}
impl JET_DBINFOUPGRADE {}
impl ::std::default::Default for JET_DBINFOUPGRADE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_DBINFOUPGRADE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_DBINFOUPGRADE {}
unsafe impl ::windows::runtime::Abi for JET_DBINFOUPGRADE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union JET_DBINFOUPGRADE_0 {
    pub ulFlags: u32,
    pub Anonymous: JET_DBINFOUPGRADE_0_0,
}
impl JET_DBINFOUPGRADE_0 {}
impl ::std::default::Default for JET_DBINFOUPGRADE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_DBINFOUPGRADE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_DBINFOUPGRADE_0 {}
unsafe impl ::windows::runtime::Abi for JET_DBINFOUPGRADE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_DBINFOUPGRADE_0_0 {
    pub _bitfield: u32,
}
impl JET_DBINFOUPGRADE_0_0 {}
impl ::std::default::Default for JET_DBINFOUPGRADE_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_DBINFOUPGRADE_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_DBINFOUPGRADE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for JET_DBINFOUPGRADE_0_0 {}
unsafe impl ::windows::runtime::Abi for JET_DBINFOUPGRADE_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const JET_DbInfoCollate: u32 = 5u32;
pub const JET_DbInfoConnect: u32 = 1u32;
pub const JET_DbInfoCountry: u32 = 2u32;
pub const JET_DbInfoCp: u32 = 4u32;
pub const JET_DbInfoDBInUse: u32 = 15u32;
pub const JET_DbInfoFileType: u32 = 19u32;
pub const JET_DbInfoFilename: u32 = 0u32;
pub const JET_DbInfoFilesize: u32 = 10u32;
pub const JET_DbInfoFilesizeOnDisk: u32 = 21u32;
pub const JET_DbInfoIsam: u32 = 9u32;
pub const JET_DbInfoLCID: u32 = 3u32;
pub const JET_DbInfoLangid: u32 = 3u32;
pub const JET_DbInfoMisc: u32 = 14u32;
pub const JET_DbInfoOptions: u32 = 6u32;
pub const JET_DbInfoPageSize: u32 = 17u32;
pub const JET_DbInfoSpaceAvailable: u32 = 12u32;
pub const JET_DbInfoSpaceOwned: u32 = 11u32;
pub const JET_DbInfoTransactions: u32 = 7u32;
pub const JET_DbInfoUpgrade: u32 = 13u32;
pub const JET_DbInfoVersion: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_ENUMCOLUMN {
    pub columnid: u32,
    pub err: i32,
    pub Anonymous: JET_ENUMCOLUMN_0,
}
impl JET_ENUMCOLUMN {}
impl ::std::default::Default for JET_ENUMCOLUMN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_ENUMCOLUMN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_ENUMCOLUMN {}
unsafe impl ::windows::runtime::Abi for JET_ENUMCOLUMN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union JET_ENUMCOLUMN_0 {
    pub Anonymous1: JET_ENUMCOLUMN_0_0,
    pub Anonymous2: JET_ENUMCOLUMN_0_1,
}
impl JET_ENUMCOLUMN_0 {}
impl ::std::default::Default for JET_ENUMCOLUMN_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_ENUMCOLUMN_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_ENUMCOLUMN_0 {}
unsafe impl ::windows::runtime::Abi for JET_ENUMCOLUMN_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_ENUMCOLUMN_0_0 {
    pub cEnumColumnValue: u32,
    pub rgEnumColumnValue: *mut JET_ENUMCOLUMNVALUE,
}
impl JET_ENUMCOLUMN_0_0 {}
impl ::std::default::Default for JET_ENUMCOLUMN_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_ENUMCOLUMN_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct")
            .field("cEnumColumnValue", &self.cEnumColumnValue)
            .field("rgEnumColumnValue", &self.rgEnumColumnValue)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_ENUMCOLUMN_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.cEnumColumnValue == other.cEnumColumnValue
            && self.rgEnumColumnValue == other.rgEnumColumnValue
    }
}
impl ::std::cmp::Eq for JET_ENUMCOLUMN_0_0 {}
unsafe impl ::windows::runtime::Abi for JET_ENUMCOLUMN_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_ENUMCOLUMN_0_1 {
    pub cbData: u32,
    pub pvData: *mut ::std::ffi::c_void,
}
impl JET_ENUMCOLUMN_0_1 {}
impl ::std::default::Default for JET_ENUMCOLUMN_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_ENUMCOLUMN_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct")
            .field("cbData", &self.cbData)
            .field("pvData", &self.pvData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_ENUMCOLUMN_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pvData == other.pvData
    }
}
impl ::std::cmp::Eq for JET_ENUMCOLUMN_0_1 {}
unsafe impl ::windows::runtime::Abi for JET_ENUMCOLUMN_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_ENUMCOLUMNID {
    pub columnid: u32,
    pub ctagSequence: u32,
    pub rgtagSequence: *mut u32,
}
impl JET_ENUMCOLUMNID {}
impl ::std::default::Default for JET_ENUMCOLUMNID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_ENUMCOLUMNID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_ENUMCOLUMNID")
            .field("columnid", &self.columnid)
            .field("ctagSequence", &self.ctagSequence)
            .field("rgtagSequence", &self.rgtagSequence)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_ENUMCOLUMNID {
    fn eq(&self, other: &Self) -> bool {
        self.columnid == other.columnid
            && self.ctagSequence == other.ctagSequence
            && self.rgtagSequence == other.rgtagSequence
    }
}
impl ::std::cmp::Eq for JET_ENUMCOLUMNID {}
unsafe impl ::windows::runtime::Abi for JET_ENUMCOLUMNID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_ENUMCOLUMNVALUE {
    pub itagSequence: u32,
    pub err: i32,
    pub cbData: u32,
    pub pvData: *mut ::std::ffi::c_void,
}
impl JET_ENUMCOLUMNVALUE {}
impl ::std::default::Default for JET_ENUMCOLUMNVALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_ENUMCOLUMNVALUE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_ENUMCOLUMNVALUE")
            .field("itagSequence", &self.itagSequence)
            .field("err", &self.err)
            .field("cbData", &self.cbData)
            .field("pvData", &self.pvData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_ENUMCOLUMNVALUE {
    fn eq(&self, other: &Self) -> bool {
        self.itagSequence == other.itagSequence
            && self.err == other.err
            && self.cbData == other.cbData
            && self.pvData == other.pvData
    }
}
impl ::std::cmp::Eq for JET_ENUMCOLUMNVALUE {}
unsafe impl ::windows::runtime::Abi for JET_ENUMCOLUMNVALUE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct JET_ERRCAT(pub i32);
pub const JET_errcatUnknown: JET_ERRCAT = JET_ERRCAT(0i32);
pub const JET_errcatError: JET_ERRCAT = JET_ERRCAT(1i32);
pub const JET_errcatOperation: JET_ERRCAT = JET_ERRCAT(2i32);
pub const JET_errcatFatal: JET_ERRCAT = JET_ERRCAT(3i32);
pub const JET_errcatIO: JET_ERRCAT = JET_ERRCAT(4i32);
pub const JET_errcatResource: JET_ERRCAT = JET_ERRCAT(5i32);
pub const JET_errcatMemory: JET_ERRCAT = JET_ERRCAT(6i32);
pub const JET_errcatQuota: JET_ERRCAT = JET_ERRCAT(7i32);
pub const JET_errcatDisk: JET_ERRCAT = JET_ERRCAT(8i32);
pub const JET_errcatData: JET_ERRCAT = JET_ERRCAT(9i32);
pub const JET_errcatCorruption: JET_ERRCAT = JET_ERRCAT(10i32);
pub const JET_errcatInconsistent: JET_ERRCAT = JET_ERRCAT(11i32);
pub const JET_errcatFragmentation: JET_ERRCAT = JET_ERRCAT(12i32);
pub const JET_errcatApi: JET_ERRCAT = JET_ERRCAT(13i32);
pub const JET_errcatUsage: JET_ERRCAT = JET_ERRCAT(14i32);
pub const JET_errcatState: JET_ERRCAT = JET_ERRCAT(15i32);
pub const JET_errcatObsolete: JET_ERRCAT = JET_ERRCAT(16i32);
pub const JET_errcatMax: JET_ERRCAT = JET_ERRCAT(17i32);
impl ::std::convert::From<i32> for JET_ERRCAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JET_ERRCAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_ERRINFOBASIC_W {
    pub cbStruct: u32,
    pub errValue: i32,
    pub errcatMostSpecific: JET_ERRCAT,
    pub rgCategoricalHierarchy: [u8; 8],
    pub lSourceLine: u32,
    pub rgszSourceFile: [u16; 64],
}
impl JET_ERRINFOBASIC_W {}
impl ::std::default::Default for JET_ERRINFOBASIC_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_ERRINFOBASIC_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_ERRINFOBASIC_W")
            .field("cbStruct", &self.cbStruct)
            .field("errValue", &self.errValue)
            .field("errcatMostSpecific", &self.errcatMostSpecific)
            .field("rgCategoricalHierarchy", &self.rgCategoricalHierarchy)
            .field("lSourceLine", &self.lSourceLine)
            .field("rgszSourceFile", &self.rgszSourceFile)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_ERRINFOBASIC_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.errValue == other.errValue
            && self.errcatMostSpecific == other.errcatMostSpecific
            && self.rgCategoricalHierarchy == other.rgCategoricalHierarchy
            && self.lSourceLine == other.lSourceLine
            && self.rgszSourceFile == other.rgszSourceFile
    }
}
impl ::std::cmp::Eq for JET_ERRINFOBASIC_W {}
unsafe impl ::windows::runtime::Abi for JET_ERRINFOBASIC_W {
    type Abi = Self;
    type DefaultType = Self;
}
pub const JET_EventLoggingDisable: u32 = 0u32;
pub const JET_EventLoggingLevelHigh: u32 = 75u32;
pub const JET_EventLoggingLevelLow: u32 = 25u32;
pub const JET_EventLoggingLevelMax: u32 = 100u32;
pub const JET_EventLoggingLevelMedium: u32 = 50u32;
pub const JET_EventLoggingLevelMin: u32 = 1u32;
pub const JET_ExceptionFailFast: u32 = 4u32;
pub const JET_ExceptionMsgBox: u32 = 1u32;
pub const JET_ExceptionNone: u32 = 2u32;
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct JET_HANDLE(pub usize);
impl ::std::default::Default for JET_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for JET_HANDLE {}
unsafe impl ::windows::runtime::Abi for JET_HANDLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct JET_INDEXCHECKING(pub i32);
pub const JET_IndexCheckingOff: JET_INDEXCHECKING = JET_INDEXCHECKING(0i32);
pub const JET_IndexCheckingOn: JET_INDEXCHECKING = JET_INDEXCHECKING(1i32);
pub const JET_IndexCheckingDeferToOpenTable: JET_INDEXCHECKING = JET_INDEXCHECKING(2i32);
pub const JET_IndexCheckingMax: JET_INDEXCHECKING = JET_INDEXCHECKING(3i32);
impl ::std::convert::From<i32> for JET_INDEXCHECKING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JET_INDEXCHECKING {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_INDEXCREATE2_A {
    pub cbStruct: u32,
    pub szIndexName: super::super::Foundation::PSTR,
    pub szKey: super::super::Foundation::PSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub Anonymous1: JET_INDEXCREATE2_A_0,
    pub Anonymous2: JET_INDEXCREATE2_A_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_A,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_INDEXCREATE2_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_INDEXCREATE2_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_INDEXCREATE2_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_INDEXCREATE2_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE2_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union JET_INDEXCREATE2_A_0 {
    pub lcid: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl JET_INDEXCREATE2_A_0 {}
impl ::std::default::Default for JET_INDEXCREATE2_A_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_INDEXCREATE2_A_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_INDEXCREATE2_A_0 {}
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE2_A_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union JET_INDEXCREATE2_A_1 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl JET_INDEXCREATE2_A_1 {}
impl ::std::default::Default for JET_INDEXCREATE2_A_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_INDEXCREATE2_A_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_INDEXCREATE2_A_1 {}
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE2_A_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_INDEXCREATE2_W {
    pub cbStruct: u32,
    pub szIndexName: super::super::Foundation::PWSTR,
    pub szKey: super::super::Foundation::PWSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub Anonymous1: JET_INDEXCREATE2_W_0,
    pub Anonymous2: JET_INDEXCREATE2_W_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_INDEXCREATE2_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_INDEXCREATE2_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_INDEXCREATE2_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_INDEXCREATE2_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE2_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union JET_INDEXCREATE2_W_0 {
    pub lcid: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl JET_INDEXCREATE2_W_0 {}
impl ::std::default::Default for JET_INDEXCREATE2_W_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_INDEXCREATE2_W_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_INDEXCREATE2_W_0 {}
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE2_W_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union JET_INDEXCREATE2_W_1 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl JET_INDEXCREATE2_W_1 {}
impl ::std::default::Default for JET_INDEXCREATE2_W_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_INDEXCREATE2_W_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_INDEXCREATE2_W_1 {}
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE2_W_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_INDEXCREATE3_A {
    pub cbStruct: u32,
    pub szIndexName: super::super::Foundation::PSTR,
    pub szKey: super::super::Foundation::PSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX2,
    pub Anonymous: JET_INDEXCREATE3_A_0,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_A,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_INDEXCREATE3_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_INDEXCREATE3_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_INDEXCREATE3_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_INDEXCREATE3_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE3_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union JET_INDEXCREATE3_A_0 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl JET_INDEXCREATE3_A_0 {}
impl ::std::default::Default for JET_INDEXCREATE3_A_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_INDEXCREATE3_A_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_INDEXCREATE3_A_0 {}
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE3_A_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_INDEXCREATE3_W {
    pub cbStruct: u32,
    pub szIndexName: super::super::Foundation::PWSTR,
    pub szKey: super::super::Foundation::PWSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX2,
    pub Anonymous: JET_INDEXCREATE3_W_0,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_INDEXCREATE3_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_INDEXCREATE3_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_INDEXCREATE3_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_INDEXCREATE3_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE3_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union JET_INDEXCREATE3_W_0 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl JET_INDEXCREATE3_W_0 {}
impl ::std::default::Default for JET_INDEXCREATE3_W_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_INDEXCREATE3_W_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_INDEXCREATE3_W_0 {}
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE3_W_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_INDEXCREATE_A {
    pub cbStruct: u32,
    pub szIndexName: super::super::Foundation::PSTR,
    pub szKey: super::super::Foundation::PSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub Anonymous1: JET_INDEXCREATE_A_0,
    pub Anonymous2: JET_INDEXCREATE_A_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_A,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_INDEXCREATE_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_INDEXCREATE_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_INDEXCREATE_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_INDEXCREATE_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union JET_INDEXCREATE_A_0 {
    pub lcid: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl JET_INDEXCREATE_A_0 {}
impl ::std::default::Default for JET_INDEXCREATE_A_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_INDEXCREATE_A_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_INDEXCREATE_A_0 {}
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE_A_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union JET_INDEXCREATE_A_1 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl JET_INDEXCREATE_A_1 {}
impl ::std::default::Default for JET_INDEXCREATE_A_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_INDEXCREATE_A_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_INDEXCREATE_A_1 {}
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE_A_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_INDEXCREATE_W {
    pub cbStruct: u32,
    pub szIndexName: super::super::Foundation::PWSTR,
    pub szKey: super::super::Foundation::PWSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub Anonymous1: JET_INDEXCREATE_W_0,
    pub Anonymous2: JET_INDEXCREATE_W_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_INDEXCREATE_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_INDEXCREATE_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_INDEXCREATE_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_INDEXCREATE_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union JET_INDEXCREATE_W_0 {
    pub lcid: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl JET_INDEXCREATE_W_0 {}
impl ::std::default::Default for JET_INDEXCREATE_W_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_INDEXCREATE_W_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_INDEXCREATE_W_0 {}
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE_W_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union JET_INDEXCREATE_W_1 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl JET_INDEXCREATE_W_1 {}
impl ::std::default::Default for JET_INDEXCREATE_W_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_INDEXCREATE_W_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_INDEXCREATE_W_1 {}
unsafe impl ::windows::runtime::Abi for JET_INDEXCREATE_W_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_INDEXID {
    pub cbStruct: u32,
    pub rgbIndexId: [u8; 16],
}
impl JET_INDEXID {}
impl ::std::default::Default for JET_INDEXID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_INDEXID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_INDEXID")
            .field("cbStruct", &self.cbStruct)
            .field("rgbIndexId", &self.rgbIndexId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_INDEXID {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.rgbIndexId == other.rgbIndexId
    }
}
impl ::std::cmp::Eq for JET_INDEXID {}
unsafe impl ::windows::runtime::Abi for JET_INDEXID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_INDEXLIST {
    pub cbStruct: u32,
    pub tableid: JET_TABLEID,
    pub cRecord: u32,
    pub columnidindexname: u32,
    pub columnidgrbitIndex: u32,
    pub columnidcKey: u32,
    pub columnidcEntry: u32,
    pub columnidcPage: u32,
    pub columnidcColumn: u32,
    pub columnidiColumn: u32,
    pub columnidcolumnid: u32,
    pub columnidcoltyp: u32,
    pub columnidCountry: u32,
    pub columnidLangid: u32,
    pub columnidCp: u32,
    pub columnidCollate: u32,
    pub columnidgrbitColumn: u32,
    pub columnidcolumnname: u32,
    pub columnidLCMapFlags: u32,
}
impl JET_INDEXLIST {}
impl ::std::default::Default for JET_INDEXLIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_INDEXLIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_INDEXLIST")
            .field("cbStruct", &self.cbStruct)
            .field("tableid", &self.tableid)
            .field("cRecord", &self.cRecord)
            .field("columnidindexname", &self.columnidindexname)
            .field("columnidgrbitIndex", &self.columnidgrbitIndex)
            .field("columnidcKey", &self.columnidcKey)
            .field("columnidcEntry", &self.columnidcEntry)
            .field("columnidcPage", &self.columnidcPage)
            .field("columnidcColumn", &self.columnidcColumn)
            .field("columnidiColumn", &self.columnidiColumn)
            .field("columnidcolumnid", &self.columnidcolumnid)
            .field("columnidcoltyp", &self.columnidcoltyp)
            .field("columnidCountry", &self.columnidCountry)
            .field("columnidLangid", &self.columnidLangid)
            .field("columnidCp", &self.columnidCp)
            .field("columnidCollate", &self.columnidCollate)
            .field("columnidgrbitColumn", &self.columnidgrbitColumn)
            .field("columnidcolumnname", &self.columnidcolumnname)
            .field("columnidLCMapFlags", &self.columnidLCMapFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_INDEXLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.tableid == other.tableid
            && self.cRecord == other.cRecord
            && self.columnidindexname == other.columnidindexname
            && self.columnidgrbitIndex == other.columnidgrbitIndex
            && self.columnidcKey == other.columnidcKey
            && self.columnidcEntry == other.columnidcEntry
            && self.columnidcPage == other.columnidcPage
            && self.columnidcColumn == other.columnidcColumn
            && self.columnidiColumn == other.columnidiColumn
            && self.columnidcolumnid == other.columnidcolumnid
            && self.columnidcoltyp == other.columnidcoltyp
            && self.columnidCountry == other.columnidCountry
            && self.columnidLangid == other.columnidLangid
            && self.columnidCp == other.columnidCp
            && self.columnidCollate == other.columnidCollate
            && self.columnidgrbitColumn == other.columnidgrbitColumn
            && self.columnidcolumnname == other.columnidcolumnname
            && self.columnidLCMapFlags == other.columnidLCMapFlags
    }
}
impl ::std::cmp::Eq for JET_INDEXLIST {}
unsafe impl ::windows::runtime::Abi for JET_INDEXLIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_INDEXRANGE {
    pub cbStruct: u32,
    pub tableid: JET_TABLEID,
    pub grbit: u32,
}
impl JET_INDEXRANGE {}
impl ::std::default::Default for JET_INDEXRANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_INDEXRANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_INDEXRANGE")
            .field("cbStruct", &self.cbStruct)
            .field("tableid", &self.tableid)
            .field("grbit", &self.grbit)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_INDEXRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.tableid == other.tableid
            && self.grbit == other.grbit
    }
}
impl ::std::cmp::Eq for JET_INDEXRANGE {}
unsafe impl ::windows::runtime::Abi for JET_INDEXRANGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_INDEX_COLUMN {
    pub columnid: u32,
    pub relop: JET_RELOP,
    pub pv: *mut ::std::ffi::c_void,
    pub cb: u32,
    pub grbit: u32,
}
impl JET_INDEX_COLUMN {}
impl ::std::default::Default for JET_INDEX_COLUMN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_INDEX_COLUMN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_INDEX_COLUMN")
            .field("columnid", &self.columnid)
            .field("relop", &self.relop)
            .field("pv", &self.pv)
            .field("cb", &self.cb)
            .field("grbit", &self.grbit)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_INDEX_COLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.columnid == other.columnid
            && self.relop == other.relop
            && self.pv == other.pv
            && self.cb == other.cb
            && self.grbit == other.grbit
    }
}
impl ::std::cmp::Eq for JET_INDEX_COLUMN {}
unsafe impl ::windows::runtime::Abi for JET_INDEX_COLUMN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_INDEX_RANGE {
    pub rgStartColumns: *mut JET_INDEX_COLUMN,
    pub cStartColumns: u32,
    pub rgEndColumns: *mut JET_INDEX_COLUMN,
    pub cEndColumns: u32,
}
impl JET_INDEX_RANGE {}
impl ::std::default::Default for JET_INDEX_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_INDEX_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_INDEX_RANGE")
            .field("rgStartColumns", &self.rgStartColumns)
            .field("cStartColumns", &self.cStartColumns)
            .field("rgEndColumns", &self.rgEndColumns)
            .field("cEndColumns", &self.cEndColumns)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_INDEX_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.rgStartColumns == other.rgStartColumns
            && self.cStartColumns == other.cStartColumns
            && self.rgEndColumns == other.rgEndColumns
            && self.cEndColumns == other.cEndColumns
    }
}
impl ::std::cmp::Eq for JET_INDEX_RANGE {}
unsafe impl ::windows::runtime::Abi for JET_INDEX_RANGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct JET_INSTANCE(pub usize);
impl ::std::default::Default for JET_INSTANCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for JET_INSTANCE {}
unsafe impl ::windows::runtime::Abi for JET_INSTANCE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_INSTANCE_INFO_A {
    pub hInstanceId: JET_INSTANCE,
    pub szInstanceName: super::super::Foundation::PSTR,
    pub cDatabases: JET_API_PTR,
    pub szDatabaseFileName: *mut *mut i8,
    pub szDatabaseDisplayName: *mut *mut i8,
    pub szDatabaseSLVFileName_Obsolete: *mut *mut i8,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_INSTANCE_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_INSTANCE_INFO_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_INSTANCE_INFO_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_INSTANCE_INFO_A")
            .field("hInstanceId", &self.hInstanceId)
            .field("szInstanceName", &self.szInstanceName)
            .field("cDatabases", &self.cDatabases)
            .field("szDatabaseFileName", &self.szDatabaseFileName)
            .field("szDatabaseDisplayName", &self.szDatabaseDisplayName)
            .field(
                "szDatabaseSLVFileName_Obsolete",
                &self.szDatabaseSLVFileName_Obsolete,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_INSTANCE_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        self.hInstanceId == other.hInstanceId
            && self.szInstanceName == other.szInstanceName
            && self.cDatabases == other.cDatabases
            && self.szDatabaseFileName == other.szDatabaseFileName
            && self.szDatabaseDisplayName == other.szDatabaseDisplayName
            && self.szDatabaseSLVFileName_Obsolete == other.szDatabaseSLVFileName_Obsolete
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_INSTANCE_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_INSTANCE_INFO_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_INSTANCE_INFO_W {
    pub hInstanceId: JET_INSTANCE,
    pub szInstanceName: super::super::Foundation::PWSTR,
    pub cDatabases: JET_API_PTR,
    pub szDatabaseFileName: *mut *mut u16,
    pub szDatabaseDisplayName: *mut *mut u16,
    pub szDatabaseSLVFileName_Obsolete: *mut *mut u16,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_INSTANCE_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_INSTANCE_INFO_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_INSTANCE_INFO_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_INSTANCE_INFO_W")
            .field("hInstanceId", &self.hInstanceId)
            .field("szInstanceName", &self.szInstanceName)
            .field("cDatabases", &self.cDatabases)
            .field("szDatabaseFileName", &self.szDatabaseFileName)
            .field("szDatabaseDisplayName", &self.szDatabaseDisplayName)
            .field(
                "szDatabaseSLVFileName_Obsolete",
                &self.szDatabaseSLVFileName_Obsolete,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_INSTANCE_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        self.hInstanceId == other.hInstanceId
            && self.szInstanceName == other.szInstanceName
            && self.cDatabases == other.cDatabases
            && self.szDatabaseFileName == other.szDatabaseFileName
            && self.szDatabaseDisplayName == other.szDatabaseDisplayName
            && self.szDatabaseSLVFileName_Obsolete == other.szDatabaseSLVFileName_Obsolete
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_INSTANCE_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_INSTANCE_INFO_W {
    type Abi = Self;
    type DefaultType = Self;
}
pub const JET_IOPriorityLow: u32 = 1u32;
pub const JET_IOPriorityNormal: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct JET_LGPOS {
    pub ib: u16,
    pub isec: u16,
    pub lGeneration: i32,
}
impl JET_LGPOS {}
impl ::std::default::Default for JET_LGPOS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JET_LGPOS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JET_LGPOS {}
unsafe impl ::windows::runtime::Abi for JET_LGPOS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_LOGINFO_A {
    pub cbSize: u32,
    pub ulGenLow: u32,
    pub ulGenHigh: u32,
    pub szBaseName: [super::super::System::SystemServices::CHAR; 4],
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_LOGINFO_A {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_LOGINFO_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for JET_LOGINFO_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_LOGINFO_A")
            .field("cbSize", &self.cbSize)
            .field("ulGenLow", &self.ulGenLow)
            .field("ulGenHigh", &self.ulGenHigh)
            .field("szBaseName", &self.szBaseName)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_LOGINFO_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.ulGenLow == other.ulGenLow
            && self.ulGenHigh == other.ulGenHigh
            && self.szBaseName == other.szBaseName
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_LOGINFO_A {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_LOGINFO_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_LOGINFO_W {
    pub cbSize: u32,
    pub ulGenLow: u32,
    pub ulGenHigh: u32,
    pub szBaseName: [u16; 4],
}
impl JET_LOGINFO_W {}
impl ::std::default::Default for JET_LOGINFO_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_LOGINFO_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_LOGINFO_W")
            .field("cbSize", &self.cbSize)
            .field("ulGenLow", &self.ulGenLow)
            .field("ulGenHigh", &self.ulGenHigh)
            .field("szBaseName", &self.szBaseName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_LOGINFO_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.ulGenLow == other.ulGenLow
            && self.ulGenHigh == other.ulGenHigh
            && self.szBaseName == other.szBaseName
    }
}
impl ::std::cmp::Eq for JET_LOGINFO_W {}
unsafe impl ::windows::runtime::Abi for JET_LOGINFO_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_LOGTIME {
    pub bSeconds: super::super::System::SystemServices::CHAR,
    pub bMinutes: super::super::System::SystemServices::CHAR,
    pub bHours: super::super::System::SystemServices::CHAR,
    pub bDay: super::super::System::SystemServices::CHAR,
    pub bMonth: super::super::System::SystemServices::CHAR,
    pub bYear: super::super::System::SystemServices::CHAR,
    pub Anonymous1: JET_LOGTIME_0,
    pub Anonymous2: JET_LOGTIME_1,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_LOGTIME {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_LOGTIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_LOGTIME {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_LOGTIME {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_LOGTIME {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub union JET_LOGTIME_0 {
    pub bFiller1: super::super::System::SystemServices::CHAR,
    pub Anonymous: JET_LOGTIME_0_0,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_LOGTIME_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_LOGTIME_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_LOGTIME_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_LOGTIME_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_LOGTIME_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_LOGTIME_0_0 {
    pub _bitfield: u8,
}
impl JET_LOGTIME_0_0 {}
impl ::std::default::Default for JET_LOGTIME_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_LOGTIME_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_LOGTIME_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for JET_LOGTIME_0_0 {}
unsafe impl ::windows::runtime::Abi for JET_LOGTIME_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub union JET_LOGTIME_1 {
    pub bFiller2: super::super::System::SystemServices::CHAR,
    pub Anonymous: JET_LOGTIME_1_0,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_LOGTIME_1 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_LOGTIME_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_LOGTIME_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_LOGTIME_1 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_LOGTIME_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_LOGTIME_1_0 {
    pub _bitfield: u8,
}
impl JET_LOGTIME_1_0 {}
impl ::std::default::Default for JET_LOGTIME_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_LOGTIME_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_LOGTIME_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for JET_LOGTIME_1_0 {}
unsafe impl ::windows::runtime::Abi for JET_LOGTIME_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct JET_LS(pub usize);
impl ::std::default::Default for JET_LS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for JET_LS {}
unsafe impl ::windows::runtime::Abi for JET_LS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const JET_MAX_COMPUTERNAME_LENGTH: u32 = 15u32;
pub const JET_MoveFirst: u32 = 2147483648u32;
pub const JET_MoveLast: u32 = 2147483647u32;
pub const JET_MovePrevious: i32 = -1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_OBJECTINFO {
    pub cbStruct: u32,
    pub objtyp: u32,
    pub dtCreate: f64,
    pub dtUpdate: f64,
    pub grbit: u32,
    pub flags: u32,
    pub cRecord: u32,
    pub cPage: u32,
}
impl JET_OBJECTINFO {}
impl ::std::default::Default for JET_OBJECTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_OBJECTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_OBJECTINFO")
            .field("cbStruct", &self.cbStruct)
            .field("objtyp", &self.objtyp)
            .field("dtCreate", &self.dtCreate)
            .field("dtUpdate", &self.dtUpdate)
            .field("grbit", &self.grbit)
            .field("flags", &self.flags)
            .field("cRecord", &self.cRecord)
            .field("cPage", &self.cPage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_OBJECTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.objtyp == other.objtyp
            && self.dtCreate == other.dtCreate
            && self.dtUpdate == other.dtUpdate
            && self.grbit == other.grbit
            && self.flags == other.flags
            && self.cRecord == other.cRecord
            && self.cPage == other.cPage
    }
}
impl ::std::cmp::Eq for JET_OBJECTINFO {}
unsafe impl ::windows::runtime::Abi for JET_OBJECTINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_OBJECTLIST {
    pub cbStruct: u32,
    pub tableid: JET_TABLEID,
    pub cRecord: u32,
    pub columnidcontainername: u32,
    pub columnidobjectname: u32,
    pub columnidobjtyp: u32,
    pub columniddtCreate: u32,
    pub columniddtUpdate: u32,
    pub columnidgrbit: u32,
    pub columnidflags: u32,
    pub columnidcRecord: u32,
    pub columnidcPage: u32,
}
impl JET_OBJECTLIST {}
impl ::std::default::Default for JET_OBJECTLIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_OBJECTLIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_OBJECTLIST")
            .field("cbStruct", &self.cbStruct)
            .field("tableid", &self.tableid)
            .field("cRecord", &self.cRecord)
            .field("columnidcontainername", &self.columnidcontainername)
            .field("columnidobjectname", &self.columnidobjectname)
            .field("columnidobjtyp", &self.columnidobjtyp)
            .field("columniddtCreate", &self.columniddtCreate)
            .field("columniddtUpdate", &self.columniddtUpdate)
            .field("columnidgrbit", &self.columnidgrbit)
            .field("columnidflags", &self.columnidflags)
            .field("columnidcRecord", &self.columnidcRecord)
            .field("columnidcPage", &self.columnidcPage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_OBJECTLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.tableid == other.tableid
            && self.cRecord == other.cRecord
            && self.columnidcontainername == other.columnidcontainername
            && self.columnidobjectname == other.columnidobjectname
            && self.columnidobjtyp == other.columnidobjtyp
            && self.columniddtCreate == other.columniddtCreate
            && self.columniddtUpdate == other.columniddtUpdate
            && self.columnidgrbit == other.columnidgrbit
            && self.columnidflags == other.columnidflags
            && self.columnidcRecord == other.columnidcRecord
            && self.columnidcPage == other.columnidcPage
    }
}
impl ::std::cmp::Eq for JET_OBJECTLIST {}
unsafe impl ::windows::runtime::Abi for JET_OBJECTLIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_OPENTEMPORARYTABLE {
    pub cbStruct: u32,
    pub prgcolumndef: *mut JET_COLUMNDEF,
    pub ccolumn: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
    pub grbit: u32,
    pub prgcolumnid: *mut u32,
    pub cbKeyMost: u32,
    pub cbVarSegMac: u32,
    pub tableid: JET_TABLEID,
}
impl JET_OPENTEMPORARYTABLE {}
impl ::std::default::Default for JET_OPENTEMPORARYTABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_OPENTEMPORARYTABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_OPENTEMPORARYTABLE")
            .field("cbStruct", &self.cbStruct)
            .field("prgcolumndef", &self.prgcolumndef)
            .field("ccolumn", &self.ccolumn)
            .field("pidxunicode", &self.pidxunicode)
            .field("grbit", &self.grbit)
            .field("prgcolumnid", &self.prgcolumnid)
            .field("cbKeyMost", &self.cbKeyMost)
            .field("cbVarSegMac", &self.cbVarSegMac)
            .field("tableid", &self.tableid)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_OPENTEMPORARYTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.prgcolumndef == other.prgcolumndef
            && self.ccolumn == other.ccolumn
            && self.pidxunicode == other.pidxunicode
            && self.grbit == other.grbit
            && self.prgcolumnid == other.prgcolumnid
            && self.cbKeyMost == other.cbKeyMost
            && self.cbVarSegMac == other.cbVarSegMac
            && self.tableid == other.tableid
    }
}
impl ::std::cmp::Eq for JET_OPENTEMPORARYTABLE {}
unsafe impl ::windows::runtime::Abi for JET_OPENTEMPORARYTABLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_OPENTEMPORARYTABLE2 {
    pub cbStruct: u32,
    pub prgcolumndef: *mut JET_COLUMNDEF,
    pub ccolumn: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX2,
    pub grbit: u32,
    pub prgcolumnid: *mut u32,
    pub cbKeyMost: u32,
    pub cbVarSegMac: u32,
    pub tableid: JET_TABLEID,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_OPENTEMPORARYTABLE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_OPENTEMPORARYTABLE2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_OPENTEMPORARYTABLE2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_OPENTEMPORARYTABLE2")
            .field("cbStruct", &self.cbStruct)
            .field("prgcolumndef", &self.prgcolumndef)
            .field("ccolumn", &self.ccolumn)
            .field("pidxunicode", &self.pidxunicode)
            .field("grbit", &self.grbit)
            .field("prgcolumnid", &self.prgcolumnid)
            .field("cbKeyMost", &self.cbKeyMost)
            .field("cbVarSegMac", &self.cbVarSegMac)
            .field("tableid", &self.tableid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_OPENTEMPORARYTABLE2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.prgcolumndef == other.prgcolumndef
            && self.ccolumn == other.ccolumn
            && self.pidxunicode == other.pidxunicode
            && self.grbit == other.grbit
            && self.prgcolumnid == other.prgcolumnid
            && self.cbKeyMost == other.cbKeyMost
            && self.cbVarSegMac == other.cbVarSegMac
            && self.tableid == other.tableid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_OPENTEMPORARYTABLE2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_OPENTEMPORARYTABLE2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_OPERATIONCONTEXT {
    pub ulUserID: u32,
    pub nOperationID: u8,
    pub nOperationType: u8,
    pub nClientType: u8,
    pub fFlags: u8,
}
impl JET_OPERATIONCONTEXT {}
impl ::std::default::Default for JET_OPERATIONCONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_OPERATIONCONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_OPERATIONCONTEXT")
            .field("ulUserID", &self.ulUserID)
            .field("nOperationID", &self.nOperationID)
            .field("nOperationType", &self.nOperationType)
            .field("nClientType", &self.nClientType)
            .field("fFlags", &self.fFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_OPERATIONCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ulUserID == other.ulUserID
            && self.nOperationID == other.nOperationID
            && self.nOperationType == other.nOperationType
            && self.nClientType == other.nClientType
            && self.fFlags == other.fFlags
    }
}
impl ::std::cmp::Eq for JET_OPERATIONCONTEXT {}
unsafe impl ::windows::runtime::Abi for JET_OPERATIONCONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct JET_OSSNAPID(pub usize);
impl ::std::default::Default for JET_OSSNAPID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for JET_OSSNAPID {}
unsafe impl ::windows::runtime::Abi for JET_OSSNAPID {
    type Abi = Self;
    type DefaultType = Self;
}
pub const JET_OnlineDefragAll: u32 = 65535u32;
pub const JET_OnlineDefragAllOBSOLETE: u32 = 1u32;
pub const JET_OnlineDefragDatabases: u32 = 2u32;
pub const JET_OnlineDefragDisable: u32 = 0u32;
pub const JET_OnlineDefragSpaceTrees: u32 = 4u32;
#[cfg(feature = "Win32_System_SystemServices")]
pub type JET_PFNDURABLECOMMITCALLBACK = unsafe extern "system" fn(
    instance: JET_INSTANCE,
    pcommitidseen: *const JET_COMMIT_ID,
    grbit: u32,
) -> i32;
pub type JET_PFNREALLOC = unsafe extern "system" fn(
    pvcontext: *const ::std::ffi::c_void,
    pv: *const ::std::ffi::c_void,
    cb: u32,
) -> *mut ::std::ffi::c_void;
pub type JET_PFNSTATUS = unsafe extern "system" fn(
    sesid: JET_SESID,
    snp: u32,
    snt: u32,
    pv: *const ::std::ffi::c_void,
) -> i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_RBSINFOMISC {
    pub lRBSGeneration: i32,
    pub logtimeCreate: JET_LOGTIME,
    pub logtimeCreatePrevRBS: JET_LOGTIME,
    pub ulMajor: u32,
    pub ulMinor: u32,
    pub cbLogicalFileSize: u64,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_RBSINFOMISC {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_RBSINFOMISC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_RBSINFOMISC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_RBSINFOMISC {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_RBSINFOMISC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_RBSREVERTINFOMISC {
    pub lGenMinRevertStart: i32,
    pub lGenMaxRevertStart: i32,
    pub lGenMinRevertEnd: i32,
    pub lGenMaxRevertEnd: i32,
    pub logtimeRevertFrom: JET_LOGTIME,
    pub cSecRevert: u64,
    pub cPagesReverted: u64,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_RBSREVERTINFOMISC {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_RBSREVERTINFOMISC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_RBSREVERTINFOMISC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_RBSREVERTINFOMISC {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_RBSREVERTINFOMISC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_RECORDLIST {
    pub cbStruct: u32,
    pub tableid: JET_TABLEID,
    pub cRecord: u32,
    pub columnidBookmark: u32,
}
impl JET_RECORDLIST {}
impl ::std::default::Default for JET_RECORDLIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_RECORDLIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_RECORDLIST")
            .field("cbStruct", &self.cbStruct)
            .field("tableid", &self.tableid)
            .field("cRecord", &self.cRecord)
            .field("columnidBookmark", &self.columnidBookmark)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_RECORDLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.tableid == other.tableid
            && self.cRecord == other.cRecord
            && self.columnidBookmark == other.columnidBookmark
    }
}
impl ::std::cmp::Eq for JET_RECORDLIST {}
unsafe impl ::windows::runtime::Abi for JET_RECORDLIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_RECPOS {
    pub cbStruct: u32,
    pub centriesLT: u32,
    pub centriesInRange: u32,
    pub centriesTotal: u32,
}
impl JET_RECPOS {}
impl ::std::default::Default for JET_RECPOS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_RECPOS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_RECPOS")
            .field("cbStruct", &self.cbStruct)
            .field("centriesLT", &self.centriesLT)
            .field("centriesInRange", &self.centriesInRange)
            .field("centriesTotal", &self.centriesTotal)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_RECPOS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.centriesLT == other.centriesLT
            && self.centriesInRange == other.centriesInRange
            && self.centriesTotal == other.centriesTotal
    }
}
impl ::std::cmp::Eq for JET_RECPOS {}
unsafe impl ::windows::runtime::Abi for JET_RECPOS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_RECSIZE {
    pub cbData: u64,
    pub cbLongValueData: u64,
    pub cbOverhead: u64,
    pub cbLongValueOverhead: u64,
    pub cNonTaggedColumns: u64,
    pub cTaggedColumns: u64,
    pub cLongValues: u64,
    pub cMultiValues: u64,
}
impl JET_RECSIZE {}
impl ::std::default::Default for JET_RECSIZE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_RECSIZE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_RECSIZE")
            .field("cbData", &self.cbData)
            .field("cbLongValueData", &self.cbLongValueData)
            .field("cbOverhead", &self.cbOverhead)
            .field("cbLongValueOverhead", &self.cbLongValueOverhead)
            .field("cNonTaggedColumns", &self.cNonTaggedColumns)
            .field("cTaggedColumns", &self.cTaggedColumns)
            .field("cLongValues", &self.cLongValues)
            .field("cMultiValues", &self.cMultiValues)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_RECSIZE {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData
            && self.cbLongValueData == other.cbLongValueData
            && self.cbOverhead == other.cbOverhead
            && self.cbLongValueOverhead == other.cbLongValueOverhead
            && self.cNonTaggedColumns == other.cNonTaggedColumns
            && self.cTaggedColumns == other.cTaggedColumns
            && self.cLongValues == other.cLongValues
            && self.cMultiValues == other.cMultiValues
    }
}
impl ::std::cmp::Eq for JET_RECSIZE {}
unsafe impl ::windows::runtime::Abi for JET_RECSIZE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_RECSIZE2 {
    pub cbData: u64,
    pub cbLongValueData: u64,
    pub cbOverhead: u64,
    pub cbLongValueOverhead: u64,
    pub cNonTaggedColumns: u64,
    pub cTaggedColumns: u64,
    pub cLongValues: u64,
    pub cMultiValues: u64,
    pub cCompressedColumns: u64,
    pub cbDataCompressed: u64,
    pub cbLongValueDataCompressed: u64,
}
impl JET_RECSIZE2 {}
impl ::std::default::Default for JET_RECSIZE2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_RECSIZE2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_RECSIZE2")
            .field("cbData", &self.cbData)
            .field("cbLongValueData", &self.cbLongValueData)
            .field("cbOverhead", &self.cbOverhead)
            .field("cbLongValueOverhead", &self.cbLongValueOverhead)
            .field("cNonTaggedColumns", &self.cNonTaggedColumns)
            .field("cTaggedColumns", &self.cTaggedColumns)
            .field("cLongValues", &self.cLongValues)
            .field("cMultiValues", &self.cMultiValues)
            .field("cCompressedColumns", &self.cCompressedColumns)
            .field("cbDataCompressed", &self.cbDataCompressed)
            .field("cbLongValueDataCompressed", &self.cbLongValueDataCompressed)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_RECSIZE2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData
            && self.cbLongValueData == other.cbLongValueData
            && self.cbOverhead == other.cbOverhead
            && self.cbLongValueOverhead == other.cbLongValueOverhead
            && self.cNonTaggedColumns == other.cNonTaggedColumns
            && self.cTaggedColumns == other.cTaggedColumns
            && self.cLongValues == other.cLongValues
            && self.cMultiValues == other.cMultiValues
            && self.cCompressedColumns == other.cCompressedColumns
            && self.cbDataCompressed == other.cbDataCompressed
            && self.cbLongValueDataCompressed == other.cbLongValueDataCompressed
    }
}
impl ::std::cmp::Eq for JET_RECSIZE2 {}
unsafe impl ::windows::runtime::Abi for JET_RECSIZE2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct JET_RELOP(pub i32);
pub const JET_relopEquals: JET_RELOP = JET_RELOP(0i32);
pub const JET_relopPrefixEquals: JET_RELOP = JET_RELOP(1i32);
pub const JET_relopNotEquals: JET_RELOP = JET_RELOP(2i32);
pub const JET_relopLessThanOrEqual: JET_RELOP = JET_RELOP(3i32);
pub const JET_relopLessThan: JET_RELOP = JET_RELOP(4i32);
pub const JET_relopGreaterThanOrEqual: JET_RELOP = JET_RELOP(5i32);
pub const JET_relopGreaterThan: JET_RELOP = JET_RELOP(6i32);
pub const JET_relopBitmaskEqualsZero: JET_RELOP = JET_RELOP(7i32);
pub const JET_relopBitmaskNotEqualsZero: JET_RELOP = JET_RELOP(8i32);
impl ::std::convert::From<i32> for JET_RELOP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JET_RELOP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_RETINFO {
    pub cbStruct: u32,
    pub ibLongValue: u32,
    pub itagSequence: u32,
    pub columnidNextTagged: u32,
}
impl JET_RETINFO {}
impl ::std::default::Default for JET_RETINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_RETINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_RETINFO")
            .field("cbStruct", &self.cbStruct)
            .field("ibLongValue", &self.ibLongValue)
            .field("itagSequence", &self.itagSequence)
            .field("columnidNextTagged", &self.columnidNextTagged)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_RETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.ibLongValue == other.ibLongValue
            && self.itagSequence == other.itagSequence
            && self.columnidNextTagged == other.columnidNextTagged
    }
}
impl ::std::cmp::Eq for JET_RETINFO {}
unsafe impl ::windows::runtime::Abi for JET_RETINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_RETRIEVECOLUMN {
    pub columnid: u32,
    pub pvData: *mut ::std::ffi::c_void,
    pub cbData: u32,
    pub cbActual: u32,
    pub grbit: u32,
    pub ibLongValue: u32,
    pub itagSequence: u32,
    pub columnidNextTagged: u32,
    pub err: i32,
}
impl JET_RETRIEVECOLUMN {}
impl ::std::default::Default for JET_RETRIEVECOLUMN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_RETRIEVECOLUMN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_RETRIEVECOLUMN")
            .field("columnid", &self.columnid)
            .field("pvData", &self.pvData)
            .field("cbData", &self.cbData)
            .field("cbActual", &self.cbActual)
            .field("grbit", &self.grbit)
            .field("ibLongValue", &self.ibLongValue)
            .field("itagSequence", &self.itagSequence)
            .field("columnidNextTagged", &self.columnidNextTagged)
            .field("err", &self.err)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_RETRIEVECOLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.columnid == other.columnid
            && self.pvData == other.pvData
            && self.cbData == other.cbData
            && self.cbActual == other.cbActual
            && self.grbit == other.grbit
            && self.ibLongValue == other.ibLongValue
            && self.itagSequence == other.itagSequence
            && self.columnidNextTagged == other.columnidNextTagged
            && self.err == other.err
    }
}
impl ::std::cmp::Eq for JET_RETRIEVECOLUMN {}
unsafe impl ::windows::runtime::Abi for JET_RETRIEVECOLUMN {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::clone::Clone for JET_RSTINFO_A {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct JET_RSTINFO_A {
    pub cbStruct: u32,
    pub rgrstmap: *mut JET_RSTMAP_A,
    pub crstmap: i32,
    pub lgposStop: JET_LGPOS,
    pub logtimeStop: JET_LOGTIME,
    pub pfnStatus: ::std::option::Option<JET_PFNSTATUS>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl JET_RSTINFO_A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for JET_RSTINFO_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for JET_RSTINFO_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for JET_RSTINFO_A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for JET_RSTINFO_A {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::clone::Clone for JET_RSTINFO_W {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct JET_RSTINFO_W {
    pub cbStruct: u32,
    pub rgrstmap: *mut JET_RSTMAP_W,
    pub crstmap: i32,
    pub lgposStop: JET_LGPOS,
    pub logtimeStop: JET_LOGTIME,
    pub pfnStatus: ::std::option::Option<JET_PFNSTATUS>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl JET_RSTINFO_W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for JET_RSTINFO_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for JET_RSTINFO_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for JET_RSTINFO_W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for JET_RSTINFO_W {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_RSTMAP_A {
    pub szDatabaseName: super::super::Foundation::PSTR,
    pub szNewDatabaseName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_RSTMAP_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_RSTMAP_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_RSTMAP_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_RSTMAP_A")
            .field("szDatabaseName", &self.szDatabaseName)
            .field("szNewDatabaseName", &self.szNewDatabaseName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_RSTMAP_A {
    fn eq(&self, other: &Self) -> bool {
        self.szDatabaseName == other.szDatabaseName
            && self.szNewDatabaseName == other.szNewDatabaseName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_RSTMAP_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_RSTMAP_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_RSTMAP_W {
    pub szDatabaseName: super::super::Foundation::PWSTR,
    pub szNewDatabaseName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_RSTMAP_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_RSTMAP_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_RSTMAP_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_RSTMAP_W")
            .field("szDatabaseName", &self.szDatabaseName)
            .field("szNewDatabaseName", &self.szNewDatabaseName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_RSTMAP_W {
    fn eq(&self, other: &Self) -> bool {
        self.szDatabaseName == other.szDatabaseName
            && self.szNewDatabaseName == other.szNewDatabaseName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_RSTMAP_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_RSTMAP_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct JET_SESID(pub usize);
impl ::std::default::Default for JET_SESID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for JET_SESID {}
unsafe impl ::windows::runtime::Abi for JET_SESID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_SETCOLUMN {
    pub columnid: u32,
    pub pvData: *mut ::std::ffi::c_void,
    pub cbData: u32,
    pub grbit: u32,
    pub ibLongValue: u32,
    pub itagSequence: u32,
    pub err: i32,
}
impl JET_SETCOLUMN {}
impl ::std::default::Default for JET_SETCOLUMN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_SETCOLUMN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_SETCOLUMN")
            .field("columnid", &self.columnid)
            .field("pvData", &self.pvData)
            .field("cbData", &self.cbData)
            .field("grbit", &self.grbit)
            .field("ibLongValue", &self.ibLongValue)
            .field("itagSequence", &self.itagSequence)
            .field("err", &self.err)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_SETCOLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.columnid == other.columnid
            && self.pvData == other.pvData
            && self.cbData == other.cbData
            && self.grbit == other.grbit
            && self.ibLongValue == other.ibLongValue
            && self.itagSequence == other.itagSequence
            && self.err == other.err
    }
}
impl ::std::cmp::Eq for JET_SETCOLUMN {}
unsafe impl ::windows::runtime::Abi for JET_SETCOLUMN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_SETINFO {
    pub cbStruct: u32,
    pub ibLongValue: u32,
    pub itagSequence: u32,
}
impl JET_SETINFO {}
impl ::std::default::Default for JET_SETINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_SETINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_SETINFO")
            .field("cbStruct", &self.cbStruct)
            .field("ibLongValue", &self.ibLongValue)
            .field("itagSequence", &self.itagSequence)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_SETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.ibLongValue == other.ibLongValue
            && self.itagSequence == other.itagSequence
    }
}
impl ::std::cmp::Eq for JET_SETINFO {}
unsafe impl ::windows::runtime::Abi for JET_SETINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_SETSYSPARAM_A {
    pub paramid: u32,
    pub lParam: JET_API_PTR,
    pub sz: super::super::Foundation::PSTR,
    pub err: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_SETSYSPARAM_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_SETSYSPARAM_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_SETSYSPARAM_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_SETSYSPARAM_A")
            .field("paramid", &self.paramid)
            .field("lParam", &self.lParam)
            .field("sz", &self.sz)
            .field("err", &self.err)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_SETSYSPARAM_A {
    fn eq(&self, other: &Self) -> bool {
        self.paramid == other.paramid
            && self.lParam == other.lParam
            && self.sz == other.sz
            && self.err == other.err
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_SETSYSPARAM_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_SETSYSPARAM_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_SETSYSPARAM_W {
    pub paramid: u32,
    pub lParam: JET_API_PTR,
    pub sz: super::super::Foundation::PWSTR,
    pub err: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_SETSYSPARAM_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_SETSYSPARAM_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_SETSYSPARAM_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_SETSYSPARAM_W")
            .field("paramid", &self.paramid)
            .field("lParam", &self.lParam)
            .field("sz", &self.sz)
            .field("err", &self.err)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_SETSYSPARAM_W {
    fn eq(&self, other: &Self) -> bool {
        self.paramid == other.paramid
            && self.lParam == other.lParam
            && self.sz == other.sz
            && self.err == other.err
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_SETSYSPARAM_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_SETSYSPARAM_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct JET_SIGNATURE {
    pub ulRandom: u32,
    pub logtimeCreate: JET_LOGTIME,
    pub szComputerName: [super::super::System::SystemServices::CHAR; 16],
}
#[cfg(feature = "Win32_System_SystemServices")]
impl JET_SIGNATURE {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for JET_SIGNATURE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for JET_SIGNATURE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for JET_SIGNATURE {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for JET_SIGNATURE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_SNPROG {
    pub cbStruct: u32,
    pub cunitDone: u32,
    pub cunitTotal: u32,
}
impl JET_SNPROG {}
impl ::std::default::Default for JET_SNPROG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_SNPROG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_SNPROG")
            .field("cbStruct", &self.cbStruct)
            .field("cunitDone", &self.cunitDone)
            .field("cunitTotal", &self.cunitTotal)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_SNPROG {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.cunitDone == other.cunitDone
            && self.cunitTotal == other.cunitTotal
    }
}
impl ::std::cmp::Eq for JET_SNPROG {}
unsafe impl ::windows::runtime::Abi for JET_SNPROG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_SPACEHINTS {
    pub cbStruct: u32,
    pub ulInitialDensity: u32,
    pub cbInitial: u32,
    pub grbit: u32,
    pub ulMaintDensity: u32,
    pub ulGrowth: u32,
    pub cbMinExtent: u32,
    pub cbMaxExtent: u32,
}
impl JET_SPACEHINTS {}
impl ::std::default::Default for JET_SPACEHINTS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_SPACEHINTS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_SPACEHINTS")
            .field("cbStruct", &self.cbStruct)
            .field("ulInitialDensity", &self.ulInitialDensity)
            .field("cbInitial", &self.cbInitial)
            .field("grbit", &self.grbit)
            .field("ulMaintDensity", &self.ulMaintDensity)
            .field("ulGrowth", &self.ulGrowth)
            .field("cbMinExtent", &self.cbMinExtent)
            .field("cbMaxExtent", &self.cbMaxExtent)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_SPACEHINTS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.ulInitialDensity == other.ulInitialDensity
            && self.cbInitial == other.cbInitial
            && self.grbit == other.grbit
            && self.ulMaintDensity == other.ulMaintDensity
            && self.ulGrowth == other.ulGrowth
            && self.cbMinExtent == other.cbMinExtent
            && self.cbMaxExtent == other.cbMaxExtent
    }
}
impl ::std::cmp::Eq for JET_SPACEHINTS {}
unsafe impl ::windows::runtime::Abi for JET_SPACEHINTS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_TABLECREATE2_A {
    pub cbStruct: u32,
    pub szTableName: super::super::Foundation::PSTR,
    pub szTemplateTableName: super::super::Foundation::PSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE_A,
    pub cIndexes: u32,
    pub szCallback: super::super::Foundation::PSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub tableid: JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_TABLECREATE2_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_TABLECREATE2_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_TABLECREATE2_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_TABLECREATE2_A")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_TABLECREATE2_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.szTableName == other.szTableName
            && self.szTemplateTableName == other.szTemplateTableName
            && self.ulPages == other.ulPages
            && self.ulDensity == other.ulDensity
            && self.rgcolumncreate == other.rgcolumncreate
            && self.cColumns == other.cColumns
            && self.rgindexcreate == other.rgindexcreate
            && self.cIndexes == other.cIndexes
            && self.szCallback == other.szCallback
            && self.cbtyp == other.cbtyp
            && self.grbit == other.grbit
            && self.tableid == other.tableid
            && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_TABLECREATE2_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_TABLECREATE2_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_TABLECREATE2_W {
    pub cbStruct: u32,
    pub szTableName: super::super::Foundation::PWSTR,
    pub szTemplateTableName: super::super::Foundation::PWSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE_W,
    pub cIndexes: u32,
    pub szCallback: super::super::Foundation::PWSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub tableid: JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_TABLECREATE2_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_TABLECREATE2_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_TABLECREATE2_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_TABLECREATE2_W")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_TABLECREATE2_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.szTableName == other.szTableName
            && self.szTemplateTableName == other.szTemplateTableName
            && self.ulPages == other.ulPages
            && self.ulDensity == other.ulDensity
            && self.rgcolumncreate == other.rgcolumncreate
            && self.cColumns == other.cColumns
            && self.rgindexcreate == other.rgindexcreate
            && self.cIndexes == other.cIndexes
            && self.szCallback == other.szCallback
            && self.cbtyp == other.cbtyp
            && self.grbit == other.grbit
            && self.tableid == other.tableid
            && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_TABLECREATE2_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_TABLECREATE2_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_TABLECREATE3_A {
    pub cbStruct: u32,
    pub szTableName: super::super::Foundation::PSTR,
    pub szTemplateTableName: super::super::Foundation::PSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE2_A,
    pub cIndexes: u32,
    pub szCallback: super::super::Foundation::PSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: u32,
    pub tableid: JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_TABLECREATE3_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_TABLECREATE3_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_TABLECREATE3_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_TABLECREATE3_A")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("pSeqSpacehints", &self.pSeqSpacehints)
            .field("pLVSpacehints", &self.pLVSpacehints)
            .field("cbSeparateLV", &self.cbSeparateLV)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_TABLECREATE3_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.szTableName == other.szTableName
            && self.szTemplateTableName == other.szTemplateTableName
            && self.ulPages == other.ulPages
            && self.ulDensity == other.ulDensity
            && self.rgcolumncreate == other.rgcolumncreate
            && self.cColumns == other.cColumns
            && self.rgindexcreate == other.rgindexcreate
            && self.cIndexes == other.cIndexes
            && self.szCallback == other.szCallback
            && self.cbtyp == other.cbtyp
            && self.grbit == other.grbit
            && self.pSeqSpacehints == other.pSeqSpacehints
            && self.pLVSpacehints == other.pLVSpacehints
            && self.cbSeparateLV == other.cbSeparateLV
            && self.tableid == other.tableid
            && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_TABLECREATE3_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_TABLECREATE3_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_TABLECREATE3_W {
    pub cbStruct: u32,
    pub szTableName: super::super::Foundation::PWSTR,
    pub szTemplateTableName: super::super::Foundation::PWSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE2_W,
    pub cIndexes: u32,
    pub szCallback: super::super::Foundation::PWSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: u32,
    pub tableid: JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_TABLECREATE3_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_TABLECREATE3_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_TABLECREATE3_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_TABLECREATE3_W")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("pSeqSpacehints", &self.pSeqSpacehints)
            .field("pLVSpacehints", &self.pLVSpacehints)
            .field("cbSeparateLV", &self.cbSeparateLV)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_TABLECREATE3_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.szTableName == other.szTableName
            && self.szTemplateTableName == other.szTemplateTableName
            && self.ulPages == other.ulPages
            && self.ulDensity == other.ulDensity
            && self.rgcolumncreate == other.rgcolumncreate
            && self.cColumns == other.cColumns
            && self.rgindexcreate == other.rgindexcreate
            && self.cIndexes == other.cIndexes
            && self.szCallback == other.szCallback
            && self.cbtyp == other.cbtyp
            && self.grbit == other.grbit
            && self.pSeqSpacehints == other.pSeqSpacehints
            && self.pLVSpacehints == other.pLVSpacehints
            && self.cbSeparateLV == other.cbSeparateLV
            && self.tableid == other.tableid
            && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_TABLECREATE3_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_TABLECREATE3_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_TABLECREATE4_A {
    pub cbStruct: u32,
    pub szTableName: super::super::Foundation::PSTR,
    pub szTemplateTableName: super::super::Foundation::PSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE3_A,
    pub cIndexes: u32,
    pub szCallback: super::super::Foundation::PSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: u32,
    pub tableid: JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_TABLECREATE4_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_TABLECREATE4_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_TABLECREATE4_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_TABLECREATE4_A")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("pSeqSpacehints", &self.pSeqSpacehints)
            .field("pLVSpacehints", &self.pLVSpacehints)
            .field("cbSeparateLV", &self.cbSeparateLV)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_TABLECREATE4_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.szTableName == other.szTableName
            && self.szTemplateTableName == other.szTemplateTableName
            && self.ulPages == other.ulPages
            && self.ulDensity == other.ulDensity
            && self.rgcolumncreate == other.rgcolumncreate
            && self.cColumns == other.cColumns
            && self.rgindexcreate == other.rgindexcreate
            && self.cIndexes == other.cIndexes
            && self.szCallback == other.szCallback
            && self.cbtyp == other.cbtyp
            && self.grbit == other.grbit
            && self.pSeqSpacehints == other.pSeqSpacehints
            && self.pLVSpacehints == other.pLVSpacehints
            && self.cbSeparateLV == other.cbSeparateLV
            && self.tableid == other.tableid
            && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_TABLECREATE4_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_TABLECREATE4_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_TABLECREATE4_W {
    pub cbStruct: u32,
    pub szTableName: super::super::Foundation::PWSTR,
    pub szTemplateTableName: super::super::Foundation::PWSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE3_W,
    pub cIndexes: u32,
    pub szCallback: super::super::Foundation::PWSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: u32,
    pub tableid: JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_TABLECREATE4_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_TABLECREATE4_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_TABLECREATE4_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_TABLECREATE4_W")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("pSeqSpacehints", &self.pSeqSpacehints)
            .field("pLVSpacehints", &self.pLVSpacehints)
            .field("cbSeparateLV", &self.cbSeparateLV)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_TABLECREATE4_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.szTableName == other.szTableName
            && self.szTemplateTableName == other.szTemplateTableName
            && self.ulPages == other.ulPages
            && self.ulDensity == other.ulDensity
            && self.rgcolumncreate == other.rgcolumncreate
            && self.cColumns == other.cColumns
            && self.rgindexcreate == other.rgindexcreate
            && self.cIndexes == other.cIndexes
            && self.szCallback == other.szCallback
            && self.cbtyp == other.cbtyp
            && self.grbit == other.grbit
            && self.pSeqSpacehints == other.pSeqSpacehints
            && self.pLVSpacehints == other.pLVSpacehints
            && self.cbSeparateLV == other.cbSeparateLV
            && self.tableid == other.tableid
            && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_TABLECREATE4_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_TABLECREATE4_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_TABLECREATE_A {
    pub cbStruct: u32,
    pub szTableName: super::super::Foundation::PSTR,
    pub szTemplateTableName: super::super::Foundation::PSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE_A,
    pub cIndexes: u32,
    pub grbit: u32,
    pub tableid: JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_TABLECREATE_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_TABLECREATE_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_TABLECREATE_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_TABLECREATE_A")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("grbit", &self.grbit)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_TABLECREATE_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.szTableName == other.szTableName
            && self.szTemplateTableName == other.szTemplateTableName
            && self.ulPages == other.ulPages
            && self.ulDensity == other.ulDensity
            && self.rgcolumncreate == other.rgcolumncreate
            && self.cColumns == other.cColumns
            && self.rgindexcreate == other.rgindexcreate
            && self.cIndexes == other.cIndexes
            && self.grbit == other.grbit
            && self.tableid == other.tableid
            && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_TABLECREATE_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_TABLECREATE_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_TABLECREATE_W {
    pub cbStruct: u32,
    pub szTableName: super::super::Foundation::PWSTR,
    pub szTemplateTableName: super::super::Foundation::PWSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE_W,
    pub cIndexes: u32,
    pub grbit: u32,
    pub tableid: JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_TABLECREATE_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_TABLECREATE_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_TABLECREATE_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_TABLECREATE_W")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("grbit", &self.grbit)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_TABLECREATE_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.szTableName == other.szTableName
            && self.szTemplateTableName == other.szTemplateTableName
            && self.ulPages == other.ulPages
            && self.ulDensity == other.ulDensity
            && self.rgcolumncreate == other.rgcolumncreate
            && self.cColumns == other.cColumns
            && self.rgindexcreate == other.rgindexcreate
            && self.cIndexes == other.cIndexes
            && self.grbit == other.grbit
            && self.tableid == other.tableid
            && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_TABLECREATE_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_TABLECREATE_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct JET_TABLEID(pub usize);
impl ::std::default::Default for JET_TABLEID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for JET_TABLEID {}
unsafe impl ::windows::runtime::Abi for JET_TABLEID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_THREADSTATS {
    pub cbStruct: u32,
    pub cPageReferenced: u32,
    pub cPageRead: u32,
    pub cPagePreread: u32,
    pub cPageDirtied: u32,
    pub cPageRedirtied: u32,
    pub cLogRecord: u32,
    pub cbLogRecord: u32,
}
impl JET_THREADSTATS {}
impl ::std::default::Default for JET_THREADSTATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_THREADSTATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_THREADSTATS")
            .field("cbStruct", &self.cbStruct)
            .field("cPageReferenced", &self.cPageReferenced)
            .field("cPageRead", &self.cPageRead)
            .field("cPagePreread", &self.cPagePreread)
            .field("cPageDirtied", &self.cPageDirtied)
            .field("cPageRedirtied", &self.cPageRedirtied)
            .field("cLogRecord", &self.cLogRecord)
            .field("cbLogRecord", &self.cbLogRecord)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_THREADSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.cPageReferenced == other.cPageReferenced
            && self.cPageRead == other.cPageRead
            && self.cPagePreread == other.cPagePreread
            && self.cPageDirtied == other.cPageDirtied
            && self.cPageRedirtied == other.cPageRedirtied
            && self.cLogRecord == other.cLogRecord
            && self.cbLogRecord == other.cbLogRecord
    }
}
impl ::std::cmp::Eq for JET_THREADSTATS {}
unsafe impl ::windows::runtime::Abi for JET_THREADSTATS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_THREADSTATS2 {
    pub cbStruct: u32,
    pub cPageReferenced: u32,
    pub cPageRead: u32,
    pub cPagePreread: u32,
    pub cPageDirtied: u32,
    pub cPageRedirtied: u32,
    pub cLogRecord: u32,
    pub cbLogRecord: u32,
    pub cusecPageCacheMiss: u64,
    pub cPageCacheMiss: u32,
}
impl JET_THREADSTATS2 {}
impl ::std::default::Default for JET_THREADSTATS2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_THREADSTATS2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_THREADSTATS2")
            .field("cbStruct", &self.cbStruct)
            .field("cPageReferenced", &self.cPageReferenced)
            .field("cPageRead", &self.cPageRead)
            .field("cPagePreread", &self.cPagePreread)
            .field("cPageDirtied", &self.cPageDirtied)
            .field("cPageRedirtied", &self.cPageRedirtied)
            .field("cLogRecord", &self.cLogRecord)
            .field("cbLogRecord", &self.cbLogRecord)
            .field("cusecPageCacheMiss", &self.cusecPageCacheMiss)
            .field("cPageCacheMiss", &self.cPageCacheMiss)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_THREADSTATS2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.cPageReferenced == other.cPageReferenced
            && self.cPageRead == other.cPageRead
            && self.cPagePreread == other.cPagePreread
            && self.cPageDirtied == other.cPageDirtied
            && self.cPageRedirtied == other.cPageRedirtied
            && self.cLogRecord == other.cLogRecord
            && self.cbLogRecord == other.cbLogRecord
            && self.cusecPageCacheMiss == other.cusecPageCacheMiss
            && self.cPageCacheMiss == other.cPageCacheMiss
    }
}
impl ::std::cmp::Eq for JET_THREADSTATS2 {}
unsafe impl ::windows::runtime::Abi for JET_THREADSTATS2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_TUPLELIMITS {
    pub chLengthMin: u32,
    pub chLengthMax: u32,
    pub chToIndexMax: u32,
    pub cchIncrement: u32,
    pub ichStart: u32,
}
impl JET_TUPLELIMITS {}
impl ::std::default::Default for JET_TUPLELIMITS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_TUPLELIMITS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_TUPLELIMITS")
            .field("chLengthMin", &self.chLengthMin)
            .field("chLengthMax", &self.chLengthMax)
            .field("chToIndexMax", &self.chToIndexMax)
            .field("cchIncrement", &self.cchIncrement)
            .field("ichStart", &self.ichStart)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_TUPLELIMITS {
    fn eq(&self, other: &Self) -> bool {
        self.chLengthMin == other.chLengthMin
            && self.chLengthMax == other.chLengthMax
            && self.chToIndexMax == other.chToIndexMax
            && self.cchIncrement == other.cchIncrement
            && self.ichStart == other.ichStart
    }
}
impl ::std::cmp::Eq for JET_TUPLELIMITS {}
unsafe impl ::windows::runtime::Abi for JET_TUPLELIMITS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JET_UNICODEINDEX {
    pub lcid: u32,
    pub dwMapFlags: u32,
}
impl JET_UNICODEINDEX {}
impl ::std::default::Default for JET_UNICODEINDEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JET_UNICODEINDEX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_UNICODEINDEX")
            .field("lcid", &self.lcid)
            .field("dwMapFlags", &self.dwMapFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JET_UNICODEINDEX {
    fn eq(&self, other: &Self) -> bool {
        self.lcid == other.lcid && self.dwMapFlags == other.dwMapFlags
    }
}
impl ::std::cmp::Eq for JET_UNICODEINDEX {}
unsafe impl ::windows::runtime::Abi for JET_UNICODEINDEX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_UNICODEINDEX2 {
    pub szLocaleName: super::super::Foundation::PWSTR,
    pub dwMapFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_UNICODEINDEX2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_UNICODEINDEX2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_UNICODEINDEX2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_UNICODEINDEX2")
            .field("szLocaleName", &self.szLocaleName)
            .field("dwMapFlags", &self.dwMapFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_UNICODEINDEX2 {
    fn eq(&self, other: &Self) -> bool {
        self.szLocaleName == other.szLocaleName && self.dwMapFlags == other.dwMapFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_UNICODEINDEX2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_UNICODEINDEX2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_USERDEFINEDDEFAULT_A {
    pub szCallback: super::super::Foundation::PSTR,
    pub pbUserData: *mut u8,
    pub cbUserData: u32,
    pub szDependantColumns: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_USERDEFINEDDEFAULT_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_USERDEFINEDDEFAULT_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_USERDEFINEDDEFAULT_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_USERDEFINEDDEFAULT_A")
            .field("szCallback", &self.szCallback)
            .field("pbUserData", &self.pbUserData)
            .field("cbUserData", &self.cbUserData)
            .field("szDependantColumns", &self.szDependantColumns)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_USERDEFINEDDEFAULT_A {
    fn eq(&self, other: &Self) -> bool {
        self.szCallback == other.szCallback
            && self.pbUserData == other.pbUserData
            && self.cbUserData == other.cbUserData
            && self.szDependantColumns == other.szDependantColumns
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_USERDEFINEDDEFAULT_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_USERDEFINEDDEFAULT_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_USERDEFINEDDEFAULT_W {
    pub szCallback: super::super::Foundation::PWSTR,
    pub pbUserData: *mut u8,
    pub cbUserData: u32,
    pub szDependantColumns: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl JET_USERDEFINEDDEFAULT_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JET_USERDEFINEDDEFAULT_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JET_USERDEFINEDDEFAULT_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JET_USERDEFINEDDEFAULT_W")
            .field("szCallback", &self.szCallback)
            .field("pbUserData", &self.pbUserData)
            .field("cbUserData", &self.cbUserData)
            .field("szDependantColumns", &self.szDependantColumns)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JET_USERDEFINEDDEFAULT_W {
    fn eq(&self, other: &Self) -> bool {
        self.szCallback == other.szCallback
            && self.pbUserData == other.pbUserData
            && self.cbUserData == other.cbUserData
            && self.szDependantColumns == other.szDependantColumns
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JET_USERDEFINEDDEFAULT_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JET_USERDEFINEDDEFAULT_W {
    type Abi = Self;
    type DefaultType = Self;
}
pub const JET_bitAbortSnapshot: u32 = 1u32;
pub const JET_bitAllDatabasesSnapshot: u32 = 1u32;
pub const JET_bitBackupAtomic: u32 = 4u32;
pub const JET_bitBackupEndAbort: u32 = 2u32;
pub const JET_bitBackupEndNormal: u32 = 1u32;
pub const JET_bitBackupIncremental: u32 = 1u32;
pub const JET_bitBackupSnapshot: u32 = 16u32;
pub const JET_bitBackupTruncateDone: u32 = 256u32;
pub const JET_bitBookmarkPermitVirtualCurrency: u32 = 1u32;
pub const JET_bitCheckUniqueness: u32 = 64u32;
pub const JET_bitColumnAutoincrement: u32 = 16u32;
pub const JET_bitColumnCompressed: u32 = 524288u32;
pub const JET_bitColumnDeleteOnZero: u32 = 131072u32;
pub const JET_bitColumnEscrowUpdate: u32 = 2048u32;
pub const JET_bitColumnFinalize: u32 = 16384u32;
pub const JET_bitColumnFixed: u32 = 1u32;
pub const JET_bitColumnMaybeNull: u32 = 8192u32;
pub const JET_bitColumnMultiValued: u32 = 1024u32;
pub const JET_bitColumnNotNULL: u32 = 4u32;
pub const JET_bitColumnTTDescending: u32 = 128u32;
pub const JET_bitColumnTTKey: u32 = 64u32;
pub const JET_bitColumnTagged: u32 = 2u32;
pub const JET_bitColumnUnversioned: u32 = 4096u32;
pub const JET_bitColumnUpdatable: u32 = 32u32;
pub const JET_bitColumnUserDefinedDefault: u32 = 32768u32;
pub const JET_bitColumnVersion: u32 = 8u32;
pub const JET_bitCommitLazyFlush: u32 = 1u32;
pub const JET_bitCompactRepair: u32 = 64u32;
pub const JET_bitCompactStats: u32 = 32u32;
pub const JET_bitConfigStoreReadControlDefault: u32 = 0u32;
pub const JET_bitConfigStoreReadControlDisableAll: u32 = 2u32;
pub const JET_bitConfigStoreReadControlInhibitRead: u32 = 1u32;
pub const JET_bitContinueAfterThaw: u32 = 4u32;
pub const JET_bitCopySnapshot: u32 = 2u32;
pub const JET_bitCreateHintAppendSequential: u32 = 2u32;
pub const JET_bitCreateHintHotpointSequential: u32 = 4u32;
pub const JET_bitDbDeleteCorruptIndexes: u32 = 16u32;
pub const JET_bitDbDeleteUnicodeIndexes: u32 = 1024u32;
pub const JET_bitDbEnableBackgroundMaintenance: u32 = 2048u32;
pub const JET_bitDbExclusive: u32 = 2u32;
pub const JET_bitDbOverwriteExisting: u32 = 512u32;
pub const JET_bitDbPurgeCacheOnAttach: u32 = 4096u32;
pub const JET_bitDbReadOnly: u32 = 1u32;
pub const JET_bitDbRecoveryOff: u32 = 8u32;
pub const JET_bitDbShadowingOff: u32 = 128u32;
pub const JET_bitDbUpgrade: u32 = 512u32;
pub const JET_bitDefragmentAvailSpaceTreesOnly: u32 = 64u32;
pub const JET_bitDefragmentBTree: u32 = 256u32;
pub const JET_bitDefragmentBatchStart: u32 = 1u32;
pub const JET_bitDefragmentBatchStop: u32 = 2u32;
pub const JET_bitDefragmentNoPartialMerges: u32 = 128u32;
pub const JET_bitDeleteAllExistingLogs: u32 = 1u32;
pub const JET_bitDeleteColumnIgnoreTemplateColumns: u32 = 1u32;
pub const JET_bitDeleteHintTableSequential: u32 = 256u32;
pub const JET_bitDumpCacheIncludeCachedPages: u32 = 32u32;
pub const JET_bitDumpCacheIncludeCorruptedPages: u32 = 64u32;
pub const JET_bitDumpCacheIncludeDirtyPages: u32 = 16u32;
pub const JET_bitDumpCacheMaximum: u32 = 8u32;
pub const JET_bitDumpCacheMinimum: u32 = 4u32;
pub const JET_bitDumpCacheNoDecommit: u32 = 128u32;
pub const JET_bitDumpMaximum: u32 = 2u32;
pub const JET_bitDumpMinimum: u32 = 1u32;
pub const JET_bitDurableCommitCallbackLogUnavailable: u32 = 1u32;
pub const JET_bitESE98FileNames: u32 = 1u32;
pub const JET_bitEightDotThreeSoftCompat: u32 = 2u32;
pub const JET_bitEnumerateCompressOutput: u32 = 524288u32;
pub const JET_bitEnumerateCopy: u32 = 1u32;
pub const JET_bitEnumerateIgnoreDefault: u32 = 32u32;
pub const JET_bitEnumerateIgnoreUserDefinedDefault: u32 = 1048576u32;
pub const JET_bitEnumerateInRecordOnly: u32 = 2097152u32;
pub const JET_bitEnumeratePresenceOnly: u32 = 131072u32;
pub const JET_bitEnumerateTaggedOnly: u32 = 262144u32;
pub const JET_bitEscrowNoRollback: u32 = 1u32;
pub const JET_bitExplicitPrepare: u32 = 8u32;
pub const JET_bitForceDetach: u32 = 1u32;
pub const JET_bitForceNewLog: u32 = 16u32;
pub const JET_bitFullColumnEndLimit: u32 = 512u32;
pub const JET_bitFullColumnStartLimit: u32 = 256u32;
pub const JET_bitHungIOEvent: u32 = 1u32;
pub const JET_bitIdleCompact: u32 = 2u32;
pub const JET_bitIdleFlushBuffers: u32 = 1u32;
pub const JET_bitIdleStatus: u32 = 4u32;
pub const JET_bitIncrementalSnapshot: u32 = 1u32;
pub const JET_bitIndexColumnMustBeNonNull: u32 = 2u32;
pub const JET_bitIndexColumnMustBeNull: u32 = 1u32;
pub const JET_bitIndexCrossProduct: u32 = 16384u32;
pub const JET_bitIndexDisallowNull: u32 = 4u32;
pub const JET_bitIndexDisallowTruncation: u32 = 65536u32;
pub const JET_bitIndexDotNetGuid: u32 = 262144u32;
pub const JET_bitIndexEmpty: u32 = 256u32;
pub const JET_bitIndexIgnoreAnyNull: u32 = 32u32;
pub const JET_bitIndexIgnoreFirstNull: u32 = 64u32;
pub const JET_bitIndexIgnoreNull: u32 = 8u32;
pub const JET_bitIndexImmutableStructure: u32 = 524288u32;
pub const JET_bitIndexKeyMost: u32 = 32768u32;
pub const JET_bitIndexLazyFlush: u32 = 128u32;
pub const JET_bitIndexNestedTable: u32 = 131072u32;
pub const JET_bitIndexPrimary: u32 = 2u32;
pub const JET_bitIndexSortNullsHigh: u32 = 1024u32;
pub const JET_bitIndexTupleLimits: u32 = 8192u32;
pub const JET_bitIndexTuples: u32 = 4096u32;
pub const JET_bitIndexUnicode: u32 = 2048u32;
pub const JET_bitIndexUnique: u32 = 1u32;
pub const JET_bitIndexUnversioned: u32 = 512u32;
pub const JET_bitKeepDbAttachedAtEndOfRecovery: u32 = 4096u32;
pub const JET_bitKeyAscending: u32 = 0u32;
pub const JET_bitKeyDataZeroLength: u32 = 16u32;
pub const JET_bitKeyDescending: u32 = 1u32;
pub const JET_bitLSCursor: u32 = 2u32;
pub const JET_bitLSReset: u32 = 1u32;
pub const JET_bitLSTable: u32 = 4u32;
pub const JET_bitLogStreamMustExist: u32 = 64u32;
pub const JET_bitMoveFirst: u32 = 0u32;
pub const JET_bitMoveKeyNE: u32 = 1u32;
pub const JET_bitNewKey: u32 = 1u32;
pub const JET_bitNoMove: u32 = 2u32;
pub const JET_bitNormalizedKey: u32 = 8u32;
pub const JET_bitObjectSystem: u32 = 2147483648u32;
pub const JET_bitObjectTableDerived: u32 = 268435456u32;
pub const JET_bitObjectTableFixedDDL: u32 = 1073741824u32;
pub const JET_bitObjectTableNoFixedVarColumnsInDerivedTables: u32 = 67108864u32;
pub const JET_bitObjectTableTemplate: u32 = 536870912u32;
pub const JET_bitPartialColumnEndLimit: u32 = 2048u32;
pub const JET_bitPartialColumnStartLimit: u32 = 1024u32;
pub const JET_bitPrereadBackward: u32 = 2u32;
pub const JET_bitPrereadFirstPage: u32 = 4u32;
pub const JET_bitPrereadForward: u32 = 1u32;
pub const JET_bitPrereadNormalizedKey: u32 = 8u32;
pub const JET_bitRangeInclusive: u32 = 1u32;
pub const JET_bitRangeInstantDuration: u32 = 4u32;
pub const JET_bitRangeRemove: u32 = 8u32;
pub const JET_bitRangeUpperLimit: u32 = 2u32;
pub const JET_bitReadLock: u32 = 1u32;
pub const JET_bitRecordInIndex: u32 = 1u32;
pub const JET_bitRecordNotInIndex: u32 = 2u32;
pub const JET_bitRecordSizeInCopyBuffer: u32 = 1u32;
pub const JET_bitRecordSizeLocal: u32 = 4u32;
pub const JET_bitRecordSizeRunningTotal: u32 = 2u32;
pub const JET_bitRecoveryWithoutUndo: u32 = 8u32;
pub const JET_bitReplayIgnoreLostLogs: u32 = 128u32;
pub const JET_bitReplayIgnoreMissingDB: u32 = 4u32;
pub const JET_bitReplayMissingMapEntryDB: u32 = 32u32;
pub const JET_bitResizeDatabaseOnlyGrow: u32 = 1u32;
pub const JET_bitResizeDatabaseOnlyShrink: u32 = 2u32;
pub const JET_bitRetrieveCopy: u32 = 1u32;
pub const JET_bitRetrieveFromIndex: u32 = 2u32;
pub const JET_bitRetrieveFromPrimaryBookmark: u32 = 4u32;
pub const JET_bitRetrieveHintReserve1: u32 = 8u32;
pub const JET_bitRetrieveHintReserve2: u32 = 64u32;
pub const JET_bitRetrieveHintReserve3: u32 = 128u32;
pub const JET_bitRetrieveHintTableScanBackward: u32 = 32u32;
pub const JET_bitRetrieveHintTableScanForward: u32 = 16u32;
pub const JET_bitRetrieveIgnoreDefault: u32 = 32u32;
pub const JET_bitRetrieveNull: u32 = 16u32;
pub const JET_bitRetrieveTag: u32 = 8u32;
pub const JET_bitRetrieveTuple: u32 = 2048u32;
pub const JET_bitRollbackAll: u32 = 1u32;
pub const JET_bitSeekEQ: u32 = 1u32;
pub const JET_bitSeekGE: u32 = 8u32;
pub const JET_bitSeekGT: u32 = 16u32;
pub const JET_bitSeekLE: u32 = 4u32;
pub const JET_bitSeekLT: u32 = 2u32;
pub const JET_bitSetAppendLV: u32 = 1u32;
pub const JET_bitSetCompressed: u32 = 131072u32;
pub const JET_bitSetContiguousLV: u32 = 262144u32;
pub const JET_bitSetIndexRange: u32 = 32u32;
pub const JET_bitSetIntrinsicLV: u32 = 1024u32;
pub const JET_bitSetOverwriteLV: u32 = 4u32;
pub const JET_bitSetRevertToDefaultValue: u32 = 512u32;
pub const JET_bitSetSeparateLV: u32 = 64u32;
pub const JET_bitSetSizeLV: u32 = 8u32;
pub const JET_bitSetUncompressed: u32 = 65536u32;
pub const JET_bitSetUniqueMultiValues: u32 = 128u32;
pub const JET_bitSetUniqueNormalizedMultiValues: u32 = 256u32;
pub const JET_bitSetZeroLength: u32 = 32u32;
pub const JET_bitShrinkDatabaseOff: u32 = 0u32;
pub const JET_bitShrinkDatabaseOn: u32 = 1u32;
pub const JET_bitShrinkDatabaseRealtime: u32 = 2u32;
pub const JET_bitShrinkDatabaseTrim: u32 = 1u32;
pub const JET_bitSpaceHintsUtilizeParentSpace: u32 = 1u32;
pub const JET_bitStopServiceAll: u32 = 0u32;
pub const JET_bitStopServiceBackgroundUserTasks: u32 = 2u32;
pub const JET_bitStopServiceQuiesceCaches: u32 = 4u32;
pub const JET_bitStopServiceResume: u32 = 2147483648u32;
pub const JET_bitStrLimit: u32 = 2u32;
pub const JET_bitSubStrLimit: u32 = 4u32;
pub const JET_bitTTDotNetGuid: u32 = 256u32;
pub const JET_bitTTErrorOnDuplicateInsertion: u32 = 32u32;
pub const JET_bitTTForceMaterialization: u32 = 32u32;
pub const JET_bitTTForwardOnly: u32 = 64u32;
pub const JET_bitTTIndexed: u32 = 1u32;
pub const JET_bitTTIntrinsicLVsOnly: u32 = 128u32;
pub const JET_bitTTScrollable: u32 = 8u32;
pub const JET_bitTTSortNullsHigh: u32 = 16u32;
pub const JET_bitTTUnique: u32 = 2u32;
pub const JET_bitTTUpdatable: u32 = 4u32;
pub const JET_bitTableClass1: u32 = 65536u32;
pub const JET_bitTableClass10: u32 = 655360u32;
pub const JET_bitTableClass11: u32 = 720896u32;
pub const JET_bitTableClass12: u32 = 786432u32;
pub const JET_bitTableClass13: u32 = 851968u32;
pub const JET_bitTableClass14: u32 = 917504u32;
pub const JET_bitTableClass15: u32 = 983040u32;
pub const JET_bitTableClass2: u32 = 131072u32;
pub const JET_bitTableClass3: u32 = 196608u32;
pub const JET_bitTableClass4: u32 = 262144u32;
pub const JET_bitTableClass5: u32 = 327680u32;
pub const JET_bitTableClass6: u32 = 393216u32;
pub const JET_bitTableClass7: u32 = 458752u32;
pub const JET_bitTableClass8: u32 = 524288u32;
pub const JET_bitTableClass9: u32 = 589824u32;
pub const JET_bitTableClassMask: u32 = 2031616u32;
pub const JET_bitTableClassNone: u32 = 0u32;
pub const JET_bitTableCreateFixedDDL: u32 = 1u32;
pub const JET_bitTableCreateImmutableStructure: u32 = 8u32;
pub const JET_bitTableCreateNoFixedVarColumnsInDerivedTables: u32 = 4u32;
pub const JET_bitTableCreateTemplateTable: u32 = 2u32;
pub const JET_bitTableDenyRead: u32 = 2u32;
pub const JET_bitTableDenyWrite: u32 = 1u32;
pub const JET_bitTableInfoBookmark: u32 = 2u32;
pub const JET_bitTableInfoRollback: u32 = 4u32;
pub const JET_bitTableInfoUpdatable: u32 = 1u32;
pub const JET_bitTableNoCache: u32 = 32u32;
pub const JET_bitTableOpportuneRead: u32 = 128u32;
pub const JET_bitTablePermitDDL: u32 = 16u32;
pub const JET_bitTablePreread: u32 = 64u32;
pub const JET_bitTableReadOnly: u32 = 4u32;
pub const JET_bitTableSequential: u32 = 32768u32;
pub const JET_bitTableUpdatable: u32 = 8u32;
pub const JET_bitTermAbrupt: u32 = 2u32;
pub const JET_bitTermComplete: u32 = 1u32;
pub const JET_bitTermDirty: u32 = 8u32;
pub const JET_bitTermStopBackup: u32 = 4u32;
pub const JET_bitTransactionReadOnly: u32 = 1u32;
pub const JET_bitTruncateLogsAfterRecovery: u32 = 16u32;
pub const JET_bitUpdateCheckESE97Compatibility: u32 = 1u32;
pub const JET_bitWaitAllLevel0Commit: u32 = 8u32;
pub const JET_bitWaitLastLevel0Commit: u32 = 2u32;
pub const JET_bitWriteLock: u32 = 2u32;
pub const JET_bitZeroLength: u32 = 1u32;
pub const JET_cbBookmarkMost: u32 = 256u32;
pub const JET_cbColumnLVPageOverhead: u32 = 82u32;
pub const JET_cbColumnMost: u32 = 255u32;
pub const JET_cbFullNameMost: u32 = 255u32;
pub const JET_cbKeyMost: u32 = 255u32;
pub const JET_cbKeyMost2KBytePage: u32 = 500u32;
pub const JET_cbKeyMost4KBytePage: u32 = 1000u32;
pub const JET_cbKeyMost8KBytePage: u32 = 2000u32;
pub const JET_cbKeyMostMin: u32 = 255u32;
pub const JET_cbLVColumnMost: u32 = 2147483647u32;
pub const JET_cbLVDefaultValueMost: u32 = 255u32;
pub const JET_cbLimitKeyMost: u32 = 256u32;
pub const JET_cbNameMost: u32 = 64u32;
pub const JET_cbPrimaryKeyMost: u32 = 255u32;
pub const JET_cbSecondaryKeyMost: u32 = 255u32;
pub const JET_cbtypAfterDelete: u32 = 64u32;
pub const JET_cbtypAfterInsert: u32 = 4u32;
pub const JET_cbtypAfterReplace: u32 = 16u32;
pub const JET_cbtypBeforeDelete: u32 = 32u32;
pub const JET_cbtypBeforeInsert: u32 = 2u32;
pub const JET_cbtypBeforeReplace: u32 = 8u32;
pub const JET_cbtypFinalize: u32 = 1u32;
pub const JET_cbtypFreeCursorLS: u32 = 512u32;
pub const JET_cbtypFreeTableLS: u32 = 1024u32;
pub const JET_cbtypNull: u32 = 0u32;
pub const JET_cbtypOnlineDefragCompleted: u32 = 256u32;
pub const JET_cbtypUserDefinedDefaultValue: u32 = 128u32;
pub const JET_ccolFixedMost: u32 = 127u32;
pub const JET_ccolKeyMost: u32 = 16u32;
pub const JET_ccolMost: u32 = 65248u32;
pub const JET_ccolVarMost: u32 = 128u32;
pub const JET_coltypBinary: u32 = 9u32;
pub const JET_coltypBit: u32 = 1u32;
pub const JET_coltypCurrency: u32 = 5u32;
pub const JET_coltypDateTime: u32 = 8u32;
pub const JET_coltypGUID: u32 = 16u32;
pub const JET_coltypIEEEDouble: u32 = 7u32;
pub const JET_coltypIEEESingle: u32 = 6u32;
pub const JET_coltypLong: u32 = 4u32;
pub const JET_coltypLongBinary: u32 = 11u32;
pub const JET_coltypLongLong: u32 = 15u32;
pub const JET_coltypLongText: u32 = 12u32;
pub const JET_coltypMax: u32 = 13u32;
pub const JET_coltypNil: u32 = 0u32;
pub const JET_coltypSLV: u32 = 13u32;
pub const JET_coltypShort: u32 = 3u32;
pub const JET_coltypText: u32 = 10u32;
pub const JET_coltypUnsignedByte: u32 = 2u32;
pub const JET_coltypUnsignedLong: u32 = 14u32;
pub const JET_coltypUnsignedLongLong: u32 = 18u32;
pub const JET_coltypUnsignedShort: u32 = 17u32;
pub const JET_configDefault: u32 = 1u32;
pub const JET_configDynamicMediumMemory: u32 = 32u32;
pub const JET_configHighConcurrencyScaling: u32 = 1024u32;
pub const JET_configLowDiskFootprint: u32 = 4u32;
pub const JET_configLowMemory: u32 = 16u32;
pub const JET_configLowPower: u32 = 64u32;
pub const JET_configMediumDiskFootprint: u32 = 8u32;
pub const JET_configRemoveQuotas: u32 = 2u32;
pub const JET_configRunSilent: u32 = 256u32;
pub const JET_configSSDProfileIO: u32 = 128u32;
pub const JET_configUnthrottledMemory: u32 = 512u32;
pub const JET_dbstateBeingConverted: u32 = 4u32;
pub const JET_dbstateCleanShutdown: u32 = 3u32;
pub const JET_dbstateDirtyShutdown: u32 = 2u32;
pub const JET_dbstateForceDetach: u32 = 5u32;
pub const JET_dbstateJustCreated: u32 = 1u32;
pub const JET_errAccessDenied: i32 = -1907i32;
pub const JET_errAfterInitialization: i32 = -1850i32;
pub const JET_errAlreadyInitialized: i32 = -1030i32;
pub const JET_errAlreadyPrepared: i32 = -1607i32;
pub const JET_errAttachedDatabaseMismatch: i32 = -1216i32;
pub const JET_errBackupAbortByServer: i32 = -801i32;
pub const JET_errBackupDirectoryNotEmpty: i32 = -504i32;
pub const JET_errBackupInProgress: i32 = -505i32;
pub const JET_errBackupNotAllowedYet: i32 = -523i32;
pub const JET_errBadBackupDatabaseSize: i32 = -561i32;
pub const JET_errBadBookmark: i32 = -328i32;
pub const JET_errBadCheckpointSignature: i32 = -532i32;
pub const JET_errBadColumnId: i32 = -1517i32;
pub const JET_errBadDbSignature: i32 = -531i32;
pub const JET_errBadEmptyPage: i32 = -351i32;
pub const JET_errBadItagSequence: i32 = -1518i32;
pub const JET_errBadLineCount: i32 = -354i32;
pub const JET_errBadLogSignature: i32 = -530i32;
pub const JET_errBadLogVersion: i32 = -514i32;
pub const JET_errBadPageLink: i32 = -327i32;
pub const JET_errBadParentPageLink: i32 = -338i32;
pub const JET_errBadPatchPage: i32 = -535i32;
pub const JET_errBadRestoreTargetInstance: i32 = -577i32;
pub const JET_errBufferTooSmall: i32 = -1038i32;
pub const JET_errCallbackFailed: i32 = -2101i32;
pub const JET_errCallbackNotResolved: i32 = -2102i32;
pub const JET_errCannotAddFixedVarColumnToDerivedTable: i32 = -1330i32;
pub const JET_errCannotBeTagged: i32 = -1521i32;
pub const JET_errCannotDeleteSystemTable: i32 = -1318i32;
pub const JET_errCannotDeleteTempTable: i32 = -1317i32;
pub const JET_errCannotDeleteTemplateTable: i32 = -1319i32;
pub const JET_errCannotDisableVersioning: i32 = -1208i32;
pub const JET_errCannotIndex: i32 = -1071i32;
pub const JET_errCannotIndexOnEncryptedColumn: i32 = -1440i32;
pub const JET_errCannotLogDuringRecoveryRedo: i32 = -512i32;
pub const JET_errCannotMaterializeForwardOnlySort: i32 = -1113i32;
pub const JET_errCannotNestDDL: i32 = -1325i32;
pub const JET_errCannotSeparateIntrinsicLV: i32 = -416i32;
pub const JET_errCatalogCorrupted: i32 = -1220i32;
pub const JET_errCheckpointCorrupt: i32 = -533i32;
pub const JET_errCheckpointDepthTooDeep: i32 = -614i32;
pub const JET_errCheckpointFileNotFound: i32 = -542i32;
pub const JET_errClientRequestToStopJetService: i32 = -1329i32;
pub const JET_errColumnCannotBeCompressed: i32 = -1538i32;
pub const JET_errColumnCannotBeEncrypted: i32 = -1439i32;
pub const JET_errColumnDoesNotFit: i32 = -1503i32;
pub const JET_errColumnDuplicate: i32 = -1508i32;
pub const JET_errColumnInRelationship: i32 = -1519i32;
pub const JET_errColumnInUse: i32 = -1046i32;
pub const JET_errColumnIndexed: i32 = -1505i32;
pub const JET_errColumnLong: i32 = -1501i32;
pub const JET_errColumnNoChunk: i32 = -1502i32;
pub const JET_errColumnNoEncryptionKey: i32 = -1540i32;
pub const JET_errColumnNotFound: i32 = -1507i32;
pub const JET_errColumnNotUpdatable: i32 = -1048i32;
pub const JET_errColumnRedundant: i32 = -1510i32;
pub const JET_errColumnTooBig: i32 = -1506i32;
pub const JET_errCommittedLogFileCorrupt: i32 = -586i32;
pub const JET_errCommittedLogFilesMissing: i32 = -582i32;
pub const JET_errConsistentTimeMismatch: i32 = -551i32;
pub const JET_errContainerNotEmpty: i32 = -1043i32;
pub const JET_errDDLNotInheritable: i32 = -1326i32;
pub const JET_errDataHasChanged: i32 = -1611i32;
pub const JET_errDatabase200Format: i32 = -1210i32;
pub const JET_errDatabase400Format: i32 = -1211i32;
pub const JET_errDatabase500Format: i32 = -1212i32;
pub const JET_errDatabaseAlreadyRunningMaintenance: i32 = -2004i32;
pub const JET_errDatabaseAlreadyUpgraded: i32 = -562i32;
pub const JET_errDatabaseAttachedForRecovery: i32 = -1231i32;
pub const JET_errDatabaseBufferDependenciesCorrupted: i32 = -255i32;
pub const JET_errDatabaseCorrupted: i32 = -1206i32;
pub const JET_errDatabaseCorruptedNoRepair: i32 = -1224i32;
pub const JET_errDatabaseDirtyShutdown: i32 = -550i32;
pub const JET_errDatabaseDuplicate: i32 = -1201i32;
pub const JET_errDatabaseFileReadOnly: i32 = -1008i32;
pub const JET_errDatabaseIdInUse: i32 = -1218i32;
pub const JET_errDatabaseInUse: i32 = -1202i32;
pub const JET_errDatabaseIncompleteUpgrade: i32 = -563i32;
pub const JET_errDatabaseInconsistent: i32 = -550i32;
pub const JET_errDatabaseInvalidName: i32 = -1204i32;
pub const JET_errDatabaseInvalidPages: i32 = -1205i32;
pub const JET_errDatabaseInvalidPath: i32 = -1217i32;
pub const JET_errDatabaseLeakInSpace: i32 = -348i32;
pub const JET_errDatabaseLocked: i32 = -1207i32;
pub const JET_errDatabaseLogSetMismatch: i32 = -539i32;
pub const JET_errDatabaseNotFound: i32 = -1203i32;
pub const JET_errDatabaseNotReady: i32 = -1230i32;
pub const JET_errDatabasePatchFileMismatch: i32 = -552i32;
pub const JET_errDatabaseSharingViolation: i32 = -1215i32;
pub const JET_errDatabaseSignInUse: i32 = -1222i32;
pub const JET_errDatabaseStreamingFileMismatch: i32 = -540i32;
pub const JET_errDatabaseUnavailable: i32 = -1091i32;
pub const JET_errDatabasesNotFromSameSnapshot: i32 = -580i32;
pub const JET_errDbTimeCorrupted: i32 = -344i32;
pub const JET_errDbTimeTooNew: i32 = -567i32;
pub const JET_errDbTimeTooOld: i32 = -566i32;
pub const JET_errDecompressionFailed: i32 = -1620i32;
pub const JET_errDecryptionFailed: i32 = -1622i32;
pub const JET_errDefaultValueTooBig: i32 = -1524i32;
pub const JET_errDeleteBackupFileFail: i32 = -524i32;
pub const JET_errDensityInvalid: i32 = -1307i32;
pub const JET_errDerivedColumnCorruption: i32 = -1529i32;
pub const JET_errDirtyShutdown: i32 = -1116i32;
pub const JET_errDisabledFunctionality: i32 = -112i32;
pub const JET_errDiskFull: i32 = -1808i32;
pub const JET_errDiskIO: i32 = -1022i32;
pub const JET_errDiskReadVerificationFailure: i32 = -1021i32;
pub const JET_errEncryptionBadItag: i32 = -1623i32;
pub const JET_errEndingRestoreLogTooLow: i32 = -553i32;
pub const JET_errEngineFormatVersionNoLongerSupportedTooLow: i32 = -619i32;
pub const JET_errEngineFormatVersionNotYetImplementedTooHigh: i32 = -620i32;
pub const JET_errEngineFormatVersionParamTooLowForRequestedFeature: i32 = -621i32;
pub const JET_errEngineFormatVersionSpecifiedTooLowForDatabaseVersion: i32 = -623i32;
pub const JET_errEngineFormatVersionSpecifiedTooLowForLogVersion: i32 = -622i32;
pub const JET_errEntryPointNotFound: i32 = -1911i32;
pub const JET_errExclusiveTableLockRequired: i32 = -1322i32;
pub const JET_errExistingLogFileHasBadSignature: i32 = -610i32;
pub const JET_errExistingLogFileIsNotContiguous: i32 = -611i32;
pub const JET_errFeatureNotAvailable: i32 = -1001i32;
pub const JET_errFileAccessDenied: i32 = -1032i32;
pub const JET_errFileAlreadyExists: i32 = -1814i32;
pub const JET_errFileClose: i32 = -102i32;
pub const JET_errFileCompressed: i32 = -4005i32;
pub const JET_errFileIOAbort: i32 = -4002i32;
pub const JET_errFileIOBeyondEOF: i32 = -4001i32;
pub const JET_errFileIOFail: i32 = -4004i32;
pub const JET_errFileIORetry: i32 = -4003i32;
pub const JET_errFileIOSparse: i32 = -4000i32;
pub const JET_errFileInvalidType: i32 = -1812i32;
pub const JET_errFileNotFound: i32 = -1811i32;
pub const JET_errFileSystemCorruption: i32 = -1121i32;
pub const JET_errFilteredMoveNotSupported: i32 = -1124i32;
pub const JET_errFixedDDL: i32 = -1323i32;
pub const JET_errFixedInheritedDDL: i32 = -1324i32;
pub const JET_errFlushMapDatabaseMismatch: i32 = -1919i32;
pub const JET_errFlushMapUnrecoverable: i32 = -1920i32;
pub const JET_errFlushMapVersionUnsupported: i32 = -1918i32;
pub const JET_errForceDetachNotAllowed: i32 = -1219i32;
pub const JET_errGivenLogFileHasBadSignature: i32 = -555i32;
pub const JET_errGivenLogFileIsNotContiguous: i32 = -556i32;
pub const JET_errIllegalOperation: i32 = -1312i32;
pub const JET_errInTransaction: i32 = -1108i32;
pub const JET_errIndexBuildCorrupted: i32 = -1412i32;
pub const JET_errIndexCantBuild: i32 = -1401i32;
pub const JET_errIndexDuplicate: i32 = -1403i32;
pub const JET_errIndexHasPrimary: i32 = -1402i32;
pub const JET_errIndexInUse: i32 = -1051i32;
pub const JET_errIndexInvalidDef: i32 = -1406i32;
pub const JET_errIndexMustStay: i32 = -1405i32;
pub const JET_errIndexNotFound: i32 = -1404i32;
pub const JET_errIndexTuplesCannotRetrieveFromIndex: i32 = -1436i32;
pub const JET_errIndexTuplesInvalidLimits: i32 = -1435i32;
pub const JET_errIndexTuplesKeyTooSmall: i32 = -1437i32;
pub const JET_errIndexTuplesNonUniqueOnly: i32 = -1432i32;
pub const JET_errIndexTuplesOneColumnOnly: i32 = -1431i32;
pub const JET_errIndexTuplesSecondaryIndexOnly: i32 = -1430i32;
pub const JET_errIndexTuplesTextBinaryColumnsOnly: i32 = -1433i32;
pub const JET_errIndexTuplesTextColumnsOnly: i32 = -1433i32;
pub const JET_errIndexTuplesTooManyColumns: i32 = -1431i32;
pub const JET_errIndexTuplesVarSegMacNotAllowed: i32 = -1434i32;
pub const JET_errInitInProgress: i32 = -1031i32;
pub const JET_errInstanceNameInUse: i32 = -1086i32;
pub const JET_errInstanceUnavailable: i32 = -1090i32;
pub const JET_errInstanceUnavailableDueToFatalLogDiskFull: i32 = -1092i32;
pub const JET_errInternalError: i32 = -107i32;
pub const JET_errInvalidBackup: i32 = -526i32;
pub const JET_errInvalidBackupSequence: i32 = -521i32;
pub const JET_errInvalidBookmark: i32 = -1045i32;
pub const JET_errInvalidBufferSize: i32 = -1047i32;
pub const JET_errInvalidCodePage: i32 = -1063i32;
pub const JET_errInvalidColumnType: i32 = -1511i32;
pub const JET_errInvalidCountry: i32 = -1061i32;
pub const JET_errInvalidCreateDbVersion: i32 = -1225i32;
pub const JET_errInvalidCreateIndex: i32 = -1409i32;
pub const JET_errInvalidDatabase: i32 = -1028i32;
pub const JET_errInvalidDatabaseId: i32 = -1010i32;
pub const JET_errInvalidDatabaseVersion: i32 = -1209i32;
pub const JET_errInvalidDbparamId: i32 = -1095i32;
pub const JET_errInvalidFilename: i32 = -1044i32;
pub const JET_errInvalidGrbit: i32 = -900i32;
pub const JET_errInvalidIndexId: i32 = -1416i32;
pub const JET_errInvalidInstance: i32 = -1115i32;
pub const JET_errInvalidLCMapStringFlags: i32 = -1064i32;
pub const JET_errInvalidLVChunkSize: i32 = -1438i32;
pub const JET_errInvalidLanguageId: i32 = -1062i32;
pub const JET_errInvalidLogDirectory: i32 = -1025i32;
pub const JET_errInvalidLogSequence: i32 = -515i32;
pub const JET_errInvalidLoggedOperation: i32 = -500i32;
pub const JET_errInvalidName: i32 = -1002i32;
pub const JET_errInvalidObject: i32 = -1316i32;
pub const JET_errInvalidOnSort: i32 = -1702i32;
pub const JET_errInvalidOperation: i32 = -1906i32;
pub const JET_errInvalidParameter: i32 = -1003i32;
pub const JET_errInvalidPath: i32 = -1023i32;
pub const JET_errInvalidPlaceholderColumn: i32 = -1530i32;
pub const JET_errInvalidPreread: i32 = -424i32;
pub const JET_errInvalidSesid: i32 = -1104i32;
pub const JET_errInvalidSesparamId: i32 = -1093i32;
pub const JET_errInvalidSettings: i32 = -1328i32;
pub const JET_errInvalidSystemPath: i32 = -1024i32;
pub const JET_errInvalidTableId: i32 = -1310i32;
pub const JET_errKeyBoundary: i32 = -324i32;
pub const JET_errKeyDuplicate: i32 = -1605i32;
pub const JET_errKeyIsMade: i32 = -1516i32;
pub const JET_errKeyNotMade: i32 = -1608i32;
pub const JET_errKeyTooBig: i32 = -408i32;
pub const JET_errKeyTruncated: i32 = -346i32;
pub const JET_errLSAlreadySet: i32 = -3001i32;
pub const JET_errLSCallbackNotSpecified: i32 = -3000i32;
pub const JET_errLSNotSet: i32 = -3002i32;
pub const JET_errLVCorrupted: i32 = -1526i32;
pub const JET_errLanguageNotSupported: i32 = -1619i32;
pub const JET_errLinkNotSupported: i32 = -1052i32;
pub const JET_errLogBufferTooSmall: i32 = -517i32;
pub const JET_errLogCorruptDuringHardRecovery: i32 = -574i32;
pub const JET_errLogCorruptDuringHardRestore: i32 = -573i32;
pub const JET_errLogCorrupted: i32 = -1852i32;
pub const JET_errLogDisabledDueToRecoveryFailure: i32 = -511i32;
pub const JET_errLogDiskFull: i32 = -529i32;
pub const JET_errLogFileCorrupt: i32 = -501i32;
pub const JET_errLogFileNotCopied: i32 = -616i32;
pub const JET_errLogFilePathInUse: i32 = -1084i32;
pub const JET_errLogFileSizeMismatch: i32 = -541i32;
pub const JET_errLogFileSizeMismatchDatabasesConsistent: i32 = -545i32;
pub const JET_errLogGenerationMismatch: i32 = -513i32;
pub const JET_errLogReadVerifyFailure: i32 = -612i32;
pub const JET_errLogSectorSizeMismatch: i32 = -546i32;
pub const JET_errLogSectorSizeMismatchDatabasesConsistent: i32 = -547i32;
pub const JET_errLogSequenceChecksumMismatch: i32 = -590i32;
pub const JET_errLogSequenceEnd: i32 = -519i32;
pub const JET_errLogSequenceEndDatabasesConsistent: i32 = -548i32;
pub const JET_errLogTornWriteDuringHardRecovery: i32 = -571i32;
pub const JET_errLogTornWriteDuringHardRestore: i32 = -570i32;
pub const JET_errLogWriteFail: i32 = -510i32;
pub const JET_errLoggingDisabled: i32 = -516i32;
pub const JET_errMakeBackupDirectoryFail: i32 = -525i32;
pub const JET_errMissingCurrentLogFiles: i32 = -565i32;
pub const JET_errMissingFileToBackup: i32 = -569i32;
pub const JET_errMissingFullBackup: i32 = -560i32;
pub const JET_errMissingLogFile: i32 = -528i32;
pub const JET_errMissingPatchPage: i32 = -534i32;
pub const JET_errMissingPreviousLogFile: i32 = -509i32;
pub const JET_errMissingRestoreLogFiles: i32 = -557i32;
pub const JET_errMultiValuedColumnMustBeTagged: i32 = -1509i32;
pub const JET_errMultiValuedDuplicate: i32 = -1525i32;
pub const JET_errMultiValuedDuplicateAfterTruncation: i32 = -1528i32;
pub const JET_errMultiValuedIndexViolation: i32 = -1411i32;
pub const JET_errMustBeSeparateLongValue: i32 = -423i32;
pub const JET_errMustDisableLoggingForDbUpgrade: i32 = -575i32;
pub const JET_errMustRollback: i32 = -1057i32;
pub const JET_errNTSystemCallFailed: i32 = -334i32;
pub const JET_errNoBackup: i32 = -520i32;
pub const JET_errNoBackupDirectory: i32 = -503i32;
pub const JET_errNoCurrentIndex: i32 = -1515i32;
pub const JET_errNoCurrentRecord: i32 = -1603i32;
pub const JET_errNodeCorrupted: i32 = -358i32;
pub const JET_errNotInTransaction: i32 = -1054i32;
pub const JET_errNotInitialized: i32 = -1029i32;
pub const JET_errNullInvalid: i32 = -1504i32;
pub const JET_errNullKeyDisallowed: i32 = -1053i32;
pub const JET_errOSSnapshotInvalidSequence: i32 = -2401i32;
pub const JET_errOSSnapshotInvalidSnapId: i32 = -2404i32;
pub const JET_errOSSnapshotNotAllowed: i32 = -2403i32;
pub const JET_errOSSnapshotTimeOut: i32 = -2402i32;
pub const JET_errObjectDuplicate: i32 = -1314i32;
pub const JET_errObjectNotFound: i32 = -1305i32;
pub const JET_errOneDatabasePerSession: i32 = -1916i32;
pub const JET_errOutOfAutoincrementValues: i32 = -1076i32;
pub const JET_errOutOfBuffers: i32 = -1014i32;
pub const JET_errOutOfCursors: i32 = -1013i32;
pub const JET_errOutOfDatabaseSpace: i32 = -1012i32;
pub const JET_errOutOfDbtimeValues: i32 = -1077i32;
pub const JET_errOutOfFileHandles: i32 = -1020i32;
pub const JET_errOutOfLongValueIDs: i32 = -1075i32;
pub const JET_errOutOfMemory: i32 = -1011i32;
pub const JET_errOutOfObjectIDs: i32 = -1074i32;
pub const JET_errOutOfSequentialIndexValues: i32 = -1078i32;
pub const JET_errOutOfSessions: i32 = -1101i32;
pub const JET_errOutOfThreads: i32 = -103i32;
pub const JET_errPageBoundary: i32 = -323i32;
pub const JET_errPageInitializedMismatch: i32 = -596i32;
pub const JET_errPageNotInitialized: i32 = -1019i32;
pub const JET_errPageSizeMismatch: i32 = -1213i32;
pub const JET_errPageTagCorrupted: i32 = -357i32;
pub const JET_errPartiallyAttachedDB: i32 = -1221i32;
pub const JET_errPatchFileMissing: i32 = -538i32;
pub const JET_errPermissionDenied: i32 = -1809i32;
pub const JET_errPreviousVersion: i32 = -322i32;
pub const JET_errPrimaryIndexCorrupted: i32 = -1413i32;
pub const JET_errReadLostFlushVerifyFailure: i32 = -1119i32;
pub const JET_errReadPgnoVerifyFailure: i32 = -1118i32;
pub const JET_errReadVerifyFailure: i32 = -1018i32;
pub const JET_errRecordDeleted: i32 = -1017i32;
pub const JET_errRecordFormatConversionFailed: i32 = -1915i32;
pub const JET_errRecordNoCopy: i32 = -1602i32;
pub const JET_errRecordNotDeleted: i32 = -1072i32;
pub const JET_errRecordNotFound: i32 = -1601i32;
pub const JET_errRecordPrimaryChanged: i32 = -1604i32;
pub const JET_errRecordTooBig: i32 = -1026i32;
pub const JET_errRecordTooBigForBackwardCompatibility: i32 = -1112i32;
pub const JET_errRecoveredWithErrors: i32 = -527i32;
pub const JET_errRecoveredWithoutUndo: i32 = -579i32;
pub const JET_errRecoveredWithoutUndoDatabasesConsistent: i32 = -584i32;
pub const JET_errRecoveryVerifyFailure: i32 = -1123i32;
pub const JET_errRedoAbruptEnded: i32 = -536i32;
pub const JET_errRequiredLogFilesMissing: i32 = -543i32;
pub const JET_errRestoreInProgress: i32 = -506i32;
pub const JET_errRestoreOfNonBackupDatabase: i32 = -615i32;
pub const JET_errRfsFailure: i32 = -100i32;
pub const JET_errRfsNotArmed: i32 = -101i32;
pub const JET_errRollbackError: i32 = -1917i32;
pub const JET_errRollbackRequired: i32 = -1109i32;
pub const JET_errRunningInMultiInstanceMode: i32 = -1081i32;
pub const JET_errRunningInOneInstanceMode: i32 = -1080i32;
pub const JET_errSPAvailExtCacheOutOfMemory: i32 = -342i32;
pub const JET_errSPAvailExtCacheOutOfSync: i32 = -340i32;
pub const JET_errSPAvailExtCorrupted: i32 = -341i32;
pub const JET_errSPOwnExtCorrupted: i32 = -343i32;
pub const JET_errSecondaryIndexCorrupted: i32 = -1414i32;
pub const JET_errSectorSizeNotSupported: i32 = -583i32;
pub const JET_errSeparatedLongValue: i32 = -421i32;
pub const JET_errSesidTableIdMismatch: i32 = -1114i32;
pub const JET_errSessionContextAlreadySet: i32 = -1912i32;
pub const JET_errSessionContextNotSetByThisThread: i32 = -1913i32;
pub const JET_errSessionInUse: i32 = -1914i32;
pub const JET_errSessionSharingViolation: i32 = -1910i32;
pub const JET_errSessionWriteConflict: i32 = -1111i32;
pub const JET_errSoftRecoveryOnBackupDatabase: i32 = -544i32;
pub const JET_errSoftRecoveryOnSnapshot: i32 = -581i32;
pub const JET_errSpaceHintsInvalid: i32 = -2103i32;
pub const JET_errStartingRestoreLogTooHigh: i32 = -554i32;
pub const JET_errStreamingDataNotLogged: i32 = -549i32;
pub const JET_errSuccess: u32 = 0u32;
pub const JET_errSystemParameterConflict: i32 = -1087i32;
pub const JET_errSystemParamsAlreadySet: i32 = -1082i32;
pub const JET_errSystemPathInUse: i32 = -1083i32;
pub const JET_errTableDuplicate: i32 = -1303i32;
pub const JET_errTableInUse: i32 = -1304i32;
pub const JET_errTableLocked: i32 = -1302i32;
pub const JET_errTableNotEmpty: i32 = -1308i32;
pub const JET_errTaggedNotNULL: i32 = -1514i32;
pub const JET_errTaskDropped: i32 = -106i32;
pub const JET_errTempFileOpenError: i32 = -1803i32;
pub const JET_errTempPathInUse: i32 = -1085i32;
pub const JET_errTermInProgress: i32 = -1000i32;
pub const JET_errTooManyActiveUsers: i32 = -1059i32;
pub const JET_errTooManyAttachedDatabases: i32 = -1805i32;
pub const JET_errTooManyColumns: i32 = -1040i32;
pub const JET_errTooManyIO: i32 = -105i32;
pub const JET_errTooManyIndexes: i32 = -1015i32;
pub const JET_errTooManyInstances: i32 = -1214i32;
pub const JET_errTooManyKeys: i32 = -1016i32;
pub const JET_errTooManyMempoolEntries: i32 = -1073i32;
pub const JET_errTooManyOpenDatabases: i32 = -1027i32;
pub const JET_errTooManyOpenIndexes: i32 = -1410i32;
pub const JET_errTooManyOpenTables: i32 = -1311i32;
pub const JET_errTooManyOpenTablesAndCleanupTimedOut: i32 = -1313i32;
pub const JET_errTooManyRecords: i32 = -1094i32;
pub const JET_errTooManySorts: i32 = -1701i32;
pub const JET_errTooManySplits: i32 = -1909i32;
pub const JET_errTransReadOnly: i32 = -1110i32;
pub const JET_errTransTooDeep: i32 = -1103i32;
pub const JET_errTransactionTooLong: i32 = -618i32;
pub const JET_errTransactionsNotReadyDuringRecovery: i32 = -1232i32;
pub const JET_errUnicodeLanguageValidationFailure: i32 = -604i32;
pub const JET_errUnicodeNormalizationNotSupported: i32 = -603i32;
pub const JET_errUnicodeTranslationBufferTooSmall: i32 = -601i32;
pub const JET_errUnicodeTranslationFail: i32 = -602i32;
pub const JET_errUnloadableOSFunctionality: i32 = -113i32;
pub const JET_errUpdateMustVersion: i32 = -1621i32;
pub const JET_errUpdateNotPrepared: i32 = -1609i32;
pub const JET_errVersionStoreEntryTooBig: i32 = -1065i32;
pub const JET_errVersionStoreOutOfMemory: i32 = -1069i32;
pub const JET_errVersionStoreOutOfMemoryAndCleanupTimedOut: i32 = -1066i32;
pub const JET_errWriteConflict: i32 = -1102i32;
pub const JET_errWriteConflictPrimaryIndex: i32 = -1105i32;
pub const JET_filetypeCheckpoint: u32 = 4u32;
pub const JET_filetypeDatabase: u32 = 1u32;
pub const JET_filetypeFlushMap: u32 = 7u32;
pub const JET_filetypeLog: u32 = 3u32;
pub const JET_filetypeTempDatabase: u32 = 5u32;
pub const JET_filetypeUnknown: u32 = 0u32;
pub const JET_objtypNil: u32 = 0u32;
pub const JET_objtypTable: u32 = 1u32;
pub const JET_paramAccessDeniedRetryPeriod: u32 = 53u32;
pub const JET_paramAlternateDatabaseRecoveryPath: u32 = 113u32;
pub const JET_paramBaseName: u32 = 3u32;
pub const JET_paramBatchIOBufferMax: u32 = 22u32;
pub const JET_paramCachePriority: u32 = 177u32;
pub const JET_paramCacheSize: u32 = 41u32;
pub const JET_paramCacheSizeMax: u32 = 23u32;
pub const JET_paramCacheSizeMin: u32 = 60u32;
pub const JET_paramCachedClosedTables: u32 = 125u32;
pub const JET_paramCheckFormatWhenOpenFail: u32 = 44u32;
pub const JET_paramCheckpointDepthMax: u32 = 24u32;
pub const JET_paramCheckpointIOMax: u32 = 135u32;
pub const JET_paramCircularLog: u32 = 17u32;
pub const JET_paramCleanupMismatchedLogFiles: u32 = 77u32;
pub const JET_paramCommitDefault: u32 = 16u32;
pub const JET_paramConfigStoreSpec: u32 = 189u32;
pub const JET_paramConfiguration: u32 = 129u32;
pub const JET_paramCreatePathIfNotExist: u32 = 100u32;
pub const JET_paramDatabasePageSize: u32 = 64u32;
pub const JET_paramDbExtensionSize: u32 = 18u32;
pub const JET_paramDbScanIntervalMaxSec: u32 = 172u32;
pub const JET_paramDbScanIntervalMinSec: u32 = 171u32;
pub const JET_paramDbScanThrottle: u32 = 170u32;
pub const JET_paramDefragmentSequentialBTrees: u32 = 160u32;
pub const JET_paramDefragmentSequentialBTreesDensityCheckFrequency: u32 = 161u32;
pub const JET_paramDeleteOldLogs: u32 = 48u32;
pub const JET_paramDeleteOutOfRangeLogs: u32 = 52u32;
pub const JET_paramDisableCallbacks: u32 = 65u32;
pub const JET_paramDisablePerfmon: u32 = 107u32;
pub const JET_paramDurableCommitCallback: u32 = 187u32;
pub const JET_paramEnableAdvanced: u32 = 130u32;
pub const JET_paramEnableDBScanInRecovery: u32 = 169u32;
pub const JET_paramEnableDBScanSerialization: u32 = 180u32;
pub const JET_paramEnableFileCache: u32 = 126u32;
pub const JET_paramEnableIndexChecking: u32 = 45u32;
pub const JET_paramEnableIndexCleanup: u32 = 54u32;
pub const JET_paramEnableOnlineDefrag: u32 = 35u32;
pub const JET_paramEnablePersistedCallbacks: u32 = 156u32;
pub const JET_paramEnableRBS: u32 = 215u32;
pub const JET_paramEnableShrinkDatabase: u32 = 184u32;
pub const JET_paramEnableSqm: u32 = 188u32;
pub const JET_paramEnableTempTableVersioning: u32 = 46u32;
pub const JET_paramEnableViewCache: u32 = 127u32;
pub const JET_paramErrorToString: u32 = 70u32;
pub const JET_paramEventLogCache: u32 = 99u32;
pub const JET_paramEventLoggingLevel: u32 = 51u32;
pub const JET_paramEventSource: u32 = 4u32;
pub const JET_paramEventSourceKey: u32 = 49u32;
pub const JET_paramExceptionAction: u32 = 98u32;
pub const JET_paramGlobalMinVerPages: u32 = 81u32;
pub const JET_paramHungIOActions: u32 = 182u32;
pub const JET_paramHungIOThreshold: u32 = 181u32;
pub const JET_paramIOPriority: u32 = 152u32;
pub const JET_paramIOThrottlingTimeQuanta: u32 = 162u32;
pub const JET_paramIgnoreLogVersion: u32 = 47u32;
pub const JET_paramIndexTupleIncrement: u32 = 132u32;
pub const JET_paramIndexTupleStart: u32 = 133u32;
pub const JET_paramIndexTuplesLengthMax: u32 = 111u32;
pub const JET_paramIndexTuplesLengthMin: u32 = 110u32;
pub const JET_paramIndexTuplesToIndexMax: u32 = 112u32;
pub const JET_paramKeyMost: u32 = 134u32;
pub const JET_paramLRUKCorrInterval: u32 = 25u32;
pub const JET_paramLRUKHistoryMax: u32 = 26u32;
pub const JET_paramLRUKPolicy: u32 = 27u32;
pub const JET_paramLRUKTimeout: u32 = 28u32;
pub const JET_paramLRUKTrxCorrInterval: u32 = 29u32;
pub const JET_paramLVChunkSizeMost: u32 = 163u32;
pub const JET_paramLegacyFileNames: u32 = 136u32;
pub const JET_paramLogBuffers: u32 = 12u32;
pub const JET_paramLogCheckpointPeriod: u32 = 14u32;
pub const JET_paramLogFileCreateAsynch: u32 = 69u32;
pub const JET_paramLogFilePath: u32 = 2u32;
pub const JET_paramLogFileSize: u32 = 11u32;
pub const JET_paramLogWaitingUserMax: u32 = 15u32;
pub const JET_paramMaxCoalesceReadGapSize: u32 = 166u32;
pub const JET_paramMaxCoalesceReadSize: u32 = 164u32;
pub const JET_paramMaxCoalesceWriteGapSize: u32 = 167u32;
pub const JET_paramMaxCoalesceWriteSize: u32 = 165u32;
pub const JET_paramMaxColtyp: u32 = 131u32;
pub const JET_paramMaxCursors: u32 = 8u32;
pub const JET_paramMaxInstances: u32 = 104u32;
pub const JET_paramMaxOpenTables: u32 = 6u32;
pub const JET_paramMaxSessions: u32 = 5u32;
pub const JET_paramMaxTemporaryTables: u32 = 10u32;
pub const JET_paramMaxTransactionSize: u32 = 178u32;
pub const JET_paramMaxValueInvalid: u32 = 217u32;
pub const JET_paramMaxVerPages: u32 = 9u32;
pub const JET_paramMinDataForXpress: u32 = 183u32;
pub const JET_paramNoInformationEvent: u32 = 50u32;
pub const JET_paramOSSnapshotTimeout: u32 = 82u32;
pub const JET_paramOneDatabasePerSession: u32 = 102u32;
pub const JET_paramOutstandingIOMax: u32 = 30u32;
pub const JET_paramPageFragment: u32 = 20u32;
pub const JET_paramPageHintCacheSize: u32 = 101u32;
pub const JET_paramPageTempDBMin: u32 = 19u32;
pub const JET_paramPreferredMaxOpenTables: u32 = 7u32;
pub const JET_paramPreferredVerPages: u32 = 63u32;
pub const JET_paramPrereadIOMax: u32 = 179u32;
pub const JET_paramProcessFriendlyName: u32 = 186u32;
pub const JET_paramRBSFilePath: u32 = 216u32;
pub const JET_paramRecordUpgradeDirtyLevel: u32 = 78u32;
pub const JET_paramRecovery: u32 = 34u32;
pub const JET_paramRuntimeCallback: u32 = 73u32;
pub const JET_paramStartFlushThreshold: u32 = 31u32;
pub const JET_paramStopFlushThreshold: u32 = 32u32;
pub const JET_paramSystemPath: u32 = 0u32;
pub const JET_paramTableClass10Name: u32 = 146u32;
pub const JET_paramTableClass11Name: u32 = 147u32;
pub const JET_paramTableClass12Name: u32 = 148u32;
pub const JET_paramTableClass13Name: u32 = 149u32;
pub const JET_paramTableClass14Name: u32 = 150u32;
pub const JET_paramTableClass15Name: u32 = 151u32;
pub const JET_paramTableClass1Name: u32 = 137u32;
pub const JET_paramTableClass2Name: u32 = 138u32;
pub const JET_paramTableClass3Name: u32 = 139u32;
pub const JET_paramTableClass4Name: u32 = 140u32;
pub const JET_paramTableClass5Name: u32 = 141u32;
pub const JET_paramTableClass6Name: u32 = 142u32;
pub const JET_paramTableClass7Name: u32 = 143u32;
pub const JET_paramTableClass8Name: u32 = 144u32;
pub const JET_paramTableClass9Name: u32 = 145u32;
pub const JET_paramTempPath: u32 = 1u32;
pub const JET_paramUnicodeIndexDefault: u32 = 72u32;
pub const JET_paramUseFlushForWriteDurability: u32 = 214u32;
pub const JET_paramVerPageSize: u32 = 128u32;
pub const JET_paramVersionStoreTaskQueueMax: u32 = 105u32;
pub const JET_paramWaitLogFlush: u32 = 13u32;
pub const JET_paramWaypointLatency: u32 = 153u32;
pub const JET_paramZeroDatabaseDuringBackup: u32 = 71u32;
pub const JET_prepCancel: u32 = 3u32;
pub const JET_prepInsert: u32 = 0u32;
pub const JET_prepInsertCopy: u32 = 5u32;
pub const JET_prepInsertCopyDeleteOriginal: u32 = 7u32;
pub const JET_prepInsertCopyReplaceOriginal: u32 = 9u32;
pub const JET_prepReplace: u32 = 2u32;
pub const JET_prepReplaceNoLock: u32 = 4u32;
pub const JET_revertstateCompleted: u32 = 3u32;
pub const JET_revertstateCopingLogs: u32 = 2u32;
pub const JET_revertstateInProgress: u32 = 1u32;
pub const JET_revertstateNone: u32 = 0u32;
pub const JET_sesparamCommitDefault: u32 = 4097u32;
pub const JET_sesparamCorrelationID: u32 = 4101u32;
pub const JET_sesparamMaxValueInvalid: u32 = 4110u32;
pub const JET_sesparamOperationContext: u32 = 4100u32;
pub const JET_sesparamTransactionLevel: u32 = 4099u32;
pub const JET_snpBackup: u32 = 9u32;
pub const JET_snpCompact: u32 = 4u32;
pub const JET_snpRepair: u32 = 2u32;
pub const JET_snpRestore: u32 = 8u32;
pub const JET_snpScrub: u32 = 11u32;
pub const JET_snpUpgrade: u32 = 10u32;
pub const JET_snpUpgradeRecordFormat: u32 = 12u32;
pub const JET_sntBegin: u32 = 5u32;
pub const JET_sntComplete: u32 = 6u32;
pub const JET_sntFail: u32 = 3u32;
pub const JET_sntProgress: u32 = 0u32;
pub const JET_sntRequirements: u32 = 7u32;
pub const JET_sqmDisable: u32 = 0u32;
pub const JET_sqmEnable: u32 = 1u32;
pub const JET_sqmFromCEIP: u32 = 2u32;
pub const JET_wrnBufferTruncated: u32 = 1006u32;
pub const JET_wrnCallbackNotRegistered: u32 = 2100u32;
pub const JET_wrnColumnDefault: u32 = 1537u32;
pub const JET_wrnColumnMaxTruncated: u32 = 1512u32;
pub const JET_wrnColumnMoreTags: u32 = 1533u32;
pub const JET_wrnColumnNotInRecord: u32 = 1539u32;
pub const JET_wrnColumnNotLocal: u32 = 1532u32;
pub const JET_wrnColumnNull: u32 = 1004u32;
pub const JET_wrnColumnPresent: u32 = 1535u32;
pub const JET_wrnColumnReference: u32 = 1541u32;
pub const JET_wrnColumnSetNull: u32 = 1068u32;
pub const JET_wrnColumnSingleValue: u32 = 1536u32;
pub const JET_wrnColumnSkipped: u32 = 1531u32;
pub const JET_wrnColumnTruncated: u32 = 1534u32;
pub const JET_wrnCommittedLogFilesLost: u32 = 585u32;
pub const JET_wrnCommittedLogFilesRemoved: u32 = 587u32;
pub const JET_wrnCopyLongValue: u32 = 1520u32;
pub const JET_wrnCorruptIndexDeleted: u32 = 1415u32;
pub const JET_wrnDataHasChanged: u32 = 1610u32;
pub const JET_wrnDatabaseAttached: u32 = 1007u32;
pub const JET_wrnDatabaseRepaired: u32 = 595u32;
pub const JET_wrnDefragAlreadyRunning: u32 = 2000u32;
pub const JET_wrnDefragNotRunning: u32 = 2001u32;
pub const JET_wrnExistingLogFileHasBadSignature: u32 = 558u32;
pub const JET_wrnExistingLogFileIsNotContiguous: u32 = 559u32;
pub const JET_wrnFileOpenReadOnly: u32 = 1813u32;
pub const JET_wrnFinishWithUndo: u32 = 588u32;
pub const JET_wrnIdleFull: u32 = 1908u32;
pub const JET_wrnKeyChanged: u32 = 1618u32;
pub const JET_wrnNoErrorInfo: u32 = 1055u32;
pub const JET_wrnNoIdleActivity: u32 = 1058u32;
pub const JET_wrnNoWriteLock: u32 = 1067u32;
pub const JET_wrnNyi: i32 = -1i32;
pub const JET_wrnPrimaryIndexOutOfDate: u32 = 1417u32;
pub const JET_wrnRemainingVersions: u32 = 321u32;
pub const JET_wrnSecondaryIndexOutOfDate: u32 = 1418u32;
pub const JET_wrnSeekNotEqual: u32 = 1039u32;
pub const JET_wrnSeparateLongValue: u32 = 406u32;
pub const JET_wrnShrinkNotPossible: u32 = 1122u32;
pub const JET_wrnSkipThisRecord: u32 = 564u32;
pub const JET_wrnSortOverflow: u32 = 1009u32;
pub const JET_wrnTableEmpty: u32 = 1301u32;
pub const JET_wrnTableInUseBySystem: u32 = 1327u32;
pub const JET_wrnTargetInstanceRunning: u32 = 578u32;
pub const JET_wrnUniqueKey: u32 = 345u32;
pub unsafe fn JetAddColumnA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szcolumnname: *const i8,
    pcolumndef: *const JET_COLUMNDEF,
    pvdefault: *const ::std::ffi::c_void,
    cbdefault: u32,
    pcolumnid: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetAddColumnA(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szcolumnname: *const i8,
                pcolumndef: *const JET_COLUMNDEF,
                pvdefault: *const ::std::ffi::c_void,
                cbdefault: u32,
                pcolumnid: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetAddColumnA(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szcolumnname),
            ::std::mem::transmute(pcolumndef),
            ::std::mem::transmute(pvdefault),
            ::std::mem::transmute(cbdefault),
            ::std::mem::transmute(pcolumnid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetAddColumnW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szcolumnname: *const u16,
    pcolumndef: *const JET_COLUMNDEF,
    pvdefault: *const ::std::ffi::c_void,
    cbdefault: u32,
    pcolumnid: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetAddColumnW(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szcolumnname: *const u16,
                pcolumndef: *const JET_COLUMNDEF,
                pvdefault: *const ::std::ffi::c_void,
                cbdefault: u32,
                pcolumnid: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetAddColumnW(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szcolumnname),
            ::std::mem::transmute(pcolumndef),
            ::std::mem::transmute(pvdefault),
            ::std::mem::transmute(cbdefault),
            ::std::mem::transmute(pcolumnid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetAttachDatabase2A<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const i8,
    cpgdatabasesizemax: u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetAttachDatabase2A(
                sesid: JET_SESID,
                szfilename: *const i8,
                cpgdatabasesizemax: u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetAttachDatabase2A(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(cpgdatabasesizemax),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetAttachDatabase2W<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const u16,
    cpgdatabasesizemax: u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetAttachDatabase2W(
                sesid: JET_SESID,
                szfilename: *const u16,
                cpgdatabasesizemax: u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetAttachDatabase2W(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(cpgdatabasesizemax),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetAttachDatabaseA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const i8,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetAttachDatabaseA(sesid: JET_SESID, szfilename: *const i8, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetAttachDatabaseA(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetAttachDatabaseW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const u16,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetAttachDatabaseW(sesid: JET_SESID, szfilename: *const u16, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetAttachDatabaseW(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetBackupA(
    szbackuppath: *const i8,
    grbit: u32,
    pfnstatus: ::std::option::Option<JET_PFNSTATUS>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetBackupA(
                szbackuppath: *const i8,
                grbit: u32,
                pfnstatus: ::windows::runtime::RawPtr,
            ) -> i32;
        }
        ::std::mem::transmute(JetBackupA(
            ::std::mem::transmute(szbackuppath),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(pfnstatus),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetBackupInstanceA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>>(
    instance: Param0,
    szbackuppath: *const i8,
    grbit: u32,
    pfnstatus: ::std::option::Option<JET_PFNSTATUS>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetBackupInstanceA(
                instance: JET_INSTANCE,
                szbackuppath: *const i8,
                grbit: u32,
                pfnstatus: ::windows::runtime::RawPtr,
            ) -> i32;
        }
        ::std::mem::transmute(JetBackupInstanceA(
            instance.into_param().abi(),
            ::std::mem::transmute(szbackuppath),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(pfnstatus),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetBackupInstanceW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>>(
    instance: Param0,
    szbackuppath: *const u16,
    grbit: u32,
    pfnstatus: ::std::option::Option<JET_PFNSTATUS>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetBackupInstanceW(
                instance: JET_INSTANCE,
                szbackuppath: *const u16,
                grbit: u32,
                pfnstatus: ::windows::runtime::RawPtr,
            ) -> i32;
        }
        ::std::mem::transmute(JetBackupInstanceW(
            instance.into_param().abi(),
            ::std::mem::transmute(szbackuppath),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(pfnstatus),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetBackupW(
    szbackuppath: *const u16,
    grbit: u32,
    pfnstatus: ::std::option::Option<JET_PFNSTATUS>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetBackupW(
                szbackuppath: *const u16,
                grbit: u32,
                pfnstatus: ::windows::runtime::RawPtr,
            ) -> i32;
        }
        ::std::mem::transmute(JetBackupW(
            ::std::mem::transmute(szbackuppath),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(pfnstatus),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetBeginExternalBackup(grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetBeginExternalBackup(grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetBeginExternalBackup(::std::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetBeginExternalBackupInstance<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetBeginExternalBackupInstance(instance: JET_INSTANCE, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetBeginExternalBackupInstance(
            instance.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetBeginSessionA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>>(
    instance: Param0,
    psesid: *mut JET_SESID,
    szusername: *const i8,
    szpassword: *const i8,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetBeginSessionA(
                instance: JET_INSTANCE,
                psesid: *mut JET_SESID,
                szusername: *const i8,
                szpassword: *const i8,
            ) -> i32;
        }
        ::std::mem::transmute(JetBeginSessionA(
            instance.into_param().abi(),
            ::std::mem::transmute(psesid),
            ::std::mem::transmute(szusername),
            ::std::mem::transmute(szpassword),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetBeginSessionW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>>(
    instance: Param0,
    psesid: *mut JET_SESID,
    szusername: *const u16,
    szpassword: *const u16,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetBeginSessionW(
                instance: JET_INSTANCE,
                psesid: *mut JET_SESID,
                szusername: *const u16,
                szpassword: *const u16,
            ) -> i32;
        }
        ::std::mem::transmute(JetBeginSessionW(
            instance.into_param().abi(),
            ::std::mem::transmute(psesid),
            ::std::mem::transmute(szusername),
            ::std::mem::transmute(szpassword),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetBeginTransaction<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetBeginTransaction(sesid: JET_SESID) -> i32;
        }
        ::std::mem::transmute(JetBeginTransaction(sesid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetBeginTransaction2<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetBeginTransaction2(sesid: JET_SESID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetBeginTransaction2(
            sesid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetBeginTransaction3<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    trxid: i64,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetBeginTransaction3(sesid: JET_SESID, trxid: i64, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetBeginTransaction3(
            sesid.into_param().abi(),
            ::std::mem::transmute(trxid),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCloseDatabase<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCloseDatabase(sesid: JET_SESID, dbid: u32, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetCloseDatabase(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCloseFile<'a, Param0: ::windows::runtime::IntoParam<'a, JET_HANDLE>>(
    hffile: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCloseFile(hffile: JET_HANDLE) -> i32;
        }
        ::std::mem::transmute(JetCloseFile(hffile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCloseFileInstance<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, JET_HANDLE>,
>(
    instance: Param0,
    hffile: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCloseFileInstance(instance: JET_INSTANCE, hffile: JET_HANDLE) -> i32;
        }
        ::std::mem::transmute(JetCloseFileInstance(
            instance.into_param().abi(),
            hffile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCloseTable<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCloseTable(sesid: JET_SESID, tableid: JET_TABLEID) -> i32;
        }
        ::std::mem::transmute(JetCloseTable(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCommitTransaction<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCommitTransaction(sesid: JET_SESID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetCommitTransaction(
            sesid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn JetCommitTransaction2<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    grbit: u32,
    cmsecdurablecommit: u32,
    pcommitid: *mut JET_COMMIT_ID,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCommitTransaction2(
                sesid: JET_SESID,
                grbit: u32,
                cmsecdurablecommit: u32,
                pcommitid: *mut JET_COMMIT_ID,
            ) -> i32;
        }
        ::std::mem::transmute(JetCommitTransaction2(
            sesid.into_param().abi(),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(cmsecdurablecommit),
            ::std::mem::transmute(pcommitid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCompactA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szdatabasesrc: *const i8,
    szdatabasedest: *const i8,
    pfnstatus: ::std::option::Option<JET_PFNSTATUS>,
    pconvert: *const CONVERT_A,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCompactA(
                sesid: JET_SESID,
                szdatabasesrc: *const i8,
                szdatabasedest: *const i8,
                pfnstatus: ::windows::runtime::RawPtr,
                pconvert: *const CONVERT_A,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCompactA(
            sesid.into_param().abi(),
            ::std::mem::transmute(szdatabasesrc),
            ::std::mem::transmute(szdatabasedest),
            ::std::mem::transmute(pfnstatus),
            ::std::mem::transmute(pconvert),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCompactW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szdatabasesrc: *const u16,
    szdatabasedest: *const u16,
    pfnstatus: ::std::option::Option<JET_PFNSTATUS>,
    pconvert: *const CONVERT_W,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCompactW(
                sesid: JET_SESID,
                szdatabasesrc: *const u16,
                szdatabasedest: *const u16,
                pfnstatus: ::windows::runtime::RawPtr,
                pconvert: *const CONVERT_W,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCompactW(
            sesid.into_param().abi(),
            ::std::mem::transmute(szdatabasesrc),
            ::std::mem::transmute(szdatabasedest),
            ::std::mem::transmute(pfnstatus),
            ::std::mem::transmute(pconvert),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetComputeStats<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetComputeStats(sesid: JET_SESID, tableid: JET_TABLEID) -> i32;
        }
        ::std::mem::transmute(JetComputeStats(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetConfigureProcessForCrashDump(grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetConfigureProcessForCrashDump(grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetConfigureProcessForCrashDump(::std::mem::transmute(
            grbit,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCreateDatabase2A<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const i8,
    cpgdatabasesizemax: u32,
    pdbid: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateDatabase2A(
                sesid: JET_SESID,
                szfilename: *const i8,
                cpgdatabasesizemax: u32,
                pdbid: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateDatabase2A(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(cpgdatabasesizemax),
            ::std::mem::transmute(pdbid),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCreateDatabase2W<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const u16,
    cpgdatabasesizemax: u32,
    pdbid: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateDatabase2W(
                sesid: JET_SESID,
                szfilename: *const u16,
                cpgdatabasesizemax: u32,
                pdbid: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateDatabase2W(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(cpgdatabasesizemax),
            ::std::mem::transmute(pdbid),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCreateDatabaseA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const i8,
    szconnect: *const i8,
    pdbid: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateDatabaseA(
                sesid: JET_SESID,
                szfilename: *const i8,
                szconnect: *const i8,
                pdbid: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateDatabaseA(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(szconnect),
            ::std::mem::transmute(pdbid),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCreateDatabaseW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const u16,
    szconnect: *const u16,
    pdbid: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateDatabaseW(
                sesid: JET_SESID,
                szfilename: *const u16,
                szconnect: *const u16,
                pdbid: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateDatabaseW(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(szconnect),
            ::std::mem::transmute(pdbid),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateIndex2A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pindexcreate: *const JET_INDEXCREATE_A,
    cindexcreate: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateIndex2A(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pindexcreate: *const JET_INDEXCREATE_A,
                cindexcreate: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateIndex2A(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pindexcreate),
            ::std::mem::transmute(cindexcreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateIndex2W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pindexcreate: *const JET_INDEXCREATE_W,
    cindexcreate: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateIndex2W(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pindexcreate: *const JET_INDEXCREATE_W,
                cindexcreate: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateIndex2W(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pindexcreate),
            ::std::mem::transmute(cindexcreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateIndex3A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pindexcreate: *const JET_INDEXCREATE2_A,
    cindexcreate: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateIndex3A(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pindexcreate: *const JET_INDEXCREATE2_A,
                cindexcreate: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateIndex3A(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pindexcreate),
            ::std::mem::transmute(cindexcreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateIndex3W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pindexcreate: *const JET_INDEXCREATE2_W,
    cindexcreate: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateIndex3W(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pindexcreate: *const JET_INDEXCREATE2_W,
                cindexcreate: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateIndex3W(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pindexcreate),
            ::std::mem::transmute(cindexcreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateIndex4A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pindexcreate: *const JET_INDEXCREATE3_A,
    cindexcreate: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateIndex4A(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pindexcreate: *const JET_INDEXCREATE3_A,
                cindexcreate: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateIndex4A(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pindexcreate),
            ::std::mem::transmute(cindexcreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateIndex4W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pindexcreate: *const JET_INDEXCREATE3_W,
    cindexcreate: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateIndex4W(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pindexcreate: *const JET_INDEXCREATE3_W,
                cindexcreate: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateIndex4W(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pindexcreate),
            ::std::mem::transmute(cindexcreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateIndexA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const i8,
    grbit: u32,
    szkey: Param4,
    cbkey: u32,
    ldensity: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateIndexA(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const i8,
                grbit: u32,
                szkey: super::super::Foundation::PSTR,
                cbkey: u32,
                ldensity: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateIndexA(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(grbit),
            szkey.into_param().abi(),
            ::std::mem::transmute(cbkey),
            ::std::mem::transmute(ldensity),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateIndexW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const u16,
    grbit: u32,
    szkey: Param4,
    cbkey: u32,
    ldensity: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateIndexW(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const u16,
                grbit: u32,
                szkey: super::super::Foundation::PWSTR,
                cbkey: u32,
                ldensity: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateIndexW(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(grbit),
            szkey.into_param().abi(),
            ::std::mem::transmute(cbkey),
            ::std::mem::transmute(ldensity),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCreateInstance2A(
    pinstance: *mut JET_INSTANCE,
    szinstancename: *const i8,
    szdisplayname: *const i8,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateInstance2A(
                pinstance: *mut JET_INSTANCE,
                szinstancename: *const i8,
                szdisplayname: *const i8,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateInstance2A(
            ::std::mem::transmute(pinstance),
            ::std::mem::transmute(szinstancename),
            ::std::mem::transmute(szdisplayname),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCreateInstance2W(
    pinstance: *mut JET_INSTANCE,
    szinstancename: *const u16,
    szdisplayname: *const u16,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateInstance2W(
                pinstance: *mut JET_INSTANCE,
                szinstancename: *const u16,
                szdisplayname: *const u16,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateInstance2W(
            ::std::mem::transmute(pinstance),
            ::std::mem::transmute(szinstancename),
            ::std::mem::transmute(szdisplayname),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCreateInstanceA(pinstance: *mut JET_INSTANCE, szinstancename: *const i8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateInstanceA(pinstance: *mut JET_INSTANCE, szinstancename: *const i8) -> i32;
        }
        ::std::mem::transmute(JetCreateInstanceA(
            ::std::mem::transmute(pinstance),
            ::std::mem::transmute(szinstancename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCreateInstanceW(pinstance: *mut JET_INSTANCE, szinstancename: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateInstanceW(pinstance: *mut JET_INSTANCE, szinstancename: *const u16) -> i32;
        }
        ::std::mem::transmute(JetCreateInstanceW(
            ::std::mem::transmute(pinstance),
            ::std::mem::transmute(szinstancename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCreateTableA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const i8,
    lpages: u32,
    ldensity: u32,
    ptableid: *mut JET_TABLEID,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateTableA(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const i8,
                lpages: u32,
                ldensity: u32,
                ptableid: *mut JET_TABLEID,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateTableA(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(lpages),
            ::std::mem::transmute(ldensity),
            ::std::mem::transmute(ptableid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateTableColumnIndex2A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
>(
    sesid: Param0,
    dbid: u32,
    ptablecreate: *mut JET_TABLECREATE2_A,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateTableColumnIndex2A(
                sesid: JET_SESID,
                dbid: u32,
                ptablecreate: *mut JET_TABLECREATE2_A,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateTableColumnIndex2A(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(ptablecreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateTableColumnIndex2W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
>(
    sesid: Param0,
    dbid: u32,
    ptablecreate: *mut JET_TABLECREATE2_W,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateTableColumnIndex2W(
                sesid: JET_SESID,
                dbid: u32,
                ptablecreate: *mut JET_TABLECREATE2_W,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateTableColumnIndex2W(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(ptablecreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateTableColumnIndex3A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
>(
    sesid: Param0,
    dbid: u32,
    ptablecreate: *mut JET_TABLECREATE3_A,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateTableColumnIndex3A(
                sesid: JET_SESID,
                dbid: u32,
                ptablecreate: *mut JET_TABLECREATE3_A,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateTableColumnIndex3A(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(ptablecreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateTableColumnIndex3W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
>(
    sesid: Param0,
    dbid: u32,
    ptablecreate: *mut JET_TABLECREATE3_W,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateTableColumnIndex3W(
                sesid: JET_SESID,
                dbid: u32,
                ptablecreate: *mut JET_TABLECREATE3_W,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateTableColumnIndex3W(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(ptablecreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateTableColumnIndex4A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
>(
    sesid: Param0,
    dbid: u32,
    ptablecreate: *mut JET_TABLECREATE4_A,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateTableColumnIndex4A(
                sesid: JET_SESID,
                dbid: u32,
                ptablecreate: *mut JET_TABLECREATE4_A,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateTableColumnIndex4A(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(ptablecreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateTableColumnIndex4W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
>(
    sesid: Param0,
    dbid: u32,
    ptablecreate: *mut JET_TABLECREATE4_W,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateTableColumnIndex4W(
                sesid: JET_SESID,
                dbid: u32,
                ptablecreate: *mut JET_TABLECREATE4_W,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateTableColumnIndex4W(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(ptablecreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateTableColumnIndexA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
>(
    sesid: Param0,
    dbid: u32,
    ptablecreate: *mut JET_TABLECREATE_A,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateTableColumnIndexA(
                sesid: JET_SESID,
                dbid: u32,
                ptablecreate: *mut JET_TABLECREATE_A,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateTableColumnIndexA(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(ptablecreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetCreateTableColumnIndexW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
>(
    sesid: Param0,
    dbid: u32,
    ptablecreate: *mut JET_TABLECREATE_W,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateTableColumnIndexW(
                sesid: JET_SESID,
                dbid: u32,
                ptablecreate: *mut JET_TABLECREATE_W,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateTableColumnIndexW(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(ptablecreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetCreateTableW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const u16,
    lpages: u32,
    ldensity: u32,
    ptableid: *mut JET_TABLEID,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetCreateTableW(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const u16,
                lpages: u32,
                ldensity: u32,
                ptableid: *mut JET_TABLEID,
            ) -> i32;
        }
        ::std::mem::transmute(JetCreateTableW(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(lpages),
            ::std::mem::transmute(ldensity),
            ::std::mem::transmute(ptableid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDefragment2A<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const i8,
    pcpasses: *mut u32,
    pcseconds: *mut u32,
    callback: ::std::option::Option<JET_CALLBACK>,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDefragment2A(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const i8,
                pcpasses: *mut u32,
                pcseconds: *mut u32,
                callback: ::windows::runtime::RawPtr,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetDefragment2A(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(pcpasses),
            ::std::mem::transmute(pcseconds),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDefragment2W<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const u16,
    pcpasses: *mut u32,
    pcseconds: *mut u32,
    callback: ::std::option::Option<JET_CALLBACK>,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDefragment2W(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const u16,
                pcpasses: *mut u32,
                pcseconds: *mut u32,
                callback: ::windows::runtime::RawPtr,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetDefragment2W(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(pcpasses),
            ::std::mem::transmute(pcseconds),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDefragment3A<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szdatabasename: *const i8,
    sztablename: *const i8,
    pcpasses: *mut u32,
    pcseconds: *mut u32,
    callback: ::std::option::Option<JET_CALLBACK>,
    pvcontext: *const ::std::ffi::c_void,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDefragment3A(
                sesid: JET_SESID,
                szdatabasename: *const i8,
                sztablename: *const i8,
                pcpasses: *mut u32,
                pcseconds: *mut u32,
                callback: ::windows::runtime::RawPtr,
                pvcontext: *const ::std::ffi::c_void,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetDefragment3A(
            sesid.into_param().abi(),
            ::std::mem::transmute(szdatabasename),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(pcpasses),
            ::std::mem::transmute(pcseconds),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(pvcontext),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDefragment3W<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szdatabasename: *const u16,
    sztablename: *const u16,
    pcpasses: *mut u32,
    pcseconds: *mut u32,
    callback: ::std::option::Option<JET_CALLBACK>,
    pvcontext: *const ::std::ffi::c_void,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDefragment3W(
                sesid: JET_SESID,
                szdatabasename: *const u16,
                sztablename: *const u16,
                pcpasses: *mut u32,
                pcseconds: *mut u32,
                callback: ::windows::runtime::RawPtr,
                pvcontext: *const ::std::ffi::c_void,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetDefragment3W(
            sesid.into_param().abi(),
            ::std::mem::transmute(szdatabasename),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(pcpasses),
            ::std::mem::transmute(pcseconds),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(pvcontext),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDefragmentA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const i8,
    pcpasses: *mut u32,
    pcseconds: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDefragmentA(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const i8,
                pcpasses: *mut u32,
                pcseconds: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetDefragmentA(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(pcpasses),
            ::std::mem::transmute(pcseconds),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDefragmentW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const u16,
    pcpasses: *mut u32,
    pcseconds: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDefragmentW(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const u16,
                pcpasses: *mut u32,
                pcseconds: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetDefragmentW(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(pcpasses),
            ::std::mem::transmute(pcseconds),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDelete<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDelete(sesid: JET_SESID, tableid: JET_TABLEID) -> i32;
        }
        ::std::mem::transmute(JetDelete(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDeleteColumn2A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szcolumnname: *const i8,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDeleteColumn2A(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szcolumnname: *const i8,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetDeleteColumn2A(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szcolumnname),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDeleteColumn2W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szcolumnname: *const u16,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDeleteColumn2W(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szcolumnname: *const u16,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetDeleteColumn2W(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szcolumnname),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDeleteColumnA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szcolumnname: *const i8,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDeleteColumnA(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szcolumnname: *const i8,
            ) -> i32;
        }
        ::std::mem::transmute(JetDeleteColumnA(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szcolumnname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDeleteColumnW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szcolumnname: *const u16,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDeleteColumnW(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szcolumnname: *const u16,
            ) -> i32;
        }
        ::std::mem::transmute(JetDeleteColumnW(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szcolumnname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDeleteIndexA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const i8,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDeleteIndexA(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const i8,
            ) -> i32;
        }
        ::std::mem::transmute(JetDeleteIndexA(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDeleteIndexW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const u16,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDeleteIndexW(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const u16,
            ) -> i32;
        }
        ::std::mem::transmute(JetDeleteIndexW(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDeleteTableA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const i8,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDeleteTableA(sesid: JET_SESID, dbid: u32, sztablename: *const i8) -> i32;
        }
        ::std::mem::transmute(JetDeleteTableA(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDeleteTableW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const u16,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDeleteTableW(sesid: JET_SESID, dbid: u32, sztablename: *const u16) -> i32;
        }
        ::std::mem::transmute(JetDeleteTableW(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDetachDatabase2A<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const i8,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDetachDatabase2A(sesid: JET_SESID, szfilename: *const i8, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetDetachDatabase2A(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDetachDatabase2W<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const u16,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDetachDatabase2W(sesid: JET_SESID, szfilename: *const u16, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetDetachDatabase2W(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDetachDatabaseA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const i8,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDetachDatabaseA(sesid: JET_SESID, szfilename: *const i8) -> i32;
        }
        ::std::mem::transmute(JetDetachDatabaseA(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDetachDatabaseW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const u16,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDetachDatabaseW(sesid: JET_SESID, szfilename: *const u16) -> i32;
        }
        ::std::mem::transmute(JetDetachDatabaseW(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDupCursor<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    ptableid: *mut JET_TABLEID,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDupCursor(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                ptableid: *mut JET_TABLEID,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetDupCursor(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(ptableid),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetDupSession<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    psesid: *mut JET_SESID,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetDupSession(sesid: JET_SESID, psesid: *mut JET_SESID) -> i32;
        }
        ::std::mem::transmute(JetDupSession(
            sesid.into_param().abi(),
            ::std::mem::transmute(psesid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetEnableMultiInstanceA(
    psetsysparam: *const JET_SETSYSPARAM_A,
    csetsysparam: u32,
    pcsetsucceed: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetEnableMultiInstanceA(
                psetsysparam: *const JET_SETSYSPARAM_A,
                csetsysparam: u32,
                pcsetsucceed: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetEnableMultiInstanceA(
            ::std::mem::transmute(psetsysparam),
            ::std::mem::transmute(csetsysparam),
            ::std::mem::transmute(pcsetsucceed),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetEnableMultiInstanceW(
    psetsysparam: *const JET_SETSYSPARAM_W,
    csetsysparam: u32,
    pcsetsucceed: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetEnableMultiInstanceW(
                psetsysparam: *const JET_SETSYSPARAM_W,
                csetsysparam: u32,
                pcsetsucceed: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetEnableMultiInstanceW(
            ::std::mem::transmute(psetsysparam),
            ::std::mem::transmute(csetsysparam),
            ::std::mem::transmute(pcsetsucceed),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetEndExternalBackup() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetEndExternalBackup() -> i32;
        }
        ::std::mem::transmute(JetEndExternalBackup())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetEndExternalBackupInstance<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetEndExternalBackupInstance(instance: JET_INSTANCE) -> i32;
        }
        ::std::mem::transmute(JetEndExternalBackupInstance(instance.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetEndExternalBackupInstance2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetEndExternalBackupInstance2(instance: JET_INSTANCE, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetEndExternalBackupInstance2(
            instance.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetEndSession<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetEndSession(sesid: JET_SESID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetEndSession(
            sesid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetEnumerateColumns<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    cenumcolumnid: u32,
    rgenumcolumnid: *const JET_ENUMCOLUMNID,
    pcenumcolumn: *mut u32,
    prgenumcolumn: *mut *mut JET_ENUMCOLUMN,
    pfnrealloc: ::std::option::Option<JET_PFNREALLOC>,
    pvrealloccontext: *const ::std::ffi::c_void,
    cbdatamost: u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetEnumerateColumns(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                cenumcolumnid: u32,
                rgenumcolumnid: *const JET_ENUMCOLUMNID,
                pcenumcolumn: *mut u32,
                prgenumcolumn: *mut *mut JET_ENUMCOLUMN,
                pfnrealloc: ::windows::runtime::RawPtr,
                pvrealloccontext: *const ::std::ffi::c_void,
                cbdatamost: u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetEnumerateColumns(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(cenumcolumnid),
            ::std::mem::transmute(rgenumcolumnid),
            ::std::mem::transmute(pcenumcolumn),
            ::std::mem::transmute(prgenumcolumn),
            ::std::mem::transmute(pfnrealloc),
            ::std::mem::transmute(pvrealloccontext),
            ::std::mem::transmute(cbdatamost),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetEscrowUpdate<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    columnid: u32,
    pv: *const ::std::ffi::c_void,
    cbmax: u32,
    pvold: *mut ::std::ffi::c_void,
    cboldmax: u32,
    pcboldactual: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetEscrowUpdate(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                columnid: u32,
                pv: *const ::std::ffi::c_void,
                cbmax: u32,
                pvold: *mut ::std::ffi::c_void,
                cboldmax: u32,
                pcboldactual: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetEscrowUpdate(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(columnid),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pvold),
            ::std::mem::transmute(cboldmax),
            ::std::mem::transmute(pcboldactual),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn JetExternalRestore2A(
    szcheckpointfilepath: *const i8,
    szlogpath: *const i8,
    rgrstmap: *const JET_RSTMAP_A,
    crstfilemap: i32,
    szbackuplogpath: *const i8,
    ploginfo: *mut JET_LOGINFO_A,
    sztargetinstancename: *const i8,
    sztargetinstancelogpath: *const i8,
    sztargetinstancecheckpointpath: *const i8,
    pfn: ::std::option::Option<JET_PFNSTATUS>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetExternalRestore2A(
                szcheckpointfilepath: *const i8,
                szlogpath: *const i8,
                rgrstmap: *const JET_RSTMAP_A,
                crstfilemap: i32,
                szbackuplogpath: *const i8,
                ploginfo: *mut JET_LOGINFO_A,
                sztargetinstancename: *const i8,
                sztargetinstancelogpath: *const i8,
                sztargetinstancecheckpointpath: *const i8,
                pfn: ::windows::runtime::RawPtr,
            ) -> i32;
        }
        ::std::mem::transmute(JetExternalRestore2A(
            ::std::mem::transmute(szcheckpointfilepath),
            ::std::mem::transmute(szlogpath),
            ::std::mem::transmute(rgrstmap),
            ::std::mem::transmute(crstfilemap),
            ::std::mem::transmute(szbackuplogpath),
            ::std::mem::transmute(ploginfo),
            ::std::mem::transmute(sztargetinstancename),
            ::std::mem::transmute(sztargetinstancelogpath),
            ::std::mem::transmute(sztargetinstancecheckpointpath),
            ::std::mem::transmute(pfn),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetExternalRestore2W(
    szcheckpointfilepath: *const u16,
    szlogpath: *const u16,
    rgrstmap: *const JET_RSTMAP_W,
    crstfilemap: i32,
    szbackuplogpath: *const u16,
    ploginfo: *mut JET_LOGINFO_W,
    sztargetinstancename: *const u16,
    sztargetinstancelogpath: *const u16,
    sztargetinstancecheckpointpath: *const u16,
    pfn: ::std::option::Option<JET_PFNSTATUS>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetExternalRestore2W(
                szcheckpointfilepath: *const u16,
                szlogpath: *const u16,
                rgrstmap: *const JET_RSTMAP_W,
                crstfilemap: i32,
                szbackuplogpath: *const u16,
                ploginfo: *mut JET_LOGINFO_W,
                sztargetinstancename: *const u16,
                sztargetinstancelogpath: *const u16,
                sztargetinstancecheckpointpath: *const u16,
                pfn: ::windows::runtime::RawPtr,
            ) -> i32;
        }
        ::std::mem::transmute(JetExternalRestore2W(
            ::std::mem::transmute(szcheckpointfilepath),
            ::std::mem::transmute(szlogpath),
            ::std::mem::transmute(rgrstmap),
            ::std::mem::transmute(crstfilemap),
            ::std::mem::transmute(szbackuplogpath),
            ::std::mem::transmute(ploginfo),
            ::std::mem::transmute(sztargetinstancename),
            ::std::mem::transmute(sztargetinstancelogpath),
            ::std::mem::transmute(sztargetinstancecheckpointpath),
            ::std::mem::transmute(pfn),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetExternalRestoreA(
    szcheckpointfilepath: *const i8,
    szlogpath: *const i8,
    rgrstmap: *const JET_RSTMAP_A,
    crstfilemap: i32,
    szbackuplogpath: *const i8,
    genlow: i32,
    genhigh: i32,
    pfn: ::std::option::Option<JET_PFNSTATUS>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetExternalRestoreA(
                szcheckpointfilepath: *const i8,
                szlogpath: *const i8,
                rgrstmap: *const JET_RSTMAP_A,
                crstfilemap: i32,
                szbackuplogpath: *const i8,
                genlow: i32,
                genhigh: i32,
                pfn: ::windows::runtime::RawPtr,
            ) -> i32;
        }
        ::std::mem::transmute(JetExternalRestoreA(
            ::std::mem::transmute(szcheckpointfilepath),
            ::std::mem::transmute(szlogpath),
            ::std::mem::transmute(rgrstmap),
            ::std::mem::transmute(crstfilemap),
            ::std::mem::transmute(szbackuplogpath),
            ::std::mem::transmute(genlow),
            ::std::mem::transmute(genhigh),
            ::std::mem::transmute(pfn),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetExternalRestoreW(
    szcheckpointfilepath: *const u16,
    szlogpath: *const u16,
    rgrstmap: *const JET_RSTMAP_W,
    crstfilemap: i32,
    szbackuplogpath: *const u16,
    genlow: i32,
    genhigh: i32,
    pfn: ::std::option::Option<JET_PFNSTATUS>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetExternalRestoreW(
                szcheckpointfilepath: *const u16,
                szlogpath: *const u16,
                rgrstmap: *const JET_RSTMAP_W,
                crstfilemap: i32,
                szbackuplogpath: *const u16,
                genlow: i32,
                genhigh: i32,
                pfn: ::windows::runtime::RawPtr,
            ) -> i32;
        }
        ::std::mem::transmute(JetExternalRestoreW(
            ::std::mem::transmute(szcheckpointfilepath),
            ::std::mem::transmute(szlogpath),
            ::std::mem::transmute(rgrstmap),
            ::std::mem::transmute(crstfilemap),
            ::std::mem::transmute(szbackuplogpath),
            ::std::mem::transmute(genlow),
            ::std::mem::transmute(genhigh),
            ::std::mem::transmute(pfn),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetFreeBuffer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pbbuf: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetFreeBuffer(pbbuf: super::super::Foundation::PSTR) -> i32;
        }
        ::std::mem::transmute(JetFreeBuffer(pbbuf.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetAttachInfoA(szzdatabases: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetAttachInfoA(szzdatabases: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::std::mem::transmute(JetGetAttachInfoA(
            ::std::mem::transmute(szzdatabases),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetAttachInfoInstanceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
    szzdatabases: *mut i8,
    cbmax: u32,
    pcbactual: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetAttachInfoInstanceA(
                instance: JET_INSTANCE,
                szzdatabases: *mut i8,
                cbmax: u32,
                pcbactual: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetAttachInfoInstanceA(
            instance.into_param().abi(),
            ::std::mem::transmute(szzdatabases),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetAttachInfoInstanceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
    szzdatabases: *mut u16,
    cbmax: u32,
    pcbactual: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetAttachInfoInstanceW(
                instance: JET_INSTANCE,
                szzdatabases: *mut u16,
                cbmax: u32,
                pcbactual: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetAttachInfoInstanceW(
            instance.into_param().abi(),
            ::std::mem::transmute(szzdatabases),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetAttachInfoW(wszzdatabases: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetAttachInfoW(wszzdatabases: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::std::mem::transmute(JetGetAttachInfoW(
            ::std::mem::transmute(wszzdatabases),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetBookmark<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pvbookmark: *mut ::std::ffi::c_void,
    cbmax: u32,
    pcbactual: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetBookmark(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pvbookmark: *mut ::std::ffi::c_void,
                cbmax: u32,
                pcbactual: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetBookmark(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pvbookmark),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetColumnInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const i8,
    pcolumnnameorid: *const i8,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetColumnInfoA(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const i8,
                pcolumnnameorid: *const i8,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetColumnInfoA(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(pcolumnnameorid),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetColumnInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const u16,
    pwcolumnnameorid: *const u16,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetColumnInfoW(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const u16,
                pwcolumnnameorid: *const u16,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetColumnInfoW(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(pwcolumnnameorid),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetCurrentIndexA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *mut i8,
    cbindexname: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetCurrentIndexA(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *mut i8,
                cbindexname: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetCurrentIndexA(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(cbindexname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetCurrentIndexW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *mut u16,
    cbindexname: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetCurrentIndexW(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *mut u16,
                cbindexname: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetCurrentIndexW(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(cbindexname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetCursorInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetCursorInfo(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetCursorInfo(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetDatabaseFileInfoA(
    szdatabasename: *const i8,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetDatabaseFileInfoA(
                szdatabasename: *const i8,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetDatabaseFileInfoA(
            ::std::mem::transmute(szdatabasename),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetDatabaseFileInfoW(
    szdatabasename: *const u16,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetDatabaseFileInfoW(
                szdatabasename: *const u16,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetDatabaseFileInfoW(
            ::std::mem::transmute(szdatabasename),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetDatabaseInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetDatabaseInfoA(
                sesid: JET_SESID,
                dbid: u32,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetDatabaseInfoA(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetDatabaseInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetDatabaseInfoW(
                sesid: JET_SESID,
                dbid: u32,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetDatabaseInfoW(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetErrorInfoW(
    pvcontext: *const ::std::ffi::c_void,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetErrorInfoW(
                pvcontext: *const ::std::ffi::c_void,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetErrorInfoW(
            ::std::mem::transmute(pvcontext),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetIndexInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const i8,
    szindexname: *const i8,
    pvresult: *mut ::std::ffi::c_void,
    cbresult: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetIndexInfoA(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const i8,
                szindexname: *const i8,
                pvresult: *mut ::std::ffi::c_void,
                cbresult: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetIndexInfoA(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbresult),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetIndexInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const u16,
    szindexname: *const u16,
    pvresult: *mut ::std::ffi::c_void,
    cbresult: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetIndexInfoW(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const u16,
                szindexname: *const u16,
                pvresult: *mut ::std::ffi::c_void,
                cbresult: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetIndexInfoW(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbresult),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetGetInstanceInfoA(
    pcinstanceinfo: *mut u32,
    painstanceinfo: *mut *mut JET_INSTANCE_INFO_A,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetInstanceInfoA(
                pcinstanceinfo: *mut u32,
                painstanceinfo: *mut *mut JET_INSTANCE_INFO_A,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetInstanceInfoA(
            ::std::mem::transmute(pcinstanceinfo),
            ::std::mem::transmute(painstanceinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetGetInstanceInfoW(
    pcinstanceinfo: *mut u32,
    painstanceinfo: *mut *mut JET_INSTANCE_INFO_W,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetInstanceInfoW(
                pcinstanceinfo: *mut u32,
                painstanceinfo: *mut *mut JET_INSTANCE_INFO_W,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetInstanceInfoW(
            ::std::mem::transmute(pcinstanceinfo),
            ::std::mem::transmute(painstanceinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetInstanceMiscInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetInstanceMiscInfo(
                instance: JET_INSTANCE,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetInstanceMiscInfo(
            instance.into_param().abi(),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetLS<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pls: *mut JET_LS,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetLS(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pls: *mut JET_LS,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetLS(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pls),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetLock<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetLock(sesid: JET_SESID, tableid: JET_TABLEID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetGetLock(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetLogInfoA(szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetLogInfoA(szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::std::mem::transmute(JetGetLogInfoA(
            ::std::mem::transmute(szzlogs),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn JetGetLogInfoInstance2A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
    szzlogs: *mut i8,
    cbmax: u32,
    pcbactual: *mut u32,
    ploginfo: *mut JET_LOGINFO_A,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetLogInfoInstance2A(
                instance: JET_INSTANCE,
                szzlogs: *mut i8,
                cbmax: u32,
                pcbactual: *mut u32,
                ploginfo: *mut JET_LOGINFO_A,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetLogInfoInstance2A(
            instance.into_param().abi(),
            ::std::mem::transmute(szzlogs),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
            ::std::mem::transmute(ploginfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetLogInfoInstance2W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
    wszzlogs: *mut u16,
    cbmax: u32,
    pcbactual: *mut u32,
    ploginfo: *mut JET_LOGINFO_W,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetLogInfoInstance2W(
                instance: JET_INSTANCE,
                wszzlogs: *mut u16,
                cbmax: u32,
                pcbactual: *mut u32,
                ploginfo: *mut JET_LOGINFO_W,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetLogInfoInstance2W(
            instance.into_param().abi(),
            ::std::mem::transmute(wszzlogs),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
            ::std::mem::transmute(ploginfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetLogInfoInstanceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
    szzlogs: *mut i8,
    cbmax: u32,
    pcbactual: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetLogInfoInstanceA(
                instance: JET_INSTANCE,
                szzlogs: *mut i8,
                cbmax: u32,
                pcbactual: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetLogInfoInstanceA(
            instance.into_param().abi(),
            ::std::mem::transmute(szzlogs),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetLogInfoInstanceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
    wszzlogs: *mut u16,
    cbmax: u32,
    pcbactual: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetLogInfoInstanceW(
                instance: JET_INSTANCE,
                wszzlogs: *mut u16,
                cbmax: u32,
                pcbactual: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetLogInfoInstanceW(
            instance.into_param().abi(),
            ::std::mem::transmute(wszzlogs),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetLogInfoW(szzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetLogInfoW(szzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::std::mem::transmute(JetGetLogInfoW(
            ::std::mem::transmute(szzlogs),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetObjectInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    objtyp: u32,
    szcontainername: *const i8,
    szobjectname: *const i8,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetObjectInfoA(
                sesid: JET_SESID,
                dbid: u32,
                objtyp: u32,
                szcontainername: *const i8,
                szobjectname: *const i8,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetObjectInfoA(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(objtyp),
            ::std::mem::transmute(szcontainername),
            ::std::mem::transmute(szobjectname),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetObjectInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    objtyp: u32,
    szcontainername: *const u16,
    szobjectname: *const u16,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetObjectInfoW(
                sesid: JET_SESID,
                dbid: u32,
                objtyp: u32,
                szcontainername: *const u16,
                szobjectname: *const u16,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetObjectInfoW(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(objtyp),
            ::std::mem::transmute(szcontainername),
            ::std::mem::transmute(szobjectname),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetRecordPosition<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    precpos: *mut JET_RECPOS,
    cbrecpos: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetRecordPosition(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                precpos: *mut JET_RECPOS,
                cbrecpos: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetRecordPosition(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(precpos),
            ::std::mem::transmute(cbrecpos),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetRecordSize<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    precsize: *mut JET_RECSIZE,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetRecordSize(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                precsize: *mut JET_RECSIZE,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetRecordSize(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(precsize),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetRecordSize2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    precsize: *mut JET_RECSIZE2,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetRecordSize2(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                precsize: *mut JET_RECSIZE2,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetRecordSize2(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(precsize),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetSecondaryIndexBookmark<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pvsecondarykey: *mut ::std::ffi::c_void,
    cbsecondarykeymax: u32,
    pcbsecondarykeyactual: *mut u32,
    pvprimarybookmark: *mut ::std::ffi::c_void,
    cbprimarybookmarkmax: u32,
    pcbprimarybookmarkactual: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetSecondaryIndexBookmark(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pvsecondarykey: *mut ::std::ffi::c_void,
                cbsecondarykeymax: u32,
                pcbsecondarykeyactual: *mut u32,
                pvprimarybookmark: *mut ::std::ffi::c_void,
                cbprimarybookmarkmax: u32,
                pcbprimarybookmarkactual: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetSecondaryIndexBookmark(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pvsecondarykey),
            ::std::mem::transmute(cbsecondarykeymax),
            ::std::mem::transmute(pcbsecondarykeyactual),
            ::std::mem::transmute(pvprimarybookmark),
            ::std::mem::transmute(cbprimarybookmarkmax),
            ::std::mem::transmute(pcbprimarybookmarkactual),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetSessionParameter<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    sesparamid: u32,
    pvparam: *mut ::std::ffi::c_void,
    cbparammax: u32,
    pcbparamactual: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetSessionParameter(
                sesid: JET_SESID,
                sesparamid: u32,
                pvparam: *mut ::std::ffi::c_void,
                cbparammax: u32,
                pcbparamactual: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetSessionParameter(
            sesid.into_param().abi(),
            ::std::mem::transmute(sesparamid),
            ::std::mem::transmute(pvparam),
            ::std::mem::transmute(cbparammax),
            ::std::mem::transmute(pcbparamactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetSystemParameterA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, JET_SESID>,
>(
    instance: Param0,
    sesid: Param1,
    paramid: u32,
    plparam: *mut JET_API_PTR,
    szparam: *mut i8,
    cbmax: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetSystemParameterA(
                instance: JET_INSTANCE,
                sesid: JET_SESID,
                paramid: u32,
                plparam: *mut JET_API_PTR,
                szparam: *mut i8,
                cbmax: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetSystemParameterA(
            instance.into_param().abi(),
            sesid.into_param().abi(),
            ::std::mem::transmute(paramid),
            ::std::mem::transmute(plparam),
            ::std::mem::transmute(szparam),
            ::std::mem::transmute(cbmax),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetSystemParameterW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, JET_SESID>,
>(
    instance: Param0,
    sesid: Param1,
    paramid: u32,
    plparam: *mut JET_API_PTR,
    szparam: *mut u16,
    cbmax: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetSystemParameterW(
                instance: JET_INSTANCE,
                sesid: JET_SESID,
                paramid: u32,
                plparam: *mut JET_API_PTR,
                szparam: *mut u16,
                cbmax: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetSystemParameterW(
            instance.into_param().abi(),
            sesid.into_param().abi(),
            ::std::mem::transmute(paramid),
            ::std::mem::transmute(plparam),
            ::std::mem::transmute(szparam),
            ::std::mem::transmute(cbmax),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetTableColumnInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szcolumnname: *const i8,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetTableColumnInfoA(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szcolumnname: *const i8,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetTableColumnInfoA(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szcolumnname),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetTableColumnInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szcolumnname: *const u16,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetTableColumnInfoW(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szcolumnname: *const u16,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetTableColumnInfoW(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szcolumnname),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetTableIndexInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const i8,
    pvresult: *mut ::std::ffi::c_void,
    cbresult: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetTableIndexInfoA(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const i8,
                pvresult: *mut ::std::ffi::c_void,
                cbresult: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetTableIndexInfoA(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbresult),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetTableIndexInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const u16,
    pvresult: *mut ::std::ffi::c_void,
    cbresult: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetTableIndexInfoW(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const u16,
                pvresult: *mut ::std::ffi::c_void,
                cbresult: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetTableIndexInfoW(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbresult),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetTableInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetTableInfoA(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetTableInfoA(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetTableInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pvresult: *mut ::std::ffi::c_void,
    cbmax: u32,
    infolevel: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetTableInfoW(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pvresult: *mut ::std::ffi::c_void,
                cbmax: u32,
                infolevel: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetTableInfoW(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(infolevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetThreadStats(pvresult: *mut ::std::ffi::c_void, cbmax: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetThreadStats(pvresult: *mut ::std::ffi::c_void, cbmax: u32) -> i32;
        }
        ::std::mem::transmute(JetGetThreadStats(
            ::std::mem::transmute(pvresult),
            ::std::mem::transmute(cbmax),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetTruncateLogInfoInstanceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
    szzlogs: *mut i8,
    cbmax: u32,
    pcbactual: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetTruncateLogInfoInstanceA(
                instance: JET_INSTANCE,
                szzlogs: *mut i8,
                cbmax: u32,
                pcbactual: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetTruncateLogInfoInstanceA(
            instance.into_param().abi(),
            ::std::mem::transmute(szzlogs),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetTruncateLogInfoInstanceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
    wszzlogs: *mut u16,
    cbmax: u32,
    pcbactual: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetTruncateLogInfoInstanceW(
                instance: JET_INSTANCE,
                wszzlogs: *mut u16,
                cbmax: u32,
                pcbactual: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGetTruncateLogInfoInstanceW(
            instance.into_param().abi(),
            ::std::mem::transmute(wszzlogs),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGetVersion<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    pwversion: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGetVersion(sesid: JET_SESID, pwversion: *mut u32) -> i32;
        }
        ::std::mem::transmute(JetGetVersion(
            sesid.into_param().abi(),
            ::std::mem::transmute(pwversion),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGotoBookmark<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pvbookmark: *const ::std::ffi::c_void,
    cbbookmark: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGotoBookmark(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pvbookmark: *const ::std::ffi::c_void,
                cbbookmark: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGotoBookmark(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pvbookmark),
            ::std::mem::transmute(cbbookmark),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGotoPosition<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    precpos: *const JET_RECPOS,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGotoPosition(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                precpos: *const JET_RECPOS,
            ) -> i32;
        }
        ::std::mem::transmute(JetGotoPosition(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(precpos),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGotoSecondaryIndexBookmark<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pvsecondarykey: *const ::std::ffi::c_void,
    cbsecondarykey: u32,
    pvprimarybookmark: *const ::std::ffi::c_void,
    cbprimarybookmark: u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGotoSecondaryIndexBookmark(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pvsecondarykey: *const ::std::ffi::c_void,
                cbsecondarykey: u32,
                pvprimarybookmark: *const ::std::ffi::c_void,
                cbprimarybookmark: u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetGotoSecondaryIndexBookmark(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pvsecondarykey),
            ::std::mem::transmute(cbsecondarykey),
            ::std::mem::transmute(pvprimarybookmark),
            ::std::mem::transmute(cbprimarybookmark),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetGrowDatabase<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    cpg: u32,
    pcpgreal: *const u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetGrowDatabase(sesid: JET_SESID, dbid: u32, cpg: u32, pcpgreal: *const u32) -> i32;
        }
        ::std::mem::transmute(JetGrowDatabase(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(cpg),
            ::std::mem::transmute(pcpgreal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetIdle<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetIdle(sesid: JET_SESID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetIdle(
            sesid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetIndexRecordCount<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pcrec: *mut u32,
    crecmax: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetIndexRecordCount(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pcrec: *mut u32,
                crecmax: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetIndexRecordCount(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pcrec),
            ::std::mem::transmute(crecmax),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetInit(pinstance: *mut JET_INSTANCE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetInit(pinstance: *mut JET_INSTANCE) -> i32;
        }
        ::std::mem::transmute(JetInit(::std::mem::transmute(pinstance)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetInit2(pinstance: *mut JET_INSTANCE, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetInit2(pinstance: *mut JET_INSTANCE, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetInit2(
            ::std::mem::transmute(pinstance),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn JetInit3A(
    pinstance: *mut JET_INSTANCE,
    prstinfo: *const JET_RSTINFO_A,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetInit3A(
                pinstance: *mut JET_INSTANCE,
                prstinfo: *const ::std::mem::ManuallyDrop<JET_RSTINFO_A>,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetInit3A(
            ::std::mem::transmute(pinstance),
            ::std::mem::transmute(prstinfo),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn JetInit3W(
    pinstance: *mut JET_INSTANCE,
    prstinfo: *const JET_RSTINFO_W,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetInit3W(
                pinstance: *mut JET_INSTANCE,
                prstinfo: *const ::std::mem::ManuallyDrop<JET_RSTINFO_W>,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetInit3W(
            ::std::mem::transmute(pinstance),
            ::std::mem::transmute(prstinfo),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetIntersectIndexes<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    rgindexrange: *const JET_INDEXRANGE,
    cindexrange: u32,
    precordlist: *mut JET_RECORDLIST,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetIntersectIndexes(
                sesid: JET_SESID,
                rgindexrange: *const JET_INDEXRANGE,
                cindexrange: u32,
                precordlist: *mut JET_RECORDLIST,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetIntersectIndexes(
            sesid.into_param().abi(),
            ::std::mem::transmute(rgindexrange),
            ::std::mem::transmute(cindexrange),
            ::std::mem::transmute(precordlist),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetMakeKey<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pvdata: *const ::std::ffi::c_void,
    cbdata: u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetMakeKey(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pvdata: *const ::std::ffi::c_void,
                cbdata: u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetMakeKey(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pvdata),
            ::std::mem::transmute(cbdata),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetMove<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    crow: i32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetMove(sesid: JET_SESID, tableid: JET_TABLEID, crow: i32, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetMove(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(crow),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOSSnapshotAbort<'a, Param0: ::windows::runtime::IntoParam<'a, JET_OSSNAPID>>(
    snapid: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOSSnapshotAbort(snapid: JET_OSSNAPID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetOSSnapshotAbort(
            snapid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOSSnapshotEnd<'a, Param0: ::windows::runtime::IntoParam<'a, JET_OSSNAPID>>(
    snapid: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOSSnapshotEnd(snapid: JET_OSSNAPID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetOSSnapshotEnd(
            snapid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetOSSnapshotFreezeA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_OSSNAPID>>(
    snapid: Param0,
    pcinstanceinfo: *mut u32,
    painstanceinfo: *mut *mut JET_INSTANCE_INFO_A,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOSSnapshotFreezeA(
                snapid: JET_OSSNAPID,
                pcinstanceinfo: *mut u32,
                painstanceinfo: *mut *mut JET_INSTANCE_INFO_A,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOSSnapshotFreezeA(
            snapid.into_param().abi(),
            ::std::mem::transmute(pcinstanceinfo),
            ::std::mem::transmute(painstanceinfo),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetOSSnapshotFreezeW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_OSSNAPID>>(
    snapid: Param0,
    pcinstanceinfo: *mut u32,
    painstanceinfo: *mut *mut JET_INSTANCE_INFO_W,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOSSnapshotFreezeW(
                snapid: JET_OSSNAPID,
                pcinstanceinfo: *mut u32,
                painstanceinfo: *mut *mut JET_INSTANCE_INFO_W,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOSSnapshotFreezeW(
            snapid.into_param().abi(),
            ::std::mem::transmute(pcinstanceinfo),
            ::std::mem::transmute(painstanceinfo),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetOSSnapshotGetFreezeInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_OSSNAPID>,
>(
    snapid: Param0,
    pcinstanceinfo: *mut u32,
    painstanceinfo: *mut *mut JET_INSTANCE_INFO_A,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOSSnapshotGetFreezeInfoA(
                snapid: JET_OSSNAPID,
                pcinstanceinfo: *mut u32,
                painstanceinfo: *mut *mut JET_INSTANCE_INFO_A,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOSSnapshotGetFreezeInfoA(
            snapid.into_param().abi(),
            ::std::mem::transmute(pcinstanceinfo),
            ::std::mem::transmute(painstanceinfo),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetOSSnapshotGetFreezeInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_OSSNAPID>,
>(
    snapid: Param0,
    pcinstanceinfo: *mut u32,
    painstanceinfo: *mut *mut JET_INSTANCE_INFO_W,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOSSnapshotGetFreezeInfoW(
                snapid: JET_OSSNAPID,
                pcinstanceinfo: *mut u32,
                painstanceinfo: *mut *mut JET_INSTANCE_INFO_W,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOSSnapshotGetFreezeInfoW(
            snapid.into_param().abi(),
            ::std::mem::transmute(pcinstanceinfo),
            ::std::mem::transmute(painstanceinfo),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOSSnapshotPrepare(psnapid: *mut JET_OSSNAPID, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOSSnapshotPrepare(psnapid: *mut JET_OSSNAPID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetOSSnapshotPrepare(
            ::std::mem::transmute(psnapid),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOSSnapshotPrepareInstance<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_OSSNAPID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    snapid: Param0,
    instance: Param1,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOSSnapshotPrepareInstance(
                snapid: JET_OSSNAPID,
                instance: JET_INSTANCE,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOSSnapshotPrepareInstance(
            snapid.into_param().abi(),
            instance.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOSSnapshotThaw<'a, Param0: ::windows::runtime::IntoParam<'a, JET_OSSNAPID>>(
    snapid: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOSSnapshotThaw(snapid: JET_OSSNAPID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetOSSnapshotThaw(
            snapid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOSSnapshotTruncateLog<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_OSSNAPID>,
>(
    snapid: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOSSnapshotTruncateLog(snapid: JET_OSSNAPID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetOSSnapshotTruncateLog(
            snapid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOSSnapshotTruncateLogInstance<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_OSSNAPID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    snapid: Param0,
    instance: Param1,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOSSnapshotTruncateLogInstance(
                snapid: JET_OSSNAPID,
                instance: JET_INSTANCE,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOSSnapshotTruncateLogInstance(
            snapid.into_param().abi(),
            instance.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOpenDatabaseA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const i8,
    szconnect: *const i8,
    pdbid: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenDatabaseA(
                sesid: JET_SESID,
                szfilename: *const i8,
                szconnect: *const i8,
                pdbid: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenDatabaseA(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(szconnect),
            ::std::mem::transmute(pdbid),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOpenDatabaseW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szfilename: *const u16,
    szconnect: *const u16,
    pdbid: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenDatabaseW(
                sesid: JET_SESID,
                szfilename: *const u16,
                szconnect: *const u16,
                pdbid: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenDatabaseW(
            sesid.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(szconnect),
            ::std::mem::transmute(pdbid),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOpenFileA(
    szfilename: *const i8,
    phffile: *mut JET_HANDLE,
    pulfilesizelow: *mut u32,
    pulfilesizehigh: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenFileA(
                szfilename: *const i8,
                phffile: *mut JET_HANDLE,
                pulfilesizelow: *mut u32,
                pulfilesizehigh: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenFileA(
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(phffile),
            ::std::mem::transmute(pulfilesizelow),
            ::std::mem::transmute(pulfilesizehigh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOpenFileInstanceA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>>(
    instance: Param0,
    szfilename: *const i8,
    phffile: *mut JET_HANDLE,
    pulfilesizelow: *mut u32,
    pulfilesizehigh: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenFileInstanceA(
                instance: JET_INSTANCE,
                szfilename: *const i8,
                phffile: *mut JET_HANDLE,
                pulfilesizelow: *mut u32,
                pulfilesizehigh: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenFileInstanceA(
            instance.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(phffile),
            ::std::mem::transmute(pulfilesizelow),
            ::std::mem::transmute(pulfilesizehigh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOpenFileInstanceW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>>(
    instance: Param0,
    szfilename: *const u16,
    phffile: *mut JET_HANDLE,
    pulfilesizelow: *mut u32,
    pulfilesizehigh: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenFileInstanceW(
                instance: JET_INSTANCE,
                szfilename: *const u16,
                phffile: *mut JET_HANDLE,
                pulfilesizelow: *mut u32,
                pulfilesizehigh: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenFileInstanceW(
            instance.into_param().abi(),
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(phffile),
            ::std::mem::transmute(pulfilesizelow),
            ::std::mem::transmute(pulfilesizehigh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOpenFileW(
    szfilename: *const u16,
    phffile: *mut JET_HANDLE,
    pulfilesizelow: *mut u32,
    pulfilesizehigh: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenFileW(
                szfilename: *const u16,
                phffile: *mut JET_HANDLE,
                pulfilesizelow: *mut u32,
                pulfilesizehigh: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenFileW(
            ::std::mem::transmute(szfilename),
            ::std::mem::transmute(phffile),
            ::std::mem::transmute(pulfilesizelow),
            ::std::mem::transmute(pulfilesizehigh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOpenTableA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const i8,
    pvparameters: *const ::std::ffi::c_void,
    cbparameters: u32,
    grbit: u32,
    ptableid: *mut JET_TABLEID,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenTableA(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const i8,
                pvparameters: *const ::std::ffi::c_void,
                cbparameters: u32,
                grbit: u32,
                ptableid: *mut JET_TABLEID,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenTableA(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(pvparameters),
            ::std::mem::transmute(cbparameters),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(ptableid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOpenTableW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const u16,
    pvparameters: *const ::std::ffi::c_void,
    cbparameters: u32,
    grbit: u32,
    ptableid: *mut JET_TABLEID,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenTableW(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const u16,
                pvparameters: *const ::std::ffi::c_void,
                cbparameters: u32,
                grbit: u32,
                ptableid: *mut JET_TABLEID,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenTableW(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(pvparameters),
            ::std::mem::transmute(cbparameters),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(ptableid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOpenTempTable<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    prgcolumndef: *const JET_COLUMNDEF,
    ccolumn: u32,
    grbit: u32,
    ptableid: *mut JET_TABLEID,
    prgcolumnid: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenTempTable(
                sesid: JET_SESID,
                prgcolumndef: *const JET_COLUMNDEF,
                ccolumn: u32,
                grbit: u32,
                ptableid: *mut JET_TABLEID,
                prgcolumnid: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenTempTable(
            sesid.into_param().abi(),
            ::std::mem::transmute(prgcolumndef),
            ::std::mem::transmute(ccolumn),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(ptableid),
            ::std::mem::transmute(prgcolumnid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOpenTempTable2<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    prgcolumndef: *const JET_COLUMNDEF,
    ccolumn: u32,
    lcid: u32,
    grbit: u32,
    ptableid: *mut JET_TABLEID,
    prgcolumnid: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenTempTable2(
                sesid: JET_SESID,
                prgcolumndef: *const JET_COLUMNDEF,
                ccolumn: u32,
                lcid: u32,
                grbit: u32,
                ptableid: *mut JET_TABLEID,
                prgcolumnid: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenTempTable2(
            sesid.into_param().abi(),
            ::std::mem::transmute(prgcolumndef),
            ::std::mem::transmute(ccolumn),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(ptableid),
            ::std::mem::transmute(prgcolumnid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOpenTempTable3<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    prgcolumndef: *const JET_COLUMNDEF,
    ccolumn: u32,
    pidxunicode: *const JET_UNICODEINDEX,
    grbit: u32,
    ptableid: *mut JET_TABLEID,
    prgcolumnid: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenTempTable3(
                sesid: JET_SESID,
                prgcolumndef: *const JET_COLUMNDEF,
                ccolumn: u32,
                pidxunicode: *const JET_UNICODEINDEX,
                grbit: u32,
                ptableid: *mut JET_TABLEID,
                prgcolumnid: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenTempTable3(
            sesid.into_param().abi(),
            ::std::mem::transmute(prgcolumndef),
            ::std::mem::transmute(ccolumn),
            ::std::mem::transmute(pidxunicode),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(ptableid),
            ::std::mem::transmute(prgcolumnid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetOpenTemporaryTable<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    popentemporarytable: *const JET_OPENTEMPORARYTABLE,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenTemporaryTable(
                sesid: JET_SESID,
                popentemporarytable: *const JET_OPENTEMPORARYTABLE,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenTemporaryTable(
            sesid.into_param().abi(),
            ::std::mem::transmute(popentemporarytable),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn JetOpenTemporaryTable2<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    popentemporarytable: *const JET_OPENTEMPORARYTABLE2,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetOpenTemporaryTable2(
                sesid: JET_SESID,
                popentemporarytable: *const JET_OPENTEMPORARYTABLE2,
            ) -> i32;
        }
        ::std::mem::transmute(JetOpenTemporaryTable2(
            sesid.into_param().abi(),
            ::std::mem::transmute(popentemporarytable),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetPrepareUpdate<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    prep: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetPrepareUpdate(sesid: JET_SESID, tableid: JET_TABLEID, prep: u32) -> i32;
        }
        ::std::mem::transmute(JetPrepareUpdate(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(prep),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetPrereadIndexRanges<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    rgindexranges: *const JET_INDEX_RANGE,
    cindexranges: u32,
    pcrangespreread: *mut u32,
    rgcolumnidpreread: *const u32,
    ccolumnidpreread: u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetPrereadIndexRanges(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                rgindexranges: *const JET_INDEX_RANGE,
                cindexranges: u32,
                pcrangespreread: *mut u32,
                rgcolumnidpreread: *const u32,
                ccolumnidpreread: u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetPrereadIndexRanges(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(rgindexranges),
            ::std::mem::transmute(cindexranges),
            ::std::mem::transmute(pcrangespreread),
            ::std::mem::transmute(rgcolumnidpreread),
            ::std::mem::transmute(ccolumnidpreread),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetPrereadKeys<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    rgpvkeys: *const *const ::std::ffi::c_void,
    rgcbkeys: *const u32,
    ckeys: i32,
    pckeyspreread: *mut i32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetPrereadKeys(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                rgpvkeys: *const *const ::std::ffi::c_void,
                rgcbkeys: *const u32,
                ckeys: i32,
                pckeyspreread: *mut i32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetPrereadKeys(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(rgpvkeys),
            ::std::mem::transmute(rgcbkeys),
            ::std::mem::transmute(ckeys),
            ::std::mem::transmute(pckeyspreread),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetReadFile<'a, Param0: ::windows::runtime::IntoParam<'a, JET_HANDLE>>(
    hffile: Param0,
    pv: *mut ::std::ffi::c_void,
    cb: u32,
    pcbactual: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetReadFile(
                hffile: JET_HANDLE,
                pv: *mut ::std::ffi::c_void,
                cb: u32,
                pcbactual: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetReadFile(
            hffile.into_param().abi(),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetReadFileInstance<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, JET_HANDLE>,
>(
    instance: Param0,
    hffile: Param1,
    pv: *mut ::std::ffi::c_void,
    cb: u32,
    pcbactual: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetReadFileInstance(
                instance: JET_INSTANCE,
                hffile: JET_HANDLE,
                pv: *mut ::std::ffi::c_void,
                cb: u32,
                pcbactual: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetReadFileInstance(
            instance.into_param().abi(),
            hffile.into_param().abi(),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRegisterCallback<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    cbtyp: u32,
    pcallback: ::std::option::Option<JET_CALLBACK>,
    pvcontext: *const ::std::ffi::c_void,
    phcallbackid: *const JET_HANDLE,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRegisterCallback(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                cbtyp: u32,
                pcallback: ::windows::runtime::RawPtr,
                pvcontext: *const ::std::ffi::c_void,
                phcallbackid: *const JET_HANDLE,
            ) -> i32;
        }
        ::std::mem::transmute(JetRegisterCallback(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(cbtyp),
            ::std::mem::transmute(pcallback),
            ::std::mem::transmute(pvcontext),
            ::std::mem::transmute(phcallbackid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRenameColumnA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szname: *const i8,
    sznamenew: *const i8,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRenameColumnA(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szname: *const i8,
                sznamenew: *const i8,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetRenameColumnA(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szname),
            ::std::mem::transmute(sznamenew),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRenameColumnW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szname: *const u16,
    sznamenew: *const u16,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRenameColumnW(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szname: *const u16,
                sznamenew: *const u16,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetRenameColumnW(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szname),
            ::std::mem::transmute(sznamenew),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRenameTableA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    szname: *const i8,
    sznamenew: *const i8,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRenameTableA(
                sesid: JET_SESID,
                dbid: u32,
                szname: *const i8,
                sznamenew: *const i8,
            ) -> i32;
        }
        ::std::mem::transmute(JetRenameTableA(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(szname),
            ::std::mem::transmute(sznamenew),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRenameTableW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    szname: *const u16,
    sznamenew: *const u16,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRenameTableW(
                sesid: JET_SESID,
                dbid: u32,
                szname: *const u16,
                sznamenew: *const u16,
            ) -> i32;
        }
        ::std::mem::transmute(JetRenameTableW(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(szname),
            ::std::mem::transmute(sznamenew),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetResetSessionContext<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetResetSessionContext(sesid: JET_SESID) -> i32;
        }
        ::std::mem::transmute(JetResetSessionContext(sesid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetResetTableSequential<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetResetTableSequential(sesid: JET_SESID, tableid: JET_TABLEID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetResetTableSequential(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetResizeDatabase<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    dbid: u32,
    cpgtarget: u32,
    pcpgactual: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetResizeDatabase(
                sesid: JET_SESID,
                dbid: u32,
                cpgtarget: u32,
                pcpgactual: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetResizeDatabase(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(cpgtarget),
            ::std::mem::transmute(pcpgactual),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRestore2A(
    sz: *const i8,
    szdest: *const i8,
    pfn: ::std::option::Option<JET_PFNSTATUS>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRestore2A(
                sz: *const i8,
                szdest: *const i8,
                pfn: ::windows::runtime::RawPtr,
            ) -> i32;
        }
        ::std::mem::transmute(JetRestore2A(
            ::std::mem::transmute(sz),
            ::std::mem::transmute(szdest),
            ::std::mem::transmute(pfn),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRestore2W(
    sz: *const u16,
    szdest: *const u16,
    pfn: ::std::option::Option<JET_PFNSTATUS>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRestore2W(
                sz: *const u16,
                szdest: *const u16,
                pfn: ::windows::runtime::RawPtr,
            ) -> i32;
        }
        ::std::mem::transmute(JetRestore2W(
            ::std::mem::transmute(sz),
            ::std::mem::transmute(szdest),
            ::std::mem::transmute(pfn),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRestoreA(szsource: *const i8, pfn: ::std::option::Option<JET_PFNSTATUS>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRestoreA(szsource: *const i8, pfn: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(JetRestoreA(
            ::std::mem::transmute(szsource),
            ::std::mem::transmute(pfn),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRestoreInstanceA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>>(
    instance: Param0,
    sz: *const i8,
    szdest: *const i8,
    pfn: ::std::option::Option<JET_PFNSTATUS>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRestoreInstanceA(
                instance: JET_INSTANCE,
                sz: *const i8,
                szdest: *const i8,
                pfn: ::windows::runtime::RawPtr,
            ) -> i32;
        }
        ::std::mem::transmute(JetRestoreInstanceA(
            instance.into_param().abi(),
            ::std::mem::transmute(sz),
            ::std::mem::transmute(szdest),
            ::std::mem::transmute(pfn),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRestoreInstanceW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>>(
    instance: Param0,
    sz: *const u16,
    szdest: *const u16,
    pfn: ::std::option::Option<JET_PFNSTATUS>,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRestoreInstanceW(
                instance: JET_INSTANCE,
                sz: *const u16,
                szdest: *const u16,
                pfn: ::windows::runtime::RawPtr,
            ) -> i32;
        }
        ::std::mem::transmute(JetRestoreInstanceW(
            instance.into_param().abi(),
            ::std::mem::transmute(sz),
            ::std::mem::transmute(szdest),
            ::std::mem::transmute(pfn),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRestoreW(szsource: *const u16, pfn: ::std::option::Option<JET_PFNSTATUS>) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRestoreW(szsource: *const u16, pfn: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(JetRestoreW(
            ::std::mem::transmute(szsource),
            ::std::mem::transmute(pfn),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRetrieveColumn<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    columnid: u32,
    pvdata: *mut ::std::ffi::c_void,
    cbdata: u32,
    pcbactual: *mut u32,
    grbit: u32,
    pretinfo: *mut JET_RETINFO,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRetrieveColumn(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                columnid: u32,
                pvdata: *mut ::std::ffi::c_void,
                cbdata: u32,
                pcbactual: *mut u32,
                grbit: u32,
                pretinfo: *mut JET_RETINFO,
            ) -> i32;
        }
        ::std::mem::transmute(JetRetrieveColumn(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(columnid),
            ::std::mem::transmute(pvdata),
            ::std::mem::transmute(cbdata),
            ::std::mem::transmute(pcbactual),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(pretinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRetrieveColumns<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pretrievecolumn: *mut JET_RETRIEVECOLUMN,
    cretrievecolumn: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRetrieveColumns(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pretrievecolumn: *mut JET_RETRIEVECOLUMN,
                cretrievecolumn: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetRetrieveColumns(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pretrievecolumn),
            ::std::mem::transmute(cretrievecolumn),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRetrieveKey<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pvkey: *mut ::std::ffi::c_void,
    cbmax: u32,
    pcbactual: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRetrieveKey(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pvkey: *mut ::std::ffi::c_void,
                cbmax: u32,
                pcbactual: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetRetrieveKey(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pvkey),
            ::std::mem::transmute(cbmax),
            ::std::mem::transmute(pcbactual),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetRollback<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetRollback(sesid: JET_SESID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetRollback(
            sesid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSeek<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSeek(sesid: JET_SESID, tableid: JET_TABLEID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetSeek(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetColumn<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    columnid: u32,
    pvdata: *const ::std::ffi::c_void,
    cbdata: u32,
    grbit: u32,
    psetinfo: *const JET_SETINFO,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetColumn(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                columnid: u32,
                pvdata: *const ::std::ffi::c_void,
                cbdata: u32,
                grbit: u32,
                psetinfo: *const JET_SETINFO,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetColumn(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(columnid),
            ::std::mem::transmute(pvdata),
            ::std::mem::transmute(cbdata),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(psetinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetColumnDefaultValueA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const i8,
    szcolumnname: *const i8,
    pvdata: *const ::std::ffi::c_void,
    cbdata: u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetColumnDefaultValueA(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const i8,
                szcolumnname: *const i8,
                pvdata: *const ::std::ffi::c_void,
                cbdata: u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetColumnDefaultValueA(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(szcolumnname),
            ::std::mem::transmute(pvdata),
            ::std::mem::transmute(cbdata),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetColumnDefaultValueW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
>(
    sesid: Param0,
    dbid: u32,
    sztablename: *const u16,
    szcolumnname: *const u16,
    pvdata: *const ::std::ffi::c_void,
    cbdata: u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetColumnDefaultValueW(
                sesid: JET_SESID,
                dbid: u32,
                sztablename: *const u16,
                szcolumnname: *const u16,
                pvdata: *const ::std::ffi::c_void,
                cbdata: u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetColumnDefaultValueW(
            sesid.into_param().abi(),
            ::std::mem::transmute(dbid),
            ::std::mem::transmute(sztablename),
            ::std::mem::transmute(szcolumnname),
            ::std::mem::transmute(pvdata),
            ::std::mem::transmute(cbdata),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetColumns<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    psetcolumn: *const JET_SETCOLUMN,
    csetcolumn: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetColumns(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                psetcolumn: *const JET_SETCOLUMN,
                csetcolumn: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetColumns(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(psetcolumn),
            ::std::mem::transmute(csetcolumn),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetCurrentIndex2A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const i8,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetCurrentIndex2A(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const i8,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetCurrentIndex2A(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetCurrentIndex2W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const u16,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetCurrentIndex2W(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const u16,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetCurrentIndex2W(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetCurrentIndex3A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const i8,
    grbit: u32,
    itagsequence: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetCurrentIndex3A(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const i8,
                grbit: u32,
                itagsequence: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetCurrentIndex3A(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(itagsequence),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetCurrentIndex3W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const u16,
    grbit: u32,
    itagsequence: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetCurrentIndex3W(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const u16,
                grbit: u32,
                itagsequence: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetCurrentIndex3W(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(itagsequence),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetCurrentIndex4A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const i8,
    pindexid: *const JET_INDEXID,
    grbit: u32,
    itagsequence: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetCurrentIndex4A(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const i8,
                pindexid: *const JET_INDEXID,
                grbit: u32,
                itagsequence: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetCurrentIndex4A(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(pindexid),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(itagsequence),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetCurrentIndex4W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const u16,
    pindexid: *const JET_INDEXID,
    grbit: u32,
    itagsequence: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetCurrentIndex4W(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const u16,
                pindexid: *const JET_INDEXID,
                grbit: u32,
                itagsequence: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetCurrentIndex4W(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
            ::std::mem::transmute(pindexid),
            ::std::mem::transmute(grbit),
            ::std::mem::transmute(itagsequence),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetCurrentIndexA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const i8,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetCurrentIndexA(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const i8,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetCurrentIndexA(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetCurrentIndexW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    szindexname: *const u16,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetCurrentIndexW(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                szindexname: *const u16,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetCurrentIndexW(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(szindexname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetCursorFilter<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    rgcolumnfilters: *const JET_INDEX_COLUMN,
    ccolumnfilters: u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetCursorFilter(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                rgcolumnfilters: *const JET_INDEX_COLUMN,
                ccolumnfilters: u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetCursorFilter(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(rgcolumnfilters),
            ::std::mem::transmute(ccolumnfilters),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetDatabaseSizeA<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szdatabasename: *const i8,
    cpg: u32,
    pcpgreal: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetDatabaseSizeA(
                sesid: JET_SESID,
                szdatabasename: *const i8,
                cpg: u32,
                pcpgreal: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetDatabaseSizeA(
            sesid.into_param().abi(),
            ::std::mem::transmute(szdatabasename),
            ::std::mem::transmute(cpg),
            ::std::mem::transmute(pcpgreal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetDatabaseSizeW<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    szdatabasename: *const u16,
    cpg: u32,
    pcpgreal: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetDatabaseSizeW(
                sesid: JET_SESID,
                szdatabasename: *const u16,
                cpg: u32,
                pcpgreal: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetDatabaseSizeW(
            sesid.into_param().abi(),
            ::std::mem::transmute(szdatabasename),
            ::std::mem::transmute(cpg),
            ::std::mem::transmute(pcpgreal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetIndexRange<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableidsrc: Param1,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetIndexRange(sesid: JET_SESID, tableidsrc: JET_TABLEID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetSetIndexRange(
            sesid.into_param().abi(),
            tableidsrc.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetLS<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
    Param2: ::windows::runtime::IntoParam<'a, JET_LS>,
>(
    sesid: Param0,
    tableid: Param1,
    ls: Param2,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetLS(sesid: JET_SESID, tableid: JET_TABLEID, ls: JET_LS, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetSetLS(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ls.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetSessionContext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_API_PTR>,
>(
    sesid: Param0,
    ulcontext: Param1,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetSessionContext(sesid: JET_SESID, ulcontext: JET_API_PTR) -> i32;
        }
        ::std::mem::transmute(JetSetSessionContext(
            sesid.into_param().abi(),
            ulcontext.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetSessionParameter<'a, Param0: ::windows::runtime::IntoParam<'a, JET_SESID>>(
    sesid: Param0,
    sesparamid: u32,
    pvparam: *const ::std::ffi::c_void,
    cbparam: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetSessionParameter(
                sesid: JET_SESID,
                sesparamid: u32,
                pvparam: *const ::std::ffi::c_void,
                cbparam: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetSessionParameter(
            sesid.into_param().abi(),
            ::std::mem::transmute(sesparamid),
            ::std::mem::transmute(pvparam),
            ::std::mem::transmute(cbparam),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetSystemParameterA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param3: ::windows::runtime::IntoParam<'a, JET_API_PTR>,
>(
    pinstance: *mut JET_INSTANCE,
    sesid: Param1,
    paramid: u32,
    lparam: Param3,
    szparam: *const i8,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetSystemParameterA(
                pinstance: *mut JET_INSTANCE,
                sesid: JET_SESID,
                paramid: u32,
                lparam: JET_API_PTR,
                szparam: *const i8,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetSystemParameterA(
            ::std::mem::transmute(pinstance),
            sesid.into_param().abi(),
            ::std::mem::transmute(paramid),
            lparam.into_param().abi(),
            ::std::mem::transmute(szparam),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetSystemParameterW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param3: ::windows::runtime::IntoParam<'a, JET_API_PTR>,
>(
    pinstance: *mut JET_INSTANCE,
    sesid: Param1,
    paramid: u32,
    lparam: Param3,
    szparam: *const u16,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetSystemParameterW(
                pinstance: *mut JET_INSTANCE,
                sesid: JET_SESID,
                paramid: u32,
                lparam: JET_API_PTR,
                szparam: *const u16,
            ) -> i32;
        }
        ::std::mem::transmute(JetSetSystemParameterW(
            ::std::mem::transmute(pinstance),
            sesid.into_param().abi(),
            ::std::mem::transmute(paramid),
            lparam.into_param().abi(),
            ::std::mem::transmute(szparam),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetSetTableSequential<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetSetTableSequential(sesid: JET_SESID, tableid: JET_TABLEID, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetSetTableSequential(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetStopBackup() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetStopBackup() -> i32;
        }
        ::std::mem::transmute(JetStopBackup())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetStopBackupInstance<'a, Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>>(
    instance: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetStopBackupInstance(instance: JET_INSTANCE) -> i32;
        }
        ::std::mem::transmute(JetStopBackupInstance(instance.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetStopService() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetStopService() -> i32;
        }
        ::std::mem::transmute(JetStopService())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetStopServiceInstance<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetStopServiceInstance(instance: JET_INSTANCE) -> i32;
        }
        ::std::mem::transmute(JetStopServiceInstance(instance.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetStopServiceInstance2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetStopServiceInstance2(instance: JET_INSTANCE, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetStopServiceInstance2(
            instance.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetTerm<'a, Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>>(
    instance: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetTerm(instance: JET_INSTANCE) -> i32;
        }
        ::std::mem::transmute(JetTerm(instance.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetTerm2<'a, Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>>(
    instance: Param0,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetTerm2(instance: JET_INSTANCE, grbit: u32) -> i32;
        }
        ::std::mem::transmute(JetTerm2(
            instance.into_param().abi(),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetTruncateLog() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetTruncateLog() -> i32;
        }
        ::std::mem::transmute(JetTruncateLog())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetTruncateLogInstance<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_INSTANCE>,
>(
    instance: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetTruncateLogInstance(instance: JET_INSTANCE) -> i32;
        }
        ::std::mem::transmute(JetTruncateLogInstance(instance.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetUnregisterCallback<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
    Param3: ::windows::runtime::IntoParam<'a, JET_HANDLE>,
>(
    sesid: Param0,
    tableid: Param1,
    cbtyp: u32,
    hcallbackid: Param3,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetUnregisterCallback(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                cbtyp: u32,
                hcallbackid: JET_HANDLE,
            ) -> i32;
        }
        ::std::mem::transmute(JetUnregisterCallback(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(cbtyp),
            hcallbackid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetUpdate<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pvbookmark: *mut ::std::ffi::c_void,
    cbbookmark: u32,
    pcbactual: *mut u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetUpdate(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pvbookmark: *mut ::std::ffi::c_void,
                cbbookmark: u32,
                pcbactual: *mut u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetUpdate(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pvbookmark),
            ::std::mem::transmute(cbbookmark),
            ::std::mem::transmute(pcbactual),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn JetUpdate2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, JET_SESID>,
    Param1: ::windows::runtime::IntoParam<'a, JET_TABLEID>,
>(
    sesid: Param0,
    tableid: Param1,
    pvbookmark: *mut ::std::ffi::c_void,
    cbbookmark: u32,
    pcbactual: *mut u32,
    grbit: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "esent")]
        extern "system" {
            fn JetUpdate2(
                sesid: JET_SESID,
                tableid: JET_TABLEID,
                pvbookmark: *mut ::std::ffi::c_void,
                cbbookmark: u32,
                pcbactual: *mut u32,
                grbit: u32,
            ) -> i32;
        }
        ::std::mem::transmute(JetUpdate2(
            sesid.into_param().abi(),
            tableid.into_param().abi(),
            ::std::mem::transmute(pvbookmark),
            ::std::mem::transmute(cbbookmark),
            ::std::mem::transmute(pcbactual),
            ::std::mem::transmute(grbit),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
#[cfg(feature = "Win32_System_Com")]
pub unsafe fn OleConvertIStorageToOLESTREAM<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, IStorage>,
>(
    pstg: Param0,
    lpolestream: *mut super::super::System::Com::OLESTREAM,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn OleConvertIStorageToOLESTREAM(
                pstg: ::windows::runtime::RawPtr,
                lpolestream: *mut super::super::System::Com::OLESTREAM,
            ) -> ::windows::runtime::HRESULT;
        }
        OleConvertIStorageToOLESTREAM(pstg.into_param().abi(), ::std::mem::transmute(lpolestream))
            .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Gdi",
    feature = "Win32_System_Com"
))]
pub unsafe fn OleConvertIStorageToOLESTREAMEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, IStorage>,
>(
    pstg: Param0,
    cfformat: u16,
    lwidth: i32,
    lheight: i32,
    dwsize: u32,
    pmedium: *mut super::super::System::Com::STGMEDIUM,
    polestm: *mut super::super::System::Com::OLESTREAM,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn OleConvertIStorageToOLESTREAMEx(
                pstg: ::windows::runtime::RawPtr,
                cfformat: u16,
                lwidth: i32,
                lheight: i32,
                dwsize: u32,
                pmedium: *mut ::std::mem::ManuallyDrop<super::super::System::Com::STGMEDIUM>,
                polestm: *mut super::super::System::Com::OLESTREAM,
            ) -> ::windows::runtime::HRESULT;
        }
        OleConvertIStorageToOLESTREAMEx(
            pstg.into_param().abi(),
            ::std::mem::transmute(cfformat),
            ::std::mem::transmute(lwidth),
            ::std::mem::transmute(lheight),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(pmedium),
            ::std::mem::transmute(polestm),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
pub unsafe fn OleConvertOLESTREAMToIStorage<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, IStorage>,
>(
    lpolestream: *mut super::super::System::Com::OLESTREAM,
    pstg: Param1,
    ptd: *const super::super::System::Com::DVTARGETDEVICE,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn OleConvertOLESTREAMToIStorage(
                lpolestream: *mut super::super::System::Com::OLESTREAM,
                pstg: ::windows::runtime::RawPtr,
                ptd: *const super::super::System::Com::DVTARGETDEVICE,
            ) -> ::windows::runtime::HRESULT;
        }
        OleConvertOLESTREAMToIStorage(
            ::std::mem::transmute(lpolestream),
            pstg.into_param().abi(),
            ::std::mem::transmute(ptd),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Graphics_Gdi",
    feature = "Win32_System_Com"
))]
pub unsafe fn OleConvertOLESTREAMToIStorageEx<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, IStorage>,
>(
    polestm: *mut super::super::System::Com::OLESTREAM,
    pstg: Param1,
    pcfformat: *mut u16,
    plwwidth: *mut i32,
    plheight: *mut i32,
    pdwsize: *mut u32,
    pmedium: *mut super::super::System::Com::STGMEDIUM,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn OleConvertOLESTREAMToIStorageEx(
                polestm: *mut super::super::System::Com::OLESTREAM,
                pstg: ::windows::runtime::RawPtr,
                pcfformat: *mut u16,
                plwwidth: *mut i32,
                plheight: *mut i32,
                pdwsize: *mut u32,
                pmedium: *mut ::std::mem::ManuallyDrop<super::super::System::Com::STGMEDIUM>,
            ) -> ::windows::runtime::HRESULT;
        }
        OleConvertOLESTREAMToIStorageEx(
            ::std::mem::transmute(polestm),
            pstg.into_param().abi(),
            ::std::mem::transmute(pcfformat),
            ::std::mem::transmute(plwwidth),
            ::std::mem::transmute(plheight),
            ::std::mem::transmute(pdwsize),
            ::std::mem::transmute(pmedium),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const PIDDI_THUMBNAIL: i32 = 2i32;
pub const PIDDSI_BYTECOUNT: u32 = 4u32;
pub const PIDDSI_CATEGORY: u32 = 2u32;
pub const PIDDSI_COMPANY: u32 = 15u32;
pub const PIDDSI_DOCPARTS: u32 = 13u32;
pub const PIDDSI_HEADINGPAIR: u32 = 12u32;
pub const PIDDSI_HIDDENCOUNT: u32 = 9u32;
pub const PIDDSI_LINECOUNT: u32 = 5u32;
pub const PIDDSI_LINKSDIRTY: u32 = 16u32;
pub const PIDDSI_MANAGER: u32 = 14u32;
pub const PIDDSI_MMCLIPCOUNT: u32 = 10u32;
pub const PIDDSI_NOTECOUNT: u32 = 8u32;
pub const PIDDSI_PARCOUNT: u32 = 6u32;
pub const PIDDSI_PRESFORMAT: u32 = 3u32;
pub const PIDDSI_SCALE: u32 = 11u32;
pub const PIDDSI_SLIDECOUNT: u32 = 7u32;
pub const PIDMSI_COPYRIGHT: i32 = 11i32;
pub const PIDMSI_EDITOR: i32 = 2i32;
pub const PIDMSI_OWNER: i32 = 8i32;
pub const PIDMSI_PRODUCTION: i32 = 10i32;
pub const PIDMSI_PROJECT: i32 = 6i32;
pub const PIDMSI_RATING: i32 = 9i32;
pub const PIDMSI_SEQUENCE_NO: i32 = 5i32;
pub const PIDMSI_SOURCE: i32 = 4i32;
pub const PIDMSI_STATUS: i32 = 7i32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
pub const PIDMSI_SUPPLIER: i32 = 3i32;
pub const PIDSI_APPNAME: i32 = 18i32;
pub const PIDSI_AUTHOR: i32 = 4i32;
pub const PIDSI_CHARCOUNT: i32 = 16i32;
pub const PIDSI_COMMENTS: i32 = 6i32;
pub const PIDSI_CREATE_DTM: i32 = 12i32;
pub const PIDSI_DOC_SECURITY: i32 = 19i32;
pub const PIDSI_EDITTIME: i32 = 10i32;
pub const PIDSI_KEYWORDS: i32 = 5i32;
pub const PIDSI_LASTAUTHOR: i32 = 8i32;
pub const PIDSI_LASTPRINTED: i32 = 11i32;
pub const PIDSI_LASTSAVE_DTM: i32 = 13i32;
pub const PIDSI_PAGECOUNT: i32 = 14i32;
pub const PIDSI_REVNUMBER: i32 = 9i32;
pub const PIDSI_SUBJECT: i32 = 3i32;
pub const PIDSI_TEMPLATE: i32 = 7i32;
pub const PIDSI_THUMBNAIL: i32 = 17i32;
pub const PIDSI_TITLE: i32 = 2i32;
pub const PIDSI_WORDCOUNT: i32 = 15i32;
pub const PID_BEHAVIOR: u32 = 2147483651u32;
pub const PID_FIRST_NAME_DEFAULT: u32 = 4095u32;
pub const PID_ILLEGAL: u32 = 4294967295u32;
pub const PID_LOCALE: u32 = 2147483648u32;
pub const PID_MAX_READONLY: u32 = 3221225471u32;
pub const PID_MIN_READONLY: u32 = 2147483648u32;
pub const PID_MODIFY_TIME: u32 = 2147483649u32;
#[repr(C)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
)]
pub struct PMemoryAllocator(pub u8);
pub const PROPSETFLAG_ANSI: u32 = 2u32;
pub const PROPSETFLAG_CASE_SENSITIVE: u32 = 8u32;
pub const PROPSETFLAG_DEFAULT: u32 = 0u32;
pub const PROPSETFLAG_NONSIMPLE: u32 = 1u32;
pub const PROPSETFLAG_UNBUFFERED: u32 = 4u32;
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295u32;
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PROPSPEC_0 {
    pub propid: u32,
    pub lpwstr: super::super::Foundation::PWSTR,
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
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
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
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::clone::Clone for PROPVARIANT {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl PROPVARIANT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for PROPVARIANT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for PROPVARIANT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for PROPVARIANT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for PROPVARIANT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::clone::Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub union PROPVARIANT_0 {
    pub Anonymous: ::std::mem::ManuallyDrop<PROPVARIANT_0_0>,
    pub decVal: super::super::System::SystemServices::DECIMAL,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl PROPVARIANT_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for PROPVARIANT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for PROPVARIANT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for PROPVARIANT_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for PROPVARIANT_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::clone::Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub struct PROPVARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl PROPVARIANT_0_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for PROPVARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for PROPVARIANT_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for PROPVARIANT_0_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for PROPVARIANT_0_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::clone::Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub union PROPVARIANT_0_0_0 {
    pub cVal: super::super::System::SystemServices::CHAR,
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
    pub cyVal: super::super::System::SystemServices::CY,
    pub date: f64,
    pub filetime: super::super::Foundation::FILETIME,
    pub puuid: *mut ::windows::runtime::GUID,
    pub pclipdata: *mut super::super::System::SystemServices::CLIPDATA,
    pub bstrVal: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub bstrblobVal: super::super::System::SystemServices::BSTRBLOB,
    pub blob: super::super::System::Com::BLOB,
    pub pszVal: super::super::Foundation::PSTR,
    pub pwszVal: super::super::Foundation::PWSTR,
    pub punkVal: ::windows::runtime::RawPtr,
    pub pdispVal: ::windows::runtime::RawPtr,
    pub pStream: ::windows::runtime::RawPtr,
    pub pStorage: ::windows::runtime::RawPtr,
    pub pVersionedStream: *mut ::std::mem::ManuallyDrop<VERSIONEDSTREAM>,
    pub parray: *mut super::super::System::OleAutomation::SAFEARRAY,
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
    pub pcVal: super::super::Foundation::PSTR,
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
    pub pdecVal: *mut super::super::System::SystemServices::DECIMAL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::super::System::SystemServices::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub ppunkVal: *mut ::windows::runtime::RawPtr,
    pub ppdispVal: *mut ::windows::runtime::RawPtr,
    pub pparray: *mut *mut super::super::System::OleAutomation::SAFEARRAY,
    pub pvarVal: *mut ::std::mem::ManuallyDrop<PROPVARIANT>,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl PROPVARIANT_0_0_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for PROPVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for PROPVARIANT_0_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for PROPVARIANT_0_0_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for PROPVARIANT_0_0_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const PRSPEC_INVALID: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn PropStgNameToFmtId<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    oszname: Param0,
) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn PropStgNameToFmtId(
                oszname: super::super::Foundation::PWSTR,
                pfmtid: *mut ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        PropStgNameToFmtId(oszname.into_param().abi(), &mut result__)
            .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn PropVariantClear(pvar: *mut PROPVARIANT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn PropVariantClear(
                pvar: *mut ::std::mem::ManuallyDrop<PROPVARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        PropVariantClear(::std::mem::transmute(pvar)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn PropVariantCopy(
    pvardest: *mut PROPVARIANT,
    pvarsrc: *const PROPVARIANT,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn PropVariantCopy(
                pvardest: *mut ::std::mem::ManuallyDrop<PROPVARIANT>,
                pvarsrc: *const ::std::mem::ManuallyDrop<PROPVARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        PropVariantCopy(
            ::std::mem::transmute(pvardest),
            ::std::mem::transmute(pvarsrc),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ReadClassStg<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(
    pstg: Param0,
) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn ReadClassStg(
                pstg: ::windows::runtime::RawPtr,
                pclsid: *mut ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        ReadClassStg(pstg.into_param().abi(), &mut result__)
            .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ReadClassStm<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>>(
    pstm: Param0,
) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn ReadClassStm(
                pstm: ::windows::runtime::RawPtr,
                pclsid: *mut ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        ReadClassStm(pstm.into_param().abi(), &mut result__)
            .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReadFmtUserTypeStg<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(
    pstg: Param0,
    pcf: *mut u16,
    lplpszusertype: *mut super::super::Foundation::PWSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn ReadFmtUserTypeStg(
                pstg: ::windows::runtime::RawPtr,
                pcf: *mut u16,
                lplpszusertype: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        ReadFmtUserTypeStg(
            pstg.into_param().abi(),
            ::std::mem::transmute(pcf),
            ::std::mem::transmute(lplpszusertype),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("RemSNB")
            .field("ulCntStr", &self.ulCntStr)
            .field("ulCntChar", &self.ulCntChar)
            .field("rgString", &self.rgString)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RemSNB {
    fn eq(&self, other: &Self) -> bool {
        self.ulCntStr == other.ulCntStr
            && self.ulCntChar == other.ulCntChar
            && self.rgString == other.rgString
    }
}
impl ::std::cmp::Eq for RemSNB {}
unsafe impl ::windows::runtime::Abi for RemSNB {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SERIALIZEDPROPERTYVALUE")
            .field("dwType", &self.dwType)
            .field("rgb", &self.rgb)
            .finish()
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
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STATPROPSETSTG {
    pub fmtid: ::windows::runtime::GUID,
    pub clsid: ::windows::runtime::GUID,
    pub grfFlags: u32,
    pub mtime: super::super::Foundation::FILETIME,
    pub ctime: super::super::Foundation::FILETIME,
    pub atime: super::super::Foundation::FILETIME,
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
        fmt.debug_struct("STATPROPSETSTG")
            .field("fmtid", &self.fmtid)
            .field("clsid", &self.clsid)
            .field("grfFlags", &self.grfFlags)
            .field("mtime", &self.mtime)
            .field("ctime", &self.ctime)
            .field("atime", &self.atime)
            .field("dwOSVersion", &self.dwOSVersion)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STATPROPSETSTG {
    fn eq(&self, other: &Self) -> bool {
        self.fmtid == other.fmtid
            && self.clsid == other.clsid
            && self.grfFlags == other.grfFlags
            && self.mtime == other.mtime
            && self.ctime == other.ctime
            && self.atime == other.atime
            && self.dwOSVersion == other.dwOSVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STATPROPSETSTG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STATPROPSETSTG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STATPROPSTG {
    pub lpwstrName: super::super::Foundation::PWSTR,
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
        fmt.debug_struct("STATPROPSTG")
            .field("lpwstrName", &self.lpwstrName)
            .field("propid", &self.propid)
            .field("vt", &self.vt)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STATSTG {
    pub pwcsName: super::super::Foundation::PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: super::super::Foundation::FILETIME,
    pub ctime: super::super::Foundation::FILETIME,
    pub atime: super::super::Foundation::FILETIME,
    pub grfMode: u32,
    pub grfLocksSupported: u32,
    pub clsid: ::windows::runtime::GUID,
    pub grfStateBits: u32,
    pub reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl STATSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STATSTG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STATSTG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STATSTG")
            .field("pwcsName", &self.pwcsName)
            .field("r#type", &self.r#type)
            .field("cbSize", &self.cbSize)
            .field("mtime", &self.mtime)
            .field("ctime", &self.ctime)
            .field("atime", &self.atime)
            .field("grfMode", &self.grfMode)
            .field("grfLocksSupported", &self.grfLocksSupported)
            .field("clsid", &self.clsid)
            .field("grfStateBits", &self.grfStateBits)
            .field("reserved", &self.reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STATSTG {
    fn eq(&self, other: &Self) -> bool {
        self.pwcsName == other.pwcsName
            && self.r#type == other.r#type
            && self.cbSize == other.cbSize
            && self.mtime == other.mtime
            && self.ctime == other.ctime
            && self.atime == other.atime
            && self.grfMode == other.grfMode
            && self.grfLocksSupported == other.grfLocksSupported
            && self.clsid == other.clsid
            && self.grfStateBits == other.grfStateBits
            && self.reserved == other.reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STATSTG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STATSTG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
pub const STGFMT_ANY: u32 = 4u32;
pub const STGFMT_DOCFILE: u32 = 5u32;
pub const STGFMT_DOCUMENT: u32 = 0u32;
pub const STGFMT_FILE: u32 = 3u32;
pub const STGFMT_NATIVE: u32 = 1u32;
pub const STGFMT_STORAGE: u32 = 0u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
pub const STGM_CONVERT: i32 = 131072i32;
pub const STGM_CREATE: i32 = 4096i32;
pub const STGM_DELETEONRELEASE: i32 = 67108864i32;
pub const STGM_DIRECT: i32 = 0i32;
pub const STGM_DIRECT_SWMR: i32 = 4194304i32;
pub const STGM_FAILIFTHERE: i32 = 0i32;
pub const STGM_NOSCRATCH: i32 = 1048576i32;
pub const STGM_NOSNAPSHOT: i32 = 2097152i32;
pub const STGM_PRIORITY: i32 = 262144i32;
pub const STGM_READ: i32 = 0i32;
pub const STGM_READWRITE: i32 = 2i32;
pub const STGM_SHARE_DENY_NONE: i32 = 64i32;
pub const STGM_SHARE_DENY_READ: i32 = 48i32;
pub const STGM_SHARE_DENY_WRITE: i32 = 32i32;
pub const STGM_SHARE_EXCLUSIVE: i32 = 16i32;
pub const STGM_SIMPLE: i32 = 134217728i32;
pub const STGM_TRANSACTED: i32 = 65536i32;
pub const STGM_WRITE: i32 = 1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STGOPTIONS {
    pub usVersion: u16,
    pub reserved: u16,
    pub ulSectorSize: u32,
    pub pwcsTemplateFile: super::super::Foundation::PWSTR,
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
        fmt.debug_struct("STGOPTIONS")
            .field("usVersion", &self.usVersion)
            .field("reserved", &self.reserved)
            .field("ulSectorSize", &self.ulSectorSize)
            .field("pwcsTemplateFile", &self.pwcsTemplateFile)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STGOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.usVersion == other.usVersion
            && self.reserved == other.reserved
            && self.ulSectorSize == other.ulSectorSize
            && self.pwcsTemplateFile == other.pwcsTemplateFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STGOPTIONS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STGOPTIONS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STGOPTIONS_VERSION: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct STGTY(pub i32);
pub const STGTY_STORAGE: STGTY = STGTY(1i32);
pub const STGTY_STREAM: STGTY = STGTY(2i32);
pub const STGTY_LOCKBYTES: STGTY = STGTY(3i32);
pub const STGTY_PROPERTY: STGTY = STGTY(4i32);
impl ::std::convert::From<i32> for STGTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STGTY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STGTY_REPEAT: i32 = 256i32;
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
pub const STG_TOEND: i32 = -1i32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct STREAM_SEEK(pub u32);
pub const STREAM_SEEK_SET: STREAM_SEEK = STREAM_SEEK(0u32);
pub const STREAM_SEEK_CUR: STREAM_SEEK = STREAM_SEEK(1u32);
pub const STREAM_SEEK_END: STREAM_SEEK = STREAM_SEEK(2u32);
impl ::std::convert::From<u32> for STREAM_SEEK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STREAM_SEEK {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for STREAM_SEEK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for STREAM_SEEK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for STREAM_SEEK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for STREAM_SEEK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for STREAM_SEEK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetConvertStg<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, IStorage>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    pstg: Param0,
    fconvert: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn SetConvertStg(
                pstg: ::windows::runtime::RawPtr,
                fconvert: super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        SetConvertStg(pstg.into_param().abi(), fconvert.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn StgConvertPropertyToVariant(
    pprop: *const SERIALIZEDPROPERTYVALUE,
    codepage: u16,
    pvar: *mut PROPVARIANT,
    pma: *const PMemoryAllocator,
) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgConvertPropertyToVariant(
                pprop: *const SERIALIZEDPROPERTYVALUE,
                codepage: u16,
                pvar: *mut ::std::mem::ManuallyDrop<PROPVARIANT>,
                pma: *const PMemoryAllocator,
            ) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(StgConvertPropertyToVariant(
            ::std::mem::transmute(pprop),
            ::std::mem::transmute(codepage),
            ::std::mem::transmute(pvar),
            ::std::mem::transmute(pma),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn StgConvertVariantToProperty<
    'a,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
>(
    pvar: *const PROPVARIANT,
    codepage: u16,
    pprop: *mut SERIALIZEDPROPERTYVALUE,
    pcb: *mut u32,
    pid: u32,
    freserved: Param5,
    pcindirect: *mut u32,
) -> *mut SERIALIZEDPROPERTYVALUE {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgConvertVariantToProperty(
                pvar: *const ::std::mem::ManuallyDrop<PROPVARIANT>,
                codepage: u16,
                pprop: *mut SERIALIZEDPROPERTYVALUE,
                pcb: *mut u32,
                pid: u32,
                freserved: super::super::Foundation::BOOLEAN,
                pcindirect: *mut u32,
            ) -> *mut SERIALIZEDPROPERTYVALUE;
        }
        ::std::mem::transmute(StgConvertVariantToProperty(
            ::std::mem::transmute(pvar),
            ::std::mem::transmute(codepage),
            ::std::mem::transmute(pprop),
            ::std::mem::transmute(pcb),
            ::std::mem::transmute(pid),
            freserved.into_param().abi(),
            ::std::mem::transmute(pcindirect),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn StgCreateDocfile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pwcsname: Param0,
    grfmode: u32,
    reserved: u32,
) -> ::windows::runtime::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgCreateDocfile(
                pwcsname: super::super::Foundation::PWSTR,
                grfmode: u32,
                reserved: u32,
                ppstgopen: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgCreateDocfile(
            pwcsname.into_param().abi(),
            ::std::mem::transmute(grfmode),
            ::std::mem::transmute(reserved),
            &mut result__,
        )
        .from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn StgCreateDocfileOnILockBytes<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ILockBytes>,
>(
    plkbyt: Param0,
    grfmode: u32,
    reserved: u32,
) -> ::windows::runtime::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgCreateDocfileOnILockBytes(
                plkbyt: ::windows::runtime::RawPtr,
                grfmode: u32,
                reserved: u32,
                ppstgopen: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgCreateDocfileOnILockBytes(
            plkbyt.into_param().abi(),
            ::std::mem::transmute(grfmode),
            ::std::mem::transmute(reserved),
            &mut result__,
        )
        .from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn StgCreatePropSetStg<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(
    pstorage: Param0,
    dwreserved: u32,
) -> ::windows::runtime::Result<IPropertySetStorage> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgCreatePropSetStg(
                pstorage: ::windows::runtime::RawPtr,
                dwreserved: u32,
                pppropsetstg: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IPropertySetStorage as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        StgCreatePropSetStg(
            pstorage.into_param().abi(),
            ::std::mem::transmute(dwreserved),
            &mut result__,
        )
        .from_abi::<IPropertySetStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn StgCreatePropStg<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
>(
    punk: Param0,
    fmtid: *const ::windows::runtime::GUID,
    pclsid: *const ::windows::runtime::GUID,
    grfflags: u32,
    dwreserved: u32,
) -> ::windows::runtime::Result<IPropertyStorage> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgCreatePropStg(
                punk: ::windows::runtime::RawPtr,
                fmtid: *const ::windows::runtime::GUID,
                pclsid: *const ::windows::runtime::GUID,
                grfflags: u32,
                dwreserved: u32,
                pppropstg: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IPropertyStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgCreatePropStg(
            punk.into_param().abi(),
            ::std::mem::transmute(fmtid),
            ::std::mem::transmute(pclsid),
            ::std::mem::transmute(grfflags),
            ::std::mem::transmute(dwreserved),
            &mut result__,
        )
        .from_abi::<IPropertyStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn StgCreateStorageEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pwcsname: Param0,
    grfmode: u32,
    stgfmt: u32,
    grfattrs: u32,
    pstgoptions: *mut STGOPTIONS,
    psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
    riid: *const ::windows::runtime::GUID,
    ppobjectopen: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgCreateStorageEx(
                pwcsname: super::super::Foundation::PWSTR,
                grfmode: u32,
                stgfmt: u32,
                grfattrs: u32,
                pstgoptions: *mut STGOPTIONS,
                psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
                riid: *const ::windows::runtime::GUID,
                ppobjectopen: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        StgCreateStorageEx(
            pwcsname.into_param().abi(),
            ::std::mem::transmute(grfmode),
            ::std::mem::transmute(stgfmt),
            ::std::mem::transmute(grfattrs),
            ::std::mem::transmute(pstgoptions),
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppobjectopen),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn StgDeserializePropVariant(
    pprop: *const SERIALIZEDPROPERTYVALUE,
    cbmax: u32,
) -> ::windows::runtime::Result<PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "propsys")]
        extern "system" {
            fn StgDeserializePropVariant(
                pprop: *const SERIALIZEDPROPERTYVALUE,
                cbmax: u32,
                ppropvar: *mut ::std::mem::ManuallyDrop<PROPVARIANT>,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgDeserializePropVariant(
            ::std::mem::transmute(pprop),
            ::std::mem::transmute(cbmax),
            &mut result__,
        )
        .from_abi::<PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn StgGetIFillLockBytesOnFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pwcsname: Param0,
) -> ::windows::runtime::Result<IFillLockBytes> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgGetIFillLockBytesOnFile(
                pwcsname: super::super::Foundation::PWSTR,
                ppflb: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IFillLockBytes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgGetIFillLockBytesOnFile(pwcsname.into_param().abi(), &mut result__)
            .from_abi::<IFillLockBytes>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn StgGetIFillLockBytesOnILockBytes<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ILockBytes>,
>(
    pilb: Param0,
) -> ::windows::runtime::Result<IFillLockBytes> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgGetIFillLockBytesOnILockBytes(
                pilb: ::windows::runtime::RawPtr,
                ppflb: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IFillLockBytes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgGetIFillLockBytesOnILockBytes(pilb.into_param().abi(), &mut result__)
            .from_abi::<IFillLockBytes>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn StgIsStorageFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pwcsname: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgIsStorageFile(
                pwcsname: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        StgIsStorageFile(pwcsname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn StgIsStorageILockBytes<'a, Param0: ::windows::runtime::IntoParam<'a, ILockBytes>>(
    plkbyt: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgIsStorageILockBytes(
                plkbyt: ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        StgIsStorageILockBytes(plkbyt.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn StgOpenAsyncDocfileOnIFillLockBytes<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, IFillLockBytes>,
>(
    pflb: Param0,
    grfmode: u32,
    asyncflags: u32,
) -> ::windows::runtime::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgOpenAsyncDocfileOnIFillLockBytes(
                pflb: ::windows::runtime::RawPtr,
                grfmode: u32,
                asyncflags: u32,
                ppstgopen: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgOpenAsyncDocfileOnIFillLockBytes(
            pflb.into_param().abi(),
            ::std::mem::transmute(grfmode),
            ::std::mem::transmute(asyncflags),
            &mut result__,
        )
        .from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn StgOpenLayoutDocfile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pwcsdfname: Param0,
    grfmode: u32,
    reserved: u32,
) -> ::windows::runtime::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "dflayout")]
        extern "system" {
            fn StgOpenLayoutDocfile(
                pwcsdfname: super::super::Foundation::PWSTR,
                grfmode: u32,
                reserved: u32,
                ppstgopen: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgOpenLayoutDocfile(
            pwcsdfname.into_param().abi(),
            ::std::mem::transmute(grfmode),
            ::std::mem::transmute(reserved),
            &mut result__,
        )
        .from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn StgOpenPropStg<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
>(
    punk: Param0,
    fmtid: *const ::windows::runtime::GUID,
    grfflags: u32,
    dwreserved: u32,
) -> ::windows::runtime::Result<IPropertyStorage> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgOpenPropStg(
                punk: ::windows::runtime::RawPtr,
                fmtid: *const ::windows::runtime::GUID,
                grfflags: u32,
                dwreserved: u32,
                pppropstg: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IPropertyStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgOpenPropStg(
            punk.into_param().abi(),
            ::std::mem::transmute(fmtid),
            ::std::mem::transmute(grfflags),
            ::std::mem::transmute(dwreserved),
            &mut result__,
        )
        .from_abi::<IPropertyStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn StgOpenStorage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, IStorage>,
>(
    pwcsname: Param0,
    pstgpriority: Param1,
    grfmode: u32,
    snbexclude: *const *const u16,
    reserved: u32,
) -> ::windows::runtime::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgOpenStorage(
                pwcsname: super::super::Foundation::PWSTR,
                pstgpriority: ::windows::runtime::RawPtr,
                grfmode: u32,
                snbexclude: *const *const u16,
                reserved: u32,
                ppstgopen: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgOpenStorage(
            pwcsname.into_param().abi(),
            pstgpriority.into_param().abi(),
            ::std::mem::transmute(grfmode),
            ::std::mem::transmute(snbexclude),
            ::std::mem::transmute(reserved),
            &mut result__,
        )
        .from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn StgOpenStorageEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pwcsname: Param0,
    grfmode: u32,
    stgfmt: u32,
    grfattrs: u32,
    pstgoptions: *mut STGOPTIONS,
    psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
    riid: *const ::windows::runtime::GUID,
    ppobjectopen: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgOpenStorageEx(
                pwcsname: super::super::Foundation::PWSTR,
                grfmode: u32,
                stgfmt: u32,
                grfattrs: u32,
                pstgoptions: *mut STGOPTIONS,
                psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
                riid: *const ::windows::runtime::GUID,
                ppobjectopen: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        StgOpenStorageEx(
            pwcsname.into_param().abi(),
            ::std::mem::transmute(grfmode),
            ::std::mem::transmute(stgfmt),
            ::std::mem::transmute(grfattrs),
            ::std::mem::transmute(pstgoptions),
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(ppobjectopen),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn StgOpenStorageOnILockBytes<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ILockBytes>,
    Param1: ::windows::runtime::IntoParam<'a, IStorage>,
>(
    plkbyt: Param0,
    pstgpriority: Param1,
    grfmode: u32,
    snbexclude: *const *const u16,
    reserved: u32,
) -> ::windows::runtime::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgOpenStorageOnILockBytes(
                plkbyt: ::windows::runtime::RawPtr,
                pstgpriority: ::windows::runtime::RawPtr,
                grfmode: u32,
                snbexclude: *const *const u16,
                reserved: u32,
                ppstgopen: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StgOpenStorageOnILockBytes(
            plkbyt.into_param().abi(),
            pstgpriority.into_param().abi(),
            ::std::mem::transmute(grfmode),
            ::std::mem::transmute(snbexclude),
            ::std::mem::transmute(reserved),
            &mut result__,
        )
        .from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn StgPropertyLengthAsVariant(
    pprop: *const SERIALIZEDPROPERTYVALUE,
    cbprop: u32,
    codepage: u16,
    breserved: u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgPropertyLengthAsVariant(
                pprop: *const SERIALIZEDPROPERTYVALUE,
                cbprop: u32,
                codepage: u16,
                breserved: u8,
            ) -> u32;
        }
        ::std::mem::transmute(StgPropertyLengthAsVariant(
            ::std::mem::transmute(pprop),
            ::std::mem::transmute(cbprop),
            ::std::mem::transmute(codepage),
            ::std::mem::transmute(breserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_Com",
    feature = "Win32_System_OleAutomation",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn StgSerializePropVariant(
    ppropvar: *const PROPVARIANT,
    ppprop: *mut *mut SERIALIZEDPROPERTYVALUE,
    pcb: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "propsys")]
        extern "system" {
            fn StgSerializePropVariant(
                ppropvar: *const ::std::mem::ManuallyDrop<PROPVARIANT>,
                ppprop: *mut *mut SERIALIZEDPROPERTYVALUE,
                pcb: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        StgSerializePropVariant(
            ::std::mem::transmute(ppropvar),
            ::std::mem::transmute(ppprop),
            ::std::mem::transmute(pcb),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn StgSetTimes<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszname: Param0,
    pctime: *const super::super::Foundation::FILETIME,
    patime: *const super::super::Foundation::FILETIME,
    pmtime: *const super::super::Foundation::FILETIME,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn StgSetTimes(
                lpszname: super::super::Foundation::PWSTR,
                pctime: *const super::super::Foundation::FILETIME,
                patime: *const super::super::Foundation::FILETIME,
                pmtime: *const super::super::Foundation::FILETIME,
            ) -> ::windows::runtime::HRESULT;
        }
        StgSetTimes(
            lpszname.into_param().abi(),
            ::std::mem::transmute(pctime),
            ::std::mem::transmute(patime),
            ::std::mem::transmute(pmtime),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: super::super::Foundation::PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl StorageLayout {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for StorageLayout {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for StorageLayout {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("StorageLayout")
            .field("LayoutType", &self.LayoutType)
            .field("pwcsElementName", &self.pwcsElementName)
            .field("cOffset", &self.cOffset)
            .field("cBytes", &self.cBytes)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for StorageLayout {
    fn eq(&self, other: &Self) -> bool {
        self.LayoutType == other.LayoutType
            && self.pwcsElementName == other.pwcsElementName
            && self.cOffset == other.cOffset
            && self.cBytes == other.cBytes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for StorageLayout {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for StorageLayout {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: ::windows::runtime::GUID,
    pub pStream: ::std::option::Option<IStream>,
}
impl VERSIONEDSTREAM {}
impl ::std::default::Default for VERSIONEDSTREAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VERSIONEDSTREAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VERSIONEDSTREAM")
            .field("guidVersion", &self.guidVersion)
            .field("pStream", &self.pStream)
            .finish()
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
    type DefaultType = Self;
}
pub unsafe fn WriteClassStg<'a, Param0: ::windows::runtime::IntoParam<'a, IStorage>>(
    pstg: Param0,
    rclsid: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn WriteClassStg(
                pstg: ::windows::runtime::RawPtr,
                rclsid: *const ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        WriteClassStg(pstg.into_param().abi(), ::std::mem::transmute(rclsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WriteClassStm<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>>(
    pstm: Param0,
    rclsid: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn WriteClassStm(
                pstm: ::windows::runtime::RawPtr,
                rclsid: *const ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        WriteClassStm(pstm.into_param().abi(), ::std::mem::transmute(rclsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WriteFmtUserTypeStg<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, IStorage>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pstg: Param0,
    cf: u16,
    lpszusertype: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ole32")]
        extern "system" {
            fn WriteFmtUserTypeStg(
                pstg: ::windows::runtime::RawPtr,
                cf: u16,
                lpszusertype: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        WriteFmtUserTypeStg(
            pstg.into_param().abi(),
            ::std::mem::transmute(cf),
            lpszusertype.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}

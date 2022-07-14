#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct BSTRBLOB {
    pub cbSize: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for BSTRBLOB {}
impl ::core::clone::Clone for BSTRBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BSTRBLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BSTRBLOB").field("cbSize", &self.cbSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for BSTRBLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BSTRBLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BSTRBLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for BSTRBLOB {}
impl ::core::default::Default for BSTRBLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl ::core::marker::Copy for CABOOL {}
impl ::core::clone::Clone for CABOOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CABOOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABOOL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CABOOL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CABOOL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CABOOL>()) == 0 }
    }
}
impl ::core::cmp::Eq for CABOOL {}
impl ::core::default::Default for CABOOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CABSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CABSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CABSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABSTR").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CABSTR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CABSTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CABSTR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CABSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CABSTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut BSTRBLOB,
}
impl ::core::marker::Copy for CABSTRBLOB {}
impl ::core::clone::Clone for CABSTRBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CABSTRBLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CABSTRBLOB").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CABSTRBLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CABSTRBLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CABSTRBLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for CABSTRBLOB {}
impl ::core::default::Default for CABSTRBLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAC {
    pub cElems: u32,
    pub pElems: ::windows::core::PSTR,
}
impl ::core::marker::Copy for CAC {}
impl ::core::clone::Clone for CAC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAC").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CAC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAC>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAC {}
impl ::core::default::Default for CAC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut CLIPDATA,
}
impl ::core::marker::Copy for CACLIPDATA {}
impl ::core::clone::Clone for CACLIPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CACLIPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CACLIPDATA").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CACLIPDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CACLIPDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CACLIPDATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for CACLIPDATA {}
impl ::core::default::Default for CACLIPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for CACLSID {}
impl ::core::clone::Clone for CACLSID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CACLSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CACLSID").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CACLSID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CACLSID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CACLSID>()) == 0 }
    }
}
impl ::core::cmp::Eq for CACLSID {}
impl ::core::default::Default for CACLSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut super::CY,
}
impl ::core::marker::Copy for CACY {}
impl ::core::clone::Clone for CACY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CACY").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CACY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CACY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CACY>()) == 0 }
    }
}
impl ::core::cmp::Eq for CACY {}
impl ::core::default::Default for CACY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl ::core::marker::Copy for CADATE {}
impl ::core::clone::Clone for CADATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CADATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CADATE").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CADATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CADATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CADATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CADATE {}
impl ::core::default::Default for CADATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
impl ::core::marker::Copy for CADBL {}
impl ::core::clone::Clone for CADBL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CADBL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CADBL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CADBL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CADBL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CADBL>()) == 0 }
    }
}
impl ::core::cmp::Eq for CADBL {}
impl ::core::default::Default for CADBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAFILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAFILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CAFILETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAFILETIME").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CAFILETIME {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CAFILETIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAFILETIME>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CAFILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CAFILETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
impl ::core::marker::Copy for CAFLT {}
impl ::core::clone::Clone for CAFLT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAFLT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAFLT").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CAFLT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAFLT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAFLT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAFLT {}
impl ::core::default::Default for CAFLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
impl ::core::marker::Copy for CAH {}
impl ::core::clone::Clone for CAH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAH").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CAH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAH>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAH {}
impl ::core::default::Default for CAH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
impl ::core::marker::Copy for CAI {}
impl ::core::clone::Clone for CAI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAI").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CAI {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAI>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAI {}
impl ::core::default::Default for CAI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl ::core::marker::Copy for CAL {}
impl ::core::clone::Clone for CAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAL {}
impl ::core::default::Default for CAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut ::windows::core::PSTR,
}
impl ::core::marker::Copy for CALPSTR {}
impl ::core::clone::Clone for CALPSTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CALPSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CALPSTR").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CALPSTR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CALPSTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CALPSTR>()) == 0 }
    }
}
impl ::core::cmp::Eq for CALPSTR {}
impl ::core::default::Default for CALPSTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut ::windows::core::PWSTR,
}
impl ::core::marker::Copy for CALPWSTR {}
impl ::core::clone::Clone for CALPWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CALPWSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CALPWSTR").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CALPWSTR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CALPWSTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CALPWSTR>()) == 0 }
    }
}
impl ::core::cmp::Eq for CALPWSTR {}
impl ::core::default::Default for CALPWSTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAPROPVARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAPROPVARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CAPROPVARIANT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAPROPVARIANT").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CAPROPVARIANT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CAPROPVARIANT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAPROPVARIANT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CAPROPVARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CAPROPVARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut i32,
}
impl ::core::marker::Copy for CASCODE {}
impl ::core::clone::Clone for CASCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CASCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CASCODE").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CASCODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CASCODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CASCODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CASCODE {}
impl ::core::default::Default for CASCODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
impl ::core::marker::Copy for CAUB {}
impl ::core::clone::Clone for CAUB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAUB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAUB").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CAUB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAUB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAUB>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAUB {}
impl ::core::default::Default for CAUB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
impl ::core::marker::Copy for CAUH {}
impl ::core::clone::Clone for CAUH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAUH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAUH").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CAUH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAUH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAUH>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAUH {}
impl ::core::default::Default for CAUH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
impl ::core::marker::Copy for CAUI {}
impl ::core::clone::Clone for CAUI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAUI").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CAUI {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAUI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAUI>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAUI {}
impl ::core::default::Default for CAUI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl ::core::marker::Copy for CAUL {}
impl ::core::clone::Clone for CAUL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAUL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAUL").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CAUL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAUL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAUL>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAUL {}
impl ::core::default::Default for CAUL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const CCH_MAX_PROPSTG_NAME: u32 = 31u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct CLIPDATA {
    pub cbSize: u32,
    pub ulClipFmt: i32,
    pub pClipData: *mut u8,
}
impl ::core::marker::Copy for CLIPDATA {}
impl ::core::clone::Clone for CLIPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CLIPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIPDATA").field("cbSize", &self.cbSize).field("ulClipFmt", &self.ulClipFmt).field("pClipData", &self.pClipData).finish()
    }
}
unsafe impl ::windows::core::Abi for CLIPDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CLIPDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CLIPDATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for CLIPDATA {}
impl ::core::default::Default for CLIPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const CWCSTORAGENAME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn CoGetInstanceFromFile<'a, P0, P1>(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::core::GUID, punkouter: P0, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: P1, presults: &mut [super::MULTI_QI]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CoGetInstanceFromFile(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: ::windows::core::PCWSTR, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows::core::HRESULT;
    }
    CoGetInstanceFromFile(::core::mem::transmute(pserverinfo), ::core::mem::transmute(pclsid), punkouter.into().abi(), dwclsctx, grfmode, pwszname.into(), presults.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(presults))).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn CoGetInstanceFromIStorage<'a, P0, P1>(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::core::GUID, punkouter: P0, dwclsctx: super::CLSCTX, pstg: P1, presults: &mut [super::MULTI_QI]) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CoGetInstanceFromIStorage(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: super::CLSCTX, pstg: *mut ::core::ffi::c_void, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows::core::HRESULT;
    }
    CoGetInstanceFromIStorage(::core::mem::transmute(pserverinfo), ::core::mem::transmute(pclsid), punkouter.into().abi(), dwclsctx, pstg.into().abi(), presults.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(presults))).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn CoGetInterfaceAndReleaseStream<'a, P0, T>(pstm: P0) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::IStream>>,
    T: ::windows::core::Interface,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CoGetInterfaceAndReleaseStream(pstm: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::option::Option::None;
    CoGetInterfaceAndReleaseStream(pstm.into().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateILockBytesOnHGlobal<'a, P0>(hglobal: isize, fdeleteonrelease: P0) -> ::windows::core::Result<ILockBytes>
where
    P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateILockBytesOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, pplkbyt: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateILockBytesOnHGlobal(hglobal, fdeleteonrelease.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ILockBytes>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStreamOnHGlobal<'a, P0>(hglobal: isize, fdeleteonrelease: P0) -> ::windows::core::Result<super::IStream>
where
    P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateStreamOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, ppstm: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateStreamOnHGlobal(hglobal, fdeleteonrelease.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IStream>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn FmtIdToPropStgName(pfmtid: *const ::windows::core::GUID, oszname: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FmtIdToPropStgName(pfmtid: *const ::windows::core::GUID, oszname: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    FmtIdToPropStgName(::core::mem::transmute(pfmtid), ::core::mem::transmute(oszname)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreePropVariantArray(rgvars: &mut [PROPVARIANT]) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FreePropVariantArray(cvariants: u32, rgvars: *mut PROPVARIANT) -> ::windows::core::HRESULT;
    }
    FreePropVariantArray(rgvars.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgvars))).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn GetConvertStg<'a, P0>(pstg: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConvertStg(pstg: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    GetConvertStg(pstg.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn GetHGlobalFromILockBytes<'a, P0>(plkbyt: P0) -> ::windows::core::Result<isize>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ILockBytes>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetHGlobalFromILockBytes(plkbyt: *mut ::core::ffi::c_void, phglobal: *mut isize) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetHGlobalFromILockBytes(plkbyt.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn GetHGlobalFromStream<'a, P0>(pstm: P0) -> ::windows::core::Result<isize>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::IStream>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetHGlobalFromStream(pstm: *mut ::core::ffi::c_void, phglobal: *mut isize) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetHGlobalFromStream(pstm.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<isize>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IDirectWriterLock(::windows::core::IUnknown);
impl IDirectWriterLock {
    pub unsafe fn WaitForWriteAccess(&self, dwtimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForWriteAccess)(::windows::core::Interface::as_raw(self), dwtimeout).ok()
    }
    pub unsafe fn ReleaseWriteAccess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseWriteAccess)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn HaveWriteAccess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HaveWriteAccess)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDirectWriterLock> for ::windows::core::IUnknown {
    fn from(value: IDirectWriterLock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectWriterLock> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectWriterLock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectWriterLock> for ::windows::core::IUnknown {
    fn from(value: &IDirectWriterLock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDirectWriterLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectWriterLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectWriterLock {}
impl ::core::fmt::Debug for IDirectWriterLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectWriterLock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectWriterLock {
    type Vtable = IDirectWriterLock_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e6d4d92_6738_11cf_9608_00aa00680db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectWriterLock_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub WaitForWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT,
    pub ReleaseWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HaveWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IEnumSTATPROPSETSTG(::windows::core::IUnknown);
impl IEnumSTATPROPSETSTG {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, rgelt: &mut [STATPROPSETSTG], pceltfetched: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATPROPSETSTG> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSTATPROPSETSTG>(result__)
    }
}
impl ::core::convert::From<IEnumSTATPROPSETSTG> for ::windows::core::IUnknown {
    fn from(value: IEnumSTATPROPSETSTG) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumSTATPROPSETSTG> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumSTATPROPSETSTG) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSTATPROPSETSTG> for ::windows::core::IUnknown {
    fn from(value: &IEnumSTATPROPSETSTG) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumSTATPROPSETSTG {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSTATPROPSETSTG {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSTATPROPSETSTG {}
impl ::core::fmt::Debug for IEnumSTATPROPSETSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSTATPROPSETSTG").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSTATPROPSETSTG {
    type Vtable = IEnumSTATPROPSETSTG_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATPROPSETSTG_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IEnumSTATPROPSTG(::windows::core::IUnknown);
impl IEnumSTATPROPSTG {
    pub unsafe fn Next(&self, rgelt: &mut [STATPROPSTG], pceltfetched: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATPROPSTG> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSTATPROPSTG>(result__)
    }
}
impl ::core::convert::From<IEnumSTATPROPSTG> for ::windows::core::IUnknown {
    fn from(value: IEnumSTATPROPSTG) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumSTATPROPSTG> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumSTATPROPSTG) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSTATPROPSTG> for ::windows::core::IUnknown {
    fn from(value: &IEnumSTATPROPSTG) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumSTATPROPSTG {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSTATPROPSTG {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSTATPROPSTG {}
impl ::core::fmt::Debug for IEnumSTATPROPSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSTATPROPSTG").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSTATPROPSTG {
    type Vtable = IEnumSTATPROPSTG_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000139_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATPROPSTG_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IEnumSTATSTG(::windows::core::IUnknown);
impl IEnumSTATSTG {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, rgelt: &mut [super::STATSTG], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATSTG> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSTATSTG>(result__)
    }
}
impl ::core::convert::From<IEnumSTATSTG> for ::windows::core::IUnknown {
    fn from(value: IEnumSTATSTG) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumSTATSTG> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumSTATSTG) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSTATSTG> for ::windows::core::IUnknown {
    fn from(value: &IEnumSTATSTG) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumSTATSTG {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSTATSTG {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSTATSTG {}
impl ::core::fmt::Debug for IEnumSTATSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSTATSTG").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSTATSTG {
    type Vtable = IEnumSTATSTG_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATSTG_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IFillLockBytes(::windows::core::IUnknown);
impl IFillLockBytes {
    pub unsafe fn FillAppend(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FillAppend)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pv), cb, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn FillAt(&self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FillAt)(::windows::core::Interface::as_raw(self), uloffset, ::core::mem::transmute(pv), cb, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetFillSize(&self, ulsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFillSize)(::windows::core::Interface::as_raw(self), ulsize).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Terminate<'a, P0>(&self, bcanceled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Terminate)(::windows::core::Interface::as_raw(self), bcanceled.into()).ok()
    }
}
impl ::core::convert::From<IFillLockBytes> for ::windows::core::IUnknown {
    fn from(value: IFillLockBytes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFillLockBytes> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFillLockBytes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFillLockBytes> for ::windows::core::IUnknown {
    fn from(value: &IFillLockBytes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IFillLockBytes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFillLockBytes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFillLockBytes {}
impl ::core::fmt::Debug for IFillLockBytes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFillLockBytes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFillLockBytes {
    type Vtable = IFillLockBytes_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99caf010_415e_11cf_8814_00aa00b569f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFillLockBytes_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub FillAppend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub FillAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub SetFillSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsize: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bcanceled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Terminate: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct ILayoutStorage(::windows::core::IUnknown);
impl ILayoutStorage {
    pub unsafe fn LayoutScript(&self, pstoragelayout: &[super::StorageLayout], glfinterleavedflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LayoutScript)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pstoragelayout)), pstoragelayout.len() as _, glfinterleavedflag).ok()
    }
    pub unsafe fn BeginMonitor(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginMonitor)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndMonitor(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndMonitor)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReLayoutDocfile<'a, P0>(&self, pwcsnewdfname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ReLayoutDocfile)(::windows::core::Interface::as_raw(self), pwcsnewdfname.into()).ok()
    }
    pub unsafe fn ReLayoutDocfileOnILockBytes<'a, P0>(&self, pilockbytes: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ILockBytes>>,
    {
        (::windows::core::Interface::vtable(self).ReLayoutDocfileOnILockBytes)(::windows::core::Interface::as_raw(self), pilockbytes.into().abi()).ok()
    }
}
impl ::core::convert::From<ILayoutStorage> for ::windows::core::IUnknown {
    fn from(value: ILayoutStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILayoutStorage> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILayoutStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILayoutStorage> for ::windows::core::IUnknown {
    fn from(value: &ILayoutStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ILayoutStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILayoutStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILayoutStorage {}
impl ::core::fmt::Debug for ILayoutStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILayoutStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILayoutStorage {
    type Vtable = ILayoutStorage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e6d4d90_6738_11cf_9608_00aa00680db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutStorage_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub LayoutScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows::core::HRESULT,
    pub BeginMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReLayoutDocfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsnewdfname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub ReLayoutDocfileOnILockBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pilockbytes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct ILockBytes(::windows::core::IUnknown);
impl ILockBytes {
    pub unsafe fn ReadAt(&self, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadAt)(::windows::core::Interface::as_raw(self), uloffset, ::core::mem::transmute(pv), cb, ::core::mem::transmute(pcbread)).ok()
    }
    pub unsafe fn WriteAt(&self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).WriteAt)(::windows::core::Interface::as_raw(self), uloffset, ::core::mem::transmute(pv), cb, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetSize(&self, cb: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSize)(::windows::core::Interface::as_raw(self), cb).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstatstg), grfstatflag).ok()
    }
}
impl ::core::convert::From<ILockBytes> for ::windows::core::IUnknown {
    fn from(value: ILockBytes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILockBytes> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILockBytes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockBytes> for ::windows::core::IUnknown {
    fn from(value: &ILockBytes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ILockBytes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILockBytes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILockBytes {}
impl ::core::fmt::Debug for ILockBytes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILockBytes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILockBytes {
    type Vtable = ILockBytes_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockBytes_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ReadAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub WriteAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cb: u64) -> ::windows::core::HRESULT,
    pub LockRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    pub UnlockRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Stat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Stat: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IPersistStorage(::windows::core::IUnknown);
impl IPersistStorage {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetClassID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).IsDirty)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn InitNew<'a, P0>(&self, pstg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
    {
        (::windows::core::Interface::vtable(self).InitNew)(::windows::core::Interface::as_raw(self), pstg.into().abi()).ok()
    }
    pub unsafe fn Load<'a, P0>(&self, pstg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
    {
        (::windows::core::Interface::vtable(self).Load)(::windows::core::Interface::as_raw(self), pstg.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, P0, P1>(&self, pstgsave: P0, fsameasload: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Save)(::windows::core::Interface::as_raw(self), pstgsave.into().abi(), fsameasload.into()).ok()
    }
    pub unsafe fn SaveCompleted<'a, P0>(&self, pstgnew: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
    {
        (::windows::core::Interface::vtable(self).SaveCompleted)(::windows::core::Interface::as_raw(self), pstgnew.into().abi()).ok()
    }
    pub unsafe fn HandsOffStorage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HandsOffStorage)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPersistStorage> for ::windows::core::IUnknown {
    fn from(value: IPersistStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPersistStorage> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPersistStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStorage> for ::windows::core::IUnknown {
    fn from(value: &IPersistStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IPersistStorage> for super::IPersist {
    fn from(value: IPersistStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPersistStorage> for &'a super::IPersist {
    fn from(value: &'a IPersistStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStorage> for super::IPersist {
    fn from(value: &IPersistStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPersistStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistStorage {}
impl ::core::fmt::Debug for IPersistStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistStorage {
    type Vtable = IPersistStorage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStorage_Vtbl {
    pub base__: super::IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstgsave: *mut ::core::ffi::c_void, fsameasload: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub SaveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstgnew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HandsOffStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IPropertyBag(::windows::core::IUnknown);
impl IPropertyBag {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<'a, P0, P1>(&self, pszpropname: P0, pvar: *mut super::VARIANT, perrorlog: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::IErrorLog>>,
    {
        (::windows::core::Interface::vtable(self).Read)(::windows::core::Interface::as_raw(self), pszpropname.into(), ::core::mem::transmute(pvar), perrorlog.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<'a, P0>(&self, pszpropname: P0, pvar: *const super::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Write)(::windows::core::Interface::as_raw(self), pszpropname.into(), ::core::mem::transmute(pvar)).ok()
    }
}
impl ::core::convert::From<IPropertyBag> for ::windows::core::IUnknown {
    fn from(value: IPropertyBag) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPropertyBag> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPropertyBag) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyBag> for ::windows::core::IUnknown {
    fn from(value: &IPropertyBag) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPropertyBag {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyBag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyBag {}
impl ::core::fmt::Debug for IPropertyBag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyBag").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyBag {
    type Vtable = IPropertyBag_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55272a00_42cb_11ce_8135_00aa004bb851);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBag_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropname: ::windows::core::PCWSTR, pvar: *mut super::VARIANT, perrorlog: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Read: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropname: ::windows::core::PCWSTR, pvar: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Write: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IPropertyBag2(::windows::core::IUnknown);
impl IPropertyBag2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<'a, P0>(&self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: P0, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IErrorLog>>,
    {
        (::windows::core::Interface::vtable(self).Read)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(cproperties), ::core::mem::transmute(ppropbag), perrlog.into().abi(), ::core::mem::transmute(pvarvalue), ::core::mem::transmute(phrerror)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Write(&self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Write)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(cproperties), ::core::mem::transmute(ppropbag), ::core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn CountProperties(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CountProperties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPropertyInfo(&self, iproperty: u32, ppropbag: &mut [PROPBAG2], pcproperties: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyInfo)(::windows::core::Interface::as_raw(self), iproperty, ppropbag.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppropbag)), ::core::mem::transmute(pcproperties)).ok()
    }
    pub unsafe fn LoadObject<'a, P0, P1, P2>(&self, pstrname: P0, dwhint: u32, punkobject: P1, perrlog: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::IErrorLog>>,
    {
        (::windows::core::Interface::vtable(self).LoadObject)(::windows::core::Interface::as_raw(self), pstrname.into(), dwhint, punkobject.into().abi(), perrlog.into().abi()).ok()
    }
}
impl ::core::convert::From<IPropertyBag2> for ::windows::core::IUnknown {
    fn from(value: IPropertyBag2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPropertyBag2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPropertyBag2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyBag2> for ::windows::core::IUnknown {
    fn from(value: &IPropertyBag2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPropertyBag2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyBag2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyBag2 {}
impl ::core::fmt::Debug for IPropertyBag2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyBag2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyBag2 {
    type Vtable = IPropertyBag2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22f55882_280b_11d0_a8a9_00a0c90c2004);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBag2_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: *mut ::core::ffi::c_void, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Read: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Write: usize,
    pub CountProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcproperties: *mut u32) -> ::windows::core::HRESULT,
    pub GetPropertyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows::core::HRESULT,
    pub LoadObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrname: ::windows::core::PCWSTR, dwhint: u32, punkobject: *mut ::core::ffi::c_void, perrlog: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IPropertySetStorage(::windows::core::IUnknown);
impl IPropertySetStorage {
    pub unsafe fn Create(&self, rfmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32) -> ::windows::core::Result<IPropertyStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rfmtid), ::core::mem::transmute(pclsid), grfflags, grfmode, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPropertyStorage>(result__)
    }
    pub unsafe fn Open(&self, rfmtid: *const ::windows::core::GUID, grfmode: u32) -> ::windows::core::Result<IPropertyStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rfmtid), grfmode, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPropertyStorage>(result__)
    }
    pub unsafe fn Delete(&self, rfmtid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rfmtid)).ok()
    }
    pub unsafe fn Enum(&self) -> ::windows::core::Result<IEnumSTATPROPSETSTG> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Enum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSTATPROPSETSTG>(result__)
    }
}
impl ::core::convert::From<IPropertySetStorage> for ::windows::core::IUnknown {
    fn from(value: IPropertySetStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPropertySetStorage> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPropertySetStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertySetStorage> for ::windows::core::IUnknown {
    fn from(value: &IPropertySetStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPropertySetStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertySetStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySetStorage {}
impl ::core::fmt::Debug for IPropertySetStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySetStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertySetStorage {
    type Vtable = IPropertySetStorage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetStorage_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, grfmode: u32, ppprstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Enum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IPropertyStorage(::windows::core::IUnknown);
impl IPropertyStorage {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadMultiple)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(cpspec), ::core::mem::transmute(rgpspec), ::core::mem::transmute(rgpropvar)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteMultiple)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(cpspec), ::core::mem::transmute(rgpspec), ::core::mem::transmute(rgpropvar), propidnamefirst).ok()
    }
    pub unsafe fn DeleteMultiple(&self, rgpspec: &[PROPSPEC]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteMultiple)(::windows::core::Interface::as_raw(self), rgpspec.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgpspec))).ok()
    }
    pub unsafe fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadPropertyNames)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(cpropid), ::core::mem::transmute(rgpropid), ::core::mem::transmute(rglpwstrname)).ok()
    }
    pub unsafe fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WritePropertyNames)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(cpropid), ::core::mem::transmute(rgpropid), ::core::mem::transmute(rglpwstrname)).ok()
    }
    pub unsafe fn DeletePropertyNames(&self, rgpropid: &[u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeletePropertyNames)(::windows::core::Interface::as_raw(self), rgpropid.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgpropid))).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Enum(&self) -> ::windows::core::Result<IEnumSTATPROPSTG> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Enum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSTATPROPSTG>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTimes(&self, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTimes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pctime), ::core::mem::transmute(patime), ::core::mem::transmute(pmtime)).ok()
    }
    pub unsafe fn SetClass(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClass)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(clsid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self) -> ::windows::core::Result<STATPROPSETSTG> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Stat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<STATPROPSETSTG>(result__)
    }
}
impl ::core::convert::From<IPropertyStorage> for ::windows::core::IUnknown {
    fn from(value: IPropertyStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPropertyStorage> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPropertyStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyStorage> for ::windows::core::IUnknown {
    fn from(value: &IPropertyStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPropertyStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStorage {}
impl ::core::fmt::Debug for IPropertyStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertyStorage {
    type Vtable = IPropertyStorage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000138_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStorage_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReadMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReadMultiple: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteMultiple: usize,
    pub DeleteMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows::core::HRESULT,
    pub ReadPropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub WritePropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub DeletePropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT,
    pub Revert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Enum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTimes: usize,
    pub SetClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Stat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatpsstg: *mut STATPROPSETSTG) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Stat: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IRootStorage(::windows::core::IUnknown);
impl IRootStorage {
    pub unsafe fn SwitchToFile<'a, P0>(&self, pszfile: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SwitchToFile)(::windows::core::Interface::as_raw(self), pszfile.into()).ok()
    }
}
impl ::core::convert::From<IRootStorage> for ::windows::core::IUnknown {
    fn from(value: IRootStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRootStorage> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRootStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRootStorage> for ::windows::core::IUnknown {
    fn from(value: &IRootStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRootStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRootStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRootStorage {}
impl ::core::fmt::Debug for IRootStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRootStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRootStorage {
    type Vtable = IRootStorage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000012_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRootStorage_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SwitchToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
pub struct IStorage(::windows::core::IUnknown);
impl IStorage {
    pub unsafe fn CreateStream<'a, P0>(&self, pwcsname: P0, grfmode: STGM, reserved1: u32, reserved2: u32) -> ::windows::core::Result<super::IStream>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateStream)(::windows::core::Interface::as_raw(self), pwcsname.into(), grfmode, reserved1, reserved2, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IStream>(result__)
    }
    pub unsafe fn OpenStream<'a, P0>(&self, pwcsname: P0, reserved1: *mut ::core::ffi::c_void, grfmode: STGM, reserved2: u32, ppstm: *mut ::core::option::Option<super::IStream>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OpenStream)(::windows::core::Interface::as_raw(self), pwcsname.into(), ::core::mem::transmute(reserved1), grfmode, reserved2, ::core::mem::transmute(ppstm)).ok()
    }
    pub unsafe fn CreateStorage<'a, P0>(&self, pwcsname: P0, grfmode: STGM, reserved1: u32, reserved2: u32) -> ::windows::core::Result<IStorage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateStorage)(::windows::core::Interface::as_raw(self), pwcsname.into(), grfmode, reserved1, reserved2, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStorage>(result__)
    }
    pub unsafe fn OpenStorage<'a, P0, P1>(&self, pwcsname: P0, pstgpriority: P1, grfmode: STGM, snbexclude: *const *const u16, reserved: u32) -> ::windows::core::Result<IStorage>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).OpenStorage)(::windows::core::Interface::as_raw(self), pwcsname.into(), pstgpriority.into().abi(), grfmode, ::core::mem::transmute(snbexclude), reserved, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStorage>(result__)
    }
    pub unsafe fn CopyTo<'a, P0>(&self, rgiidexclude: &[::windows::core::GUID], snbexclude: *const *const u16, pstgdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
    {
        (::windows::core::Interface::vtable(self).CopyTo)(::windows::core::Interface::as_raw(self), rgiidexclude.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgiidexclude)), ::core::mem::transmute(snbexclude), pstgdest.into().abi()).ok()
    }
    pub unsafe fn MoveElementTo<'a, P0, P1, P2>(&self, pwcsname: P0, pstgdest: P1, pwcsnewname: P2, grfflags: STGMOVE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).MoveElementTo)(::windows::core::Interface::as_raw(self), pwcsname.into(), pstgdest.into().abi(), pwcsnewname.into(), grfflags).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: super::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumElements(&self, reserved1: u32, reserved2: *mut ::core::ffi::c_void, reserved3: u32, ppenum: *mut ::core::option::Option<IEnumSTATSTG>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumElements)(::windows::core::Interface::as_raw(self), reserved1, ::core::mem::transmute(reserved2), reserved3, ::core::mem::transmute(ppenum)).ok()
    }
    pub unsafe fn DestroyElement<'a, P0>(&self, pwcsname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DestroyElement)(::windows::core::Interface::as_raw(self), pwcsname.into()).ok()
    }
    pub unsafe fn RenameElement<'a, P0, P1>(&self, pwcsoldname: P0, pwcsnewname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RenameElement)(::windows::core::Interface::as_raw(self), pwcsoldname.into(), pwcsnewname.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetElementTimes<'a, P0>(&self, pwcsname: P0, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetElementTimes)(::windows::core::Interface::as_raw(self), pwcsname.into(), ::core::mem::transmute(pctime), ::core::mem::transmute(patime), ::core::mem::transmute(pmtime)).ok()
    }
    pub unsafe fn SetClass(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClass)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(clsid)).ok()
    }
    pub unsafe fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStateBits)(::windows::core::Interface::as_raw(self), grfstatebits, grfmask).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstatstg), grfstatflag).ok()
    }
}
impl ::core::convert::From<IStorage> for ::windows::core::IUnknown {
    fn from(value: IStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IStorage> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorage> for ::windows::core::IUnknown {
    fn from(value: &IStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorage {}
impl ::core::fmt::Debug for IStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStorage {
    type Vtable = IStorage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorage_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, grfmode: STGM, reserved1: u32, reserved2: u32, ppstm: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, reserved1: *mut ::core::ffi::c_void, grfmode: STGM, reserved2: u32, ppstm: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, grfmode: STGM, reserved1: u32, reserved2: u32, ppstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, pstgpriority: *mut ::core::ffi::c_void, grfmode: STGM, snbexclude: *const *const u16, reserved: u32, ppstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ciidexclude: u32, rgiidexclude: *const ::windows::core::GUID, snbexclude: *const *const u16, pstgdest: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveElementTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, pstgdest: *mut ::core::ffi::c_void, pwcsnewname: ::windows::core::PCWSTR, grfflags: STGMOVE) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: super::STGC) -> ::windows::core::HRESULT,
    pub Revert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved1: u32, reserved2: *mut ::core::ffi::c_void, reserved3: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DestroyElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RenameElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsoldname: ::windows::core::PCWSTR, pwcsnewname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetElementTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetElementTimes: usize,
    pub SetClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetStateBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfstatebits: u32, grfmask: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Stat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Stat: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LOCKTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const LOCK_WRITE: LOCKTYPE = LOCKTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const LOCK_EXCLUSIVE: LOCKTYPE = LOCKTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const LOCK_ONLYONCE: LOCKTYPE = LOCKTYPE(4i32);
impl ::core::marker::Copy for LOCKTYPE {}
impl ::core::clone::Clone for LOCKTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOCKTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LOCKTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for LOCKTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCKTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct OLESTREAM {
    pub lpstbl: *mut OLESTREAMVTBL,
}
impl ::core::marker::Copy for OLESTREAM {}
impl ::core::clone::Clone for OLESTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLESTREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLESTREAM").field("lpstbl", &self.lpstbl).finish()
    }
}
unsafe impl ::windows::core::Abi for OLESTREAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OLESTREAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OLESTREAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for OLESTREAM {}
impl ::core::default::Default for OLESTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct OLESTREAMVTBL {
    pub Get: isize,
    pub Put: isize,
}
impl ::core::marker::Copy for OLESTREAMVTBL {}
impl ::core::clone::Clone for OLESTREAMVTBL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLESTREAMVTBL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLESTREAMVTBL").field("Get", &self.Get).field("Put", &self.Put).finish()
    }
}
unsafe impl ::windows::core::Abi for OLESTREAMVTBL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OLESTREAMVTBL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OLESTREAMVTBL>()) == 0 }
    }
}
impl ::core::cmp::Eq for OLESTREAMVTBL {}
impl ::core::default::Default for OLESTREAMVTBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAM<'a, P0>(pstg: P0, lpolestream: *mut OLESTREAM) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OleConvertIStorageToOLESTREAM(pstg: *mut ::core::ffi::c_void, lpolestream: *mut OLESTREAM) -> ::windows::core::HRESULT;
    }
    OleConvertIStorageToOLESTREAM(pstg.into().abi(), ::core::mem::transmute(lpolestream)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAMEx<'a, P0>(pstg: P0, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *mut super::STGMEDIUM, polestm: *mut OLESTREAM) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OleConvertIStorageToOLESTREAMEx(pstg: *mut ::core::ffi::c_void, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *mut super::STGMEDIUM, polestm: *mut OLESTREAM) -> ::windows::core::HRESULT;
    }
    OleConvertIStorageToOLESTREAMEx(pstg.into().abi(), cfformat, lwidth, lheight, dwsize, ::core::mem::transmute(pmedium), ::core::mem::transmute(polestm)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage<'a, P0>(lpolestream: *mut OLESTREAM, pstg: P0, ptd: *const super::DVTARGETDEVICE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OleConvertOLESTREAMToIStorage(lpolestream: *mut OLESTREAM, pstg: *mut ::core::ffi::c_void, ptd: *const super::DVTARGETDEVICE) -> ::windows::core::HRESULT;
    }
    OleConvertOLESTREAMToIStorage(::core::mem::transmute(lpolestream), pstg.into().abi(), ::core::mem::transmute(ptd)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx<'a, P0>(polestm: *mut OLESTREAM, pstg: P0, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::STGMEDIUM) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OleConvertOLESTREAMToIStorageEx(polestm: *mut OLESTREAM, pstg: *mut ::core::ffi::c_void, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::STGMEDIUM) -> ::windows::core::HRESULT;
    }
    OleConvertOLESTREAMToIStorageEx(::core::mem::transmute(polestm), pstg.into().abi(), ::core::mem::transmute(pcfformat), ::core::mem::transmute(plwwidth), ::core::mem::transmute(plheight), ::core::mem::transmute(pdwsize), ::core::mem::transmute(pmedium)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDI_THUMBNAIL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_BYTECOUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_CATEGORY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_COMPANY: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_DOCPARTS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_HEADINGPAIR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_HIDDENCOUNT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_LINECOUNT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_LINKSDIRTY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_MANAGER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_MMCLIPCOUNT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_NOTECOUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_PARCOUNT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_PRESFORMAT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_SCALE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDDSI_SLIDECOUNT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_COPYRIGHT: i32 = 11i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_EDITOR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_OWNER: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_PRODUCTION: i32 = 10i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_PROJECT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_RATING: i32 = 9i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_SEQUENCE_NO: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_SOURCE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS: i32 = 7i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PIDMSI_STATUS_VALUE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_NORMAL: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(0i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_NEW: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(1i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_PRELIM: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(2i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_DRAFT: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(3i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_INPROGRESS: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(4i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_EDIT: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(5i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_REVIEW: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(6i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_PROOF: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(7i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_FINAL: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(8i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_STATUS_OTHER: PIDMSI_STATUS_VALUE = PIDMSI_STATUS_VALUE(32767i32);
impl ::core::marker::Copy for PIDMSI_STATUS_VALUE {}
impl ::core::clone::Clone for PIDMSI_STATUS_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PIDMSI_STATUS_VALUE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PIDMSI_STATUS_VALUE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PIDMSI_STATUS_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PIDMSI_STATUS_VALUE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDMSI_SUPPLIER: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_APPNAME: i32 = 18i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_AUTHOR: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_CHARCOUNT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_COMMENTS: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_CREATE_DTM: i32 = 12i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_DOC_SECURITY: i32 = 19i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_EDITTIME: i32 = 10i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_KEYWORDS: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_LASTAUTHOR: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_LASTPRINTED: i32 = 11i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_LASTSAVE_DTM: i32 = 13i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_PAGECOUNT: i32 = 14i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_REVNUMBER: i32 = 9i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_SUBJECT: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_TEMPLATE: i32 = 7i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_THUMBNAIL: i32 = 17i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_TITLE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PIDSI_WORDCOUNT: i32 = 15i32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_BEHAVIOR: u32 = 2147483651u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_CODEPAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_DICTIONARY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_FIRST_NAME_DEFAULT: u32 = 4095u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_FIRST_USABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_ILLEGAL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_LOCALE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_MAX_READONLY: u32 = 3221225471u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_MIN_READONLY: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_MODIFY_TIME: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PID_SECURITY: u32 = 2147483650u32;
#[repr(C)]
pub struct PMemoryAllocator(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct PROPBAG2 {
    pub dwType: u32,
    pub vt: u16,
    pub cfType: u16,
    pub dwHint: u32,
    pub pstrName: ::windows::core::PWSTR,
    pub clsid: ::windows::core::GUID,
}
impl ::core::marker::Copy for PROPBAG2 {}
impl ::core::clone::Clone for PROPBAG2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROPBAG2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROPBAG2").field("dwType", &self.dwType).field("vt", &self.vt).field("cfType", &self.cfType).field("dwHint", &self.dwHint).field("pstrName", &self.pstrName).field("clsid", &self.clsid).finish()
    }
}
unsafe impl ::windows::core::Abi for PROPBAG2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROPBAG2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPBAG2>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROPBAG2 {}
impl ::core::default::Default for PROPBAG2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_ANSI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_CASE_SENSITIVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_NONSIMPLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETFLAG_UNBUFFERED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct PROPSPEC {
    pub ulKind: PROPSPEC_KIND,
    pub Anonymous: PROPSPEC_0,
}
impl ::core::marker::Copy for PROPSPEC {}
impl ::core::clone::Clone for PROPSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROPSPEC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROPSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSPEC>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROPSPEC {}
impl ::core::default::Default for PROPSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub union PROPSPEC_0 {
    pub propid: u32,
    pub lpwstr: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PROPSPEC_0 {}
impl ::core::clone::Clone for PROPSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROPSPEC_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROPSPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSPEC_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROPSPEC_0 {}
impl ::core::default::Default for PROPSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPSPEC_KIND(pub u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PRSPEC_LPWSTR: PROPSPEC_KIND = PROPSPEC_KIND(0u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PRSPEC_PROPID: PROPSPEC_KIND = PROPSPEC_KIND(1u32);
impl ::core::marker::Copy for PROPSPEC_KIND {}
impl ::core::clone::Clone for PROPSPEC_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPSPEC_KIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROPSPEC_KIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPSPEC_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPSPEC_KIND").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT {
    fn clone(&self) -> Self {
        Self { Anonymous: self.Anonymous.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROPVARIANT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROPVARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROPVARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPVARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PROPVARIANT_0 {
    pub Anonymous: ::core::mem::ManuallyDrop<PROPVARIANT_0_0>,
    pub decVal: super::super::super::Foundation::DECIMAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROPVARIANT_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROPVARIANT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPVARIANT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROPVARIANT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPVARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPVARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        Self { vt: self.vt, wReserved1: self.wReserved1, wReserved2: self.wReserved2, wReserved3: self.wReserved3, Anonymous: self.Anonymous.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROPVARIANT_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROPVARIANT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.vt == other.vt && self.wReserved1 == other.wReserved1 && self.wReserved2 == other.wReserved2 && self.wReserved3 == other.wReserved3 && self.Anonymous == other.Anonymous
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROPVARIANT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPVARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
    pub puuid: *mut ::windows::core::GUID,
    pub pclipdata: *mut CLIPDATA,
    pub bstrVal: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>,
    pub bstrblobVal: BSTRBLOB,
    pub blob: super::BLOB,
    pub pszVal: ::windows::core::PSTR,
    pub pwszVal: ::windows::core::PWSTR,
    pub punkVal: ::core::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub pdispVal: ::core::mem::ManuallyDrop<::core::option::Option<super::IDispatch>>,
    pub pStream: ::core::mem::ManuallyDrop<::core::option::Option<super::IStream>>,
    pub pStorage: ::core::mem::ManuallyDrop<::core::option::Option<IStorage>>,
    pub pVersionedStream: *mut VERSIONEDSTREAM,
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
    pub pcVal: ::windows::core::PSTR,
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
    pub pbstrVal: *mut super::super::super::Foundation::BSTR,
    pub ppunkVal: *mut ::core::option::Option<::windows::core::IUnknown>,
    pub ppdispVal: *mut ::core::option::Option<super::IDispatch>,
    pub pparray: *mut *mut super::SAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROPVARIANT_0_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROPVARIANT_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPVARIANT_0_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROPVARIANT_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const PRSPEC_INVALID: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn PropStgNameToFmtId<'a, P0>(oszname: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PropStgNameToFmtId(oszname: ::windows::core::PCWSTR, pfmtid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PropStgNameToFmtId(oszname.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PropVariantClear(pvar: *mut PROPVARIANT) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PropVariantClear(pvar: *mut PROPVARIANT) -> ::windows::core::HRESULT;
    }
    PropVariantClear(::core::mem::transmute(pvar)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PropVariantCopy(pvardest: *mut PROPVARIANT, pvarsrc: *const PROPVARIANT) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PropVariantCopy(pvardest: *mut PROPVARIANT, pvarsrc: *const PROPVARIANT) -> ::windows::core::HRESULT;
    }
    PropVariantCopy(::core::mem::transmute(pvardest), ::core::mem::transmute(pvarsrc)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn ReadClassStg<'a, P0>(pstg: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReadClassStg(pstg: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    ReadClassStg(pstg.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn ReadClassStm<'a, P0>(pstm: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::IStream>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReadClassStm(pstm: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    ReadClassStm(pstm.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn ReadFmtUserTypeStg<'a, P0>(pstg: P0, pcf: *mut u16, lplpszusertype: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReadFmtUserTypeStg(pstg: *mut ::core::ffi::c_void, pcf: *mut u16, lplpszusertype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    ReadFmtUserTypeStg(pstg.into().abi(), ::core::mem::transmute(pcf), ::core::mem::transmute(lplpszusertype)).ok()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct RemSNB {
    pub ulCntStr: u32,
    pub ulCntChar: u32,
    pub rgString: [u16; 1],
}
impl ::core::marker::Copy for RemSNB {}
impl ::core::clone::Clone for RemSNB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RemSNB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemSNB").field("ulCntStr", &self.ulCntStr).field("ulCntChar", &self.ulCntChar).field("rgString", &self.rgString).finish()
    }
}
unsafe impl ::windows::core::Abi for RemSNB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RemSNB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RemSNB>()) == 0 }
    }
}
impl ::core::cmp::Eq for RemSNB {}
impl ::core::default::Default for RemSNB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct SERIALIZEDPROPERTYVALUE {
    pub dwType: u32,
    pub rgb: [u8; 1],
}
impl ::core::marker::Copy for SERIALIZEDPROPERTYVALUE {}
impl ::core::clone::Clone for SERIALIZEDPROPERTYVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERIALIZEDPROPERTYVALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIALIZEDPROPERTYVALUE").field("dwType", &self.dwType).field("rgb", &self.rgb).finish()
    }
}
unsafe impl ::windows::core::Abi for SERIALIZEDPROPERTYVALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERIALIZEDPROPERTYVALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERIALIZEDPROPERTYVALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERIALIZEDPROPERTYVALUE {}
impl ::core::default::Default for SERIALIZEDPROPERTYVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STATFLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STATFLAG_DEFAULT: STATFLAG = STATFLAG(0i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STATFLAG_NONAME: STATFLAG = STATFLAG(1i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STATFLAG_NOOPEN: STATFLAG = STATFLAG(2i32);
impl ::core::marker::Copy for STATFLAG {}
impl ::core::clone::Clone for STATFLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STATFLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STATFLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for STATFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STATFLAG").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STATPROPSETSTG {
    pub fmtid: ::windows::core::GUID,
    pub clsid: ::windows::core::GUID,
    pub grfFlags: u32,
    pub mtime: super::super::super::Foundation::FILETIME,
    pub ctime: super::super::super::Foundation::FILETIME,
    pub atime: super::super::super::Foundation::FILETIME,
    pub dwOSVersion: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STATPROPSETSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATPROPSETSTG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STATPROPSETSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATPROPSETSTG").field("fmtid", &self.fmtid).field("clsid", &self.clsid).field("grfFlags", &self.grfFlags).field("mtime", &self.mtime).field("ctime", &self.ctime).field("atime", &self.atime).field("dwOSVersion", &self.dwOSVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STATPROPSETSTG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STATPROPSETSTG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STATPROPSETSTG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STATPROPSETSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STATPROPSETSTG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct STATPROPSTG {
    pub lpwstrName: ::windows::core::PWSTR,
    pub propid: u32,
    pub vt: u16,
}
impl ::core::marker::Copy for STATPROPSTG {}
impl ::core::clone::Clone for STATPROPSTG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STATPROPSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATPROPSTG").field("lpwstrName", &self.lpwstrName).field("propid", &self.propid).field("vt", &self.vt).finish()
    }
}
unsafe impl ::windows::core::Abi for STATPROPSTG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STATPROPSTG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STATPROPSTG>()) == 0 }
    }
}
impl ::core::cmp::Eq for STATPROPSTG {}
impl ::core::default::Default for STATPROPSTG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STGFMT(pub u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_STORAGE: STGFMT = STGFMT(0u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_NATIVE: STGFMT = STGFMT(1u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_FILE: STGFMT = STGFMT(3u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_ANY: STGFMT = STGFMT(4u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_DOCFILE: STGFMT = STGFMT(5u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGFMT_DOCUMENT: STGFMT = STGFMT(0u32);
impl ::core::marker::Copy for STGFMT {}
impl ::core::clone::Clone for STGFMT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STGFMT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STGFMT {
    type Abi = Self;
}
impl ::core::fmt::Debug for STGFMT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGFMT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STGM(pub u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_DIRECT: STGM = STGM(0u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_TRANSACTED: STGM = STGM(65536u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SIMPLE: STGM = STGM(134217728u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_READ: STGM = STGM(0u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_WRITE: STGM = STGM(1u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_READWRITE: STGM = STGM(2u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SHARE_DENY_NONE: STGM = STGM(64u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SHARE_DENY_READ: STGM = STGM(48u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SHARE_DENY_WRITE: STGM = STGM(32u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_SHARE_EXCLUSIVE: STGM = STGM(16u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_PRIORITY: STGM = STGM(262144u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_DELETEONRELEASE: STGM = STGM(67108864u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_NOSCRATCH: STGM = STGM(1048576u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_CREATE: STGM = STGM(4096u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_CONVERT: STGM = STGM(131072u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_FAILIFTHERE: STGM = STGM(0u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_NOSNAPSHOT: STGM = STGM(2097152u32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGM_DIRECT_SWMR: STGM = STGM(4194304u32);
impl ::core::marker::Copy for STGM {}
impl ::core::clone::Clone for STGM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STGM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STGM {
    type Abi = Self;
}
impl ::core::fmt::Debug for STGM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGM").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for STGM {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for STGM {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for STGM {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for STGM {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for STGM {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STGMOVE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGMOVE_MOVE: STGMOVE = STGMOVE(0i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGMOVE_COPY: STGMOVE = STGMOVE(1i32);
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGMOVE_SHALLOWCOPY: STGMOVE = STGMOVE(2i32);
impl ::core::marker::Copy for STGMOVE {}
impl ::core::clone::Clone for STGMOVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STGMOVE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STGMOVE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STGMOVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGMOVE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct STGOPTIONS {
    pub usVersion: u16,
    pub reserved: u16,
    pub ulSectorSize: u32,
    pub pwcsTemplateFile: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for STGOPTIONS {}
impl ::core::clone::Clone for STGOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STGOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STGOPTIONS").field("usVersion", &self.usVersion).field("reserved", &self.reserved).field("ulSectorSize", &self.ulSectorSize).field("pwcsTemplateFile", &self.pwcsTemplateFile).finish()
    }
}
unsafe impl ::windows::core::Abi for STGOPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STGOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STGOPTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for STGOPTIONS {}
impl ::core::default::Default for STGOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub const STGOPTIONS_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConvertStg<'a, P0, P1>(pstg: P0, fconvert: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
    P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConvertStg(pstg: *mut ::core::ffi::c_void, fconvert: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    SetConvertStg(pstg.into().abi(), fconvert.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgConvertPropertyToVariant(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut PROPVARIANT, pma: *const PMemoryAllocator) -> super::super::super::Foundation::BOOLEAN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgConvertPropertyToVariant(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut PROPVARIANT, pma: *const PMemoryAllocator) -> super::super::super::Foundation::BOOLEAN;
    }
    StgConvertPropertyToVariant(::core::mem::transmute(pprop), codepage, ::core::mem::transmute(pvar), ::core::mem::transmute(pma))
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgConvertVariantToProperty<'a, P0>(pvar: *const PROPVARIANT, codepage: u16, pprop: *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32, pid: u32, freserved: P0, pcindirect: *mut u32) -> *mut SERIALIZEDPROPERTYVALUE
where
    P0: ::std::convert::Into<super::super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgConvertVariantToProperty(pvar: *const PROPVARIANT, codepage: u16, pprop: *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32, pid: u32, freserved: super::super::super::Foundation::BOOLEAN, pcindirect: *mut u32) -> *mut SERIALIZEDPROPERTYVALUE;
    }
    StgConvertVariantToProperty(::core::mem::transmute(pvar), codepage, ::core::mem::transmute(pprop), ::core::mem::transmute(pcb), pid, freserved.into(), ::core::mem::transmute(pcindirect))
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgCreateDocfile<'a, P0>(pwcsname: P0, grfmode: STGM, reserved: u32) -> ::windows::core::Result<IStorage>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgCreateDocfile(pwcsname: ::windows::core::PCWSTR, grfmode: STGM, reserved: u32, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StgCreateDocfile(pwcsname.into(), grfmode, reserved, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStorage>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgCreateDocfileOnILockBytes<'a, P0>(plkbyt: P0, grfmode: STGM, reserved: u32) -> ::windows::core::Result<IStorage>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ILockBytes>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgCreateDocfileOnILockBytes(plkbyt: *mut ::core::ffi::c_void, grfmode: STGM, reserved: u32, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StgCreateDocfileOnILockBytes(plkbyt.into().abi(), grfmode, reserved, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStorage>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgCreatePropSetStg<'a, P0>(pstorage: P0, dwreserved: u32) -> ::windows::core::Result<IPropertySetStorage>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgCreatePropSetStg(pstorage: *mut ::core::ffi::c_void, dwreserved: u32, pppropsetstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StgCreatePropSetStg(pstorage.into().abi(), dwreserved, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPropertySetStorage>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgCreatePropStg<'a, P0>(punk: P0, fmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, dwreserved: u32) -> ::windows::core::Result<IPropertyStorage>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgCreatePropStg(punk: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StgCreatePropStg(punk.into().abi(), ::core::mem::transmute(fmtid), ::core::mem::transmute(pclsid), grfflags, dwreserved, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPropertyStorage>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn StgCreateStorageEx<'a, P0, P1>(pwcsname: P0, grfmode: STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: P1, riid: *const ::windows::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::super::Security::PSECURITY_DESCRIPTOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgCreateStorageEx(pwcsname: ::windows::core::PCWSTR, grfmode: STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: super::super::super::Security::PSECURITY_DESCRIPTOR, riid: *const ::windows::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    StgCreateStorageEx(pwcsname.into(), grfmode, stgfmt, grfattrs, ::core::mem::transmute(pstgoptions), psecuritydescriptor.into(), ::core::mem::transmute(riid), ::core::mem::transmute(ppobjectopen)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32) -> ::windows::core::Result<PROPVARIANT> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32, ppropvar: *mut PROPVARIANT) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StgDeserializePropVariant(::core::mem::transmute(pprop), cbmax, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPVARIANT>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgGetIFillLockBytesOnFile<'a, P0>(pwcsname: P0) -> ::windows::core::Result<IFillLockBytes>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgGetIFillLockBytesOnFile(pwcsname: ::windows::core::PCWSTR, ppflb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StgGetIFillLockBytesOnFile(pwcsname.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFillLockBytes>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgGetIFillLockBytesOnILockBytes<'a, P0>(pilb: P0) -> ::windows::core::Result<IFillLockBytes>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ILockBytes>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgGetIFillLockBytesOnILockBytes(pilb: *mut ::core::ffi::c_void, ppflb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StgGetIFillLockBytesOnILockBytes(pilb.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFillLockBytes>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgIsStorageFile<'a, P0>(pwcsname: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgIsStorageFile(pwcsname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    StgIsStorageFile(pwcsname.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgIsStorageILockBytes<'a, P0>(plkbyt: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ILockBytes>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgIsStorageILockBytes(plkbyt: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    StgIsStorageILockBytes(plkbyt.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgOpenAsyncDocfileOnIFillLockBytes<'a, P0>(pflb: P0, grfmode: u32, asyncflags: u32) -> ::windows::core::Result<IStorage>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IFillLockBytes>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgOpenAsyncDocfileOnIFillLockBytes(pflb: *mut ::core::ffi::c_void, grfmode: u32, asyncflags: u32, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StgOpenAsyncDocfileOnIFillLockBytes(pflb.into().abi(), grfmode, asyncflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStorage>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgOpenLayoutDocfile<'a, P0>(pwcsdfname: P0, grfmode: u32, reserved: u32) -> ::windows::core::Result<IStorage>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgOpenLayoutDocfile(pwcsdfname: ::windows::core::PCWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StgOpenLayoutDocfile(pwcsdfname.into(), grfmode, reserved, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStorage>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgOpenPropStg<'a, P0>(punk: P0, fmtid: *const ::windows::core::GUID, grfflags: u32, dwreserved: u32) -> ::windows::core::Result<IPropertyStorage>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgOpenPropStg(punk: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StgOpenPropStg(punk.into().abi(), ::core::mem::transmute(fmtid), grfflags, dwreserved, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPropertyStorage>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgOpenStorage<'a, P0, P1>(pwcsname: P0, pstgpriority: P1, grfmode: STGM, snbexclude: *const *const u16, reserved: u32) -> ::windows::core::Result<IStorage>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgOpenStorage(pwcsname: ::windows::core::PCWSTR, pstgpriority: *mut ::core::ffi::c_void, grfmode: STGM, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StgOpenStorage(pwcsname.into(), pstgpriority.into().abi(), grfmode, ::core::mem::transmute(snbexclude), reserved, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStorage>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn StgOpenStorageEx<'a, P0, P1>(pwcsname: P0, grfmode: STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: P1, riid: *const ::windows::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::super::Security::PSECURITY_DESCRIPTOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgOpenStorageEx(pwcsname: ::windows::core::PCWSTR, grfmode: STGM, stgfmt: STGFMT, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: super::super::super::Security::PSECURITY_DESCRIPTOR, riid: *const ::windows::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    StgOpenStorageEx(pwcsname.into(), grfmode, stgfmt, grfattrs, ::core::mem::transmute(pstgoptions), psecuritydescriptor.into(), ::core::mem::transmute(riid), ::core::mem::transmute(ppobjectopen)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgOpenStorageOnILockBytes<'a, P0, P1>(plkbyt: P0, pstgpriority: P1, grfmode: u32, snbexclude: *const *const u16, reserved: u32) -> ::windows::core::Result<IStorage>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ILockBytes>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgOpenStorageOnILockBytes(plkbyt: *mut ::core::ffi::c_void, pstgpriority: *mut ::core::ffi::c_void, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    StgOpenStorageOnILockBytes(plkbyt.into().abi(), pstgpriority.into().abi(), grfmode, ::core::mem::transmute(snbexclude), reserved, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStorage>(result__)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32;
    }
    StgPropertyLengthAsVariant(::core::mem::transmute(pprop), cbprop, codepage, breserved)
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgSerializePropVariant(ppropvar: *const PROPVARIANT, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgSerializePropVariant(ppropvar: *const PROPVARIANT, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows::core::HRESULT;
    }
    StgSerializePropVariant(::core::mem::transmute(ppropvar), ::core::mem::transmute(ppprop), ::core::mem::transmute(pcb)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgSetTimes<'a, P0>(lpszname: P0, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StgSetTimes(lpszname: ::windows::core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
    }
    StgSetTimes(lpszname.into(), ::core::mem::transmute(pctime), ::core::mem::transmute(patime), ::core::mem::transmute(pmtime)).ok()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
pub struct VERSIONEDSTREAM {
    pub guidVersion: ::windows::core::GUID,
    pub pStream: ::core::option::Option<super::IStream>,
}
impl ::core::clone::Clone for VERSIONEDSTREAM {
    fn clone(&self) -> Self {
        Self { guidVersion: self.guidVersion, pStream: self.pStream.clone() }
    }
}
impl ::core::fmt::Debug for VERSIONEDSTREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VERSIONEDSTREAM").field("guidVersion", &self.guidVersion).field("pStream", &self.pStream).finish()
    }
}
unsafe impl ::windows::core::Abi for VERSIONEDSTREAM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for VERSIONEDSTREAM {
    fn eq(&self, other: &Self) -> bool {
        self.guidVersion == other.guidVersion && self.pStream == other.pStream
    }
}
impl ::core::cmp::Eq for VERSIONEDSTREAM {}
impl ::core::default::Default for VERSIONEDSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn WriteClassStg<'a, P0>(pstg: P0, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WriteClassStg(pstg: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
    }
    WriteClassStg(pstg.into().abi(), ::core::mem::transmute(rclsid)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn WriteClassStm<'a, P0>(pstm: P0, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::IStream>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WriteClassStm(pstm: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
    }
    WriteClassStm(pstm.into().abi(), ::core::mem::transmute(rclsid)).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
#[inline]
pub unsafe fn WriteFmtUserTypeStg<'a, P0, P1>(pstg: P0, cf: u16, lpszusertype: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, IStorage>>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WriteFmtUserTypeStg(pstg: *mut ::core::ffi::c_void, cf: u16, lpszusertype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    WriteFmtUserTypeStg(pstg.into().abi(), cf, lpszusertype.into()).ok()
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

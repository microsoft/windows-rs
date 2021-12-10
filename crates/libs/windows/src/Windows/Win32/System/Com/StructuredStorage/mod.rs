#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub struct CAC {
    pub cElems: u32,
    pub pElems: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CAC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CAC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CAC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CAC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALPSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALPSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CALPSTR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CALPSTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CALPSTR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CALPSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CALPSTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALPWSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALPWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CALPWSTR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CALPWSTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CALPWSTR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CALPWSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CALPWSTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
pub const CCH_MAX_PROPSTG_NAME: u32 = 31u32;
#[repr(C)]
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
pub const CWCSTORAGENAME: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoGetInstanceFromFile<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::core::GUID, punkouter: Param2, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: Param5, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetInstanceFromFile(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: super::CLSCTX, grfmode: u32, pwszname: super::super::super::Foundation::PWSTR, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows::core::HRESULT;
        }
        CoGetInstanceFromFile(::core::mem::transmute(pserverinfo), ::core::mem::transmute(pclsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(grfmode), pwszname.into_param().abi(), ::core::mem::transmute(dwcount), ::core::mem::transmute(presults)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoGetInstanceFromIStorage<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, IStorage>>(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::core::GUID, punkouter: Param2, dwclsctx: super::CLSCTX, pstg: Param4, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetInstanceFromIStorage(pserverinfo: *const super::COSERVERINFO, pclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: super::CLSCTX, pstg: ::windows::core::RawPtr, dwcount: u32, presults: *mut super::MULTI_QI) -> ::windows::core::HRESULT;
        }
        CoGetInstanceFromIStorage(::core::mem::transmute(pserverinfo), ::core::mem::transmute(pclsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclsctx), pstg.into_param().abi(), ::core::mem::transmute(dwcount), ::core::mem::transmute(presults)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetInterfaceAndReleaseStream<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>, T: ::windows::core::Interface>(pstm: Param0) -> ::windows::core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetInterfaceAndReleaseStream(pstm: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CoGetInterfaceAndReleaseStream(pstm.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateILockBytesOnHGlobal<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(hglobal: isize, fdeleteonrelease: Param1) -> ::windows::core::Result<ILockBytes> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateILockBytesOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, pplkbyt: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateILockBytesOnHGlobal(::core::mem::transmute(hglobal), fdeleteonrelease.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ILockBytes>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStreamOnHGlobal<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(hglobal: isize, fdeleteonrelease: Param1) -> ::windows::core::Result<super::IStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStreamOnHGlobal(hglobal: isize, fdeleteonrelease: super::super::super::Foundation::BOOL, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateStreamOnHGlobal(::core::mem::transmute(hglobal), fdeleteonrelease.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::IStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FmtIdToPropStgName(pfmtid: *const ::windows::core::GUID, oszname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FmtIdToPropStgName(pfmtid: *const ::windows::core::GUID, oszname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        FmtIdToPropStgName(::core::mem::transmute(pfmtid), ::core::mem::transmute(oszname)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreePropVariantArray(cvariants: u32, rgvars: *mut PROPVARIANT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreePropVariantArray(cvariants: u32, rgvars: *mut PROPVARIANT) -> ::windows::core::HRESULT;
        }
        FreePropVariantArray(::core::mem::transmute(cvariants), ::core::mem::transmute(rgvars)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConvertStg<'a, Param0: ::windows::core::IntoParam<'a, IStorage>>(pstg: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConvertStg(pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        GetConvertStg(pstg.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetHGlobalFromILockBytes<'a, Param0: ::windows::core::IntoParam<'a, ILockBytes>>(plkbyt: Param0) -> ::windows::core::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetHGlobalFromILockBytes(plkbyt: ::windows::core::RawPtr, phglobal: *mut isize) -> ::windows::core::HRESULT;
        }
        let mut result__: isize = ::core::mem::zeroed();
        GetHGlobalFromILockBytes(plkbyt.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetHGlobalFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(pstm: Param0) -> ::windows::core::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetHGlobalFromStream(pstm: ::windows::core::RawPtr, phglobal: *mut isize) -> ::windows::core::HRESULT;
        }
        let mut result__: isize = ::core::mem::zeroed();
        GetHGlobalFromStream(pstm.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IDirectWriterLock(::windows::core::IUnknown);
impl IDirectWriterLock {
    pub unsafe fn WaitForWriteAccess(&self, dwtimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtimeout)).ok()
    }
    pub unsafe fn ReleaseWriteAccess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn HaveWriteAccess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDirectWriterLock> for ::windows::core::IUnknown {
    fn from(value: IDirectWriterLock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectWriterLock> for ::windows::core::IUnknown {
    fn from(value: &IDirectWriterLock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectWriterLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectWriterLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IDirectWriterLock {
    type Vtable = IDirectWriterLockVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e6d4d92_6738_11cf_9608_00aa00680db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectWriterLockVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumSTATPROPSETSTG(::windows::core::IUnknown);
impl IEnumSTATPROPSETSTG {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATPROPSETSTG> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSTATPROPSETSTG>(result__)
    }
}
impl ::core::convert::From<IEnumSTATPROPSETSTG> for ::windows::core::IUnknown {
    fn from(value: IEnumSTATPROPSETSTG) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSTATPROPSETSTG> for ::windows::core::IUnknown {
    fn from(value: &IEnumSTATPROPSETSTG) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumSTATPROPSETSTG {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumSTATPROPSETSTG {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IEnumSTATPROPSETSTG {
    type Vtable = IEnumSTATPROPSETSTGVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATPROPSETSTGVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumSTATPROPSTG(::windows::core::IUnknown);
impl IEnumSTATPROPSTG {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATPROPSTG> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSTATPROPSTG>(result__)
    }
}
impl ::core::convert::From<IEnumSTATPROPSTG> for ::windows::core::IUnknown {
    fn from(value: IEnumSTATPROPSTG) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSTATPROPSTG> for ::windows::core::IUnknown {
    fn from(value: &IEnumSTATPROPSTG) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumSTATPROPSTG {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumSTATPROPSTG {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IEnumSTATPROPSTG {
    type Vtable = IEnumSTATPROPSTGVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000139_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATPROPSTGVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumSTATSTG(::windows::core::IUnknown);
impl IEnumSTATSTG {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATSTG> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSTATSTG>(result__)
    }
}
impl ::core::convert::From<IEnumSTATSTG> for ::windows::core::IUnknown {
    fn from(value: IEnumSTATSTG) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSTATSTG> for ::windows::core::IUnknown {
    fn from(value: &IEnumSTATSTG) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumSTATSTG {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumSTATSTG {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IEnumSTATSTG {
    type Vtable = IEnumSTATSTGVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATSTGVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFillLockBytes(::windows::core::IUnknown);
impl IFillLockBytes {
    pub unsafe fn FillAppend(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn FillAt(&self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uloffset), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetFillSize(&self, ulsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Terminate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, bcanceled: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bcanceled.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFillLockBytes> for ::windows::core::IUnknown {
    fn from(value: IFillLockBytes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFillLockBytes> for ::windows::core::IUnknown {
    fn from(value: &IFillLockBytes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFillLockBytes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFillLockBytes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IFillLockBytes {
    type Vtable = IFillLockBytesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99caf010_415e_11cf_8814_00aa00b569f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFillLockBytesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsize: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bcanceled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ILayoutStorage(::windows::core::IUnknown);
impl ILayoutStorage {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LayoutScript(&self, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstoragelayout), ::core::mem::transmute(nentries), ::core::mem::transmute(glfinterleavedflag)).ok()
    }
    pub unsafe fn BeginMonitor(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EndMonitor(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReLayoutDocfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsnewdfname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pwcsnewdfname.into_param().abi()).ok()
    }
    pub unsafe fn ReLayoutDocfileOnILockBytes<'a, Param0: ::windows::core::IntoParam<'a, ILockBytes>>(&self, pilockbytes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pilockbytes.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ILayoutStorage> for ::windows::core::IUnknown {
    fn from(value: ILayoutStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILayoutStorage> for ::windows::core::IUnknown {
    fn from(value: &ILayoutStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILayoutStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILayoutStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ILayoutStorage {
    type Vtable = ILayoutStorageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e6d4d90_6738_11cf_9608_00aa00680db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutStorageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsnewdfname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pilockbytes: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ILockBytes(::windows::core::IUnknown);
impl ILockBytes {
    pub unsafe fn ReadAt(&self, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uloffset), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    pub unsafe fn WriteAt(&self, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uloffset), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetSize(&self, cb: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cb)).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
}
impl ::core::convert::From<ILockBytes> for ::windows::core::IUnknown {
    fn from(value: ILockBytes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockBytes> for ::windows::core::IUnknown {
    fn from(value: &ILockBytes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILockBytes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILockBytes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ILockBytes {
    type Vtable = ILockBytesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockBytesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cb: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IPersistStorage(::windows::core::IUnknown);
impl IPersistStorage {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn InitNew<'a, Param0: ::windows::core::IntoParam<'a, IStorage>>(&self, pstg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstg.into_param().abi()).ok()
    }
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IStorage>>(&self, pstg: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstg.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, IStorage>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, pstgsave: Param0, fsameasload: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstgsave.into_param().abi(), fsameasload.into_param().abi()).ok()
    }
    pub unsafe fn SaveCompleted<'a, Param0: ::windows::core::IntoParam<'a, IStorage>>(&self, pstgnew: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pstgnew.into_param().abi()).ok()
    }
    pub unsafe fn HandsOffStorage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IPersistStorage> for super::IPersist {
    fn from(value: IPersistStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStorage> for super::IPersist {
    fn from(value: &IPersistStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IPersist> for IPersistStorage {
    fn into_param(self) -> ::windows::core::Param<'a, super::IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IPersist> for &IPersistStorage {
    fn into_param(self) -> ::windows::core::Param<'a, super::IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPersistStorage> for ::windows::core::IUnknown {
    fn from(value: IPersistStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStorage> for ::windows::core::IUnknown {
    fn from(value: &IPersistStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPersistStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IPersistStorage {
    type Vtable = IPersistStorageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStorageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstgsave: ::windows::core::RawPtr, fsameasload: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstgnew: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IPropertyBag(::windows::core::IUnknown);
impl IPropertyBag {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::IErrorLog>>(&self, pszpropname: Param0, pvar: *mut super::VARIANT, perrorlog: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar), perrorlog.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pszpropname: Param0, pvar: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pvar)).ok()
    }
}
impl ::core::convert::From<IPropertyBag> for ::windows::core::IUnknown {
    fn from(value: IPropertyBag) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyBag> for ::windows::core::IUnknown {
    fn from(value: &IPropertyBag) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyBag {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPropertyBag {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IPropertyBag {
    type Vtable = IPropertyBagVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55272a00_42cb_11ce_8135_00aa004bb851);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBagVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropname: super::super::super::Foundation::PWSTR, pvar: *mut super::VARIANT, perrorlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropname: super::super::super::Foundation::PWSTR, pvar: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct IPropertyBag2(::windows::core::IUnknown);
impl IPropertyBag2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<'a, Param2: ::windows::core::IntoParam<'a, super::IErrorLog>>(&self, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: Param2, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cproperties), ::core::mem::transmute(ppropbag), perrlog.into_param().abi(), ::core::mem::transmute(pvarvalue), ::core::mem::transmute(phrerror)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Write(&self, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cproperties), ::core::mem::transmute(ppropbag), ::core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn CountProperties(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyInfo(&self, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iproperty), ::core::mem::transmute(cproperties), ::core::mem::transmute(ppropbag), ::core::mem::transmute(pcproperties)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, super::IErrorLog>>(&self, pstrname: Param0, dwhint: u32, punkobject: Param2, perrlog: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstrname.into_param().abi(), ::core::mem::transmute(dwhint), punkobject.into_param().abi(), perrlog.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPropertyBag2> for ::windows::core::IUnknown {
    fn from(value: IPropertyBag2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyBag2> for ::windows::core::IUnknown {
    fn from(value: &IPropertyBag2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IPropertyBag2 {
    type Vtable = IPropertyBag2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22f55882_280b_11d0_a8a9_00a0c90c2004);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyBag2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: ::windows::core::RawPtr, pvarvalue: *mut super::VARIANT, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcproperties: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrname: super::super::super::Foundation::PWSTR, dwhint: u32, punkobject: *mut ::core::ffi::c_void, perrlog: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IPropertySetStorage(::windows::core::IUnknown);
impl IPropertySetStorage {
    pub unsafe fn Create(&self, rfmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32) -> ::windows::core::Result<IPropertyStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rfmtid), ::core::mem::transmute(pclsid), ::core::mem::transmute(grfflags), ::core::mem::transmute(grfmode), ::core::mem::transmute(&mut result__)).from_abi::<IPropertyStorage>(result__)
    }
    pub unsafe fn Open(&self, rfmtid: *const ::windows::core::GUID, grfmode: u32) -> ::windows::core::Result<IPropertyStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rfmtid), ::core::mem::transmute(grfmode), ::core::mem::transmute(&mut result__)).from_abi::<IPropertyStorage>(result__)
    }
    pub unsafe fn Delete(&self, rfmtid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rfmtid)).ok()
    }
    pub unsafe fn Enum(&self) -> ::windows::core::Result<IEnumSTATPROPSETSTG> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSTATPROPSETSTG>(result__)
    }
}
impl ::core::convert::From<IPropertySetStorage> for ::windows::core::IUnknown {
    fn from(value: IPropertySetStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertySetStorage> for ::windows::core::IUnknown {
    fn from(value: &IPropertySetStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertySetStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPropertySetStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IPropertySetStorage {
    type Vtable = IPropertySetStorageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetStorageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID, grfmode: u32, ppprstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IPropertyStorage(::windows::core::IUnknown);
impl IPropertyStorage {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpspec), ::core::mem::transmute(rgpspec), ::core::mem::transmute(rgpropvar)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpspec), ::core::mem::transmute(rgpspec), ::core::mem::transmute(rgpropvar), ::core::mem::transmute(propidnamefirst)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteMultiple(&self, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpspec), ::core::mem::transmute(rgpspec)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropid), ::core::mem::transmute(rgpropid), ::core::mem::transmute(rglpwstrname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropid), ::core::mem::transmute(rgpropid), ::core::mem::transmute(rglpwstrname)).ok()
    }
    pub unsafe fn DeletePropertyNames(&self, cpropid: u32, rgpropid: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpropid), ::core::mem::transmute(rgpropid)).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Enum(&self) -> ::windows::core::Result<IEnumSTATPROPSTG> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSTATPROPSTG>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTimes(&self, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pctime), ::core::mem::transmute(patime), ::core::mem::transmute(pmtime)).ok()
    }
    pub unsafe fn SetClass(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self) -> ::windows::core::Result<STATPROPSETSTG> {
        let mut result__: STATPROPSETSTG = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<STATPROPSETSTG>(result__)
    }
}
impl ::core::convert::From<IPropertyStorage> for ::windows::core::IUnknown {
    fn from(value: IPropertyStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPropertyStorage> for ::windows::core::IUnknown {
    fn from(value: &IPropertyStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPropertyStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IPropertyStorage {
    type Vtable = IPropertyStorageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000138_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStorageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatpsstg: *mut STATPROPSETSTG) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IRootStorage(::windows::core::IUnknown);
impl IRootStorage {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SwitchToFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pszfile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszfile.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRootStorage> for ::windows::core::IUnknown {
    fn from(value: IRootStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRootStorage> for ::windows::core::IUnknown {
    fn from(value: &IRootStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRootStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRootStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IRootStorage {
    type Vtable = IRootStorageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000012_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRootStorageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfile: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IStorage(::windows::core::IUnknown);
impl IStorage {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsname: Param0, grfmode: u32, reserved1: u32, reserved2: u32) -> ::windows::core::Result<super::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2), ::core::mem::transmute(&mut result__)).from_abi::<super::IStream>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsname: Param0, reserved1: *mut ::core::ffi::c_void, grfmode: u32, reserved2: u32, ppstm: *mut ::core::option::Option<super::IStream>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), ::core::mem::transmute(reserved1), ::core::mem::transmute(grfmode), ::core::mem::transmute(reserved2), ::core::mem::transmute(ppstm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateStorage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsname: Param0, grfmode: u32, reserved1: u32, reserved2: u32) -> ::windows::core::Result<IStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2), ::core::mem::transmute(&mut result__)).from_abi::<IStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenStorage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IStorage>>(&self, pwcsname: Param0, pstgpriority: Param1, grfmode: u32, snbexclude: *const *const u16, reserved: u32) -> ::windows::core::Result<IStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), pstgpriority.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(snbexclude), ::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<IStorage>(result__)
    }
    pub unsafe fn CopyTo<'a, Param3: ::windows::core::IntoParam<'a, IStorage>>(&self, ciidexclude: u32, rgiidexclude: *const ::windows::core::GUID, snbexclude: *const *const u16, pstgdest: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ciidexclude), ::core::mem::transmute(rgiidexclude), ::core::mem::transmute(snbexclude), pstgdest.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveElementTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IStorage>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsname: Param0, pstgdest: Param1, pwcsnewname: Param2, grfflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), pstgdest.into_param().abi(), pwcsnewname.into_param().abi(), ::core::mem::transmute(grfflags)).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EnumElements(&self, reserved1: u32, reserved2: *mut ::core::ffi::c_void, reserved3: u32, ppenum: *mut ::core::option::Option<IEnumSTATSTG>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2), ::core::mem::transmute(reserved3), ::core::mem::transmute(ppenum)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pwcsname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenameElement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsoldname: Param0, pwcsnewname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pwcsoldname.into_param().abi(), pwcsnewname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetElementTimes<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwcsname: Param0, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), ::core::mem::transmute(pctime), ::core::mem::transmute(patime), ::core::mem::transmute(pmtime)).ok()
    }
    pub unsafe fn SetClass(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid)).ok()
    }
    pub unsafe fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfstatebits), ::core::mem::transmute(grfmask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
}
impl ::core::convert::From<IStorage> for ::windows::core::IUnknown {
    fn from(value: IStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorage> for ::windows::core::IUnknown {
    fn from(value: &IStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IStorage {
    type Vtable = IStorageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved1: u32, reserved2: u32, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, reserved1: *mut ::core::ffi::c_void, grfmode: u32, reserved2: u32, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved1: u32, reserved2: u32, ppstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, pstgpriority: ::windows::core::RawPtr, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ciidexclude: u32, rgiidexclude: *const ::windows::core::GUID, snbexclude: *const *const u16, pstgdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, pstgdest: ::windows::core::RawPtr, pwcsnewname: super::super::super::Foundation::PWSTR, grfflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved1: u32, reserved2: *mut ::core::ffi::c_void, reserved3: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsoldname: super::super::super::Foundation::PWSTR, pwcsnewname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: super::super::super::Foundation::PWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfstatebits: u32, grfmask: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub type LOCKTYPE = i32;
pub const LOCK_WRITE: LOCKTYPE = 1i32;
pub const LOCK_EXCLUSIVE: LOCKTYPE = 2i32;
pub const LOCK_ONLYONCE: LOCKTYPE = 4i32;
#[repr(C)]
pub struct OLESTREAM {
    pub lpstbl: *mut OLESTREAMVTBL,
}
impl ::core::marker::Copy for OLESTREAM {}
impl ::core::clone::Clone for OLESTREAM {
    fn clone(&self) -> Self {
        *self
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
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAM<'a, Param0: ::windows::core::IntoParam<'a, IStorage>>(pstg: Param0, lpolestream: *mut OLESTREAM) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleConvertIStorageToOLESTREAM(pstg: ::windows::core::RawPtr, lpolestream: *mut OLESTREAM) -> ::windows::core::HRESULT;
        }
        OleConvertIStorageToOLESTREAM(pstg.into_param().abi(), ::core::mem::transmute(lpolestream)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAMEx<'a, Param0: ::windows::core::IntoParam<'a, IStorage>>(pstg: Param0, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *mut super::STGMEDIUM, polestm: *mut OLESTREAM) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleConvertIStorageToOLESTREAMEx(pstg: ::windows::core::RawPtr, cfformat: u16, lwidth: i32, lheight: i32, dwsize: u32, pmedium: *mut super::STGMEDIUM, polestm: *mut OLESTREAM) -> ::windows::core::HRESULT;
        }
        OleConvertIStorageToOLESTREAMEx(pstg.into_param().abi(), ::core::mem::transmute(cfformat), ::core::mem::transmute(lwidth), ::core::mem::transmute(lheight), ::core::mem::transmute(dwsize), ::core::mem::transmute(pmedium), ::core::mem::transmute(polestm)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage<'a, Param1: ::windows::core::IntoParam<'a, IStorage>>(lpolestream: *mut OLESTREAM, pstg: Param1, ptd: *const super::DVTARGETDEVICE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleConvertOLESTREAMToIStorage(lpolestream: *mut OLESTREAM, pstg: ::windows::core::RawPtr, ptd: *const super::DVTARGETDEVICE) -> ::windows::core::HRESULT;
        }
        OleConvertOLESTREAMToIStorage(::core::mem::transmute(lpolestream), pstg.into_param().abi(), ::core::mem::transmute(ptd)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx<'a, Param1: ::windows::core::IntoParam<'a, IStorage>>(polestm: *mut OLESTREAM, pstg: Param1, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::STGMEDIUM) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OleConvertOLESTREAMToIStorageEx(polestm: *mut OLESTREAM, pstg: ::windows::core::RawPtr, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::STGMEDIUM) -> ::windows::core::HRESULT;
        }
        OleConvertOLESTREAMToIStorageEx(::core::mem::transmute(polestm), pstg.into_param().abi(), ::core::mem::transmute(pcfformat), ::core::mem::transmute(plwwidth), ::core::mem::transmute(plheight), ::core::mem::transmute(pdwsize), ::core::mem::transmute(pmedium)).ok()
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
pub type PIDMSI_STATUS_VALUE = i32;
pub const PIDMSI_STATUS_NORMAL: PIDMSI_STATUS_VALUE = 0i32;
pub const PIDMSI_STATUS_NEW: PIDMSI_STATUS_VALUE = 1i32;
pub const PIDMSI_STATUS_PRELIM: PIDMSI_STATUS_VALUE = 2i32;
pub const PIDMSI_STATUS_DRAFT: PIDMSI_STATUS_VALUE = 3i32;
pub const PIDMSI_STATUS_INPROGRESS: PIDMSI_STATUS_VALUE = 4i32;
pub const PIDMSI_STATUS_EDIT: PIDMSI_STATUS_VALUE = 5i32;
pub const PIDMSI_STATUS_REVIEW: PIDMSI_STATUS_VALUE = 6i32;
pub const PIDMSI_STATUS_PROOF: PIDMSI_STATUS_VALUE = 7i32;
pub const PIDMSI_STATUS_FINAL: PIDMSI_STATUS_VALUE = 8i32;
pub const PIDMSI_STATUS_OTHER: PIDMSI_STATUS_VALUE = 32767i32;
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
pub const PID_CODEPAGE: u32 = 1u32;
pub const PID_DICTIONARY: u32 = 0u32;
pub const PID_FIRST_NAME_DEFAULT: u32 = 4095u32;
pub const PID_FIRST_USABLE: u32 = 2u32;
pub const PID_ILLEGAL: u32 = 4294967295u32;
pub const PID_LOCALE: u32 = 2147483648u32;
pub const PID_MAX_READONLY: u32 = 3221225471u32;
pub const PID_MIN_READONLY: u32 = 2147483648u32;
pub const PID_MODIFY_TIME: u32 = 2147483649u32;
pub const PID_SECURITY: u32 = 2147483650u32;
#[repr(C)]
pub struct PMemoryAllocator(pub u8);
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPBAG2 {
    pub dwType: u32,
    pub vt: u16,
    pub cfType: u16,
    pub dwHint: u32,
    pub pstrName: super::super::super::Foundation::PWSTR,
    pub clsid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPBAG2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPBAG2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROPBAG2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROPBAG2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPBAG2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROPBAG2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPBAG2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const PROPSETFLAG_ANSI: u32 = 2u32;
pub const PROPSETFLAG_CASE_SENSITIVE: u32 = 8u32;
pub const PROPSETFLAG_DEFAULT: u32 = 0u32;
pub const PROPSETFLAG_NONSIMPLE: u32 = 1u32;
pub const PROPSETFLAG_UNBUFFERED: u32 = 4u32;
pub const PROPSETHDR_OSVERSION_UNKNOWN: u32 = 4294967295u32;
pub const PROPSET_BEHAVIOR_CASE_SENSITIVE: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPSPEC {
    pub ulKind: PROPSPEC_KIND,
    pub Anonymous: PROPSPEC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROPSPEC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROPSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSPEC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROPSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PROPSPEC_0 {
    pub propid: u32,
    pub lpwstr: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROPSPEC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROPSPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSPEC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROPSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type PROPSPEC_KIND = u32;
pub const PRSPEC_LPWSTR: PROPSPEC_KIND = 0u32;
pub const PRSPEC_PROPID: PROPSPEC_KIND = 1u32;
#[repr(C)]
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
    pub pszVal: super::super::super::Foundation::PSTR,
    pub pwszVal: super::super::super::Foundation::PWSTR,
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
pub const PRSPEC_INVALID: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PropStgNameToFmtId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(oszname: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropStgNameToFmtId(oszname: super::super::super::Foundation::PWSTR, pfmtid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        PropStgNameToFmtId(oszname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PropVariantClear(pvar: *mut PROPVARIANT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantClear(pvar: *mut PROPVARIANT) -> ::windows::core::HRESULT;
        }
        PropVariantClear(::core::mem::transmute(pvar)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PropVariantCopy(pvardest: *mut PROPVARIANT, pvarsrc: *const PROPVARIANT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropVariantCopy(pvardest: *mut PROPVARIANT, pvarsrc: *const PROPVARIANT) -> ::windows::core::HRESULT;
        }
        PropVariantCopy(::core::mem::transmute(pvardest), ::core::mem::transmute(pvarsrc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReadClassStg<'a, Param0: ::windows::core::IntoParam<'a, IStorage>>(pstg: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadClassStg(pstg: ::windows::core::RawPtr, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        ReadClassStg(pstg.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReadClassStm<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(pstm: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadClassStm(pstm: ::windows::core::RawPtr, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        ReadClassStm(pstm.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadFmtUserTypeStg<'a, Param0: ::windows::core::IntoParam<'a, IStorage>>(pstg: Param0, pcf: *mut u16, lplpszusertype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadFmtUserTypeStg(pstg: ::windows::core::RawPtr, pcf: *mut u16, lplpszusertype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        ReadFmtUserTypeStg(pstg.into_param().abi(), ::core::mem::transmute(pcf), ::core::mem::transmute(lplpszusertype)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
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
pub type STATFLAG = i32;
pub const STATFLAG_DEFAULT: STATFLAG = 0i32;
pub const STATFLAG_NONAME: STATFLAG = 1i32;
pub const STATFLAG_NOOPEN: STATFLAG = 2i32;
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub struct STATPROPSTG {
    pub lpwstrName: super::super::super::Foundation::PWSTR,
    pub propid: u32,
    pub vt: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STATPROPSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATPROPSTG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STATPROPSTG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STATPROPSTG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STATPROPSTG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STATPROPSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STATPROPSTG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type STGC = i32;
pub const STGC_DEFAULT: STGC = 0i32;
pub const STGC_OVERWRITE: STGC = 1i32;
pub const STGC_ONLYIFCURRENT: STGC = 2i32;
pub const STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE: STGC = 4i32;
pub const STGC_CONSOLIDATE: STGC = 8i32;
pub const STGFMT_ANY: u32 = 4u32;
pub const STGFMT_DOCFILE: u32 = 5u32;
pub const STGFMT_DOCUMENT: u32 = 0u32;
pub const STGFMT_FILE: u32 = 3u32;
pub const STGFMT_NATIVE: u32 = 1u32;
pub const STGFMT_STORAGE: u32 = 0u32;
pub type STGMOVE = i32;
pub const STGMOVE_MOVE: STGMOVE = 0i32;
pub const STGMOVE_COPY: STGMOVE = 1i32;
pub const STGMOVE_SHALLOWCOPY: STGMOVE = 2i32;
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
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STGOPTIONS {
    pub usVersion: u16,
    pub reserved: u16,
    pub ulSectorSize: u32,
    pub pwcsTemplateFile: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STGOPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STGOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STGOPTIONS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STGOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STGOPTIONS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STGOPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STGOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const STGOPTIONS_VERSION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConvertStg<'a, Param0: ::windows::core::IntoParam<'a, IStorage>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(pstg: Param0, fconvert: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConvertStg(pstg: ::windows::core::RawPtr, fconvert: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        SetConvertStg(pstg.into_param().abi(), fconvert.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgConvertPropertyToVariant(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut PROPVARIANT, pma: *const PMemoryAllocator) -> super::super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgConvertPropertyToVariant(pprop: *const SERIALIZEDPROPERTYVALUE, codepage: u16, pvar: *mut PROPVARIANT, pma: *const PMemoryAllocator) -> super::super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(StgConvertPropertyToVariant(::core::mem::transmute(pprop), ::core::mem::transmute(codepage), ::core::mem::transmute(pvar), ::core::mem::transmute(pma)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgConvertVariantToProperty<'a, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOLEAN>>(pvar: *const PROPVARIANT, codepage: u16, pprop: *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32, pid: u32, freserved: Param5, pcindirect: *mut u32) -> *mut SERIALIZEDPROPERTYVALUE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgConvertVariantToProperty(pvar: *const PROPVARIANT, codepage: u16, pprop: *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32, pid: u32, freserved: super::super::super::Foundation::BOOLEAN, pcindirect: *mut u32) -> *mut SERIALIZEDPROPERTYVALUE;
        }
        ::core::mem::transmute(StgConvertVariantToProperty(::core::mem::transmute(pvar), ::core::mem::transmute(codepage), ::core::mem::transmute(pprop), ::core::mem::transmute(pcb), ::core::mem::transmute(pid), freserved.into_param().abi(), ::core::mem::transmute(pcindirect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgCreateDocfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwcsname: Param0, grfmode: u32, reserved: u32) -> ::windows::core::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgCreateDocfile(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        StgCreateDocfile(pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StgCreateDocfileOnILockBytes<'a, Param0: ::windows::core::IntoParam<'a, ILockBytes>>(plkbyt: Param0, grfmode: u32, reserved: u32) -> ::windows::core::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgCreateDocfileOnILockBytes(plkbyt: ::windows::core::RawPtr, grfmode: u32, reserved: u32, ppstgopen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        StgCreateDocfileOnILockBytes(plkbyt.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StgCreatePropSetStg<'a, Param0: ::windows::core::IntoParam<'a, IStorage>>(pstorage: Param0, dwreserved: u32) -> ::windows::core::Result<IPropertySetStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgCreatePropSetStg(pstorage: ::windows::core::RawPtr, dwreserved: u32, pppropsetstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        StgCreatePropSetStg(pstorage.into_param().abi(), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IPropertySetStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StgCreatePropStg<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0, fmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, dwreserved: u32) -> ::windows::core::Result<IPropertyStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgCreatePropStg(punk: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        StgCreatePropStg(punk.into_param().abi(), ::core::mem::transmute(fmtid), ::core::mem::transmute(pclsid), ::core::mem::transmute(grfflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IPropertyStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn StgCreateStorageEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgCreateStorageEx(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        StgCreateStorageEx(pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(stgfmt), ::core::mem::transmute(grfattrs), ::core::mem::transmute(pstgoptions), ::core::mem::transmute(psecuritydescriptor), ::core::mem::transmute(riid), ::core::mem::transmute(ppobjectopen)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32) -> ::windows::core::Result<PROPVARIANT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgDeserializePropVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbmax: u32, ppropvar: *mut PROPVARIANT) -> ::windows::core::HRESULT;
        }
        let mut result__: ::core::mem::ManuallyDrop<PROPVARIANT> = ::core::mem::zeroed();
        StgDeserializePropVariant(::core::mem::transmute(pprop), ::core::mem::transmute(cbmax), ::core::mem::transmute(&mut result__)).from_abi::<PROPVARIANT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgGetIFillLockBytesOnFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwcsname: Param0) -> ::windows::core::Result<IFillLockBytes> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgGetIFillLockBytesOnFile(pwcsname: super::super::super::Foundation::PWSTR, ppflb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        StgGetIFillLockBytesOnFile(pwcsname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFillLockBytes>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StgGetIFillLockBytesOnILockBytes<'a, Param0: ::windows::core::IntoParam<'a, ILockBytes>>(pilb: Param0) -> ::windows::core::Result<IFillLockBytes> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgGetIFillLockBytesOnILockBytes(pilb: ::windows::core::RawPtr, ppflb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        StgGetIFillLockBytesOnILockBytes(pilb.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFillLockBytes>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgIsStorageFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwcsname: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgIsStorageFile(pwcsname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        StgIsStorageFile(pwcsname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StgIsStorageILockBytes<'a, Param0: ::windows::core::IntoParam<'a, ILockBytes>>(plkbyt: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgIsStorageILockBytes(plkbyt: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        StgIsStorageILockBytes(plkbyt.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StgOpenAsyncDocfileOnIFillLockBytes<'a, Param0: ::windows::core::IntoParam<'a, IFillLockBytes>>(pflb: Param0, grfmode: u32, asyncflags: u32) -> ::windows::core::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgOpenAsyncDocfileOnIFillLockBytes(pflb: ::windows::core::RawPtr, grfmode: u32, asyncflags: u32, ppstgopen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        StgOpenAsyncDocfileOnIFillLockBytes(pflb.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(asyncflags), ::core::mem::transmute(&mut result__)).from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgOpenLayoutDocfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwcsdfname: Param0, grfmode: u32, reserved: u32) -> ::windows::core::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgOpenLayoutDocfile(pwcsdfname: super::super::super::Foundation::PWSTR, grfmode: u32, reserved: u32, ppstgopen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        StgOpenLayoutDocfile(pwcsdfname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StgOpenPropStg<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0, fmtid: *const ::windows::core::GUID, grfflags: u32, dwreserved: u32) -> ::windows::core::Result<IPropertyStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgOpenPropStg(punk: *mut ::core::ffi::c_void, fmtid: *const ::windows::core::GUID, grfflags: u32, dwreserved: u32, pppropstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        StgOpenPropStg(punk.into_param().abi(), ::core::mem::transmute(fmtid), ::core::mem::transmute(grfflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IPropertyStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgOpenStorage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IStorage>>(pwcsname: Param0, pstgpriority: Param1, grfmode: u32, snbexclude: *const *const u16, reserved: u32) -> ::windows::core::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgOpenStorage(pwcsname: super::super::super::Foundation::PWSTR, pstgpriority: ::windows::core::RawPtr, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        StgOpenStorage(pwcsname.into_param().abi(), pstgpriority.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(snbexclude), ::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn StgOpenStorageEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgOpenStorageEx(pwcsname: super::super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: *mut STGOPTIONS, psecuritydescriptor: *const super::super::super::Security::SECURITY_DESCRIPTOR, riid: *const ::windows::core::GUID, ppobjectopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        StgOpenStorageEx(pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(stgfmt), ::core::mem::transmute(grfattrs), ::core::mem::transmute(pstgoptions), ::core::mem::transmute(psecuritydescriptor), ::core::mem::transmute(riid), ::core::mem::transmute(ppobjectopen)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StgOpenStorageOnILockBytes<'a, Param0: ::windows::core::IntoParam<'a, ILockBytes>, Param1: ::windows::core::IntoParam<'a, IStorage>>(plkbyt: Param0, pstgpriority: Param1, grfmode: u32, snbexclude: *const *const u16, reserved: u32) -> ::windows::core::Result<IStorage> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgOpenStorageOnILockBytes(plkbyt: ::windows::core::RawPtr, pstgpriority: ::windows::core::RawPtr, grfmode: u32, snbexclude: *const *const u16, reserved: u32, ppstgopen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        StgOpenStorageOnILockBytes(plkbyt.into_param().abi(), pstgpriority.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(snbexclude), ::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<IStorage>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgPropertyLengthAsVariant(pprop: *const SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: u8) -> u32;
        }
        ::core::mem::transmute(StgPropertyLengthAsVariant(::core::mem::transmute(pprop), ::core::mem::transmute(cbprop), ::core::mem::transmute(codepage), ::core::mem::transmute(breserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgSerializePropVariant(ppropvar: *const PROPVARIANT, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgSerializePropVariant(ppropvar: *const PROPVARIANT, ppprop: *mut *mut SERIALIZEDPROPERTYVALUE, pcb: *mut u32) -> ::windows::core::HRESULT;
        }
        StgSerializePropVariant(::core::mem::transmute(ppropvar), ::core::mem::transmute(ppprop), ::core::mem::transmute(pcb)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StgSetTimes<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(lpszname: Param0, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StgSetTimes(lpszname: super::super::super::Foundation::PWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        StgSetTimes(lpszname.into_param().abi(), ::core::mem::transmute(pctime), ::core::mem::transmute(patime), ::core::mem::transmute(pmtime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: ::windows::core::GUID,
    pub pStream: ::core::option::Option<super::IStream>,
}
impl ::core::clone::Clone for VERSIONEDSTREAM {
    fn clone(&self) -> Self {
        Self { guidVersion: self.guidVersion, pStream: self.pStream.clone() }
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
#[inline]
pub unsafe fn WriteClassStg<'a, Param0: ::windows::core::IntoParam<'a, IStorage>>(pstg: Param0, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteClassStg(pstg: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        WriteClassStg(pstg.into_param().abi(), ::core::mem::transmute(rclsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WriteClassStm<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(pstm: Param0, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteClassStm(pstm: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        WriteClassStm(pstm.into_param().abi(), ::core::mem::transmute(rclsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteFmtUserTypeStg<'a, Param0: ::windows::core::IntoParam<'a, IStorage>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pstg: Param0, cf: u16, lpszusertype: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteFmtUserTypeStg(pstg: ::windows::core::RawPtr, cf: u16, lpszusertype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        WriteFmtUserTypeStg(pstg.into_param().abi(), ::core::mem::transmute(cf), lpszusertype.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
